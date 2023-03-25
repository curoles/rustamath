//! Vector trait
//!
//! (c) 2013 Igor Lesik
//! MIT license
//!
use std::fmt;
use super::{TnsrValType};

/// Vector as 1D Tensor
pub trait Vector<T>
where
    T: TnsrValType
{
    /// For fmt::Debug and pretty printing
    fn fmt_vector(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;

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