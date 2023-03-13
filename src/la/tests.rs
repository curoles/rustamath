use crate::la::*;

#[test]
fn vec_new() {
    let mut v = Tnsr::<f64>::new_vector(10);
    assert_eq!(v.raw().capacity(), 10);
    assert_eq!(v.raw().len(), 10);
    assert_eq!(v.v.capacity(), 10);
    assert_eq!(v.v.len(), 10);
    assert_eq!(v.v, vec![0.0; 10]);
    v.v = vec![1.234; 10];
    assert_eq!(v.v, vec![1.234; 10]);
    assert_eq!(None, v.v.get(10));
    assert_eq!(Some(&1.234), v.v.get(9));
    assert_eq!(Some(&[1.234, 1.234][..]), v.v.get(0..2));
    v.v[6] = 5.67;
    assert_eq!(5.67, v.v[6]);

    let mut v = Tnsr::<f64>::new_vector(2);
    v.v = vec![3.0, 4.0]; // 3^2 + 4^2 = 25
    assert_f64_near!(5.0, v.norm());

    assert_eq!(v.nr_dims(), 1);
    assert_eq!(v.sizes(), &[2]);
    assert_eq!(v.dim(0), Some(2));
    assert_eq!(v.dim(1), None);
}

#[test]
fn vec_get_at() {
    let mut v = Tnsr::<f64>::new_vector(5);
    v.v = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    assert_eq!(v.get_at_row(0), 1.0);
    assert_eq!(v.get_at_row(1), 2.0);
    assert_eq!(v.get_at_row(4), 5.0);
}