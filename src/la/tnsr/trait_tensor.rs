//! Tensor trait
//!
//! (c) 2013 Igor Lesik
//! MIT license
//!
use std::fmt;
use super::{TnsrValType};

/// N-dimentional Tensor
pub trait Tensor<T>
where
    T: TnsrValType
{
    /// For fmt::Debug and pretty printing
    fn fmt_tensor(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;

    /// Get numbers of dimensions
    fn nr_dims(&self) -> usize;

    /// Get size for each dimension
    fn sizes(&self) -> &[usize];

    /// Get size for a dimension
    fn dim(&self, dim_index: usize) -> Option<usize>;
}