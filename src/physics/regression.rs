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
    use std::thread;

    let ids: Vec<usize> = find_equation_by_units(unit_inputs, unit_outputs);

    let mut eqs: Vec<(usize, f64)> = Vec::new();
    let mut ths = Vec::new();

    for id in ids.iter() {
        let id = *id;
        let mut inputs_copy = Vec::<f64>::with_capacity(inputs.len());//TODO shall I use scoped threads?
        inputs_copy.extend_from_slice(inputs);
        let mut outputs_copy = Vec::<f64>::with_capacity(outputs.len());
        outputs_copy.extend_from_slice(outputs);
        let th = thread::spawn(move || {
            (id, goodness_of_fit(id, &inputs_copy, &outputs_copy))
        });
        ths.push(th);
    }

    for th in ths {
        let id_with_fit = th.join().unwrap();
        eqs.push(id_with_fit);
    }

    eqs.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    eqs
}

/// Return Reduced χ² Chi-squared goodness-of_fit value.
///
/// Return (χ²/degree_freedom), the fit is reasonably good when it is of order 1.0.
///
/// See [Reduced Chi-squared statistics](https://en.wikipedia.org/wiki/Reduced_chi-squared_statistic).
///
/// χ² = ∑ ((Oᵢ - fᵢ)/sigmaᵢ)²
/// where Oᵢ are individual observed values (measurements);
/// fᵢ(params) is predicted value of the model
/// with M parameters which are set to some reasonable trial value.
///
///
pub fn goodness_of_fit(id: usize, inputs: &[f64], outputs: &[f64]) -> f64
{
    let equation_builder = &EQUATIONS[id];
    let (out_params, cns_params, inp_params) = (equation_builder.params)();
    let (nr_out_params, nr_cns_params, nr_inp_params) = (out_params.len(), cns_params.len(), inp_params.len());

    let nr_measurements = inputs.len() / nr_inp_params;
    assert_eq!(outputs.len() / nr_out_params, nr_measurements);

    let mut equation_constants: Vec<f64> = Vec::new();
    equation_constants.resize(nr_cns_params, 1.0);

    let mut equation = (equation_builder.new)(&equation_constants);

    if nr_cns_params > 0 {
        //find_equation_parameters
        //fit_data
    }

    let mut predictions: Vec<f64> = Vec::with_capacity(outputs.len());

    for i in 0..nr_measurements {
        let input_start_index = i * nr_inp_params;
        let input_end_index = input_start_index + nr_inp_params;
        let mut prediction = equation.run(&inputs[input_start_index..input_end_index]);
        predictions.append(&mut prediction);
    }

    assert!(nr_out_params == 1);//FIXME !!! XXX !!!

    let mut chi2: f64 = 0.0_f64;

    for i in 0..nr_measurements {
        let output_start_index = i * nr_out_params;
        //let output_end_index = output_start_index + nr_out_params;
        for j in 0..nr_out_params {
            let diff = outputs[output_start_index + j] - predictions[output_start_index + j];
            chi2 += diff * diff;//FIXME divide by sigma2
        }
    }

    let degrees_of_freedom = if nr_measurements > nr_cns_params { nr_measurements - nr_cns_params } else { 1 };

    chi2 /= degrees_of_freedom as f64;

    chi2
}

/* /// [Variance](https://en.wikipedia.org/wiki/Variance)
fn variance(xs: &[f64]) -> f64 {
    let mut u = 0.0;
    let mut sigma = 0.0;

    for x in xs {
        u += x;
        sigma += x * x;
    }

    sigma /= xs.len() as f64;
    u /= xs.len() as f64;

    sigma - u * u
}*/

/* /// Fitting the data changing const parameters using Chi-squared minimization.
///
/// [Dos and donts of reduced chi-squared](https://arxiv.org/pdf/1012.3754.pdf)
fn fit_data() {
    //
}*/

// cargo test --lib test_circle_vs_square -- --nocapture
#[cfg(test)]
#[test]
fn test_circle_vs_square() {
    use crate::physics::*;

    println!("\nDo 3 -> 18 which is close to circle perimeter 2*3.14*3\n");
    let eqs = find_equation(&[DISTANCE_UNIT], &[DISTANCE_UNIT], &[3.0], &[18.0]);

    for (i, eq) in eqs.iter().enumerate() {
        let equation_info = &EQUATIONS[eq.0];
        println!("#{}: fit = {:8.2} {}", i+1, eq.1, equation_info.desc);
    }

    let eq_index = get_equation_by_typeid(figure::circle::CirclePerimeter::params).unwrap();
    assert_eq!(eq_index, eqs[0].0);

    println!("\nNext do 3 -> 12.1 which is close to square perimeter 3*4\n");
    let eqs = find_equation(&[DISTANCE_UNIT], &[DISTANCE_UNIT], &[3.0], &[12.1]);

    for (i, eq) in eqs.iter().enumerate() {
        let equation_info = &EQUATIONS[eq.0];
        println!("#{}: fit = {:8.2} {}", i+1, eq.1, equation_info.desc);
    }

    let eq_index = get_equation_by_typeid(figure::rectangle::SquarePerimeter::params).unwrap();
    assert_eq!(eq_index, eqs[0].0);
}

// cargo test --lib test_sine_vs_square -- --nocapture
#[cfg(test)]
#[test]
fn test_sine_vs_square() {
    use crate::physics::*;

    let inputs = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    let outputs = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];

    //println!("\nDo 3 -> 18 which is close to circle perimeter 2*3.14*3\n");
    let eqs = find_equation(&[SCALAR_UNIT], &[SCALAR_UNIT], &inputs, &outputs);

    for (i, eq) in eqs.iter().enumerate() {
        let equation_info = &EQUATIONS[eq.0];
        println!("#{}: fit = {:8.2} {}", i+1, eq.1, equation_info.desc);
    }
/*
    let eq_index = get_equation_by_typeid(figure::circle::CirclePerimeter::params).unwrap();
    assert_eq!(eq_index, eqs[0].0);

    println!("\nNext do 3 -> 12.1 which is close to square perimeter 3*4\n");
    let eqs = find_equation(&[DISTANCE_UNIT], &[DISTANCE_UNIT], &[3.0], &[12.1]);

    for (i, eq) in eqs.iter().enumerate() {
        let equation_info = &EQUATIONS[eq.0];
        println!("#{}: fit = {:8.2} {}", i+1, eq.1, equation_info.desc);
    }

    let eq_index = get_equation_by_typeid(figure::rectangle::SquarePerimeter::params).unwrap();
    assert_eq!(eq_index, eqs[0].0);*/
}