use crate::cmap;

#[test]
fn equal() {
    let map1 = cmap!(
        10 => 1.0, 2.0, 3.0;
        20 => 5.0, 6.0, 7.0;
    );
    let map2 = cmap!(
        10 => 1.0, 2.0, 3.0;
        20 => 5.0, 6.0, 7.0;
    );
    assert_eq!(map1, map2);
}

#[test]
fn value_different() {
    let map1 = cmap!(10 => 1.0, 2.0, 3.0);
    let map2 = cmap!(10 => 1.0, 2.0, 5.0);
    assert_ne!(map1, map2);
}

#[test]
fn slice_ends_early() {
    let map1 = cmap!(10 => 1.0, 2.0, 3.0);
    let map2 = cmap!(10 => 1.0, 2.0);
    assert_ne!(map1, map2);
}

#[test]
fn slice_starts_late() {
    let map1 = cmap!(10 => 1.0, 2.0, 3.0);
    let map2 = cmap!(11 => 2.0, 3.0);
    assert_ne!(map1, map2);
}

#[test]
fn extra_slice() {
    let map1 = cmap!(10 => 1.0, 2.0, 3.0);
    let map2 = cmap!(
        10 => 1.0, 2.0, 3.0;
        20 => 5.0, 6.0, 7.0;
    );
    assert_ne!(map1, map2);
}

#[test]
fn contains_nan() {
    let map1 = cmap!(10 => 1.0, f64::NAN, 3.0);
    let map2 = cmap!(10 => 1.0, f64::NAN, 3.0);
    assert_ne!(map1, map2);
}
