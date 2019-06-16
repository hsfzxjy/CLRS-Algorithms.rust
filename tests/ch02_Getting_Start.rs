#![allow(non_snake_case, dead_code)]

mod common;

#[test]
fn insertion_sort() {
    use CLRS::ch02_Getting_Start::insertion_sort::insertion_sort;
    let mut A = common::random_vec::<f64>(100);
    insertion_sort(A.as_mut_slice());
    common::assert_asc(&A);

}