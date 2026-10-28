use super::*;

#[test]
fn layout_aligns_each_texture_to_its_copy_alignment() {
    let plan = |total, alignment| TexturePlan {
        copies: Vec::new(),
        total,
        alignment,
    };
    // RGB8 (3-byte texels → 12-byte alignment), BC1 (8), BC7 (16).
    let plans = [plan(30, 12), plan(8, 8), plan(16, 16), plan(5, 12)];
    let (bases, total) = layout(plans.iter()).unwrap();
    assert_eq!(bases, [0, 32, 48, 72]);
    assert_eq!(total, 77);
}
