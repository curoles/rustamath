//! Matrix
//!
//! (c) 2013 Igor Lesik
//! MIT license
//!

//use inquire::{Text, validator::{StringValidator, Validation}};
//use inquire::{CustomType, validator::Validation};
use rustamath::la::tnsr::{Tnsr, Matrix};

fn main() {

    let mx_a = &mut Tnsr::<f64>::new_matrix(8, 10) as &mut dyn Matrix::<f64>;

    mx_a.set(0, 0, 1.1).set(0, 1, 2.2e12);

    println!("print1: {:?}", mx_a);
}