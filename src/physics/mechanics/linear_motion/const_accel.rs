//! Linear motion with constant acceleration.
//!
//! (c) 2023 Igor Lesik
//! MIT license
//!
//! References:
//!
//! - <https://en.wikipedia.org/wiki/List_of_equations_in_classical_mechanics>
//!
use rustamath_mks::*;
use super::super::super::{EqParams, Equation};

/// Velocity formula parameters type
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
    /// eq.calc(10.0);
    /// assert_eq!(eq.velocity.val, 32.0);
    /// ```
    pub fn calc(&mut self, t: f64) {
        self.time.val = t;
        self.velocity = self.initial_velocity +
            self.acceleration * self.time;
    }
}

impl Equation for VelocityEquation {
    /// Get parameters type.
    ///
    /// # Example
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

    /// Create new equation with constant parameters provided.
    fn make(cns: &[f64]) -> Self {
        VelocityEquation::new(cns[0], cns[1])
    }

    /// Create new equation with constant parameters provided.
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath::physics::mechanics::linear_motion::const_accel::VelocityEquation;
    /// use rustamath::physics::Equation;
    /// let mut eq = VelocityEquation::make(&[2.0, 3.0]);
    /// let res = eq.run(&[10.0]);
    /// assert_eq!(res[0], 32.0);
    /// ```
    fn run(&mut self, inp: &[f64]) -> Vec<f64> {
        self.calc(inp[0]);
        vec![self.velocity.val]
    }
}

/// Distance formula parameters type
pub const DISTANCE_EQ_PARAMS: EqParams<1, 2, 1> = EqParams {
    out: [DISTANCE_UNIT], cns: [VELOCITY_UNIT, ACCEL_UNIT], inp: [TIME_UNIT]};

/// Distance equation
pub struct DistanceEquation {
    /// Distance `s = v0*t + (a*t^2)/2`.
    pub distance: MksVal,
    /// Initial velocity
    pub initial_velocity: MksVal,
    /// Constant acceleration
    pub acceleration: MksVal,
    /// Time
    pub time: MksVal,
}

impl DistanceEquation {
    /// Parameters type
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath::physics::mechanics::linear_motion::const_accel::DistanceEquation;
    /// use rustamath_mks::*;
    /// assert!(DistanceEquation::PARAMS.out == [DISTANCE_UNIT]);
    /// assert!(DistanceEquation::PARAMS.cns == [VELOCITY_UNIT, ACCEL_UNIT]);
    /// ```
    pub const PARAMS: EqParams<1, 2, 1> = DISTANCE_EQ_PARAMS;

    /// Initialize constants
    pub fn new(v0: f64, a: f64) -> DistanceEquation {
        DistanceEquation {
            distance: MksVal {val: 0.0, unit: DISTANCE_UNIT},
            initial_velocity: MksVal {val: v0, unit: VELOCITY_UNIT},
            acceleration: MksVal {val: a, unit: ACCEL_UNIT},
            time: MksVal {val: 0.0, unit: TIME_UNIT},
        }
    }

    /// Calculate distance by time with constant acceleration.
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath::physics::mechanics::linear_motion::const_accel::DistanceEquation;
    /// let mut eq = DistanceEquation::new(2.0, 3.0);
    /// eq.calc(10.0);
    /// assert_eq!(eq.distance.val, (2.0 * 10.0) + (3.0 * 100.0)/2.0);
    /// ```
    pub fn calc(&mut self, t: f64) {
        self.time.val = t;
        self.distance =
            self.initial_velocity * self.time +
            (self.acceleration * self.time * self.time) / MksVal::new_scalar(2.0);
    }
}

impl Equation for DistanceEquation {
    /// Get parameters type.
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath::physics::mechanics::linear_motion::const_accel::DistanceEquation;
    /// use rustamath::physics::Equation;
    /// use rustamath_mks::*;
    /// let params = DistanceEquation::params();
    /// assert!(params.0 == &[DISTANCE_UNIT]);
    /// assert!(params.1 == &[VELOCITY_UNIT, ACCEL_UNIT]);
    /// ```
    fn params() -> (&'static [MksUnit], &'static [MksUnit], &'static [MksUnit]) {
        (&Self::PARAMS.out, &Self::PARAMS.cns, &Self::PARAMS.inp)
    }

    /// Create new equation with constant parameters provided.
    fn make(cns: &[f64]) -> Self {
        DistanceEquation::new(cns[0], cns[1])
    }

    /// Create new equation with constant parameters provided.
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath::physics::mechanics::linear_motion::const_accel::DistanceEquation;
    /// use rustamath::physics::Equation;
    /// let mut eq = DistanceEquation::make(&[2.0, 3.0]);
    /// let res = eq.run(&[10.0]);
    /// assert_eq!(res[0], (2.0 * 10.0) + (3.0 * 100.0)/2.0);
    /// ```
    fn run(&mut self, inp: &[f64]) -> Vec<f64> {
        self.calc(inp[0]);
        vec![self.distance.val]
    }
}