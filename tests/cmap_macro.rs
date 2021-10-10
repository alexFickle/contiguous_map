//! This file ensures that the cmap macro works as expected across crate boundaries.
//!
//! I intentionally do not import anything from the contiguous_map crate to ensure
//! that the macro does not rely on these imports.

#![no_implicit_prelude]

#[test]
fn empty() {
    let map: ::contiguous_map::ContiguousMap<i8, i8> = ::contiguous_map::cmap!();
    assert!(map.is_empty())
}

#[test]
fn one_value() {
    let map = ::contiguous_map::cmap!(1 => 2);
    ::std::assert_eq!(1, map.len());
    ::std::assert_eq!(2, *map.get(1).unwrap());
}

#[test]
fn one_value_trailing_comma() {
    let map = ::contiguous_map::cmap!(1 => 2,);
    ::std::assert_eq!(1, map.len());
    ::std::assert_eq!(2, *map.get(1).unwrap());
}

#[test]
fn one_value_trailing_semicolon() {
    let map = ::contiguous_map::cmap!(1 => 2;);
    ::std::assert_eq!(1, map.len());
    ::std::assert_eq!(2, *map.get(1).unwrap());
}

#[test]
fn two_values() {
    let map = ::contiguous_map::cmap!(1 => 2; 2 => 4);
    ::std::assert_eq!(2, map.len());
    ::std::assert_eq!(2, *map.get(1).unwrap());
    ::std::assert_eq!(4, *map.get(2).unwrap());
}

#[test]
fn one_sequence() {
    let map = ::contiguous_map::cmap!(1 => 1, 2, 3);
    ::std::assert_eq!(3, map.len());
    ::std::assert_eq!(&[1, 2, 3], map.get_slice(1..).unwrap());
}

#[test]
fn one_sequence_trailing_comma() {
    let map = ::contiguous_map::cmap!(1 => 1, 2, 3,);
    ::std::assert_eq!(3, map.len());
    ::std::assert_eq!(&[1, 2, 3], map.get_slice(1..).unwrap());
}

#[test]
fn two_sequences() {
    let map = ::contiguous_map::cmap!(
        0 => 1, 2, 3;
        10 => 11, 12, 13, 14
    );
    ::std::assert_eq!(7, map.len());
    ::std::assert_eq!(&[1, 2, 3], map.get_slice(0..).unwrap());
    ::std::assert_eq!(&[11, 12, 13, 14], map.get_slice(10..).unwrap());
}

#[test]
fn two_sequences_trailing_comma_and_semicolon() {
    let map = ::contiguous_map::cmap!(
        0 => 1, 2, 3,;
        10 => 11, 12, 13, 14,;
    );
    ::std::assert_eq!(7, map.len());
    ::std::assert_eq!(&[1, 2, 3], map.get_slice(0..).unwrap());
    ::std::assert_eq!(&[11, 12, 13, 14], map.get_slice(10..).unwrap());
}

#[test]
fn near_overflow() {
    let map = ::contiguous_map::cmap!(
        usize::MAX - 1 => 1
    );
    ::std::assert_eq!(1, map.len());
}
