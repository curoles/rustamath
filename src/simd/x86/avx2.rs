//! Vector operations with AVX2 intrinsics

/*#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;*/

fn binary_op<T>(op: fn(T,T) -> T, az: &mut[T], bs: &[T])
   where T: Copy
{
    for i in 0..az.len() {
        az[i] = op(az[i], bs[i]);
    }
}

/// add elements of 2 vectors with AVX2 XXX
///
/// # Example
///
/// ```
/// # use rustomath::simd;
/// let mut az = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
/// let bs = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
/// simd::x86::avx2::add(&mut az, &bs);
/// assert_eq!(az[5], 10);
/// ```
pub fn add<T>(az: &mut[T], bs: &[T])
   where T: std::ops::Add<Output = T>, T: Copy
{
    binary_op::<T>(std::ops::Add::<T>::add, az, bs);
    //TODO;
    //_mm256_add_epi64(...);
}
