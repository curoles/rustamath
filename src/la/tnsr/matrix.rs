//! Matrix
//!
//! (c) 2013 Igor Lesik
//! MIT license
//!
use num_traits::float;
use std::fmt;
use super::{Tnsr, Matrix};

impl<T> fmt::Debug for dyn Matrix<T>
where
    T: float::Float,
    T: std::fmt::Display,
    T: std::fmt::LowerExp
{
    /// Implement fmt::Debug for dyn Matrix<T> redirecting to Matrix<T>::fmt_matrix
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_matrix(f)
    }
}

impl<T> Matrix<T> for Tnsr<T>
where
    T: float::Float,
    T: std::fmt::Display,
    T: std::fmt::LowerExp
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

    /// Get value at (row,col)
    /// FIXME TODO check bounds and return Option/Result
    fn get(&self, row: usize, col: usize) -> T {
        self.v[ (self.order.val_pos)(&self.order, &[row, col], &self.sizes) ]
    }

    /// Set value at (row,col)
    fn set(&mut self, row: usize, col: usize, val: T) -> &mut dyn Matrix<T> {
        self.v[ (self.order.val_pos)(&self.order, &[row, col], &self.sizes) ] = val;
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
}