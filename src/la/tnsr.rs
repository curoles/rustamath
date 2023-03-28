//! Linear Algebra Tensor implementation

//use std::vec;
use num_traits::float;
use crate::simd;

mod order;
pub use self::order::{TnsrOrderType, TnsrOrder, TnsrView};

mod trait_tensor;
pub use self::trait_tensor::{Tensor};
mod tensor;

mod trait_matrix;
pub use self::trait_matrix::{Matrix};
mod matrix;

mod trait_transp_matrix;
pub use self::trait_transp_matrix::{TranspMatrix};
mod transp_matrix;

mod trait_vector;
pub use self::trait_vector::{Vector};
mod vector;

#[cfg(test)]
mod tests;

/// All traits that value must implement
pub trait TnsrValType: float::Float
    + std::fmt::Debug
    + std::fmt::Display
    + std::fmt::LowerExp
    {}
impl<T: float::Float
    + std::fmt::Debug
    + std::fmt::Display
    + std::fmt::LowerExp> TnsrValType for T {} // blanket implementation


/// N-dimentional Tensor structure
pub struct Tnsr<T>
where
    T: TnsrValType
{
    /// Data storage `Vec<T>`
    pub v: std::vec::Vec<T>,
    nr_dims: usize,
    sizes: std::vec::Vec<usize>,
    order: TnsrOrder,
    view: TnsrView,
}

impl<T> Tnsr<T>
where
    T: TnsrValType
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
            view: TnsrView {transposed: false},
        };
        t.v.resize(size, T::neg_zero());
        t
    }

    /// Create new matrix
    pub fn new_matrix(nr_rows: usize, nr_cols: usize) -> Self {
        let mut t = Tnsr {
            v : simd::vec::new(nr_rows*nr_cols),
            nr_dims : 2,
            sizes: vec![nr_rows, nr_cols],
            order: TnsrOrder::new(TnsrOrderType::RowMajor, 2),
            view: TnsrView {transposed: false},
        };
        t.v.resize(nr_rows*nr_cols, T::neg_zero());
        t
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
            view: TnsrView {transposed: false},
        }
    }

    /// Get raw std::vec::Vec vector ref
    pub fn raw(&self) -> &std::vec::Vec::<T> {
        &self.v
    }

    /// Get raw std::vec::Vec vector mut ref
    pub fn mut_raw(&mut self) -> &mut std::vec::Vec::<T> {
        &mut self.v
    }
}

use std::fmt;

impl<T> fmt::Debug for Tnsr<T>
where
    T: TnsrValType
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.nr_dims {
            1 => self.fmt_vector(f),
            2 => Matrix::fmt_matrix(self, f),
            _ => self.fmt_tensor(f),
        }
    }
}

impl<T> std::clone::Clone for Tnsr<T>
where
    T: TnsrValType
{
    fn clone(&self) -> Self {
        let mut new_tensor = match self.nr_dims {
            1 => Tnsr::<T>::new_vector(self.sizes[0]),
            2 => Tnsr::<T>::new_matrix(self.sizes[0], self.sizes[1]),
            _ => Tnsr::<T>::new_tensor(&self.sizes),
        };
        new_tensor.v.copy_from_slice(&self.v);
        new_tensor.view = self.view;
        new_tensor
    }
}