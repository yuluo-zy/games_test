//! 屋顶类型切换（visual/switch_roof_type.rs）。
//!
//! `switch_roof_type`（0x1421ac4f0）。候选参数面（skeleton 富化）：
//! `(EventReader<SwitchRoofTypeCmd>, Commands, Res<AssetMeshLibrary>,
//!  EventWriter<RecalculateWallBricks>, EventWriter<…>)`。
//!
//! 机器码（building-edits / editing.rs 既有实现链）：消费
//! `SwitchRoofTypeCmd`（edit_roof 在高度穿越时发出）；平顶→坡顶切换的
//! 游戏侧效果：RoofSound（kind 0/6 + 平顶 style@0x54）、gable 通知
//! （ridge==1 也通知）、非平顶 source_flag==1 时墙高夹取
//! `max(1.6, min(14.0, height))` 写回 InnerWalls；网格切换
//! （AssetMeshLibrary drawable/VAO）为渲染边界后置登记。
use country_core::resources::{
    roofs::{Roof, commands::*},
    walls::InnerWalls,
};
use bevy_ecs::prelude::*;

/// B 0x1421ac4f0。
pub fn switch_roof_type(
    mut requests: EventReader<SwitchRoofTypeCmd>,
    roofs: Query<&Roof>,
    mut walls: ResMut<InnerWalls>,
    mut gable: EventWriter<WallGableChanged>,
    mut sounds: EventWriter<RoofSound>,
    mut bricks: EventWriter<RecalculateWallBricks>,
) {
    use country_core::resources::roofs::geometry::edit_rules::HeightCrossing;
    for request in requests.read() {
        let roof = roofs
            .get(request.entity)
            .expect("roof type change requires its entity");
        let pitched = roof.is_pitched();
        assert_eq!(
            pitched,
            request.crossing == HeightCrossing::Up,
            "roof type event disagrees with Roof::ty"
        );
        sounds.write(RoofSound {
            entity: request.entity,
            wall: request.wall,
            pivot: roof.pivot_3d().to_array(),
            kind: if pitched { 0 } else { 6 },
            flat_surface: (!pitched).then_some(roof.observed()[0x54]),
        });
        // Flat transitions also notify when ridge==1; is_gable() would suppress
        // that notification because the height has already become flat.
        if f32::from_le_bytes(roof.observed()[0x34..0x38].try_into().unwrap()) == 1.0 {
            gable.write(WallGableChanged(roof.wall_id()));
        }
        if !pitched && request.source_flag == 1 {
            let height = walls.get_unwrapped(request.wall).height();
            if height != 1.6 {
                // 1421acc7e..8a: MAXSS(1.6,height), MINSS(14,result).
                let minimum = if 1.6 > height { 1.6 } else { height };
                let bounded = if 14.0 < minimum { 14.0 } else { minimum };
                walls.update_wall_state_height(request.wall, bounded);
            }
        }
        bricks.write(RecalculateWallBricks(request.wall));
    }
}
