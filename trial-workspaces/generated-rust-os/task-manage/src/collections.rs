use core::hash::{BuildHasherDefault, Hasher};

const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

#[derive(Clone)]
pub(crate) struct SimpleHasher(u64);

impl Default for SimpleHasher {
    fn default() -> Self {
        Self(FNV_OFFSET_BASIS)
    }
}

impl Hasher for SimpleHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        let mut hash = self.0;
        for &b in bytes {
            hash ^= b as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        self.0 = hash;
    }
}

pub type HashMap<K, V> = hashbrown::HashMap<K, V, BuildHasherDefault<SimpleHasher>>;
pub type HashSet<T> = hashbrown::HashSet<T, BuildHasherDefault<SimpleHasher>>;

pub fn hash_map<K, V>() -> HashMap<K, V> {
    HashMap::with_hasher(BuildHasherDefault::default())
}

pub fn hash_set<T>() -> HashSet<T> {
    HashSet::with_hasher(BuildHasherDefault::default())
}
