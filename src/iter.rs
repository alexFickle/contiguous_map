use super::{ContiguousMap, Key};
use std::{collections::btree_map, iter::FusedIterator};

/// Implementation function for [`IntoIter`], [`Iter`], and [`IterMut`]'s next() function.
///
/// This function attempts to extract a (Key, Value) pair from three given sources.
///
/// # Arguments
/// * `front_entry` — Produces (Key, Value) pairs that have been extracted from the front of `map_iter` but
///  have not yet been yielded from the iterator using this function.
/// * `map_iter` — An iterator that yields something convertible to what is stored in `front_entry`.
///  This is iterated over to populate `front_entry` whenever it is empty.
/// * `back_entry` — Produces (Key, Value) pairs that have been extracted from the back of `map_iter` but
///  have not yet been yielded from the iterator using this function.
///  Once `map_iter` is exhausted this is consumed to iterate over any potentially remaining values.
/// * `extract` — Function that is used to convert the values yielded by `map_iter` into what is stored
///  in `front_entry` and `back_entry`.
fn next_impl<K, V, ValIter, MapIter, ExtractFn, ExtractInput>(
    front_entry: &mut Option<(K, ValIter)>,
    map_iter: &mut MapIter,
    back_entry: &mut Option<(K, ValIter)>,
    extract: ExtractFn,
) -> Option<(K, V)>
where
    K: Key,
    ValIter: Iterator<Item = V> + FusedIterator + ExactSizeIterator,
    MapIter: Iterator<Item = ExtractInput> + FusedIterator,
    ExtractFn: Fn(ExtractInput) -> (K, ValIter),
{
    loop {
        // attempt to consume a (K, V) from front_entry
        if let Some((key, iter)) = front_entry {
            if let Some(value) = iter.next() {
                let item = (key.clone(), value);
                if iter.len() != 0 {
                    *key = key.add_one().unwrap();
                } else {
                    *front_entry = None
                }
                return Some(item);
            }
        }

        // attempt to refill front_entry
        *front_entry = map_iter.next().map(&extract).or_else(|| back_entry.take());

        // test if all iterators are now exhausted
        front_entry.as_ref()?;
    }
}

/// Implementation function for [`IntoIter`], [`Iter`], and [`IterMut`]'s next_back() function.
///
/// This function attempts to extract a (Key, Value) pair from three given sources.
///
/// # Arguments
/// * `front_entry` — Produces (Key, Value) pairs that have been extracted from the front of `map_iter` but
///  have not yet been yielded from the iterator using this function.
///  Once `map_iter` is exhausted this is consumed to iterate over any potentially remaining values.
/// * `map_iter` — An iterator that yields something convertible to what is stored in `back_entry`.
///  This is iterated over to populate `back_entry` whenever it is empty.
/// * `back_entry` — Produces (Key, Value) pairs that have been extracted from the back of `map_iter` but
///  have not yet been yielded from the iterator using this function.
/// * `extract` — Function that is used to convert the values yielded by `map_iter` into what is stored
///  in `front_entry` and `back_entry`.
fn next_back_impl<K, V, ValIter, MapIter, ExtractFn, ExtractInput>(
    front_entry: &mut Option<(K, ValIter)>,
    map_iter: &mut MapIter,
    back_entry: &mut Option<(K, ValIter)>,
    extract: ExtractFn,
) -> Option<(K, V)>
where
    K: Key,
    ValIter: Iterator<Item = V> + DoubleEndedIterator + FusedIterator + ExactSizeIterator,
    MapIter: Iterator<Item = ExtractInput> + DoubleEndedIterator + FusedIterator,
    ExtractFn: Fn(ExtractInput) -> (K, ValIter),
{
    loop {
        // attempt to consume a (K, V) from back_entry
        if let Some((key, iter)) = back_entry {
            if let Some(value) = iter.next_back() {
                let key = key.add_usize(iter.len()).unwrap();
                return Some((key, value));
            } else {
                *back_entry = None;
            }
        }

        // attempt to refill back_entry
        *back_entry = map_iter
            .next_back()
            .map(&extract)
            .or_else(|| front_entry.take());

        // test if all iterators are now exhausted
        back_entry.as_ref()?;
    }
}

