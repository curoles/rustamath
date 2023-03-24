//! Linear Algebra Tensor implementation

//use std::vec;
use num_traits::float;
use crate::simd;

mod order;
pub use self::order::{TnsrOrderType, TnsrOrder};

mod trait_tensor;
pub use self::trait_tensor::{Tensor};
mod tensor;

mod trait_matrix;
pub use self::trait_matrix::{Matrix};
mod matrix;

mod trait_vector;
pub use self::trait_vector::{Vector};
mod vector;

#[cfg(test)]
mod tests;

/// N-dimentional Tensor structure
pub struct Tnsr<T>
    where T: float::Float
{
    /// Data storage `Vec<T>`
    pub v: std::vec::Vec<T>,
    nr_dims: usize,
    sizes: std::vec::Vec<usize>,
    order: TnsrOrder
}

impl<T> Tnsr<T>
    where T: float::Float
{
    /// Create new vector
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath::la::tnsr::{Tnsr, Vector};
    /// let t = Tnsr::<f64>::new_vector(10);
    /// assert_eq!(t.raw().len(), 10);
    /// assert_eq!(t.v.len(), 10);
    /// let v = &t as &dyn Vector::<f64>;
    /// assert_eq!(v.size(), 10);
    /// ```
    pub fn new_vector(size: usize) -> Self {
        let mut t = Tnsr {
            v : simd::vec::new(size),
            nr_dims : 1,
            sizes: vec![size],
            order: TnsrOrder::new(TnsrOrderType::RowMajor, 1),
        };
        t.v.resize(size, T::neg_zero());
        t
    }

    /// Create new matrix
    pub fn new_matrix(nr_rows: usize, nr_cols: usize) -> Self {
        Tnsr {
            v : simd::vec::new(nr_rows*nr_cols),
            nr_dims : 2,
            sizes: vec![nr_rows, nr_cols],
            order: TnsrOrder::new(TnsrOrderType::RowMajor, 2),
        }
    }

    /* perhaps SparseTnsr struct; Create new sparse matrix
    pub fn new_sparse_matrix(nr_rows: usize, nr_cols: usize) -> Self {
        Tnsr {
            v : Vec::new(),
            nr_dims : 2,
            sizes: vec![nr_rows, nr_cols],
            order: TnsrOrder::new(TnsrOrderType::SparseHash, 2),
        }
    }*/

    /// Create new tensor
    pub fn new_tensor(sizes: &[usize]) -> Self {
        Tnsr {
            v : simd::vec::new(sizes.iter().sum()),
            nr_dims : sizes.len(),
            sizes: Vec::from(sizes),
            order: TnsrOrder::new(TnsrOrderType::RowMajor, sizes.len()),
        }
    }

    /// Get raw std::vec::Vec vector ref
    pub fn raw(&self) -> &std::vec::Vec::<T> {
        &self.v
    }

    /// Get raw std::vec::Vec vector mut ref
    pub fn mraw(&mut self) -> &mut std::vec::Vec::<T> {
        &mut self.v
    }
}