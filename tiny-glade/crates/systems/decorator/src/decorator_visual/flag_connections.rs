//! B 1407e1430 / 4312B and 1407e2510 / 2604B business adapters.
//!
//! The real ClothInfo caller uses indexed stable greedy connections and actual
//! thread Commands into the existing transient owner. FlagAnchor is only a
//! semantic geometry projection, not native CandidateInfo's declaration/ABI.
//! The unrelated general-flag producer/styled thread remain unrecovered.
use crate::{
    decorator_visual_state::DecoratorTransientVisualState,
    utils::visual_entity_builder::WindowVisualEntityBuilder,
};
use bevy_ecs::prelude::*;
use country_core::{
    asset_libraries::{mesh_library::AssetMeshLibrary, shader_library::AssetShaderLibrary},
    resources::walls::decorator_storage::DecoratorStorage,
};
use glam::{Mat3, Quat, Vec2, Vec3};
use std::collections::HashSet;

pub const CONNECT_MAX_PER_NODE: usize = 2;
pub const CONNECT_ALLOW_SAME_WALL: bool = true;
pub const CONNECT_NEAR_DIFFERENT_WALL: f32 = 10.0;
pub const CONNECT_FAR_SAME_WALL: f32 = 5.5;
pub const CONNECT_MIN_SAME_WALL: f32 = 2.0;
pub const CONNECT_MIN_DISTANCE: f32 = 0.02;
pub const CONNECT_OCCLUSION_MARGIN: f32 = -0.01;
pub const CONNECT_EDGE_WEIGHT_BIAS: f32 = 3.0;
pub const CLOTH_SAG: f32 = 0.08;
pub const CLOTH_MAX_SAG_FRACTION: f32 = 0.25;
pub const CLOTH_SEGMENT_WIDTH: f32 = 1.0;
pub const CLOTH_MESH_THREAD: &str = "decorators/setdressing_clothline_thread";
pub const CLOTH_MESH_FLAG_LINE: &str = "decorators/flag_line";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlagAnchor {
    pub id: u32,
    pub wall: u64,
    pub pos: Vec3,
    pub source: bool,
    pub target: bool,
}

/// Five original business inputs; ray margin, distance floor and bias are fixed.
#[derive(Clone, Copy, Debug)]
pub struct ConnectParams {
    pub max_per_node: usize,
    pub allow_same_wall: bool,
    pub near_different_wall: f32,
    pub far_same_wall: f32,
    pub min_same_wall: f32,
}
impl Default for ConnectParams {
    /// Separate general-flag caller's values; ClothInfo supplies its own five.
    fn default() -> Self {
        Self {
            max_per_node: CONNECT_MAX_PER_NODE,
            allow_same_wall: CONNECT_ALLOW_SAME_WALL,
            near_different_wall: CONNECT_NEAR_DIFFERENT_WALL,
            far_same_wall: CONNECT_FAR_SAME_WALL,
            min_same_wall: CONNECT_MIN_SAME_WALL,
        }
    }
}

/// Indices always refer to the caller's original slice, including duplicate IDs.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConnectResult {
    pub edges: Vec<(usize, usize)>,
    pub degrees: Vec<usize>,
    pub sorted_indices: Vec<usize>,
}

pub fn candidates_within_window(a: &FlagAnchor, b: &FlagAnchor, p: &ConnectParams) -> Option<f32> {
    if !(a.source && b.target) || (!p.allow_same_wall && a.wall == b.wall) {
        return None;
    }
    let d = a.pos - b.pos;
    // 1407e1853..1890: Y square + X square, then Z square.
    let d2 = (d.y * d.y + d.x * d.x) + d.z * d.z;
    let ok = if a.wall != b.wall {
        d2 < p.near_different_wall * p.near_different_wall
    } else {
        p.min_same_wall * p.min_same_wall < d2 && d2 < p.far_same_wall * p.far_same_wall
    };
    ok.then(|| d2.sqrt())
}

