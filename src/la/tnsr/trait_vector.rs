//! Vector trait

use num_traits::float;

/// Vector as 1D Tensor
pub trait Vector<T>
    where T: float::Float
{
    /// Get vector size or length
    fn size(&self) -> usize;

    /// Get value at position
    fn get(&self, pos: usize) -> T;

    /// Set value at position
    fn set(&mut self, pos: usize, val: T);

    /// Norm of a vector `v` is the length or magnitute of the `v`.
    /// Formula `norm(v) = sqrt( sum(v[i]^2) )`
    fn norm(&self) -> T;
}