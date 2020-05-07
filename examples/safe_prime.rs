use num_primes::Generator;

fn main(){
    // p = 2q + 1 where p is the safe prime and q is a prime that fits the equation
    let safe_prime = Generator::safe_prime(128);

    // Prints safe prime and prime number
    println!("safe prime: {}",safe_prime);
}