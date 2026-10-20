//! B14078b550 relationship portion of wall height initialization.
//! Prepared base/third caches belong to the actual PublicWallState path owner.
use bevy_ecs::prelude::*;
use country_core::resources::{
    roofs::Roof,
    walls::public_walls::{OnWallChanged, PublicWalls},
};

pub fn init_wall_height_and_arch_segment_height(
    mut changed: EventReader<OnWallChanged>,
    mut walls: ResMut<PublicWalls>,
    roofs: Query<&Roof>,
) {
    for event in changed.read() {
        // Native PublicWalls table absence is unwrap_failed, not silent skip.
        let wall = walls
            .get(event.0)
            .expect("wall height event must reference a published wall");
        // Native query is reached only for a rectangle. A present association
        // must resolve the same generation; another roof with equal WallId
        // cannot replace a stale/missing associated component.
        let roof = if wall.is_rectangle().is_some() {
            walls
                .roof_entity(event.0)
                .filter(|entity| entity.to_bits() >> 32 != 0)
                .map(|entity| {
                    roofs
                        .get(entity)
                        .expect("associated rectangle roof component must resolve")
                })
        } else {
            None
        };
        walls
            .get_mut(event.0)
            .unwrap()
            .update_gable_heights(roof.map(Roof::observed))
            .expect("invalid authoritative gable height input");
    }
}
