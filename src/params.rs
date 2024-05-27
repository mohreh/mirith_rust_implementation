use crate::config::MODE;

#[cfg(not(feature = "hypercube"))]
const fn init(mode: u8) -> ParamsTuple {
    match mode {
        0 => (16, 15, 15, 78, 6, 5, 39, 2, 4, 16, 16, 32, 4, 31), // Ia, fast
        1 => (16, 15, 15, 78, 6, 9, 19, 2, 8, 256, 16, 32, 8, 511), // Ia, short
        2 => (16, 16, 16, 142, 4, 5, 39, 2, 4, 16, 16, 32, 4, 31), // Ib, fast
        3 => (16, 16, 16, 142, 4, 9, 19, 2, 8, 256, 16, 32, 8, 511), // Ib, short
        4 => (16, 19, 19, 109, 8, 7, 55, 2, 4, 16, 24, 48, 4, 31), // IIIa, fast
        5 => (16, 19, 19, 109, 8, 9, 29, 2, 8, 256, 24, 48, 8, 511), // IIIa, short
        6 => (16, 19, 19, 167, 6, 7, 55, 2, 4, 16, 24, 48, 4, 31), // IIIb, fast
        7 => (16, 19, 19, 167, 6, 9, 29, 2, 8, 256, 24, 48, 8, 511), // IIIb, short
        8 => (16, 21, 21, 189, 7, 10, 71, 2, 4, 16, 32, 64, 4, 31), // Va, fast
        9 => (16, 21, 21, 189, 7, 10, 38, 2, 8, 256, 32, 64, 8, 511), // Va, short
        10 => (16, 22, 22, 254, 6, 10, 71, 2, 4, 16, 32, 64, 4, 31), // Vb, fast
        11 => (16, 22, 22, 254, 6, 10, 38, 2, 8, 256, 32, 64, 8, 511), // Vb, short
        _ => panic!("MIRITH_MODE not implemented!"),
    }
}

#[cfg(feature = "hypercube")]
const fn init(mode: u8) -> ParamsTuple {
    match mode {
        0 => (16, 15, 15, 78, 6, 5, 39, 2, 4, 16, 16, 32, 4, 31), // Ia, fast
        1 => (16, 15, 15, 78, 6, 9, 19, 2, 8, 256, 16, 32, 8, 511), // Ia, short
        2 => (16, 15, 15, 78, 6, 12, 13, 2, 12, 4096, 16, 32, 12, 8191), // Ia, shorter
        3 => (16, 15, 15, 78, 6, 12, 10, 2, 16, 65536, 16, 32, 16, 131071), // Ia, shortest
        4 => (16, 16, 16, 142, 4, 5, 39, 2, 4, 16, 16, 32, 4, 31), // Ib, fast
        5 => (16, 16, 16, 142, 4, 9, 19, 2, 8, 256, 16, 32, 8, 511), // Ib, short
        6 => (16, 16, 16, 142, 4, 12, 13, 2, 12, 4096, 16, 32, 12, 8191), // Ib, shorter
        7 => (16, 16, 16, 142, 4, 12, 10, 2, 16, 65536, 16, 32, 16, 131071), // Ib, shortest
        8 => (16, 19, 19, 109, 8, 7, 55, 2, 4, 16, 24, 48, 4, 31), // IIIa, fast
        9 => (16, 19, 19, 109, 8, 9, 29, 2, 8, 256, 24, 48, 8, 511), // IIIa, short
        10 => (16, 19, 19, 109, 8, 13, 19, 2, 12, 4096, 24, 48, 12, 8191), // IIIa, shorter
        11 => (16, 19, 19, 109, 8, 13, 15, 2, 16, 65536, 24, 48, 16, 131071), // IIIa, shortest
        12 => (16, 19, 19, 167, 6, 7, 55, 2, 4, 16, 24, 48, 4, 31), // IIIb, fast
        13 => (16, 19, 19, 167, 6, 9, 29, 2, 8, 256, 24, 48, 8, 511), // IIIb, short
        14 => (16, 19, 19, 167, 6, 13, 19, 2, 12, 4096, 24, 48, 12, 8191), // IIIb, shorter
        15 => (16, 19, 19, 167, 6, 13, 15, 2, 16, 65536, 24, 48, 16, 131071), // IIIb, shortest
        16 => (16, 21, 21, 189, 7, 10, 71, 2, 4, 16, 32, 64, 4, 31), // Va, fast
        17 => (16, 21, 21, 189, 7, 10, 38, 2, 8, 256, 32, 64, 8, 511), // Va, short
        18 => (16, 21, 21, 189, 7, 14, 26, 2, 12, 4096, 32, 64, 12, 8191), // Va, shorter
        19 => (16, 21, 21, 189, 7, 14, 20, 2, 16, 65536, 32, 64, 16, 131071), // Va, shortest
        20 => (16, 22, 22, 254, 6, 10, 71, 2, 4, 16, 32, 64, 4, 31), // Vb, fast
        21 => (16, 22, 22, 254, 6, 10, 38, 2, 8, 256, 32, 64, 8, 511), // Vb, short
        22 => (16, 22, 22, 254, 6, 14, 26, 2, 12, 4096, 32, 64, 12, 8191), // Vb, shorter
        23 => (16, 22, 22, 254, 6, 14, 20, 2, 16, 65536, 32, 64, 16, 131071), // Vb, shortest
        _ => panic!("MIRITH_MODE not implemented!"),
    }
}

type ParamsTuple = (
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
    usize,
);

const INIT: ParamsTuple = init(MODE);
pub const PAR_Q: usize = INIT.0;
pub const PAR_M: usize = INIT.1;
pub const PAR_N: usize = INIT.2;
pub const PAR_K: usize = INIT.3;
pub const PAR_R: usize = INIT.4;
pub const PAR_S: usize = INIT.5;
pub const TAU: usize = INIT.6;
pub const N_PARTIES: usize = INIT.7;
pub const D_DIMENSION: usize = INIT.8;
pub const N_PARTIES_ROUND: usize = INIT.9;
pub const SEED_SIZE: usize = INIT.10;
pub const HASH_SIZE: usize = INIT.11;
pub const TREE_HEIGHT: usize = INIT.12;
pub const TREE_N_NODES: usize = INIT.13;
