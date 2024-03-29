//! Tensor internal order

/// Tensor internal index-to-value mapping order type
///
/// <https://en.wikipedia.org/wiki/Row-_and_column-major_order>
///
pub enum TnsrOrderType {
    /// Dense with row-major
    RowMajor,
    /// Dense with column-major
    ColMajor,
    /// Sparse Hash(index -> value)
    SparseHash,
}

/// Control tensor's internal order
///
pub struct TnsrOrder {
    /// Internal order type
    pub kind: TnsrOrderType,

    /// Dense vs sparse
    pub is_sparse: bool,

    /// Function pointer `(i,j,k) -> index` in storage vector
    pub val_pos: fn(&Self, i: &[usize], sz: &[usize]) -> usize,
}

impl TnsrOrder {

    /// Create new Order with type and dimensions
    ///
    pub fn new(kind: TnsrOrderType, nr_dims: usize) -> Self {
        match kind {
            TnsrOrderType::RowMajor => TnsrOrder {
                kind,
                is_sparse: false,
                val_pos: match nr_dims {
                    1 => TnsrOrder::row_major_1d,
                    2 => TnsrOrder::row_major_2d,
                    _ => TnsrOrder::row_major_nd,
                }
            },
            TnsrOrderType::ColMajor => TnsrOrder {
                kind,
                is_sparse: false,
                val_pos: match nr_dims {
                    1 => TnsrOrder::col_major_1d,
                    2 => TnsrOrder::col_major_2d,
                    _ => TnsrOrder::col_major_nd,
                }
            },
            TnsrOrderType::SparseHash => TnsrOrder {
                kind,
                is_sparse: true,
                val_pos: match nr_dims {
                    1 => TnsrOrder::row_major_1d,//FIXME
                    2 => TnsrOrder::row_major_2d,
                    _ => TnsrOrder::row_major_nd,
                }
            },
        }
    }

    #[inline] fn row_major_1d(&self, i: &[usize], _sz: &[usize]) -> usize {
        i[0]
    }

    #[inline] fn row_major_2d(&self, i: &[usize], sz: &[usize]) -> usize {
        i[1] + i[0]*sz[1]
    }

    /// `iₖ + Sₖ·(iₖ₋₁ + Sₖ₋₁·(iₖ₋₂ + Sₖ₋₂·( ... + S₁i₀`
    /// see <https://en.wikipedia.org/wiki/Row-_and_column-major_order>
    fn row_major_nd(&self, i: &[usize], sz: &[usize]) -> usize {
        let mut n = i[0];
        for j in 1..i.len() {
            n = (n * sz[j]) + i[j]
        }
        n
    }

    #[inline] fn col_major_1d(&self, i: &[usize], _sz: &[usize]) -> usize {
        i[0]
    }

    #[inline] fn col_major_2d(&self, i: &[usize], sz: &[usize]) -> usize {
        i[0] + i[1]*sz[0]
    }

    fn col_major_nd(&self, i: &[usize], sz: &[usize]) -> usize {
        let mut n = i[i.len() - 1];
        for j in (0..i.len()-1).rev() {
            n = (n * sz[j]) + i[j]
        }
        n
    }

    // Change major
    //pub fn change_major(&mut self, to_col_major: bool) {
    //    self.val_pos = if to_col_major {TnsrOrder::col_major_2d} else {TnsrOrder::row_major_2d};
    //}
}

/// View at Tnsr.
///
/// View allow to change how external viewer sees the tensor
/// without changing the tensor internal structure.
///
#[derive(Copy, Clone)]
pub struct TnsrView {
    /// Transposed
    pub transposed: bool,
}