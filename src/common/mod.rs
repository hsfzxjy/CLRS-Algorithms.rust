extern crate rand;

use rand::distributions::{
    Distribution,
    Uniform,
};
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
    (0..n).map(|_| rand::random()).collect()
}

pub fn random_vec_range<T>(n: usize, low: T, high: T) -> Vec<T>
where
    T: rand::distributions::uniform::SampleUniform,
{
    let dist = Uniform::new(low, high);
    let mut rng = rand::thread_rng();

    (0..n).map(|_| dist.sample(&mut rng)).collect()
}
