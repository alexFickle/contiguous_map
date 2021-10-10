use crate::{cmap, ContiguousMap, Index};

#[test]
fn empty() {
    let map = ContiguousMap::<i32, i32>::new();
    assert_eq!(None, map.find_less(&0));
}

#[test]
fn before_slice() {
    let map = cmap!(0 => 1, 2, 3);
    assert_eq!(None, map.find_less(&-1));
}

#[test]
fn front_of_slice() {
    let map = cmap!(0 => 1, 2, 3);
    assert_eq!(None, map.find_less(&0));
}

#[test]
fn middle_of_slice() {
    let map = cmap!(0 => 1, 2, 3);
    assert_eq!(Index { key: 0, offset: 0 }, map.find_less(&1).unwrap());
}

#[test]
fn end_of_slice() {
    let map = cmap!(0 => 1, 2, 3);
    assert_eq!(Index { key: 0, offset: 1 }, map.find_less(&2).unwrap());
}

#[test]
fn after_slice() {
    let map = cmap!(0 => 1, 2, 3);
    assert_eq!(Index { key: 0, offset: 2 }, map.find_less(&3).unwrap());
}

#[test]
fn between_slices() {
    let map = cmap!(
        1 => 1, 2;
        4 => 4, 5;
    );
    assert_eq!(Index { key: 1, offset: 1 }, map.find_less(&3).unwrap());
}

#[test]
fn start_of_second_slice() {
    let map = cmap!(
        1 => 1, 2;
        4 => 4, 5;
    );
    assert_eq!(Index { key: 1, offset: 1 }, map.find_less(&4).unwrap());
}
