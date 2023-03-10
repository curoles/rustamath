//! Vector and matrix operations without explicitly using SIMD instructions

/// add elements of 2 vectors
///
/// # Example
///
/// ```
/// # use rustomath::simd;
/// let mut az = [1, 2, 3, 4, 5];
/// let bs = [1, 2, 3, 4, 5];
/// simd::noarch::add(&mut az, &bs);
/// assert_eq!(az[4], 10);
/// ```
pub fn add<T>(az: &mut[T], bs: &[T])
   where T: std::ops::Add<Output = T>, T: Copy
{
    for i in 0..az.len() {
        az[i] = az[i] + bs[i];
    }
}
