//! Matrix
//!
//! (c) 2013 Igor Lesik
//! MIT license
//!
use super::super::{Tnsr, TnsrValType, Matrix};

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

/// Transpose matrix in-place
///
/// Inspired by: https://github.com/aldro61/in-place-transpose/blob/master/itranspose_tests.cpp
///
/// References:
///
/// - https://link.springer.com/chapter/10.1007/978-3-540-75755-9_68
/// - <https://dl.acm.org/doi/pdf/10.1145/355611.355612>
/// - <https://www3.nd.edu/~shu/research/papers/ipdps01.pdf>
///
/// # Example
///
/// ```
/// use rustamath::la::tnsr::{Tnsr, Matrix};
/// let mx_a = &mut Tnsr::<f64>::new_matrix(5, 2) as &mut dyn Matrix::<f64>;
/// mx_a.set(0, 0, 1.1).set(0, 1, 2.2);
/// mx_a.set(1, 0, 3.3).set(1, 1, 4.4);
/// mx_a.set(2, 0, 5.5).set(2, 1, 6.6);
/// mx_a.set(3, 0, 7.7).set(3, 1, 8.8);
/// mx_a.set(4, 0, 9.0).set(4, 1, 0.1);
/// let mx_b = mx_a.clone();
/// mx_a.transpose();
/// assert!(mx_a.is_transpose(&mx_b));
/// ```
pub fn transpose_in_place<T>(t: &mut Tnsr<T>)
where
    T: TnsrValType
{
    let nr_rows = t.sizes[0]; // nr rows before transposition
    let nr_cols = t.sizes[1]; // nr cols before transposition
    t.sizes.swap(0, 1);  // rotate nxm -> mxn

    let a = t as &mut dyn Matrix<T>;

    let mut visited = Vec::<bool>::new();
    visited.resize(nr_rows * nr_cols, false);

    for row in 0..nr_cols {
        for col in 0..nr_rows {

            let pos = row * nr_rows + col;

            if !visited[pos] {
                let mut curr_pos = pos;

                let mut val = a.get(row, col);

                while !visited[curr_pos] {
                    visited[curr_pos] = true;

                    let a_c = curr_pos / nr_cols;
                    let a_r = curr_pos - nr_cols * a_c;

                    let tmp = a.get(a_r, a_c);
                    a.set(a_r, a_c, val);
                    val = tmp;

                    curr_pos = a_r*nr_rows + a_c;
                }
            }
        }
    }
}

#[cfg(test)]
#[test]
fn test_transpose_in_place() {
    use crate::la::tnsr::{Tnsr, Matrix};
    for nr_rows in 1..20 {
        for nr_cols in 1..20 {
            let a = &mut Tnsr::<f64>::new_matrix(nr_rows, nr_cols) as &mut dyn Matrix::<f64>;
            for row in 0..nr_rows {
                for col in 0..nr_cols {
                    a.set(row, col, (0.1+row as f64) + (0.2+col as f64)/123.4);
                }
            }
            let b = a.clone();
            a.transpose();
            assert!(a.is_transpose(&b));
        }
    }
}