# Num-Primes: CSPRNG Large Composite, Prime, Safe Prime Generator

[![Crates.io](https://img.shields.io/crates/v/num-primes?style=flat-square)](https://crates.io/crates/num-primes)
![Crates.io](https://img.shields.io/crates/l/num-primes?style=flat-square)

This crate provides a **beautifully simplistic API** for generating large, cryptographically-random, unsigned integers in rust, including but not limited to **composite, prime, and safe prime numbers**.

It takes full advantage of the [num](https://crates.io/crates/num) crate on **stable rust**.

* Read the [About](#about)
* Read the [License](#license)
* Read the [Contribution](#contribution)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
num-primes = "0.1.2"
```



## How To Use

There are three main structs that are included in this library

| Structs       | Description                                                  |
| ------------- | ------------------------------------------------------------ |
| Generator     | Allows the **random generation** of composite numbers, prime numbers, and safe prime numbers. |
| Verification  | Allows the **verification** of composite, prime, safe prime, and very smooth numbers. |
| Factorization | Allows the **factorization** of Composite and Prime Numbers into their largest Prime Factor. |

## Generator

### Generate Composite Number

This function will generate a **composite number** of `n-bits`.

```rust
use num_primes::Generator;

fn main(){
  // Generate Composite of 1024 bits
  let composite = Generator::new_composite(1024);
}
```

### Generate Prime Number

This function will generate a **prime number** of `n-bits`.

```rust
use num_primes::Generator;

fn main(){
  // Generate two primes (p,q) of 512 bits each
  let p = Generator::new_prime(512);
  let q = Generator::new_prime(512);
  
  // Multiply to get the modulus (n)
  let n = p * q;
}
```

### Generate Safe Prime

This function will generate a **safe prime number** of `n-bits`. This function uses the [same tests openssl uses](https://www.openssl.org/docs/man1.1.1/man1/openssl-prime.html) to generate safe primes, which is `(n-1)/2`.

This function is quite time consuming and should be avoided for large sizes.

```rust
use num_primes::Generator;

fn main(){
  // Generate Safe Prime of 64 bits | Uses (n-1)/2 to check
  let safe_prime = Generator::safe_prime(64);
}
```

### Generate Large Unsigned Integer

This function will generate a **large unsigned integer** of `n-bits`. This function is **faster** than generating a composite or prime number due to no checks being done.

```rust
use num_primes::Generator;

fn main(){
  // Generate a Large Unsigned Integer of 1024 bits without running any checks
  let x = Generator::new_uint(1024);
}
```

## Verification

### Verify Composite Number

This function will verify whether a `BigUint` type is a **composite** by returning a boolean value.

```rust
use num_primes::{Generator,Verification};

fn main(){
  // Generates Composite Number of 1024 bits
  let x = Generator::new_composite(1024);
  
  // Check if the number is a Composite Number
  let is_composite: bool = Verification::is_composite(&x);
  
  // Asserts that 'is_composite' is true
  assert_eq!(is_composite, true);
}
```

### Verify Prime Number

This function will verify whether a `BigUint` type is a **prime** by returning a boolean value.

```rust
use num_primes::{Generator,Verification};

fn main(){
  // Generates Prime Number of 1024 bits
  let x = Generator::new_prime(1024);
  
  // Check if the number is a Prime Number
  let is_prime: bool = Verification::is_prime(&x);
  
  // Asserts that 'is_prime' is true
  assert_eq!(is_prime, true);
}
```

### Verify Safe Prime Number

This function will verify whether a `BigUint` type is a **safe prime** by returning a boolean value.

```rust
use num_primes::{Generator,Verification};

fn main(){
  // Generates a Safe Prime Number of 128 bits
  let x = Generator::safe_prime(128);
  
  // Check if the number is a Safe Prime Number
  let is_safe_prime: bool = Verification::is_safe_prime(&x);
  
  // Asserts that `is_safe_prime` is true
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

This function lets you **factorize** composite numbers and prime numbers to find their **Greatest Prime Factor**.

```rust
use num_primes::{Generator,Factorization};

fn main() {
    // Generates New Unsighed Integer of 32 bits
    let uint = Generator::new_uint(32);
    
  	// Prime Factorization Returns Option<BigUint>    
    let factor = Factorization::prime_factor(uint);

  	// match the Option<BigUint>
    match factor {
        Some(factor) => println!("Largest Prime Factor: {}",factor),
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

---

# About

## Prime Number Generation Design

The Prime Number Generation and parts of its code is based on [Snips-AI's Medium Blog on Generating Prime Numbers](https://medium.com/snips-ai/prime-number-generation-2a02f28508ff).

A **conservative attempt** is made at deciding whether a number is prime or not. The number goes through the generation phase and 3 tests to determine if its prime:

### Generation Phase

1. A single parameter is passed to the generator function that indicates the number of bits the prime should be.

2. The userspace CSPRNG is seeded by the operating system to generate the random numbers using the rand crate.

3. An unsigned integer is generated until it passes the prime test.

4. The number is sent to be processed by **four tests**

### Primality Tests

The numbers go through multiple tests to determine whether they are composite or prime.

1. A check is done to see if the number is even.
2. An array of the first **2048 primes** is used to check whether the number is divisble by any of the primes in the array.
3. **Fermat's Little Theorem** is performed
4. **Miller-Rabin Primality Test**, the gold standard recommended by the official RSA documentation and by [NIST](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-4.pdf) on generating primes, is performed with **16 iterations**, the same used by Apple's cryptosystem.

If the number passes these tests, it is considered with high probability to be prime. Feel free to verify them yourselves on [Wolfram Alpha](https://www.wolframalpha.com/) by simply typing in the prime number.

### Safe Primes

**Safe Primes** are generated simply by checking if `(p-1)/2` is a prime with the tests listed above.

## OpenSSL-Prime vs. Num-Primes

**OpenSSL LTS (1.1.1)** has a [doc page](https://www.openssl.org/docs/man1.1.1/man1/openssl-prime.html) for **prime generation**, including how safe primes are generated. 

> OpenSSL should be prefered for serious cryptographic implementations due to the security of their code and their code having been audited.

Num-Primes may be useful in certain situations, like in embedded systems or where OpenSSLis overkill.

---

**OpenSSL-prime:**

* Generates **Safe Primes** using `(n-1)/2` but is much more efficient
* Performs a **default** of **20 checks** on prime numbers.

**Num-Primes:**

* Generates **Safe Primes** using `(n-1)/2` but takes much longer for unknown reasons
* Performs a **default** of **16 checks** on prime numbers through 4 different primality tests
* Supports #[no_std] and stable rust
* May be useful in situations where using OpenSSL would be overkill.

## Differences Between `num-primes` and `ramp-primes`

[ramp-primes](https://github.com/0xSilene/ramp-primes) is the **original implementation** of the prime number generator using the **ramp** library.

[num-primes](https://github.com/0xSilene/num-primes) is the **improved implementation** using the **num** library and is available on **stable release** with **no_std** support.

### num-primes (Stable)

Uses [num](https://crates.io/crates/num), a **pure-rust implementation** for **numeric types and traits**.

* **num** is **stable** and can work on the **Default, Beta, and Nightly Branch**

* **num** is in **Pure Rust**
* **num** implements more **features**
* **num** has around **~6,000,000 downloads**
* **num** is more developed than **ramp**.

**num-primes** has:

* **Better API** and **Documentation**
* More **Functionality**
* **no_std support**

### ramp-primes (Unstable)

Uses [ramp](https://crates.io/crates/ramp), a **high-performance multiple-precision** (aka "BigNum") library for working with numbers bigger than can normally be handled.

* **ramp** only works on **unstable rust** (the nightly branch).
* **ramp** is written in **Rust** and **Inline Assembly** (security concern)
* **ramp** is generally **faster**
* **ramp** is specifically focused on **multiple-precision arithmetic**
* **ramp** has around **~17,000 downloads**
* **ramp** is not as developed as **num**

**ramp-primes** is:

* The First Implementation
* Generally **Faster** For Generation
* Less Features
* Only Works On **Unstable Rust** (Nightly Build)

# License

Licensed under either of

* Apache License, Version 2.0

* MIT license

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
