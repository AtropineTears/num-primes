use num_primes::Generator;

fn main(){
    // p = 2q + 1 where p is the safe prime and q is a prime that fits the equation
    let (p,q) = Generator::safe_prime(128);

    // Prints safe prime and prime number
    println!("safe prime (p): {}",p);
    println!("q: {}",q)
}