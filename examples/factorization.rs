use num_primes::{Generator,Factorization};

fn main() {
    // Generates New Unsighed Integer of 64 bits
    let uint = Generator::new_uint(64);
    // Prime Factorization    
    let prime_factor = Factorization::prime_factor(uint);

    match prime_factor {
        Some(x) => println!("Largest Prime Factor: {}",x),
        None => println!("No Prime Factors Found"),
    }
}