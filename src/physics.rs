//! Equations of physics.
//!
//! (c) 2023 Igor Lesik
//! MIT license
//!
//! References:
//!
//! - <https://en.wikipedia.org/wiki/Lists_of_physics_equations>
//! - [Deep symbolic regression for physics guided by units constraints](https://arxiv.org/pdf/2303.03192.pdf)
//!
use rustamath_mks::MksUnit;

pub mod mechanics;

/// Equation parameters
pub struct EqParams<const NR_OUT: usize, const NR_CONST: usize, const NR_IN: usize> {
    /// Output params
    pub out: [MksUnit; NR_OUT],
    /// Constant params
    pub cns: [MksUnit; NR_CONST],
    /// Input params
    pub inp: [MksUnit; NR_IN]
}

/// Equation interface
pub trait Equation {
    /// Return tuple with paramerts type
    fn params() -> (&'static [MksUnit], &'static [MksUnit], &'static [MksUnit]);

    /// Create new equation with constant parameters provided.
    fn make(cns: &[f64]) -> Self;

    /// Create new equation with constant parameters provided.
    fn run(&mut self, inp: &[f64]) -> Vec<f64>;
}

