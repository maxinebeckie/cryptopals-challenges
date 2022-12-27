// 1. Implement PKCS#7 padding. 
//
//  A block cipher transforms a fixed size block (usual 8, 16 bytes) of plaintext to ciphertext.
//  Message sizes are irregular, so we create padding, making a plaintext that is a multiple of
//  the blocksize. PKCS#7 is the most popular padding scheme. 
//
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



fn main() {

    println!("Hello, world!");
}
