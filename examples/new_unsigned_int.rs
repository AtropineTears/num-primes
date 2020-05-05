use num_primes::Generator;

fn main() {
    // Generate Large Random Unsigned Integer of 1024 bits
    let x = Generator::new_uint(1024);

    // Print Integer
    println!("Large Unsigned Integer: {}",x);
}