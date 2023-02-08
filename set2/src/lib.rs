// 1. Implement PKCS#7 padding.
//
//  A block cipher transforms a fixed size block (usual 8, 16 bytes) of plaintext to ciphertext.
//  Message sizes are irregular, so we create padding, making a plaintext that is a multiple of
//  the blocksize. PKCS#7 is the most popular padding scheme.

///pads bytes of plaintext to a multiple of blocksize bytes with \x04
fn pkcs7_padding(bytes: Vec<u8>, blocksize: u8) {
    todo!();
}

//  So: pad any block to a specific block length, by appending the number of bytes of padding
//  to the end of the block. For instance
//
//  "YELLOW SUBMARINE"
//
//  padded to 20 bytes becomes
//
//  "YELLOW SUBMARINE\x04\x04\x04\x04"

// 2. Implement CBC mode. ????
//
// CBC mode is a block cipher mode that allows us to encrypt irregularly-sized messages, despite
// the fact that block ciphers natively only transform individual blocks.
//
// In CBC mode, each ciphertext block is added to the next plaintext block before the call to the
// cipher core. ??
//
// The first PT block, with no associated previous CT block, is added to a fake 0th CT block called
// "the initialization vector", or "IV".
//
// Implement CBC mode by hand by taking the ECB function from set1, making it encrypt instead
// of decrypt (verify this), and using XOR function to combine them.
//
// The file 2.txt is intelligible when CBC decrypted against "YELLOW SUBMARINE" with IV of all
// ASCII 0 (\x00\x00\x00 &c)

// 3. An ECB/CBC detection oracle.
//
//  write a function to generate a random AES key (16 bytes).
//
//  write a function that encrypts data under an unknown key -- generates random key and
//  encrypts under it.
//
//  should look like:
//
//      encryption_oracle(your-input)
//      => [MEANINGLESS]
//
//  also, have function append 5-10 bytes (count chosen randomly) and 5-10 bytes after pliaintext.
//
//  now, have function encrypt in ECB half the time and CBC other half (random IVs each time for
//  CBC). use rand(2) to decide what to use. should end up with a function that when pointed
//  at a block (?black) box that is encrypting either ECB or CBC, tells you which one is happening.
//
//

// 4. Byte-at-a-time ECB decryption (simple)
//
//  Copy your oracle function to a new function that encrypts buffers under ECB mode using a consistent but unknown key (for instance, assign a single random key, once, to a global variable).
// Now take that same function and have it append to the plaintext, BEFORE ENCRYPTING, the following string:

const four_str: &str = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg
aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq
dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg
YnkK";

// Base64 decode the string before appending it. Do not base64 decode the string by hand; make your code do it. The point is that you don't know its contents.

// What you have now is a function that produces:
//
//  AES-128-ECB(your-string || unknown-string, random-key)
//
//   It turns out: you can decrypt "unknown-string" with repeated calls to the oracle function!

//Here's roughly how:

//  Feed identical bytes of your-string to the function 1 at a time --- start with 1 byte ("A"), then "AA", then "AAA" and so on. Discover the block size of the cipher. You know it, but do this step anyway.
// Detect that the function is using ECB. You already know, but do this step anyways.
// Knowing the block size, craft an input block that is exactly 1 byte short (for instance, if the block size is 8 bytes, make "AAAAAAA"). Think about what the oracle function is going to put in that last byte position.
//Make a dictionary of every possible last byte by feeding different strings to the oracle; for instance, "AAAAAAAA", "AAAAAAAB", "AAAAAAAC", remembering the first block of each invocation.
//Match the output of the one-byte-short input to one of the entries in your dictionary. You've now discovered the first byte of unknown-string.
// Repeat for the next byte.

fn main() {
    println!("Hello, world!");
}
