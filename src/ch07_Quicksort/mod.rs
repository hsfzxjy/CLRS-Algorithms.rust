extern crate rand;

use rand::Rng;
use std::cmp::PartialOrd;

fn partition<T: PartialOrd>(A: &mut [T]) -> usize {
    // `partition` fails if `A` has less than 2 elements.
    // Caller should avoid this.
    assert!(A.len() > 1);

    // Note that we don't write
    // `let ref_to_x = A.last().unwrap();` (which borrows `A` as immutable)
    // or
    // `let x = A.last().cloned().unwrap();` (which requires `T` to implement `Clone` trait)
    let last_index = A.len() - 1;

    // Note that `i` should be some signed type.
    let mut i: i64 = -1;

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
