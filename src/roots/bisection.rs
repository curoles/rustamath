//! Bisection root finding algorithm

/// Errors that may happen in Bisection algorithm
#[derive(Debug)]
pub enum BisectionErr {
    /// x range with y endpoints that do not straddle y=0
    EndpointsNotStraddleYeq0,
    /// Callback function failed
    FunctionFailed,
}

/// Bisection x and y ranges.
pub type BisectionRange = (f64, f64, f64, f64);

/// Solver iterate function
pub type Solver = fn(fn(x: f64) -> Result<f64, ()>, BisectionRange
    ) -> Result<(f64, BisectionRange), BisectionErr>;

/// Prepare for bisecting iterations
pub fn bisection_init(
    f: fn(x: f64) -> Result<f64, ()>,
    x_left: f64,
    x_right: f64) -> Result<(f64, BisectionRange), BisectionErr>
{
    // guess root is in a middle of the range
    let root = (x_left + x_right)/2.0;

    let f_left = match f(x_left) {
        Ok(y) => y,
        Err(_) => return Err(BisectionErr::FunctionFailed),
    };

    let f_right = match f(x_right) {
        Ok(y) => y,
        Err(_) => return Err(BisectionErr::FunctionFailed),
    };

    if (f_left < 0.0 && f_right < 0.0) || (f_left > 0.0 && f_right > 0.0) {
        return Err(BisectionErr::EndpointsNotStraddleYeq0);
    }

    Ok((root, (x_left, x_right, f_left, f_right)))
}

/// Bisect searching range by half
pub fn bisection_iterate(
    f: fn(x: f64) -> Result<f64, ()>,
    (x_left, x_right, f_left, f_right): BisectionRange
    ) -> Result<(f64, BisectionRange), BisectionErr>
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
        Err(_) => return Err(BisectionErr::FunctionFailed),
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

/// Bisect with [Interpolate Truncate and Project](https://en.wikipedia.org/wiki/ITP_method)
///
/// https://github.com/paulnorthrop/itp
///
/// TODO check all parameters, especially k1
/// update `for_rk *= 0.5`? see https://github.com/paulnorthrop/itp/blob/main/src/itp_c.cpp
///
pub fn itp_iterate(
    f: fn(x: f64) -> Result<f64, ()>,
    (x_left, x_right, f_left, f_right): BisectionRange
    ) -> Result<(f64, BisectionRange), BisectionErr>
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
        Err(_) => return Err(BisectionErr::FunctionFailed),
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

#[cfg(test)]
mod tests {

    use super::*;

    #[inline] fn sin(x: f64) -> Result<f64, ()> {
        Ok(x.sin())
    }

    #[inline] fn lambert(x: f64) -> Result<f64, ()> {
        Ok(x * x.exp() - 1.0)
    }

    #[inline] fn staircase(x: f64) -> Result<f64, ()> {
        Ok((x * 10.0 - 1.0).ceil() + 0.5)
    }

    // https://github.com/paulnorthrop/itp
    #[inline] fn warsaw(x: f64) -> Result<f64, ()> {
        Ok(if x > -1.0 { (1.0/(1.0 + x)).sin() } else { -1.0 })
    }

    fn test_solver(
        msg: &str,
        f: fn(x: f64) -> Result<f64, ()>,
        solver: Solver,
        left: f64,
        right: f64,
        epsilon: f64,
        expect: f64)
    {
        let x_left = left;
        let x_right = right;

        let (mut root, mut range) = match bisection_init(sin, x_left, x_right) {
            Ok(res) => res,
            Err(err) => { assert!(false, "initialization error: {:?}", err); return; }
        };

        const MAX_ITERS: usize = 100;
        let mut i: usize = 0;
        let mut old_root = root;

        while i < MAX_ITERS {
            (root, range) = match solver(f, range) {
                Ok(res) => res,
                Err(err) => { assert!(false, "error: {:?}", err); return; }
            };
            if expect_float_absolute_eq!(root, old_root, epsilon).is_ok() {
            //if expect_f64_near!(root, expect).is_ok() {
                break;
            }
            old_root = root;
            i += 1;
        }

        // cargo test --lib tests::bisection -- --nocapture
        println!("{}: root={}, expected={} iterations={}", msg, root, expect, i);

        assert!(i < MAX_ITERS);
    }

    #[test]
    fn bisection() {
        test_solver("bisection sin(x)", sin, bisection_iterate, 3.0, 4.0, 1.0e-7, std::f64::consts::PI);
        test_solver("bisection lambert(x)", lambert, bisection_iterate, -1.0, 1.0, 1.0e-7, 0.5671);
        test_solver("bisection staircase(x)", staircase, bisection_iterate, -1.0, 1.0, 1.0e-11, 7.4e-11);
        test_solver("bisection warsaw(x)", warsaw, bisection_iterate, -1.0, 1.0, 1.0e-7, -0.6817);
    }

    #[test]
    fn itp() {
        test_solver("ITP sin(x)", sin, itp_iterate, 3.0, 4.0, 1.0e-7, std::f64::consts::PI);
        test_solver("ITP lambert(x)", lambert, itp_iterate, -1.0, 1.0, 1.0e-7, 0.5671);
        test_solver("ITP staircase(x)", staircase, itp_iterate, -1.0, 1.0, 1.0e-11, 7.4e-11);
        test_solver("ITP warsaw(x)", warsaw, itp_iterate, -1.0, 1.0, 1.0e-7, -0.6817);
    }
}