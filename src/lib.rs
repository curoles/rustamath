//! Math functions

#![warn(missing_docs)]

#[cfg(test)]
#[macro_use]
extern crate assert_float_eq;

pub mod simd;
pub mod polynomial;
pub mod random;
pub mod la;

/// i am foo
pub fn foo(n: i32) -> i32 {
    n
}

