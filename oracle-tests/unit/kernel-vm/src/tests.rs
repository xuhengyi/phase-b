extern crate std;

use super::{AddressSpace, PageManager};
use page_table::{MmuMeta, Pte, VAddr, VmFlags, PPN, VPN};
use std::alloc::{alloc_zeroed, Layout};
use std::collections::{BTreeMap, BTreeSet};
use std::ptr::NonNull;
use std::sync::{Mutex, OnceLock};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct TestSv39;

impl MmuMeta for TestSv39 {
    const P_ADDR_BITS: usize = 56;
    const PAGE_BITS: usize = 12;
    const LEVEL_BITS: &'static [usize] = &[9; 3];
    const PPN_POS: usize = 10;

    fn is_leaf(value: usize) -> bool {
        const MASK: usize = 0b1110;
        value & MASK != 0
    }
}

const PAGE_SIZE: usize = 1 << TestSv39::PAGE_BITS;
const FLAG_R: usize = 1 << 1;
const FLAG_W: usize = 1 << 2;
const FLAG_X: usize = 1 << 3;

fn flags(bits: usize) -> VmFlags<TestSv39> {
    unsafe { VmFlags::from_raw(TestSv39::VALID_FLAG | bits) }
}

#[derive(Default)]
struct GlobalPages {
    next_ppn: usize,
    ppn_to_addr: BTreeMap<usize, usize>,
    addr_to_ppn: BTreeMap<usize, usize>,
}

impl GlobalPages {
    fn alloc_block(&mut self, len: usize) -> (usize, NonNull<u8>) {
        let layout = Layout::from_size_align(len * PAGE_SIZE, PAGE_SIZE).unwrap();
        let ptr = unsafe { alloc_zeroed(layout) };
        let ptr = NonNull::new(ptr).expect("page allocation failed");

        let start_ppn = self.next_ppn.max(1);
        self.next_ppn = start_ppn + len;
        for i in 0..len {
            let addr = unsafe { ptr.as_ptr().add(i * PAGE_SIZE) } as usize;
            self.ppn_to_addr.insert(start_ppn + i, addr);
            self.addr_to_ppn.insert(addr, start_ppn + i);
        }
        (start_ppn, ptr)
    }

    fn ptr_for_ppn<T>(&self, ppn: usize) -> NonNull<T> {
        NonNull::new(self.ppn_to_addr[&ppn] as *mut T).unwrap()
    }

    fn ppn_for_ptr<T>(&self, ptr: NonNull<T>) -> usize {
        let addr = (ptr.as_ptr() as usize) & !(PAGE_SIZE - 1);
        self.addr_to_ppn[&addr]
    }
}

fn global_pages() -> &'static Mutex<GlobalPages> {
    static GLOBAL: OnceLock<Mutex<GlobalPages>> = OnceLock::new();
    GLOBAL.get_or_init(|| Mutex::new(GlobalPages::default()))
}

struct MockManager {
    root_ppn: usize,
    owned: BTreeSet<usize>,
}

impl PageManager<TestSv39> for MockManager {
    fn new_root() -> Self {
        let mut global = global_pages().lock().unwrap();
        let (root_ppn, _) = global.alloc_block(1);
        let mut owned = BTreeSet::new();
        owned.insert(root_ppn);
        Self { root_ppn, owned }
    }

    fn root_ptr(&self) -> NonNull<Pte<TestSv39>> {
        global_pages()
            .lock()
            .unwrap()
            .ptr_for_ppn::<Pte<TestSv39>>(self.root_ppn)
    }

    fn root_ppn(&self) -> PPN<TestSv39> {
        PPN::new(self.root_ppn)
    }

    fn p_to_v<T>(&self, ppn: PPN<TestSv39>) -> NonNull<T> {
        global_pages().lock().unwrap().ptr_for_ppn(ppn.val())
    }

    fn v_to_p<T>(&self, ptr: NonNull<T>) -> PPN<TestSv39> {
        PPN::new(global_pages().lock().unwrap().ppn_for_ptr(ptr))
    }

