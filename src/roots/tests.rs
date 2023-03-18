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

fn print_res(
    msg: &str,
    x1: f64,
    x2: f64,
    max_iters: usize,
    eps: f64,
    expect: f64,
    res: &RootFinderResult)
{
    println!("{:12} [{}..{}] root={} expected:{} iterations:{} (max:{}) epsilon:{}",
        msg, x1, x2, res.root, expect, res.nr_iterations, max_iters, eps);
}

fn test_bisection(
    msg: &str,
    fun: FnWithRoots,
    x1: f64,
    x2: f64,
    max_iters: usize,
    eps: f64,
    expect: f64)
{
    use super::bisection::*;

    let finder = new_bisection_finder();

    let res = finder.find(fun, x1, x2, eps, max_iters);

    if let Ok(res) = res {
        print!("Bisection ");
        print_res(msg, x1, x2, max_iters, eps, expect, &res);
    }
}

fn test_itp(
    msg: &str,
    fun: FnWithRoots,
    x1: f64,
    x2: f64,
    max_iters: usize,
    eps: f64,
    expect: f64)
{
    use super::bisection::*;

    let finder = new_itp_finder();

    let res = finder.find(fun, x1, x2, eps, max_iters);

    if let Ok(res) = res {
        print!("ITP       ");
        print_res(msg, x1, x2, max_iters, eps, expect, &res);
    }
}

fn test_solvers(
    msg: &str,
    fun: FnWithRoots,
    x1: f64,
    x2: f64,
    max_iters: usize,
    eps: f64,
    expect: f64)
{
    test_bisection(msg, fun, x1, x2, max_iters, eps, expect);
    test_itp      (msg, fun, x1, x2, max_iters, eps, expect);
}


#[test]
fn solvers() {
    test_solvers("f=sin",             sin,  3.0, 4.0, 100, 1.0e-7, std::f64::consts::PI);
    // https://github.com/paulnorthrop/itp
    test_solvers("f=lambert",     lambert, -1.0, 1.0, 100, 1.0e-7, 0.5671);
    test_solvers("f=staircase", staircase, -1.0, 1.0, 100, 1.0e-14, 7.4e-11);
    test_solvers("f=warsaw",       warsaw, -1.0, 1.0, 100, 1.0e-7, -0.6817);
}