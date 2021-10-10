use super::assert_map_same;
use crate::cmap;

mod range {
    use super::*;

    #[test]
    fn entire_slice() {
        let mut map = cmap!(3 => 13, 14, 15);
        assert_eq!([13, 14, 15], map.get_slice_mut(3..6).unwrap());
    }

    #[test]
    fn front_of_slice() {
        let mut map = cmap!(3 => 13, 14, 15);
        assert_eq!([13, 14], map.get_slice_mut(3..5).unwrap());
    }

    #[test]
    fn back_of_slice() {
        let mut map = cmap!(3 => 13, 14, 15);
        assert_eq!([14, 15], map.get_slice_mut(4..6).unwrap());
    }

    #[test]
    fn middle_of_slice() {
        let mut map = cmap!(3 => 13, 14, 15);
        assert_eq!([14], map.get_slice_mut(4..5).unwrap());
    }

    #[test]
    fn too_long() {
        let mut map = cmap!(3 => 13, 14, 15);
        assert_eq!(None, map.get_slice_mut(3..7));
    }

    #[test]
    fn starts_too_early() {
        let mut map = cmap!(3 => 13, 14, 15);
        assert_eq!(None, map.get_slice_mut(2..6));
    }

    #[test]
    fn starts_too_late() {
        let mut map = cmap!(3 => 13, 14, 15);
        assert_eq!(None, map.get_slice_mut(6..7));
    }

    #[test]
    fn contains_gap() {
        let mut map = cmap!(3 => 13; 5 => 15);
        assert_eq!(None, map.get_slice_mut(3..6));
    }

    #[test]
    fn empty_range() {
        // range is invalid
        let mut map = cmap!(3 => 13, 14, 15);
        assert_eq!(None, map.get_slice_mut(4..4));
    }

    #[test]
    fn backwards_range() {
        // range is invalid
        let mut map = cmap!(3 => 13, 14, 15);
        assert_eq!(None, map.get_slice_mut(4..3));
    }
}

mod range_to_inclusive {
    use super::*;

    #[test]
    fn entire_slice() {
        let mut map = cmap!(3 => 13, 14, 15);
        assert_eq!([13, 14, 15], map.get_slice_mut(3..=5).unwrap());
    }

    #[test]
    fn front_of_slice() {
        let mut map = cmap!(3 => 13, 14, 15);
        assert_eq!([13, 14], map.get_slice_mut(3..=4).unwrap());
    }

    #[test]
    fn back_of_slice() {
        let mut map = cmap!(3 => 13, 14, 15);
        assert_eq!([14, 15], map.get_slice_mut(4..=5).unwrap());
    }

    #[test]
    fn middle_of_slice() {
        let mut map = cmap!(3 => 13, 14, 15);
        assert_eq!([14], map.get_slice_mut(4..=4).unwrap());
    }

    #[test]
    fn too_long() {
        let mut map = cmap!(3 => 13, 14, 15);
        assert_eq!(None, map.get_slice_mut(3..=6));
    }

    #[test]
    fn starts_too_early() {
        let mut map = cmap!(3 => 13, 14, 15);
        assert_eq!(None, map.get_slice_mut(2..=5));
    }

    #[test]
    fn starts_too_late() {
        let mut map = cmap!(3 => 13, 14, 15);
        assert_eq!(None, map.get_slice_mut(6..=6));
    }

    #[test]
    fn contains_gap() {
        let mut map = cmap!(3 => 13; 5 => 15);
        assert_eq!(None, map.get_slice_mut(3..=5));
    }

    #[test]
    fn backwards_range() {
        // range is invalid
        let mut map = cmap!(3 => 13, 14, 15);
        assert_eq!(None, map.get_slice_mut(4..=3));
    }
}

mod range_from {
    use super::*;

    #[test]
    fn entire_slice() {
        let mut map = cmap!(3 => 13, 14, 15);
        assert_eq!([13, 14, 15], map.get_slice_mut(3..).unwrap());
    }

    #[test]
    fn part_of_slice() {
        let mut map = cmap!(3 => 13, 14, 15);
        assert_eq!([14, 15], map.get_slice_mut(4..).unwrap());
    }

    #[test]
    fn last_element_of_slice() {
        let mut map = cmap!(3 => 13, 14, 15);
        assert_eq!([15], map.get_slice_mut(5..).unwrap());
    }

    #[test]
    fn starts_too_early() {
        let mut map = cmap!(3 => 13, 14, 15);
        assert_eq!(None, map.get_slice_mut(2..));
    }

    #[test]
    fn starts_too_late() {
        let mut map = cmap!(3 => 13, 14, 15);
        assert_eq!(None, map.get_slice_mut(6..));
    }
}

#[test]
fn mutate() {
    let mut map = cmap!(1 => 1, 2, 3);
    let slice = map.get_slice_mut(1..=3).unwrap();
    slice[0] = 4;
    slice[1] = 5;
    slice[2] = 6;
    assert_map_same(&map, [(1, vec![4, 5, 6])]);
}
