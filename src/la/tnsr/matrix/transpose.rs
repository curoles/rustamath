//! Matrix
//!
//! (c) 2013 Igor Lesik
//! MIT license
//!
use super::super::{Matrix, TnsrValType};

/// Transpose square matrix
///
/// # Example
///
/// ```
/// use rustamath::la::tnsr::{Tnsr, Matrix};
/// let mx_a = &mut Tnsr::<f64>::new_matrix(3, 3) as &mut dyn Matrix::<f64>;
/// mx_a.set(0, 0, 1.1).set(0, 1, 2.2).set(0, 2, 3.3);
/// mx_a.set(1, 0, 4.4).set(1, 1, 5.5).set(1, 2, 6.6);
/// mx_a.set(2, 0, 7.7).set(2, 1, 8.8).set(2, 2, 9.9);
/// let mx_b = mx_a.clone();
/// mx_a.transpose();
/// assert!(mx_a.is_transpose(&mx_b));
/// ```
pub fn transpose_square<T: TnsrValType>(a: &mut dyn Matrix<T>) {
    for i in 0..a.nr_rows()-1 {
        for j in i+1..a.nr_cols() {
            let tmp = a.get(j, i);
            a.set(j, i, a.get(i, j));
            a.set(i, j, tmp);
        }
    }
}