//! Bisection root finding algorithm

use super::{RootsErr, Range, FnWithRoots, RootFinder};

/// Bisection state that is transferred between iteration
pub struct BisectionState {}

/// Bisect searching range by half
pub fn bisection_iterate(
    f: FnWithRoots,
    (x_left, x_right, f_left, f_right): Range,
    //_state: &mut BisectionState
    ) -> Result<(f64, Range), RootsErr>
{
    if f_left == 0.0 {
        return Ok((/*root=*/x_left, (x_left, x_left, f_left, f_left)));
    }

    if f_right == 0.0 {
        return Ok((/*root=*/x_right, (x_right, x_right, f_right, f_right)));
    }

    let x_bisect = (x_left + x_right) / 2.0;

    let f_bisect = match f(x_bisect) {
        Ok(y) => y,
        Err(_) => return Err(RootsErr::FunctionFailed),
    };

    if f_bisect == 0.0 {
        return Ok((/*root=*/x_bisect, (x_bisect, x_bisect, f_bisect, f_bisect)));
    }

    // Discard the half of the interval which doesn't contain the root.
    if (f_left > 0.0 && f_bisect < 0.0) || (f_left < 0.0 && f_bisect > 0.0) {
        let root = (x_left + x_bisect) / 2.0;
        Ok((root, (x_left, x_bisect, f_left, f_bisect)))
    }
    else {
        let root = (x_bisect + x_right) / 2.0;
        Ok((root, (x_bisect, x_right, f_bisect, f_right)))
    }
}

/// Create new bisection root finder
pub fn new_bisection_finder() -> RootFinder<BisectionState> {
    RootFinder::<BisectionState> {
        state: BisectionState {},
        solver: bisection_iterate,
    }
}

/// ITP state that is transferred between iteration
pub struct ItpState {}

/// Bisect with [Interpolate Truncate and Project](https://en.wikipedia.org/wiki/ITP_method)
///
/// https://github.com/paulnorthrop/itp
///
/// TODO check all parameters, especially k1
/// update `for_rk *= 0.5`? see https://github.com/paulnorthrop/itp/blob/main/src/itp_c.cpp
///
pub fn itp_iterate(
    f: fn(x: f64) -> Result<f64, ()>,
    (x_left, x_right, f_left, f_right): Range
    ) -> Result<(f64, Range), RootsErr>
{
    let   epsilon: f64 = 1e-10;
    let   k1: f64 = 0.1;//0.2 / (x_right - x_left);
    const K2: f64 = 2.0;
    const N0: f64 = 1.0;
    let   log2_epsilon: f64 = epsilon.log2();
    let   log2bma = (x_right - x_left).log2();
    let   for_rk = 2.0_f64.powf(N0 - 1.0 + log2_epsilon + (log2bma - log2_epsilon).ceil());

    if (x_right - x_left) <= 2.0*epsilon {
        return bisection_iterate(f, (x_left, x_right, f_left, f_right));
    }

    if f_left == 0.0 {
        return Ok((/*root=*/x_left, (x_left, x_left, f_left, f_left)));
    }

    if f_right == 0.0 {
        return Ok((/*root=*/x_right, (x_right, x_right, f_right, f_right)));
    }

    // Interpolation [Regula falsi](https://en.wikipedia.org/wiki/Regula_falsi)
    let x_interpolation = (f_right * x_left - f_left * x_right) / (f_right - f_left) ;

    // Truncation
    let x_bisect = (x_left + x_right) / 2.0;
    let x_diff = x_bisect - x_interpolation;
    let sigma: f64 = if x_diff.is_sign_negative() { -1.0 } else { 1.0 };
    let delta = k1 * (x_right - x_left).powf(K2) ;
    let x_trunc = if delta <= x_diff.abs() { x_interpolation + sigma * delta } else { x_bisect };

    // Projection
    let rk = for_rk - (x_right - x_left) / 2.0 ;
    let x_itp = if x_diff.abs() <= rk { x_trunc } else { x_bisect - sigma * rk };

    // Update range
    let y_itp = match f(x_itp) {
        Ok(y) => y,
        Err(_) => return Err(RootsErr::FunctionFailed),
    };

    if y_itp.is_sign_positive() {
        let x_right_new = x_itp;
        let f_right_new = y_itp;
        let root = (x_left + x_right_new) / 2.0;
        Ok((root, (x_left, x_right_new, f_left, f_right_new)))
    } else if y_itp.is_sign_negative() {
        let x_left_new = x_itp;
        let f_left_new = y_itp;
        let root = (x_left_new + x_right) / 2.0;
        Ok((root, (x_left_new, x_right, f_left_new, f_right)))
    } else {
        Ok((x_itp, (x_itp, x_itp, y_itp, y_itp)))
    }
}

/// Create new ITP root finder
pub fn new_itp_finder() -> RootFinder<ItpState> {
    RootFinder::<ItpState> {
        state: ItpState {},
        solver: itp_iterate,
    }
}