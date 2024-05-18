use mirith::fips202::{sha3_256, sha3_512, shake128, shake256};

pub fn to_hex_string(bytes: Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    strs.join("")
}

fn main() {
    let in0 = "Hello World".to_string();
    let in0 = in0.as_bytes();
    let in1 = "Hello world".to_string();
    let in1 = in1.as_bytes();
    let in2 = "".to_string();
    let in2 = in2.as_bytes();
    let mut output0 = [0u8; 64];
    let mut output1 = [0u8; 64];
    let mut output2 = [0u8; 64];
    sha3_512(&mut output0, in0);
    sha3_512(&mut output1, in1);
    sha3_512(&mut output2, in2);
    println!("{:?}", to_hex_string(output0.to_vec()));
    println!("{:?}", to_hex_string(output1.to_vec()));
    println!("{:?}", to_hex_string(output2.to_vec()));
}
