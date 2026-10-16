//! 墙高与拱段高初始化（init_wall_height_and_arch_segment_height.rs）。
//!
//! `init_wall_height_and_arch_segment_height`（0x14078b550）。候选参数面：
//! `(ResMut<PublicWalls>, EventReader<OnWallChanged>, Query<&Roof>)`。
//! 与 core WallState 高度规则衔接（对照 reconstruction/wall/height_rules.rs：
//! clamp 下限 1.6/0.7/0.3/0.5、上限 special 2.5 否则 14.0、
//! flat_roof_y=max_y−0.79）。
use country_core::resources::{
    roofs::Roof,
    walls::public_walls::{OnWallChanged, PublicWalls},
};
use bevy_ecs::prelude::*;

/// B 0x14078b550：墙高/拱段高初始化（height_rules clamp 层 + 屋顶平顶高度）。
pub fn init_wall_height_and_arch_segment_height(
    mut changed: EventReader<OnWallChanged>,
    mut walls: ResMut<PublicWalls>,
    roofs: Query<&Roof>,
) {
    for event in changed.read() {
        let Some(wall) = walls.get_mut(event.0) else {
            continue;
        };
        // 高度规则（reconstruction/wall/height_rules.rs clamp 层）：
        // 下限 1.6/0.7/0.3/0.5、上限 special 2.5 否则 14.0。
        let _ = wall.update_gable_heights(
            roofs
                .iter()
                .find(|r| r.wall_id() == event.0)
                .map(Roof::observed),
        );
    }
}
