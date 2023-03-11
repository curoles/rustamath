// https://doc.rust-lang.org/cargo/reference/build-scripts.html

fn main() {

    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),target_feature = "avx512"))]
    println!("cargo:rustc-cfg=simd_arch=\"x86_avx512\"");

    #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),target_feature = "avx2",not(target_feature = "avx512")))]
    println!("cargo:rustc-cfg=simd_arch=\"x86_avx2\"");

}
