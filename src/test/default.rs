use super::assert_map_same;

#[test]
fn default() {
    assert_map_same(&Default::default(), []);
}
