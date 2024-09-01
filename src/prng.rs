use crate::{
    fips202::{shake256_absorb, shake256_finalize, shake256_squeeze},
    params::{HASH_SIZE, SEED_SIZE},
};
use crate::{
    fips202::{shake256_init, KeccakState},
    hash::{Hash, Seed},
};

pub struct Prng {
    prng: KeccakState,
}

impl Prng {
    pub fn new(salt: &Option<Hash>, seed: &Option<Seed>) -> Self {
        let mut input = [0u8; HASH_SIZE + SEED_SIZE];
        let inlen = input.len();
        let mut prng = shake256_init();
        if let Some(salt) = salt {
            input[..HASH_SIZE].copy_from_slice(salt);
        }
        if let Some(seed) = seed {
            input[HASH_SIZE..].copy_from_slice(seed);
        }
        shake256_absorb(&mut prng, &mut input, inlen);
        shake256_finalize(&mut prng);

        Self { prng }
    }

    pub fn gen_bytes(&mut self, out: &mut [u8], outlen: usize) {
        shake256_squeeze(&mut self.prng, out, outlen)
        // self.prng.keccak_squeeze(out, outlen);
    }
}
