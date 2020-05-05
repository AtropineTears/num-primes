use num_primes::Generator;

fn main(){
    // Generate Two Primes (p,q) of 512 bits
    let p = Generator::new_prime(512);
    let q = Generator::new_prime(512);

    // Multiply p and q and return n
    let n = p * q;

    // Print n
    println!("n: {}",n);
}