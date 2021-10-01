//! Contains [`ContiguousMap`]: a map that stores values with adjacent keys contiguously
//! so they may be accessed as a slice.
#![warn(missing_docs)]

use std::{borrow::Borrow, cmp::Ordering, collections::BTreeMap, ops::{Bound, RangeBounds}};

mod macros;

mod iter;
pub use iter::{IntoIter, Iter, IterMut, IterSlice, IterSliceMut, IterVec};
mod key;
pub use key::Key;
mod range_bounds;
pub use range_bounds::InclusiveStartRangeBounds;

/// An index into a ContiguousMap.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Index<K: Key> {
    key: K,
    offset: usize,
}

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

    /// Gets the number of entries in this map.
    ///
    /// This is the total number of values in the map, not the number of contiguous regions.
    /// For the number of contiguous regions use [`ContiguousMap::num_contiguous_regions()`]
    pub fn len(&self) -> usize {
        self.map.values().map(|vec| vec.len()).sum()
    }

    /// Gets if this map is empty.
    pub fn is_empty(&self) -> bool {
        // as no empty entries are allowed in the map we do not
        // need to check for a map full of empty vectors
        self.map.is_empty()
    }

    /// Gets the number of contiguous regions in this map.
    ///
    /// This is the number of (key, slice) tuples that will be iterated over
    /// by iterators returned by [`ContiguousMap::iter_slice()`] and
    /// [`ContiguousMap::iter_slice_mut()`].
    ///
    /// This is also th number of (key, vector) tuples that will be iterated over
    /// by iterators returned by [`ContiguousMap::iter_vec()`].
    pub fn num_contiguous_regions(&self) -> usize {
        self.map.len()
    }

    /// Gets an index for the first entry in this map.
    /// Returns None if this map is empty.
    fn first(&self) -> Option<Index<K>> {
        let entry = self.map.iter().next()?;
        Some(Index {
            key: entry.0.clone(),
            offset: 0,
        })
    }

    /// Gets an index for the last entry in this map.
    /// Returns None if this map is empty.
    fn last(&self) -> Option<Index<K>> {
        let entry = self.map.iter().next_back()?;
        Some(Index {
            key: entry.0.clone(),
            offset: entry.1.len() - 1,
        })
    }

    /// Gets an index for a key.  Returns None if the key is not in this map.
    fn find(&self, key: &K) -> Option<Index<K>> {
        let entry = self.map.range(..=key).next_back()?;
        let offset = key.difference(entry.0)?;
        if offset >= entry.1.len() {
            None
        } else {
            Some(Index {
                key: entry.0.clone(),
                offset,
            })
        }
    }

    /// Gets an index for the largest key that is at most the given key.
    /// Returns None if all keys in the map are greater than the given key.
    fn find_at_most(&self, key: &K) -> Option<Index<K>> {
        let entry = self.map.range(..=key).next_back()?;
        let offset = key.difference(entry.0)?;
        Some(Index {
            key: entry.0.clone(),
            offset: std::cmp::min(offset, entry.1.len() - 1),
        })
    }

    /// Gets an index for the largest key that is less than the given key.
    /// Returns None if all keys in the map are greater than or equal to the given key.
    fn find_less(&self, key: &K) -> Option<Index<K>> {
        self.find_at_most(&key.sub_one()?)
    }

    /// Gets an index for the smallest key that is at least the given key.
    /// Returns None if all keys in the map are smaller than the given key.
    fn find_at_least(&self, key: &K) -> Option<Index<K>> {
        if let Some(index) = self.find(key) {
            Some(index)
        } else {
            let entry = self.map.range(key..).next()?;
            Some(Index {
                key: entry.0.clone(),
                offset: 0,
            })
        }
    }

    /// Gets an index for the smallest key that is greater than the given key.
    /// Returns None if all keys in the map are smaller than or equal to the given key.
    fn find_more(&self, key: &K) -> Option<Index<K>> {
        self.find_at_least(&key.add_one()?)
    }

    /// Finds the inclusive bounds of a range within this map.
    /// Returns None if there are no elements within the range in this map.
    fn find_range<R: RangeBounds<K>>(&self, range: R) -> Option<(Index<K>, Index<K>)> {
        let start = match range.start_bound() {
            Bound::Excluded(start) => self.find_more(start)?,
            Bound::Included(start) => self.find_at_least(start)?,
            Bound::Unbounded => self.first()?,
        };
        let end = match range.end_bound() {
            Bound::Excluded(end) => self.find_less(end)?,
            Bound::Included(end) => self.find_at_most(end)?,
            Bound::Unbounded => self.last()?,
        };
        if start <= end {
            Some((start, end))
        } else {
            None
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
        let (start, end) = match self.find_range(range) {
            Some(range) => range,
            None => return,
        };

        if start.key == end.key {
            // entire removal is in a single region
            let vec = self.map.get_mut(&start.key).unwrap();
            match (start.offset == 0, end.offset == (vec.len() - 1)) {
                (true, true) => {
                    // remove entire entry
                    self.map.remove(&start.key).unwrap();
                }
                (false, true) => {
                    // pop off elements from the back of the vec
                    vec.truncate(start.offset);
                }
                (true, false) => {
                    // extract the vec
                    let mut vec = self.map.remove(&start.key).unwrap();
                    // remove the front of the vector that was marked for clearing
                    let num_to_remove = end.offset + 1;
                    vec.rotate_left(num_to_remove);
                    vec.truncate(vec.len() - num_to_remove);
                    // add the tail back into the map right after the region of clearing
                    self.map.insert(
                        end.key.add_usize(end.offset).unwrap().add_one().unwrap(),
                        vec,
                    );
                }
                (false, false) => {
                    // split the tail that will be retained off of vec
                    let tail = vec.split_off(end.offset + 1);
                    // remove the interior elements marked for clearing
                    vec.truncate(start.offset);
                    // insert the tail back into the map right after the region of clearing
                    self.map.insert(
                        end.key.add_usize(end.offset).unwrap().add_one().unwrap(),
                        tail,
                    );
                }
            }
        } else {
            // removal spans multiple regions

            // handle the start region
            if start.offset == 0 {
                // remove entire entry
                self.map.remove(&start.key).unwrap();
            } else {
                // remove the tail of the entry
                let vec = self.map.get_mut(&start.key).unwrap();
                vec.truncate(start.offset);
            }

            // remove any regions between start and end
            while let Some((key, _)) = self
                .map
                .range((Bound::Excluded(&start.key), Bound::Excluded(&end.key)))
                .next()
            {
                let key = key.clone();
                self.map.remove(&key).unwrap();
            }

            // handle the end region
            let vec = self.map.get(&end.key).unwrap();
            if vec.len() - 1 == end.offset {
                // remove entire region
                self.map.remove(&end.key).unwrap();
            } else {
                // extract the vec
                let mut vec = self.map.remove(&end.key).unwrap();
                // remove the front of the vector that was marked for clearing
                let num_to_remove = end.offset + 1;
                vec.rotate_left(num_to_remove);
                vec.truncate(vec.len() - num_to_remove);
                // add the tail back into the map right after the region of clearing
                self.map.insert(
                    end.key.add_usize(end.offset).unwrap().add_one().unwrap(),
                    vec,
                );
            }
        }
    }

    /// Removes all entries starting at the provided key for the next len adjacent keys.
    pub fn clear_with_len<KB: Borrow<K>>(&mut self, start_key: KB, len: usize) {
        let start = start_key.borrow();
        let end = start.add_usize(len);
        match end {
            None => self.clear_range(start..),
            Some(end) => self.clear_range(start..&end),
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
