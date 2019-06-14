#![allow(non_snake_case, dead_code)]

#[test]
fn heapsort() {
    use CLRS::ch06_Heapsort::heap_sort;
    let mut A = vec![3, 4, 5, 1, 5, 7];
    heap_sort(A.as_mut_slice());
    println!("{:?}", A);
    for i in 0..A.len() - 1 {
        assert!(A[i] <= A[i + 1]);
    }
}
