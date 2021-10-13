use super::assert_map_same;
use crate::cmap;

#[test]
fn empty() {
    let map = cmap!();
    let cloned = map.clone();
    assert_map_same(&cloned, []);
}

#[test]
fn one_slice() {
    let map = cmap!(1 => 1, 2, 3);
    let cloned = map.clone();
    assert_map_same(&cloned, [(1, vec![1, 2, 3])]);
}

#[test]
fn two_slices() {
    let map = cmap!(
        10 => 1, 2, 3;
        20 => 6, 7, 8, 9;
    );
    let cloned = map.clone();
    assert_map_same(&cloned, [(10, vec![1, 2, 3]), (20, vec![6, 7, 8, 9])]);
}
