use super::assert_de_iter_empty;
use crate::cmap;

#[test]
fn empty() {
    let mut map = cmap!(
        1 => 11, 12;
        5 => 15, 16, 17;
    );
    let range = map.range_mut(3..5);
    assert_de_iter_empty(range);
}

#[test]
fn single_slice() {
    let mut map = cmap!(
        1 => 11, 12;
        5 => 15, 16, 17;
    );
    let mut range = map.range_mut(5..10);
    assert_eq!((5, &mut 15), range.next().unwrap());
    assert_eq!((7, &mut 17), range.next_back().unwrap());
    assert_eq!((6, &mut 16), range.next().unwrap());
    assert_de_iter_empty(range);
}

#[test]
fn multi_slice() {
    let mut map = cmap!(
        1 => 11, 12;
        5 => 15, 16, 17;
    );
    let mut range = map.range_mut(2..7);
    assert_eq!((2, &mut 12), range.next().unwrap());
    assert_eq!((6, &mut 16), range.next_back().unwrap());
    assert_eq!((5, &mut 15), range.next().unwrap());
    assert_de_iter_empty(range);
}

#[test]
fn forward() {
    let mut map = cmap!(1 => 11; 3 => 13; 5 => 15);
    let mut range = map.range_mut(..);
    assert_eq!((1, &mut 11), range.next().unwrap());
    assert_eq!((3, &mut 13), range.next().unwrap());
    assert_eq!((5, &mut 15), range.next().unwrap());
    assert_de_iter_empty(range);
}

#[test]
fn reverse() {
    let mut map = cmap!(1 => 11; 3 => 13; 5 => 15);
    let mut range = map.range_mut(..);
    assert_eq!((5, &mut 15), range.next_back().unwrap());
    assert_eq!((3, &mut 13), range.next_back().unwrap());
    assert_eq!((1, &mut 11), range.next_back().unwrap());
    assert_de_iter_empty(range);
}