    fn allocate(&mut self, len: usize, _flags: &mut VmFlags<TestSv39>) -> NonNull<u8> {
        let mut global = global_pages().lock().unwrap();
        let (start_ppn, ptr) = global.alloc_block(len);
        for ppn in start_ppn..start_ppn + len {
            self.owned.insert(ppn);
        }
        ptr
    }

    fn deallocate(&mut self, _pte: Pte<TestSv39>, len: usize) -> usize {
        len
    }

    fn check_owned(&self, pte: Pte<TestSv39>) -> bool {
        self.owned.contains(&pte.ppn().val())
    }

    fn drop_root(&mut self) {}
}

#[test]
fn test_map_and_translate_follow_contract() {
    let mut space = AddressSpace::<TestSv39, MockManager>::new();
    let start_vpn = VPN::<TestSv39>::new(0x123);
    let end_vpn = VPN::<TestSv39>::new(start_vpn.val() + 1);
    let addr_base = start_vpn.val() << TestSv39::PAGE_BITS;
    space.map(start_vpn..end_vpn, b"rust", 4, flags(FLAG_R | FLAG_W));

    let prefix = space
        .translate::<u8>(VAddr::<TestSv39>::new(addr_base + 3), flags(FLAG_R))
        .unwrap();
    let data = space
        .translate::<u8>(VAddr::<TestSv39>::new(addr_base + 4), flags(FLAG_R))
        .unwrap();
    assert!(space
        .translate::<u8>(VAddr::<TestSv39>::new(addr_base + 4), flags(FLAG_X))
        .is_none());

    unsafe {
        assert_eq!(*prefix.as_ptr(), 0);
        assert_eq!(std::slice::from_raw_parts(data.as_ptr(), 4), b"rust");
    }
}

#[test]
fn test_cloneself_allocates_independent_pages() {
    let mut source = AddressSpace::<TestSv39, MockManager>::new();
    let start_vpn = VPN::<TestSv39>::new(0x456);
    let end_vpn = VPN::<TestSv39>::new(start_vpn.val() + 1);
    let addr = (start_vpn.val() << TestSv39::PAGE_BITS) + 8;

    source.map(start_vpn..end_vpn, b"abcd", 8, flags(FLAG_R | FLAG_W));

    let mut cloned = AddressSpace::<TestSv39, MockManager>::new();
    source.cloneself(&mut cloned);

    let source_ptr = source
        .translate::<u8>(VAddr::<TestSv39>::new(addr), flags(FLAG_W))
        .unwrap();
    unsafe {
        *source_ptr.as_ptr() = b'Z';
    }

    let cloned_ptr = cloned
        .translate::<u8>(VAddr::<TestSv39>::new(addr), flags(FLAG_R))
        .unwrap();
    unsafe {
        assert_eq!(*cloned_ptr.as_ptr(), b'a');
    }
}

#[test]
fn test_map_extern_can_share_original_mapping() {
    let mut source = AddressSpace::<TestSv39, MockManager>::new();
    let vpn = VPN::<TestSv39>::new(0x789);
    let end_vpn = VPN::<TestSv39>::new(vpn.val() + 1);
    let addr = vpn.val() << TestSv39::PAGE_BITS;

    source.map(vpn..end_vpn, b"xy", 0, flags(FLAG_R | FLAG_W));

    let source_ptr = source
        .translate::<u8>(VAddr::<TestSv39>::new(addr), flags(FLAG_R))
        .unwrap();
    let shared_ppn = PPN::new(global_pages().lock().unwrap().ppn_for_ptr(source_ptr));

    let mut target = AddressSpace::<TestSv39, MockManager>::new();
    target.map_extern(vpn..end_vpn, shared_ppn, flags(FLAG_R | FLAG_W));

    let write_ptr = source
        .translate::<u8>(VAddr::<TestSv39>::new(addr), flags(FLAG_W))
        .unwrap();
    unsafe {
        *write_ptr.as_ptr() = b'Q';
    }

    let target_ptr = target
        .translate::<u8>(VAddr::<TestSv39>::new(addr), flags(FLAG_R))
        .unwrap();
    unsafe {
        assert_eq!(*target_ptr.as_ptr(), b'Q');
    }
}
