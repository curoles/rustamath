//! Roots finding algorithm

pub mod bisection;
pub mod itp;
//https://github.com/ampl/gsl/blob/master/roots/newton.c
//https://github.com/ampl/gsl/blob/master/roots/steffenson.c
//https://github.com/ampl/gsl/blob/master/roots/secant.c
//https://github.com/ampl/gsl/blob/master/roots/falsepos.c
//https://github.com/ampl/gsl/blob/master/roots/brent.c

#[cfg(test)]
mod tests;

/// Errors that may happen in root finding algorithm
#[derive(Debug)]
pub enum RootsErr {
    /// Call to the function with roots failed
    FunctionFailed,
    /// Input x range with y endpoints that do not straddle y=0
    EndpointsNotStraddleYeq0,
}

/// Bisection x and y ranges.
pub type Range = (f64, f64, f64, f64);

/// Function with roots that we investigate
pub type FnWithRoots = fn(x: f64) -> Result<f64, ()>;

/// Solver function that performs one root finding iteration
pub type Solver<S> = fn(
    FnWithRoots,
    Range,
    &S,
) -> Result<(f64, Range), RootsErr>;


/// Prepare for bisecting iterations.
///
/// Check input range.
/// Return x and y ranges and initial root.
///
pub fn basic_init(
    f: FnWithRoots,
    x_left: f64,
    x_right: f64) -> Result<(f64, Range), RootsErr>
{
    // guess root is in a middle of the range
    let root = (x_left + x_right)/2.0;

    let f_left = match f(x_left) {
        Ok(y) => y,
        Err(_) => return Err(RootsErr::FunctionFailed),
    };

    let f_right = match f(x_right) {
        Ok(y) => y,
        Err(_) => return Err(RootsErr::FunctionFailed),
    };

    if (f_left < 0.0 && f_right < 0.0) || (f_left > 0.0 && f_right > 0.0) {
        return Err(RootsErr::EndpointsNotStraddleYeq0);
    }

    Ok((root, (x_left, x_right, f_left, f_right)))
}

/// Root finder perform root finding iterations
pub struct RootFinder<S> {
    /// State that gets transferred  between iterations
    pub state: S,
    ///
    solver: Solver<S>
}

/// Root found value, precision, number iterations and etc.
///
pub struct RootFinderResult {
    /// Root found value
    pub root: f64,
    /// Number of iterations
    pub nr_iterations: usize
}

/// State trait
pub trait RootFinderState {
    /// Create new state
    fn new() -> Self;
}

impl<S: RootFinderState> RootFinder<S> {

    // TODO check https://github.com/ampl/gsl/blob/master/roots/convergence.c
    /// Call solvers till `abs(prev-next) > epsilon`
    pub fn find(
        &self,
        fun: FnWithRoots,
        x_left: f64,
        x_right: f64,
        epsilon: f64,
        max_iterations: usize
    ) -> Result<RootFinderResult, RootsErr>
    {
        let (mut root, mut range) = match basic_init(fun, x_left, x_right) {
            Ok(res) => res,
            Err(err) => { return Err(err); },
        };

        let state: S = S::new();

        let mut nr_iterations: usize = 0;
        let mut old_root = root;

        while nr_iterations < max_iterations {
            (root, range) = match (self.solver)(fun, range, &state) {
                Ok(res) => res,
                Err(err) => { return Err(err); },
            };
            if expect_float_absolute_eq!(root, old_root, epsilon).is_ok() {
                break;
            }
            old_root = root;
            nr_iterations += 1;
        }

        // cargo test --lib xxx -- --nocapture
        //println!("root={} iterations={}", root, i);

        Ok(RootFinderResult {
            root,
            nr_iterations
        })
    }

}