/// Stable ID order, one stable weight-sorted neighbor list per source; no global
/// edge sort. `blocked` receives real origin, normalized direction and max_toi.
/// Production supplies actual RaycastWorld; test callbacks are not scene inputs.
pub fn greedily_connect(
    anchors: &[FlagAnchor],
    p: &ConnectParams,
    blocked: impl Fn(Vec3, Vec3, f32) -> bool,
) -> ConnectResult {
    let mut order: Vec<_> = (0..anchors.len()).collect();
    order.sort_by_key(|&i| anchors[i].id);
    let mut neighbors = Vec::with_capacity(order.len());
    for (i, &a) in order.iter().enumerate() {
        let mut next = Vec::new();
        for (j, &b) in order.iter().enumerate().skip(i + 1) {
            if let Some(distance) = candidates_within_window(&anchors[a], &anchors[b], p) {
                let weight = if p.allow_same_wall && anchors[a].wall == anchors[b].wall {
                    distance + CONNECT_EDGE_WEIGHT_BIAS
                } else {
                    distance
                };
                next.push((j, weight));
            }
        }
        next.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        neighbors.push(next);
    }
    let mut degrees = vec![0; anchors.len()];
    let mut accepted = HashSet::new();
    let mut edges = Vec::new();
    for (i, next) in neighbors.iter().enumerate() {
        for &(j, _) in next {
            if degrees[i] >= p.max_per_node
                || degrees[j] >= p.max_per_node
                || accepted.contains(&(i, j))
            {
                continue;
            }
            let a = anchors[order[i]].pos;
            let delta = anchors[order[j]].pos - a;
            let horizontal = (delta.z * delta.z + delta.x * delta.x).sqrt();
            if delta.y.abs() + delta.y.abs() > horizontal {
                continue;
            }
            let distance = (delta.z * delta.z + (delta.y * delta.y + delta.x * delta.x)).sqrt();
            if !(distance > CONNECT_MIN_DISTANCE) {
                continue;
            }
            let direction = delta * (1.0 / distance);
            if blocked(a, direction, distance + CONNECT_OCCLUSION_MARGIN) {
                continue;
            }
            accepted.insert((i, j));
            edges.push((order[i], order[j]));
            degrees[i] += 1;
            degrees[j] += 1;
        }
    }
    let mut original_degrees = vec![0; anchors.len()];
    for (i, &original) in order.iter().enumerate() {
        original_degrees[original] = degrees[i];
    }
    ConnectResult {
        edges,
        degrees: original_degrees,
        sorted_indices: order,
    }
}

/// Original ceilf and saturating conversion. The actual producer iterates lazily.
pub fn clothline_sample_count(distance: f32, spacing: f32) -> usize {
    ((distance / spacing).ceil() as usize).max(2)
}
pub fn clothline_sag_fraction(sag: f32, distance: f32) -> f32 {
    (sag / distance).clamp(0.0, CLOTH_MAX_SAG_FRACTION)
}
pub fn cloth_sag_offset(t: f32, distance: f32) -> f32 {
    let u = t + t - 1.0;
    ((1.0 - u * u) * CLOTH_SAG) * distance
}
fn thread_point(a: Vec3, b: Vec3, blend: f32, distance: f32, index: usize, count: usize) -> Vec3 {
    let t = index as f32 / count as f32;
    let u = ((1.0 - t) * blend) + t;
    let sag = cloth_sag_offset(u, distance);
    Vec3::new(
        (u * b.x + (1.0 - u) * a.x) - sag * 0.0,
        (u * b.y + (1.0 - u) * a.y) - sag,
        (b.z * u + a.z * (1.0 - u)) - sag * 0.0,
    )
}

