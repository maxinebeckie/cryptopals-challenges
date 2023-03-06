//! 57. Diffie-Hellman revisited: small subgroup confinement
//! 
//! DEPENDS: previous DH exercise. 
//!
//! First, some stuff with traditional DH.
//! 
//! First, build your typical Diffie-Hellman key agreement: Alice and Bob exchange public keys and derive the same
//! shared secret. Then Bob sends Alice some message with a MAC over it.
//!
//! Use the parameters P, G, Q as below. 

use num_bigint::BigUInt;

const P: BigUInt = 7199773997391911030609999317773941274322764333428698921736339643928346453700085358802973900485592910475480089726140708102474957429903531369589969318716771;
const G: BigUInt = 4565356397095740655436854503483826832136106141639563487732438195343690437606117828318042418238184896212352329118608100083187535033402010599512641674644143;

///the order of generator G
const Q: BigUInt = 236234353446506858198510045061214171961;

//! Notice that G^Q = 1 mod P and Q divides P - 1. 
//! Alice and Bob choose secret keys as random integers mod Q. There is no point in choosing mod P since
//! G^x mod P = g^(x + kQ) mod P, for any x, k. 
//!
//! The rest is the same as the previous DH exercise. 
//!
//! How can we attack this protocol? 
//! The fact that Q | P - 1 guarantees existence of elements of order Q. What if there are smaller divisors of 
//! P - 1? 
//! Choose j = (P - 1) / Q. Find these smaller factors by factoring j, as below. 

const J: BigUInt = 30477252323177606811760882179058908038824640750610513771646768011063128035873508507547741559514324673960576895059570;

//! Don't need to factor J all the way, just find a bunch of factors smaller than 2^16. Avoid repeated factors. 
//! We can use these factors to recover Bob's secret key using the Pohlig-Hellman alg for discrete logs. 
//!
//!     1. Take a small factor r of j. We need an element h of order r. We use the formula
//!         h = rand(1, p)^((p-1)/r) mod p
//!     until we get a nonunit h. 
//!
//!     2. You're Eve. Send Bob h as your public key. It is not a valid public key but Bob doesn't know that. 
//! 
//!     3. Bob computes K = h^x mod p, where x is Bob's secret key and K is the shared secret. Bob sends 
//!     (m, t) with m = "crazy flamboyant for the rap enjoyment" and t = MAC(K, m) 
//!     
//!     4. We (Eve) can't compute K because h isn't a valid public key. We notice again that 
//!     g^x starts repeating for x > q, and notice that h has the same property with r. This means there 
//!     are r possible values of K that Bob could have generated. We can do a brute force search through these 
//!     values until we find a K with t = MAC(K, m). 
//!     Now we know Bob's secret key (x) mod r. 
//!
//!     5. We repeat steps 1-4 many times, getting 
//!        x = b1 mod r1
//!        x = b2 mod r2
//!        x = b3 mod r3
//!        ...
//!     We stop when prod(r1, r2, ...) > q. Then we use the CRT to recover Bob's secret key x. 
