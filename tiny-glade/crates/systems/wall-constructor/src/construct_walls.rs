//! 主墙构造系统（construct_walls.rs）。
//!
//! `construct_walls`（0x140757800）。候选参数面（skeleton 富化）：
//! `((Res<PublicWalls>, Res<AssetSsboLibrary>, Query<&Roof>,
//!   Res<TerrainHeightsData>), ResMut<…>)`。
//!
//! 调用关系铁律（building-structures findings G05 首条）：construct_walls
//! **实调** add_wall_corners（0x141215540）/add_skirts（0x1412221f0）/
//! stairs_signifiers（0x141223d10），**不** CALL construct_arches/
//! construct_lintels（二者是独立墙事件消费者）。A8 参数实传 0。
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::{
    roofs::Roof,
    terrain_height::TerrainHeightsData,
    walls::public_walls::{OnWallChanged, PublicWalls},
};
use bevy_ecs::prelude::*;

/// B 0x140757800：消费墙变更事件 → from_curve 链 → 96B 砖实例 + 角/裙边/指示符。
pub fn construct_walls(
    mut changed: EventReader<OnWallChanged>,
    walls: Res<PublicWalls>,
    ssbo: Res<AssetSsboLibrary>,
    roofs: Query<&Roof>,
    _terrain: Res<TerrainHeightsData>,
) {
    for event in changed.read() {
        let Some(wall) = walls.get(event.0) else {
            continue;
        };
        let curve_points: Vec<glam::Vec2> = wall
            .simplified_curve_2d()
            .points
            .iter()
            .map(|p| glam::Vec2::new(p[0], p[1]))
            .collect();
        let has_roof = walls
            .roof_entity(event.0)
            .and_then(|entity| roofs.get(entity).ok())
            .is_some();
        let material = if has_roof { 2 } else { 1 };
        // from_curve 安全闭包：resolved rows/holes → 96B 实例（中段桥登记）。
        let _records = crate::wall_constructor::from_curve_resolved(
            &curve_points,
            material,
            event.0 .0,
            &[],
            wall.simplified_curve_2d().length,
            1.0,
            false,
            event.0 .0,
            false,
        );
        // 角/裙边/指示符（G05 实调：add_wall_corners 0x141215540 /
        // add_skirts 0x1412221f0 / stairs_signifiers 0x141223d10）。
        let points3: Vec<glam::Vec3> = curve_points
            .iter()
            .map(|p| glam::Vec3::new(p.x, p.y, 0.0))
            .collect();
        let mut corner_records = Vec::new();
        crate::utils::wall_corners::add_wall_corners_new(&points3, &mut corner_records);
        let mut skirt_records = Vec::new();
        crate::utils::wall_skirts::add_skirts_new(&points3, &mut skirt_records);
        let mut signifier_records = Vec::new();
        crate::utils::stairs_signifiers::stairs_signifiers_new(
            event.0 .0,
            &points3,
            &mut signifier_records,
        );
        let _ = ssbo;
    }
}
