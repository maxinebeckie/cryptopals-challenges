use base64;
use hex;

// 1. DONE convert hex to base64

fn hex_to_base64(hex: &str) -> String {
    let bytes = hex::decode(hex).unwrap();
    base64::encode(bytes)
}

// 2. DONE fixed XOR: write a function that takes two buffers with equal length
// and produces their XOR.

fn fixed_xor(bytes_one: Vec<u8>, bytes_two: Vec<u8>) -> Vec<u8> {
    bytes_one
        .iter()
        .zip(bytes_two.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect()
}

#[cfg(test)]
mod tests_small_crates {
    use super::*;

    //fixed_xor constants
    const STRING1_2: &str = "1c0111001f010100061a024b53535009181c";
    const STRING2_2: &str = "686974207468652062756c6c277320657965";
    const STRING_RESULT_2: &str = "746865206b696420646f6e277420706c6179";

    //hex to base64 constants
    const HEX_1: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    const BASE64_1: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    //repeating_xor constants
    const STANZA: &str = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
    const ANS_EXPECTED: &str = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    const KEY: &str = "ICE";
/*
    #[test]
    fn test_fixed_xor() {
        assert_eq!(STRING_RESULT_2, &fixed_xor(STRING1_2, STRING2_2));
    }

    #[test]
    fn test_hex_to_base64() {
        assert_eq!(BASE64_1, &hex_to_base64(HEX_1));
    }
*/
/*
    #[test]
    fn test_repeating_key_xor() {
        let plaintext = STANZA.as_bytes();
        let key = KEY.as_bytes();
        let ciphertext = repeating_key_xor(plaintext, key);
        assert_eq!(ciphertext, ANS_EXPECTED);
    }
    */
}

// 3. decode single-byte XOR cipher: the hex encoded string has been xor'd
// against a single character. find key, decrypt message.
// method: make function to score english plaintext. character frequency
// is good metric.
//
// ALRIGHT: it seems my work is cut out for me.
//
//  what I think the problem is: the bytes not listed in the freq hashmap do not contribute to
//  the running average. This leads to strings with lots of non-ASCII bytes having very few
//  data points in the count hashmap, which eventually messes up the average.
//
//      -- if this analysis is correct, initializing all possible bytes not in freq_map to 0
//          and operating on raw bytes only should fix this problem.
//
//  1. change all function signatures to raw bytes      -- done, I think.
//
//  2. change distribution hashmap to use raw bytes instead of chars.       -- done.
//
//  3. one by one, implement and test to make sure each little function is working properly.
//      refactor when needed.
//
//  4. put them together in the break function
//
//

mod three {

    use crate::fixed_xor;
    use std::collections::HashMap;
    use std::error::Error;
    use std::io;

    //--------------------------------------------------------------------------------------------
    //constants
    //
    const STRING_3: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    //ascii characters that I think the plaintext might contain
    const ALPHABET: [u8; 94] = [
        b'!', b'"', b'#', b'$', b'%', b'&', b'\'', b'(', b')', b'*', b'+', b',', b'-', b'.', b'/',
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b':', b';', b'<', b'=', b'>',
        b'?', b'@', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M',
        b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'[', b'\\',
        b']', b'^', b'_', b'`', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k',
        b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
        b'{', b'|', b'}', b'~',
    ];

    //static FREQMAP: HashMap<u8, f64> = initialize_frequency_map();

    //todo: make the rest of this compile, print out the pairs (byte, 0.0) for all the bytes not in
    //ALPHABET, then copy that to be hardcoded into a const FREQMAP: [(u8, f64); 256] value.

    //---------------------------------------------------------------------------------------------
    //functions

    ///encrypt/decrypt a text with single byte xor. Error for empty text.
    fn single_byte_xor(text: Vec<u8>, key: u8) -> Result<Vec<u8>, std::io::Error> {
        let length: usize = text.len();
        if length == 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "Error: text is empty"));
        }
        let key_lengthened = (0..length).map(|_| key).collect();
        Ok(fixed_xor(text, key_lengthened))
    }

    ///decrypts single byte xor, returns the most likely plaintext.
    pub fn break_single_byte_xor(ciphertext: Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
        //check ciphertext is nonempty
        if ciphertext.len() == 0 {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Error: text is empty",
            )));
        }
