use crate::cmap;

#[test]
fn entire_slice() {
    let map = cmap!(3 => 13, 14, 15);
    assert_eq!([13, 14, 15], map.get_slice_with_len(3, 3).unwrap());
}

#[test]
fn front_of_slice() {
    let map = cmap!(3 => 13, 14, 15);
    assert_eq!([13, 14], map.get_slice_with_len(3, 2).unwrap());
}

#[test]
fn middle_of_slice() {
    let map = cmap!(3 => 13, 14, 15);
    assert_eq!([14], map.get_slice_with_len(4, 1).unwrap());
}

#[test]
fn back_of_slice() {
    let map = cmap!(3 => 13, 14, 15);
    assert_eq!([14, 15], map.get_slice_with_len(4, 2).unwrap());
}

#[test]
fn length_too_long() {
    let map = cmap!(3 => 13, 14, 15);
    assert_eq!(None, map.get_slice_with_len(3, 4));
}

#[test]
fn starts_too_early() {
    let map = cmap!(3 => 13, 14, 15);
    assert_eq!(None, map.get_slice_with_len(2, 3));
}

#[test]
fn starts_too_late() {
    let map = cmap!(3 => 13, 14, 15);
    assert_eq!(None, map.get_slice_with_len(6, 2));
}

#[test]
fn near_length_overflow() {
    let map = cmap!(usize::MAX - 3 => 1, 2, 3);
    assert_eq!(
        [1, 2, 3],
        map.get_slice_with_len(usize::MAX - 3, 3).unwrap()
    );
}

#[test]
fn length_overflow() {
    let map = cmap!(usize::MAX - 3 => 1, 2, 3);
    assert_eq!(None, map.get_slice_with_len(usize::MAX - 3, 4));
}

#[test]
fn length_zero() {
    let map = cmap!(3 => 13, 14, 15);
    // should always error
    assert_eq!(None, map.get_slice_with_len(4, 0));
}
