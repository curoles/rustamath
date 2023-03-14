


- polynomial tool
  * [x] eval x
  * [] roots range
  * [] plot with https://crates.io/crates/plotters
- polynomial find roots
- LA module
  * UI, display vector and matrix
- UI/CLI crates
  * inquire
  * comfy-table
  * prettytable-rs
- use axv2 functions for avx512 tails, no need to write tails for 512
- Sparse Tensor
  * struct SparseTnsr
  * use `HashMap([i0,i1,i2], val:T)` to store/find value
  * maybe, keep the storage vector, but map `HashMap([i0,i1,i2], vector_index)`
  * whould be nice to extend Tnsr; generic param? check mixin pattern
  * https://eigen.tuxfamily.org/dox/group__TutorialSparse.html
  * crate `sprs`
  * crate `sparse21`
  * IndexValMap as generic parameter? then row/col major becomes this param as well
- LA lib implementations
  * https://www.andreinc.net/2021/01/20/writing-your-own-linear-algebra-matrix-library-in-c
- SIMD
  * https://doc.rust-lang.org/core/simd/struct.Simd.html
  * https://github.com/rust-lang/rust/issues/86656
  * check https://github.com/AdamNiederer/faster
  * use std::simd::Simd;
