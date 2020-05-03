use num_primes::new_prime;

fn main(){
    // Generate Two Primes (p,q) of 512 bits
    let p = new_prime(512);
    let q = new_prime(512);

    // Multiply p and q and return n
    let n = p * q;

    // Print n
    println!("n: {}",n);
}