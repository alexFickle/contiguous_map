//! Contains [`ContiguousMap`]: a map that stores values with adjacent keys contiguously
//! so they may be accessed as a slice.
#![warn(missing_docs)]

use std::{borrow::Borrow, collections::BTreeMap};

mod key;
pub use key::Key;
mod range_bounds;
pub use range_bounds::InclusiveStartRangeBounds;

/// An ordered, associative container like [`std::collections::BTreeMap`].
/// Additionally stores values with adjacent keys contiguously so they may
/// be accessed as a slice.
pub struct ContiguousMap<K: key::Key, V> {
    map: BTreeMap<K, Vec<V>>,
}

impl<K: Key, V> ContiguousMap<K, V> {
    /// Makes a new, empty ContiguousMap.
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    /// Inserts a value into a map with a given key.
    /// Returns the old value for this key if one existed.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // attempt to find an already existing insertion point
        if let Some(insertion_entry) = self.map.range_mut(..=key.clone()).next_back() {
            if let Some(index) = key.difference(&insertion_entry.0) {
                if index < insertion_entry.1.len() {
                    // inserting into the insertion_entry
                    let mut value = value;
                    std::mem::swap(&mut value, &mut insertion_entry.1[index]);
                    return Some(value);
                } else if index == insertion_entry.1.len() {
                    // appending to the insertion_entry
                    insertion_entry.1.push(value);
                    // might need to merge with the next entry in the map
                    if let Some(one_after_key) = key.add_one() {
                        let extend_to_key = insertion_entry.0.clone();
                        if let Some(append_values) = self.map.remove(&one_after_key) {
                            self.map
                                .get_mut(&extend_to_key)
                                .unwrap()
                                .extend(append_values);
                        }
                    }
                    return None;
                }
            }
        }

        // No insertion point already exists in the map.
        // Have to make one, but may have to extend it with already existing values in the map.
        let mut vec = vec![value];
        if let Some(one_after_key) = key.add_one() {
            if let Some(append_values) = self.map.remove(&one_after_key) {
                vec.extend(append_values);
            }
        }
        self.map.insert(key, vec);
        None
    }

    /// Returns a reference to a key's value, if it exists.
    pub fn get<'a, KB: Borrow<K>>(&'a self, key: KB) -> Option<&'a V> {
        let key = key.borrow();
        let entry = self.map.range(..=key).next_back()?;
        let index = key.difference(entry.0)?;
        entry.1.get(index)
    }

    /// Returns a mutable reference to a key's value, if it exists.
    pub fn get_mut<'a, KB: Borrow<K>>(&'a mut self, key: KB) -> Option<&'a mut V> {
        let key = key.borrow();
        let entry = self.map.range_mut(..=key.clone()).next_back()?;
        let index = key.difference(entry.0)?;
        entry.1.get_mut(index)
    }

    /// Gets a slice from this map using a range of keys.
    pub fn get_slice<R: InclusiveStartRangeBounds<K>>(&self, range: R) -> Option<&[V]> {
        let entry = self.map.range(..=range.start_bound()).next_back()?;
        let offset = range.start_bound().difference(entry.0)?;
        let slice = if offset < entry.1.len() {
            &entry.1[offset..]
        } else {
            return None;
        };
        use std::ops::Bound;
        let length = match range.end_bound() {
            Bound::Unbounded => slice.len(),
            Bound::Excluded(end) => end.difference(range.start_bound())?,
            Bound::Included(inclusive_end) => inclusive_end
                .difference(range.start_bound())?
                .checked_add(1)?,
        };
        slice.chunks_exact(length).next()
    }

    /// Gets a slice from this map using a key and a length.
    pub fn get_slice_with_len<KB: Borrow<K>>(&self, key: KB, len: usize) -> Option<&[V]> {
        self.get_slice(key.borrow()..)
            .map(|slice| slice.chunks_exact(len).next())
            .flatten()
    }

    /// Gets a mutable slice from this map using a range of keys.
    pub fn get_slice_mut<R: InclusiveStartRangeBounds<K>>(&mut self, range: R) -> Option<&mut [V]> {
        let entry = self.map.range_mut(..=range.start_bound()).next_back()?;
        let offset = range.start_bound().difference(entry.0)?;
        let slice = if offset < entry.1.len() {
            &mut entry.1[offset..]
        } else {
            return None;
        };
        use std::ops::Bound;
        let length = match range.end_bound() {
            Bound::Unbounded => slice.len(),
            Bound::Excluded(end) => end.difference(range.start_bound())?,
            Bound::Included(inclusive_end) => inclusive_end
                .difference(range.start_bound())?
                .checked_add(1)?,
        };
        slice.chunks_exact_mut(length).next()
    }

    /// Gets a mutable slice from this map using a key and a length.
    pub fn get_slice_with_len_mut<KB: Borrow<K>>(
        &mut self,
        key: KB,
        len: usize,
    ) -> Option<&mut [V]> {
        self.get_slice_mut(key.borrow()..)
            .map(|slice| slice.chunks_exact_mut(len).next())
            .flatten()
    }
}

