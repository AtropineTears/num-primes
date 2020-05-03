use num_primes::new_uint;

fn main() {
    // Generate Large Random Unsigned Integer of 1024 bits
    let x = new_uint(1024);

    // Print Integer
    println!("Large Unsigned Integer: {}",x);
}