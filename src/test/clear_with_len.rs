use super::assert_map_same;
use crate::cmap;

#[test]
fn empty() {
    let mut map = cmap!();
    map.clear_with_len(10, 5);
    assert_map_same(&map, []);
}

#[test]
fn start_of_region() {
    let mut map = cmap!(10 => 0, 1, 2, 3);
    map.clear_with_len(10, 2);
    assert_map_same(&map, [(12, vec![2, 3])]);
}

#[test]
fn middle_of_region() {
    let mut map = cmap!(10 => 0, 1, 2, 3);
    map.clear_with_len(11, 2);
    assert_map_same(&map, [(10, vec![0]), (13, vec![3])]);
}

#[test]
fn end_of_region() {
    let mut map = cmap!(10 => 0, 1, 2, 3);
    map.clear_with_len(12, 2);
    assert_map_same(&map, [(10, vec![0, 1])]);
}

#[test]
fn entire_region() {
    let mut map = cmap!(10 => 0, 1, 2, 3, 4, 5);
    map.clear_with_len(10, 6);
    assert_map_same(&map, []);
}

#[test]
fn across_regions() {
    let mut map = cmap!(
        10 => 0, 1, 2, 3;
        20 => 0, 1, 2, 3;
    );
    map.clear_with_len(12, 10);
    assert_map_same(&map, [(10, vec![0, 1]), (22, vec![2, 3])]);
}

#[test]
fn entire_map() {
    let mut map = cmap!(
        10 => 0, 1, 2, 3;
        20 => 0, 1, 2, 3;
        30 => 0, 1, 2, 3;
    );
    map.clear_with_len(10, 24);
    assert_map_same(&map, []);
}

#[test]
fn overflow() {
    let mut map = cmap!(usize::MAX - 3 => 1, 2, 3);
    map.clear_with_len(usize::MAX - 2, 3);
    assert_map_same(&map, [(usize::MAX - 3, vec![1])]);
}
