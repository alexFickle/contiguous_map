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
    let mut map = ContiguousMap::<usize, usize>::new();
    map.insert(2, 12);
    map.insert(3, 13);
    map.insert(4, 14);
    map.insert(9, 19);
    assert_eq!(Some(&12), map.get(&2));
    assert_eq!(Some(&13), map.get(3));
    assert_eq!(Some(&14), map.get(&4));
    assert_eq!(Some(&19), map.get(9));
}

#[test]
fn get_missing() {
    let mut map = ContiguousMap::<usize, usize>::new();
    assert_eq!(None, map.get(&2));
    map.insert(1, 11);
    map.insert(3, 13);
    assert_eq!(None, map.get(&0));
    assert_eq!(None, map.get(&2));
    assert_eq!(None, map.get(&4));
}

#[test]
fn get_mut() {
    let mut map = ContiguousMap::<usize, usize>::new();
    map.insert(2, 12);
    map.insert(3, 13);
    map.insert(4, 14);
    map.insert(9, 19);
    assert_eq!(Some(&mut 12), map.get_mut(&2));
    assert_eq!(Some(&mut 13), map.get_mut(3));
    assert_eq!(Some(&mut 14), map.get_mut(&4));
    assert_eq!(Some(&mut 19), map.get_mut(9));
}

#[test]
fn get_mut_missing() {
    let mut map = ContiguousMap::<usize, usize>::new();
    assert_eq!(None, map.get_mut(&2));
    map.insert(1, 11);
    map.insert(3, 13);
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
    let map = {
        let mut map = ContiguousMap::<usize, usize>::new();
        map.insert_slice(3, &[1, 2, 3]);
        map
    };

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
    let map = {
        let mut map = ContiguousMap::<usize, usize>::new();
        map.insert_slice(3, &[1, 2, 3]);
        map
    };

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
    let map = {
        let mut map = ContiguousMap::<usize, usize>::new();
        map.insert_slice(3, &[1, 2, 3]);
        map
    };

    assert_eq!(None, map.get_slice(2..));
    assert_eq!([1, 2, 3], map.get_slice(3..).unwrap());
    assert_eq!([2, 3], map.get_slice(4..).unwrap());
    assert_eq!([3], map.get_slice(5..).unwrap());
    assert_eq!(None, map.get_slice(6..));
}

#[test]
fn get_slice_with_len() {
    let map = {
        let mut map = ContiguousMap::<usize, usize>::new();
        map.insert_slice(3, &[1, 2, 3]);
        map
    };

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
    let mut map = ContiguousMap::<usize, usize>::new();
    map.insert_slice(3, &[1, 2, 3]);

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
    let mut map = ContiguousMap::<usize, usize>::new();
    map.insert_slice(3, &[1, 2, 3]);

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
    let mut map = ContiguousMap::<usize, usize>::new();
    map.insert_slice(3, &[1, 2, 3]);

    assert_eq!(None, map.get_slice_mut(2..));
    assert_eq!([1, 2, 3], map.get_slice_mut(3..).unwrap());
    assert_eq!([2, 3], map.get_slice_mut(4..).unwrap());
    assert_eq!([3], map.get_slice_mut(5..).unwrap());
    assert_eq!(None, map.get_slice_mut(6..));
}

#[test]
fn get_slice_with_len_mut() {
    let mut map = ContiguousMap::<usize, usize>::new();
    map.insert_slice(3, &[1, 2, 3]);

    assert_eq!(None, map.get_slice_with_len_mut(2, 2));
    assert_eq!(None, map.get_slice_with_len_mut(2, 4));
    assert_eq!(None, map.get_slice_with_len_mut(3, 4));
    assert_eq!([1, 2, 3], map.get_slice_with_len_mut(3, 3).unwrap());
    assert_eq!([2, 3], map.get_slice_with_len_mut(4, 2).unwrap());
    assert_eq!([3], map.get_slice_with_len_mut(5, 1).unwrap());
    assert_eq!([1, 2], map.get_slice_with_len_mut(3, 2).unwrap());
}
