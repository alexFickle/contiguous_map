use crate::{cmap, ContiguousMap, Index};

#[test]
fn empty() {
    let map = ContiguousMap::<i32, i32>::new();
    assert_eq!(None, map.first());
}

#[test]
fn non_empty() {
    let map = cmap!(
        1 => 1, 2;
        5 => 1, 2, 3;
    );
    assert_eq!(Index { key: 1, offset: 0 }, map.first().unwrap());
}
