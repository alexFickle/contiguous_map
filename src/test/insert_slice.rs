use super::assert_map_same;
use crate::{cmap, ContiguousMap};

#[test]
fn into_empty() {
    let mut map = ContiguousMap::new();
    map.insert_slice(3, &[1, 2, 3]);
    assert_map_same(&map, [(3, vec![1, 2, 3])]);
}

#[test]
fn overwrite() {
    let mut map = cmap!(10 => 1, 2, 3);
    map.insert_slice(10, &[4, 5, 6]);
    assert_map_same(&map, [(10, vec![4, 5, 6])]);
}

#[test]
fn append() {
    let mut map = cmap!(10 => 1, 2, 3);
    map.insert_slice(13, &[4, 5, 6]);
    assert_map_same(&map, [(10, vec![1, 2, 3, 4, 5, 6])]);
}

#[test]
fn prepend() {
    let mut map = cmap!(10 => 1, 2, 3);
    map.insert_slice(7, &[4, 5, 6]);
    assert_map_same(&map, [(7, vec![4, 5, 6, 1, 2, 3])]);
}

#[test]
fn merge() {
    let mut map = cmap!(
        10 => 1, 2, 3;
        16 => 4, 5, 6;
    );
    map.insert_slice(13, &[7, 8, 9]);
    assert_map_same(&map, [(10, vec![1, 2, 3, 7, 8, 9, 4, 5, 6])]);
}
