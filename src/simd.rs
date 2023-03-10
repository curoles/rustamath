//! Vector and matrix operations using SIMD instructions

pub mod noarch;
pub mod x86_avx2;

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),target_feature = "avx512"))]
pub mod vec {
    pub fn add() {
    }
}

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),target_feature = "avx2",not(target_feature = "avx512")))]
/// vector operations using SIMD instructions
pub mod vec {

    use crate::simd::x86_avx2;

    /// add all elements of 2 vectors
    #[inline] pub fn add<T>(a: &[T], b: &[T]) {
        x86_avx2::add(a, b);
    }
}
