use crate::{cmap, ContiguousMap};

#[test]
fn empty() {
    let map = ContiguousMap::<i32, i32>::new();
    assert!(map.is_empty());
}

#[test]
fn one_region() {
    let map = cmap!(0 => 1, 2, 3);
    assert!(!map.is_empty());
}

#[test]
fn two_regions() {
    let map = cmap!(
        0 => 1, 2, 3;
        5 => 1, 2, 3, 4;
    );
    assert!(!map.is_empty());
}
