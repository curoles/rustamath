//! Matrix
//!
//! (c) 2013 Igor Lesik
//! MIT license
//!

//use inquire::{Text, validator::{StringValidator, Validation}};
//use inquire::{CustomType, validator::Validation};
use rustamath::la::tnsr::{Tnsr, Matrix};

fn main() {

    let mx_a = &mut Tnsr::<f64>::new_matrix(4, 2) as &mut dyn Matrix::<f64>;

    mx_a.set(0, 0, 1.1).set(0, 1, 2.2e12).set(1, 0, 3.3).set(1, 1, 4.4);
    mx_a.set(2, 0, 5.5).set(2, 1, 6.6e12).set(3, 0, 7.7).set(3, 1, 8.8);

    println!("{:?}", mx_a);

    let mx_at = mx_a.make_transposed();

    println!("transposed\n{:?}", mx_at);

    assert!(mx_a.is_transpose(&mx_at));

    mx_a.transpose_view();
    println!("transposed view\n{:?}", mx_a);

    mx_a.set(0, 3, 9.9);
    mx_a.transpose_view();
    println!("back to normal view\n{:?}", mx_a);

    let mx_b = &mut Tnsr::<f64>::new_matrix(2, 2) as &mut dyn Matrix::<f64>;
    mx_b.set(0, 0, 1.1).set(0, 1, 2.2).set(1, 0, 3.3).set(1, 1, 4.4);
    println!("b\n{:?}", mx_b);
    mx_b.transpose();
    println!("hard transposed\n{:?}", mx_b);
}