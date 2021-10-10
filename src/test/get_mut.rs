use super::assert_map_same;
use crate::{cmap, ContiguousMap};

#[test]
fn empty() {
    let mut map = ContiguousMap::<usize, usize>::new();
    assert_eq!(None, map.get_mut(2));
}

#[test]
fn before_first_region() {
    let mut map = cmap!(
        1 => 11, 12, 13;
        5 => 15;
    );
    assert_eq!(None, map.get_mut(0));
}

#[test]
fn front_of_slice() {
    let mut map = cmap!(
        1 => 11, 12, 13;
        5 => 15;
    );
    assert_eq!(&mut 11, map.get_mut(1).unwrap());
}

#[test]
fn middle_of_slice() {
    let mut map = cmap!(
        1 => 11, 12, 13;
        5 => 15;
    );
    assert_eq!(&mut 12, map.get_mut(2).unwrap());
}

#[test]
fn end_of_slice() {
    let mut map = cmap!(
        1 => 11, 12, 13;
        5 => 15;
    );
    assert_eq!(&mut 13, map.get_mut(3).unwrap());
}

#[test]
fn in_gap() {
    let mut map = cmap!(
        1 => 11, 12, 13;
        5 => 15;
    );
    assert_eq!(None, map.get_mut(4));
}

#[test]
fn isolated() {
    let mut map = cmap!(
        1 => 11, 12, 13;
        5 => 15;
    );
    assert_eq!(&mut 15, map.get_mut(5).unwrap());
}

#[test]
fn after_last_region() {
    let mut map = cmap!(
        1 => 11, 12, 13;
        5 => 15;
    );
    assert_eq!(None, map.get_mut(6));
}

#[test]
fn mutate() {
    let mut map = cmap!(1 => 1, 2, 3);
    *map.get_mut(2).unwrap() = 10;
    assert_map_same(&map, [(1, vec![1, 10, 3])]);
}
