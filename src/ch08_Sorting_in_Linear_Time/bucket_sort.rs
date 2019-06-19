use num::{
    Bounded,
    Num,
};
use std::cmp::PartialOrd;

pub fn bucket_sort<T>(A: &mut [T])
where
    T: Bounded + Num + PartialOrd + Copy,
    T: Into<f64>,
{
    use crate::ch02_Getting_Start::insertion_sort::insertion_sort;

    let (margin, min, max) = {
        let mut min = T::max_value();
        let mut max = T::min_value();

        for &x in A.iter() {
            if x < min {
                min = x
            }
            if x > max {
                max = x
            }
        }

        ((max - min).into() / A.len() as f64, min, max)
    };

    if min == max {
        return;
    }

    let mut buckets: Vec<Vec<T>> = Vec::with_capacity(A.len() + 1);
    for _ in 0..=A.len() {
        buckets.push(Vec::new());
    }

    for &x in A.iter() {
        let bucket_id = ((x - min).into() / margin) as usize;
        buckets[bucket_id].push(x);
    }

    for bucket in buckets.iter_mut() {
        insertion_sort(bucket);
    }

    let mut i = 0;
    for bucket in buckets.iter() {
        for &x in bucket.iter() {
            A[i] = x;
            i += 1;
        }
    }
}

mod tests {
    #[test]
    fn bucket_sort() {
        use super::bucket_sort;
        use crate::common;
        use num::Bounded;

        let mut A = common::random_vec_range::<f64>(
            200,
            Bounded::min_value(),
            Bounded::max_value(),
        );
        bucket_sort(A.as_mut_slice());
        common::assert_asc(&A);
    }
}
