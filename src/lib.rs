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
mod test;
