use num_primes::{Generator,Verification};

#[cfg(test)]
#[test]
fn generate_all(){
    let prime = Generator::new_prime(512);
    let _uint = Generator::new_uint(1024);

    // p = 2q + 1 || where p is safe prime
    let _safe_prime = Generator::safe_prime(64);

    let _ver: bool = Verification::is_prime(&prime);
}