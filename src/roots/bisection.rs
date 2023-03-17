//! Bisection root finding algorithm

/// Errors that may happen in Bisection algorithm
pub enum BisectionErr {
    /// x range with y endpoints that do not straddle y=0
    EndpointsNotStraddleYeq0,
    /// Callback function failed
    FunctionFailed,
}

/// Prepare for bisecting iterations
pub fn bisection_init(
    f: fn(x: f64) -> Result<f64, ()>,
    x_left: f64,
    x_right: f64) -> Result<(f64,f64,f64), BisectionErr>
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

    Ok((root, f_left, f_right))
}

/// Bisection x and y ranges.
pub type BisectionRange = (f64, f64, f64, f64);

/// Bisect searching range by half
pub fn bisection_iterate(
    f: fn(x: f64) -> Result<f64, ()>,
    (mut x_left, mut x_right, mut f_left, mut f_right): BisectionRange
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

    let /*mut*/ root;

    // Discard the half of the interval which doesn't contain the root.
  
    if (f_left > 0.0 && f_bisect < 0.0) || (f_left < 0.0 && f_bisect > 0.0) {
        root = (x_left + x_bisect) / 2.0;
        x_right = x_bisect;
        f_right = f_bisect;
    }
    else {
        root = (x_bisect + x_right) / 2.0;
        x_left = x_bisect;
        f_left = f_bisect;
    }

    Ok((root, (x_left, x_right, f_left, f_right)))
}