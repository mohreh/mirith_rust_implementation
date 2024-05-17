use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crate::params::N_PARTIES_ROUND;

pub fn get_second_challenges<const TAU: usize, const HASH_SIZE: usize>(
    i_star: &mut [usize; TAU],
    // hash2: &[u8; HASH_SIZE],
    hash2: &[u8; 32],
) {
    let mut prng = ChaCha20Rng::from_seed(*hash2);

    for i_star_elem in i_star.iter_mut() {
        let r = prng.gen::<usize>();
        *i_star_elem = r % *N_PARTIES_ROUND;
    }
}
