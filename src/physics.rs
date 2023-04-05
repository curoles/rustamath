//! Equations of physics.
//!
//! (c) 2023 Igor Lesik
//! MIT license
//!
//! References:
//!
//! - <https://en.wikipedia.org/wiki/Lists_of_physics_equations>
//! - [Deep symbolic regression for physics guided by units constraints](https://arxiv.org/pdf/2303.03192.pdf)

use rustamath_mks::MksUnit;

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
}

/// Equations in classical mechanics.
///
/// <https://en.wikipedia.org/wiki/List_of_equations_in_classical_mechanics>
///
pub mod mechanics {
    /// Equations of linear motion in classical mechanics.
    pub mod linear_motion {
        /// Linear motion with constant acceleration
        pub mod const_accel {
            use rustamath_mks::*;
            use super::super::super::{EqParams, Equation};

            /// Parameters type
            pub const VELOCITY_EQ_PARAMS: EqParams<1, 2, 1> = EqParams {
                out: [VELOCITY_UNIT], cns: [VELOCITY_UNIT, ACCEL_UNIT], inp: [TIME_UNIT]};
        
            /// Velocity equation
            pub struct VelocityEquation {
                /// Velocity `v = v0 + at`.
                pub velocity: MksVal,
                /// Initial velocity
                pub initial_velocity: MksVal,
                /// Constant acceleration
                pub acceleration: MksVal,
                /// Time
                pub time: MksVal,
            }

            impl VelocityEquation {
                /// Parameters type
                ///
                /// # Example
                ///
                /// ```
                /// use rustamath::physics::mechanics::linear_motion::const_accel::VelocityEquation;
                /// use rustamath_mks::*;
                /// assert!(VelocityEquation::PARAMS.out == [VELOCITY_UNIT]);
                /// assert!(VelocityEquation::PARAMS.cns == [VELOCITY_UNIT, ACCEL_UNIT]);
                /// ```
                pub const PARAMS: EqParams<1, 2, 1> = VELOCITY_EQ_PARAMS;

                /// Initialize constants
                pub fn new(v0: f64, a: f64) -> VelocityEquation {
                    VelocityEquation {
                        velocity: MksVal {val: 0.0, unit: VELOCITY_UNIT},
                        initial_velocity: MksVal {val: v0, unit: VELOCITY_UNIT},
                        acceleration: MksVal {val: a, unit: ACCEL_UNIT},
                        time: MksVal {val: 0.0, unit: TIME_UNIT},
                    }
                }

                /// Calculate velocity by time with constant acceleration.
                ///
                /// # Example
                ///
                /// ```
                /// use rustamath::physics::mechanics::linear_motion::const_accel::VelocityEquation;
                /// let mut eq = VelocityEquation::new(2.0, 3.0);
                /// eq.run(10.0);
                /// assert_eq!(eq.velocity.val, 32.0);
                /// ```
                pub fn run(&mut self, t: f64) {
                    self.time.val = t;
                    self.velocity = self.initial_velocity +
                        self.acceleration * self.time;
                }
            }

            impl Equation for VelocityEquation {
                ///
                /// ```
                /// use rustamath::physics::mechanics::linear_motion::const_accel::VelocityEquation;
                /// use rustamath::physics::Equation;
                /// use rustamath_mks::*;
                /// let params = VelocityEquation::params();
                /// assert!(params.0 == &[VELOCITY_UNIT]);
                /// assert!(params.1 == &[VELOCITY_UNIT, ACCEL_UNIT]);
                /// ```
                fn params() -> (&'static [MksUnit], &'static [MksUnit], &'static [MksUnit]) {
                    (&Self::PARAMS.out, &Self::PARAMS.cns, &Self::PARAMS.inp)
                }
            }
        }
    }
}