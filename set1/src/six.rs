
// 6. BREAK REPEATING KEY XOR. File break-repeat-xor.txt has been base64'd after encrypted with
//      repeating key XOR. Method to break:
//
//      a. let KEYSIZE be guessed length of key. Try values 2 to 40 (inclusive).
//
//      b. write a function to compute hamming distance (number of differing bits). The distance
//          between "this is a test" and "wokka wokka!!!" is 37.

///hamming/flip distance between two texts of the same length. Far from optimized.
fn hamming_dist(a: Vec<u8>, b: Vec<u8>) -> usize {
    assert!((&a).len() == (&b).len());
    a.into_iter()
        .zip(b.into_iter())
        .map(|(a_byte, b_byte)| a_byte ^ b_byte)
        .map(|xored_byte| xored_byte.count_ones() as usize)
        .sum()
}

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

#[cfg(test)]
mod test_six {
    use super::*;

    //hamming_dist consts
    const A: &str = "this is a test";
    const B: &str = "wokka wokka!!!";

    #[test]
    fn test_hamming_dist() {
        assert_eq!(
            hamming_dist(A.as_bytes().to_vec(), B.as_bytes().to_vec()),
            37
        );
        let vec1: Vec<u8> = vec![0b0000];
        let vec2: Vec<u8> = vec![0b1111];
        assert_eq!(hamming_dist(vec1, vec2), 4);
    }
}

