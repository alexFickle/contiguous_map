//! Contains [`ContiguousMap`]: a map that stores values with adjacent keys contiguously
//! so they may be accessed as a slice.
#![warn(missing_docs)]

use std::{borrow::Borrow, collections::btree_map, collections::BTreeMap, iter::Peekable};

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
                use std::cmp::Ordering;
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
        IterVec {
            inner: self.map.into_iter(),
        }
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
        IterSlice {
            inner: self.map.iter(),
        }
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
        IterSliceMut {
            inner: self.map.iter_mut(),
        }
    }
}

impl<K: Key, V> Default for ContiguousMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

/// An owning iterator over all `(Key, Value)` entries
/// in a [`ContiguousMap`] in ascending key order.
pub struct IntoIter<K: Key, V> {
    entry_iter: Option<(K, Peekable<std::vec::IntoIter<V>>)>,
    map_iter: btree_map::IntoIter<K, Vec<V>>,
}

impl<K: Key, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // attempt to consume a (K, V) from the entry_iter
            if let Some((key, iter)) = &mut self.entry_iter {
                if let Some(value) = iter.next() {
                    let item = (key.clone(), value);
                    if iter.peek().is_some() {
                        *key = key.add_one().unwrap();
                    } else {
                        self.entry_iter = None
                    }
                    return Some(item);
                }
            }

            // attempt to refill entry_iter
            self.entry_iter = self
                .map_iter
                .next()
                .map(|(k, v)| (k, v.into_iter().peekable()));

            // ensure we did not exhaust the map iterator
            self.entry_iter.as_ref()?;
        }
    }
}

impl<K: Key, V> IntoIterator for ContiguousMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        IntoIter {
            entry_iter: None,
            map_iter: self.map.into_iter(),
        }
    }
}

/// An iterator over all `(Key, &Value)` entries
/// in a [`ContiguousMap`] in ascending key order.
///
/// See [`ContiguousMap::iter()`].
pub struct Iter<'a, K: Key, V> {
    entry_iter: Option<(K, Peekable<std::slice::Iter<'a, V>>)>,
    map_iter: btree_map::Iter<'a, K, Vec<V>>,
}

impl<'a, K: Key, V> Iterator for Iter<'a, K, V> {
    type Item = (K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // attempt to consume a (K, V) from the entry_iter
            if let Some((key, iter)) = &mut self.entry_iter {
                if let Some(value) = iter.next() {
                    let item = (key.clone(), value);
                    if iter.peek().is_some() {
                        *key = key.add_one().unwrap();
                    } else {
                        self.entry_iter = None
                    }
                    return Some(item);
                }
            }

            // attempt to refill entry_iter
            self.entry_iter = self
                .map_iter
                .next()
                .map(|(k, v)| (k.clone(), v[..].iter().peekable()));

            // ensure we did not exhaust the map iterator
            self.entry_iter.as_ref()?;
        }
    }
}

impl<'a, K: Key, V> IntoIterator for &'a ContiguousMap<K, V> {
    type Item = (K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            entry_iter: None,
            map_iter: self.map.iter(),
        }
    }
}

/// A mutable iterator over all `(Key, &mut Value)` entries
/// in a [`ContiguousMap`] in ascending key order.
///
/// See [`ContiguousMap::iter_mut()`].
pub struct IterMut<'a, K: Key, V> {
    entry_iter: Option<(K, Peekable<std::slice::IterMut<'a, V>>)>,
    map_iter: btree_map::IterMut<'a, K, Vec<V>>,
}

impl<'a, K: Key, V> Iterator for IterMut<'a, K, V> {
    type Item = (K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // attempt to consume a (K, V) from the entry_iter
            if let Some((key, iter)) = &mut self.entry_iter {
                if let Some(value) = iter.next() {
                    let item = (key.clone(), value);
                    if iter.peek().is_some() {
                        *key = key.add_one().unwrap();
                    } else {
                        self.entry_iter = None
                    }
                    return Some(item);
                }
            }

            // attempt to refill entry_iter
            self.entry_iter = self
                .map_iter
                .next()
                .map(|(k, v)| (k.clone(), v[..].iter_mut().peekable()));

            // ensure we did not exhaust the map iterator
            self.entry_iter.as_ref()?;
        }
    }
}

impl<'a, K: Key, V> IntoIterator for &'a mut ContiguousMap<K, V> {
    type Item = (K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            entry_iter: None,
            map_iter: self.map.iter_mut(),
        }
    }
}

/// An owning iterator over all the contiguous `(Key, Vec<Value>)` entries
/// in a [`ContiguousMap`] in ascending key order.
///
/// See [`ContiguousMap::iter_vec()`].
pub struct IterVec<K: Key, V> {
    inner: btree_map::IntoIter<K, Vec<V>>,
}

impl<K: Key, V> Iterator for IterVec<K, V> {
    type Item = (K, Vec<V>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// An iterator over all the contiguous `(&Key, &[Value])` entries
/// in a [`ContiguousMap`] in ascending key order.
///
/// See [`ContiguousMap::iter_slice()`].
pub struct IterSlice<'a, K: Key, V> {
    inner: btree_map::Iter<'a, K, Vec<V>>,
}

impl<'a, K: Key, V> Iterator for IterSlice<'a, K, V> {
    type Item = (&'a K, &'a [V]);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| (k, &v[..]))
    }
}

/// A mutable iterator over all the contiguous `(&Key, &mut [Value])` entries
/// in a [`ContiguousMap`] in ascending key order.
///
/// See [`ContiguousMap::iter_slice_mut()`].
pub struct IterSliceMut<'a, K: Key, V> {
    inner: btree_map::IterMut<'a, K, Vec<V>>,
}

impl<'a, K: Key, V> Iterator for IterSliceMut<'a, K, V> {
    type Item = (&'a K, &'a mut [V]);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| (k, &mut v[..]))
    }
}

#[cfg(test)]
mod test;
