//! Polynomial functions.
//!

/// Polynomial c0 + c1*x + c2*x^2
///
/// # Example
///
/// ```
/// # use rustomath::polynomial::*;
/// assert_eq!(polynomial_2(2.1, &[3.3, 1.0, 1.0]), (3.3 + 2.1 + 2.1*2.1));
/// ```
pub fn polynomial_2(x: f64, c: &[f64]) -> f64 {
    c[0] + c[1]*x + c[2]*x*x
}

/// Polynomial c0 + c1*x + c2*x^2 + c3*x^3
///
/// # Example
///
/// ```
/// # use rustomath::polynomial::*;
/// assert_eq!(polynomial_3(2.2, &[3.3, 1.0, 1.0, 1.0]), (3.3 + 2.2 + 2.2*2.2 + 2.2*2.2*2.2));
/// ```
pub fn polynomial_3(x: f64, c: &[f64]) -> f64 {
    let x2 = x*x;
    c[0] + c[1]*x + c[2]*x2 + c[3]*x2*x
}

/// Polynomial c0 + c1*x + c2*x^2 + c3*x^3 + c4*x^4
pub fn polynomial_4(x: f64, c: &[f64]) -> f64 {
    let x2 = x*x;
    c[0] + c[1]*x + c[2]*x2 + c[3]*x2*x + c[4]*x2*x2
}

/// Polynomial c0 + c1*x + c2*x^2 + c3*x^3 + c4*x^4 + c5*x^5
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
/// # use rustomath::polynomial::*;
/// let c = [1.1, 2.2, 3.3, 4.4];
/// let x = 1.0;
/// assert_eq!(polynomial_n(3, x, &c), 11.0);
/// ```
pub fn polynomial_n(n: usize, x: f64, c: &[f64]) -> f64 {
    let mut xn: f64  = 1.0;
    let mut res = c[0];
    for i in 1..=n {
        xn *= x;
        res += c[i] * xn;
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::polynomial::*;

    #[test]
    fn p2() {
        assert_eq!(polynomial_2(2.1, &[3.3, 1.0, 1.0]), (3.3 + 2.1 + 2.1*2.1));
    }

    #[test]
    fn p3() {
        let c = [1.1, 2.2, 3.3, 4.4];
        let x = 0.12345678;
        assert_eq!(polynomial_3(x, &c), polynomial_n(3, x, &c));
    }

    #[test]
    fn p4_5() {
        let c = [1.1, 2.2, 3.3, 4.4, 5.5, 6.6];
        let x = 0.12345678;
        assert_eq!(polynomial_4(x, &c), polynomial_n(4, x, &c));
        assert_eq!(polynomial_5(x, &c), polynomial_n(5, x, &c));
    }
}
