//! Matrix which can be transposed via changing view
//! w/o changing internal representation.
//!
//! (c) 2013 Igor Lesik
//! MIT license
//!
use std::fmt;
use super::{Tnsr, TnsrValType};

/// Matrix which can be transposed via changing view
/// w/o changing internal representation.
pub trait TranspMatrix<T>
where
    T: TnsrValType
{
    /// For fmt::Debug and pretty printing
    fn fmt_matrix(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;

    /// Get access to underlying tensor struct
    fn raw_tensor(&self) -> &Tnsr<T>;

    /// Get position in internal storage
    fn get_raw_pos(&self, row: usize, col: usize) -> usize;

    /// Get value at (row,col)
    fn get(&self, row: usize, col: usize) -> T;

    /// Set value at (row,col)
    fn set(&mut self, row: usize, col: usize, val: T) -> &mut dyn TranspMatrix<T>;

    /// Get number of rows
    fn nr_rows(&self) -> usize;

    /// Get number of columns
    fn nr_cols(&self) -> usize;

    /// Return true if other matrix is transposition to this matrix
    fn is_transpose(&self, other: &Tnsr<T>) -> bool;

    /// Return transposed matrix, `C = Aᵀ => c(i,j)=a(j,i)`
    fn make_transposed(&self) -> Tnsr<T>;

    /// Transpose view w/o changing internal representation
    fn transpose(&mut self);

    /// Matrix addition `C = A + B => c(i,j) = a(i,j) + b(i,j)`
    fn add(&mut self, rhs: &Tnsr<T>) /*-> Result if sizes diff*/;
}