//! Vector and matrix operations using SIMD instructions

//use std::simd::Simd;

pub mod noarch;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod x86;

/// vector operations using SIMD instructions
pub mod vec {

    #[cfg(simd_arch = "x86_avx2")]
    use crate::simd::x86::avx2;

    #[cfg(simd_arch = "x86_avx512")]
    use crate::simd::x86::avx512;

    /// add all elements of 2 vectors
    #[inline] pub fn add<T>(a: &mut[T], b: &[T])
        where T: std::ops::Add<Output = T>,
              T: Copy,
              //T: std::simd::SimdElement
    {
        #[cfg(simd_arch = "x86_avx2")]
        avx2::add(a, b);
        #[cfg(simd_arch = "x86_avx512")]
        avx512::add(a, b);
    }
}

#[cfg(test)]
mod tests {
    use crate::simd;

    #[test]
    fn add() {
        let mut az = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let bs = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        simd::vec::add(&mut az, &bs);
        assert_eq!(az[5], 10);
    }
}
