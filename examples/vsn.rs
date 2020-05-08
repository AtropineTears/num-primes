use num::traits::FromPrimitive;
use num_bigint::BigUint;
use num_primes::Verification;

fn main() {
    // Set BigUint To 7u64
    let x: BigUint = BigUint::from_u64(7u64).unwrap();

    // Verify Its A Smooth Number
    let result: bool = Verification::is_very_smooth_number(&x,31.0,5);

    println!("Is A {} Smooth Number: {}",x,result);
}