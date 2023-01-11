use hex;
use std::error::Error;
use std::io;

pub use crate::two;

// 3. decode single-byte XOR cipher: the hex encoded string has been xor'd
// against a single character. find key, decrypt message.
// method: make function to score english plaintext. character frequency
// is good metric.
//

//--------------------------------------------------------------------------------------------
//constants
//
pub const STRING_3: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

//ascii characters that I think the plaintext might contain
const ALPHABET: [u8; 94] = [
    b'!', b'"', b'#', b'$', b'%', b'&', b'\'', b'(', b')', b'*', b'+', b',', b'-', b'.', b'/',
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b':', b';', b'<', b'=', b'>', b'?',
    b'@', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O',
    b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'[', b'\\', b']', b'^',
    b'_', b'`', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
    b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'{', b'|', b'}', b'~',
];

//---------------------------------------------------------------------------------------------
//functions
pub fn initialize_frequency_map() -> [f64; 256] {
    let mut pairmap: [(u8, f64); 94] = [
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
    ];

    let mut returnmap: [f64; 256] = [0.0; 256];

    for (byte, freq) in pairmap {
        returnmap[usize::from(byte)] = freq;
    }

    returnmap
}

pub fn break_single_byte_xor(
    ciphertext: Vec<u8>,
    freqmap: &[f64; 256],
) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    //check ciphertext is nonempty
    if ciphertext.len() == 0 {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "Error: text is empty",
        )));
    }

    //create list of probable plaintexts
    let mut probable_plaintexts: Vec<Vec<u8>> = Vec::new();
    let ciphertext_length = (&ciphertext).len();
    for byte in ALPHABET {
        let single_byte_extended: Vec<u8> = vec![byte; ciphertext_length];
        let candidate_plaintext: Vec<u8> = two::fixed_xor(&ciphertext, &single_byte_extended);
        probable_plaintexts.push(candidate_plaintext);
    }

    //score 'em
    let mut plaintexts_ranked: Vec<(Vec<u8>, f64)> = Vec::new();
    for pt in probable_plaintexts {
        plaintexts_ranked.push((pt.clone(), score_english_plaintext(pt, &freqmap)));
    }

    //pick the n with the lowest score
    let lowest = lowest_n(plaintexts_ranked, 25);

    Ok(lowest)
}

///returns the n plaintexts with the lowest score
fn lowest_n(mut scored_plaintexts: Vec<(Vec<u8>, f64)>, n: usize) -> Vec<Vec<u8>> {
    let mut lowest_n: Vec<Vec<u8>> = Vec::new();
    for i in 0..n {
        //get pt closest to english in scored_plaintexts
        let lowest: Vec<u8> = scored_plaintexts
            .iter()
            .fold((Vec::new(), 0.0), |acc, (pt, score)| {
                if score > &acc.1 {
                    ((*pt).clone(), *score)
                } else {
                    acc
                }
            })
            .0;

        //remove said plaintext from scored_plaintexts
        scored_plaintexts = scored_plaintexts
            .into_iter()
            .filter(|(pt, score)| *pt != lowest)
            .collect();

        //add said plaintext to lowest_three
        lowest_n.push(lowest);
    }
    lowest_n
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
fn score_english_plaintext(plaintext: Vec<u8>, freqmap: &(&[f64; 256])) -> f64 {
    let pt_freqmap: [f64; 256] = countmap_to_percentmap(countmap(plaintext.clone()));

    //for indicies where either pt_freqmap or freqmap are nonzero, record dist
    let dists: Vec<f64> = (&pt_freqmap)
        .into_iter()
        .zip((*freqmap).iter())
        .map(|(pt_freq, freq)| dist(*pt_freq, *freq))
        .filter(|dist| *dist != 0.0)
        .collect();

    //compute avg of dists
    let len = (&dists).len() as f64;
    let avg = dists.into_iter().sum::<f64>() / len;

    avg
}

//tests: 1. tests all cases.
///Returns the euclidean distance between two floats
fn dist(a: f64, b: f64) -> f64 {
    if a == b {
        return 0.0;
    } else if a > b {
        return a - b;
    } else {
        return b - a;
    }
}

///given a map with keys and a f64 occurance count, gives the percent of each key
fn countmap_to_percentmap(countmap: [f64; 256]) -> [f64; 256] {
    let sum: f64 = countmap.iter().sum();
    countmap
        .iter()
        .map(|&count| count / sum * 100.0)
        .collect::<Vec<f64>>()
        .try_into()
        .expect("error converting vec to array in countmap_to_percentmap")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_countmap() {
        let mut expected: [f64; 256] = [0.0; 256];
        expected[0] = 1.0;
        expected[1] = 1.0;
        assert_eq!(countmap(vec![0 as u8, 1 as u8]), expected);
    }

    #[test]
    fn test_dist() {
        assert_eq!(dist(0.0, 0.0), 0.0, "a == b == 0.0");
        assert_eq!(dist(1.0, -1.0), 2.0, "a == 1.0, b == -1.0");
        assert_eq!(dist(-1.0, 2.0), 3.0, "a == -1.0, b == 2.0");
    }

    #[test]
    fn test_countmap_to_percentmap() {
        let countmap1 = [1.0; 256];
        let expected1 = [0.390625; 256];
        assert_eq!(countmap_to_percentmap(countmap1), expected1);
    }
}
