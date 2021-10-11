use std::convert::TryInto;

/// Trait that must be implemented for all key types
/// used in a [`ContiguousMap`](crate::ContiguousMap).
///
/// See the blanket implementation for an alternative way of
/// implementing this trait using the [`ToIndex`] and [`TryFromIndex`]
/// traits.
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

    /// Gets the key that is num steps after this key.
    /// Returns None if this overflows the key type.
    fn add_usize(&self, num: usize) -> Option<Self>;
}

/// Trait to convert a type to an index that implements the [`Key`] trait.
///
/// This can be implemented in combination with [`TryFromIndex`] to
/// enable a blanket implementation of the [`Key`] trait.
pub trait ToIndex {
    /// The key type used as an index for this type.
    type Index: Key;

    /// Converts self to an index.
    fn to_index(&self) -> Self::Index;
}

/// Trait to convert an index that implements the [`Key`] trait
/// to a different type.
///
/// This can be implemented in combination with [`ToIndex`] to
/// enable a blanket implementation of the [`Key`] trait.
pub trait TryFromIndex: ToIndex
where
    Self: Sized,
{
    /// Attempts to convert from an index to this type.
    fn try_from_index(index: Self::Index) -> Option<Self>;
}

/// Blanket implementation of the [`Key`] trait for any type that implements
/// [`ToIndex`], [`TryFromIndex`], and the basic trait requirements of [`Key`].
///
/// Self and it's index type must have the same ordering relationship.
/// This means that `x.cmp(&y)` must always be equivalent to
/// `x.to_index().cmp(&y.to_index())`.
///
/// All indexes for a type must be a contiguous group of the values of the index
/// type.  There is no requirement for what values this contiguous group starts
/// and stops at.
///
/// # Example
/// ```
/// use contiguous_map::cmap;
///
/// #[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
/// enum MyKey {
///     One = 1,
///     Two = 2,
///     Three = 3,
/// }
///
/// impl contiguous_map::ToIndex for MyKey {
///     type Index = u8;
///     fn to_index(&self) -> Self::Index {
///         *self as Self::Index
///     }
/// }
///
/// impl contiguous_map::TryFromIndex for MyKey {
///     fn try_from_index(index: Self::Index) -> Option<Self> {
///         match index {
///             1 => Some(Self::One),
///             2 => Some(Self::Two),
///             3 => Some(Self::Three),
///             _ => None
///         }
///     }
/// }
///
/// let map = cmap!(MyKey::One => 10, 12);
/// assert_eq!(Some(&10), map.get(MyKey::One));
/// assert_eq!(Some(&12), map.get(MyKey::Two));
/// assert_eq!(None, map.get(MyKey::Three));
/// ```
impl<T, I: Key> Key for T
where
    Self: Sized + Clone + Ord + Eq + ToIndex<Index = I> + TryFromIndex,
{
    fn add_one(&self) -> Option<Self> {
        self.to_index()
            .add_one()
            .map(Self::try_from_index)
            .flatten()
    }

    fn difference(&self, smaller: &Self) -> Option<usize> {
        self.to_index().difference(&smaller.to_index())
    }

    fn add_usize(&self, num: usize) -> Option<Self> {
        self.to_index()
            .add_usize(num)
            .map(Self::try_from_index)
            .flatten()
    }
}

