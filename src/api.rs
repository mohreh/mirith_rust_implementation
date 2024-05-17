// Copyright 2023 Carlo Sanna, Javier Verbel, and Floyd Zweydinger.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::config::MIRITH_MODE;

// pub const CRYPTO_PUBLICKEYBYTES: usize;
// pub const CRYPTO_SECRETKEYBYTES: usize;
// pub const CRYPTO_BYTES: usize;

const fn init() -> (usize, usize, usize) {
    match MIRITH_MODE {
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

const INIT: (usize, usize, usize) = init();

pub const CRYPTO_PUBLICKEYBYTES: usize = INIT.0;
pub const CRYPTO_SECRETKEYBYTES: usize = INIT.1;
pub const CRYPTO_BYTES: usize = INIT.2;
