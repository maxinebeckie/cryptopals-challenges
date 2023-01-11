use base64;
use hex;

const HEX_1: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
const BASE64_1: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

// 1. DONE convert hex to base64

fn hex_to_base64(hex: &str) -> String {
    let bytes = hex::decode(hex).unwrap();
    base64::encode(bytes)
}

#[test]
fn test_hex_to_base64() {
    assert_eq!(BASE64_1, &hex_to_base64(HEX_1));
}
