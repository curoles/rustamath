//! Bisection root finding algorithm.
//!
//! (c) Igor Lesik 2023
//! MIT license

use super::{RootsErr, Range, FnWithRoots, RootFinder, RootFinderState};

/// Bisection state that is transferred between iteration
pub struct BisectionState {}

impl RootFinderState for BisectionState {
    fn new() -> BisectionState { BisectionState{} }
}

/// Bisect searching range by half
pub fn bisection_iterate(
    f: FnWithRoots,
    (x_left, x_right, f_left, f_right): Range,
    state: BisectionState,
    ) -> Result<(f64, Range, BisectionState), RootsErr>
{
    if f_left == 0.0 {
        return Ok((/*root=*/x_left, (x_left, x_left, f_left, f_left), state));
    }

    if f_right == 0.0 {
        return Ok((/*root=*/x_right, (x_right, x_right, f_right, f_right), state));
    }

    let x_bisect = (x_left + x_right) / 2.0;

    let f_bisect = match f(x_bisect) {
        Ok(y) => y,
        Err(_) => return Err(RootsErr::FunctionFailed),
    };

    if f_bisect == 0.0 {
        return Ok((/*root=*/x_bisect, (x_bisect, x_bisect, f_bisect, f_bisect), state));
    }

    // Discard the half of the interval which doesn't contain the root.
    if (f_left > 0.0 && f_bisect < 0.0) || (f_left < 0.0 && f_bisect > 0.0) {
        let root = (x_left + x_bisect) / 2.0;
        Ok((root, (x_left, x_bisect, f_left, f_bisect), state))
    }
    else {
        let root = (x_bisect + x_right) / 2.0;
        Ok((root, (x_bisect, x_right, f_bisect, f_right), state))
    }
}

/// Create new bisection root finder
pub fn new_bisection_finder() -> RootFinder<BisectionState> {
    RootFinder::<BisectionState> {
        state: BisectionState {},
        solver: bisection_iterate,
    }
}

#[cfg(test)]
#[test]
fn test_bisection() {
    #[inline] fn sin(x: f64) -> Result<f64, ()> {
        Ok(x.sin())
    }

    let finder = new_bisection_finder();

    let res = finder.find(sin, 3.0, 4.0, 1.0e-15, 50);

    if let Ok(res) = res {
        assert!(res.nr_iterations < 50);
        // https://docs.rs/assert_float_eq/latest/assert_float_eq/macro.assert_f64_near.html
        assert_f64_near!(res.root, std::f64::consts::PI, 4);
    }
    else {
        assert!(false, "root finder failed");
    }
}