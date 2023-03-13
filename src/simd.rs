//! Vector and matrix operations using SIMD instructions

//use std::simd::Simd;

pub mod noarch;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod x86;

/// vector operations using SIMD instructions
pub mod vec {

    use num_traits::float;

    #[cfg(simd_arch = "x86_avx2")]
    use crate::simd::x86::avx2;

    #[cfg(simd_arch = "x86_avx512")]
    use crate::simd::x86::avx512;

    /// Create new SIMD aligned `Vec<T>`
    ///
    /// # Example
    ///
    /// ```
    /// use rustamath::simd;
    /// let t = simd::vec::new::<f64>(16);
    /// #[cfg(any(simd_arch = "x86_avx2", simd_arch = "x86_avx512"))]
    /// assert_eq!(t.as_ptr() as usize % 32, 0);
    /// ```
    pub fn new<T>(size: usize) -> Vec<T> {
        #[cfg(simd_arch = "x86_avx2")]
        return maligned::align_first::<T, maligned::A32>(size);
        #[cfg(simd_arch = "x86_avx512")]
        return maligned::align_first::<T, maligned::A64>(size);
    }

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

    /// a[i] -> a[i]^2
    #[inline] pub fn pow2<T>(a: &mut[T])
        where T: std::ops::Mul<Output = T>,
              T: Copy,
              //T: std::simd::SimdElement
    {
        #[cfg(simd_arch = "x86_avx2")]
        avx2::pow2(a);
        #[cfg(simd_arch = "x86_avx512")]
        avx512::pow2(a);
    }


    /// `sqrt( sum(a[i]^2) )`
    #[inline] pub fn norm<T>(a: &[T]) -> T
        where T: std::ops::Mul<Output = T>,
              //T: std::ops::AddAssign,
              T: float::Float,
              //T: std::simd::SimdElement
    {
        #[cfg(simd_arch = "x86_avx2")]
        return avx2::norm(a);
        #[cfg(simd_arch = "x86_avx512")]
        return avx512::norm(a);
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
