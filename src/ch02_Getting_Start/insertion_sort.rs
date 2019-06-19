use std::cmp::PartialOrd;

#[inline]
pub fn insertion_sort<T: PartialOrd + Clone>(A: &mut [T]) {
    for j in 1..A.len() {
        let mut i: isize = j as isize - 1;
        let key = A[j].clone();
        while i >= 0 && A[i as usize] > key {
            A[(i + 1) as usize] = A[i as usize].clone();
            i -= 1;
        }
        A[(i + 1) as usize] = key;
    }
}

mod tests {
    #[test]
    fn insertion_sort() {
        use super::insertion_sort;
        use crate::common;
        let mut A = common::random_vec::<f64>(100);
        insertion_sort(A.as_mut_slice());
        common::assert_asc(&A);
    }
}
