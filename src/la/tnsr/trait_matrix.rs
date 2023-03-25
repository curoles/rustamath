//! Matrix trait
//!
//! (c) 2013 Igor Lesik
//! MIT license
//!
use num_traits::float;
use std::fmt;

/// Matrix as 2D Tensor
pub trait Matrix<T>
where
    T: float::Float,
    T: std::fmt::Display,
    T: std::fmt::LowerExp
{
    /// For fmt::Debug and pretty printing
    fn fmt_matrix(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;

    /// Get value at (row,col)
    fn get(&self, row: usize, col: usize) -> T;

    /// Set value at (row,col)
    fn set(&mut self, row: usize, col: usize, val: T) -> &mut dyn Matrix<T>;

    /// Get number of rows
    fn nr_rows(&self) -> usize;

    /// Get number of columns
    fn nr_cols(&self) -> usize;
}