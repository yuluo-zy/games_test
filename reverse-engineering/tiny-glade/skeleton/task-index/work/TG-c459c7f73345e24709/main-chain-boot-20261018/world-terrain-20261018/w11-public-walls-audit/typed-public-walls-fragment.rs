// Review/foreign owner handoff only. No production edits or new resource type.
// Integrate into existing rasterize_buildings.rs in place of GroundedSource
// conversion, using that file's canonical GroundedBuildingsSsbo and SKIP_ABOVE.
pub fn grounded_building_from_public_wall(
    wall: &crate::resources::walls::public_wall_state::PublicWallState,
) -> Option<GroundedBuildingsSsbo> {
    // Original Public+280 copies source+58, not plaster peel/source+60.
    // ucomiss followed by JA excludes strictly above; unordered is retained.
    if wall.authority.elevation() > SKIP_ABOVE {
        return None;
    }
    if let Some(circle) = wall.is_circle() {
        let radius = circle.radius.to_bits();
        return Some(GroundedBuildingsSsbo([
            circle.center.x.to_bits(),
            circle.center.y.to_bits(),
            radius,
            radius,
            0,
            0,
            0,
            0,
        ]));
    }
    wall.is_rectangle().map(|rect| {
        GroundedBuildingsSsbo([
            rect.center.x.to_bits(),
            rect.center.y.to_bits(),
            rect.size.x.to_bits(),
            rect.size.y.to_bits(),
            rect.axis.x.to_bits(),
            rect.axis.y.to_bits(),
            1,
            0,
        ])
    })
}
