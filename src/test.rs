use std::{iter::FusedIterator, vec};

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
            start,
            expected_vec,
            vec,
            map.map
        );
    }
}

#[test]
fn new() {
    assert_map_same(&ContiguousMap::new(), []);
}

#[test]
fn default() {
    assert_map_same(&Default::default(), []);
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
    let map = cmap!(
        2 => 12, 13, 14;
        9 => 19;
    );
    assert_eq!(Some(&12), map.get(&2));
    assert_eq!(Some(&13), map.get(3));
    assert_eq!(Some(&14), map.get(&4));
    assert_eq!(Some(&19), map.get(9));
}

#[test]
fn get_missing() {
    let map = ContiguousMap::<usize, usize>::new();
    assert_eq!(None, map.get(&2));

    let map = cmap!(1 => 11; 3 => 13);
    assert_eq!(None, map.get(&0));
    assert_eq!(None, map.get(&2));
    assert_eq!(None, map.get(&4));
}

#[test]
fn get_mut() {
    let mut map = cmap!(
        2 => 12, 13, 14;
        9 => 19;
    );
    assert_eq!(Some(&mut 12), map.get_mut(&2));
    assert_eq!(Some(&mut 13), map.get_mut(3));
    assert_eq!(Some(&mut 14), map.get_mut(&4));
    assert_eq!(Some(&mut 19), map.get_mut(9));
}

#[test]
fn get_mut_missing() {
    let mut map = ContiguousMap::<usize, usize>::new();
    assert_eq!(None, map.get_mut(&2));
    let mut map = cmap!(1 => 11; 3 => 13);
    assert_eq!(None, map.get_mut(&0));
    assert_eq!(None, map.get_mut(&2));
    assert_eq!(None, map.get_mut(&4));
}

#[test]
fn insert_slice() {
    let mut map = ContiguousMap::new();
    map.insert_slice(3, &[1, 2, 3]);
    assert_map_same(&map, [(3, vec![1, 2, 3])]);
}

#[test]
fn get_slice_with_range() {
    let map = cmap!(3 => 1, 2, 3);
    assert_eq!(None, map.get_slice(2..4));
    assert_eq!(None, map.get_slice(2..6));
    assert_eq!(None, map.get_slice(3..7));
    assert_eq!([1, 2, 3], map.get_slice(3..6).unwrap());
    assert_eq!([2, 3], map.get_slice(4..6).unwrap());
    assert_eq!([3], map.get_slice(5..6).unwrap());
    assert_eq!([1, 2], map.get_slice(3..5).unwrap());
}

#[test]
fn get_slice_with_range_inclusive() {
    let map = cmap!(3 => 1, 2, 3);
    assert_eq!(None, map.get_slice(2..=3));
    assert_eq!(None, map.get_slice(2..=5));
    assert_eq!(None, map.get_slice(3..=6));
    assert_eq!([1, 2, 3], map.get_slice(3..=5).unwrap());
    assert_eq!([2, 3], map.get_slice(4..=5).unwrap());
    assert_eq!([3], map.get_slice(5..=5).unwrap());
    assert_eq!([1, 2], map.get_slice(3..=4).unwrap());
}

#[test]
fn get_slice_with_range_from() {
    let map = cmap!(3 => 1, 2, 3);
    assert_eq!(None, map.get_slice(2..));
    assert_eq!([1, 2, 3], map.get_slice(3..).unwrap());
    assert_eq!([2, 3], map.get_slice(4..).unwrap());
    assert_eq!([3], map.get_slice(5..).unwrap());
    assert_eq!(None, map.get_slice(6..));
}

#[test]
fn get_slice_with_len() {
    let map = cmap!(3 => 1, 2, 3);
    assert_eq!(None, map.get_slice_with_len(2, 2));
    assert_eq!(None, map.get_slice_with_len(2, 4));
    assert_eq!(None, map.get_slice_with_len(3, 4));
    assert_eq!([1, 2, 3], map.get_slice_with_len(3, 3).unwrap());
    assert_eq!([2, 3], map.get_slice_with_len(4, 2).unwrap());
    assert_eq!([3], map.get_slice_with_len(5, 1).unwrap());
    assert_eq!([1, 2], map.get_slice_with_len(3, 2).unwrap());
}

