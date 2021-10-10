use super::assert_map_same;
use crate::cmap;

#[test]
fn empty() {
    let mut map = cmap!();
    assert_eq!(None, map.remove(0));
    assert_map_same(&map, []);
}

#[test]
fn before_slice() {
    let mut map = cmap!(10 => 0, 1, 2);
    assert_eq!(None, map.remove(9));
    assert_map_same(&map, [(10, vec![0, 1, 2])]);
}

#[test]
fn after_slice() {
    let mut map = cmap!(10 => 0, 1, 2);
    assert_eq!(None, map.remove(13));
    assert_map_same(&map, [(10, vec![0, 1, 2])]);
}

#[test]
fn between_slices() {
    let mut map = cmap!(
        1 => 1, 2;
        4 => 4, 5, 6;
    );
    assert_eq!(None, map.remove(3));
    assert_map_same(&map, [(1, vec![1, 2]), (4, vec![4, 5, 6])]);
}

#[test]
fn front_of_slice() {
    let mut map = cmap!(10 => 0, 1, 2);
    assert_eq!(0, map.remove(10).unwrap());
    assert_map_same(&map, [(11, vec![1, 2])]);
}

#[test]
fn middle_of_slice() {
    let mut map = cmap!(10 => 0, 1, 2);
    assert_eq!(1, map.remove(11).unwrap());
    assert_map_same(&map, [(10, vec![0]), (12, vec![2])]);
}

#[test]
fn back_of_slice() {
    let mut map = cmap!(10 => 0, 1, 2);
    assert_eq!(2, map.remove(12).unwrap());
    assert_map_same(&map, [(10, vec![0, 1])]);
}

#[test]
fn isolated() {
    let mut map = cmap!(0 => 10; 2 => 12; 4 => 14);
    assert_eq!(12, map.remove(2).unwrap());
    assert_map_same(&map, [(0, vec![10]), (4, vec![14])]);
}
