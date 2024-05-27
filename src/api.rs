use crate::config::MODE;

#[cfg(not(feature = "hypercube"))]
const fn init(mode: u8) -> (usize, usize, usize) {
    match mode {
        0 => (129, 145, 7877),   /* Ia, fast. */
        1 => (129, 145, 5673),   /* Ia, short. */
        2 => (144, 160, 9105),   /* Ib, fast. */
        3 => (144, 160, 6309),   /* Ib, short. */
        4 => (205, 229, 17139),  /* IIIa, fast. */
        5 => (205, 229, 12440),  /* IIIa, short. */
        6 => (205, 229, 18459),  /* IIIb, fast. */
        7 => (205, 229, 13136),  /* IIIb, short. */
        8 => (253, 285, 31468),  /* Va, fast. */
        9 => (253, 285, 21795),  /* Va, short. */
        10 => (274, 306, 34059), /* Vb, fast. */
        11 => (274, 306, 23182), /* Vb, short. */
        _ => panic!("MIRITH_MODE not implemented!"),
    }
}

#[cfg(feature = "hypercube")]
const fn init(mode: u8) -> (usize, usize, usize) {
    match mode {
        0 => (129, 145, 7877),   /* Ia, fast. */
        1 => (129, 145, 5673),   /* Ia, short. */
        2 => (129, 145, 5036),   /* Ia, shorter. */
        3 => (129, 145, 4536),   /* Ia, shortest. */
        4 => (144, 160, 9105),   /* Ib, fast. */
        5 => (144, 160, 6309),   /* Ib, short. */
        6 => (144, 160, 5491),   /* Ib, shorter. */
        7 => (144, 160, 4886),   /* Ib, shortest. */
        8 => (205, 229, 17139),  /* IIIa, fast. */
        9 => (205, 229, 12440),  /* IIIa, short. */
        10 => (205, 229, 10746), /* IIIa, shorter. */
        11 => (205, 229, 9954),  /* IIIa, shortest. */
        12 => (205, 229, 18459), /* IIIb, fast. */
        13 => (205, 229, 13136), /* IIIb, short. */
        14 => (205, 229, 11202), /* IIIb, shorter. */
        15 => (205, 229, 10314), /* IIIb, shortest. */
        16 => (253, 285, 31468), /* Va, fast. */
        17 => (253, 285, 21795), /* Va, short. */
        18 => (253, 285, 19393), /* Va, shorter. */
        19 => (253, 285, 17522), /* Va, shortest. */
        20 => (274, 306, 34059), /* Vb, fast. */
        21 => (274, 306, 23182), /* Vb, short. */
        22 => (274, 306, 20394), /* Vb, shorter. */
        23 => (274, 306, 18292), /* Vb, shortest. */
        _ => panic!("MIRITH_MODE not implemented!"),
    }
}

const INIT_VALUES: (usize, usize, usize) = init(MODE);
pub const CRYPTO_PUBLICKEYBYTES: usize = INIT_VALUES.0;
pub const CRYPTO_SECRETKEYBYTES: usize = INIT_VALUES.1;
pub const CRYPTO_BYTES: usize = INIT_VALUES.2;