#[test]
fn get_slice_mut_with_range() {
    let mut map = cmap!(3 => 1, 2, 3);
    assert_eq!(None, map.get_slice_mut(2..4));
    assert_eq!(None, map.get_slice_mut(2..6));
    assert_eq!(None, map.get_slice_mut(3..7));
    assert_eq!([1, 2, 3], map.get_slice_mut(3..6).unwrap());
    assert_eq!([2, 3], map.get_slice_mut(4..6).unwrap());
    assert_eq!([3], map.get_slice_mut(5..6).unwrap());
    assert_eq!([1, 2], map.get_slice_mut(3..5).unwrap());
}

#[test]
fn get_slice_mut_with_range_inclusive() {
    let mut map = cmap!(3 => 1, 2, 3);
    assert_eq!(None, map.get_slice_mut(2..=3));
    assert_eq!(None, map.get_slice_mut(2..=5));
    assert_eq!(None, map.get_slice_mut(3..=6));
    assert_eq!([1, 2, 3], map.get_slice_mut(3..=5).unwrap());
    assert_eq!([2, 3], map.get_slice_mut(4..=5).unwrap());
    assert_eq!([3], map.get_slice_mut(5..=5).unwrap());
    assert_eq!([1, 2], map.get_slice_mut(3..=4).unwrap());
}

#[test]
fn get_slice_mut_with_range_from() {
    let mut map = cmap!(3 => 1, 2, 3);
    assert_eq!(None, map.get_slice_mut(2..));
    assert_eq!([1, 2, 3], map.get_slice_mut(3..).unwrap());
    assert_eq!([2, 3], map.get_slice_mut(4..).unwrap());
    assert_eq!([3], map.get_slice_mut(5..).unwrap());
    assert_eq!(None, map.get_slice_mut(6..));
}

#[test]
fn get_slice_with_len_mut() {
    let mut map = cmap!(3 => 1, 2, 3);
    assert_eq!(None, map.get_slice_with_len_mut(2, 2));
    assert_eq!(None, map.get_slice_with_len_mut(2, 4));
    assert_eq!(None, map.get_slice_with_len_mut(3, 4));
    assert_eq!([1, 2, 3], map.get_slice_with_len_mut(3, 3).unwrap());
    assert_eq!([2, 3], map.get_slice_with_len_mut(4, 2).unwrap());
    assert_eq!([3], map.get_slice_with_len_mut(5, 1).unwrap());
    assert_eq!([1, 2], map.get_slice_with_len_mut(3, 2).unwrap());
}

#[track_caller]
fn assert_iter_exhausted<I: FusedIterator>(mut iter: I) {
    for _ in 0..10 {
        assert!(
            iter.next().is_none(),
            "Expected the iterator to be exhausted."
        );
    }
}

#[track_caller]
fn assert_double_ended_iter_exhausted<I: DoubleEndedIterator + FusedIterator>(mut iter: I) {
    for _ in 0..10 {
        assert!(
            iter.next().is_none(),
            "Expected the iterator to be exhausted, iter.next() gave a value."
        );
        assert!(
            iter.next_back().is_none(),
            "Expected the iterator to be exhausted, iter.next_back() gave a value."
        );
    }
}

#[test]
fn into_iter() {
    let map = cmap!(
        3 => 1, 2, 3;
        10 => 14, 15;
        20 => 21;
    );
    let mut iter = map.into_iter();
    assert_eq!((3, 1), iter.next().unwrap());
    assert_eq!((4, 2), iter.next().unwrap());
    assert_eq!((5, 3), iter.next().unwrap());
    assert_eq!((10, 14), iter.next().unwrap());
    assert_eq!((11, 15), iter.next().unwrap());
    assert_eq!((20, 21), iter.next().unwrap());
    assert_iter_exhausted(iter);
}

