use num_primes::{Generator,Factorization};

#[test]
fn factor_uint(){
    // Generate a Unsigned Integer of 16 bits
    let p = Generator::new_prime(16);
    let q = Generator::new_prime(16);
    let f = &p * &q;

    // Factor The Largest Prime of x
    let prime_factor = Factorization::prime_factor(f).unwrap();

    if &p > &q {
        assert_eq!(prime_factor, p);
    } else {
        assert_eq!(prime_factor, q);
    }
}