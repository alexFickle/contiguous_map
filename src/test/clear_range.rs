use super::assert_map_same;
use crate::cmap;

#[test]
fn empty() {
    let mut map = cmap!();
    map.clear_range(..);
    assert_map_same(&map, []);
}

#[test]
fn start_of_region() {
    let mut map = cmap!(10 => 0, 1, 2, 3, 4);
    map.clear_range(10..13);
    assert_map_same(&map, [(13, vec![3, 4])]);
}

#[test]
fn middle_of_region() {
    let mut map = cmap!(10 => 0, 1, 2, 3, 4);
    map.clear_range(11..14);
    assert_map_same(&map, [(10, vec![0]), (14, vec![4])]);
}

#[test]
fn end_of_region() {
    let mut map = cmap!(10 => 0, 1, 2, 3, 4);
    map.clear_range(12..15);
    assert_map_same(&map, [(10, vec![0, 1])]);
}

#[test]
fn entire_region() {
    let mut map = cmap!(10 => 0, 1, 2, 3, 4, 5);
    map.clear_range(10..16);
    assert_map_same(&map, []);
}

#[test]
fn across_regions() {
    let mut map = cmap!(
        10 => 0, 1, 2, 3, 4;
        20 => 0, 1, 2, 3, 4;
    );
    map.clear_range(12..22);
    assert_map_same(&map, [(10, vec![0, 1]), (22, vec![2, 3, 4])]);
}

#[test]
fn entire_map() {
    let mut map = cmap!(
        10 => 0, 1, 2, 3;
        20 => 0, 1, 2, 3;
        30 => 0, 1, 2, 3;
    );
    map.clear_range(..);
    assert_map_same(&map, []);
}
