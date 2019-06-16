#![allow(non_snake_case, dead_code)]
mod common;

#[test]
fn counting_sort() {
    use CLRS::ch08_Sorting_in_Linear_Time::counting_sort::counting_sort;
    let mut A = common::random_vec_range::<i32>(100, 1050, 1100);
    counting_sort(A.as_mut_slice(), 1050, 1100);
    common::assert_asc(&A);
}

macro_rules! test_radix_sort {
    ($t:ty) => {
        let mut A = common::random_vec_range::<$t>(
            200,
            Bounded::min_value(),
            Bounded::max_value(),
        );
        radix_sort(A.as_mut_slice());
        common::assert_asc(&A);
    };
}

#[test]
fn radix_sort() {
    use num::Bounded;
    use CLRS::ch08_Sorting_in_Linear_Time::radix_sort::radix_sort;

    test_radix_sort!(i8);
    test_radix_sort!(i16);
    test_radix_sort!(i32);
    test_radix_sort!(i64);
    test_radix_sort!(isize);

    test_radix_sort!(u8);
    test_radix_sort!(u16);
    test_radix_sort!(u32);
    test_radix_sort!(u64);
    test_radix_sort!(usize);
}

#[test]
fn bucket_sort() {
    use num::Bounded;
    use CLRS::ch08_Sorting_in_Linear_Time::bucket_sort::bucket_sort;

    let mut A = common::random_vec_range::<f64>(
        200,
        Bounded::min_value(),
        Bounded::max_value(),
    );
    bucket_sort(A.as_mut_slice());
    common::assert_asc(&A);
}
