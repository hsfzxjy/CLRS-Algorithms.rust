use crate::ch07_Quicksort::randomized_partition;
use std::cmp::PartialOrd;

pub fn randomized_select<T: PartialOrd + Copy>(A: &mut [T], i: usize) -> T {
    assert!(i < A.len());

    if A.len() == 1 {
        return A[0];
    }

    let q = randomized_partition(A);
    if q == i {
        A[q]
    } else if q < i {
        randomized_select(&mut A[q..], i - q)
    } else {
        randomized_select(&mut A[..q], i)
    }
}

mod tests {
    #[test]
    fn randomized_select() {
        use super::randomized_select;
        use crate::common;
        use rand;
        use rand::Rng;
        let mut A = common::random_vec::<i64>(100);
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0, 100);
        let x = randomized_select(A.as_mut_slice(), i);
        A.sort();
        assert_eq!(x, A[i]);
    }

}
