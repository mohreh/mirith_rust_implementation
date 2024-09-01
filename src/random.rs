#[cfg(not(feature = "deterministic"))]
pub fn randombytes<R>(target: &mut [u8]) {
    use rand::{thread_rng, RngCore};
    thread_rng().fill_bytes(target);
}

#[cfg(feature = "deterministic")]
pub mod prng {
    use crate::hash::Seed;
    use rand::{rngs::SmallRng, RngCore, SeedableRng};

    static mut PRNG_SINGLETON: Option<SmallRng> = None;

    // #[derive(Default)]
    // struct Prng {
    //     prng_singleton: Option<SmallRng>,
    // }
    //
    // impl Prng {
    //     pub fn new(seed: &Seed) {
    //         let mut prng = Prng::default();
    //         prng.prng_singleton = Some(SmallRng::from_seed(*seed));
    //     }
    //
    //     pub fn gen_bytes(&mut self, out: &mut [u8], outlen: usize) {
    //         if let Some(ref mut prng) = self.prng_singleton {
    //             prng.fill_bytes(out);
    //         } else {
    //             panic!("PRNG not initialized!");
    //         }
    //     }
    // }

    pub fn prng_init(seed: Seed) {
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
pub fn randombytes(target: &mut [u8]) {
    prng::prng_bytes(target);
}
