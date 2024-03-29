//! Tensor
//!
//! (c) 2013 Igor Lesik
//! MIT license
//!
use std::fmt;
use super::{Tnsr, TnsrValType, Tensor};

impl<T> Tensor<T> for Tnsr<T>
where
    T: TnsrValType
{
    fn fmt_tensor(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "todo tensor")
    }

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