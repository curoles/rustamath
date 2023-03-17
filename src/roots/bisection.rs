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

/// Bisect searching range by 8 slices and find smallest f(x)
pub fn bisection_iterate_8(
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

#[cfg(test)]
mod tests {

    use super::*;

    #[inline] fn sin(x: f64) -> Result<f64, ()> {
        Ok(x.sin())
    }

    #[test]
    fn bisection() {

        let x_left = 3.0;
        let x_right = 4.0;

        let (mut root, mut range) = match bisection_init(sin, x_left, x_right) {
            Ok(res) => res,
            Err(err) => { assert!(false, "initialization error: {:?}", err); return; }
        };

        const MAX_ITERS: usize = 100;
        let mut i: usize = 0;

        while i < MAX_ITERS {
            (root, range) = match bisection_iterate(sin, range) {
                Ok(res) => res,
                Err(err) => { assert!(false, "error: {:?}", err); return; }
            };
            if expect_f64_near!(root, std::f64::consts::PI).is_ok() {
                break;
            }
            i += 1;
        }

        // cargo test --lib tests::bisection -- --nocapture
        println!("root: {}, iterations: {}, PI={}", root, i, std::f64::consts::PI);

        assert!(i < MAX_ITERS);
    }
}