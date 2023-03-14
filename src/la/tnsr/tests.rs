use crate::la::tnsr::*;

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
    let mut t = Tnsr::<f64>::new_vector(5);
    t.v = vec![1.0, 2.0, 3.0, 4.0, 5.0];

    // https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name
    assert_eq!(Vector::get(&t, 0), 1.0);
    assert_eq!(Vector::get(&t, 1), 2.0);
    assert_eq!(Vector::get(&t, 4), 5.0);

    let v: &mut dyn Vector<f64> = &mut t;
    assert_eq!(v.get(0), 1.0);
    v.set(1, 2.2);
    assert_eq!(v.get(1), 2.2);

    let v = &mut t as &mut dyn Vector::<f64>;
    assert_eq!(v.get(0), 1.0);
    v.set(2, 3.3);
    assert_eq!(v.get(2), 3.3);

    let v = &mut Tnsr::<f64>::new_vector(5) as &mut dyn Vector::<f64>;
    assert_eq!(v.get(1), 0.0);
    v.set(1, 7.7);
    assert_eq!(v.get(1), 7.7);
}

#[test]
fn matrix_new() {
    let mut t = Tnsr::<f64>::new_matrix(3, 4);
    t.v = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0];

    let m: &mut dyn Matrix<f64> = &mut t;
    assert_eq!(m.get(0,0), 1.0);
    assert_eq!(m.get(0,1), 2.0);
    assert_eq!(m.get(0,2), 3.0);
    assert_eq!(m.get(0,3), 4.0);
    assert_eq!(m.get(1,0), 5.0);
    assert_eq!(m.get(1,1), 6.0);
    assert_eq!(m.get(1,3), 8.0);
}