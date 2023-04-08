//! Linear equation solution by Gauss-Jordan elimination.
//!
//! (c) 2023 Igor Lesik
//! MIT license
//!


/// Linear equation solution by Gauss-Jordan elimination.
///
/// Reference: William H. Press - Numerical recipes, the art of scientific computing.
/// Cambridge University Press (2007).
///
/// The input matrix is `a[0..n-1][0..n-1]`.
/// `b[0..n-1][0..m-1]` is input containing the `m` right-hand side vectors.
/// On output, `a` is replaced by its matrix inverse, and `b` is replaced by
/// the corresponding set of solution vectors.
///
/// n -numbers of unknowns, m - number of equations.
pub fn gauss_jordan(n: usize, m: usize, a: &mut [f64], b: &mut [f64]) {
    // n=a.nrows(), m=b.ncols();
    assert!(a.len() >= n*n);
    assert!(b.len() >= n*m);

    // These integer arrays are used for bookkeeping on the pivoting.
    let mut indxc = vec![0; n];
    let mut indxr = vec![0; n];
    let mut ipiv  = vec![0; n];
}