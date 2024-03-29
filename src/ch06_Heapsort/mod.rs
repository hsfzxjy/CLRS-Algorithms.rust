use std::cmp::PartialOrd;

#[inline]
fn parent(i: usize) -> usize {
    (i + 1) / 2 - 1
}

#[inline]
fn left(i: usize) -> usize {
    (i + 1) * 2 - 1
}

#[inline]
fn right(i: usize) -> usize {
    (i + 1) * 2
}

fn init_heap<T: PartialOrd>(A: &mut [T]) {
    let last_parent = parent(A.len() - 1);
    for i in (0..last_parent + 1).rev() {
        heapify(A, i);
    }
}

fn heapify<T: PartialOrd>(A: &mut [T], i: usize) {
    let (l, r) = (left(i), right(i));
    let mut largest = if l < A.len() && A[l] > A[i] {
        l
    } else {
        i
    };
    if r < A.len() && A[r] > A[largest] {
        largest = r;
    }
    if largest != i {
        A.swap(largest, i);
        heapify(A, largest);
    }
}

pub fn heap_sort<T: PartialOrd>(A: &mut [T]) {
    if A.len() <= 1 {
        return;
    }

    init_heap(A);

    for i in (1..A.len()).rev() {
        A.swap(0, i);
        heapify(&mut A[..i], 0);
    }
}

mod tests {
    #[test]
    fn heap_sort() {
        use super::heap_sort;
        use crate::common;
        let mut A = common::random_vec::<f64>(100);
        heap_sort(A.as_mut_slice());
        common::assert_asc(&A);
    }
}
