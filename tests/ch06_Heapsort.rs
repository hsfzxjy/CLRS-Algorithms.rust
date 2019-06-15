#![allow(non_snake_case, dead_code)]

mod common;

#[test]
fn heap_sort() {
    use CLRS::ch06_Heapsort::heap_sort;
    let mut A = common::random_vec::<f64>(100);
    heap_sort(A.as_mut_slice());
    common::assert_asc(&A);
}
