/// Helper function that asserts that a ContiguousMap is upholding
/// all required internal invariants.
///
/// # Invariants
/// 1. The map does not contain any empty vectors.
/// 2. The map contains at most one value for every key.
///   For example, a map with an entry of 0 => 1, 2, 3
///   and 2 => 4 would contain two values for the key of 2
///   and therefore be invalid.
/// 3. Any entires that may be merged must be merged.
///   For example, a map containing the entires 0 => 1, 2
///   and 2 => 3 would be able to represent the same values
///   with a single merged entry of 0 => 1, 2, 3.
///   Therefore the map is invalid.
/// 4. There are no values with a key outside of the range
///   of valid keys.
///   For example, a map with a key type of usize and an entry
///   of usize::MAX => 1, 2 would have a value with a key of
///   usize::MAX+1.  This is outside the range of the key type
///   and therefore the map is invalid.
/// 5. The internal length equal to the total number of values
///   in the map.
#[track_caller]
fn assert_map_valid<V: std::fmt::Debug>(map: &crate::ContiguousMap<usize, V>) {
    // check invariant 1
    for (_, vector) in map.map.iter() {
        assert!(
            vector.len() != 0,
            "Internal ContiguousMap invariant violation: Contains an empty vector.\nmap{:?}",
            map.map,
        );
    }
    // check invariant 4
    for (key, vector) in map.map.iter() {
        assert!(
            key.checked_add(vector.len() - 1).is_some(),
            "Internal ContiguousMap invariant violation: Entry at key {} overflows the key type.\nmap{:?}",
            key,
            map.map,
        );
    }
    // check invariant 2
    use itertools::Itertools;
    for ((key, vec), (next_key, _)) in map.map.iter().tuple_windows() {
        let key_of_last_in_first = key + (vec.len() - 1);
        assert!(
            key_of_last_in_first < *next_key,
            "Internal ContiguousMap invariant violation: Multiple values for key of {}.\nmap:{:?}",
            next_key,
            map.map,
        );
    }
    // check invariant 3
    for ((key, vec), (next_key, _)) in map.map.iter().tuple_windows() {
        let key_of_last_in_first = key + (vec.len() - 1);
        assert!(
            key_of_last_in_first != next_key - 1,
            "Internal ContiguousMap invariant violation: Map contains mergeable entires with keys {} and {}.\nmap:{:?}",
            key,
            next_key,
            map.map,
        );
    }
    // check invariant 5
    let len: usize = map.map.values().map(|vec| vec.len()).sum();
    assert!(
        len == map.length,
        "Internal ContiguousMap invariant violation: Map's stored length of {} does not match the number of values of {}.\nmap:{:?}",
        map.length,
        len,
        map.map,
    );
}

/// Helper function that asserts that a ContiguousMap contains exactly
/// the given entries.
/// The entries must be given in order sorted by their keys.
/// Also asserts that the map is valid.
#[track_caller]
fn assert_map_same<const NUM_ENTRIES: usize>(
    map: &crate::ContiguousMap<usize, i32>,
    entries: [(usize, Vec<i32>); NUM_ENTRIES],
) {
    assert_map_valid(map);
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
            start,
            expected_vec,
            vec,
            map.map
        );
    }
}

/// Helper function that asserts that a fused double ended iterator is empty.
#[track_caller]
fn assert_de_iter_empty<I: std::iter::FusedIterator + DoubleEndedIterator>(mut iter: I) {
    for _ in 0..10 {
        assert!(
            iter.next().is_none(),
            "Expected the iterator to be exhausted, but iter.next() gave a value."
        );
        assert!(
            iter.next_back().is_none(),
            "Expected the iterator to be exhausted, but iter.next_back() gave a value."
        );
    }
}

mod clear;
mod clear_range;
mod clear_with_len;
mod debug;
mod default;
mod find;
mod find_at_least;
mod find_at_most;
mod find_less;
mod find_more;
mod find_range;
mod first;
mod get;
mod get_mut;
mod get_slice;
mod get_slice_mut;
mod get_slice_with_len;
mod get_slice_with_len_mut;
mod insert;
mod insert_slice;
mod into_iter;
mod is_empty;
mod iter;
mod iter_mut;
mod iter_slice;
mod iter_slice_mut;
mod iter_vec;
mod last;
mod len;
mod new;
mod num_contiguous_regions;
mod range;
mod range_mut;
mod remove;
