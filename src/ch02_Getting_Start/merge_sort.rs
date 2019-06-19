use std::cmp::PartialOrd;

fn merge<T: PartialOrd + Clone>(A: &[T], B: &[T], out: &mut Vec<T>) {
    let mut i = 0;
    let mut j = 0;

    for _ in 0..A.len() + B.len() {
        if j == B.len() || i < A.len() && A[i] < B[j] {
            out.push(A[i].clone());
            i += 1;
        } else {
            out.push(B[j].clone());
            j += 1;
        }
    }
}

pub fn merge_sort<T: PartialOrd + Clone>(A: &mut [T]) {
    if A.len() <= 1 {
        return;
    }

    let mid = A.len() / 2;
    merge_sort(&mut A[..mid]);
    merge_sort(&mut A[mid..]);
    let mut temp = Vec::with_capacity(A.len());
    merge(&A[..mid], &A[mid..], &mut temp);
    A.clone_from_slice(&temp);
}

mod tests {
    #[test]
    fn merge_sort() {
        use super::merge_sort;
        use crate::common;
        let mut A = common::random_vec::<f64>(10);
        merge_sort(A.as_mut_slice());
        common::assert_asc(&A);
    }
}