/// Engineering context for the inspected absent-style branch; all owners are
/// the actual Commands/assets/global transient values supplied by its caller.
/// It has no styled/Some API that could silently pretend that branch is None.
pub struct ClothlineContext<'a, 'w, 's> {
    pub commands: &'a mut Commands<'w, 's>,
    pub meshes: &'a AssetMeshLibrary,
    pub shaders: &'a AssetShaderLibrary,
    pub transient: &'a mut DecoratorTransientVisualState,
}

/// Original absent-style helper now builds each real thread entity. No quad is
/// returned or discarded. This exact caller requests stretch=true, width=1.
pub fn clothline(
    context: &mut ClothlineContext<'_, '_, '_>,
    id: u32,
    a: Vec3,
    b: Vec3,
    spacing: f32,
    sag: f32,
    width: f32,
) {
    let delta = b - a;
    let distance = delta.length();
    let count = clothline_sample_count(distance, spacing);
    let blend = clothline_sag_fraction(sag, distance);
    let mut previous = thread_point(a, b, blend, distance, 0, count);
    for i in 0..count {
        let next = thread_point(a, b, blend, distance, i + 1, count);
        let delta = next - previous;
        let segment_length = delta.length();
        let forward = delta * (1.0 / segment_length);
        // 1407e2a6f..2e0a signed zero products are actual business axis values.
        let right = Vec3::new(
            forward.z + forward.y * (-0.0),
            forward.x * 0.0 + forward.z * (-0.0),
            -forward.x - forward.y * (-0.0),
        )
        .normalize();
        let up = forward.cross(right).normalize();
        let rotation = Quat::from_mat3(&Mat3::from_cols(right, up, forward));
        let mut builder = WindowVisualEntityBuilder::new(previous, rotation, id);
        builder.scale = Vec3::new(width, width, segment_length);
        builder.mesh_name = CLOTH_MESH_THREAD;
        builder.nani_name = "nani_solid_vertex_color";
        builder.build(
            context.commands,
            id,
            context.meshes,
            context.shaders,
            context.transient,
        );
        previous = next;
    }
}

// Legacy pure quad utilities below are retained for the separate incomplete
// general-flag domain only. The actual cloth system uses real Commands/clothline.
/// Legacy pure quad sampling for the separate incomplete flag route. This is
/// not the recovered original thread producer; actual ClothInfo uses clothline.
pub fn clothline_points(a: Vec3, b: Vec3, spacing: f32, sag_len: f32, sag_scale: f32) -> Vec<Vec3> {
    let dist = (b - a).length();
    let n = clothline_sample_count(dist, spacing);
    let blend = clothline_sag_fraction(sag_len, dist) * sag_scale;
    let mut points = Vec::with_capacity(n + 1);
    for i in 0..=n {
        let t = i as f32 / n as f32;
        let t = t + (1.0 - t) * blend;
        let mut p = a.lerp(b, t);
        p.y -= cloth_sag_offset(t, dist);
        points.push(p);
    }
    points
}

/// Legacy quad approximation retained outside the actual ClothInfo chain.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClothQuad {
    pub corners: [Vec3; 4],
}

/// 逐段布条四角（段数 = 采样点数 - 1）。
pub fn clothline_quads(points: &[Vec3], width: f32) -> Vec<ClothQuad> {
    let half = width * 0.5;
    let mut quads = Vec::with_capacity(points.len().saturating_sub(1));
    for pair in points.windows(2) {
        let (p0, p1) = (pair[0], pair[1]);
        let dir = Vec2::new(p1.x - p0.x, p1.z - p0.z).normalize_or(Vec2::Y);
        let perp = Vec2::new(-dir.y, dir.x) * half;
        quads.push(ClothQuad {
            corners: [
                Vec3::new(p0.x + perp.x, p0.y, p0.z + perp.y),
                Vec3::new(p0.x - perp.x, p0.y, p0.z - perp.y),
                Vec3::new(p1.x + perp.x, p1.y, p1.z + perp.y),
                Vec3::new(p1.x - perp.x, p1.y, p1.z - perp.y),
            ],
        });
    }
    quads
}

