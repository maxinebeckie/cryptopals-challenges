// 2. DONE fixed XOR: write a function that takes two buffers with equal length
// and produces their XOR.

//fixed_xor constants
const STRING1_2: &str = "1c0111001f010100061a024b53535009181c";
const STRING2_2: &str = "686974207468652062756c6c277320657965";
const STRING_RESULT_2: &str = "746865206b696420646f6e277420706c6179";

pub fn fixed_xor(bytes_one: &Vec<u8>, bytes_two: &Vec<u8>) -> Vec<u8> {
    assert!(bytes_one.len() == bytes_two.len());
    bytes_one
        .iter()
        .zip(bytes_two.iter())
        .map(|(x1, x2)| x1 ^ x2)
        .collect()
}

#[test]
fn test_fixed_xor() {
    assert_eq!(
        &hex::decode(STRING_RESULT_2).unwrap(),
        &fixed_xor(
            &hex::decode(STRING1_2).unwrap(),
            &hex::decode(STRING2_2).unwrap()
        )
    );
}
