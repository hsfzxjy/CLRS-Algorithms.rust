use crate::{
    ch02_Getting_Start::insertion_sort::insertion_sort,
    ch07_Quicksort::partition_by,
};
use std::cmp::PartialOrd;
use std::f64;

pub fn select<T: PartialOrd + Copy>(A: &mut [T], i: usize) -> T {
    assert!(i < A.len());

    if A.len() == 1 {
        return A[0];
    }

    let mut medians = {
        let mut medians =
            Vec::with_capacity((A.len() as f64 / 5f64).ceil() as usize);

        for chunk in A.chunks_mut(5) {
            insertion_sort(chunk);
            medians.push(chunk[chunk.len() / 2]);
        }
        medians
    };

    let m_of_m_index = {
        let median_index = medians.len() / 2;
        let m_of_m = select(&mut medians, median_index);
        A.iter().position(|&r| r == m_of_m).unwrap()
    };
    let q = partition_by(A, m_of_m_index);

    if q == i {
        A[q]
    } else if q < i {
        select(&mut A[q..], i - q)
    } else {
        select(&mut A[..q], i)
    }
}
