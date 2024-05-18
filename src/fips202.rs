const KECCAK_F_ROUND_CONSTANTS: [u64; 24] = [
    0x0000000000000001,
    0x0000000000008082,
    0x800000000000808A,
    0x8000000080008000,
    0x000000000000808B,
    0x0000000080000001,
    0x8000000080008081,
    0x8000000000008009,
    0x000000000000008A,
    0x0000000000000088,
    0x0000000080008009,
    0x000000008000000A,
    0x000000008000808B,
    0x800000000000008B,
    0x8000000000008089,
    0x8000000000008003,
    0x8000000000008002,
    0x8000000000000080,
    0x000000000000800A,
    0x800000008000000A,
    0x8000000080008081,
    0x8000000000008080,
    0x0000000080000001,
    0x8000000080008008,
];
const RHO_OFFSETS: [usize; 25] = [
    0, 1, 62, 28, 27, 36, 44, 6, 55, 20, 3, 10, 43, 25, 39, 41, 45, 15, 21, 8, 18, 2, 61, 56, 14,
];
const PI_INDEXES: [usize; 25] = [
    0, 10, 20, 5, 15, 16, 1, 11, 21, 6, 7, 17, 2, 12, 22, 23, 8, 18, 3, 13, 14, 24, 9, 19, 4,
];
const SHAKE128_RATE: usize = 168;
const SHAKE256_RATE: usize = 136;
const SHA3_256_RATE: usize = 136;
const SHA3_512_RATE: usize = 72;

fn load64(x: &[u8]) -> u64 {
    x.iter()
        .take(8)
        .fold(0, |acc, &byte| acc << 8 | byte as u64)
}

fn store64(x: &mut [u8], u: u64) {
    for (i, byte) in x.iter_mut().enumerate().take(8) {
        *byte = (u >> (8 * i)) as u8;
    }
}

#[derive(Default, Debug)]
#[allow(dead_code)]
struct KeccakState {
    state: [u64; 25],
    pos: usize,
    rate: usize,
}

#[allow(dead_code)]
impl KeccakState {
    fn new(rate: usize) -> Self {
        Self {
            rate,
            ..Default::default()
        }
    }

    fn keccak_absorb(&mut self, in_bytes: &[u8]) -> usize {
        let mut inlen = in_bytes.len();
        while self.pos + inlen >= self.rate {
            for i in self.pos..self.rate {
                self.state[i / 8] ^= (in_bytes[i - self.pos] as u64) << (8 * (i % 8));
            }
            inlen -= self.rate - self.pos;
            self.keccak_f1600_state_permute();
            self.pos = 0;
        }
        for i in self.pos..(self.pos + inlen) {
            self.state[i / 8] ^= (in_bytes[i - self.pos] as u64) << (8 * (i % 8));
        }
        self.pos + inlen
    }

    fn keccak_absorb_once(&mut self, mut in_bytes: &[u8], mut inlen: usize, p: u8) -> &mut Self {
        while inlen >= self.rate {
            for i in 0..self.rate / 8 {
                self.state[i] ^= load64(&in_bytes[8 * i..8 * i + 8]);
            }
            in_bytes = &in_bytes[self.rate..];
            inlen -= self.rate;
            self.keccak_f1600_state_permute();
        }
        for (i, by) in in_bytes.iter().enumerate().take(inlen) {
            self.state[i / 8] ^= (*by as u64) << (8 * (i % 8));
        }
        self.state[inlen / 8] ^= (p as u64) << (8 * (inlen % 8));
        self.state[(self.rate - 1) / 8] ^= 1u64 << 63;
        self.pos = self.rate;
        self
    }

