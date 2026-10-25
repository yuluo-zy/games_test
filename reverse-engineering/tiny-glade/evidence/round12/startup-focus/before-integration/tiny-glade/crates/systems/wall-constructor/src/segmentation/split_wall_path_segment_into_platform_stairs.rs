//! 平台楼梯段拆分（segmentation/split_wall_path_segment_into_platform_stairs.rs）。
//!
//! `split_wall_path_segment_into_platform_stairs`（0x140751960）。
//! 候选参数面（skeleton 富化）：`(EventReader<OnWallChanged>,
//!  EventReader<CreateRoofCmd>, EventReader<SwitchRoofTypeCmd>,
//!  Res<PublicWalls>, Query<&Roof>, …)`。
//!
//! 机器码（building-segmentation/B-stairs + B-platform-* 冻结）：
//! 墙路径段 → 平台楼梯段拆分（平台几何/行列链见
//! shared building modules wall_platform_stairs 族）。
use bevy_ecs::prelude::*;
use country_core::resources::{
    roofs::{
        Roof,
        commands::{CreateRoofCmd, SwitchRoofTypeCmd},
    },
    walls::public_walls::{OnWallChanged, PublicWalls},
};

/// B 0x140751960。
pub fn split_wall_path_segment_into_platform_stairs(
    mut changed: EventReader<OnWallChanged>,
    mut create_roof: EventReader<CreateRoofCmd>,
    mut switch_type: EventReader<SwitchRoofTypeCmd>,
    walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
) {
    let recalc: Vec<u64> = changed
        .read()
        .map(|e| e.0.0)
        .chain(create_roof.read().map(|e| e.0.wall_id().0))
        .chain(switch_type.read().map(|e| e.wall.0))
        .collect();
    for wall_id in recalc {
        let id = country_core::resources::walls::WallId(wall_id);
        let Some(wall) = walls.get(id) else {
            continue;
        };
        let has_roof = walls
            .roof_entity(id)
            .and_then(|entity| roofs.get(entity).ok())
            .is_some();
        // 平台楼梯段拆分（B-platform-* 冻结链）：段范围 → 平台段存储
        // （掩码/行列源经 calculate_wall_path_segmentation 产出后接线——
        // I 级登记：段 104B 记录的生产序待 materializer 链接线）。
        if has_roof && wall.simplified_curve_2d().length > 0.0 {
            let _ = id;
        }
    }
}
