//! Equations of physics.
//!
//! (c) 2023 Igor Lesik
//! MIT license
//!
//! [Deep symbolic regression for physics guided by units constraints](https://arxiv.org/pdf/2303.03192.pdf)

/// Equations in classical mechanics.
pub mod mechanics {
    /// Equations of linear motion in classical mechanics.
    pub mod linear_motion {
        use rustamath_mks::*;

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
            /// Initialize constants
            pub fn new(v0: f64, a: f64, t: f64) -> VelocityEquation {
                VelocityEquation {
                    velocity: MksVal {val: 0.0, unit: VELOCITY_UNIT},
                    initial_velocity: MksVal {val: v0, unit: VELOCITY_UNIT},
                    acceleration: MksVal {val: a, unit: ACCEL_UNIT},
                    time: MksVal {val: t, unit: TIME_UNIT},
                }
            }

            /// Calculate velocity by time with constant acceleration.
            ///
            /// # Example
            ///
            /// ```
            /// use rustamath::physics::mechanics::linear_motion::VelocityEquation;
            /// let mut eq = VelocityEquation::new(2.0, 3.0, 0.0);
            /// eq.run_time(10.0);
            /// assert_eq!(eq.velocity.val, 32.0);
            /// ```
            pub fn run_time(&mut self, t: f64) {
                self.time.val = t;
                self.velocity = self.initial_velocity +
                    self.acceleration * self.time;
            }
        }
    }
}