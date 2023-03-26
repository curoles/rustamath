//! Matrix
//!
//! (c) 2013 Igor Lesik
//! MIT license
//!
use std::fmt;
use super::{Tnsr, TnsrValType, Matrix};
mod transpose;

impl<T> fmt::Debug for dyn Matrix<T>
where
    T: TnsrValType
{
    /// Implement `fmt::Debug` for `dyn Matrix<T>` redirecting to `Matrix<T>::fmt_matrix`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_matrix(f)
    }
}

impl<T> Matrix<T> for Tnsr<T>
where
    T: TnsrValType
{
    fn fmt_matrix(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let print_row = |row_id: &usize| -> String {
            let mut row = String::new();
            if self.nr_cols() < 10 {
                for col_id in 0..self.nr_cols() {
                    row.push_str(&format!("{:10.3e}, ", self.get(*row_id, col_id)));
                }
            }
            else {
                for col_id in 0..4 {
                    row.push_str(&format!("{:10.3e}, ", self.get(*row_id, col_id)));
                }
                row.push_str("... ");
                for col_id in self.nr_cols()-4..self.nr_cols() {
                    row.push_str(&format!("{:10.3e}, ", self.get(*row_id, col_id)));
                }
            }
            row
        };
        let mut s = String::new();
        if self.nr_rows() < 16 {
            for row_id in 0..self.nr_rows() {
                s.push_str(&print_row(&row_id));
                s.push('\n');
            }
        }
        else {
            for row_id in 0..7 {
                s.push_str(&print_row(&row_id));
                s.push('\n');
            }
            s.push_str("...\n");
            for row_id in self.nr_rows()-7..self.nr_rows() {
                s.push_str(&print_row(&row_id));
                s.push('\n');
            }
        }
        write!(f, "{}x{}\n{}", self.nr_cols(), self.nr_rows(), s)
    }

    /// Clone
    fn clone(&self) -> Tnsr<T> {
        Clone::clone(self)
    }

    /// Get value at (row,col)
    /// FIXME TODO check bounds and return Option/Result
    fn get(&self, row: usize, col: usize) -> T {
        let rowt = if self.view.transposed {col} else {row};
        let colt = if self.view.transposed {row} else {col};
        self.v[ (self.order.val_pos)(&self.order, &[rowt, colt], &self.sizes) ]
    }

    /// Set value at (row,col)
    fn set(&mut self, row: usize, col: usize, val: T) -> &mut dyn Matrix<T> {
        let rowt = if self.view.transposed {col} else {row};
        let colt = if self.view.transposed {row} else {col};
        self.v[ (self.order.val_pos)(&self.order, &[rowt, colt], &self.sizes) ] = val;
        self
    }

    /// Get number of rows
    fn nr_rows(&self) -> usize {
        self.sizes[if self.view.transposed {1} else {0}]
    }

    /// Get number of columns
    fn nr_cols(&self) -> usize {
        self.sizes[if self.view.transposed {0} else {1}]
    }

    /// Return true if other matrix is transposition to this matrix
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath::la::tnsr::{Tnsr, Matrix};
    /// let mx_a = &mut Tnsr::<f64>::new_matrix(4, 2) as &mut dyn Matrix::<f64>;
    /// mx_a.set(0, 0, 1.1).set(0, 1, 2.2).set(1, 0, 3.3).set(1, 1, 4.4);
    /// mx_a.set(2, 0, 5.5).set(2, 1, 6.6).set(3, 0, 7.7).set(3, 1, 8.8);
    /// let mx_at = mx_a.make_transposed();
    /// assert!(mx_a.is_transpose(&mx_at));
    /// ```
    fn is_transpose(&self, other: &dyn Matrix<T>) -> bool {
        if self.nr_rows() != other.nr_cols() || self.nr_cols() != other.nr_rows() {
            return false;
        }
        for i in 0..self.nr_rows() {
            for j in 0..self.nr_cols() {
                if self.get(i, j) != other.get(j, i) {
                    return false;
                }
            }
        }
        true
    }

    /// Return transposed matrix, C = Aᵀ => c(i,j)=a(j,i)
    fn make_transposed(&self) -> Tnsr<T> {
        let mut at = Tnsr::<T>::new_matrix(self.nr_cols(), self.nr_rows());
        for i in 0..self.nr_rows() {
            for j in 0..self.nr_cols() {
                at.set(j, i, self.get(i, j));
            }
        }
        at
    }

    /// Transpose view w/o changing internal representation
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath::la::tnsr::{Tnsr, Matrix};
    /// let mut tz_a = Tnsr::<f64>::new_matrix(4, 2);
    /// let mx_a = &mut tz_a as &mut dyn Matrix::<f64>;
    /// mx_a.set(0, 0, 1.1).set(0, 1, 2.2).set(1, 0, 3.3).set(1, 1, 4.4);
    /// mx_a.set(2, 0, 5.5).set(2, 1, 6.6).set(3, 0, 7.7).set(3, 1, 8.8);
    /// let mx_b = mx_a.clone();
    /// mx_a.transpose_view();
    /// assert!(mx_a.is_transpose(&mx_b));
    /// ```
    fn transpose_view(&mut self) {
        self.view.transposed = !self.view.transposed;
    }

    /// Transpose matrix in place, C = Aᵀ => c(i,j)=a(j,i).
    ///
    /// <https://en.wikipedia.org/wiki/In-place_matrix_transposition>
    /// <https://dl.acm.org/doi/pdf/10.1145/355611.355612>
    /// <https://www3.nd.edu/~shu/research/papers/ipdps01.pdf>
    fn transpose(&mut self) {
        use self::transpose::transpose_square;
        if self.nr_rows() == self.nr_cols() {
            transpose_square(self as &mut dyn Matrix::<T>);
        }
    }

}