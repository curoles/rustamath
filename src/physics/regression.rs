//! Simple Symbolic Regression.
//!
//! (c) 2023 Igor Lesik
//! MIT license
//!
//! Find equation(s) based on input/output unit type and
//! input/output values.
use rustamath_mks::*;
use super::{find_equation_by_units, EQUATIONS};

/// Get list of equations that sutisfy specified input/output unit types
/// and fit to measured input/output values.
///
/// This is simple Symbolic Regression algorithm.
///
/// # Example
///
/// ```
/// //use rustamath::physics::{find_equation_by_units, Equation, EquationMaker, EQUATIONS};
/// //use rustamath_mks::*;
/// //let ids = find_equation_by_units(&[TIME_UNIT], &[VELOCITY_UNIT]);
/// //let mut equation = (EQUATIONS[ids[0]].new)(&[3.0, 2.0]);
/// //assert_eq!(equation.run(&[10.0])[0], 3.0 + 2.0*10.0);
/// ```
pub fn find_equation(
    unit_inputs: &[MksUnit],
    unit_outputs: &[MksUnit],
    inputs: &[f64],
    outputs: &[f64]
) -> Vec<(usize, f64)>
{
    let ids: Vec<usize> = find_equation_by_units(unit_inputs, unit_outputs);

    let mut eqs: Vec<(usize, f64)> = Vec::new();

    for id in ids.iter() {
        //let equation_info = &EQUATIONS[*id];
        //println!("#{}: {}", *id, equation_info.desc);
        eqs.push((*id, fitness(*id, inputs, outputs)));
    }

    eqs
}

fn fitness(id: usize, _inputs: &[f64], _outputs: &[f64]) -> f64
{
    let _equation_info = &EQUATIONS[id];
    1.1
}

#[cfg(test)]
#[test]
fn test_circle_vs_square() {
    // cargo test --lib test_circle_vs_square -- --nocapture
    //use rustamath_mks::*;
    use crate::physics::{find_equation, EQUATIONS};

    let eqs = find_equation(&[DISTANCE_UNIT], &[DISTANCE_UNIT], &[3.0], &[18.0]);

    for (i, eq) in eqs.iter().enumerate() {
        let equation_info = &EQUATIONS[eq.0];
        println!("#{}: fit={} {}", i+1, eq.1, equation_info.desc);
    }
}