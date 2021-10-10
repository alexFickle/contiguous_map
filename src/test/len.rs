use crate::{cmap, ContiguousMap};

#[test]
fn empty() {
    let map = ContiguousMap::<i32, i32>::new();
    assert_eq!(0, map.len());
}

#[test]
fn one_region() {
    let map = cmap!(0 => 1, 2, 3);
    assert_eq!(3, map.len());
}

#[test]
fn two_regions() {
    let map = cmap!(
        0 => 1, 2, 3;
        5 => 1, 2, 3, 4;
    );
    assert_eq!(7, map.len());
}
