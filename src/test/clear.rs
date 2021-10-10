use super::assert_map_same;
use crate::cmap;

#[test]
fn empty() {
    let mut map = cmap!();
    map.clear();
    assert_map_same(&map, []);
}

#[test]
fn one_slice() {
    let mut map = cmap!(10 => 0, 1, 2);
    map.clear();
    assert_map_same(&map, []);
}

#[test]
fn two_slices() {
    let mut map = cmap!(
        10 => 0, 1, 2;
        20 => 0, 1;
    );
    map.clear();
    assert_map_same(&map, []);
}
