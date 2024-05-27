#[cfg(not(feature = "deterministic"))]
pub fn randombytes<R>(target: &mut [u8]) {
    use rand::{thread_rng, RngCore};
    thread_rng().fill_bytes(target);
}

#[cfg(feature = "deterministic")]
mod prng {
    use crate::hash::Seed;
    use rand::{rngs::SmallRng, RngCore, SeedableRng};

    static mut PRNG_SINGLETON: Option<SmallRng> = None;

    pub fn init_prng(seed: Seed) {
        unsafe {
            PRNG_SINGLETON = Some(SmallRng::from_seed(seed));
        }
    }

    pub fn prng_bytes(target: &mut [u8]) {
        unsafe {
            if let Some(ref mut prng) = PRNG_SINGLETON {
                prng.fill_bytes(target);
            } else {
                panic!("PRNG not initialized!");
            }
        }
    }
}

#[cfg(feature = "deterministic")]
pub fn randombytes<R>(target: &mut [u8]) {
    prng::prng_bytes(target);
}
