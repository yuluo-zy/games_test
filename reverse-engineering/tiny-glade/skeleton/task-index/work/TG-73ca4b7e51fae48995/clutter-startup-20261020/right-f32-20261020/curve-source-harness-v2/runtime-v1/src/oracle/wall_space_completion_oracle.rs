include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/oracle/foundation-completion-2026-10-13/wall-heightmap/wall_tests.rs"
));

#[test]
fn region_axes_use_strict_touching_and_wall_topology() {
    let region = |u, y| WallSpaceRegion {
        center: WallSpaceCoord::new_unchecked(u, y),
        half_extent: Vec2::splat(0.5),
    };
    let mut space = WallSpace {
        is_loop: false,
        length: 10.0,
    };
    assert!(!region(0.2, 1.0).intersects(region(9.8, 1.0), &space));
    space.is_loop = true;
    assert!(region(0.2, 1.0).intersects(region(9.8, 1.0), &space));
    assert!(!region(0.2, 1.0).intersects(region(9.8, 2.0), &space));
    assert!(!region(0.0, 1.0).intersects(region(1.0, 1.0), &space));
    assert_eq!(
        WallSpaceCoord::new_unchecked(0.0, -0.0)
            .offset_x_clamped(&space, 2.0)
            .y
            .to_bits(),
        (-0.0f32).to_bits()
    );
}
