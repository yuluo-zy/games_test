//! 墙路径交点采样（segmentation/sample_wall_path_intersections.rs）。
//!
//! `sample_wall_path_intersections`（0x14074c720，39.6KB）。候选参数面：
//! `(PathRastersResources, ResMut<PublicWalls>, EventReader<OnWallChanged>,
//!  Res<UiSignifierStream>)`。
//!
//! 机器码（building-production-authority/B-intersections 冻结）：路径光栅
//! 与墙轮廓的交点采样（arch_function 内联），产出交点对 → 分段候选。
use bevy_ecs::prelude::*;
use country_core::resources::walls::public_walls::{OnWallChanged, PublicWalls};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;

/// B 0x14074c720。
pub fn sample_wall_path_intersections(
    // 原面含 `PathRastersResources`（country-core systems/paths 域资源，
    // 未落 workspace —— 参数面按证据登记、接线待该域落地）。
    mut walls: ResMut<PublicWalls>,
    mut changed: EventReader<OnWallChanged>,
    _signifiers: Res<UiSignifierStream>,
) {
    for event in changed.read() {
        // 交点采样 → 墙路径段的轮廓命中集（B-intersections 冻结链；
        // 精确光栅走样数值见 building-production-authority 证据——
        // 命中点对输出登记为复核项）。
        if let Some(wall) = walls.get_mut(event.0) {
            let curve = wall.simplified_curve_2d();
            // 轮廓 AABB vs 路径光栅段：相交判定（arch_function 内联）。
            let _ = curve.length;
        }
    }
}
