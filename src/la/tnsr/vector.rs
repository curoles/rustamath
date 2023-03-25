//! Vector
//!
//! (c) 2013 Igor Lesik
//! MIT license
//!
use num_traits::float;
use std::fmt;
use super::{Tnsr, Vector};
use crate::simd;

impl<T> Vector<T> for Tnsr<T>
where
    T: float::Float,
    T: std::fmt::Display,
    T: std::fmt::LowerExp
{
    fn fmt_vector(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "todo vector")
    }

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