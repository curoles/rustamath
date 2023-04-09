//! Bracket function minimum.
//!
//! (c) 2023 Igor Lesik
//! MIT license
//!
//! References:
//!
//! - William H. Press - Numerical recipes, the art of scientific computing.
//!   Cambridge University Press (2007).
//!
use std::mem;

type FunToMnmz = fn (input: f64) -> f64;

/// Bracketing points for a minimum.
pub struct BracketRes((f64, f64), (f64, f64), (f64, f64), usize);

/// Default ratio by which successive intervals are magnified
const GOLD: f64 = 1.618034_f64;

/// Maximum magnification allowed for a parabolic-fit step.
const GLIMIT: f64 = 100.0_f64;

const TINY: f64 = 1.0e-20_f64;

/// Bracket a minimum.
///
/// - William H. Press - Numerical recipes, the art of scientific computing.
///   Cambridge University Press (2007).
///
/// Given a function `fun`, and given distinct initial points ax and bx, this routine
/// searches in the downhill direction (defined by the function as evaluated at the initial points)
/// and returns new points ax, bx, cx that bracket a minimum of the function. Also returned
/// are the function values at the three points, fa, fb, and fc.
///
pub fn find_bracket(fun: FunToMnmz, a: f64, b: f64) -> BracketRes {
    let mut a = a;
    let mut b = b;
    let mut fa = fun(a);
    let mut fb = fun(b);

    // Switch roles of a and b so that we can go downhill in the direction from a to b.
    if fb > fa {
        mem::swap(&mut a, &mut b);
        mem::swap(&mut fb, &mut fa);
    }

    // First guess for c.
    let mut c = b + GOLD*(b - a);
    let mut fc = fun(c);

    let mut fu: f64;
    let mut nr_iterations: usize = 1;

    while fb > fc { // Keep returning here until we bracket.
        // Compute u by parabolic extrapolation from a, b, c.
        let r = (b-a)*(fb-fc);
        let q = (b-c)*(fb-fa);
        let q_r = (q-r).abs().max(TINY);
        let q_r = q_r.copysign(q-r);
        let mut u = b - ((b-c)*q - (b-a)*r)/(2.0*q_r);
        let ulim = b + GLIMIT*(c-b);

        // We won’t go farther than this.
        // Test various possibilities:
        if (b-u)*(u-c) > 0.0 { // Parabolic u is between b and c: try it.
            fu = fun(u);
            if fu < fc { // Got a minimum between b and c.
                a  = b;
                b  = u;
                fa = fb;
                fb = fu;
                break;
            }
            else if fu > fb { // Got a minimum between between a and u.
                c  = u;
                fc = fu;
                break;
            }
            // Parabolic fit was no use. Use default magfnification.
            u = c + GOLD*(c-b);
            fu = fun(u);
        }
        else if (c-u)*(u-ulim) > 0.0 { // Parabolic fit is between c and its allowed limit.
            fu = fun(u);
            if fu < fc {
                let d = u + GOLD*(u-c);
                shft3(&mut b, &mut c, &mut u, d);
                shft3(&mut fb, &mut fc, &mut fu, fun(u));
            }
        }
        else if (u-ulim)*(ulim-c) >= 0.0 { // Limit parabolic u to maximum allowed value.
            u = ulim;
            fu = fun(u);
        }
        else { // Reject parabolic u, use default magnification.
            u = c + GOLD*(c-b);
            fu = fun(u);
        }

        // Eliminate oldest point and continue.
        shft3(&mut a, &mut b, &mut c, u);
        shft3(&mut fa, &mut fb, &mut fc, fu);

        nr_iterations += 1;
    }

    BracketRes((a, fa), (b, fb), (c, fc), nr_iterations)
}

/*#[inline] fn shft2(a: &mut f64, b: &mut f64, c: f64) {
    *a = *b;
    *b = c;
}*/

#[inline] fn shft3(a: &mut f64, b: &mut f64, c: &mut f64, d: f64) {
    *a = *b;
    *b = *c;
    *c = d;
}

/*#[inline] fn mov3(a: &mut f64, b: &mut f64, c: &mut f64, d: f64, e: f64, f: f64) {
    *a = d;
    *b = e;
    *c = f;
}*/

#[cfg(test)]
#[test]
fn test_poly2() {
    // Roots 1.0 and 2.0, minimum at 1.5.
    let poly2 = |x: f64| (x-1.0)*(x-2.0);

    let ranges = vec![(10.0, 20.0), (20.0, 10.0), (-10.0, 0.0),
        (-2000.0, -1000.0), (-10_000.0, 30_000.0), (0.0001, 0.0002), (-0.00001, 1.4999)];

    for range in ranges {
        let bracket = find_bracket(poly2, range.0, range.1);

        println!("Bracket: [{:6.2} < {:6.2} < {:6.2}] with values [{:6.2} < {:6.2} < {:6.2}] iterations:{}",
            bracket.0.0, bracket.1.0, bracket.2.0,
            bracket.0.1, bracket.1.1, bracket.2.1,
            bracket.3
        );

        assert!(bracket.0.1 > bracket.1.1 && bracket.1.1 < bracket.2.1);
    }
}