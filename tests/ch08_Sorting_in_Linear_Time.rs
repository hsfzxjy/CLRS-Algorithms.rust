#![allow(non_snake_case, dead_code)]
mod common;

#[test]
fn counting_sort() {
    use CLRS::ch08_Sorting_in_Linear_Time::counting_sort::counting_sort;
    let mut A = common::random_vec_range::<i32>(100, 1050, 1100);
    counting_sort(A.as_mut_slice(), 1050, 1100);
    common::assert_asc(&A);
}