use num_primes::{Generator,Verification};

fn main(){
    let prime = Generator::new_prime(1024);
    let safe = Generator::safe_prime(128);

    let is_prime: bool = Verification::is_prime(&prime);
    let is_safe_prime: bool = Verification::is_safe_prime(&safe);

    assert_eq!(is_prime, true);
    assert_eq!(is_safe_prime, true);
}