use num_bigint::BigUint;
use num_primes::*;
use num::*;


#[test]
fn bug1() {

    let primes = [
        Generator::new_prime(8),
        num_bigint::ToBigUint::to_biguint(&849376067).unwrap(),
        num_bigint::ToBigUint::to_biguint(&37463).unwrap(),
        num_bigint::ToBigUint::to_biguint(&1).unwrap(),
        num_bigint::ToBigUint::to_biguint(&5).unwrap(),
        num_bigint::ToBigUint::to_biguint(&17957).unwrap(),
        num_bigint::ToBigUint::to_biguint(&2).unwrap(),
        num_bigint::ToBigUint::to_biguint(&79).unwrap(),
    ];
    

    for number in primes {
        if Verification::is_prime(&number) {
            println!("[Prime] {}", number);
        }
        else {
            println!("[Composite] {}", number);
        }
    }
}