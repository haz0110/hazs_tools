/*!
Mathematics library written in pure Rust.

This library is inspired while solving projecteuler.net problems. You may find the functions of this library useful for those problems.

Note that this library mostly works with `usize` integers, tested on 64 bit systems, and most of the functions return whether a `usize` or `Vec<usize>`
*/

pub struct ERAMath<T> {
    pub result: Result<T, String>,
    pub duration: std::time::Duration,
}

impl<T> ERAMath<T> {
    pub fn new(result: Result<T, String>, duration: std::time::Duration) -> Self {
        ERAMath { result, duration }
    }
}

pub struct ERABool {
    pub result: Result<bool, String>,
    pub duration: std::time::Duration,
}

impl ERABool {
    pub fn new(result: Result<bool, String>, duration: std::time::Duration) -> Self {
        ERABool { result, duration }
    }
}

pub mod general;
pub use general::ERAGeneralTrait;
pub use general::ERAGeneral;

pub mod trigonometry;
pub use trigonometry::ERATrigonometryTrait;
pub use trigonometry::ERATrigonometry;

pub mod primes;
pub use primes::{primes, nth_prime, is_prime};