
pub fn polynomial_2(x: f64, c: &[f64]) -> f64 {
    c[0] + c[1]*x*x
}

pub fn polynomial_3(x: f64, c: &[f64]) -> f64 {
    let x2 = x*x;
    c[0] + c[1]*x2 + c[2]*x2*x
}

pub fn polynomial_4(x: f64, c: &[f64]) -> f64 {
    let x2 = x*x;
    c[0] + c[1]*x2 + c[2]*x2*x + c[3]*x2*x2
}

pub fn polynomial_5(x: f64, c: &[f64]) -> f64 {
    let x2 = x*x;
    let x3 = x2*x;
    c[0] + c[1]*x2 + c[2]*x3 + c[3]*x2*x2 + c[4]*x3*x2
}

pub fn polynomial_n(n: usize, x: f64, c: &[f64]) -> f64 {
    let mut xn: f64  = 1.0;
    let mut res = c[0];
    for i in 1..n {
        xn *= x;
        res += c[i] * xn;
    }
    res
}
