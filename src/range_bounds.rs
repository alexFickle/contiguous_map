use crate::Key;
use std::ops::Bound;

/// Trait similar to [`std::ops::RangeBounds`] that requires an inclusive start to the range.
///
/// This means this trait is only usable for ranges like
/// [`1..`](std::ops::RangeFrom),
/// [`1..5`](std::ops::Range),
/// and [`1..=5`](std::ops::RangeInclusive).
///
/// Ranges like
/// [`..`](std::ops::RangeFull),
/// [`..5`](std::ops::RangeTo), and
/// [`..=5`](std::ops::RangeToInclusive) are not supported.
///
/// This trait is used for looking up slices in a [`ContiguousMap`](crate::ContiguousMap).
/// The semantics of the non-supported ranges are odd and not yet (and maybe never) implemented.
/// For example, with [`..`](std::ops::RangeFull) you'd expect a slice that contains all of the
/// values in the map, however only elements with adjacent keys can be in the same slice.
pub trait InclusiveStartRangeBounds<K: Key> {
    /// The inclusive starting bound of this range.
    fn start_bound(&self) -> &K;

    /// The end bound of this range.
    fn end_bound(&self) -> Bound<&K>;
}

impl<K: Key> InclusiveStartRangeBounds<K> for std::ops::Range<K> {
    fn start_bound(&self) -> &K {
        &self.start
    }

    fn end_bound(&self) -> Bound<&K> {
        Bound::Excluded(&self.end)
    }
}

impl<K: Key> InclusiveStartRangeBounds<K> for std::ops::Range<&K> {
    fn start_bound(&self) -> &K {
        self.start
    }

    fn end_bound(&self) -> Bound<&K> {
        Bound::Excluded(self.end)
    }
}

impl<K: Key> InclusiveStartRangeBounds<K> for std::ops::RangeFrom<K> {
    fn start_bound(&self) -> &K {
        &self.start
    }

    fn end_bound(&self) -> Bound<&K> {
        Bound::Unbounded
    }
}

impl<K: Key> InclusiveStartRangeBounds<K> for std::ops::RangeFrom<&K> {
    fn start_bound(&self) -> &K {
        self.start
    }

    fn end_bound(&self) -> Bound<&K> {
        Bound::Unbounded
    }
}

impl<K: Key> InclusiveStartRangeBounds<K> for std::ops::RangeInclusive<K> {
    fn start_bound(&self) -> &K {
        self.start()
    }

    fn end_bound(&self) -> Bound<&K> {
        Bound::Included(self.end())
    }
}

impl<K: Key> InclusiveStartRangeBounds<K> for std::ops::RangeInclusive<&K> {
    fn start_bound(&self) -> &K {
        self.start()
    }

    fn end_bound(&self) -> Bound<&K> {
        Bound::Included(self.end())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn range() {
        let range = 2u8..7;
        assert_eq!(&2, range.start_bound());
        assert_eq!(Bound::Excluded(&7), range.end_bound());
    }

    #[test]
    fn range_ref() {
        let range = &2u8..&7;
        assert_eq!(&2, range.start_bound());
        assert_eq!(Bound::Excluded(&7), range.end_bound());
    }

    #[test]
    fn range_from() {
        let range = 2u8..;
        assert_eq!(&2, range.start_bound());
        assert_eq!(Bound::Unbounded, range.end_bound());
    }

    #[test]
    fn range_from_ref() {
        let range = &2u8..;
        assert_eq!(&2, range.start_bound());
        assert_eq!(Bound::Unbounded, range.end_bound());
    }

    #[test]
    fn range_inclusive() {
        let range = 2u8..=7;
        assert_eq!(&2, range.start_bound());
        assert_eq!(Bound::Included(&7), range.end_bound());
    }

    #[test]
    fn range_inclusive_ref() {
        let range = &2u8..=&7;
        assert_eq!(&2, range.start_bound());
        assert_eq!(Bound::Included(&7), range.end_bound());
    }
}
