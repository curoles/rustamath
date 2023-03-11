//! Polynomial functions.
//!

/// Polynomial c0 + c1*x + c2*x^2
///
/// # Example
///
/// ```
/// # use rustamath::polynomial::*;
/// assert_eq!(polynomial_2(2.1, &[3.3, 1.0, 1.0]), (3.3 + 2.1 + 2.1*2.1));
/// ```
#[inline]
pub fn polynomial_2(x: f64, c: &[f64]) -> f64 {
    c[0] + c[1]*x + c[2]*x*x
}

/// Polynomial c0 + c1*x + c2*x^2 + c3*x^3
///
/// # Example
///
/// ```
/// # use rustamath::polynomial::*;
/// assert_eq!(polynomial_3(2.2, &[3.3, 1.0, 1.0, 1.0]), (3.3 + 2.2 + 2.2*2.2 + 2.2*2.2*2.2));
/// ```
#[inline]
pub fn polynomial_3(x: f64, c: &[f64]) -> f64 {
    let x2 = x*x;
    c[0] + c[1]*x + c[2]*x2 + c[3]*x2*x
}

/// Polynomial c0 + c1*x + c2*x^2 + c3*x^3 + c4*x^4
#[inline]
pub fn polynomial_4(x: f64, c: &[f64]) -> f64 {
    let x2 = x*x;
    c[0] + c[1]*x + c[2]*x2 + c[3]*x2*x + c[4]*x2*x2
}

/// Polynomial c0 + c1*x + c2*x^2 + c3*x^3 + c4*x^4 + c5*x^5
#[inline]
pub fn polynomial_5(x: f64, c: &[f64]) -> f64 {
    let x2 = x*x;
    let x3 = x2*x;
    c[0] + c[1]*x + c[2]*x2 + c[3]*x3 + c[4]*x2*x2 + c[5]*x3*x2
}

/// Polynomial function of degree n
///
/// # Example
///
/// ```
/// # use rustamath::polynomial::*;
/// let c = [1.1, 2.2, 0.0, 4.4];
/// let x = 2.0;
/// assert_eq!(naive_polynomial_n(x, &c), (1.1 + 2.2*2.0 + 0.0 + 4.4*2.0*2.0*2.0));
/// ```
pub fn naive_polynomial_n(x: f64, cs: &[f64]) -> f64 {
    let mut xn: f64  = 1.0;
    let mut res = cs[0];

    for c in cs.iter().skip(1) {
        xn *= x;
        res += c * xn;
    }
    res
}

/// Polynomial function of degree n calculated with Horner's method
///
/// [Horner's method](https://en.wikipedia.org/wiki/Horner%27s_method) uses Horner rule:
/// `c0 + c1*x + c2*x^2 + c3*x^3 ... = c0 + x(c1 + x(c2 + x(c3 ...`
///
/// # Example
///
/// ```
/// # use rustamath::polynomial::*;
/// let c = [1.1, 2.2, 3.3, 4.4];
/// let x = 2.0;
/// assert_eq!(polynomial_n(x, &c), (1.1 + 2.2*2.0 + 3.3*2.0*2.0 + 4.4*2.0*2.0*2.0));
/// ```
///
/// ```
/// # use rustamath::polynomial::*;
/// # use assert_float_eq::*;
/// let c = [1.1, 2.2, 3.3, 4.4, 5.5, 6.6];
/// let x = 0.12345678;
/// assert_f64_near!(polynomial_5(x, &c), polynomial_n(x, &c));
/// ```
pub fn polynomial_n(x: f64, cs: &[f64]) -> f64 {
    let mut res = cs[cs.len()-1];
    //SLOW: for c in cs.iter().take(cs.len()-1).rev() {
    for i in (0..cs.len()-1).rev() {
        res = res*x + cs[i];
    }
    res
}

/// Polynomial function of variable degree
///
/// # Example
///
/// ```
/// # use rustamath::polynomial::*;
/// assert_eq!(polynomial!(2.1, 3.3, 4.4, 5.5), (3.3 + 4.4*2.1 + 5.5*2.1*2.1));
/// ```
#[macro_export]
macro_rules! polynomial {
    ( $x:expr, $( $c:expr ),* ) => {
        {
            let mut res: f64 = 0.0;
            let mut _xn: f64 = 1.0;
            $(
                res += $c * _xn;
                _xn *= $x;
            )*
            res
        }
    };
}

// this makes path: `use rastomath::polynomial::polynomial`
pub use polynomial;

#[cfg(test)]
mod tests {
    use crate::polynomial::*;

    #[test]
    fn p2() {
        assert_eq!(polynomial_2(2.1, &[3.3, 1.0, 1.0]), (3.3 + 2.1 + 2.1*2.1));
        assert_eq!(polynomial!(2.1, 3.3, 4.4, 5.5), (3.3 + 4.4*2.1 + 5.5*2.1*2.1));
    }

    #[test]
    fn p3() {
        let c = [1.1, 2.2, 3.3, 4.4];
        let x = 0.12345678;
        assert_eq!(polynomial_3(x, &c), naive_polynomial_n(x, &c));
        assert_eq!(polynomial_3(x, &c), polynomial!(x, 1.1, 2.2, 3.3, 4.4));
        assert_f64_near!(polynomial_3(x, &c), polynomial_n(x, &c));
    }

    #[test]
    fn p4_5() {
        let c = [1.1, 2.2, 3.3, 4.4, 5.5, 6.6];
        let x = 0.12345678;
        assert_eq!(polynomial_4(x, &c), naive_polynomial_n(x, &c[..5]));
        assert_f64_near!(polynomial_4(x, &c), polynomial_n(x, &c[..5]));
        assert_eq!(polynomial_4(x, &c), polynomial!(x, 1.1, 2.2, 3.3, 4.4, 5.5));
        assert_eq!(polynomial_5(x, &c), naive_polynomial_n(x, &c));
        assert_f64_near!(polynomial_5(x, &c), polynomial_n(x, &c));
        assert_eq!(polynomial_5(x, &c), polynomial!(x, 1.1, 2.2, 3.3, 4.4, 5.5, 6.6));
    }
}
