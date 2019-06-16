use std::cmp::PartialOrd;

pub fn bubble_sort<T: PartialOrd>(A: &mut [T]) {
    for i in 0..A.len() - 1 {
        for j in (i + 1..A.len()).rev() {
            if A[j] < A[j - 1] {
                A.swap(j, j - 1)
            }
        }
    }
}
