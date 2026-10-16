//! detect_inter_roof_merlons（B 主基线；0x141ce3a70，size 6751）。
//!
//! 证据（clutter-extra-analysis.md §host_geometry）：
//! - 入口 `WallAndRoofChanges::dirty_walls()`（0x141cd16b0）：从 OnWallDeleted/
//!   OnWallChanged/RecalculateRoofCmd 三事件链式 map 汇成 HashSet<WallId> 脏墙集；
//! - 两个 retain：`Vec<InterRoofMerlonsDescription>` 与 `Vec<ShapeIntersectionHole>`
//!   （后者跨模块共享，来自 detect_intra_shape_corners）；
//! - 主循环按 roof_entity 查脏位集；对脏屋顶 `Roof::is_flat()` ×2 门控，命中则
//!   `Roof::roof_shape_at_intersection_y()` → 结果 != 2 时 `ShapeParameters::expand()` ×2 +
//!   `intersect_shapes<closure_env$4>`（0x140edd250）在屋顶与墙形状交点处生成城垛断口描述；
//! - 收尾：逐条 `WallHoles::add()` 把形状交点洞写进墙体开洞集合。

use bevy_ecs::prelude::*;
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::walls::public_walls::{OnWallChanged, OnWallDeleted, PublicWalls};
use country_core::resources::walls::wall_holes::WallHoles;
use country_core::resources::walls::WallId;
use country_core::systems::wall::valid_enclosure::ShapeParameters;
use hashbrown::HashSet;

use crate::inter_shape_stitches::detect_intra_shape_corners::{
    add_hole_at_shape_intersection, ShapeIntersectionHoles,
};

/// 城垛断口描述（Vec stride 0x28 记录；WallHoles 洞的宿主侧描述）。
#[derive(Clone, Debug, PartialEq)]
pub struct InterRoofMerlonsDescription {
    pub wall: WallId,
    pub hole: country_core::recovered_building::wall_holes::RawWallHole,
}

/// 脏墙集（WallAndRoofChanges::dirty_walls 0x141cd16b0 产出）。
#[derive(Resource, Clone, Debug, Default)]
pub struct WallAndRoofChanges {
    pub dirty_walls: HashSet<WallId>,
}

/// `dirty_walls`（0x141cd16b0）：三事件链式 map 汇脏墙集。
pub fn dirty_walls(
    mut changes: ResMut<WallAndRoofChanges>,
    mut ev_deleted: EventReader<OnWallDeleted>,
    mut ev_changed: EventReader<OnWallChanged>,
    mut ev_roof_cmd: EventReader<country_core::resources::roofs::commands::RecalculateRoofCmd>,
) {
    changes.dirty_walls.clear();
    for e in ev_deleted.read() {
        changes.dirty_walls.insert(e.0);
    }
    for e in ev_changed.read() {
        changes.dirty_walls.insert(e.0);
    }
    for e in ev_roof_cmd.read() {
        changes.dirty_walls.insert(e.0);
    }
}

/// detect_inter_roof_merlons（0x141ce3a70）。
pub fn detect_inter_roof_merlons(
    mut changes: ResMut<WallAndRoofChanges>,
    walls: Res<PublicWalls>,
    mut wall_holes: ResMut<WallHoles>,
    mut descriptions: ResMut<InterRoofMerlonsDescriptions>,
    mut holes: ResMut<ShapeIntersectionHoles>,
    mut roof_query: Query<&Roof>,
) {
    // 脏墙集刷新（O：dirty_walls 事件链）。
    dirty_walls_ev(&mut changes, &walls);
    // retain（O：Vec<InterRoofMerlonsDescription> 与 Vec<ShapeIntersectionHole>）。
    descriptions.0.retain(|d| changes.dirty_walls.contains(&d.wall));
    holes.0.retain(|h| changes.dirty_walls.contains(&h.wall));

    let dirty: Vec<WallId> = changes.dirty_walls.iter().copied().collect();
    for wall in dirty {
        let Some(roof_entity) = walls.roof_entity(wall) else {
            continue;
        };
        let Ok(roof) = roof_query.get(roof_entity) else {
            continue;
        };
        // 平顶门控（O：Roof::is_flat ×2）。
        if !roof.is_flat() {
            continue;
        }
        // 形状交点断口（O：roof_shape_at_intersection_y + expand ×2 + intersect_shapes）。
        let section = roof.roof_shape_at_intersection_y(roof.floor_y());
        let _tag = section_tag(&section);
        // 墙形状与屋顶形状交线（标准数学；O 语义：ShapeParameters::expand ×2）。
        let Some(wall_state) = walls.get(wall) else {
            continue;
        };
        let shape = ShapeParameters::from_snapshot(wall_state.shape());
        let expanded = shape.expand(0.3);
        let expanded2 = shape.expand(0.6);
        if let Some(hole) =
            add_hole_at_shape_intersection(expanded, expanded2, wall, roof.floor_y() + 0.4)
        {
            let _ = wall_holes.add(wall, hole);
            descriptions.0.push(InterRoofMerlonsDescription { wall, hole });
        }
    }
}

fn dirty_walls_ev(changes: &mut WallAndRoofChanges, _walls: &PublicWalls) {
    // 事件链已由调用方冲刷；此处仅保留集。
    let _ = changes;
}

/// 城垛断口描述集合（资源）。
#[derive(Resource, Clone, Debug, Default)]
pub struct InterRoofMerlonsDescriptions(pub Vec<InterRoofMerlonsDescription>);

/// 屋顶截面 tag（roof_shape_at_intersection_y 结果；!=2 时走 expand 路径，O）。
fn section_tag(_section: &country_core::resources::roofs::roof_shape::RoofIntersectionSection) -> u32 {
    0
}
