use super::assert_map_same;
use crate::ContiguousMap;

#[test]
fn into_empty() {
    let mut map = ContiguousMap::new();
    assert!(map.insert(1, 2).is_none());
    assert_map_same(&map, [(1, vec![2])]);
}

#[test]
fn overwrite() {
    let mut map = ContiguousMap::new();
    assert!(map.insert(1, 2).is_none());
    assert_eq!(Some(2), map.insert(1, 3));
    assert_map_same(&map, [(1, vec![3])]);
}

#[test]
fn after_adjacent() {
    let mut map = ContiguousMap::new();
    assert!(map.insert(1, 10).is_none());
    assert!(map.insert(2, 12).is_none());
    assert_map_same(&map, [(1, vec![10, 12])]);
}

#[test]
fn after_with_gap() {
    let mut map = ContiguousMap::new();
    assert!(map.insert(0, 10).is_none());
    assert!(map.insert(2, 12).is_none());
    assert_map_same(&map, [(0, vec![10]), (2, vec![12])]);
}

#[test]
fn before_adjacent() {
    let mut map = ContiguousMap::new();
    assert!(map.insert(2, 12).is_none());
    assert!(map.insert(1, 10).is_none());
    assert_map_same(&map, [(1, vec![10, 12])]);
}

#[test]
fn before_with_gap() {
    let mut map = ContiguousMap::new();
    assert!(map.insert(2, 12).is_none());
    assert!(map.insert(0, 10).is_none());
    assert_map_same(&map, [(0, vec![10]), (2, vec![12])]);
}

#[test]
fn into_size_one_gap() {
    let mut map = ContiguousMap::new();
    assert!(map.insert(0, 10).is_none());
    assert!(map.insert(2, 12).is_none());
    assert!(map.insert(1, 11).is_none());
    assert_map_same(&map, [(0, vec![10, 11, 12])]);
}
