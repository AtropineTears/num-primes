# Num-Primes

This rust library is dependent on the [num](https://crates.io/crates/num) crate. It allows for the generation and verification of:

- Large Unsigned Integers
- Large Prime Numbers
- Large Safe Prime Numbers

It also includes factorization functions.

## How To Use

There are three main structs that are included in this library

| Structs       | Description                                                  |
| ------------- | ------------------------------------------------------------ |
| Generator     | Allows the **random generation** of large unsigned integers**, **prime numbers, and safe prime numbers |
| Verification  | Allows the **verification** of large composite, prime, safe prime, and very smooth numbers. |
| Factorization | Allows the **factorization** of Composite Numbers into their largest prime factor. |

## Generator

### Generate Large Unsigned Integer

This function will generate a **large unsigned integer** of n-bits.

```rust
use num_primes::Generator;

fn main(){
  let x = Generator::new_uint(1024);
}
```

### Generate Prime Number

This function will generate a **prime number** of the size n-bits.

```rust
use num_primes::Generator;

fn main(){
  let p = Generator::new_prime(512);
  let q = Generator::new_prime(512);
  
  let n = p * q;
}
```

### Generate Safe Prime

This function will generate a **safe prime number** of the size n-bits. This function uses the [same tests openssl uses](https://www.openssl.org/docs/man1.1.1/man1/openssl-prime.html) to generate safe primes, which is `(n-1)/2`. This function is quite time consuming and should be avoided for large sizes.

```rust
use num_primes::Generator;

fn main(){
  let safe_prime = Generator::safe_prime(64);
}
```

## Verification

### Verify Composite Number

This function will verify whether a `BigUint` type is a **composite** by returning a boolean value.

```rust
use num_primes::{Generator,Verification};

fn main(){
  let x = Generator::new_uint(1024);
  
  // Check whether the unsigned integer is a composite or not which can still be prime
  let is_composite: bool = Verification::is_composite(&x);
}
```

### Verify Prime Number

This function will verify whether a `BigUint` type is a **prime** by returning a boolean value.

```rust
use num_primes::{Generator,Verification};

fn main(){
  let x = Generator::new_prime(1024);
  
  let is_prime: bool = Verification::is_prime(&x);
  
  assert_eq!(is_prime, true);
}
```

### Verify Safe Prime Number

This function will verify whether a `BigUint` type is a **safe prime** by returning a boolean value.

```rust
use num_primes::{Generator,Verification};

fn main(){
  let x = Generator::safe_prime(128);
  
  let is_safe_prime: bool = Verification::is_safe_prime(&x);
  
  assert_eq!(is_safe_prime, true);
}
```

### [Experimental] Verify VSN (Smooth Numbers)

**EXPERIMENTAL: Please Avoid Using This Function As Of Now**

Read [Wolfram Alpha - Smooth Numbers](https://mathworld.wolfram.com/SmoothNumber.html)

Read [OEIS - P-Smooth Numbers](http://oeis.org/wiki/P-smooth_numbers)

Read [Wikipedia - Examples of VSN and VSSR](https://en.wikipedia.org/wiki/Very_smooth_hash#Examples_of_VSN_and_VSSR)

---

This function will verify whether a number is a **very smooth number**. It accepts three parameters as follows:

- m: `&BigUint` | prime
- n: `f64` | constant
- c: `u32` | constant

It follows the following equation:

> 1. Return `m`'s **Greatest Prime Factor** as `p`
>    1. if `p` `<=` `log(n)`<sup>`c`</sup> then its **p-smooth**
>    2. if `p` `>` `log(n)`<sup>`c`</sup> then its **not p-smooth**

```rust
use num::traits::FromPrimitive;
use num_bigint::BigUint;
use num_primes::Verification;

fn main() {
    // Set BigUint To 7u64
    let x: BigUint = BigUint::from_u64(7u64).unwrap();

    // Verify Its A Smooth Number with parameters 
  		// m = 7 (&BigUint)
  		// n = 31.0 (f64)
  		// c = 5 (u32)
    let result: bool = Verification::is_very_smooth_number(&x,31.0,5);

  	// Print What Type of Smooth Number It Is And Whether Or Not It Is Smooth
    println!("Is A {} Smooth Number: {}",x,result);
  
  	// This problem should be 7-smooth
  	assert_eq!(result, true);
}
```

## Factorization

**NOTICE:** Factorization is still in the works.

### Prime Factorization

Read [GeeksforGeeks - Efficient program to print all prime factors of a given number](https://www.geeksforgeeks.org/print-all-prime-factors-of-a-given-number/)

---

This function lets you factorize composite numbers and find their **Greatest Prime Factor**.

```rust
use num_primes::{Generator,Factorization};

fn main() {
    // Generates New Unsighed Integer of 64 bits
    let uint = Generator::new_uint(64);
    
  	// Prime Factorization Returns Option<BigUint>    
    let prime_factor = Factorization::prime_factor(uint);

    match prime_factor {
        Some(x) => println!("Largest Prime Factor: {}",x),
        None => println!("No Prime Factors Found"),
    }
}
```

#### How Does It Work

The steps are listed below with `n` being the **input number** being factored:

A **Primality Check** is used first to determine whether the number is a prime or not.

1. while `n` is even, divide by 2
2. After Step 1, `n` must be odd.
   1. `n_sqrt` = Take the square root of `n`
3. Start a loop from `i = 3` to `n_sqrt`
   1. While `i` / `n`
      1. Divide `n` by `i`
   2. On Failure of `i` dividing by `n`,
      1. increment `i` by 2 and continue
4. If `n` is a prime number and `n` > `2`
   1. Return `n`