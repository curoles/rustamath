//! Vector operations with AVX2 intrinsics

use std::mem;
//https://doc.rust-lang.org/core/simd/struct.Simd.html
//https://github.com/rust-lang/rust/issues/86656
//check https://github.com/AdamNiederer/faster
//use std::simd::Simd;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

// https://doc.rust-lang.org/core/arch/x86/struct.__m256i.html#impl-From%3CSimd%3Ci32%2C%208%3E%3E-for-__m256i
//https://doc.rust-lang.org/core/arch/x86_64/fn._mm256_add_epi32.html
//pub unsafe fn _mm256_add_epi32(a: __m256i, b: __m256i) -> __m256i

fn binary_op<T, const VL: usize>
   (op: fn(T,T) -> T, az: &mut[T], bs: &[T])
       where T: Copy,
             //T: std::simd::SimdElement
{
    let _nr: usize = (VL/8)/mem::size_of::<T>();
    //let _simd_vec: Simd<T, {_nr}>;
    //pub const fn from_slice(slice: &[T]) -> Simd<T, LANES>
    let _chunk: __m256i;// = __m256i::from(simd_vec);;

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
   where T: std::ops::Add<Output = T>,
         T: Copy,
         //T: std::simd::SimdElement
{
    binary_op::<T, 256>(std::ops::Add::<T>::add, az, bs);
    //TODO;
    //_mm256_add_epi64(...);
}
