use num_primes::{Generator,Factorization};

fn main() {
    // Generates New Unsighed Integer of 32 bits
    let uint = Generator::new_uint(32);
    
    // Prime Factorization    
    let factor = Factorization::prime_factor(uint);

    match factor {
        Some(factor) => println!("Largest Prime Factor: {}",factor),
        None => println!("No Prime Factors Found"),
    }
}