#[test]
fn into_iter_reverse() {
    let map = cmap!(
        3 => 1, 2, 3;
        10 => 14, 15;
        20 => 21;
    );
    let mut iter = map.into_iter();
    assert_eq!((20, 21), iter.next_back().unwrap());
    assert_eq!((11, 15), iter.next_back().unwrap());
    assert_eq!((10, 14), iter.next_back().unwrap());
    assert_eq!((5, 3), iter.next_back().unwrap());
    assert_eq!((4, 2), iter.next_back().unwrap());
    assert_eq!((3, 1), iter.next_back().unwrap());
    assert_double_ended_iter_exhausted(iter);
}

#[test]
fn into_iter_double_ended() {
    let map = cmap!(0 => 10, 11, 12, 13);
    let mut iter = map.into_iter();
    assert_eq!((0, 10), iter.next().unwrap());
    assert_eq!((3, 13), iter.next_back().unwrap());
    assert_eq!((1, 11), iter.next().unwrap());
    assert_eq!((2, 12), iter.next_back().unwrap());
    assert_double_ended_iter_exhausted(iter);
}

#[test]
fn iter() {
    let map = cmap!(
        3 => 1, 2, 3;
        10 => 14, 15;
        20 => 21;
    );
    let mut iter = map.iter();
    assert_eq!((3, &1), iter.next().unwrap());
    assert_eq!((4, &2), iter.next().unwrap());
    assert_eq!((5, &3), iter.next().unwrap());
    assert_eq!((10, &14), iter.next().unwrap());
    assert_eq!((11, &15), iter.next().unwrap());
    assert_eq!((20, &21), iter.next().unwrap());
    assert_iter_exhausted(iter);
}

#[test]
fn iter_reverse() {
    let map = cmap!(
        3 => 1, 2, 3;
        10 => 14, 15;
        20 => 21;
    );
    let mut iter = map.iter();
    assert_eq!((20, &21), iter.next_back().unwrap());
    assert_eq!((11, &15), iter.next_back().unwrap());
    assert_eq!((10, &14), iter.next_back().unwrap());
    assert_eq!((5, &3), iter.next_back().unwrap());
    assert_eq!((4, &2), iter.next_back().unwrap());
    assert_eq!((3, &1), iter.next_back().unwrap());
    assert_double_ended_iter_exhausted(iter);
}

#[test]
fn iter_double_ended() {
    let map = cmap!(0 => 10, 11, 12, 13);
    let mut iter = map.iter();
    assert_eq!((0, &10), iter.next().unwrap());
    assert_eq!((3, &13), iter.next_back().unwrap());
    assert_eq!((1, &11), iter.next().unwrap());
    assert_eq!((2, &12), iter.next_back().unwrap());
    assert_double_ended_iter_exhausted(iter);
}

#[test]
fn iter_mut() {
    let mut map = cmap!(
        3 => 1, 2, 3;
        10 => 14, 15;
        20 => 21;
    );
    let mut iter = map.iter_mut();
    assert_eq!((3, &mut 1), iter.next().unwrap());
    assert_eq!((4, &mut 2), iter.next().unwrap());
    assert_eq!((5, &mut 3), iter.next().unwrap());
    assert_eq!((10, &mut 14), iter.next().unwrap());
    assert_eq!((11, &mut 15), iter.next().unwrap());
    assert_eq!((20, &mut 21), iter.next().unwrap());
    assert_iter_exhausted(iter);
}

#[test]
fn iter_mut_reverse() {
    let mut map = cmap!(
        3 => 1, 2, 3;
        10 => 14, 15;
        20 => 21;
    );
    let mut iter = map.iter_mut();
    assert_eq!((20, &mut 21), iter.next_back().unwrap());
    assert_eq!((11, &mut 15), iter.next_back().unwrap());
    assert_eq!((10, &mut 14), iter.next_back().unwrap());
    assert_eq!((5, &mut 3), iter.next_back().unwrap());
    assert_eq!((4, &mut 2), iter.next_back().unwrap());
    assert_eq!((3, &mut 1), iter.next_back().unwrap());
    assert_double_ended_iter_exhausted(iter);
}