/// An owning iterator over all `(Key, Value)` entries
/// in a [`ContiguousMap`] in ascending key order.
pub struct IntoIter<K: Key, V> {
    front_entry: Option<(K, std::vec::IntoIter<V>)>,
    map_iter: btree_map::IntoIter<K, Vec<V>>,
    back_entry: Option<(K, std::vec::IntoIter<V>)>,
}

impl<K: Key, V> IntoIterator for ContiguousMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        IntoIter {
            front_entry: None,
            map_iter: self.map.into_iter(),
            back_entry: None,
        }
    }
}

impl<K: Key, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        next_impl(
            &mut self.front_entry,
            &mut self.map_iter,
            &mut self.back_entry,
            |(k, v)| (k, v.into_iter()),
        )
    }
}

impl<K: Key, V> DoubleEndedIterator for IntoIter<K, V> {
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        next_back_impl(
            &mut self.front_entry,
            &mut self.map_iter,
            &mut self.back_entry,
            |(k, v)| (k, v.into_iter()),
        )
    }
}

impl<K: Key, V> FusedIterator for IntoIter<K, V> {}

/// An iterator over all `(Key, &Value)` entries
/// in a [`ContiguousMap`] in ascending key order.
///
/// See [`ContiguousMap::iter()`].
pub struct Iter<'a, K: Key, V> {
    front_entry: Option<(K, std::slice::Iter<'a, V>)>,
    map_iter: btree_map::Iter<'a, K, Vec<V>>,
    back_entry: Option<(K, std::slice::Iter<'a, V>)>,
}

impl<'a, K: Key, V> IntoIterator for &'a ContiguousMap<K, V> {
    type Item = (K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            front_entry: None,
            map_iter: self.map.iter(),
            back_entry: None,
        }
    }
}

impl<'a, K: Key, V> Iterator for Iter<'a, K, V> {
    type Item = (K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        next_impl(
            &mut self.front_entry,
            &mut self.map_iter,
            &mut self.back_entry,
            |(k, v)| (k.clone(), v.iter()),
        )
    }
}

impl<'a, K: Key, V> DoubleEndedIterator for Iter<'a, K, V> {
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        next_back_impl(
            &mut self.front_entry,
            &mut self.map_iter,
            &mut self.back_entry,
            |(k, v)| (k.clone(), v.iter()),
        )
    }
}

impl<'a, K: Key, V> FusedIterator for Iter<'a, K, V> {}

/// A mutable iterator over all `(Key, &mut Value)` entries
/// in a [`ContiguousMap`] in ascending key order.
///
/// See [`ContiguousMap::iter_mut()`].
pub struct IterMut<'a, K: Key, V> {
    front_entry: Option<(K, std::slice::IterMut<'a, V>)>,
    map_iter: btree_map::IterMut<'a, K, Vec<V>>,
    back_entry: Option<(K, std::slice::IterMut<'a, V>)>,
}

impl<'a, K: Key, V> IntoIterator for &'a mut ContiguousMap<K, V> {
    type Item = (K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            front_entry: None,
            map_iter: self.map.iter_mut(),
            back_entry: None,
        }
    }
}

impl<'a, K: Key, V> Iterator for IterMut<'a, K, V> {
    type Item = (K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        next_impl(
            &mut self.front_entry,
            &mut self.map_iter,
            &mut self.back_entry,
            |(k, v)| (k.clone(), v.iter_mut()),
        )
    }
}

impl<'a, K: Key, V> DoubleEndedIterator for IterMut<'a, K, V> {
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        next_back_impl(
            &mut self.front_entry,
            &mut self.map_iter,
            &mut self.back_entry,
            |(k, v)| (k.clone(), v.iter_mut()),
        )
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

impl<K: Key, V> DoubleEndedIterator for IterVec<K, V> {
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        self.inner.next_back()
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

impl<'a, K: Key, V> DoubleEndedIterator for IterSlice<'a, K, V> {
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        self.inner.next_back().map(|(k, v)| (k, &v[..]))
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

impl<'a, K: Key, V> DoubleEndedIterator for IterSliceMut<'a, K, V> {
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        self.inner.next_back().map(|(k, v)| (k, &mut v[..]))
    }
}

impl<'a, K: Key, V> FusedIterator for IterSliceMut<'a, K, V> {}
