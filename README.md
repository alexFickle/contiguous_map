Contains `ContiguousMap`: a map that stores values with adjacent keys contiguously so they may be accessed as a slice.

# Docs

This library is not currently released.
For now its documentation can be viewed at:
<https://alexfickle.github.io/contiguous_map-docs/contiguous_map>.
This documentation is periodically updated.
For the most up to date docs clone and run `cargo doc --open`.

# Examples


## Slices and Ranges
The main goal of this library is to be able to interact with contiguous
sequences of keys and values in a map.
Contiguous sequences of values are interacted with as slices.
Contiguous sequences of keys are interacted with as both ranges and as a
start key and length.
```rust
use contiguous_map::ContiguousMap;

// Create a new map.
let mut map = ContiguousMap::new();

// Insert a slice starting at a key of 1.
map.insert_slice(1, &[1, 2, 3]);

// Can use Range, RangeInclusive, or RangeFrom with get_slice().
assert_eq!(Some(&[1, 2, 3][..]), map.get_slice(1..4));
assert_eq!(Some(&[1, 2, 3][..]), map.get_slice(1..=3));
assert_eq!(Some(&[1, 2, 3][..]), map.get_slice(1..));

// Can also use a start and length with get_slice_with_len().
assert_eq!(Some(&[1, 2, 3][..]), map.get_slice_with_len(1, 3));

// get_slice_mut() and get_slice_with_len_mut() are also available
// to get mutable references from a map.

// Inspect the total number of contiguous regions in a map.
assert_eq!(1, map.num_contiguous_regions());

// Remove all entries with a key at or above 3.
// Can use any range type with clear_range().
map.clear_range(3..);
// Remove 10 keys starting at the key of 1.
// Equivalent to map.clear_range(1..11).
map.clear_with_len(1, 10);

// Can also iterate over all of the contiguous regions in a
// map using iter_vec(), iter_slice(), and iter_slice_mut().
```

## BTreeMap Methods
This library also implements many of `std::collections::BTreeMap`'s methods.
```rust
use contiguous_map::ContiguousMap;

// Create a map.
let mut map = ContiguousMap::new();

// Insert a single element.
map.insert(1, 2);

// Get a single element.
assert_eq!(Some(&2), map.get(1));

// Overwrite an element.
let old_value = map.insert(1, 10);
assert_eq!(Some(2), old_value);

// Remove an element.
let removed_value = map.remove(1);
assert_eq!(Some(10), removed_value);

// Inspect a map's length.
assert_eq!(0, map.len());
assert!(map.is_empty());

// Clear the map.
map.clear();

// In addition many of BTreeMap's iteration methods are implemented,
// like: into_iter(), iter(), iter_mut(), range(), and range_mut().
```

## cmap!()
A macro is provided to assist with the creation of `ContiguousMap`s.
The syntax of the macro is a key followed by `=>` then a comma
separated list of values.
For a map with multiple slices add a semicolon between the list of
values and the next key.
Both the keys and values may be arbitrary expressions.
```rust
use contiguous_map::cmap;

// Single slice in a map.
let map = cmap!(1 => 1, 2, 3);
assert_eq!(Some(&[1, 2, 3][..]), map.get_slice(1..));

// Two separate entries in a map.
let map = cmap!(1 => 2; 5 => 7);
assert_eq!(Some(&2), map.get(1));
assert_eq!(Some(&7), map.get(5));

// Two separate slices in a map.
let map = cmap!(
    1 => 10, 12, 15;
    7 => 20, 21, 24;
);
assert_eq!(Some(&[10, 12, 15][..]), map.get_slice(1..));
assert_eq!(Some(&[20, 21, 24][..]), map.get_slice(7..));
```

# Key Types

All keys types used in a `ContiguousMap` need to have both the concepts of
ordering and adjacency.
This is implemented using the custom `Key` trait in this library.
All primitive integer types and char have a provided implementation.
A user may implement the `Key` trait for their own types if they desire
to use them as a key in a `ContiguousMap`.