#[test]
fn iter_mut_double_ended() {
    let mut map = cmap!(0 => 10, 11, 12, 13);
    let mut iter = map.iter_mut();
    assert_eq!((0, &mut 10), iter.next().unwrap());
    assert_eq!((3, &mut 13), iter.next_back().unwrap());
    assert_eq!((1, &mut 11), iter.next().unwrap());
    assert_eq!((2, &mut 12), iter.next_back().unwrap());
    assert_double_ended_iter_exhausted(iter);
}

#[test]
fn iter_vec() {
    let map = cmap!(
        3 => 1, 2, 3;
        10 => 14, 15;
        20 => 21;
    );
    let mut iter = map.iter_vec();
    assert_eq!((3, vec![1, 2, 3]), iter.next().unwrap());
    assert_eq!((10, vec![14, 15]), iter.next().unwrap());
    assert_eq!((20, vec![21]), iter.next().unwrap());
    assert_iter_exhausted(iter);
}

#[test]
fn iter_vec_double_ended() {
    let map = cmap!(
        3 => 1, 2, 3;
        10 => 14, 15;
        20 => 21;
    );
    let mut iter = map.iter_vec();
    assert_eq!((20, vec![21]), iter.next_back().unwrap());
    assert_eq!((3, vec![1, 2, 3]), iter.next().unwrap());
    assert_eq!((10, vec![14, 15]), iter.next_back().unwrap());
    assert_double_ended_iter_exhausted(iter);
}

#[test]
fn iter_slice() {
    let map = cmap!(
        3 => 1, 2, 3;
        10 => 14, 15;
        20 => 21;
    );
    let mut iter = map.iter_slice();
    assert_eq!((&3, &[1, 2, 3][..]), iter.next().unwrap());
    assert_eq!((&10, &[14, 15][..]), iter.next().unwrap());
    assert_eq!((&20, &[21][..]), iter.next().unwrap());
    assert_iter_exhausted(iter);
}

#[test]
fn iter_slice_double_ended() {
    let map = cmap!(
        3 => 1, 2, 3;
        10 => 14, 15;
        20 => 21;
    );
    let mut iter = map.iter_slice();
    assert_eq!((&20, &[21][..]), iter.next_back().unwrap());
    assert_eq!((&3, &[1, 2, 3][..]), iter.next().unwrap());
    assert_eq!((&10, &[14, 15][..]), iter.next_back().unwrap());
    assert_double_ended_iter_exhausted(iter);
}

#[test]
fn iter_slice_mut() {
    let mut map = cmap!(
        3 => 1, 2, 3;
        10 => 14, 15;
        20 => 21;
    );
    let mut iter = map.iter_slice_mut();
    assert_eq!((&3, &mut [1, 2, 3][..]), iter.next().unwrap());
    assert_eq!((&10, &mut [14, 15][..]), iter.next().unwrap());
    assert_eq!((&20, &mut [21][..]), iter.next().unwrap());
    assert_iter_exhausted(iter);
}

#[test]
fn iter_slice_mut_double_ended() {
    let mut map = cmap!(
        3 => 1, 2, 3;
        10 => 14, 15;
        20 => 21;
    );
    let mut iter = map.iter_slice_mut();
    assert_eq!((&20, &mut [21][..]), iter.next_back().unwrap());
    assert_eq!((&3, &mut [1, 2, 3][..]), iter.next().unwrap());
    assert_eq!((&10, &mut [14, 15][..]), iter.next_back().unwrap());
    assert_iter_exhausted(iter);
}

#[test]
fn remove_missing() {
    let mut map = cmap!(4 => 1, 2, 3);
    for i in 0..4 {
        assert!(map.remove(i).is_none(), "i={}", i);
    }
    for i in 7..10 {
        assert!(map.remove(i).is_none(), "i={}", i);
    }
}

