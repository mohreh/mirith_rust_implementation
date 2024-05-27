use openssl::hash::{Hasher, MessageDigest};

use crate::params::{HASH_SIZE, SEED_SIZE};

pub type Hash = [u8; HASH_SIZE];
pub type Seed = [u8; SEED_SIZE];
pub type Ff = u8;

trait MatrixBytesSize {
    fn matrix_bytes_size(dim1: usize, dim2: usize) -> usize;
}

impl MatrixBytesSize for usize {
    fn matrix_bytes_size(dim1: usize, dim2: usize) -> usize {
        dim1 * dim2
    }
}

pub struct HashCtx {
    hasher: Hasher,
}

impl Default for HashCtx {
    fn default() -> Self {
        let hasher = match HASH_SIZE {
            32 => Hasher::new(MessageDigest::sha3_256()).unwrap(),
            48 => Hasher::new(MessageDigest::sha3_384()).unwrap(),
            64 => Hasher::new(MessageDigest::sha3_512()).unwrap(),
            _ => panic!("HASH_SIZE not implemented!"),
        };
        HashCtx { hasher }
    }
}

impl HashCtx {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn update(&mut self, data: &[u8]) {
        self.hasher.update(data).unwrap();
    }

    pub fn finalize(mut self, hash: &mut Hash) {
        let result = self.hasher.finish().unwrap();
        hash.copy_from_slice(&result[..HASH_SIZE]);
    }
}

pub fn hash_equal(hash1: &Hash, hash2: &Hash) -> bool {
    hash1 == hash2
}

pub fn hash_digest0(hash: &mut Hash, salt: &Hash, l: i32, i: i32, seed: &Seed) {
    let mut ctx = HashCtx::new();
    ctx.update(salt);
    ctx.update(&l.to_le_bytes());
    ctx.update(&i.to_le_bytes());
    ctx.update(seed);
    ctx.finalize(hash);
}

#[allow(clippy::too_many_arguments)]
pub fn hash_digest0_aux(
    hash: &mut Hash,
    salt: &Hash,
    l: i32,
    i: i32,
    seed: &Seed,
    a_aux: &[Ff],
    k_aux: &[Ff],
    c_aux: &[Ff],
) {
    let mut ctx = HashCtx::new();
    ctx.update(salt);
    ctx.update(&l.to_le_bytes());
    ctx.update(&i.to_le_bytes());
    ctx.update(seed);
    ctx.update(a_aux);
    ctx.update(k_aux);
    ctx.update(c_aux);
    ctx.finalize(hash);
}