macro_rules! unsigned_key_impl {
    ($type:ty) => {
        impl Key for $type {
            fn add_one(&self) -> Option<Self> {
                self.checked_add(1)
            }

            fn difference(&self, smaller: &Self) -> Option<usize> {
                self.checked_sub(*smaller)
                    .map(|value| value.try_into().ok())
                    .flatten()
            }

            fn add_usize(&self, num: usize) -> Option<Self> {
                self.checked_add(num.try_into().ok()?)
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

            fn add_usize(&self, num: usize) -> Option<Self> {
                if *self >= 0 {
                    // self non-negative, so num must fit in the signed type for
                    // the addition to possibly not overflow.
                    // Therefore can do the addition in the singed type
                    self.checked_add(num.try_into().ok()?)
                } else {
                    // self negative, num must only fit in the unsigned version of
                    // the signed type for the addition to possibly not overflow
                    let negated_self = self.unsigned_abs();
                    let num = num.try_into().ok()?;
                    if negated_self <= num {
                        // self is negative and has smaller than or equal magnitude to num,
                        // the result of the addition will be non-negative
                        (num - negated_self).try_into().ok()
                    } else {
                        // self is negative and has larger magnitude than num,
                        // the result of the addition will be negative.
                        // Additionally since the result of the addition is negative and
                        // self fit in the signed type the result will always fit in the signed
                        // type.
                        let negated_result = (negated_self - num);
                        if num == 0 {
                            // Just because the result will fit in the signed type does not
                            // mean that the negated_result will fit in the signed type.
                            // For 2's complement integers (which rust uses) this happens only
                            // if the result will be the minimum value of the signed type.
                            // This is only possible of num was zero which leads to a trivial
                            // sum of self.
                            Some(*self)
                        } else {
                            // Negation can be done in the signed type thanks to checks.
                            Some(0 - negated_result as Self)
                        }
                    }
                }
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
    use super::*;

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
        for i in i8::MIN..=i8::MAX {
            for j in i8::MIN..=i8::MAX {
                let difference = i as i16 - j as i16;
                let usize_difference: Option<usize> = difference.try_into().ok();
                assert_eq!(
                    usize_difference,
                    i.difference(&j),
                    "i: {}, j: {}, difference: {}",
                    i,
                    j,
                    difference
                );
            }
        }
    }

    #[test]
    fn i128_difference_overflow() {
        if usize::BITS < 128 {
            assert_eq!(None, i128::MAX.difference(&0));
            assert_eq!(None, 0i128.difference(&i128::MIN));
        }
    }

    #[test]
    fn u8_add_usize() {
        assert_eq!(3, 1u8.add_usize(2).unwrap());
        assert_eq!(255, 0u8.add_usize(255).unwrap());
        assert_eq!(None, 1u8.add_usize(255));
        assert_eq!(None, 0u8.add_usize(256));
    }

    #[test]
    fn i8_add_usize() {
        for i in i8::MIN..=i8::MAX {
            for num in 0..=256 {
                let sum = i as i16 + num as i16;
                let i8_sum: Option<i8> = sum.try_into().ok();
                assert_eq!(
                    i8_sum,
                    i.add_usize(num),
                    "i: {}, num: {}, sum (as i16): {}",
                    i,
                    num,
                    sum
                );
            }
        }
    }

    // test type that uses ToIndex and TryFromIndex to implement Key
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct LessThan100(u8);

    impl LessThan100 {
        fn new(val: u8) -> Option<Self> {
            if val < 100 {
                Some(Self(val))
            } else {
                None
            }
        }
    }

    impl ToIndex for LessThan100 {
        type Index = u8;
        fn to_index(&self) -> Self::Index {
            self.0
        }
    }

    impl TryFromIndex for LessThan100 {
        fn try_from_index(val: Self::Index) -> Option<Self> {
            Self::new(val)
        }
    }

    #[test]
    fn bounded_u8_add_one() {
        assert_eq!(
            LessThan100::new(1).unwrap(),
            LessThan100::new(0).unwrap().add_one().unwrap()
        );
        assert_eq!(
            LessThan100::new(99).unwrap(),
            LessThan100::new(98).unwrap().add_one().unwrap()
        );
        assert_eq!(None, LessThan100::new(99).unwrap().add_one());
    }

    #[test]
    fn bounded_u8_difference() {
        assert_eq!(
            10,
            LessThan100::new(30)
                .unwrap()
                .difference(&LessThan100::new(20).unwrap())
                .unwrap()
        );
        assert_eq!(
            None,
            LessThan100::new(10)
                .unwrap()
                .difference(&LessThan100::new(11).unwrap())
        );
    }

    #[test]
    fn bounded_u8_add_usize() {
        assert_eq!(
            LessThan100::new(30).unwrap(),
            LessThan100::new(10).unwrap().add_usize(20).unwrap()
        );
        assert_eq!(None, LessThan100::new(1).unwrap().add_usize(255));
        assert_eq!(None, LessThan100::new(1).unwrap().add_usize(99));
    }
}
