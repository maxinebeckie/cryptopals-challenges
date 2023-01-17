// 33. Implement Diffie-Hellman
//
//      Set a variable "p" to 37 and "g" to 5.
//       Generate "a", a random number mod 37. Now generate "A", which is "g" raised to the "a" power mode 37 --- A = (g**a) % p.
//
//         Do the same for "b" and "B".
//      "A" and "B" are public keys. Generate a session key with them; set "s" to "B" raised to the "a" power mod 37 --- s = (B**a) % p.
//      Do the same with A**b, check that you come up with the same "s".
//      To turn "s" into a key, you can just hash it to create 128 bits of key material (or SHA256 it to create a key for encrypting and a key for a MAC).

fn diffie_hellman(p: u64, g: u64) -> () {
    todo!();
}

fn main() {
    let key = diffie_hellman(37, 5);
}
