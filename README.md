# Cryptopals challenges

My solutions to a collection of cryptography exercises from [cryptopals.com](https://www.cryptopals.com). 

# Progress:

## Set 1: Basics
  1. Convert hex to base64 - done.
  2. Fixed xor - done. 
  3. Single-byte xor cipher - this one has been a hassle. Maybe I'll finish it someday.
  4. Detect single byte xor - depends on the previous exercise.
  5. Implement repeating-key xor (vigenere) - done.
  6. Break repeating-key xor - depends on 3.
  7. AES in ECB mode
  8. Detect AES in ECB mode

## Set 2: Block crypto
  9. Implement PKCS#7 padding
  10. Implement CBC mode
  11. An ECB/CBC detection oracle
  12. Byte-at-a-time ECB decryption (simple)
  13. ECB cut-and-paste
  14. Byte-at-a-time ECB decryption (harder)
  15. PKCS#7 padding validation
  16. CBC bitflipping attacks

## Set 3: Block and stream crypto
  17. The CBC padding oracle
  18. Implement CTR, the stream cipher mode
  19. Break fixed-nonce CTR mode using substitutions
  20. Break fixed-nonce CTR statistically
  21. Implement the MT19937 Mersenne Twister RNG
  22. Crack an MT19937 seed
  23. Clone an MT19937 RNG from its output 
  24. Create the MT19937 stream cipher and break it

## Set 4. Stream crypto and randomness
  25. Break "random access read/write" AES CTR
  26. CTR bitflipping
  27. Recover the key from CBC with IV=key
  28. Implement a SHA-1 keyed MAC
  29. Break a SHA-1 keyed MAC using length extension
  30. Break an MD4 keyed MAC using length extension 
  31. Implement and break HMAC-SHA1 with an artificial timing leak
  32. Break HMAC-SHA1 with a slightly less artificial timing leak
  
## Set 5. Diffie-Hellman and friends 
  33. Implement DH 
  34. Implement a MITM key-fixing attack on DH with parameter injection
  35. Implement DH with negotiated groups, and break it with malicious 'g' parameters
  36. Implement Secure Remote Password (SRP) 
  37. Break SRP with a zero key
  38. Offline dictionary attack on simplified SRP 
  39. Implement RSA
  40. Implement an E=3 RSA broadcast attack

## Set 6. RSA and DSA
  41. Implement unpadded message recovery oracle 
  42. Bleichenbacher's e=3 RSA attack
  43. DSA key recovery from nonce
  44. DSA nonce recovery from repeated nonce
  45. DSA parameter tampering
  46. RSA parity oracle 
  47. Bleichenbacher's PKCS 1.5 padding oracle (simple case)
  48. Bleichenbacher's PKCS 1.5 padding oracle (complete case) 

## Set 7. Hashes
  49. CBC-MAC message forgery
  50. Hashing with CBC-MAC
  51. Compression ratio side-channel attacks
  52. Iterated hash function multicollisions 
  53. Kelsey and Schneier's expandable messages
  54. Kelsey and Kohno's nostradamus attack
  55. MD4 collisions
  56. RC4 single-byte biases

## Set 8. Abstract algebra
  57. DH revisited: small subgroup confinement
  58. Pollard's method for catching kangaroos
  59. Elliptic curve DH and invalid-curve attacks 
  60. Single-coordinate ladders and insecure twists 
  61. Duplicate-signature key selection in ECDSA /RSA
  62. Key-recovery attacks on ECDSA with biased nonces
  63. Key-recovery attacks on GCM with repeated nonces 
  64. Key-recovery attacks on GCM with a truncated MAC
  65. Truncated-MAC GCM revisited: improving via ciphertext length extension
  66. Exploiting implementation errors in DH 

