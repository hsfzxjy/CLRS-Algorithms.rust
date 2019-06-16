use std::mem;

mod traits {
    use num::{FromPrimitive, PrimInt, ToPrimitive};
    use std::mem;

    pub trait Radix: Clone {
        fn n_buckets() -> usize;
        fn n_rounds() -> usize;
        fn get_bucket(&self, round: usize) -> usize;
    }

    #[inline]
    fn nth_hex_digit<T>(x: T, n: u32) -> u8
    where
        T: FromPrimitive + PrimInt + ToPrimitive,
    {
        // `n` start from 0
        let mask = T::from_u8(0xfu8).unwrap();
        x.unsigned_shr(n * 4).bitand(mask).to_u8().unwrap()
    }

    macro_rules! impl_radix_unsigned {
        ($t:ty) => {
            impl Radix for $t {
                #[inline]
                fn n_buckets() -> usize {
                    0x10
                }

                #[inline]
                fn n_rounds() -> usize {
                    mem::size_of::<$t>() * 2
                }

                #[inline]
                fn get_bucket(&self, round: usize) -> usize {
                    nth_hex_digit(*self, round as u32) as usize
                }
            }
        };
    }

    macro_rules! impl_radix_signed {
        ($t:ty) => {
            impl Radix for $t {
                #[inline]
                fn n_buckets() -> usize {
                    0x10
                }

                #[inline]
                fn n_rounds() -> usize {
                    mem::size_of::<$t>() * 2 + 1
                }

                #[inline]
                fn get_bucket(&self, round: usize) -> usize {
                    if round < <$t as Radix>::n_rounds() - 1 {
                        return nth_hex_digit(*self, round as u32) as usize;
                    }

                    if *self < 0 {
                        0
                    } else if *self == 0 {
                        1
                    } else {
                        2
                    }
                }
            }
        };
    }

    impl_radix_unsigned!(u8);
    impl_radix_unsigned!(u16);
    impl_radix_unsigned!(u32);
    impl_radix_unsigned!(u64);
    impl_radix_unsigned!(usize);

    impl_radix_signed!(i8);
    impl_radix_signed!(i16);
    impl_radix_signed!(i32);
    impl_radix_signed!(i64);
    impl_radix_signed!(isize);
}

use traits::*;

const BUCKET_NUM: usize = 0x10;

#[inline]
pub fn fill_bucket_from_iter<T, I>(buckets: &mut Vec<Vec<T>>, iter: I, round: usize)
where
    T: Radix,
    I: Iterator<Item = T>,
{
    for x in iter {
        buckets[x.get_bucket(round)].push(x);
    }
}

pub fn radix_sort<T>(A: &mut [T])
where
    T: Radix,
{
    let n_rounds = T::n_rounds();
    let n_buckets = T::n_buckets();
    let bucket_preset_capacity = A.len() / n_buckets;

    let mut buckets_A: Vec<Vec<T>> = Vec::with_capacity(n_buckets);
    let mut buckets_B: Vec<Vec<T>> = Vec::with_capacity(n_buckets);

    for _ in 0..n_buckets {
        buckets_A.push(Vec::with_capacity(bucket_preset_capacity));
        buckets_B.push(Vec::with_capacity(bucket_preset_capacity));
    }

    fill_bucket_from_iter(&mut buckets_A, A.iter().cloned(), 0);

    for round in 1..n_rounds {
        for bucket in &mut buckets_B {
            bucket.clear()
        }

        for bucket in &buckets_A {
            fill_bucket_from_iter(&mut buckets_B, bucket.iter().cloned(), round)
        }

        mem::swap(&mut buckets_A, &mut buckets_B);
    }

    let mut i = 0;
    buckets_A.into_iter().for_each(|bucket| {
        bucket.into_iter().for_each(|x| {
            A[i] = x;
            i += 1;
        })
    });
}
