//! Dimensionless scaling factors.
//!
//! (c) Igor Lesik 2023
//! MIT license
//!

/// Dimensionless scaling factors
pub trait Scale: Copy + core::ops::Mul<Output = Self> +  core::ops::Div<Output = Self> {

    /// Scale a number by factor.
    ///
    /// # Example
    ///
    /// ```
    /// # use rustamath::constant::scale::{Scale};
    /// assert_eq!(2.1f64.scale(f64::MEGA), 2100_000.0_f64)
    /// ```
    fn scale(&self, factor: Self) -> Self {
        *self * factor
    }

    /// Divide a number by factor.
    ///
    /// # Example
    ///
    /// ```
    /// # use rustamath::constant::scale::{Scale};
    /// assert_eq!(2.1f64.scale(f64::MEGA).in_units(f64::KILO), 2100.0_f64)
    /// ```
    fn in_units(&self, factor: Self) -> Self {
        *self / factor
    }

    /// 10^24
    const YOTTA: Self;
    /// 10^21
    const ZETTA: Self;
    /// 10^18
    const EXA: Self;
    /// 10^15
    const PETA: Self;
    /// 10^12
    const TERA: Self;
    /// 10^9
    const GIGA: Self;
    /// 10^6
    const MEGA: Self;
    /// 10^3
    ///
    /// # Example
    ///
    /// ```
    /// # use rustamath::constant::scale::{Scale};
    /// assert_eq!(f64::KILO, 1000.0_f64)
    /// ```
    const KILO: Self;
    /// -3
    const MILLI: Self;
    /// -6
    const MICRO: Self;
    /// -9
    const NANO: Self;
    /// -12
    const PICO: Self;
    /// -15
    const FEMTO: Self;
    /// -18
    const ATTO: Self;
    /// -21
    const ZEPTO: Self;
    /// -24
    const YOCTO: Self;
}

impl Scale for f64 {
    const YOTTA: f64 = 1.0e24_f64;
    const ZETTA: f64 = 1.0e21_f64;
    const EXA:   f64 = 1.0e18_f64;
    const PETA:  f64 = 1.0e15_f64;
    const TERA:  f64 = 1.0e12_f64;
    const GIGA:  f64 = 1.0e9_f64;
    const MEGA:  f64 = 1.0e6_f64;
    const KILO:  f64 = 1.0e3_f64;
    const MILLI: f64 = 1.0e-3_f64;
    const MICRO: f64 = 1.0e-6_f64;
    const NANO:  f64 = 1.0e-9_f64;
    const PICO:  f64 = 1.0e-12_f64;
    const FEMTO: f64 = 1.0e-15_f64;
    const ATTO:  f64 = 1.0e-18_f64;
    const ZEPTO: f64 = 1.0e-21_f64;
    const YOCTO: f64 = 1.0e-24_f64;
}