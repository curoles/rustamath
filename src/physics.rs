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

/// Equation creation interface
pub trait EquationMaker {
    /// Return tuple with paramerts type
    fn params() -> (&'static [MksUnit], &'static [MksUnit], &'static [MksUnit]);

    /// Create new equation with provided constant parameters.
    fn make(cns: &[f64]) -> Box<dyn Equation>;
}

/// Equation interface
pub trait Equation {
    /// Run equation with provided input parameters.
    fn run(&mut self, inp: &[f64]) -> Vec<f64>;
}

/// Record about an equation.
///
/// 1st function returns `tuple(output, constant, input : &[MksUnit])`.
///
pub type BuildTuple<'a> = (
    &'a str,
    fn () -> (&'static [MksUnit], &'static [MksUnit], &'static [MksUnit]),
    fn (&[f64]) -> Box<dyn Equation>,
);

/// List/array of all equations.
pub const EQUATIONS: [BuildTuple; 4] = [
    ("Linear motion const accel velocity `v = v0 + a*t`",
    mechanics::linear_motion::const_accel::VelocityEquation::params,
    mechanics::linear_motion::const_accel::VelocityEquation::make),
    ("Linear motion const accel velocity `v = sqrt(v0^2 + 2*a*s)`",
    mechanics::linear_motion::const_accel::VelocityByDistEquation::params,
    mechanics::linear_motion::const_accel::VelocityByDistEquation::make),
    ("Linear motion const accel distance `s = v0*t + (a*t^2)/2`",
    mechanics::linear_motion::const_accel::DistanceEquation::params,
    mechanics::linear_motion::const_accel::DistanceEquation::make),
    ("Linear motion const accel distance `s = t*(v0 + v)/2`",
    mechanics::linear_motion::const_accel::DistanceByVelEquation::params,
    mechanics::linear_motion::const_accel::DistanceByVelEquation::make),
];

/// Get list of equations that have specified input/output unit types.
///
/// # Example
///
/// ```
/// use rustamath::physics::{find_equation_by_units, Equation, EquationMaker, EQUATIONS};
/// use rustamath_mks::*;
/// let ids = find_equation_by_units(&[TIME_UNIT], &[VELOCITY_UNIT]);
/// assert_eq!(ids[0], 0);
/// let mut equation = EQUATIONS[ids[0]].2(&[3.0, 2.0]);
/// assert_eq!(equation.run(&[10.0])[0], 3.0 + 2.0*10.0);
/// ```
pub fn find_equation_by_units(inputs: &[MksUnit], outputs: &[MksUnit]) -> Vec<usize> {
    let mut eqs: Vec<usize> = Vec::new();

    for (index, eq) in EQUATIONS.iter().enumerate() {
        let (out, _cns, inp) = eq.1();
        if out == outputs && inp == inputs {
            eqs.push(index);
        }
    }
    eqs
}
