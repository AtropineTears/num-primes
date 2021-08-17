use num_primes::*;


// ===Issue #1===
// - Found on 12 Aug 2021
// The bug is that verification of small prime numbers does not work correctly. It also hangs when generating a 14 bit prime number.
// This affects both Verfication::is_prime() and Verification::is_composite() as is_composite simply is the inverse of the primality check
// The Bug is from the miller-rabin implementation. I cannot find out a fix.

#[test]
fn bug1() {
    let numbers = [
        Generator::new_prime(512),
        Generator::new_prime(32),
        Generator::new_prime(16),
        // Two prime numbers
        17957u32.into(),
        5u32.into(),
    ];

    for number in numbers {
        if Verification::is_prime(&number) {
            println!("{} is a prime number", number);
        }
        else {
            println!("{} is not a prime number", number);
        }
    }
}