extern crate std;

use super::{BlockDevice, EasyFileSystem, FileHandle, Inode, OpenFlags, UserBuffer, BLOCK_SZ};
use std::boxed::Box;
use std::string::ToString;
use std::sync::{Arc, Mutex as StdMutex, OnceLock};
use std::vec;
use std::vec::Vec;

struct MockBlockDevice {
    blocks: Arc<StdMutex<Vec<Vec<u8>>>>,
}

impl MockBlockDevice {
    fn new(block_size: usize, num_blocks: usize) -> Self {
        let mut blocks = Vec::new();
        for _ in 0..num_blocks {
            blocks.push(vec![0u8; block_size]);
        }
        Self {
            blocks: Arc::new(StdMutex::new(blocks)),
        }
    }
}

impl BlockDevice for MockBlockDevice {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        let blocks = self.blocks.lock().unwrap();
        if block_id < blocks.len() {
            let block = &blocks[block_id];
            let len = buf.len().min(block.len());
            buf[..len].copy_from_slice(&block[..len]);
        }
    }

    fn write_block(&self, block_id: usize, buf: &[u8]) {
        let mut blocks = self.blocks.lock().unwrap();
        if block_id < blocks.len() {
            let block = &mut blocks[block_id];
            let len = buf.len().min(block.len());
            block[..len].copy_from_slice(&buf[..len]);
        }
    }
}

const TEST_TOTAL_BLOCKS: u32 = 4096;
const TEST_INODE_BITMAP_BLOCKS: u32 = 1;

static TEST_LOCK: OnceLock<StdMutex<()>> = OnceLock::new();
static TEST_DEVICE: OnceLock<Arc<MockBlockDevice>> = OnceLock::new();

fn test_lock() -> std::sync::MutexGuard<'static, ()> {
    TEST_LOCK
        .get_or_init(|| StdMutex::new(()))
        .lock()
        .unwrap_or_else(|err| err.into_inner())
}

fn test_device() -> Arc<MockBlockDevice> {
    TEST_DEVICE
        .get_or_init(|| Arc::new(MockBlockDevice::new(BLOCK_SZ, TEST_TOTAL_BLOCKS as usize)))
        .clone()
}

fn with_test_device<T>(f: impl FnOnce(Arc<MockBlockDevice>) -> T) -> T {
    let _guard = test_lock();
    let device = test_device();
    f(device)
}

fn with_test_fs<T>(f: impl FnOnce(Arc<MockBlockDevice>, Inode) -> T) -> T {
    let _guard = test_lock();
    let device = test_device();
    let efs = EasyFileSystem::create(device.clone(), TEST_TOTAL_BLOCKS, TEST_INODE_BITMAP_BLOCKS);
    let root = EasyFileSystem::root_inode(&efs);
    f(device, root)
}

#[test]
fn test_block_size_and_block_device_trait() {
    assert_eq!(BLOCK_SZ, 512);
    let device = Arc::new(MockBlockDevice::new(BLOCK_SZ, 10));
    let test_data = vec![0xAA; BLOCK_SZ];
    device.write_block(0, &test_data);
    let mut read_buf = vec![0u8; BLOCK_SZ];
    device.read_block(0, &mut read_buf);
    assert_eq!(read_buf, test_data);
}

#[test]
fn test_open_flags_and_user_buffer() {
    assert_eq!(OpenFlags::RDONLY.bits(), 0);
    assert_eq!(OpenFlags::WRONLY.bits(), 1);
    assert_eq!(OpenFlags::RDWR.bits(), 2);
    assert_eq!(OpenFlags::CREATE.bits(), 512);
    assert_eq!(OpenFlags::TRUNC.bits(), 1024);
    assert_eq!(OpenFlags::RDONLY.read_write(), (true, false));
    assert_eq!(OpenFlags::WRONLY.read_write(), (false, true));
    assert_eq!(OpenFlags::RDWR.read_write(), (true, true));

    let buf1 = Box::leak(Box::new(vec![0u8; 10]));
    let buf2 = Box::leak(Box::new(vec![0u8; 20]));
    let user_buf = UserBuffer::new(vec![buf1.as_mut_slice(), buf2.as_mut_slice()]);
    assert_eq!(user_buf.len(), 30);
    assert_eq!(UserBuffer::new(vec![]).len(), 0);
}

