#![allow(non_snake_case, dead_code)]

use rand;
use rand::Rng;
mod common;

#[test]
fn randomized_select() {
    use CLRS::ch09_Medians_and_Order_Statistics::randomized_select::randomized_select;
    let mut A = common::random_vec::<i64>(100);
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0, 100);
    let x = randomized_select(A.as_mut_slice(), i);
    A.sort();
    assert_eq!(x, A[i]);
}

#[test]
fn select() {
    use CLRS::ch09_Medians_and_Order_Statistics::select::select;
    let mut A = common::random_vec::<i64>(100);
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0, 100);
    let x = select(A.as_mut_slice(), i);
    A.sort();
    assert_eq!(x, A[i]);
}
