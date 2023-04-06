//! List of all equations.
//!
//! (c) 2023 Igor Lesik
//! MIT license
//!
use super::{Equation, ParamsUnit};
use super::*;

/// Record about an equation.
///
/// 1st function returns `tuple(output, constant, input : &[MksUnit])`.
///
pub struct BuildTuple<'a> {
    /// Short equation descrioption
    pub desc: &'a str,
    /// Unit types of out/const/in parameters
    pub params: fn () -> ParamsUnit,
    /// Function to create an instance of equation
    pub new: fn (&[f64]) -> Box<dyn Equation>,
}

/// List/array of all equations.
pub const EQUATIONS: [BuildTuple; 4] = [
    BuildTuple {
        desc:   "Linear motion const accel velocity `v = v0 + a*t`",
        params: mechanics::linear_motion::const_accel::VelocityEquation::params,
        new:    mechanics::linear_motion::const_accel::VelocityEquation::make},
    BuildTuple {
        desc:  "Linear motion const accel velocity `v = sqrt(v0^2 + 2*a*s)`",
        params: mechanics::linear_motion::const_accel::VelocityByDistEquation::params,
        new:    mechanics::linear_motion::const_accel::VelocityByDistEquation::make},
    BuildTuple {
        desc:  "Linear motion const accel distance `s = v0*t + (a*t^2)/2`",
        params: mechanics::linear_motion::const_accel::DistanceEquation::params,
        new:    mechanics::linear_motion::const_accel::DistanceEquation::make},
    BuildTuple {
        desc:  "Linear motion const accel distance `s = t*(v0 + v)/2`",
        params: mechanics::linear_motion::const_accel::DistanceByVelEquation::params,
        new:    mechanics::linear_motion::const_accel::DistanceByVelEquation::make},
];