#[test]
fn remove_front_of_slice() {
    let mut map = cmap!(
        0 => 10;
        5 => 15, 16, 17;
        9 => 19;
    );
    assert_eq!(15, map.remove(5).unwrap());
    assert_map_same(&map, [(0, vec![10]), (6, vec![16, 17]), (9, vec![19])]);
}

#[test]
fn remove_middle_of_slice() {
    let mut map = cmap!(
        0 => 10;
        5 => 15, 16, 17;
        9 => 19;
    );
    assert_eq!(16, map.remove(6).unwrap());
    assert_map_same(
        &map,
        [(0, vec![10]), (5, vec![15]), (7, vec![17]), (9, vec![19])],
    );
}

#[test]
fn remove_end_of_slice() {
    let mut map = cmap!(
        0 => 10;
        5 => 15, 16, 17;
        9 => 19;
    );
    assert_eq!(17, map.remove(7).unwrap());
    assert_map_same(&map, [(0, vec![10]), (5, vec![15, 16]), (9, vec![19])]);
}

#[test]
fn remove_isolated() {
    let mut map = cmap!(
        0 => 10;
        5 => 15;
        9 => 19;
    );
    assert_eq!(15, map.remove(5).unwrap());
    assert_map_same(&map, [(0, vec![10]), (9, vec![19])]);
}

#[test]
fn clear() {
    let mut map = cmap!(10 => 20, 21, 22, 23, 24, 25);
    map.clear();
    assert_map_same(&map, []);
}

#[test]
fn clear_range_full() {
    let mut map = cmap!(10 => 20, 21, 22, 23, 24, 25);
    map.clear_range(..);
    assert_map_same(&map, []);
}

#[test]
fn clear_range() {
    let mut map = cmap!(10 => 20, 21, 22, 23, 24, 25);
    map.clear_range(11..14);
    assert_map_same(&map, [(10, vec![20]), (14, vec![24, 25])]);
}

#[test]
fn clear_range_inclusive() {
    let mut map = cmap!(10 => 20, 21, 22, 23, 24, 25);
    map.clear_range(11..=14);
    assert_map_same(&map, [(10, vec![20]), (15, vec![25])]);
}

#[test]
fn clear_range_from() {
    let mut map = cmap!(10 => 20, 21, 22, 23, 24, 25);
    map.clear_range(11..);
    assert_map_same(&map, [(10, vec![20])]);
}

#[test]
fn clear_range_from_entire_map() {
    let mut map = cmap!(
        10 => 20, 21, 22, 23, 24, 25;
        30 => 20, 21, 22, 23, 24, 25;
    );
    map.clear_range(10..);
    assert_map_same(&map, []);
}

#[test]
fn clear_range_from_half_map() {
    let mut map = cmap!(
        10 => 20, 21, 22, 23, 24, 25;
        30 => 20, 21, 22, 23, 24, 25;
    );
    map.clear_range(20..);
    assert_map_same(&map, [(10, vec![20, 21, 22, 23, 24, 25])]);
}

#[test]
fn clear_range_from_too_big() {
    let mut map = cmap!(10 => 1, 2, 3);
    map.clear_range(50..);
    assert_map_same(&map, [(10, vec![1, 2, 3])]);
}

#[test]
fn clear_range_from_at_end() {
    let mut map = cmap!(10 => 1, 2, 3);
    map.clear_range(12..);
    // should have cleared the last element
    assert_map_same(&map, [(10, vec![1, 2])]);
}

#[test]
fn clear_range_from_start_exclusive_at_end() {
    use std::ops::Bound::*;
    let mut map = cmap!(10 => 1, 2, 3);
    map.clear_range((Excluded(12), Unbounded));
    // should have done nothing
    assert_map_same(&map, [(10, vec![1, 2, 3])]);
}

#[test]
fn clear_range_to() {
    let mut map = cmap!(10 => 20, 21, 22, 23, 24, 25);
    map.clear_range(..12);
    assert_map_same(&map, [(12, vec![22, 23, 24, 25])]);
}

