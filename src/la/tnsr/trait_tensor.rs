//! Tensor trait

use num_traits::float;

/// N-dimentional Tensor
pub trait Tensor<T>
    where T: float::Float
{
    /// Get numbers of dimensions
    fn nr_dims(&self) -> usize;

    /// Get size for each dimension
    fn sizes(&self) -> &[usize];

    /// Get size for a dimension
    fn dim(&self, dim_index: usize) -> Option<usize>;
}