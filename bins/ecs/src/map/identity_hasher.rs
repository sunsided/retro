use std::hash::{BuildHasher, Hasher};

#[derive(Default)]
pub struct IdentityHasher {
    hash: u64,
}

impl IdentityHasher {
    pub const fn new() -> Self {
        Self { hash: 0 }
    }
}

impl Hasher for IdentityHasher {
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write(&mut self, bytes: &[u8]) {
        debug_assert!(bytes.len() <= 8);
        let mut array = [0u8; 8];
        array[..bytes.len()].copy_from_slice(bytes);
        self.hash = u64::from_le_bytes(array);
    }

    fn write_u32(&mut self, i: u32) {
        self.hash = i as u64;
    }

    fn write_u64(&mut self, i: u64) {
        self.hash = i;
    }
}

#[derive(Default)]
pub struct BuildIdentityHasher;

impl BuildHasher for BuildIdentityHasher {
    type Hasher = IdentityHasher;
    fn build_hasher(&self) -> Self::Hasher {
        IdentityHasher::default()
    }
}
