use num_primes::{Generator,Verification,Factorization};

#[test]
fn main(){
    let x = Generator::new_uint(32);

    println!("Number: {}",x);

    let prime_factor = Factorization::prime_factor(x);

    match prime_factor {
        Some(prime_factor) => println!("Prime Factor: {}",prime_factor),
        None => println!("There are no prime factors")
    }

}