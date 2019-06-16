use crate::ch07_Quicksort::randomized_partition;
use std::cmp::PartialOrd;

pub fn randomized_select<T: PartialOrd + Copy>(A: &mut [T], i: usize) -> T {
    assert!(i < A.len());

    if A.len() == 1 {
        return A[0];
    }

    let q = randomized_partition(A);
    if q == i {
        A[q]
    } else if q < i {
        randomized_select(&mut A[q..], i - q)
    } else {
        randomized_select(&mut A[..q], i)
    }
}
