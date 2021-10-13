use crate::cmap;

#[test]
fn equal() {
    let map1 = cmap!(
        10 => 1, 2, 3;
        20 => 5, 6, 7;
    );
    let map2 = cmap!(
        10 => 1, 2, 3;
        20 => 5, 6, 7;
    );
    assert_eq!(map1, map2);
}

#[test]
fn value_different() {
    let map1 = cmap!(10 => 1, 2, 3);
    let map2 = cmap!(10 => 1, 2, 5);
    assert_ne!(map1, map2);
}

#[test]
fn slice_ends_early() {
    let map1 = cmap!(10 => 1, 2, 3);
    let map2 = cmap!(10 => 1, 2);
    assert_ne!(map1, map2);
}

#[test]
fn slice_starts_late() {
    let map1 = cmap!(10 => 1, 2, 3);
    let map2 = cmap!(11 => 2, 3);
    assert_ne!(map1, map2);
}

#[test]
fn extra_slice() {
    let map1 = cmap!(10 => 1, 2, 3);
    let map2 = cmap!(
        10 => 1, 2, 3;
        20 => 5, 6, 7;
    );
    assert_ne!(map1, map2);
}
