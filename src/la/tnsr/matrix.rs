//! Matrix
//!
//! (c) 2013 Igor Lesik
//! MIT license
//!
use std::fmt;
use crate::simd;
use super::{Tnsr, TnsrValType, TnsrErr, Matrix};
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

    /// Get access to underlying tensor struct
    fn raw_tensor(&self) -> &Tnsr<T> {
        self
    }

    /// Get position in internal storage
    fn get_raw_pos(&self, row: usize, col: usize) -> usize {
        (self.order.val_pos)(&self.order, &[row, col], &self.sizes)
    }

    /// Get value at (row,col)
    /// FIXME TODO check bounds and return Option/Result
    fn get(&self, row: usize, col: usize) -> T {
        self.v[ self.get_raw_pos(row, col) ]
    }

    /// Set value at (row,col)
    fn set(&mut self, row: usize, col: usize, val: T) -> &mut dyn Matrix<T> {
        let pos = self.get_raw_pos(row, col);
        self.v[pos] = val;
        self
    }

    /// Get number of rows
    fn nr_rows(&self) -> usize {
        self.sizes[0]
    }

    /// Get number of columns
    fn nr_cols(&self) -> usize {
        self.sizes[1]
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
    fn is_transpose(&self, other: &Tnsr<T>) -> bool {
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

    /// Transpose matrix in place, C = Aᵀ => c(i,j)=a(j,i).
    ///
    /// <https://en.wikipedia.org/wiki/In-place_matrix_transposition>
    fn transpose(&mut self) {
        use self::transpose::{transpose_square, transpose_in_place};
        if self.nr_rows() == self.nr_cols() {
            transpose_square(self as &mut dyn Matrix::<T>);
        }
        else {
            transpose_in_place(self);
        }
    }

    /// Matrix addition `C = A + B => c(i,j) = a(i,j) + b(i,j)`
    fn add(&mut self, rhs: &Tnsr<T>) -> Result<(), TnsrErr> {
        if self.nr_dims != 2 {
            return Err(TnsrErr::IllegalNrDimentions{expect: 2, nr_dims: self.nr_dims});
        }
        if rhs.nr_dims != 2 {
            return Err(TnsrErr::IllegalNrDimentions{expect: 2, nr_dims: rhs.nr_dims});
        }
        if self.nr_rows() != rhs.nr_rows() || self.nr_cols() != rhs.nr_cols() {
            return Err(TnsrErr::DimentionsMismatch);
        }
        //TODO if view.transposed
        //if triangle/symmetric/hermitian
        //else
        simd::vec::add(&mut self.v, &rhs.raw_tensor().v);
        Ok(())
    }

    /// Scalar-matrix multiplication `C = αA => c(i,j) = αa(i,j)`
    fn scale(&mut self, factor: T) {
        //self.v.iter_mut().for_each(|x| *x = *x * factor);
        for val in &mut self.v {
            *val = *val * factor;
        }
    }

    /// Matrix-matrix multiplication
    ///
    /// `Rm×p × Rp×n → Rm×n`,
    /// `C = AB => c(i,j) = ∑ a(i,k)b(k,j)`
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath::la::tnsr::{Tnsr, TnsrErr, Matrix};
    /// let mx_a = &mut Tnsr::<f64>::new_matrix(4, 2) as &mut dyn Matrix::<f64>;
    /// mx_a.set(0, 0, 1.1).set(0, 1, 2.2).set(1, 0, 3.3).set(1, 1, 4.4);
    /// mx_a.set(2, 0, 5.5).set(2, 1, 6.6).set(3, 0, 7.7).set(3, 1, 8.8);
    /// let mx_b = mx_a.make_transposed();
    /// let mx_c = &mx_a.mul(&mx_b)? as &dyn Matrix::<f64>;
    /// assert!(mx_c.nr_rows() == 4 && mx_c.nr_cols() == 4);
    /// assert!(mx_c.get(3, 3) == 7.7*7.7 + 8.8*8.8);
    /// # Ok::<(), TnsrErr>(())
    /// ```
    fn mul(&self, rhs: &Tnsr<T>) -> Result<Tnsr<T>, TnsrErr> {
        if self.nr_dims != 2 {
            return Err(TnsrErr::IllegalNrDimentions{expect: 2, nr_dims: self.nr_dims});
        }
        if rhs.nr_dims != 2 {
            return Err(TnsrErr::IllegalNrDimentions{expect: 2, nr_dims: rhs.nr_dims});
        }
        if self.nr_cols() != rhs.nr_rows() {
            return Err(TnsrErr::DimentionsMismatch);
        }
        //TODO if view.transposed
        //if triangle/symmetric/hermitian
        //else
        let mut a_x_b = Tnsr::<T>::new_matrix(self.nr_rows(), rhs.nr_cols());
        for r in 0..self.nr_rows() {
            for c in 0..rhs.nr_cols() {
                let mut sum_products: T = T::default();
                for k in 0..self.nr_cols() {
                    sum_products += self.get(r, k) * rhs.get(k, c);
                }
                a_x_b.set(r, c, sum_products);
            }
        }
        Ok(a_x_b)
    }
}