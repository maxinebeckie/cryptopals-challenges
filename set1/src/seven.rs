// 7. Decrypt AES in ECB mode. The base64'd content in aes-ecb.txt has been encrypted via
//      AES-128 in ECB mode under the key "YELLOW SUBMARINE" (no quotes, 16 chars). easiest way:
//      use OpenSSL::Cipher with AES-128-ECB.

use base64;

use openssl::cipher;

use std::env;
use std::fs;

const KEY: &str = "YELLOW SUBMARINE";

/// encrypts/decrypts the given bytes under the key in AES-128-ECB mode.
fn aes_128_ecb(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    // call aes_128_ecb() -> &'static CipherRef in openssl crate OR
    // call openssl directly OR implement myself 
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_aes_128_ecb() {
        let text: &[u8] = &fs::read_to_string("textfiles/aes-ecb.txt")
            .expect("couldn't read file")
            //(remember to base64 decode too)
            .as_bytes();
        // assert_eq!(aes_128_ecb(text, &key.as_bytes());
    }
}
