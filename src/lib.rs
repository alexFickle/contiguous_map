//! Contains [`ContiguousMap`]: a map that stores values with adjacent keys contiguously
//! so they may be accessed as a slice.
#![warn(missing_docs)]

use std::{
    borrow::Borrow,
    cmp::Ordering,
    collections::BTreeMap,
    ops::{Bound, RangeBounds},
};

mod iter;
pub use iter::{IntoIter, Iter, IterMut, IterSlice, IterSliceMut, IterVec};
mod key;
pub use key::Key;
mod range_bounds;
pub use range_bounds::InclusiveStartRangeBounds;

/// An ordered, associative container like [`std::collections::BTreeMap`].
/// Additionally stores values with adjacent keys contiguously so they may
/// be accessed as a slice.
pub struct ContiguousMap<K: Key, V> {
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
        if let Some(insertion_entry) = self.map.range_mut(..=&key).next_back() {
            if let Some(index) = key.difference(insertion_entry.0) {
                match index.cmp(&insertion_entry.1.len()) {
                    Ordering::Less => {
                        // overwriting a value in insertion_entry
                        let mut value = value;
                        std::mem::swap(&mut value, &mut insertion_entry.1[index]);
                        return Some(value);
                    }
                    Ordering::Equal => {
                        // appending to insertion_entry
                        insertion_entry.1.push(value);
                        // might need to merge with the next entry in the map
                        if let Some(one_after_key) = key.add_one() {
                            let extend_to_key = insertion_entry.0.clone();
                            if let Some(append_values) = self.map.remove(&one_after_key) {
                                self.map
                                    .get_mut(&extend_to_key)
                                    .expect("lookup with key cloned from entry in map")
                                    .extend(append_values);
                            }
                        }
                        return None;
                    }
                    Ordering::Greater => {
                        // insertion_entry can not contain our key due to gap
                    }
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

    /// Inserts values into the map from a slice starting at a given key.
    pub fn insert_slice(&mut self, start_key: K, values: &[V])
    where
        V: Clone,
    {
        let mut key = start_key;
        for value in values.iter().cloned() {
            self.insert(key.clone(), value);
            key = match key.add_one() {
                Some(k) => k,
                None => return,
            };
        }
    }

    /// Removes a key's value in this map, returning it if it existed.
    pub fn remove<KB: Borrow<K>>(&mut self, key: KB) -> Option<V> {
        let key = key.borrow();
        let entry = self.map.range_mut(..=key).next_back()?;
        let index = key.difference(entry.0)?;
        // (entry.1.len() - 1) always valid due to ContiguousMap vectors never being empty
        match index.cmp(&(entry.1.len() - 1)) {
            Ordering::Greater => {
                // index out of bounds
                None
            }
            Ordering::Equal => {
                // just need to pop off the last item in the entry's vec
                let value = entry
                    .1
                    .pop()
                    .expect("internal vectors in ContiguousMap are never empty");
                if entry.1.is_empty() {
                    self.map
                        .remove(key)
                        .expect("removing now empty entry from map that we know exists");
                }
                Some(value)
            }
            Ordering::Less => {
                // need to remove a non-last item in the entry's vec
                if index == 0 {
                    // remove the front element and create a new entry for the next adjacent key
                    let mut entry = self
                        .map
                        .remove_entry(key)
                        .expect("removing entry that is known to exist");
                    entry.0 = entry.0.add_one().expect(
                        "key has a value for the next adjacent key, this next key must exist",
                    );
                    let value = entry.1.remove(0);
                    self.map.insert(entry.0, entry.1);
                    Some(value)
                } else {
                    // split off the tail of the vector, creating a new entry for it
                    let tail = entry.1.split_off(index + 1);
                    let value = entry.1.pop().expect(
                        "removing last item from vector whose size is known to be at least 2",
                    );
                    let tail_key = key.add_one().expect(
                        "key has a value for the next adjacent key, this next key must exist",
                    );
                    self.map.insert(tail_key, tail);
                    Some(value)
                }
            }
        }
    }

    /// Removes all entries from this map.
    pub fn clear(&mut self) {
        self.map.clear()
    }

    /// Removes all entries within a range of keys.
    pub fn clear_range<R: RangeBounds<K>>(&mut self, range: R) {
        // decode the bounds, delegating to other functions if unbounded
        let (start, start_included) = match range.start_bound() {
            Bound::Unbounded => return self.clear_up_to_bound(range.end_bound()),
            Bound::Included(start) => (start, true),
            Bound::Excluded(start) => (start, false),
        };
        let (end, end_included) = match range.end_bound() {
            Bound::Unbounded => return self.clear_from_bound(start, start_included),
            Bound::Included(end) => (end, true),
            Bound::Excluded(end) => (end, false),
        };
        // handle bounded ranges
        let mut key = if start_included {
            start.clone()
        } else {
            match start.add_one() {
                Some(key) => key,
                None => return,
            }
        };
        while &key < end {
            self.remove(&key);
            key = key.add_one().unwrap();
        }
        if end_included && &key == end {
            self.remove(&key);
        }
    }

    /// Internal function that clears the range (Unbounded, upper_bound).
    fn clear_up_to_bound(&mut self, upper_bound: Bound<&K>) {
        // decode the end bound, if it is unbounded just clear the map
        let (end, end_included) = match upper_bound {
            Bound::Unbounded => return self.clear(),
            Bound::Included(end) => (end, true),
            Bound::Excluded(end) => (end, false),
        };
        loop {
            // the first entry in the map will be tested for clearing
            let entry = match self.map.iter().next() {
                Some(entry) => entry,
                None => return,
            };
            // if the entire entry is outside of the range of clearing we are done
            let out_of_range = match (entry.0.cmp(end), end_included) {
                (Ordering::Less, _) => false,
                (Ordering::Equal, true) => false,
                (Ordering::Equal, false) => true,
                (Ordering::Greater, _) => true,
            };
            if out_of_range {
                return;
            }
            // if the entire entry is in bounds of the range of clearing remove it
            let first_key_in_entry = entry.0.clone();
            let last_key_in_entry = entry.0.add_usize(entry.1.len() - 1).unwrap();
            let remove_entry = match last_key_in_entry.cmp(end) {
                Ordering::Less => true,
                Ordering::Equal => end_included,
                Ordering::Greater => false,
            };
            if remove_entry {
                self.map.remove(&first_key_in_entry).unwrap();
            } else {
                // need to clear elements from the front of the entry, then we are done
                let mut vec = self.map.remove(&first_key_in_entry).unwrap();
                let num_to_clear =
                    end.difference(&first_key_in_entry).unwrap() + end_included as usize;
                vec.rotate_left(num_to_clear);
                vec.truncate(vec.len() - num_to_clear);
                let key = if end_included {
                    end.add_one().unwrap()
                } else {
                    end.clone()
                };
                self.map.insert(key, vec);
                return;
            }
        }
    }

    /// Internal function that clears the range (lower_bound, Unbounded).
    ///
    /// Unlike clear_up_to_bound() the bound is already decoded into a
    /// starting key and if the start is inclusive.  This means that
    /// an unbounded lower_bound is not possible.  This difference is
    /// due to how these functions are used in clear_range().
    fn clear_from_bound(&mut self, start: &K, start_included: bool) {
        loop {
            // the last entry in the map will be tested for clearing
            let entry = match self.map.iter().next_back() {
                Some(entry) => entry,
                None => return,
            };
            // if the entire entry is outside of the range of removal we are done
            let last_key_in_entry = entry.0.add_usize(entry.1.len() - 1).unwrap();
            let out_of_range = match (start.cmp(&last_key_in_entry), start_included) {
                (Ordering::Less, _) => false,
                (Ordering::Equal, true) => false,
                (Ordering::Equal, false) => true,
                (Ordering::Greater, _) => true,
            };
            if out_of_range {
                return;
            }
            // if the entire entry is in bounds of the range of removal remove it
            let first_key_in_entry = entry.0.clone();
            let remove_entry = match first_key_in_entry.cmp(start) {
                Ordering::Less => false,
                Ordering::Equal => start_included,
                Ordering::Greater => true,
            };
            if remove_entry {
                self.map.remove(&first_key_in_entry).unwrap();
            } else {
                // need to clear elements from the back of the entry, then we are done
                let vec = self.map.get_mut(&first_key_in_entry).unwrap();
                let num_to_clear =
                    last_key_in_entry.difference(start).unwrap() + start_included as usize;
                vec.truncate(vec.len() - num_to_clear);
                return;
            }
        }
    }

    /// Removes all entries starting at the provided key for the next len adjacent keys.
    pub fn clear_with_len<KB: Borrow<K>>(&mut self, start_key: KB, len: usize) {
        let mut key = start_key.borrow().clone();
        for _ in 0..len {
            self.remove(&key);
            key = match key.add_one() {
                Some(key) => key,
                None => break,
            };
        }
    }

    /// Returns a reference to a key's value, if it exists.
    pub fn get<KB: Borrow<K>>(&self, key: KB) -> Option<&V> {
        let key = key.borrow();
        let entry = self.map.range(..=key).next_back()?;
        let index = key.difference(entry.0)?;
        entry.1.get(index)
    }

    /// Returns a mutable reference to a key's value, if it exists.
    pub fn get_mut<KB: Borrow<K>>(&mut self, key: KB) -> Option<&mut V> {
        let key = key.borrow();
        let entry = self.map.range_mut(..=key).next_back()?;
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

    /// Iteration over all keys and values in this map in ascending key order.
    ///
    /// Unlike [`std::collections::BTreeMap`] the tuples yielded by the iterator
    /// contains a key directly instead of a reference to a key.
    /// This is due to how contiguous regions are stored internally.
    pub fn iter(&self) -> Iter<K, V> {
        self.into_iter()
    }

    /// Mutable iteration over all keys and values in this map in ascending key order.
    ///
    /// Unlike [`std::collections::BTreeMap`] the tuples yielded by the iterator
    /// contains a key directly instead of a reference to a key.
    /// This is due to how contiguous regions are stored internally.
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        self.into_iter()
    }

    /// Owning iteration over all keys and values in this map grouped up
    /// in contiguous regions in ascending key order.
    ///
    /// The iterator yields tuples containing a key and vector of values.
    /// The key of each value is its index in the vector plus
    /// the key in the tuple.
    ///
    /// The iterator will never yield a tuple with an empty vector.
    pub fn iter_vec(self) -> IterVec<K, V> {
        IterVec::new(self)
    }

    /// Iteration over all keys and values in this map grouped up
    /// in contiguous regions in ascending key order.
    ///
    /// The iterator yields tuples containing a key and slice of values.
    /// The key of each value is its index in the slice plus
    /// the key in the tuple.
    ///
    /// The iterator will never yield a tuple with an empty slice.
    pub fn iter_slice(&self) -> IterSlice<K, V> {
        IterSlice::new(self)
    }

    /// Mutable iteration over all keys and values in this map grouped up
    /// in contiguous regions in ascending key order.
    ///
    /// The iterator yields tuples containing a key and slice of values.
    /// The key of each value is its index in the slice plus
    /// the key in the tuple.
    ///
    /// The iterator will never yield a tuple with an empty slice.
    pub fn iter_slice_mut(&mut self) -> IterSliceMut<K, V> {
        IterSliceMut::new(self)
    }
}

impl<K: Key, V> Default for ContiguousMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test;
