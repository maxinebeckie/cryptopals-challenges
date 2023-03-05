//! Implement a SHA1-keyed MAC
//!
//! 1. find a SHA1 implentation in the language you code in
//!
//! 2. write a function to authenticate a message under a secret key by using
//!     a secret-prefix MAC, which is SHA1( key || message) . (what does "||" mean??)
//! 
//! 3. Verify you cannot tamper with the message without breaking the MAC, and can't 
//!     produce a new MAC without knowing the key.
