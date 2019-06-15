#![allow(non_snake_case, dead_code)]
mod common;

use CLRS::ch07_Quicksort::*;

#[test]
fn quick_sort_last() {
    let mut A = common::random_vec::<f64>(100);
    quick_sort(A.as_mut_slice(), PartitionBy::Last);
    common::assert_asc(&A);
}

#[test]
fn quick_sort_random() {
    let mut A = common::random_vec::<f64>(100);
    quick_sort(A.as_mut_slice(), PartitionBy::Random);
    common::assert_asc(&A);
}
