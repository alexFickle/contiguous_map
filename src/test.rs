/// Helper function that asserts that a ContiguousMap contains exactly
/// the given entries.
/// The entries must be given in order sorted by their keys.
#[track_caller]
fn assert_map_same<const NUM_ENTRIES: usize>(
    map: &crate::ContiguousMap<usize, i32>,
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
