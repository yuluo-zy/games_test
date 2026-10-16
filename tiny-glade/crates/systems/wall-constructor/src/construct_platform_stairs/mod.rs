//! 平台楼梯构造（construct_platform_stairs/mod.rs）。
//!
//! `construct_platform_stairs`（0x1407744d0）。候选参数面（skeleton 富化）：
//! `(ResMut<WallHoles>, ResMut<PlatformStairsStorage>, Res<PublicWalls>,
//!  Res<UiSignifierStream>, Res<AssetSsboLibrary>, …)`。
//!
//! 机器码（building-platform-stairs B-dirty-tiles/B-collision 冻结）：
//! 平台楼梯构造 + 碰撞几何（rapier 边界按 building-platform-stairs 结论）；
//! CPU 链 = recovered_building wall_platform_stairs 族
//! （collect_platform_stair_blockers/prepare_platform_stair_segments/
//! platform_stair_cell/platform_stair_geometry/platform_stair_holes）。
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::{
    walls::{public_walls::PublicWalls, wall_holes::WallHoles},
};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use bevy_ecs::prelude::*;

/// B `system_wall_constructor::PlatformStairsStorage`（跨帧平台段存储）。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct PlatformStairsStorage {
    /// 墙 id → 平台段记录（布局 I）。
    pub segments: hashbrown::HashMap<u64, Vec<[u8; 104]>>,
}

/// B 0x1407744d0。
pub fn construct_platform_stairs(
    mut holes: ResMut<WallHoles>,
    mut storage: ResMut<PlatformStairsStorage>,
    walls: Res<PublicWalls>,
    _signifiers: Res<UiSignifierStream>,
    ssbo: Res<AssetSsboLibrary>,
) {
    for (wall_id, segments) in storage.segments.iter_mut() {
        let id = country_core::resources::walls::WallId(*wall_id);
        let Some(wall) = walls.get(id) else {
            continue;
        };
        // 冻结链：platform_stair_geometry/holes（碰撞几何 rapier 边界后置；
        // 段源 = split_wall_path_segment_into_platform_stairs 产出）。
        let _ = (wall.simplified_curve_2d().length, &mut *holes, &ssbo, segments);
    }
}
