use num_bigint::BigUint;
use num_primes::*;
use num::*;


// ===Issue #1===
// - Found on 12 Aug 2021
// The bug is that verification of small prime numbers does not work correctly. It also hangs when generating a 14 bit prime number.
// This affects both Verfication::is_prime() and Verification::is_composite() as is_composite simply is the inverse of the primality check
// The Bug is from the miller-rabin implementation. I cannot find out a fix.

#[test]
fn bug1() {
    

    let numbers = [
        Generator::new_prime(8),
        Generator::new_prime(32),
        Generator::new_prime(16),
        // Two prime numbers
        17957u32.into(), // Prime
        5u32.into(), // Prime
        2usize.into(), // Prime
        num_bigint::ToBigUint::to_biguint(&5).unwrap(), // Prime
        num_bigint::ToBigUint::to_biguint(&37463).unwrap() // Prime
    ];

    for number in numbers {
        if Verification::is_prime(&number) {
            println!("[Prime] {}", number);
        }
        else {
            println!("[Composite] {}", number);
        }
    }
}