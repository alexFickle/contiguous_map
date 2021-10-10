use super::assert_map_same;
use crate::cmap;

#[test]
fn entire_slice() {
    let mut map = cmap!(3 => 13, 14, 15);
    assert_eq!([13, 14, 15], map.get_slice_with_len_mut(3, 3).unwrap());
}

#[test]
fn front_of_slice() {
    let mut map = cmap!(3 => 13, 14, 15);
    assert_eq!([13, 14], map.get_slice_with_len_mut(3, 2).unwrap());
}

#[test]
fn middle_of_slice() {
    let mut map = cmap!(3 => 13, 14, 15);
    assert_eq!([14], map.get_slice_with_len_mut(4, 1).unwrap());
}

#[test]
fn back_of_slice() {
    let mut map = cmap!(3 => 13, 14, 15);
    assert_eq!([14, 15], map.get_slice_with_len_mut(4, 2).unwrap());
}

#[test]
fn length_too_long() {
    let mut map = cmap!(3 => 13, 14, 15);
    assert_eq!(None, map.get_slice_with_len_mut(3, 4));
}

#[test]
fn starts_too_early() {
    let mut map = cmap!(3 => 13, 14, 15);
    assert_eq!(None, map.get_slice_with_len_mut(2, 3));
}

#[test]
fn starts_too_late() {
    let mut map = cmap!(3 => 13, 14, 15);
    assert_eq!(None, map.get_slice_with_len_mut(6, 2));
}

#[test]
fn near_length_overflow() {
    let mut map = cmap!(usize::MAX - 3 => 1, 2, 3);
    assert_eq!(
        [1, 2, 3],
        map.get_slice_with_len_mut(usize::MAX - 3, 3).unwrap()
    );
}

#[test]
fn length_overflow() {
    let mut map = cmap!(usize::MAX - 3 => 1, 2, 3);
    assert_eq!(None, map.get_slice_with_len_mut(usize::MAX - 3, 4));
}

#[test]
fn length_zero() {
    let mut map = cmap!(3 => 13, 14, 15);
    // should always error
    assert_eq!(None, map.get_slice_with_len_mut(4, 0));
}

#[test]
fn mutate() {
    let mut map = cmap!(1 => 1, 2, 3);
    let slice = map.get_slice_with_len_mut(1, 3).unwrap();
    slice[0] = 4;
    slice[1] = 5;
    slice[2] = 6;
    assert_map_same(&map, [(1, vec![4, 5, 6])]);
}