/*
        //initialize the hashmap with (candidate_PT, score)
        let candidate_scoremap: HashMap<Vec<u8>, f64> = (0..256)
            .map(|byte| single_byte_xor(ciphertext, byte).unwrap())
            .map(|candidate_plaintext| {
                (
                    candidate_plaintext,
                    score_english_plaintext(candidate_plaintext),
                )
            })
            .collect();
*/
        //return the key with the smallest value
        todo!();
    }

    //tests: 1.
    ///counts the occurances of each byte. Returns array where index = byte, val = count.
    fn countmap(text: Vec<u8>) -> [f64; 256] {
        let mut map = [0.0; 256];
        for byte in text {
            let mut curr_count = &mut map[usize::from(byte)];
            *curr_count += 1.0;
        }
        map
    }

    ///Smaller number => more likely to be english text
    fn score_english_plaintext(plaintext: Vec<u8>) -> f64 {
        let pt_freq_map = countmap_to_percentmap(countmap(plaintext));
        todo!();    
    }

    //tests: 1. tests all cases. 
    ///Returns the euclidean distance between two floats
    fn dist(a: &f64, b: &f64) -> f64 {
        if *a == *b {
            return 0.0;
        } else if *a > *b {
            return *a - *b;
        } else {
            return *b - *a;
        }
    }

    //tests: 1
    ///given a map with keys and a f64 occurance count, gives the percent of each key
    fn countmap_to_percentmap(countmap: [f64; 256]) -> [f64; 256] {
        let sum: f64 = countmap
            .iter()
            .sum();
        countmap
            .iter()
            .map(|&count| count / sum * 100.0)
            .collect::<Vec<f64>>()
            .try_into()
            .expect("error converting vec to array in countmap_to_percentmap")
    }
/*
    fn initialize_frequency_map() -> HashMap<u8, f64> {
        let mut map: HashMap<u8, f64> = HashMap::from([
            (b'!', 0.02),
            (b'"', 0.16),
            (b'#', 0.05),
            (b'$', 0.05),
            (b'%', 0.02),
            (b'&', 0.01),
            (b'\'', 0.17),
            (b'(', 0.31),
            (b')', 0.31),
            (b'*', 0.04),
            (b'+', 0.05),
            (b',', 1.14),
            (b'-', 0.56),
            (b'.', 2.31),
            (b'/', 0.29),
            (b'0', 0.83),
            (b'1', 0.67),
            (b'2', 0.48),
            (b'3', 0.34),
            (b'4', 0.29),
            (b'5', 0.26),
            (b'6', 0.24),
            (b'7', 0.19),
            (b'8', 0.22),
            (b'9', 0.18),
            (b':', 0.41),
            (b';', 0.07),
            (b'<', 0.04),
            (b'=', 0.15),
            (b'>', 0.06),
            (b'?', 0.02),
            (b'@', 0.01),
            (b'A', 0.41),
            (b'B', 0.19),
            (b'C', 0.34),
            (b'D', 0.24),
            (b'E', 0.28),
            (b'F', 0.23),
            (b'G', 0.11),
            (b'H', 0.13),
            (b'I', 0.43),
            (b'J', 0.04),
            (b'K', 0.06),
            (b'L', 0.20),
            (b'M', 0.20),
            (b'N', 0.21),
            (b'O', 0.20),
            (b'P', 0.37),
            (b'Q', 0.02),
            (b'R', 0.23),
            (b'S', 0.45),
            (b'T', 0.52),
            (b'U', 0.12),
            (b'V', 0.07),
            (b'W', 0.14),
            (b'X', 0.04),
            (b'Y', 0.06),
            (b'Z', 0.02),
            (b'[', 0.09),
            (b'\\', 0.05),
            (b']', 0.09),
            (b'^', 0.00),
            (b'_', 0.24),
            (b'`', 0.00),
            (b'a', 6.47),
            (b'b', 1.19),
            (b'c', 3.34),
            (b'd', 2.95),
            (b'e', 10.45),
            (b'f', 1.73),
            (b'g', 1.65),
            (b'h', 3.33),
            (b'i', 6.14),
            (b'j', 0.10),
            (b'k', 0.76),
            (b'l', 3.41),
            (b'm', 2.06),
            (b'n', 5.96),
            (b'o', 6.44),
            (b'p', 2.16),
            (b'q', 0.13),
            (b'r', 5.42),
            (b's', 5.83),
            (b't', 8.12),
            (b'u', 2.49),
            (b'v', 0.88),
            (b'w', 1.27),
            (b'x', 0.40),
            (b'y', 1.43),
            (b'z', 0.11),
            (b'{', 0.03),
            (b'|', 0.04),
            (b'}', 0.03),
            (b'~', 0.01),
        ]);

        //initialize the rest of the bytes to zero
        (0..256).map(|byte| {
            if !(&map.contains_key(&byte)) {
                &map.insert(byte, 0.0);
            }
        });

        map
    }
*/
    #[cfg(test)]
    mod tests {
        use super::*;
        use std::collections::HashMap;

/*
        #[test]
        fn test_countmap_to_percentmap() {
            let input: HashMap<u8, f64> = HashMap::from([('a', 1), ('b', 1)]);
            let expected = HashMap::from([('a', 50.0), ('b', 50.0)]);
            assert_eq!(expected, countmap_to_percentmap(input));
        }
*/
/*
        #[test]
        fn test_parse_frequency_abba() {
            let mut expected: HashMap<char, f64> = HashMap::new();
            for character in ALPHABET {
                expected.insert(character, 0.0);
            }
            let a_ptr: &mut f64 = expected.get_mut(&'a').unwrap();
            *a_ptr = 50.0;

            let b_ptr: &mut f64 = expected.get_mut(&'b').unwrap();
            *b_ptr = 50.0;

            let test_string = "abba".to_string();
            let result = parse_string(test_string);

            //assert_eq!(expected, result);
            assert_eq!(expected[&'a'], result[&'a']); //50.0
            assert_eq!(expected[&'b'], result[&'b']); //50.0
            assert_eq!(expected[&'c'], result[&'c']); //0.0
        }
*/
        #[test]
        fn test_countmap() {
            let mut expected: [f64; 256] = [0.0; 256];
            expected[0] = 1.0;
            expected[1] = 1.0;
            assert_eq!(countmap(vec![0 as u8, 1 as u8]), expected);
        }

        #[test]
        fn test_dist() {
            assert_eq!(dist(&0.0, &0.0), 0.0, "a == b == 0.0");
            assert_eq!(dist(&1.0, &-1.0), 2.0, "a == 1.0, b == -1.0");
            assert_eq!(dist(&-1.0, &2.0), 3.0, "a == -1.0, b == 2.0");
        }

        #[test]
        fn test_countmap_to_percentmap() {
            let countmap1 = [1.0; 256];
            let expected1 = [0.390625; 256]; 
            assert_eq!(countmap_to_percentmap(countmap1), expected1);
        }
    }
}
// 4. Detect single-character XOR. one of the 60-char strings in single-char-xor.txt has been
//    encrypted with single-character XOR. (use code from 3)
//
//          I'd imagine this could be done by just breaking each string, then using
//          parse_frequency(..) to see which one is most likely english.

