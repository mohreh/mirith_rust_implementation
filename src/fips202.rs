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
const NROUNDS: usize = 24;

fn rol(a: u64, offset: u64) -> u64 {
    (a << offset) ^ (a >> (64 - offset))
}

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

    fn keccak_f1600_state_permute(&mut self) -> &mut Self {
        //copyFromState(A, state)
        let mut aba = self.state[0];
        let mut abe = self.state[1];
        let mut abi = self.state[2];
        let mut abo = self.state[3];
        let mut abu = self.state[4];
        let mut aga = self.state[5];
        let mut age = self.state[6];
        let mut agi = self.state[7];
        let mut ago = self.state[8];
        let mut agu = self.state[9];
        let mut aka = self.state[10];
        let mut ake = self.state[11];
        let mut aki = self.state[12];
        let mut ako = self.state[13];
        let mut aku = self.state[14];
        let mut ama = self.state[15];
        let mut ame = self.state[16];
        let mut ami = self.state[17];
        let mut amo = self.state[18];
        let mut amu = self.state[19];
        let mut asa = self.state[20];
        let mut ase = self.state[21];
        let mut asi = self.state[22];
        let mut aso = self.state[23];
        let mut asu = self.state[24];

        for round in (0..NROUNDS).step_by(2) {
            // prepareTheta
            let mut bca = aba ^ aga ^ aka ^ ama ^ asa;
            let mut bce = abe ^ age ^ ake ^ ame ^ ase;
            let mut bci = abi ^ agi ^ aki ^ ami ^ asi;
            let mut bco = abo ^ ago ^ ako ^ amo ^ aso;
            let mut bcu = abu ^ agu ^ aku ^ amu ^ asu;

            //thetaRhoPiChiIotaPrepareTheta(round  , A, E)
            let mut da = bcu ^ rol(bce, 1);
            let mut de = bca ^ rol(bci, 1);
            let mut di = bce ^ rol(bco, 1);
            let mut d_o = bci ^ rol(bcu, 1);
            let mut du = bco ^ rol(bca, 1);

            aba ^= da;
            bca = aba;
            age ^= de;
            bce = rol(age, 44);
            aki ^= di;
            bci = rol(aki, 43);
            amo ^= d_o;
            bco = rol(amo, 21);
            asu ^= du;
            bcu = rol(asu, 14);
            let mut eba = bca ^ ((!bce) & bci);
            eba ^= KECCAK_F_ROUND_CONSTANTS[round];
            let mut ebe = bce ^ ((!bci) & bco);
            let mut ebi = bci ^ ((!bco) & bcu);
            let mut ebo = bco ^ ((!bcu) & bca);
            let mut ebu = bcu ^ ((!bca) & bce);

            abo ^= d_o;
            bca = rol(abo, 28);
            agu ^= du;
            bce = rol(agu, 20);
            aka ^= da;
            bci = rol(aka, 3);
            ame ^= de;
            bco = rol(ame, 45);
            asi ^= di;
            bcu = rol(asi, 61);
            let mut ega = bca ^ ((!bce) & bci);
            let mut ege = bce ^ ((!bci) & bco);
            let mut egi = bci ^ ((!bco) & bcu);
            let mut ego = bco ^ ((!bcu) & bca);
            let mut egu = bcu ^ ((!bca) & bce);

            abe ^= de;
            bca = rol(abe, 1);
            agi ^= di;
            bce = rol(agi, 6);
            ako ^= d_o;
            bci = rol(ako, 25);
            amu ^= du;
            bco = rol(amu, 8);
            asa ^= da;
            bcu = rol(asa, 18);
            let mut eka = bca ^ ((!bce) & bci);
            let mut eke = bce ^ ((!bci) & bco);
            let mut eki = bci ^ ((!bco) & bcu);
            let mut eko = bco ^ ((!bcu) & bca);
            let mut eku = bcu ^ ((!bca) & bce);

            abu ^= du;
            bca = rol(abu, 27);
            aga ^= da;
            bce = rol(aga, 36);
            ake ^= de;
            bci = rol(ake, 10);
            ami ^= di;
            bco = rol(ami, 15);
            aso ^= d_o;
            bcu = rol(aso, 56);
            let mut ema = bca ^ ((!bce) & bci);
            let mut eme = bce ^ ((!bci) & bco);
            let mut emi = bci ^ ((!bco) & bcu);
            let mut emo = bco ^ ((!bcu) & bca);
            let mut emu = bcu ^ ((!bca) & bce);

            abi ^= di;
            bca = rol(abi, 62);
            ago ^= d_o;
            bce = rol(ago, 55);
            aku ^= du;
            bci = rol(aku, 39);
            ama ^= da;
            bco = rol(ama, 41);
            ase ^= de;
            bcu = rol(ase, 2);
            let mut esa = bca ^ ((!bce) & bci);
            let mut ese = bce ^ ((!bci) & bco);
            let mut esi = bci ^ ((!bco) & bcu);
            let mut eso = bco ^ ((!bcu) & bca);
            let mut esu = bcu ^ ((!bca) & bce);

            //  prepareTheta
            bca = eba ^ ega ^ eka ^ ema ^ esa;
            bce = ebe ^ ege ^ eke ^ eme ^ ese;
            bci = ebi ^ egi ^ eki ^ emi ^ esi;
            bco = ebo ^ ego ^ eko ^ emo ^ eso;
            bcu = ebu ^ egu ^ eku ^ emu ^ esu;

            //thetaRhoPiChiIotaPrepareTheta(round+1, E, A)
            da = bcu ^ rol(bce, 1);
            de = bca ^ rol(bci, 1);
            di = bce ^ rol(bco, 1);
            d_o = bci ^ rol(bcu, 1);
            du = bco ^ rol(bca, 1);

            eba ^= da;
            bca = eba;
            ege ^= de;
            bce = rol(ege, 44);
            eki ^= di;
            bci = rol(eki, 43);
            emo ^= d_o;
            bco = rol(emo, 21);
            esu ^= du;
            bcu = rol(esu, 14);
            aba = bca ^ ((!bce) & bci);
            aba ^= KECCAK_F_ROUND_CONSTANTS[round + 1];
            abe = bce ^ ((!bci) & bco);
            abi = bci ^ ((!bco) & bcu);
            abo = bco ^ ((!bcu) & bca);
            abu = bcu ^ ((!bca) & bce);

            ebo ^= d_o;
            bca = rol(ebo, 28);
            egu ^= du;
            bce = rol(egu, 20);
            eka ^= da;
            bci = rol(eka, 3);
            eme ^= de;
            bco = rol(eme, 45);
            esi ^= di;
            bcu = rol(esi, 61);
            aga = bca ^ ((!bce) & bci);
            age = bce ^ ((!bci) & bco);
            agi = bci ^ ((!bco) & bcu);
            ago = bco ^ ((!bcu) & bca);
            agu = bcu ^ ((!bca) & bce);

            ebe ^= de;
            bca = rol(ebe, 1);
            egi ^= di;
            bce = rol(egi, 6);
            eko ^= d_o;
            bci = rol(eko, 25);
            emu ^= du;
            bco = rol(emu, 8);
            esa ^= da;
            bcu = rol(esa, 18);
            aka = bca ^ ((!bce) & bci);
            ake = bce ^ ((!bci) & bco);
            aki = bci ^ ((!bco) & bcu);
            ako = bco ^ ((!bcu) & bca);
            aku = bcu ^ ((!bca) & bce);

            ebu ^= du;
            bca = rol(ebu, 27);
            ega ^= da;
            bce = rol(ega, 36);
            eke ^= de;
            bci = rol(eke, 10);
            emi ^= di;
            bco = rol(emi, 15);
            eso ^= d_o;
            bcu = rol(eso, 56);
            ama = bca ^ ((!bce) & bci);
            ame = bce ^ ((!bci) & bco);
            ami = bci ^ ((!bco) & bcu);
            amo = bco ^ ((!bcu) & bca);
            amu = bcu ^ ((!bca) & bce);

            ebi ^= di;
            bca = rol(ebi, 62);
            ego ^= d_o;
            bce = rol(ego, 55);
            eku ^= du;
            bci = rol(eku, 39);
            ema ^= da;
            bco = rol(ema, 41);
            ese ^= de;
            bcu = rol(ese, 2);
            asa = bca ^ ((!bce) & bci);
            ase = bce ^ ((!bci) & bco);
            asi = bci ^ ((!bco) & bcu);
            aso = bco ^ ((!bcu) & bca);
            asu = bcu ^ ((!bca) & bce);
        }

        self.state[0] = aba;
        self.state[1] = abe;
        self.state[2] = abi;
        self.state[3] = abo;
        self.state[4] = abu;
        self.state[5] = aga;
        self.state[6] = age;
        self.state[7] = agi;
        self.state[8] = ago;
        self.state[9] = agu;
        self.state[10] = aka;
        self.state[11] = ake;
        self.state[12] = aki;
        self.state[13] = ako;
        self.state[14] = aku;
        self.state[15] = ama;
        self.state[16] = ame;
        self.state[17] = ami;
        self.state[18] = amo;
        self.state[19] = amu;
        self.state[20] = asa;
        self.state[21] = ase;
        self.state[22] = asi;
        self.state[23] = aso;
        self.state[24] = asu;

        self

        // for round in &KECCAK_F_ROUND_CONSTANTS {
        //     // Theta step
        //     let mut c = [0u64; 5];
        //     for (x, c_item) in c.iter_mut().enumerate() {
        //         *c_item = self.state[x]
        //             ^ self.state[x + 5]
        //             ^ self.state[x + 10]
        //             ^ self.state[x + 15]
        //             ^ self.state[x + 20];
        //     }
        //
        //     let mut d = [0u64; 5];
        //     for x in 0..5 {
        //         d[x] = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
        //     }
        //
        //     for (i, d_item) in d.iter().enumerate() {
        //         for j in 0..5 {
        //             self.state[i + 5 * j] ^= *d_item;
        //         }
        //     }
        //
        //     // Rho and Pi steps
        //     let mut b = [0u64; 25];
        //     for x in 0..5 {
        //         for y in 0..5 {
        //             let index = x + 5 * y;
        //             b[PI_INDEXES[index]] = self.state[index].rotate_left(RHO_OFFSETS[index] as u32);
        //         }
        //     }
        //
        //     // Chi step
        //     for y in 0..5 {
        //         for x in 0..5 {
        //             self.state[x + 5 * y] =
        //                 b[x + 5 * y] ^ ((!b[(x + 1) % 5 + 5 * y]) & b[(x + 2) % 5 + 5 * y]);
        //         }
        //     }
        //
        //     // Iota step
        //     self.state[0] ^= round
        // }
        // self
    }

    fn keccak_squeeze(&mut self, out: &mut [u8], mut outlen: usize) -> usize {
        let mut idx = 0;
        while outlen > 0 {
            if self.pos == self.rate {
                self.keccak_f1600_state_permute();
                self.pos = 0;
            }
            let mut i = self.pos;
            let mut w = i / 8;
            while i < self.rate && i < self.pos + outlen {
                store64(&mut out[idx..], self.state[w]);
                i += 8;
                w += 1;
                idx += 8;
            }
            outlen -= i - self.pos;
            self.pos = i;
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
