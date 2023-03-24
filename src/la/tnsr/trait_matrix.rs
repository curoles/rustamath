//! Matrix trait
//!
//! (c) 2013 Igor Lesik
//! MIT license
//!
use num_traits::float;
use std::fmt;

/// Matrix as 2D Tensor
pub trait Matrix<T>
    where T: float::Float
{
    /// For fmt::Debug and pretty printing
    fn fmt_matrix(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;

    /// Get value at (row,col)
    fn get(&self, row: usize, col: usize) -> T;

    /// Get number of columns
    fn nr_cols(&self) -> usize;

    /// Get number of rows
    fn nr_rows(&self) -> usize;
}