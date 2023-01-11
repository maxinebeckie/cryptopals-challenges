use hex;
use std::io;

pub mod one;
pub mod two;
pub mod three;
pub mod four;
pub mod five;
pub mod six;
pub mod seven;
pub mod eight;

fn main() {
    //3 break single-byte xor

    let freqmap: [f64; 256] = three::initialize_frequency_map();
    let ciphertext: Vec<u8> = hex::decode(three::STRING_3).expect("error hex-decoding plaintext");
    let plaintexts =
        three::break_single_byte_xor(ciphertext, &freqmap).expect("error breaking single byte xor");
    for pt in plaintexts {
        println!("{}", std::str::from_utf8(&pt).unwrap());
    }
}