impl<K: Key, V: Clone> ContiguousMap<K, V> {
    /// Inserts values into the map from a slice starting at a given key.
    pub fn insert_slice(&mut self, start_key: K, values: &[V]) {
        let mut key = start_key;
        for value in values.iter().cloned() {
            self.insert(key.clone(), value);
            key = match key.add_one() {
                Some(k) => k,
                None => return,
            };
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Helper function that asserts that a ContiguousMap contains exactly
    /// the given entries.
    /// The entries must be given in order sorted by their keys.
    #[track_caller]
    fn assert_map_same<const NUM_ENTRIES: usize>(
        map: &super::ContiguousMap<usize, i32>,
        entries: [(usize, Vec<i32>); NUM_ENTRIES],
    ) {
        assert!(
            NUM_ENTRIES == map.map.len(),
            "Expected {} entries in the internal map, not {}.\nmap:{:?}",
            NUM_ENTRIES,
            map.map.len(),
            map.map
        );
        for (index, ((expected_start, expected_vec), (start, vec))) in
            entries.iter().zip(map.map.iter()).enumerate()
        {
            assert!(
                expected_start == start,
                "Expected the entry at index {} of the internal map to start at a key value of {}, not {}.\nmap: {:?}",
                index, expected_start, start, map.map
            );
            assert!(
                expected_vec == vec,
                "Expected the vector starting at the key value of {} to be {:?}, not {:?}.\nmap: {:?}",
                start, expected_vec, vec, map.map
            );
        }
    }

    #[test]
    fn new() {
        assert_map_same(&ContiguousMap::new(), []);
    }

    #[test]
    fn insert_into_empty() {
        let mut map = ContiguousMap::new();
        assert!(map.insert(1, 2).is_none());
        assert_map_same(&map, [(1, vec![2])]);
    }

    #[test]
    fn insert_overwrite() {
        let mut map = ContiguousMap::new();
        assert!(map.insert(1, 2).is_none());
        assert_eq!(Some(2), map.insert(1, 3));
        assert_map_same(&map, [(1, vec![3])]);
    }

    #[test]
    fn insert_after() {
        let mut map = ContiguousMap::new();
        assert!(map.insert(0, 10).is_none());
        assert!(map.insert(2, 12).is_none());
        assert_map_same(&map, [(0, vec![10]), (2, vec![12])]);
    }

    #[test]
    fn insert_one_after() {
        let mut map = ContiguousMap::new();
        assert!(map.insert(1, 10).is_none());
        assert!(map.insert(2, 12).is_none());
        assert_map_same(&map, [(1, vec![10, 12])]);
    }

    #[test]
    fn insert_before() {
        let mut map = ContiguousMap::new();
        assert!(map.insert(2, 12).is_none());
        assert!(map.insert(0, 10).is_none());
        assert_map_same(&map, [(0, vec![10]), (2, vec![12])]);
    }

    #[test]
    fn insert_one_before() {
        let mut map = ContiguousMap::new();
        assert!(map.insert(2, 12).is_none());
        assert!(map.insert(1, 10).is_none());
        assert_map_same(&map, [(1, vec![10, 12])]);
    }

    #[test]
    fn insert_into_gap() {
        let mut map = ContiguousMap::new();
        assert!(map.insert(0, 10).is_none());
        assert!(map.insert(2, 12).is_none());
        assert!(map.insert(1, 11).is_none());
        assert_map_same(&map, [(0, vec![10, 11, 12])]);
    }

    #[test]
    fn get() {
        let mut map = ContiguousMap::<usize, usize>::new();
        map.insert(2, 12);
        map.insert(3, 13);
        map.insert(4, 14);
        map.insert(9, 19);
        assert_eq!(Some(&12), map.get(&2));
        assert_eq!(Some(&13), map.get(3));
        assert_eq!(Some(&14), map.get(&4));
        assert_eq!(Some(&19), map.get(9));
    }

    #[test]
    fn get_missing() {
        let mut map = ContiguousMap::<usize, usize>::new();
        assert_eq!(None, map.get(&2));
        map.insert(1, 11);
        map.insert(3, 13);
        assert_eq!(None, map.get(&0));
        assert_eq!(None, map.get(&2));
        assert_eq!(None, map.get(&4));
    }

    #[test]
    fn get_mut() {
        let mut map = ContiguousMap::<usize, usize>::new();
        map.insert(2, 12);
        map.insert(3, 13);
        map.insert(4, 14);
        map.insert(9, 19);
        assert_eq!(Some(&mut 12), map.get_mut(&2));
        assert_eq!(Some(&mut 13), map.get_mut(3));
        assert_eq!(Some(&mut 14), map.get_mut(&4));
        assert_eq!(Some(&mut 19), map.get_mut(9));
    }

    #[test]
    fn get_mut_missing() {
        let mut map = ContiguousMap::<usize, usize>::new();
        assert_eq!(None, map.get_mut(&2));
        map.insert(1, 11);
        map.insert(3, 13);
        assert_eq!(None, map.get_mut(&0));
        assert_eq!(None, map.get_mut(&2));
        assert_eq!(None, map.get_mut(&4));
    }

    #[test]
    fn insert_slice() {
        let mut map = ContiguousMap::new();
        map.insert_slice(3, &[1, 2, 3]);
        assert_map_same(&map, [(3, vec![1, 2, 3])]);
    }

    #[test]
    fn get_slice_with_range() {
        let map = {
            let mut map = ContiguousMap::<usize, usize>::new();
            map.insert_slice(3, &[1, 2, 3]);
            map
        };

        assert_eq!(None, map.get_slice(2..4));
        assert_eq!(None, map.get_slice(2..6));
        assert_eq!(None, map.get_slice(3..7));
        assert_eq!([1, 2, 3], map.get_slice(3..6).unwrap());
        assert_eq!([2, 3], map.get_slice(4..6).unwrap());
        assert_eq!([3], map.get_slice(5..6).unwrap());
        assert_eq!([1, 2], map.get_slice(3..5).unwrap());
    }

    #[test]
    fn get_slice_with_range_inclusive() {
        let map = {
            let mut map = ContiguousMap::<usize, usize>::new();
            map.insert_slice(3, &[1, 2, 3]);
            map
        };

        assert_eq!(None, map.get_slice(2..=3));
        assert_eq!(None, map.get_slice(2..=5));
        assert_eq!(None, map.get_slice(3..=6));
        assert_eq!([1, 2, 3], map.get_slice(3..=5).unwrap());
        assert_eq!([2, 3], map.get_slice(4..=5).unwrap());
        assert_eq!([3], map.get_slice(5..=5).unwrap());
        assert_eq!([1, 2], map.get_slice(3..=4).unwrap());
    }

    #[test]
    fn get_slice_with_range_from() {
        let map = {
            let mut map = ContiguousMap::<usize, usize>::new();
            map.insert_slice(3, &[1, 2, 3]);
            map
        };

        assert_eq!(None, map.get_slice(2..));
        assert_eq!([1, 2, 3], map.get_slice(3..).unwrap());
        assert_eq!([2, 3], map.get_slice(4..).unwrap());
        assert_eq!([3], map.get_slice(5..).unwrap());
        assert_eq!(None, map.get_slice(6..));
    }

    #[test]
    fn get_slice_with_len() {
        let map = {
            let mut map = ContiguousMap::<usize, usize>::new();
            map.insert_slice(3, &[1, 2, 3]);
            map
        };

        assert_eq!(None, map.get_slice_with_len(2, 2));
        assert_eq!(None, map.get_slice_with_len(2, 4));
        assert_eq!(None, map.get_slice_with_len(3, 4));
        assert_eq!([1, 2, 3], map.get_slice_with_len(3, 3).unwrap());
        assert_eq!([2, 3], map.get_slice_with_len(4, 2).unwrap());
        assert_eq!([3], map.get_slice_with_len(5, 1).unwrap());
        assert_eq!([1, 2], map.get_slice_with_len(3, 2).unwrap());
    }

    #[test]
    fn get_slice_mut_with_range() {
        let mut map = ContiguousMap::<usize, usize>::new();
        map.insert_slice(3, &[1, 2, 3]);

        assert_eq!(None, map.get_slice_mut(2..4));
        assert_eq!(None, map.get_slice_mut(2..6));
        assert_eq!(None, map.get_slice_mut(3..7));
        assert_eq!([1, 2, 3], map.get_slice_mut(3..6).unwrap());
        assert_eq!([2, 3], map.get_slice_mut(4..6).unwrap());
        assert_eq!([3], map.get_slice_mut(5..6).unwrap());
        assert_eq!([1, 2], map.get_slice_mut(3..5).unwrap());
    }

    #[test]
    fn get_slice_mut_with_range_inclusive() {
        let mut map = ContiguousMap::<usize, usize>::new();
        map.insert_slice(3, &[1, 2, 3]);

        assert_eq!(None, map.get_slice_mut(2..=3));
        assert_eq!(None, map.get_slice_mut(2..=5));
        assert_eq!(None, map.get_slice_mut(3..=6));
        assert_eq!([1, 2, 3], map.get_slice_mut(3..=5).unwrap());
        assert_eq!([2, 3], map.get_slice_mut(4..=5).unwrap());
        assert_eq!([3], map.get_slice_mut(5..=5).unwrap());
        assert_eq!([1, 2], map.get_slice_mut(3..=4).unwrap());
    }

    #[test]
    fn get_slice_mut_with_range_from() {
        let mut map = ContiguousMap::<usize, usize>::new();
        map.insert_slice(3, &[1, 2, 3]);

        assert_eq!(None, map.get_slice_mut(2..));
        assert_eq!([1, 2, 3], map.get_slice_mut(3..).unwrap());
        assert_eq!([2, 3], map.get_slice_mut(4..).unwrap());
        assert_eq!([3], map.get_slice_mut(5..).unwrap());
        assert_eq!(None, map.get_slice_mut(6..));
    }

    #[test]
    fn get_slice_with_len_mut() {
        let mut map = ContiguousMap::<usize, usize>::new();
        map.insert_slice(3, &[1, 2, 3]);

        assert_eq!(None, map.get_slice_with_len_mut(2, 2));
        assert_eq!(None, map.get_slice_with_len_mut(2, 4));
        assert_eq!(None, map.get_slice_with_len_mut(3, 4));
        assert_eq!([1, 2, 3], map.get_slice_with_len_mut(3, 3).unwrap());
        assert_eq!([2, 3], map.get_slice_with_len_mut(4, 2).unwrap());
        assert_eq!([3], map.get_slice_with_len_mut(5, 1).unwrap());
        assert_eq!([1, 2], map.get_slice_with_len_mut(3, 2).unwrap());
    }
}
