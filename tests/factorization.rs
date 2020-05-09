use num_primes::{Generator,Factorization};

#[test]
fn factor_uint(){
    // Generate a Large Unsigned Integer of 32 bits
    let x = Generator::new_uint(32);

    println!("Number: {}",x);

    // Factor The Largest Prime of x
    let prime_factor = Factorization::prime_factor(x);

    // Print Out The Statements
    match prime_factor {
        Some(prime_factor) => println!("Prime Factor: {}",prime_factor),
        None => println!("There are no prime factors")
    }

}