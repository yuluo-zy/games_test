//! 平台楼梯构造（construct_platform_stairs/mod.rs）。
//!
//! `construct_platform_stairs`（0x1407744d0）。候选参数面（skeleton 富化）：
//! `(ResMut<WallHoles>, ResMut<PlatformStairsStorage>, Res<PublicWalls>,
//!  Res<UiSignifierStream>, Res<AssetSsboLibrary>, …)`。
//!
//! 机器码（building-platform-stairs B-dirty-tiles/B-collision 冻结）：
//! 平台楼梯构造 + 碰撞几何（rapier 边界按 building-platform-stairs 结论）；
//! CPU 链 = shared building modules wall_platform_stairs 族
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

/// B 0x1407744d0。**未实现（零壳）**：真实逻辑为 80B 装饰源
/// （raw[0x4e]==0x3f 且 raw[0x2a]∉{2,3,4}）→ blockers（coordinate=[f(8),f(12)]、
/// terrain 差<3.0）→ process_loop_range_into_wallpath_segments → cells/碰撞
/// （update_phys/add_colision_geo）→ 96B 砖/holes/stats。现有实现仅遍历
/// storage 不产输出。
pub fn construct_platform_stairs(
    _holes: ResMut<WallHoles>,
    _storage: ResMut<PlatformStairsStorage>,
    _walls: Res<PublicWalls>,
    _signifiers: Res<UiSignifierStream>,
    _ssbo: Res<AssetSsboLibrary>,
) {
    todo!("未实现: construct_platform_stairs 0x1407744d0")
}