#[test]
fn clear_range_to_entire_map() {
    let mut map = cmap!(
        10 => 20, 21, 22, 23, 24, 25;
        30 => 20, 21, 22, 23, 24, 25;
    );
    map.clear_range(..=35);
    assert_map_same(&map, []);
}

#[test]
fn clear_range_to_half_map() {
    let mut map = cmap!(
        10 => 20, 21, 22, 23, 24, 25;
        30 => 20, 21, 22, 23, 24, 25;
    );
    map.clear_range(..=20);
    assert_map_same(&map, [(30, vec![20, 21, 22, 23, 24, 25])]);
}

#[test]
fn clear_range_to_at_start() {
    let mut map = cmap!(10 => 1, 2, 3);
    map.clear_range(..10);
    // should have done nothing
    assert_map_same(&map, [(10, vec![1, 2, 3])]);
}

#[test]
fn clear_range_to_too_small() {
    let mut map = cmap!(10 => 1, 2, 3);
    map.clear_range(..5);
    assert_map_same(&map, [(10, vec![1, 2, 3])]);
}

#[test]
fn clear_range_to_inclusive() {
    let mut map = cmap!(10 => 20, 21, 22, 23, 24, 25);
    map.clear_range(..=12);
    assert_map_same(&map, [(13, vec![23, 24, 25])]);
}

#[test]
fn clear_range_to_inclusive_at_start() {
    let mut map = cmap!(10 => 1, 2, 3);
    map.clear_range(..=10);
    // should have cleared just the first element
    assert_map_same(&map, [(11, vec![2, 3])]);
}

#[test]
fn clear_range_start_exclusive() {
    use std::ops::Bound::*;
    let mut map = cmap!(10 => 20, 21, 22, 23, 24, 25);
    map.clear_range((Excluded(&11), Excluded(&14)));
    assert_map_same(&map, [(10, vec![20, 21]), (14, vec![24, 25])]);
}

#[test]
fn clear_range_start_exclusive_near_overflow() {
    use std::ops::Bound::*;
    let mut map = cmap!(10 => 20, 21, 22, 23, 24, 25);
    // should do nothing, but should not panic
    map.clear_range((Excluded(&usize::MAX), Excluded(&usize::MAX)));
    assert_map_same(&map, [(10, vec![20, 21, 22, 23, 24, 25])]);
}

#[test]
fn clear_range_invalid() {
    use std::ops::Bound::*;
    let mut map = cmap!(10 => 20, 21, 22, 23, 24, 25);
    // should do nothing, but should not panic
    map.clear_range((Included(&13), Excluded(&10)));
    assert_map_same(&map, [(10, vec![20, 21, 22, 23, 24, 25])]);
}

#[test]
fn clear_range_gap() {
    let mut map = cmap!(
        10 => 0, 1, 2;
        30 => 0, 1, 2;
    );
    map.clear_range(20..25);
    assert_map_same(&map, [(10, vec![0, 1, 2]), (30, vec![0, 1, 2])]);
}

#[test]
fn clear_range_overlap() {
    let mut map = cmap!(
        10 => 0, 1, 2;
        20 => 0, 1, 2;
        30 => 0, 1, 2;
    );
    map.clear_range(12..32);
    assert_map_same(&map, [(10, vec![0, 1]), (32, vec![2])]);
}

#[test]
fn clear_with_len() {
    let mut map = cmap!(10 => 20, 21, 22, 23, 24, 25);
    map.clear_with_len(11, 3);
    assert_map_same(&map, [(10, vec![20]), (14, vec![24, 25])]);
}

#[test]
fn clear_with_len_overflow() {
    let mut map = cmap!(usize::MAX - 3 => 1, 2, 3);
    map.clear_with_len(usize::MAX - 1, 100);
    assert_map_same(&map, [(usize::MAX - 3, vec![1, 2])]);
}

