use crate::{cmap, ContiguousMap};

#[test]
fn empty() {
    let map = ContiguousMap::<usize, usize>::new();
    assert_eq!(None, map.get(2));
}

#[test]
fn before_first_region() {
    let map = cmap!(
        1 => 11, 12, 13;
        5 => 15;
    );
    assert_eq!(None, map.get(0));
}

#[test]
fn front_of_slice() {
    let map = cmap!(
        1 => 11, 12, 13;
        5 => 15;
    );
    assert_eq!(&11, map.get(1).unwrap());
}

#[test]
fn middle_of_slice() {
    let map = cmap!(
        1 => 11, 12, 13;
        5 => 15;
    );
    assert_eq!(&12, map.get(2).unwrap());
}

#[test]
fn end_of_slice() {
    let map = cmap!(
        1 => 11, 12, 13;
        5 => 15;
    );
    assert_eq!(&13, map.get(3).unwrap());
}

#[test]
fn in_gap() {
    let map = cmap!(
        1 => 11, 12, 13;
        5 => 15;
    );
    assert_eq!(None, map.get(4));
}

#[test]
fn isolated() {
    let map = cmap!(
        1 => 11, 12, 13;
        5 => 15;
    );
    assert_eq!(&15, map.get(5).unwrap());
}

#[test]
fn after_last_region() {
    let map = cmap!(
        1 => 11, 12, 13;
        5 => 15;
    );
    assert_eq!(None, map.get(6));
}
