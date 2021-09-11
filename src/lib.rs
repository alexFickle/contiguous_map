//! Contains [`ContiguousMap`]: a map that stores values with adjacent keys contiguously
//! so they may be accessed as a slice.
#![warn(missing_docs)]

use std::collections::BTreeMap;

mod key;
pub use key::Key;

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
    pub fn get<'a>(&'a self, key: &K) -> Option<&'a V> {
        let entry = self.map.range(..=key.clone()).next_back()?;
        let index = key.difference(entry.0)?;
        entry.1.get(index)
    }

    /// Returns a mutable reference to a key's value, if it exists.
    pub fn get_mut<'a>(&'a mut self, key: &K) -> Option<&'a mut V> {
        let entry = self.map.range_mut(..=key.clone()).next_back()?;
        let index = key.difference(entry.0)?;
        entry.1.get_mut(index)
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
        assert_eq!(Some(&13), map.get(&3));
        assert_eq!(Some(&14), map.get(&4));
        assert_eq!(Some(&19), map.get(&9));
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
        assert_eq!(Some(&mut 13), map.get_mut(&3));
        assert_eq!(Some(&mut 14), map.get_mut(&4));
        assert_eq!(Some(&mut 19), map.get_mut(&9));
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
}
