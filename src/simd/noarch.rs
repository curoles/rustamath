//! Vector and matrix operations without explicitly using SIMD instructions

/// add elements of 2 vectors
pub fn add<T: std::ops::Add<Output = T>>(az: &mut[T], _bs: &[T])
{
    for (_i, _a) in az.iter().enumerate() {
        //*a = *a + bs[i];
    }
}
