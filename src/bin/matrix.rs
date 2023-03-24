//! Matrix
//!
//! (c) 2013 Igor Lesik
//! MIT license
//!

//use inquire::{Text, validator::{StringValidator, Validation}};
//use inquire::{CustomType, validator::Validation};
use rustamath::la::tnsr::{Tnsr, Matrix};

fn main() {

    let m = &Tnsr::<f64>::new_matrix(4, 3) as &dyn Matrix::<f64>;

    println!("print1: {:?}", m);
    println!("print2: {:#?}", m);
}