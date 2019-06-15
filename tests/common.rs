extern crate rand;

use std::cmp::PartialOrd;

pub fn assert_asc<T: PartialOrd>(arr: &[T]) {
    for i in 0..arr.len() - 1 {
        assert!(arr[i] <= arr[i + 1])
    }
}

pub fn assert_desc<T: PartialOrd>(arr: &[T]) {
    for i in 0..arr.len() - 1 {
        assert!(arr[i] >= arr[i + 1])
    }
}

pub fn random_vec<T>(n: usize) -> Vec<T>
where
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    let mut result = Vec::with_capacity(n);

    for _ in 0..n {
        result.push(rand::random::<T>());
    }
    result
}
