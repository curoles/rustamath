//! Functions to generate random lottery numbers

use rand::Rng;
use num_traits::{Num, One};
use num_traits::cast::AsPrimitive;

/// Create lottery sequence
///
/// `T: std::iter::Step` causes:
/// `error[E0658]: use of unstable library feature 'step_trait': recently redesigned`
///
pub fn lottery<T>(low: T, high: T) -> Vec<T>
where
    T: Num + One + PartialOrd + Copy + Clone + std::fmt::Debug,
    T: AsPrimitive<usize>
{
    let mut list = Vec::<T>::new();

    let one = T::one();

    // create list[low, low+1, low+2, ..., high]
    let mut val = low;
    while val <= high {
        list.push(val);
        val = val + one;
    }

    // randomly shuffle
    let mut rng = rand::thread_rng();
    let range_len: usize = (high - low + one).as_(); // num_traits::cast::AsPrimitive
    for len in (2..=range_len).rev() {
        let random_index = rng.gen_range(0..len);
        list.swap(random_index, len-1);
        //println!("len={} range={:?} index={} list={:?}",
        //    len, (0..len), random_index, list);
    }

    list
}

#[cfg(test)]
#[test]
fn test_lottery() {
    let list = lottery(1u32, 100u32);
    assert_eq!(list.len(), 100);

    let has_duplicates = (1..list.len())
        .any(|i| list[i..].contains(&list[i - 1]));
    assert_eq!(has_duplicates, false);
}