#[test]
fn len_empty() {
    let map = ContiguousMap::<i32, i32>::new();
    assert_eq!(0, map.len());
    assert!(map.is_empty());
    assert_eq!(0, map.num_contiguous_regions());
}

#[test]
fn len_one_region() {
    let map = cmap!(0 => 1, 2, 3);
    assert_eq!(3, map.len());
    assert!(!map.is_empty());
    assert_eq!(1, map.num_contiguous_regions());
}

#[test]
fn len_two_regions() {
    let map = cmap!(
        0 => 1, 2, 3;
        10 => 1, 2, 3, 4;
    );
    assert_eq!(7, map.len());
    assert!(!map.is_empty());
    assert_eq!(2, map.num_contiguous_regions());
}

#[test]
fn first() {
    let map = cmap!(
        1 => 11, 12;
        5 => 15, 16, 17;
    );
    assert_eq!(Index { key: 1, offset: 0 }, map.first().unwrap());
    assert_eq!(None, ContiguousMap::<usize, usize>::new().first());
}

#[test]
fn last() {
    let map = cmap!(
        1 => 11, 12;
        5 => 15, 16, 17;
    );
    assert_eq!(Index { key: 5, offset: 2 }, map.last().unwrap());
    assert_eq!(None, ContiguousMap::<usize, usize>::new().last());
}

#[test]
fn find() {
    let map = cmap!(
        1 => 11, 12;
        5 => 15, 16, 17;
    );
    assert_eq!(None, map.find(&0));
    assert_eq!(Index { key: 1, offset: 0 }, map.find(&1).unwrap());
    assert_eq!(Index { key: 1, offset: 1 }, map.find(&2).unwrap());
    assert_eq!(None, map.find(&3));
    assert_eq!(None, map.find(&4));
    assert_eq!(Index { key: 5, offset: 0 }, map.find(&5).unwrap());
    assert_eq!(Index { key: 5, offset: 1 }, map.find(&6).unwrap());
    assert_eq!(Index { key: 5, offset: 2 }, map.find(&7).unwrap());
    assert_eq!(None, map.find(&8));
}

#[test]
fn find_at_most() {
    let map = cmap!(
        1 => 11, 12;
        5 => 15, 16, 17;
    );
    assert_eq!(None, map.find_at_most(&0));
    assert_eq!(Index { key: 1, offset: 0 }, map.find_at_most(&1).unwrap());
    assert_eq!(Index { key: 1, offset: 1 }, map.find_at_most(&4).unwrap());
    assert_eq!(Index { key: 5, offset: 2 }, map.find_at_most(&9).unwrap());
}

#[test]
fn find_less() {
    let map = cmap!(
        1 => 11, 12;
        5 => 15, 16, 17;
    );
    assert_eq!(None, map.find_less(&0));
    assert_eq!(None, map.find_less(&1));
    assert_eq!(Index { key: 1, offset: 0 }, map.find_less(&2).unwrap());
    assert_eq!(Index { key: 1, offset: 1 }, map.find_less(&5).unwrap());
    assert_eq!(Index { key: 5, offset: 2 }, map.find_less(&100).unwrap());
}

#[test]
fn find_at_least() {
    let map = cmap!(
        1 => 11, 12;
        5 => 15, 16, 17;
    );
    assert_eq!(Index { key: 1, offset: 0 }, map.find_at_least(&0).unwrap());
    assert_eq!(Index { key: 1, offset: 0 }, map.find_at_least(&1).unwrap());
    assert_eq!(Index { key: 1, offset: 1 }, map.find_at_least(&2).unwrap());
    assert_eq!(Index { key: 5, offset: 0 }, map.find_at_least(&3).unwrap());
    assert_eq!(None, map.find_at_least(&100));
}

#[test]
fn find_more() {
    let map = cmap!(
        1 => 11, 12;
        5 => 15, 16, 17;
    );
    assert_eq!(Index { key: 1, offset: 0 }, map.find_more(&0).unwrap());
    assert_eq!(Index { key: 1, offset: 1 }, map.find_more(&1).unwrap());
    assert_eq!(Index { key: 5, offset: 0 }, map.find_more(&2).unwrap());
    assert_eq!(Index { key: 5, offset: 1 }, map.find_more(&5).unwrap());
    assert_eq!(None, map.find_more(&100));
}

