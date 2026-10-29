//! 矩形支柱（construct_elevation_supports/rectangle_pillars.rs）。
//!
//! `construct_rectangle_pillars`（0x14124fcb0）+ `find_stone_column_start_y`
//! （0x141250720）。支柱常量族（re-analyst §5.2：0.2/1.2/0.01/0.5/±0.2/2.1/
//! ±0.25/0.56/1.26）。
//!
//! 2026-10-17 restorer-remaining-wallconstructor：`find_stone_column_start_y`
//! 已按 fresh 机器码（105 条指令 + 跳表 `0x142c20af8` 17 项 + 常量逐字节）
//! 逐语句重建；矩形柱候选阶段已恢复，完整生产者仍未接线。
use country_core::resources::terrain_height::TerrainHeightsData;
use country_core::systems::collision::world::{
    RaycastSimpleResult, RaycastWorld,
};
use glam::{Vec2, Vec3};
use ::utils::scalar::min_f32;
use utils::geometry::rectangle::Rectangle2d;
use utils::swizzlers::xvy;

/// B `find_stone_column_start_y`（0x141250720）：石柱起始高度。
///
/// fresh 机器码逐语句（`evidence/fsy-dis.txt`，105 条指令；跳表
/// `0x142c20af8` 17 项；常量 `0x1429ddd00`/`0x142925904`/`0x142925980`/
/// `0x1429cd030`/`0x14295b630`/`0x1429df16c`/`0x14295a0e4` 逐字节读取）：
///
/// 1. `RaycastWorld::raycast_simple_exclude_wall_id(position, (-0.0, -1.0, -0.0),
///    0x1202, excluded)`（`0x1408ccc50`；XMM3=max f32）。原包装传递 Some(WallId)，
///    与不排除墙的 0x1408ccc20 不同；复用现有 typed 排除方法。
/// 2. `code = hit.handle[0] as u8`：
///    - `0x11`（未命中）：`terrain.sample_at_world_xz(x, z)` 后 `xvy` 写回
///      hit 记录位置（其结果在本函数不再被读取；调用保留）。
///    - `0x01` → `None`。
///    - `0x02` → `Some(hit.position.y + (handle[0] bit8 ? -0.2 : -1.0))`
///      （`TEST byte[handle+1],1`；常量 `0x1429df16c=-0.2`、`0x14295a0e4=-1.0`）。
///    - `0x0d` → `Some(hit.position.y - 0.2)`（跳表 idx13 → `0x141250937`）。
///    - `0x00,0x06..=0x0b,0x0e` → 落入步骤 3。
///    - `0x03,0x04,0x05,0x0c,0x0f,0x10` → panic（跳表 panic 桩）。
/// 3. `axis = rect.is_some() ? rect.axis : (1.0, 0.0)`（`CMP byte[RDI],0`；
///    `RDI` 是 `&Option<Rectangle2d>`：tag u32@0 + payload 24B@4）。
///    `Rectangle2d::new(center=(position.x, position.z), size=(0.7, 0.7), axis)`，
///    `as_points2()` 取 4 角，逐点 `terrain.sample_at_world_xz`；
///    结果 = `min_f32(s0, 1000.0)` → 依次 `min_f32(s1, ·)`, `min_f32(s2, ·)`,
///    `min_f32(s3, ·)`（常量 `0x14295b630=1000.0`）。
/// 4. 返回 `Some(min)`（判别 `EAX=1`；`None` 为 `EAX=0`）。
pub fn find_stone_column_start_y(
    world: &RaycastWorld,
    position: Vec3,
    terrain: &TerrainHeightsData,
    rect: Option<Rectangle2d>,
    excluded: country_core::resources::walls::WallId,
) -> Option<f32> {
    let hit: RaycastSimpleResult = world.raycast_simple_excluding_wall_id(
        position,
        Vec3::new(-0.0, -1.0, -0.0),
        0x1202,
        excluded,
    );
    let x = position.x;
    let z = position.z;
    let code = hit.handle[0] as u8;
    if code == 0x11 {
        // 未命中：地形采样 + xvy 写回 hit 位置（结果随后被轴选择覆盖）。
        let sampled = terrain.sample_at_world_xz(x, z);
        let _ = xvy(Vec2::new(x, z), sampled);
    } else {
        match code {
            0x00 | 0x06..=0x0b | 0x0e => {}
            0x01 => return None,
            0x02 => {
                let add = if hit.handle[0] & 0x100 != 0 {
                    -0.2
                } else {
                    -1.0
                };
                return Some(hit.position.y + add);
            }
            0x0d => return Some(hit.position.y - 0.2),
            // 跳表 panic 桩（unreachable）。
            0x03 | 0x04 | 0x05 | 0x0c | 0x0f | 0x10 => {
                panic!("find_stone_column_start_y: unreachable hit code")
            }
            _ => panic!("find_stone_column_start_y: hit code out of jump table"),
        }
    }
    let axis = match rect {
        None => Vec2::new(1.0, 0.0),
        Some(r) => r.axis,
    };
    let rectangle = Rectangle2d::new(Vec2::new(x, z), Vec2::splat(0.7), axis);
    let p = rectangle.as_points2();
    let s0 = terrain.sample_at_world_xz(p[0].x, p[0].y);
    let s1 = terrain.sample_at_world_xz(p[1].x, p[1].y);
    let s2 = terrain.sample_at_world_xz(p[2].x, p[2].y);
    let s3 = terrain.sample_at_world_xz(p[3].x, p[3].y);
    Some(min_f32(s3, min_f32(s2, min_f32(s1, min_f32(s0, 1000.0)))))
}

