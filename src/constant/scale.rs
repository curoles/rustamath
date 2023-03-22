//! Dimensionless scaling factors.
//!
//! (c) Igor Lesik 2023
//! MIT license
//!

/// Dimensionless scaling factors
pub trait Scale {

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
    const KILO: Self;

}

impl Scale for f64 {

    const YOTTA: f64 = 1.0e24_f64;
    const ZETTA: f64 = 1.0e21_f64;
    const EXA:   f64 = 1.0e18_f64;
    const PETA:  f64 = 1.0e15_f64;
    const TERA:  f64 = 1.0e12_f64;
    const GIGA:  f64 = 1.0e9_f64;
    const MEGA:  f64 = 1.0e6_f64;

    /// 10^3
    ///
    /// # Example
    ///
    /// ```
    /// # use rustamath::constant::scale::{Scale};
    /// assert_eq!(f64::KILO, 1000.0_f64)
    /// ```
    const KILO: f64 = 1.0e3_f64;

}