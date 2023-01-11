// 5. Implement repeating key XOR. Encrypt STANZA under key "ICE" to get ANS_EXPECTED. Try
//    encrypting a bunch of random shit with repeating key XOR.
//
    const STANZA: &str = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
    const ANS_EXPECTED: &str = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    const KEY: &str = "ICE";
 
fn repeating_key_xor(plaintext: &Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    let mut key_letter_count = 0;
    let keysize = key.len();
    let mut ciphertext: Vec<u8> = Vec::new();

    for byte in plaintext {
        ciphertext.push(byte ^ key[key_letter_count]);
        key_letter_count = (key_letter_count + 1) % keysize;
    }
    ciphertext
}

     #[test]
    fn test_repeating_key_xor() {
        let plaintext = STANZA.as_bytes();
        let key = KEY.as_bytes();
        let ciphertext = repeating_key_xor(&plaintext.to_vec(), key.to_vec());
        assert_eq!(ciphertext, hex::decode(ANS_EXPECTED).unwrap());
    }

