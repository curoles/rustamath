//! Linear Algebra Tensor implementation

//use std::vec;
use num_traits::float;
use crate::simd;

#[cfg(test)]
mod tests;

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

/// Matrix as 2D Tensor
pub trait Matrix<T>
    where T: float::Float
{
    /// Get value at (row,col)
    fn get(&self, row: usize, col: usize) -> T;
}

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

/// Tensor internal index to value mapping order
///
/// https://en.wikipedia.org/wiki/Row-_and_column-major_order
///
pub enum TnsrOrderType {
    /// Dense with row-major
    RowMajor,
    /// Dense with column-major
    ColMajor,
    /// Sparse Hash(index -> value)
    SparseHash,
}

/// Control tensor's internal order
///
pub struct TnsrOrder {
    /// Internal order type
    pub kind: TnsrOrderType,
    /// Dense vs sparse
    pub is_sparse: bool,

    /// Function pointer `(i,j,k) -> index` in storage vector
    pub val_pos: fn(&Self, i: &[usize], sz: &[usize]) -> usize,
}


impl TnsrOrder {

    /// Create new Order with type and dimensions
    ///
    pub fn new(kind: TnsrOrderType, nr_dims: usize) -> Self {
        match kind {
            TnsrOrderType::RowMajor => TnsrOrder {
                kind,
                is_sparse: false,
                val_pos: match nr_dims {
                    1 => TnsrOrder::row_major_1d,
                    2 => TnsrOrder::row_major_2d,
                    _ => TnsrOrder::row_major_nd,
                }
            },
            TnsrOrderType::ColMajor => TnsrOrder {
                kind,
                is_sparse: false,
                val_pos: match nr_dims {
                    1 => TnsrOrder::col_major_1d,
                    2 => TnsrOrder::col_major_2d,
                    _ => TnsrOrder::col_major_nd,
                }
            },
            TnsrOrderType::SparseHash => TnsrOrder {
                kind,
                is_sparse: true,
                val_pos: match nr_dims {
                    1 => TnsrOrder::row_major_1d,//FIXME
                    2 => TnsrOrder::row_major_2d,
                    _ => TnsrOrder::row_major_nd,
                }
            },
        }
    }

    #[inline] fn row_major_1d(&self, i: &[usize], _sz: &[usize]) -> usize {
        i[0]
    }

    #[inline] fn row_major_2d(&self, i: &[usize], sz: &[usize]) -> usize {
        i[1] + i[0]*sz[1]
    }

    fn row_major_nd(&self, i: &[usize], sz: &[usize]) -> usize {
        i[1] + i[0]*sz[1]
    }

    #[inline] fn col_major_1d(&self, i: &[usize], _sz: &[usize]) -> usize {
        i[0]
    }

    #[inline] fn col_major_2d(&self, i: &[usize], sz: &[usize]) -> usize {
        i[1] + i[0]*sz[1]
    }

    fn col_major_nd(&self, i: &[usize], sz: &[usize]) -> usize {
        i[1] + i[0]*sz[1]
    }
}

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

impl<T> Tensor<T> for Tnsr<T>
    where T: float::Float
{
    /// Get numbers of dimensions
    fn nr_dims(&self) -> usize {
        self.nr_dims
    }

    /// Get size for each dimension
    fn sizes(&self) -> &[usize] {
        &self.sizes
    }

    /// Get size for a dimension
    fn dim(&self, dim_index: usize) -> Option<usize> {
        if dim_index < self.nr_dims {
            Some(self.sizes[dim_index])
        }
        else { None }
    }
}

impl<T> Matrix<T> for Tnsr<T>
    where T: float::Float
{
    /// Get value at (row,col)
    fn get(&self, row: usize, col: usize) -> T {
        self.v[ (self.order.val_pos)(&self.order, &[row, col], &self.sizes) ]
    }
}

impl<T> Vector<T> for Tnsr<T>
    where T: float::Float
{
    /// Get vector size or length
    fn size(&self) -> usize {
        self.sizes[0]
    }

    /// Get value at position
    fn get(&self, pos: usize) -> T {
        self.v[ (self.order.val_pos)(&self.order, &[pos], &self.sizes) ]
    }

    /// Set value at position
    fn set(&mut self, pos: usize, val: T)  {
        self.v[ (self.order.val_pos)(&self.order, &[pos], &self.sizes) ] = val;
    }

    /// Norm of a vector `v` is the length or magnitute of the `v`.
    ///
    /// Formula `norm(v) = sqrt( sum(v[i]^2) )`
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath::la::tnsr::{Tnsr, Vector}; use assert_float_eq::*;
    /// let mut t = Tnsr::<f64>::new_vector(3);
    /// t.v = vec![1.1, 2.2, 3.3];
    /// assert_f64_near!(t.norm(),
    ///     f64::sqrt( vec![1.1, 2.2, 3.3].iter().fold(0.0, |acc,x| acc + x*x) ));
    /// ```
    fn norm(&self) -> T {
        //if dense
        simd::vec::norm(&self.v)
    }
}
