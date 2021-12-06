use num_bigint::BigUint;
use num_primes::Verification;

#[test]
fn primality_test() {
      let big_prime = BigUint::parse_bytes(b"169511182982703321453314585423962898651587669459838234386506572286328885534468792292646838949809616446341407457141008401355628947670484184607678853094537849610289912805960069455687743151708433319901176932959509872662610091644590437761688516626993416011399330087939042347256922771590903190536793274742859624657", 10).unwrap();
      let med_prime = BigUint::parse_bytes(b"17957", 10).unwrap();
      let small_prime = BigUint::parse_bytes(b"5", 10).unwrap();

      assert!(Verification::is_prime(&small_prime));
      assert!(Verification::is_prime(&med_prime));
      assert!(Verification::is_prime(&big_prime));
}
