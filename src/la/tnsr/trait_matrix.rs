//! Matrix trait

use num_traits::float;

/// Matrix as 2D Tensor
pub trait Matrix<T>
    where T: float::Float
{
    /// Get value at (row,col)
    fn get(&self, row: usize, col: usize) -> T;
}