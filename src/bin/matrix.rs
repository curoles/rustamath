//! Matrix
//!
//! (c) 2013 Igor Lesik
//! MIT license
//!

//use inquire::{Text, validator::{StringValidator, Validation}};
//use inquire::{CustomType, validator::Validation};
use rustamath::la::tnsr::{Tnsr, Matrix, TranspMatrix};

fn main() {

    let mx_a = &mut Tnsr::<f64>::new_matrix(4, 2) as &mut dyn Matrix::<f64>;

    mx_a.set(0, 0, 1.1).set(0, 1, 2.2e12).set(1, 0, 3.3).set(1, 1, 4.4);
    mx_a.set(2, 0, 5.5).set(2, 1, 6.6e12).set(3, 0, 7.7).set(3, 1, 8.8);

    println!("{:?}", mx_a);

    let mx_at = mx_a.make_transposed();

    println!("transposed\n{:?}", mx_at);

    assert!(mx_a.is_transpose(&mx_at));

    let mx_d = &mut mx_a.raw_tensor().clone() as &mut dyn TranspMatrix::<f64>;
    mx_d.transpose();
    println!("transposed view\n{:?}", mx_d);

    mx_d.set(0, 3, 9.9);
    mx_d.transpose();
    println!("back to normal view\n{:?}", mx_d);

    let mx_b = &mut Tnsr::<f64>::new_matrix(5, 2) as &mut dyn Matrix::<f64>;
    mx_b.set(0, 0, 1.1).set(0, 1, 2.2);
    mx_b.set(1, 0, 3.3).set(1, 1, 4.4);
    mx_b.set(2, 0, 5.5).set(2, 1, 6.6);
    mx_b.set(3, 0, 7.7).set(3, 1, 8.8);
    mx_b.set(4, 0, 9.0).set(4, 1, 0.1);
    println!("b\n{:?}", mx_b);
    let mx_c = mx_b.raw_tensor().clone();
    mx_b.transpose();
    println!("hard transposed\n{:?}", mx_b);
    assert!(mx_b.is_transpose(&mx_c));
}