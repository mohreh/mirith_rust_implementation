use crate::params::{HASH_SIZE, SEED_SIZE};
use crate::{
    fips202::KeccakState,
    hash::{Hash, Seed},
};

pub type Prng = KeccakState;

pub fn prng_init(prng: &mut Prng, salt: Option<&Hash>, seed: Option<&Seed>) {
    let mut input = [0u8; HASH_SIZE + SEED_SIZE];
    prng.init();
    // Set 'input = salt | seed'.
    if let Some(salt) = salt {
        input[..HASH_SIZE].copy_from_slice(salt);
    }
    if let Some(seed) = seed {
        input[HASH_SIZE..].copy_from_slice(seed);
    }
    prng.keccak_absorb(&input);
    prng.keccak_finalize(0x1F);
}

pub fn prng_bytes(prng: &mut Prng, out: &mut [u8], outlen: usize) {
    prng.keccak_squeeze(out, outlen);
}