// 5. Implement repeating key XOR. Encrypt STANZA under key "ICE" to get ANS_EXPECTED. Try
//    encrypting a bunch of random shit with repeating key XOR.
//
//    -- I think it's done but I haven't tested it because I'm refactoring 3 code before compiling.

/*
fn repeating_key_xor(plaintext: &Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    let mut key_letter_count = 0;
    let keysize = key.len();
    let mut ciphertext: Vec<u8> = Vec::new();

    for byte in *plaintext {
        ciphertext.push(byte ^ key[key_letter_count]);
        key_letter_count = (key_letter_count + 1) % keysize;
    }
    ciphertext
}
*/

// 6. BREAK REPEATING KEY XOR. File break-repeat-xor.txt has been base64'd after encrypted with
//      repeating key XOR. Method to break:
//
//      a. let KEYSIZE be guessed length of key. Try values 2 to 40 (inclusive).
//
//      b. write a function to compute hamming distance (number of differing bits). The distance
//          between "this is a test" and "wokka wokka!!!" is 37.
//
//      c. for each KEYSIZE take first KEYSIZE length of bytes, then second KEYSIZE length of
//          bytes, find hamming distance btwn them. Normalize by dividing by KEYSIZE.
//          the KEYSIZE with smallest normalized hamming distance is probably the size.
//          can vary like taking smallest 2-3 ones, or average 4 KEYSIZE blocks instead of 2
//          (presumably to increase variance? idk)
//
//      d. (for each KEYSIZE found in c) break ciphertext into blocks of KEYSIZE length.
//
//      e. transpose blocks: make a block that is first byte of every block, second byte of
//          every block .. KEYSIZE byte
//
//      f. solve each block as if it was single character XOR (from before)
//
//      g. for each block find the single-byte XOR key that gets best result from previous
//          scoring function is the repeating-key XOR byte for that block. put together
//          to get key.
//
//      note: this code is also called Vigenere.

// 7. Decrypt AES in ECB mode. The base64'd content in aes-ecb.txt has been encrypted via
//      AES-128 in ECB mode under the key "YELLOW SUBMARINE" (no quotes, 16 chars). easiest way:
//      use OpenSSL::Cipher with AES-128-ECB.

// 8. Detect AES in ECB mode. The file detect-aes-ecb.txt contains a bunch of hex-encoded
//    ciphertexts. One has been encrypted with ECB, detect which one.
//    hint: ECB is stateless and deterministic: the same 16 byte plaintext block will always
//    produce the same 16 byte ciphertext.

fn main() {
    //nothing here for now
}
