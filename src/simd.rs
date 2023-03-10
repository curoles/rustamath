//! Vector and matrix operations using SIMD instructions

pub mod noarch;

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),target_feature = "avx2",not(target_feature = "avx512")))]
pub mod x86_avx2;

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),target_feature = "avx512"))]
pub mod x86_avx512;

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),target_feature = "avx512"))]
/// vector operations using SIMD instructions
pub mod vec {

    use crate::simd::x86_avx512;

    /// add all elements of 2 vectors
    #[inline] pub fn add<T>(a: &mut[T], b: &[T]) {
        x86_avx512::add(a, b);
    }
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),target_feature = "avx2",not(target_feature = "avx512")))]
/// vector operations using SIMD instructions
pub mod vec {

    use crate::simd::x86_avx2;

    /// add all elements of 2 vectors
    #[inline] pub fn add<T>(a: &mut[T], b: &[T])
        where T: std::ops::Add<Output = T>, T: Copy
    {
        x86_avx2::add(a, b);
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
