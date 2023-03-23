//! Math functions
//!
//! (c) Igor Lesik 2023
//! MIT license
//!
//! See existing functions: <https://doc.rust-lang.org/std/primitive.f64.html>

use num_traits::float::{Float};

pub mod complex;

/// Return true if value is Not-a-Number
///
/// # Example
/// ```
/// # use rustamath::function::*;
/// assert!(is_float_nan(f64::NAN));
/// assert!(!is_float_nan(12.34_f64));
/// ```
#[inline] pub fn is_float_nan<T: Float>(val: T) -> bool
{
    val.is_nan()
}

/// Return true if value is +/- infinity
///
/// # Example
/// ```
/// # use rustamath::function::*;
/// assert!(is_float_infinite(f64::NEG_INFINITY));
/// assert!(!is_float_infinite(12.34_f64));
/// ```
#[inline] pub fn is_float_infinite<T: Float>(val: T) -> bool
{
    val.is_infinite()
}

/// Check two f32 numbers are close within number of steps/ulps
#[inline] pub fn f32_near(a: f32, b: f32, steps: u32) -> bool {
    expect_f32_near!(a, b, steps.max(4)).is_ok()
}

/// Check two f64 numbers are close within number of steps/ulps
#[inline] pub fn f64_near(a: f64, b: f64, steps: u32) -> bool {
    expect_f64_near!(a, b, steps.max(4)).is_ok()
}

/// Compare two f64 float numbers
#[inline] pub fn f64_near_precision(a: f64, b: f64, epsilon: f64, relative: bool) -> bool
{
    a == b ||
    (if relative { (a - b)/a } else { a - b }).abs() <= epsilon.abs() ||
    f64_near(a, b, 4)
}

/// Return accurate `ln(1 + x)`
/// # Example
/// ```
/// # use rustamath::function::*;
/// # use assert_float_eq::*;
/// assert_f64_near!(ln_1_plus_x(0.00123), (1.0f64 + 0.00123).ln(), 1000);
/// assert_f64_near!(ln_1_plus_x(0.00123), 0.00123_f64.ln_1p(), 4);
/// ```
#[inline] pub fn ln_1_plus_x<T: Float>(x: T) -> T {
    x.ln_1p()
}

/// Calculate sqrt(a^2 + b^2)
///
/// check it prevents overflow as <https://www.gnu.org/software/gsl/doc/html/math.html#c.gsl_hypot>
#[inline] pub fn hypotenuse<T: Float>(a: T, b: T) -> T {
    a.hypot(b)
}

/// Calculate `e^x -1`
///
/// <https://www.gnu.org/software/gsl/doc/html/math.html#c.gsl_expm1>
#[inline] pub fn exp_x_minus_1<T: Float>(x: T) -> T {
    x.exp_m1()
}

/// Calculate `1/x`
///
#[inline] pub fn reciprocal<T: Float>(x: T) -> T {
    x.recip()
}

//TODO <https://www.gnu.org/software/gsl/doc/html/math.html#c.gsl_ldexp>

//TODO <https://www.gnu.org/software/gsl/doc/html/math.html#c.gsl_frexp>

/// `a*b + c`
#[inline] pub fn mul_add<T: Float>(a: T, b: T, c:T) -> T {
    a.mul_add(b, c)
}

/// x^2
#[inline] pub fn pow_2<T: Float>(x: T) -> T {
    x * x
}

/// x^3
#[inline] pub fn pow_3<T: Float>(x: T) -> T {
    let x2 = x * x;
    x2 * x
}

/// x^4
#[inline] pub fn pow_4<T: Float>(x: T) -> T {
    let x2 = x * x;
    x2 * x2
}

/// x^5
#[inline] pub fn pow_5<T: Float>(x: T) -> T {
    let x2 = x * x;
    x2 * x2 * x
}

/// x^6
#[inline] pub fn pow_6<T: Float>(x: T) -> T {
    let x3 = x * x * x;
    x3 * x3
}

/// x^7
#[inline] pub fn pow_7<T: Float>(x: T) -> T {
    let x3 = x * x * x;
    x3 * x3 * x
}

/// x^8
#[inline] pub fn pow_8<T: Float>(x: T) -> T {
    let x2 = x * x;
    let x4 = x2 * x2;
    x4 * x4
}

/// sin(x)
#[inline] pub fn sin<T: Float>(x: T) -> T {
    x.sin()
}
