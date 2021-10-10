use super::assert_map_same;
use crate::ContiguousMap;

#[test]
fn new() {
    assert_map_same(&ContiguousMap::new(), []);
}
