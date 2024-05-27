const fn parse_u8(string: &str) -> u8 {
    let mut res: u8 = 0;
    let mut bytes = string.as_bytes();
    while let [byte, rest @ ..] = bytes {
        bytes = rest;
        if let b'0'..=b'9' = byte {
            res *= 10;
            res += *byte - b'0';
        } else {
            panic!("not a number")
        }
    }
    res
}

pub const MODE: u8 = parse_u8(std::env!("MIRITH_MODE"));

pub const CRYPTO_ALGNAME: &str = "MiRitH";
