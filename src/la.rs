//! Linear Algebra

//use std::vec;

/// N-dimentional Tensor
pub trait Tensor<T>
    where T: Clone
{
    /// Create new tensor
    fn new(sizes: &[usize]) -> Self;

    /// Get numbers of dimensions
    fn nr_dims(&self) -> usize;

    /// Get size in each dimension
    fn sizes(&self) -> &[usize];
}

/// Matrix as 2D Tensor
pub trait Matrix<T>
    where T: Clone
{

    /// Create new matrix
    fn new(nr_rows: usize, nr_cols: usize) -> Self;
}

/// Vector as 1D Tensor
pub trait Vector<T>
    where T: Clone
{

    /// Create new vector
    fn new(size: usize) -> Self;

    /// Get raw std::vec::Vec vector ref
    fn to_vector(&self) -> &std::vec::Vec::<T>;

    /// Norm of a vector `v` is the length or magnitute of the `v`.
    /// Formula `norm(v) = sqrt( sum(v[i]^2) )`
    fn norm(&self) -> T; //FIXME mul+add type
}

/// N-dimentional Tensor structure
pub struct Tnsr<T>
    where T: Clone
{
    v: std::vec::Vec<T>,
    nr_dims: usize,
    sizes: std::vec::Vec<usize>
}

impl<T> Tensor<T> for Tnsr<T>
    where T: Clone
{

    /// Create new tensor
    fn new(sizes: &[usize]) -> Self {
        Tnsr {
            v : Vec::<T>::with_capacity(sizes.iter().sum()),
            nr_dims : sizes.len(),
            sizes: Vec::from(sizes)
        }
    }

    /// Get numbers of dimensions
    fn nr_dims(&self) -> usize {
        self.nr_dims
    }

    /// Get size in each dimension
    fn sizes(&self) -> &[usize] {
        &self.sizes
    }
}

impl<T> Matrix<T> for Tnsr<T>
    where T: Clone
{

    /// Create new matrix
    fn new(nr_rows: usize, nr_cols: usize) -> Self {
        Tnsr {
            v : Vec::<T>::with_capacity(nr_rows*nr_cols),
            nr_dims : 2,
            sizes: vec![nr_rows, nr_cols]
        }
    }
}

impl<T> Vector<T> for Tnsr<T>
    where T: Clone + Default
{

    /// Create new vector
    fn new(size: usize) -> Self {
        Tnsr {
            v : Vec::<T>::with_capacity(size),
            nr_dims : 1,
            sizes: vec![size]
        }
    }

    /// Get raw std::vec::Vec vector ref
    fn to_vector(&self) -> &std::vec::Vec::<T> {
        &self.v
    }

    /// Norm of a vector `v` is the length or magnitute of the `v`.
    /// Formula `norm(v) = sqrt( sum(v[i]^2) )`
    fn norm(&self) -> T {
        //
        let n: T = T::default();
        n
    }
}
