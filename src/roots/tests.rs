use super::*;

#[inline] fn sin(x: f64) -> Result<f64, ()> {
    Ok(x.sin())
}

#[inline] fn cos(x: f64) -> Result<f64, ()> {
    Ok(x.cos())
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
    expect: f64,
    check_expected: bool)
{
    use super::bisection::*;

    let finder = new_bisection_finder();

    let res = finder.find(fun, x1, x2, eps, max_iters);

    if let Ok(res) = res {
        print!("Bisection ");
        print_res(msg, x1, x2, max_iters, eps, expect, &res);
        if check_expected {
            assert!(res.nr_iterations < max_iters);
            // assert_f64_near!(res.root, expect, 4);
            assert_float_absolute_eq!(res.root, expect, eps);
        }
    }
    else {
        assert!(false, "root finder failed");
    }
}

fn test_itp(
    msg: &str,
    fun: FnWithRoots,
    x1: f64,
    x2: f64,
    max_iters: usize,
    eps: f64,
    expect: f64,
    check_expected: bool)
{
    use super::itp::*;

    let finder = new_itp_finder();

    let res = finder.find(fun, x1, x2, eps, max_iters);

    if let Ok(res) = res {
        print!("ITP       ");
        print_res(msg, x1, x2, max_iters, eps, expect, &res);
        if check_expected {
            assert!(res.nr_iterations < max_iters);
            //assert_f64_near!(res.root, expect, 4);
            assert_float_absolute_eq!(res.root, expect, eps);
        }
    }
    else {
        assert!(false, "root finder failed");
    }
}

fn test_solvers(
    msg: &str,
    fun: FnWithRoots,
    x1: f64,
    x2: f64,
    max_iters: usize,
    eps: f64,
    expect: f64,
    check_expected: bool)
{
    test_bisection(msg, fun, x1, x2, max_iters, eps, expect, check_expected);
    test_itp      (msg, fun, x1, x2, max_iters, eps, expect, check_expected);
}


#[test]
fn solvers() {
    // https://github.com/ampl/gsl/blob/master/roots/test.c
    test_solvers("f=sin[3,4]",       sin,  3.0,     4.0, 100, 1.0e-15, std::f64::consts::PI, true);
    test_solvers("f=sin[-4,-3]",     sin, -4.0,    -3.0, 100, 1.0e-15, -std::f64::consts::PI, true);
    test_solvers("f=sin[-1/3,1]",    sin, -1.0/3.0, 1.0, 100, 1.0e-15, 0.0, true);
    test_solvers("f=cos[0,3]",       cos,  0.0,     3.0, 100, 1.0e-15, std::f64::consts::PI / 2.0, true);
    test_solvers("f=cos[-3,0]",      cos, -3.0,     0.0, 100, 1.0e-15, -std::f64::consts::PI / 2.0, true);
    // https://github.com/ampl/gsl/blob/master/roots/test_funcs.c
    test_solvers("f=x^20 - 1 [0.1,2]",  |x| Ok(x.powi(20) - 1.0),  0.1, 2.0, 100, 1.0e-15, 1.0, true);
    //test_solvers("f=(x-1)^7 [0.9995, 1.0002]",  |x| Ok((x - 1.0).powi(7)),  0.9995, 1.0002, 100, 1.0e-7, 1.0, true);// ITP problem FIXME
    test_solvers("f=x*exp(-x) [-1/3,2]",  |x| Ok(x*(-x).exp()),  -1.0/3.0, 2.0, 100, 1.0e-15, 0.0, true);
    test_solvers("f=x^2 - 1e-8 [0,1]",  |x| Ok(x*x - 1.0e-8),  0.0, 1.0, 100, 1.0e-15, (1.0e-8f64).sqrt(), true);
    //"sqrt(|x|)*sgn(x)", -1.0 / 3.0, 1.0, 0.0);

    // https://github.com/paulnorthrop/itp
    test_solvers("f=lambert",     lambert, -1.0, 1.0, 100, 1.0e-7, 0.5671, false);
    test_solvers("f=staircase", staircase, -1.0, 1.0, 100, 1.0e-14, 7.4e-11, false);
    test_solvers("f=warsaw",       warsaw, -1.0, 1.0, 100, 1.0e-7, -0.6817, false);
}