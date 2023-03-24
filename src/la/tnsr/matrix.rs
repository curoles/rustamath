//! Matrix
//!
//! (c) 2013 Igor Lesik
//! MIT license
//!
use num_traits::float;
use std::fmt;
use super::{Tnsr, Matrix};

impl<T> fmt::Debug for dyn Matrix<T>
    where T: float::Float
{
    /// Implement fmt::Debug for dyn Matrix<T> redirecting to Matrix<T>::fmt_matrix
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_matrix(f)
    }
}

impl<T> Matrix<T> for Tnsr<T>
    where T: float::Float
{
    fn fmt_matrix(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.nr_cols(), self.nr_rows())
    }

    /// Get value at (row,col)
    /// FIXME TODO check bounds and return Option
    fn get(&self, row: usize, col: usize) -> T {
        self.v[ (self.order.val_pos)(&self.order, &[row, col], &self.sizes) ]
    }

    /// Get number of columns
    fn nr_cols(&self) -> usize {
        self.sizes[0]
    }

    /// Get number of rows
    fn nr_rows(&self) -> usize {
        self.sizes[1]
    }
}