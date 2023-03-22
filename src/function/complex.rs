//! Math functions with complex numbers
//!
//! (c) Igor Lesik 2023
//! MIT license
//!
//! <https://docs.rs/num/latest/num/complex/struct.Complex.html>
//! <https://www.gnu.org/software/gsl/doc/html/complex.html>

use num_traits::float::{Float};
use num_complex::{Complex};

/// Return the compex conjugate
///
/// # Example
///
/// ```
/// # use rustamath::function::complex::*;
/// # use num_complex::{Complex};
/// let a = Complex::new(1.0, 2.0);
/// let b = conjugate(a);
/// assert_eq!(b.im, -2.0);
/// ```
#[inline] pub fn conjugate<T: Float>(x: Complex<T>) -> Complex<T>
{
    x.conj()
}

// <https://docs.rs/num/latest/num/complex/struct.Complex.html#method.sin>
/// sin(x)
#[inline] pub fn sin<T: Float>(x: Complex<T>) -> Complex<T>
{
    x.sin()
}