/// Legacy incomplete general-flag projection, outside the recovered ClothInfo chain.
/// Its upstream wall-side tags/style and real output are not restored; do not
/// register this as evidence of a working flag producer. It still only derives quads.
///
/// 锚点从已放置旗装饰读取 `DecoratorExtraInfo::try_as_flag`（O callee）与缓存
/// 变换位置。原函数还用 FUN_1407df050 做墙侧相邻查询（U，锚点来源由调用者
/// 提供的候选集承载）。遮挡查询在世界系统内（U：RaycastWorld），此处以
/// “无遮挡”回调承载并在交付登记。
pub fn generate_flag_connections(
    decorator_storage: Res<DecoratorStorage>,
    cached: Res<
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms,
    >,
    mut transient: ResMut<DecoratorTransientVisualState>,
) {
    let mut anchors: Vec<FlagAnchor> = Vec::new();
    for (wall, decorators) in decorator_storage.iter_walls() {
        for (id, info) in decorators.iter_modified() {
            if info.extra().try_as_flag().is_none() {
                continue;
            }
            let pos = cached.position(id);
            anchors.push(FlagAnchor {
                id,
                wall: wall.0,
                pos,
                // O：0x50B 候选的 +0x4d 由墙侧查询填充；U 源/目标标记来源。
                source: true,
                target: true,
            });
        }
    }
    let params = ConnectParams::default();
    let result = greedily_connect(&anchors, &params, |_a, _b, _len| false);
    for &(a_index, b_index) in &result.edges {
        let a = &anchors[a_index];
        let b = &anchors[b_index];
        // O：两次布料调用（0x142a88b30 len 0x27 / 0x142a88b57 len 0x14）。
        let points = clothline_points(a.pos, b.pos, 0.3, 0.0, 0.0);
        let _quads = clothline_quads(&points, CLOTH_SEGMENT_WIDTH);
    }
    let _ = &mut transient;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn anchor(id: u32, wall: u64, pos: Vec3) -> FlagAnchor {
        FlagAnchor {
            id,
            wall,
            pos,
            source: true,
            target: true,
        }
    }

    #[test]
    fn distance_windows_match_machine_constants() {
        let p = ConnectParams::default();
        let a = anchor(1, 0, Vec3::ZERO);
        // 异墙 < 10
        let b = anchor(2, 1, Vec3::new(9.9, 0.0, 0.0));
        assert!(candidates_within_window(&a, &b, &p).is_some());
        let b = anchor(2, 1, Vec3::new(10.1, 0.0, 0.0));
        assert!(candidates_within_window(&a, &b, &p).is_none());
        // 同墙 2..5.5
        let b = anchor(2, 0, Vec3::new(1.9, 0.0, 0.0));
        assert!(candidates_within_window(&a, &b, &p).is_none());
        let b = anchor(2, 0, Vec3::new(2.1, 0.0, 0.0));
        assert!(candidates_within_window(&a, &b, &p).is_some());
        let b = anchor(2, 0, Vec3::new(5.6, 0.0, 0.0));
        assert!(candidates_within_window(&a, &b, &p).is_none());
    }

    #[test]
    fn source_target_flags_gate_pairs() {
        let p = ConnectParams::default();
        let mut a = anchor(1, 0, Vec3::ZERO);
        let mut b = anchor(2, 1, Vec3::X);
        a.source = false;
        assert!(candidates_within_window(&a, &b, &p).is_none());
        a.source = true;
        b.target = false;
        assert!(candidates_within_window(&a, &b, &p).is_none());
    }

    #[test]
    fn greedy_respects_capacity_two() {
        // 三个互相靠近的点：中心节点最多连 2 条。
        let anchors = vec![
            anchor(1, 1, Vec3::ZERO),
            anchor(2, 0, Vec3::new(3.0, 0.0, 0.0)),
            anchor(3, 0, Vec3::new(-3.0, 0.0, 0.0)),
        ];
        let r = greedily_connect(&anchors, &ConnectParams::default(), |_, _, _| false);
        assert_eq!(r.degrees[0], 2);
        assert_eq!(r.edges.len(), 2);
    }

    #[test]
    fn greedy_slope_gate_rejects_steep_edge() {
        let anchors = vec![
            anchor(1, 0, Vec3::ZERO),
            anchor(2, 0, Vec3::new(3.0, 10.0, 0.0)),
        ];
        let r = greedily_connect(&anchors, &ConnectParams::default(), |_, _, _| false);
        assert!(r.edges.is_empty(), "2*|dy| > horizontal");
    }

    #[test]
    fn greedy_occlusion_rejects_edge() {
        let anchors = vec![
            anchor(1, 0, Vec3::ZERO),
            anchor(2, 0, Vec3::new(3.0, 0.0, 0.0)),
        ];
        let r = greedily_connect(&anchors, &ConnectParams::default(), |_, _, _| true);
        assert!(r.edges.is_empty());
    }

    #[test]
    fn greedy_min_distance_rejects_close_pair() {
        let anchors = vec![
            anchor(1, 1, Vec3::ZERO),
            anchor(2, 0, Vec3::new(0.01, 0.0, 0.0)),
        ];
        let r = greedily_connect(&anchors, &ConnectParams::default(), |_, _, _| false);
        assert!(r.edges.is_empty());
    }

    #[test]
    fn sample_count_is_at_least_two() {
        assert_eq!(clothline_sample_count(0.1, 1.0), 2);
        assert_eq!(clothline_sample_count(3.0, 1.0), 3);
        assert_eq!(clothline_sample_count(10.0, 0.3), 34);
    }

    #[test]
    fn sag_midpoint_is_eight_percent_of_distance() {
        assert!((cloth_sag_offset(0.5, 10.0) - 0.8).abs() < 1e-6);
        assert_eq!(cloth_sag_offset(0.0, 10.0), 0.0);
        assert_eq!(cloth_sag_offset(1.0, 10.0), 0.0);
    }

    #[test]
    fn clothline_points_droop_and_endpoints() {
        let a = Vec3::ZERO;
        let b = Vec3::new(10.0, 0.0, 0.0);
        let pts = clothline_points(a, b, 1.0, 0.0, 0.0);
        assert_eq!(pts.len(), 11);
        assert_eq!(pts[0], a);
        assert!((pts[10] - b).length() < 1e-6);
        // 中点下垂 0.08*10 = 0.8。
        assert!((pts[5].y + 0.8).abs() < 1e-5);
    }

    #[test]
    fn quads_have_width_across_segment() {
        let pts = vec![Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0)];
        let quads = clothline_quads(&pts, 1.0);
        assert_eq!(quads.len(), 1);
        let c = &quads[0].corners;
        assert!((c[0].z - 0.5).abs() < 1e-6);
        assert!((c[2].z - 0.5).abs() < 1e-6);
    }
}