    #[allow(unused_assignments)]
    #[allow(unused_variables)]
    fn keccak_f1600_state_permute(&mut self) -> &mut Self {
        for round in &KECCAK_F_ROUND_CONSTANTS {
            // Theta step
            let mut c = [0u64; 5];
            for (x, c_item) in c.iter_mut().enumerate() {
                *c_item = self.state[x]
                    ^ self.state[x + 5]
                    ^ self.state[x + 10]
                    ^ self.state[x + 15]
                    ^ self.state[x + 20];
            }

            let mut d = [0u64; 5];
            for x in 0..5 {
                d[x] = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
            }

            for (i, d_item) in d.iter().enumerate() {
                for j in 0..5 {
                    self.state[i + 5 * j] ^= *d_item;
                }
            }

            // Rho and Pi steps
            let mut b = [0u64; 25];
            for x in 0..5 {
                for y in 0..5 {
                    let index = x + 5 * y;
                    b[PI_INDEXES[index]] = self.state[index].rotate_left(RHO_OFFSETS[index] as u32);
                }
            }

            // Chi step
            for y in 0..5 {
                for x in 0..5 {
                    self.state[x + 5 * y] =
                        b[x + 5 * y] ^ ((!b[(x + 1) % 5 + 5 * y]) & b[(x + 2) % 5 + 5 * y]);
                }
            }

            // Iota step
            self.state[0] ^= round
        }
        self
    }

    fn keccak_squeeze(&mut self, out: &mut [u8], mut outlen: usize) -> usize {
        while outlen > 0 {
            if self.pos == self.rate {
                self.keccak_f1600_state_permute();
                self.pos = 0;
            }
            for i in self.pos..std::cmp::min(self.rate, self.pos + outlen) {
                out[i - self.pos] = (self.state[i / 8] >> (8 * (i % 8))) as u8;
            }
            outlen -= std::cmp::min(self.rate, self.pos + outlen) - self.pos;
            self.pos = std::cmp::min(self.rate, self.pos + outlen);
        }
        self.pos
    }

    fn keccak_squeezeblocks(&mut self, mut out: &mut [u8], mut nblocks: usize) -> &mut Self {
        while nblocks > 0 {
            self.keccak_f1600_state_permute();
            for i in 0..self.rate / 8 {
                store64(&mut out[8 * i..8 * i + 8], self.state[i]);
            }
            out = &mut out[self.rate..];
            nblocks -= 1;
        }
        self
    }
}

pub fn shake128(mut out: &mut [u8], in_bytes: &[u8]) {
    let (inlen, mut outlen) = (in_bytes.len(), out.len());
    let nblocks = outlen / SHAKE128_RATE;
    outlen -= nblocks * SHAKE128_RATE;
    out = &mut out[nblocks * SHAKE128_RATE..];

    KeccakState::new(SHAKE128_RATE)
        .keccak_absorb_once(in_bytes, inlen, 0x1F)
        .keccak_squeezeblocks(out, nblocks)
        .keccak_squeeze(out, outlen);
}

pub fn shake256(mut out: &mut [u8], in_bytes: &[u8]) {
    let (inlen, mut outlen) = (in_bytes.len(), out.len());
    let nblocks = outlen / SHAKE256_RATE;
    outlen -= nblocks * SHAKE256_RATE;
    out = &mut out[nblocks * SHAKE256_RATE..];
    KeccakState::new(SHAKE256_RATE)
        .keccak_absorb_once(in_bytes, inlen, 0x1F)
        .keccak_squeezeblocks(out, nblocks)
        .keccak_squeeze(out, outlen);
}

pub fn sha3_256(h: &mut [u8; 32], in_bytes: &[u8]) {
    let inlen = in_bytes.len();
    let mut state = KeccakState::new(SHA3_256_RATE);
    state
        .keccak_absorb_once(in_bytes, inlen, 0x06)
        .keccak_f1600_state_permute();

    for i in 0..4 {
        store64(&mut h[8 * i..8 * i + 8], state.state[i]);
    }
}

pub fn sha3_512(h: &mut [u8; 64], in_bytes: &[u8]) {
    let inlen = in_bytes.len();
    let mut state = KeccakState::new(SHA3_512_RATE);
    state
        .keccak_absorb_once(in_bytes, inlen, 0x06)
        .keccak_f1600_state_permute();
    for i in 0..8 {
        store64(&mut h[8 * i..8 * i + 8], state.state[i]);
    }
}
