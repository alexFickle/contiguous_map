use crate::cmap;

#[test]
fn must_compile() {
    // just checking that debug printing compiles
    let map = cmap!(1 => 1, 2, 3);
    let _ = format!("{:?}", map);
}
