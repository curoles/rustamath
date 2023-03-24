//! Matrix
//!
//! (c) 2013 Igor Lesik
//! MIT license
//!
use num_traits::float;
use super::{Tnsr, Matrix};

impl<T> Matrix<T> for Tnsr<T>
    where T: float::Float
{
    /// Get value at (row,col)
    /// FIXME TODO check bounds and return Option
    fn get(&self, row: usize, col: usize) -> T {
        self.v[ (self.order.val_pos)(&self.order, &[row, col], &self.sizes) ]
    }
}