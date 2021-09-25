use super::{ContiguousMap, Key};
use std::{
    collections::btree_map,
    iter::{FusedIterator, Peekable},
};

/// An owning iterator over all `(Key, Value)` entries
/// in a [`ContiguousMap`] in ascending key order.
pub struct IntoIter<K: Key, V> {
    entry_iter: Option<(K, Peekable<std::vec::IntoIter<V>>)>,
    map_iter: btree_map::IntoIter<K, Vec<V>>,
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

impl<K: Key, V> FusedIterator for IntoIter<K, V> {}

/// An iterator over all `(Key, &Value)` entries
/// in a [`ContiguousMap`] in ascending key order.
///
/// See [`ContiguousMap::iter()`].
pub struct Iter<'a, K: Key, V> {
    entry_iter: Option<(K, Peekable<std::slice::Iter<'a, V>>)>,
    map_iter: btree_map::Iter<'a, K, Vec<V>>,
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

impl<'a, K: Key, V> FusedIterator for Iter<'a, K, V> {}

/// A mutable iterator over all `(Key, &mut Value)` entries
/// in a [`ContiguousMap`] in ascending key order.
///
/// See [`ContiguousMap::iter_mut()`].
pub struct IterMut<'a, K: Key, V> {
    entry_iter: Option<(K, Peekable<std::slice::IterMut<'a, V>>)>,
    map_iter: btree_map::IterMut<'a, K, Vec<V>>,
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

impl<'a, K: Key, V> FusedIterator for IterMut<'a, K, V> {}

/// An owning iterator over all the contiguous `(Key, Vec<Value>)` entries
/// in a [`ContiguousMap`] in ascending key order.
///
/// See [`ContiguousMap::iter_vec()`].
pub struct IterVec<K: Key, V> {
    inner: btree_map::IntoIter<K, Vec<V>>,
}

impl<K: Key, V> IterVec<K, V> {
    pub(crate) fn new(map: ContiguousMap<K, V>) -> Self {
        Self {
            inner: map.map.into_iter(),
        }
    }
}

impl<K: Key, V> Iterator for IterVec<K, V> {
    type Item = (K, Vec<V>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<K: Key, V> FusedIterator for IterVec<K, V> {}

/// An iterator over all the contiguous `(&Key, &[Value])` entries
/// in a [`ContiguousMap`] in ascending key order.
///
/// See [`ContiguousMap::iter_slice()`].
pub struct IterSlice<'a, K: Key, V> {
    inner: btree_map::Iter<'a, K, Vec<V>>,
}

impl<'a, K: Key, V> IterSlice<'a, K, V> {
    pub(crate) fn new(map: &'a ContiguousMap<K, V>) -> Self {
        Self {
            inner: map.map.iter(),
        }
    }
}

impl<'a, K: Key, V> Iterator for IterSlice<'a, K, V> {
    type Item = (&'a K, &'a [V]);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| (k, &v[..]))
    }
}

impl<'a, K: Key, V> FusedIterator for IterSlice<'a, K, V> {}

/// A mutable iterator over all the contiguous `(&Key, &mut [Value])` entries
/// in a [`ContiguousMap`] in ascending key order.
///
/// See [`ContiguousMap::iter_slice_mut()`].
pub struct IterSliceMut<'a, K: Key, V> {
    inner: btree_map::IterMut<'a, K, Vec<V>>,
}

impl<'a, K: Key, V> IterSliceMut<'a, K, V> {
    pub(crate) fn new(map: &'a mut ContiguousMap<K, V>) -> Self {
        Self {
            inner: map.map.iter_mut(),
        }
    }
}

impl<'a, K: Key, V> Iterator for IterSliceMut<'a, K, V> {
    type Item = (&'a K, &'a mut [V]);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| (k, &mut v[..]))
    }
}

impl<'a, K: Key, V> FusedIterator for IterSliceMut<'a, K, V> {}
