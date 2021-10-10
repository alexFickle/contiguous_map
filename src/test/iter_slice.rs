use super::assert_de_iter_empty;
use crate::{cmap, ContiguousMap};

#[test]
fn empty() {
    let map = ContiguousMap::<usize, i32>::new();
    let iter = map.iter_slice();
    assert_de_iter_empty(iter);
}

#[test]
fn forward() {
    let map = cmap!(
        10 => 0, 1, 2;
        20 => 0, 1;
        30 => 0,
    );
    let mut iter = map.iter_slice();
    assert_eq!((&10, &[0, 1, 2][..]), iter.next().unwrap());
    assert_eq!((&20, &[0, 1][..]), iter.next().unwrap());
    assert_eq!((&30, &[0][..]), iter.next().unwrap());
    assert_de_iter_empty(iter);
}

#[test]
fn reverse() {
    let map = cmap!(
        10 => 0, 1, 2;
        20 => 0, 1;
        30 => 0,
    );
    let mut iter = map.iter_slice();
    assert_eq!((&30, &[0][..]), iter.next_back().unwrap());
    assert_eq!((&20, &[0, 1][..]), iter.next_back().unwrap());
    assert_eq!((&10, &[0, 1, 2][..]), iter.next_back().unwrap());
    assert_de_iter_empty(iter);
}

#[test]
fn double_ended() {
    let map = cmap!(
        10 => 0, 1, 2;
        20 => 0, 1;
        30 => 0,
    );
    let mut iter = map.iter_slice();
    assert_eq!((&10, &[0, 1, 2][..]), iter.next().unwrap());
    assert_eq!((&30, &[0][..]), iter.next_back().unwrap());
    assert_eq!((&20, &[0, 1][..]), iter.next().unwrap());
    assert_de_iter_empty(iter);
}
