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
        eqs.push((*id, fitness(*id, inputs, outputs)));
    }

    eqs
}

/// Fitting the data using Chi-squared minimization.
///
/// Return (χ²/nr_total), the fit is reasonably good when is of order 1.0
///
/// χ² = ∑ ((Measureᵢ -fᵢ)/sigmaᵢ)²
/// where Measureᵢ are individual measurements;
/// fᵢ(params) is predicted value of the model
/// with M parameters which are set to some reasonable trial value.
///
/// The fit is reasonably good when (χ²/nr_total) is of order 1.0.
///
pub fn fitness(id: usize, inputs: &[f64], outputs: &[f64]) -> f64
{
    let equation_builder = &EQUATIONS[id];
    let (out_params, cns_params, inp_params) = (equation_builder.params)();
    let (nr_out_params, _nr_cns_params, nr_inp_params) = (out_params.len(), cns_params.len(), inp_params.len());

    let nr_measurements = inputs.len() / nr_inp_params;
    assert_eq!(outputs.len() / nr_out_params, nr_measurements);

    let equation_constants: Vec<f64> = Vec::new();
    //FIXME constants

    let mut equation = (equation_builder.new)(&equation_constants);

    let mut predictions: Vec<f64> = Vec::with_capacity(outputs.len());

    for i in 0..nr_measurements {
        let input_start_index = i * nr_inp_params;
        let input_end_index = input_start_index + nr_inp_params;
        let mut prediction = equation.run(&inputs[input_start_index..input_end_index]);
        predictions.append(&mut prediction);
    }

    let mut chi2: f64 = 0.0_f64;
    for i in 0..nr_measurements {
        let output_start_index = i * nr_out_params;
        //let output_end_index = output_start_index + nr_out_params;
        for j in 0..nr_out_params {
            let diff = outputs[output_start_index + j] - predictions[output_start_index + j];
            chi2 += diff * diff;
        }
    }

    chi2 /= nr_measurements as f64;

    chi2
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