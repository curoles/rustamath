//! Vector operations with Intel x86  intrinsics

/*#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;*/

#[cfg(simd_arch = "x86_avx2")]
pub mod avx2;

#[cfg(simd_arch = "x86_avx512")]
pub mod avx512;