#[test]
fn test_file_handle_read_write_and_empty() {
    with_test_fs(|_device, root| {
        let inode = root.create("handle_file").unwrap();
        let mut writer = FileHandle::new(false, true, inode);

        let write_slice: &'static mut [u8] = Box::leak(Box::new([b'a', b'b', b'c']));
        let write_buf = UserBuffer::new(vec![write_slice]);
        let write_len = writer.write(write_buf);
        assert_eq!(write_len, 3);

        let inode = root.find("handle_file").unwrap();
        let mut handle = FileHandle::new(true, true, inode);
        let read_box = Box::new([0u8; 3]);
        let read_ptr = read_box.as_ptr();
        let read_slice: &'static mut [u8] = Box::leak(read_box);
        let read_buf = UserBuffer::new(vec![read_slice]);
        let read_len = handle.read(read_buf);
        assert_eq!(read_len, 3);
        let read_back = unsafe { std::slice::from_raw_parts(read_ptr, 3) };
        assert_eq!(read_back, b"abc");
        assert!(handle.readable());
        assert!(handle.writable());

        let empty = FileHandle::empty(true, false);
        assert!(empty.readable());
        assert!(!empty.writable());
    });
}

#[test]
fn test_easy_filesystem_create_open_and_root_inode() {
    with_test_device(|device| {
        let efs1 =
            EasyFileSystem::create(device.clone(), TEST_TOTAL_BLOCKS, TEST_INODE_BITMAP_BLOCKS);
        let root1 = EasyFileSystem::root_inode(&efs1);
        assert_eq!(root1.readdir().len(), 0);
        root1.create("test_file").unwrap();

        let efs2 = EasyFileSystem::open(device);
        let root2 = EasyFileSystem::root_inode(&efs2);
        assert!(root2.find("test_file").is_some());
    });
}

#[test]
fn test_inode_find_create_readdir_and_clear() {
    with_test_fs(|_device, root| {
        assert!(root.find("nonexistent").is_none());
        let file = root.create("test_file").unwrap();
        assert!(root.find("test_file").is_some());

        root.create("file1").unwrap();
        root.create("file2").unwrap();
        root.create("file3").unwrap();
        assert_eq!(
            root.readdir(),
            vec![
                "test_file".to_string(),
                "file1".to_string(),
                "file2".to_string(),
                "file3".to_string()
            ]
        );

        file.write_at(0, b"test data");
        let mut buf = vec![0u8; 9];
        assert_eq!(file.read_at(0, &mut buf), 9);
        assert_eq!(&buf, b"test data");
        file.clear();
        assert_eq!(file.read_at(0, &mut buf), 0);
    });
}

#[test]
#[should_panic]
fn test_inode_create_name_too_long_panics() {
    with_test_fs(|_device, root| {
        let name = "a".repeat(29);
        let _ = root.create(&name);
    });
}

#[test]
fn test_inode_read_write_and_offsets() {
    with_test_fs(|_device, root| {
        let file = root.create("test_file").unwrap();
        let test_data = b"Hello, easy-fs!";
        file.write_at(0, test_data);

        let mut read_buf = vec![0u8; test_data.len()];
        let read_len = file.read_at(0, &mut read_buf);
        assert_eq!(read_len, test_data.len());
        assert_eq!(&read_buf[..read_len], test_data);

        file.clear();
        file.write_at(0, b"Hello, World!");
        let mut buf = vec![0u8; 5];
        assert_eq!(file.read_at(7, &mut buf), 5);
        assert_eq!(&buf, b"World");

        file.write_at(7, b"Rust");
        let mut buf = vec![0u8; 13];
        assert_eq!(file.read_at(0, &mut buf), 13);
        assert_eq!(&buf, b"Hello, Rustd!");

        let mut eof = [0u8; 1];
        assert_eq!(file.read_at(13, &mut eof), 0);
    });
}

#[test]
fn test_inode_read_write_large() {
    with_test_fs(|_device, root| {
        let file = root.create("large_file").unwrap();
        let test_data_len = BLOCK_SZ * 28 + 1;
        let test_data: Vec<u8> = (0..test_data_len).map(|i| (i % 256) as u8).collect();
        file.write_at(0, &test_data);

        let mut read_buf = vec![0u8; test_data.len()];
        let read_len = file.read_at(0, &mut read_buf);
        assert_eq!(read_len, test_data.len());
        assert_eq!(&read_buf[..read_len], &test_data);
    });
}

#[test]
#[should_panic]
fn test_open_invalid_superblock_panics() {
    let device = Arc::new(MockBlockDevice::new(BLOCK_SZ, 32));
    let _ = EasyFileSystem::open(device);
}