#[test]
fn range_empty() {
    let map = cmap!(
        1 => 11, 12;
        5 => 15, 16, 17;
    );
    let range = map.range(3..5);
    assert_double_ended_iter_exhausted(range);
}

#[test]
fn range_single_slice() {
    let map = cmap!(
        1 => 11, 12;
        5 => 15, 16, 17;
    );
    let mut range = map.range(5..10);
    assert_eq!((5, &15), range.next().unwrap());
    assert_eq!((7, &17), range.next_back().unwrap());
    assert_eq!((6, &16), range.next().unwrap());
    assert_double_ended_iter_exhausted(range);
}

#[test]
fn range_multi_slice() {
    let map = cmap!(
        1 => 11, 12;
        5 => 15, 16, 17;
    );
    let mut range = map.range(2..7);
    assert_eq!((2, &12), range.next().unwrap());
    assert_eq!((6, &16), range.next_back().unwrap());
    assert_eq!((5, &15), range.next().unwrap());
    assert_double_ended_iter_exhausted(range);
}

#[test]
fn range_forward() {
    let map = cmap!(1 => 11; 3 => 13; 5 => 15);
    let mut range = map.range(..);
    assert_eq!((1, &11), range.next().unwrap());
    assert_eq!((3, &13), range.next().unwrap());
    assert_eq!((5, &15), range.next().unwrap());
    assert_double_ended_iter_exhausted(range);
}

#[test]
fn range_backward() {
    let map = cmap!(1 => 11; 3 => 13; 5 => 15);
    let mut range = map.range(..);
    assert_eq!((5, &15), range.next_back().unwrap());
    assert_eq!((3, &13), range.next_back().unwrap());
    assert_eq!((1, &11), range.next_back().unwrap());
    assert_double_ended_iter_exhausted(range);
}

#[test]
fn range_mut_empty() {
    let mut map = cmap!(
        1 => 11, 12;
        5 => 15, 16, 17;
    );
    let range = map.range_mut(3..5);
    assert_double_ended_iter_exhausted(range);
}

#[test]
fn range_mut_single_slice() {
    let mut map = cmap!(
        1 => 11, 12;
        5 => 15, 16, 17;
    );
    let mut range = map.range_mut(5..10);
    assert_eq!((5, &mut 15), range.next().unwrap());
    assert_eq!((7, &mut 17), range.next_back().unwrap());
    assert_eq!((6, &mut 16), range.next().unwrap());
    assert_double_ended_iter_exhausted(range);
}

#[test]
fn range_mut_multi_slice() {
    let mut map = cmap!(
        1 => 11, 12;
        5 => 15, 16, 17;
    );
    let mut range = map.range_mut(2..7);
    assert_eq!((2, &mut 12), range.next().unwrap());
    assert_eq!((6, &mut 16), range.next_back().unwrap());
    assert_eq!((5, &mut 15), range.next().unwrap());
    assert_double_ended_iter_exhausted(range);
}

#[test]
fn range_mut_forward() {
    let mut map = cmap!(1 => 11; 3 => 13; 5 => 15);
    let mut range = map.range_mut(..);
    assert_eq!((1, &mut 11), range.next().unwrap());
    assert_eq!((3, &mut 13), range.next().unwrap());
    assert_eq!((5, &mut 15), range.next().unwrap());
    assert_double_ended_iter_exhausted(range);
}

#[test]
fn range_mut_backward() {
    let mut map = cmap!(1 => 11; 3 => 13; 5 => 15);
    let mut range = map.range_mut(..);
    assert_eq!((5, &mut 15), range.next_back().unwrap());
    assert_eq!((3, &mut 13), range.next_back().unwrap());
    assert_eq!((1, &mut 11), range.next_back().unwrap());
    assert_double_ended_iter_exhausted(range);
}
