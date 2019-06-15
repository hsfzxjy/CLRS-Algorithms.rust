extern crate rand;

use rand::Rng;
use std::cmp::PartialOrd;

fn partition<T: PartialOrd>(A: &mut [T]) -> usize {
    assert!(A.len() > 1);

    let last_index = A.len() - 1;
    let mut i: i32 = -1;
    for j in 0..A.len() - 1 {
        if A[j] <= A[last_index] {
            i += 1;
            A.swap(i as usize, j);
        }
    }
    let result = (i + 1) as usize;
    A.swap(result, A.len() - 1);
    result
}

fn randomized_partition<T: PartialOrd>(A: &mut [T]) -> usize {
    let mut rnd = rand::thread_rng();
    let q = rnd.gen_range(0, A.len());
    A.swap(q, A.len() - 1);
    partition(A)
}

#[derive(Copy, Clone)]
pub enum PartitionBy {
    Last,
    Random,
}

pub fn quick_sort<T: PartialOrd>(A: &mut [T], partition_type: PartitionBy) {
    if A.len() <= 1 {
        return;
    }
    let q = match partition_type {
        PartitionBy::Last => partition(A),
        PartitionBy::Random => randomized_partition(A),
    };
    quick_sort(&mut A[..q], partition_type);
    quick_sort(&mut A[q..], partition_type);
}
