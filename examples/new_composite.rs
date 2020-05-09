use num_primes::Generator;

fn main(){
    // Generate Composite Number of 1024 bits
    let composite = Generator::new_composite(1024);

    // Print Out Composite Number
    println!("Composite Number: {}",composite);
}