#[cfg(test)]
mod restored_connection_tests {
    use super::*;
    fn a(id: u32, wall: u64, x: f32) -> FlagAnchor {
        FlagAnchor {
            id,
            wall,
            pos: Vec3::new(x, 2.0, 0.0),
            source: true,
            target: true,
        }
    }
    fn cloth_params() -> ConnectParams {
        ConnectParams {
            max_per_node: 1,
            allow_same_wall: true,
            near_different_wall: 9.0,
            far_same_wall: 6.0,
            min_same_wall: 0.0,
        }
    }
    #[test]
    fn stable_unsigned_order_keeps_duplicate_ids_as_distinct_dense_indices() {
        let anchors = [a(u32::MAX, 3, 4.0), a(7, 1, 0.0), a(7, 2, 2.0)];
        let result = greedily_connect(&anchors, &cloth_params(), |_, _, _| false);
        assert_eq!(result.sorted_indices, [1, 2, 0]);
        assert_eq!(result.edges, [(1, 2)]);
        assert_eq!(result.degrees, [0, 1, 1]);
    }
    #[test]
    fn each_source_sorts_weights_with_same_wall_bias_and_stable_ties() {
        let anchors = [a(1, 0, 0.0), a(2, 0, 2.0), a(3, 1, 4.0)];
        let result = greedily_connect(&anchors, &cloth_params(), |_, _, _| false);
        assert_eq!(result.edges, [(0, 2)]); // Same-wall2+3 loses to different-wall4.
        let ties = [a(1, 0, 0.0), a(2, 1, 4.0), a(3, 2, -4.0)];
        assert_eq!(
            greedily_connect(&ties, &cloth_params(), |_, _, _| false).edges,
            [(0, 1)]
        );
        let source_order = [a(1, 0, 0.0), a(2, 1, 4.0), a(3, 2, 4.1), a(4, 3, 8.0)];
        assert_eq!(
            greedily_connect(&source_order, &cloth_params(), |_, _, _| false).edges,
            [(0, 1), (2, 3)]
        );
        // A global shortest-edge sort would incorrectly pick (1,2) first.
    }
    #[test]
    fn disabled_same_wall_still_allows_different_wall_and_nan_is_rejected() {
        let mut params = cloth_params();
        params.allow_same_wall = false;
        assert!(
            greedily_connect(&[a(1, 2, 0.0), a(2, 2, 2.0)], &params, |_, _, _| false)
                .edges
                .is_empty()
        );
        assert_eq!(
            greedily_connect(&[a(1, 2, 0.0), a(2, 3, 2.0)], &params, |_, _, _| false).edges,
            [(0, 1)]
        );
        assert!(
            greedily_connect(&[a(1, 2, 0.0), a(2, 3, f32::NAN)], &params, |_, _, _| false)
                .edges
                .is_empty()
        );
        assert!(clothline_sag_fraction(f32::NAN, 1.0).is_nan());
        assert_eq!(clothline_sample_count(10.0, 0.3), 34);
        assert_eq!(clothline_sample_count(f32::NAN, 0.3), 2);
    }
    #[test]
    fn actual_finite_raycast_blocks_before_endpoint_but_not_beyond_it() {
        use country_core::systems::collision::world::{
            upsert_collider, RaycastGeoSource, RaycastMeshData, RaycastWorld,
        };
        use std::{collections::HashMap, sync::Arc};
        let anchors = [a(1, 1, 0.0), a(2, 2, 4.0)];
        let mut world = RaycastWorld::new();
        let mut receipts = HashMap::new();
        upsert_collider(
            &mut world,
            &mut receipts,
            1u32,
            RaycastGeoSource::Slot0,
            Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO)),
            Vec3::new(6.0, 2.0, 0.0),
            Quat::IDENTITY,
        );
        world.flush_colliders();
        let visible = greedily_connect(&anchors, &cloth_params(), |origin, direction, max_toi| {
            assert_eq!(origin, anchors[0].pos);
            assert_eq!(direction, Vec3::X);
            assert_eq!(max_toi, 4.0 - 0.01);
            world.intersects_any_with_max_toi(origin, direction, max_toi, 0x30db)
        });
        assert_eq!(visible.edges, [(0, 1)]);
        upsert_collider(
            &mut world,
            &mut receipts,
            1u32,
            RaycastGeoSource::Slot0,
            Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO)),
            Vec3::new(2.0, 2.0, 0.0),
            Quat::IDENTITY,
        );
        world.flush_colliders();
        assert!(greedily_connect(&anchors, &cloth_params(), |o, d, t| world
            .intersects_any_with_max_toi(o, d, t, 0x30db))
        .edges
        .is_empty());
    }
}
