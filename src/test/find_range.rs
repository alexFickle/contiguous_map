use crate::{cmap, ContiguousMap, Index};
use std::ops::Bound;

#[test]
fn included_start() {
    let map = cmap!(10 => 1, 2, 3);
    let start = map.find_range(11..).unwrap().0;
    assert_eq!(Index { key: 10, offset: 1 }, start);
}

#[test]
fn excluded_start() {
    let map = cmap!(10 => 1, 2, 3);
    let start = map
        .find_range((Bound::Excluded(10), Bound::Unbounded))
        .unwrap()
        .0;
    assert_eq!(Index { key: 10, offset: 1 }, start);
}

#[test]
fn unbounded_start() {
    let map = cmap!(10 => 1, 2, 3);
    let start = map.find_range(..).unwrap().0;
    assert_eq!(Index { key: 10, offset: 0 }, start);
}

#[test]
fn included_end() {
    let map = cmap!(10 => 1, 2, 3);
    let end = map.find_range(..=11).unwrap().1;
    assert_eq!(Index { key: 10, offset: 1 }, end);
}

#[test]
fn excluded_end() {
    let map = cmap!(10 => 1, 2, 3);
    let end = map.find_range(..12).unwrap().1;
    assert_eq!(Index { key: 10, offset: 1 }, end);
}

#[test]
fn unbounded_end() {
    let map = cmap!(10 => 1, 2, 3);
    let end = map.find_range(..).unwrap().1;
    assert_eq!(Index { key: 10, offset: 2 }, end);
}

#[test]
fn empty() {
    let map = ContiguousMap::<usize, i32>::new();
    assert_eq!(None, map.find_range(..));
}

#[test]
fn in_gap() {
    let map = cmap!(
        10 => 1, 2, 3;
        20 => 1, 2, 3;
    );
    assert_eq!(None, map.find_range(13..20));
}

#[test]
fn single_entry_in_range() {
    let map = cmap!(10 => 1, 2, 3);
    let (start, end) = map.find_range(11..12).unwrap();
    assert_eq!(Index { key: 10, offset: 1 }, start);
    assert_eq!(Index { key: 10, offset: 1 }, end);
}

#[test]
fn spans_multiple_regions() {
    let map = cmap!(
        10 => 0, 1, 2, 3;
        20 => 0, 1, 2, 3;
    );
    let (start, end) = map.find_range(11..=22).unwrap();
    assert_eq!(Index { key: 10, offset: 1 }, start);
    assert_eq!(Index { key: 20, offset: 2 }, end);
}
