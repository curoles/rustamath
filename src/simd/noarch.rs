//! Vector and matrix operations without explicitly using SIMD instructions

/// add elements of 2 vectors
pub fn add<T>(az: &mut[T], bs: &[T])
   where T: std::ops::Add<Output = T>, T: Copy
{
    for i in 0..az.len() {
        az[i] = az[i] + bs[i];
    }
}
