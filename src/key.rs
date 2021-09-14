/// Trait that must be implemented for all key types used in a [`ContiguousMap`](crate::ContiguousMap).
pub trait Key
where
    Self: Sized + Clone + Ord + Eq,
{
    /// Gets the next adjacent key.
    /// Returns None if there is no adjacent key due to self being the max key.
    fn add_one(&self) -> Option<Self>;

    /// Gets the difference between this key and another one.
    /// Returns None if the difference does not fit in a usize.
    fn difference(&self, smaller: &Self) -> Option<usize>;
}

macro_rules! unsigned_key_impl {
    ($type:ty) => {
        impl Key for $type {
            fn add_one(&self) -> Option<Self> {
                self.checked_add(1)
            }

            fn difference(&self, smaller: &Self) -> Option<usize> {
                use std::convert::TryInto;
                self.checked_sub(*smaller)
                    .map(|value| value.try_into().ok())
                    .flatten()
            }
        }
    };
}

unsigned_key_impl!(u8);
unsigned_key_impl!(u16);
unsigned_key_impl!(u32);
unsigned_key_impl!(u64);
unsigned_key_impl!(u128);
unsigned_key_impl!(usize);

macro_rules! signed_key_impl {
    ($type:ty) => {
        impl Key for $type {
            fn add_one(&self) -> Option<Self> {
                self.checked_add(1)
            }

            fn difference(&self, smaller: &Self) -> Option<usize> {
                use std::convert::TryInto;
                let unsigned = match (*self < 0, *smaller < 0) {
                    (true, true) => {
                        // both negative
                        smaller.unsigned_abs().checked_sub(self.unsigned_abs())?
                    }
                    (true, false) => {
                        // self negative, smaller positive, so self < smaller.
                        return None;
                    }
                    (false, true) => {
                        // self positive, smaller negative
                        self.unsigned_abs().checked_add(smaller.unsigned_abs())?
                    }
                    (false, false) => {
                        // both positive
                        self.unsigned_abs().checked_sub(smaller.unsigned_abs())?
                    }
                };
                unsigned.try_into().ok()
            }
        }
    };
}

signed_key_impl!(i8);
signed_key_impl!(i16);
signed_key_impl!(i32);
signed_key_impl!(i64);
signed_key_impl!(i128);
signed_key_impl!(isize);

#[cfg(test)]
mod test {
    use crate::Key;

    #[test]
    fn usize_add_one() {
        use super::Key;
        assert_eq!(Some(1), 0usize.add_one());
        assert_eq!(Some(2), 1usize.add_one());
        assert_eq!(Some(usize::MAX), (usize::MAX - 1).add_one());
        assert_eq!(None, usize::MAX.add_one());
    }

    #[test]
    fn usize_difference() {
        use super::Key;
        assert_eq!(Some(1), 5usize.difference(&4));
        assert_eq!(Some(0), 5usize.difference(&5));
        assert_eq!(None, 5usize.difference(&6));
    }

    #[test]
    fn u128_difference_overflow() {
        if usize::BITS < 128 {
            assert_eq!(None, u128::MAX.difference(&0));
            assert_eq!(None, (u128::MAX - 10).difference(&10));
        }
    }

    #[test]
    fn i8_add_one() {
        assert_eq!(-99, (-100i8).add_one().unwrap());
        assert_eq!(10, 9i8.add_one().unwrap());
        assert_eq!(None, i8::MAX.add_one());
    }

    #[test]
    fn i8_difference() {
        assert_eq!(2, (-3i8).difference(&-5).unwrap());
        assert_eq!(127 as usize, (-1i8).difference(&-128).unwrap());
        assert_eq!(None, (-3i8).difference(&2));
        assert_eq!(5, 2i8.difference(&-3).unwrap());
        assert_eq!(u8::MAX as usize, i8::MAX.difference(&i8::MIN).unwrap());
        assert_eq!(5, 12i8.difference(&7).unwrap());
    }

    #[test]
    fn i128_difference_overflow() {
        if usize::BITS < 128 {
            assert_eq!(None, i128::MAX.difference(&0));
            assert_eq!(None, 0i128.difference(&i128::MIN));
        }
    }
}