/// 原矩形柱候选阶段：四角地形及邻墙门控、三方向排除自身墙射线、
/// 柱起点和原平滑宽度。输出只含 typed 业务字段，不复制原 64B 私有载荷。
pub(crate) fn collect_rectangle_pillars(
    id: country_core::resources::walls::WallId,
    wall: &country_core::resources::walls::public_walls::PublicWallState,
    rectangle: Rectangle2d,
    segments: &[super::find_overlap_segments::OverhangSegment],
    walls: &country_core::resources::walls::public_walls::PublicWalls,
    world: &RaycastWorld,
    terrain: &TerrainHeightsData,
) -> Vec<super::util_pillar_construction::PillarProposal> {
    use super::util_pillar_construction::PillarProposal;
    if wall.authority.removed_supports() != 0 { return Vec::new(); }
    let elevation = wall.authority.elevation();
    let mut proposals = Vec::new();
    for index in 0..4 {
        let point = rectangle.get_point(index);
        if !(terrain.sample_at_world_xz(point.x, point.y) + 0.05 <= elevation) { continue; }
        let mut inside = false;
        for segment in segments {
            let other = walls.get(segment.wall).expect("原矩形柱引用墙必须存在");
            if super::circle_categorize::shape_contains(other.get_wall_shape(), point) { inside = true; break; }
        }
        if inside { continue; }
        let position = Vec3::new(point.x, elevation, point.y);
        if !segments.is_empty() {
            let prev = rectangle.get_point((index + 3) & 3) - point;
            let next = rectangle.get_point((index + 1) & 3) - point;
            let normalize = |v: Vec2| {
                let inverse = 1.0 / (v.y * v.y + v.x * v.x).sqrt();
                Vec2::new(v.x * inverse, v.y * inverse)
            };
            let directions = [normalize(prev), normalize(next), -rectangle.get_corner_normal(index)];
            let mut distance = 10000.0f32;
            for direction in directions {
                let hit = world.legacy_raycast_w_params_excluding_wall_id(position, Vec3::new(direction.x, 0.0, direction.y), 0, id);
                if hit.discriminant == 2 { continue; }
                let tag = hit.handle[0] as u8;
                if (0x6d84u32 >> (u32::from(tag) & 31)) & 1 != 0 { continue; }
                assert_eq!(tag, 0, "原矩形柱遇到未支持的碰撞来源");
                let delta = position - hit.position;
                let candidate = (delta.z * delta.z + delta.x * delta.x + delta.y * delta.y).sqrt();
                distance = if distance.is_nan() { candidate } else if distance <= candidate { distance } else { candidate };
            }
            if !(2.3 <= distance) { continue; }
        }
        let Some(bottom) = find_stone_column_start_y(world, position, terrain, Some(rectangle), id) else { continue };
        let raw = ((elevation - bottom) - 5.0) / 5.0;
        let t = if 0.0 <= raw { raw } else { 0.0 };
        let t = if t <= 1.0 { t } else { 1.0 };
        let t2 = t * t;
        let smooth = (3.0 - (t + t)) * t2;
        let size = smooth * 1.5 + (1.0 - smooth);
        let normal = rectangle.get_corner_normal(index);
        let center = Vec2::new(point.x + normal.x * size * -0.4, point.y + normal.y * size * -0.4);
        let distances = if index & 1 == 0 { [rectangle.size.x, rectangle.size.y] } else { [rectangle.size.y, rectangle.size.x] };
        proposals.push(PillarProposal { center, index, axis: rectangle.axis,
            adjacent_directions: rectangle.get_edge_dirs_from_corner_id(index), bottom, top: elevation,
            size, adjacent_distances: distances });
    }
    proposals
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::walls::WallId;
    use country_core::systems::collision::world::{upsert_collider, RaycastGeoSource, RaycastMeshData};
    use std::{collections::HashMap, sync::Arc};

    #[test]
    fn rectangle_pillar_proposals_match_native_empty_intersections() {
        check_native_proposals(Rectangle2d::new(Vec2::ZERO, Vec2::splat(4.0), Vec2::X),
            include_str!("../../../../../reports/module-recovery-2026-10-23/m06-rectangle-pillar-native-output.json"));
    }

    #[test]
    fn rectangle_pillar_proposals_match_native_rotated_nonsquare() {
        check_native_proposals(Rectangle2d::new(Vec2::new(3.0, -2.0), Vec2::new(4.0, 6.0), Vec2::new(0.6, 0.8)),
            include_str!("../../../../../reports/module-recovery-2026-10-23/m06-rectangle-pillar-rotated-native-output.json"));
    }

    fn check_native_proposals(rectangle: Rectangle2d, fixture: &str) {
        use country_core::resources::{roofs::geometry::surface::Curve2, walls::{InnerWalls, WallState,
            public_walls::{PublicWalls, WallPublicationDirty, WallSpatialHash}, publication_source::TerrainHeightGrid512}};
        use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
        let curve = Curve2::try_new(vec![[0.0,0.0],[4.0,0.0]], false).unwrap();
        let mut inner = InnerWalls::new();
        let id = inner.new_wall(WallState::new(curve, 8.0, 0, 0, false));
        inner.update_elevation(id, 4.0);
        inner.update_wall_state_enclosure(id, ShapeParameters::from_shape(Shape::Rectangle(rectangle)).to_snapshot());
        let grid = TerrainHeightGrid512 { encoded: vec![0;512*512].into(), minimum_y:0.0, maximum_y:0.0, sampling_fix:false };
        let mut walls = PublicWalls::default();
        walls.consume_with_paths(&mut inner, &grid, None, &WallPublicationDirty::default(), &WallSpatialHash::default());
        let mut terrain = TerrainHeightsData::new();
        terrain.set_height_range(0.0, 0.0);
        let actual = collect_rectangle_pillars(id, walls.get(id).unwrap(), rectangle, &[], &walls, &RaycastWorld::new(), &terrain);
        let expected: Vec<u32> = fixture.split(|c: char| !c.is_ascii_digit()).filter(|v| !v.is_empty()).map(|v| v.parse().unwrap()).collect();
        assert_eq!(actual.len(), 4);
        for (i, p) in actual.iter().enumerate() {
            let words = [p.center.x.to_bits(), p.center.y.to_bits(), p.index as u32, 0,
                p.axis.x.to_bits(), p.axis.y.to_bits(), p.adjacent_directions[0].x.to_bits(), p.adjacent_directions[0].y.to_bits(),
                p.adjacent_directions[1].x.to_bits(), p.adjacent_directions[1].y.to_bits(), p.bottom.to_bits(), p.top.to_bits(), p.size.to_bits(),
                p.adjacent_distances[0].to_bits(), p.adjacent_distances[1].to_bits()];
            for (j, value) in words.into_iter().enumerate() { assert_eq!(value, expected[i*16+j], "pillar {i} field {j}"); }
            if rectangle.size.y == 6.0 {
                let fixture = include_str!("../../../../../reports/module-recovery-2026-10-23/m06-rectangle-pillar-rotated-bounds-native-output.json");
                let expected_bounds: Vec<u32> = fixture.split(|c: char| !c.is_ascii_digit()).filter(|v| !v.is_empty()).map(|v| v.parse().unwrap()).collect();
                let bounds = p.stone_column_bounds();
                let words = [bounds.footprint.axis.x, bounds.footprint.axis.y, bounds.footprint.center.x,
                    bounds.footprint.center.y, bounds.footprint.size.x, bounds.footprint.size.y,
                    bounds.bottom, bounds.top];
                for (j, value) in words.into_iter().enumerate() { assert_eq!(value.to_bits(), expected_bounds[i*8+j], "bounds {i} field {j}"); }
            }
        }
    }

    /// 自墙屋顶与别墙屋顶同时落在射线上：只排除目标墙，不能只验证未命中。
    #[test]
    fn column_start_excludes_own_wall_but_hits_other_wall() {
        let own = WallId(77);
        let other = WallId(88);
        let mut world = RaycastWorld::new();
        let mut colliders = HashMap::new();
        for (id, y) in [(own, 5.0), (other, 1.0)] {
            upsert_collider(&mut world, &mut colliders, id,
                RaycastGeoSource::roof(id, false, false),
                Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO)),
                Vec3::Y * y, glam::Quat::IDENTITY);
        }
        world.flush_colliders();
        let terrain = TerrainHeightsData::new();
        let origin = Vec3::Y * 10.0;
        assert_eq!(find_stone_column_start_y(&world, origin, &terrain, None, own), Some(0.5));
        assert_eq!(find_stone_column_start_y(&world, origin, &terrain, None, other), Some(4.5));
    }
}
