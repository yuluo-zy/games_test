//! B `decorator_visual/stairs.rs` — 玩家楼梯装饰（D1）。
//!
//! 证据：fresh RE-MCP 反编译（11 业务 VA）+ 骨架 callees（O）+ 常量 rne-read。
//!
//! 已全文恢复（O）：
//! - `determine_segment_type` 0x14080c810：段类型（坡度比 → 1/2/3）
//! - `densely_sample_spline_segment` 0x140809ad0：委托 utils::Spline2D::
//!   densely_sample_segment（utils 侧已按本函数 VA 恢复，B 原文 451 模块）
//! - `derive_stair_node_params` 0x14080c640：真实锚点、墙法线和图度数
//!
//! 结构恢复（O 业务链，U 主体）：find_alignment_preferences /
//! generate_playermade_stair_decorators / generate_playermade_stair_requests
//! <false,true> / walk_shape / maintain_stair_graph（apply_decorator_op 消费）/
//! compute_stair_assembly_params / find_best_neighbor_alignments。
//!
//! 常量（O）：段类型阈值 = |Δh|/弦长（0x1429cd064=2.5、0x142925950=0.25）。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator::DecoratorId;
use country_core::resources::stairs::stairs_assembly_params::{
    AlignKind, StairNodeAnchor, StairWallAnchor,
};
use country_core::resources::stairs::stairs_state::StairsState;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator::{DecoratorInfo, DecoratorRotationXZ};
use country_core::resources::walls::decorator_storage::DecoratorStorageAddr;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes;
use glam::{Vec2, Vec3};
use utils::natural_cubic::{DenseSplineSegment2D, Spline2D};

use crate::blueprint::DecoratorBlueprints;

/// 段类型阈值（O：0x1429cd064 = 2.5、0x142925950 = 0.25）。
const STEEP_SLOPE: f32 = 2.5;
const GENTLE_SLOPE: f32 = 0.25;

/// 原 0x14080c810：先验证高度，再取路径首尾，保留原有序比较与第二返回位。
pub fn determine_segment_type(points: &[Vec2], heights: &[f32]) -> (u8, u8) {
    let first_height = *heights.first().expect("determine_segment_type: empty heights");
    let last_height = *heights.last().expect("determine_segment_type: empty heights");
    let first = points.first().expect("determine_segment_type: empty points");
    let last = points.last().expect("determine_segment_type: empty points");
    let dx = first.x - last.x;
    let dz = first.y - last.y;
    let slope = (first_height - last_height).abs() / (dz * dz + dx * dx).sqrt();
    let kind = if slope > STEEP_SLOPE {
        3
    } else if slope > GENTLE_SLOPE {
        2
    } else {
        // 原 SETA/CMOVBE 对 NaN 取此分支；不能改成 <=2.5 外门。
        1
    };
    (kind, 1)
}

/// 0x140809ad0（O）：密集采样样条段（原属 stairs.rs 模块 451；已恢复于
/// utils::Spline2D::densely_sample_segment，此处直通）。
pub fn densely_sample_spline_segment(spline: &Spline2D, index: usize) -> DenseSplineSegment2D {
    spline.densely_sample_segment(index)
}

/// 原图节点到请求几何的实际本轮值；normal 与沿曲线的 tangent 不混用。
#[derive(Clone, Copy, Debug)]
struct StairPoint {
    addr: DecoratorStorageAddr,
    wall_normal: Option<Vec2>,
    position: Vec3,
    id: DecoratorId,
    width: f32,
}

/// 原 14048ce90 中翻转后交换两分量，保持墙两侧的真实法线。
fn stair_wall_normal(
    info: &DecoratorInfo,
    addr: DecoratorStorageAddr,
    walls: &PublicWalls,
) -> Vec2 {
    let wall = walls.get(addr.as_wall()).expect("stair point: missing wall");
    let tangent = info.wall_space_coord().get_tangent(wall.simplified_curve_2d());
    if info.dst_tag() == 0 {
        Vec2::new(tangent.y, -tangent.x)
    } else {
        Vec2::new(-tangent.y, tangent.x)
    }
}

/// 原 14048ce90 完整节点 fold；只构造本轮临时表，所有值来自同一权威 owner。
fn collect_stair_points(
    storage: &DecoratorStorage,
    cached: &CachedDecoratorTransforms,
    walls: &PublicWalls,
    stairs: &StairsState,
) -> std::collections::HashMap<DecoratorId, StairPoint> {
    stairs.graph().nodes().map(|raw_id| {
        let id = DecoratorId(raw_id);
        let addr = storage.try_lookup_storage(id).expect("stair point: missing storage address");
        let bucket = storage.get(addr).expect("stair point: missing storage");
        let info = bucket.get(id).expect("stair point: missing decorator");
        let transform = cached.transform(raw_id);
        let extra = info.extra().as_stair();
        let width = f32::from_le_bytes(extra.0[4..8].try_into().unwrap());
        let wall_normal = if info.dst_tag() <= 1 {
            Some(stair_wall_normal(info, addr, walls))
        } else {
            None
        };
        (id, StairPoint { addr, wall_normal, position: transform.translation, id, width })
    }).collect()
}

/// 原 0x14080c640 的真实锚点和度数，不把跳表索引当成业务输出。
pub fn derive_stair_node_params(
    info: &DecoratorInfo,
    walls: &PublicWalls,
    stairs: &StairsState,
    addr: DecoratorStorageAddr,
    id: DecoratorId,
) -> (StairNodeAnchor, u8) {
    // 原低字节转换会回绕；邻居未发布其它信息也不影响此真实图计数。
    let degree = stairs.graph().neighbors(id.0).count() as u8;
    let tag = info.dst_tag();
    let anchor = match tag {
        2 | 3 => {
            let extra = info.extra().as_stair();
            let height = f32::from_le_bytes(extra.0[..4].try_into().unwrap());
            if tag == 2 { StairNodeAnchor::Roof { height } }
            else { StairNodeAnchor::Terrain { height } }
        }
        4 => unreachable!(),
        _ => {
            info.dst_as_wall();
            let normal = stair_wall_normal(info, addr, walls);
            let angle = DecoratorRotationXZ::from_xz_forward_vector(normal).0;
            StairNodeAnchor::Wall(StairWallAnchor::new(addr.as_wall(), angle))
        }
    };
    (anchor, degree)
}

/// 0x14080beb0（O 结构，6+ 参面）：对齐偏好计算。
/// U：偏好准则（邻墙对齐）主体。
pub fn find_alignment_preferences(
    _p1: u64,
    _p2: u64,
    _p3: u64,
    _f4: f32,
    _f5: f32,
) -> Vec<(u32, f32)> {
    Vec::new() // U。
}

/// B 0x14080c360：遍历完整邻接候选组合，返回所有选择及其总分。
/// 冲突方向每次扣 10；严格大于才替换，保持原先后顺序与 NaN 分支。
pub fn find_best_neighbor_alignments(
    candidates: &[(DecoratorId, Vec<(AlignKind, f32)>)],
    used_directions: u8,
) -> (Vec<(DecoratorId, AlignKind)>, f32) {
    let Some(((id, options), remaining)) = candidates.split_first() else {
        return (Vec::new(), 0.0);
    };
    let mut best = Vec::new();
    let mut best_score = -f32::MAX;
    for &(kind, score) in options {
        let direction = match kind.tag {
            0 => 1,
            1 => {
                let left = kind.first.x * kind.second.y;
                let right = kind.first.y * kind.second.x;
                // 原 UCOMISS/SETBE 在 unordered 时也取 mask4。
                if left > right { 2 } else { 4 }
            }
            _ => unreachable!(),
        };
        let (mut choices, child_score) =
            find_best_neighbor_alignments(remaining, used_directions | direction);
        let penalty = if used_directions & direction != 0 { -10.0 } else { 0.0 };
        let total = (score + child_score) + penalty;
        if total > best_score {
            // 接管递归的真实 Vec；原算法在前部插入当前邻居，避免整表 clone。
            choices.insert(0, (*id, kind));
            best = choices;
            best_score = total;
        }
    }
    (best, best_score)
}

/// 0x140803dd0（O 结构）：生成玩家楼梯装饰视觉（walk_shape 消费 + 蓝图）。
pub fn generate_playermade_stair_decorators(
    _storage: Res<DecoratorStorage>,
    _derived: Res<DerivedDecoratorSubtypes>,
    _blueprints: ResMut<DecoratorBlueprints>,
    _transient: ResMut<crate::decorator_visual_state::DecoratorTransientVisualState>,
) {
    // U：楼梯视觉生成主体待取证。
}

/// 0x140276e80 / 0x14027e470（O 结构，泛型 <false,true>）：生成玩家楼梯请求
/// （形状遍历 → 段类型 → 节点参数 → 装配参数）。
pub fn generate_playermade_stair_requests<const X: bool>() {
    // U：主体待取证。
}

/// 0x140809cc0（O 结构）：形状遍历（多边形 → 边段采样）。
/// U：遍历算法主体。
pub fn walk_shape(
    _shape: &[Vec2],
) -> Vec<(u32, Vec3)> {
    Vec::new() // U。
}

/// 0x140806110（O 结构，8 参面）：维护楼梯图（StairsState 消费 +
/// apply_decorator_op 应用）。U：图维护主体。
pub fn maintain_stair_graph(
    _storage: Res<DecoratorStorage>,
    _stairs: Res<country_core::resources::stairs::stairs_state::StairsState>,
    _commands: Commands,
) {
    // U。
}

/// 0x140807090（O 结构，6 参面）：计算楼梯装配参数。
/// U：装配参数主体（StairWallAnchor 消费）。
pub fn compute_stair_assembly_params(
    _nodes: &[(u32, Vec3)],
) -> Vec<f32> {
    Vec::new() // U。
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::walls::decorator::DecoratorInfo;

    fn direction_kind(tag: u8, first: Vec2, second: Vec2) -> AlignKind {
        AlignKind { tag, first, second, ..AlignKind::default() }
    }

    #[test]
    fn alignment_search_chooses_the_global_non_conflicting_combination() {
        let straight = direction_kind(0, Vec2::X, Vec2::ZERO);
        let turn = direction_kind(1, Vec2::X, Vec2::Y);
        let candidates = vec![
            (DecoratorId(7), vec![(straight, 9.0), (turn, 8.0)]),
            (DecoratorId(29), vec![(straight, 9.0)]),
        ];
        let before = candidates.clone();
        let (choices, score) = find_best_neighbor_alignments(&candidates, 0);
        assert_eq!(choices, vec![(DecoratorId(7), turn), (DecoratorId(29), straight)]);
        assert_eq!(score, 17.0);
        assert_eq!(candidates, before);
    }

    #[test]
    fn alignment_search_preserves_the_first_equal_score_candidate() {
        let first = direction_kind(0, Vec2::X, Vec2::ZERO);
        let second = direction_kind(0, Vec2::Y, Vec2::ZERO);
        let candidates = [(DecoratorId(3), vec![(first, 5.0), (second, 5.0)])];
        assert_eq!(find_best_neighbor_alignments(&candidates, 0),
            (vec![(DecoratorId(3), first)], 5.0));
    }

    #[test]
    fn alignment_empty_inputs_and_empty_candidate_lists_have_distinct_scores() {
        assert_eq!(find_best_neighbor_alignments(&[], 7), (vec![], 0.0));
        let candidates = [(DecoratorId(3), vec![])];
        assert_eq!(find_best_neighbor_alignments(&candidates, 0), (vec![], -f32::MAX));
    }

    #[test]
    fn alignment_unordered_cross_uses_mask_four_and_nan_score_never_wins() {
        let kind = direction_kind(1, Vec2::new(f32::NAN, 0.0), Vec2::Y);
        let candidates = [(DecoratorId(3), vec![(kind, 5.0)])];
        assert_eq!(find_best_neighbor_alignments(&candidates, 4).1, -5.0);
        assert_eq!(find_best_neighbor_alignments(&candidates, 2).1, 5.0);
        let straight = direction_kind(0, Vec2::X, Vec2::ZERO);
        let scores = [(DecoratorId(3), vec![(straight, f32::NAN), (straight, 1.0)])];
        assert_eq!(find_best_neighbor_alignments(&scores, 0),
            (vec![(DecoratorId(3), straight)], 1.0));
        let all_nan = [(DecoratorId(3), vec![(straight, f32::NAN)])];
        assert_eq!(find_best_neighbor_alignments(&all_nan, 0), (vec![], -f32::MAX));
    }

    #[test]
    #[should_panic(expected = "internal error: entered unreachable code")]
    fn alignment_none_variant_retains_original_panic() {
        let none = AlignKind { tag: 2, ..AlignKind::default() };
        let _ = find_best_neighbor_alignments(&[(DecoratorId(3), vec![(none, 0.0)])], 0);
    }

    fn stair_info(tag: u8, height: f32, width: f32) -> DecoratorInfo {
        use country_core::resources::walls::decorator::{DecoratorExtraInfo, StairExtra};
        let mut info = DecoratorInfo {
            u20: u32::from(tag) << 16,
            dst_wall: u64::from(1.0f32.to_bits()) | (u64::from(2.0f32.to_bits()) << 32),
            ..DecoratorInfo::default()
        };
        let mut extra = [0; 20];
        extra[..4].copy_from_slice(&height.to_le_bytes());
        extra[4..8].copy_from_slice(&width.to_le_bytes());
        info.set_extra(DecoratorExtraInfo::Stair(StairExtra(extra)));
        info
    }

    fn published_wall() -> (PublicWalls, country_core::resources::walls::WallId) {
        use country_core::resources::roofs::geometry::surface::Curve2;
        use country_core::resources::walls::publication_source::TerrainHeightGrid512;
        use country_core::resources::walls::{InnerWalls, WallState};
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0], [4.0, 4.0]], false).unwrap();
        let mut inner = InnerWalls::default();
        let wall = inner.new_wall(WallState::new(curve, 4.0, 0, 0, false));
        let terrain = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0, maximum_y: 0.0, sampling_fix: false,
        };
        let mut public = PublicWalls::default();
        public.consume(&mut inner, &terrain);
        assert!(public.get(wall).is_some());
        (public, wall)
    }

    #[test]
    fn actual_world_graph_points_and_both_wall_sides_use_published_curve() {
        use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform;
        let (walls, wall) = published_wall();
        let mut storage = DecoratorStorage::default();
        let mut cached = CachedDecoratorTransforms::default();
        let mut stairs = StairsState::new();
        for (raw_id, tag, position) in [
            (29, 0, Vec3::new(8.0, 3.0, 9.0)),
            (7, 1, Vec3::new(2.0, 4.0, 5.0)),
            (60, 2, Vec3::new(1.0, 6.0, 3.0)),
            (90, 3, Vec3::new(5.0, 2.0, 7.0)),
        ] {
            let addr = if tag == 3 { DecoratorStorageAddr::Terrain }
                       else { DecoratorStorageAddr::Wall(wall) };
            storage.apply_decorator_place(addr, DecoratorId(raw_id), &stair_info(tag, 2.5, 1.75));
            cached.insert(raw_id, CachedDecoratorTransform {
                rotation: glam::Quat::IDENTITY, translation: position, scale: Vec3::ONE,
            });
            stairs.add_node(raw_id);
        }
        stairs.add_edge(29, 7);
        stairs.add_edge(29, 60);
        let mut world = World::new();
        world.insert_resource(storage);
        world.insert_resource(cached);
        world.insert_resource(walls);
        world.insert_resource(stairs);
        let storage = world.resource::<DecoratorStorage>();
        let walls = world.resource::<PublicWalls>();
        let stairs = world.resource::<StairsState>();
        let points = collect_stair_points(storage, world.resource::<CachedDecoratorTransforms>(), walls, stairs);
        assert_eq!(points.len(), 4);
        for (id, expected) in [(29, Vec2::NEG_Y), (7, Vec2::Y)] {
            let point = points[&DecoratorId(id)];
            assert_eq!(point.id, DecoratorId(id));
            assert_eq!(point.width, 1.75);
            assert_eq!(point.wall_normal, Some(expected));
            let info = storage.get(point.addr).unwrap().get(point.id).unwrap();
            let (anchor, degree) = derive_stair_node_params(info, walls, stairs, point.addr, point.id);
            let anchor = anchor.try_as_wall().unwrap();
            assert_eq!(anchor.wall, wall);
            assert!((anchor.wall_normal() - expected).length() < 0.000002);
            assert_eq!(degree, if id == 29 { 2 } else { 1 });
        }
        assert_eq!(points[&DecoratorId(29)].position, Vec3::new(8.0, 3.0, 9.0));
        assert!(points[&DecoratorId(60)].wall_normal.is_none());
        assert!(points[&DecoratorId(90)].wall_normal.is_none());
        let point = points[&DecoratorId(90)];
        let info = storage.get(point.addr).unwrap().get(point.id).unwrap();
        assert_eq!(derive_stair_node_params(info, walls, stairs, point.addr, point.id),
                   (StairNodeAnchor::Terrain { height: 2.5 }, 0));
    }

    #[test]
    fn derive_preserves_height_bits_and_original_degree_byte_wrap() {
        let mut stairs = StairsState::new();
        stairs.add_node(700);
        for other in 0..256 { stairs.add_edge(700, other); }
        let height = f32::from_bits(0x7fc1_2345);
        for tag in [2, 3] {
            let (anchor, degree) = derive_stair_node_params(
                &stair_info(tag, height, 2.0), &PublicWalls::default(), &stairs,
                DecoratorStorageAddr::Terrain, DecoratorId(700),
            );
            assert_eq!(degree, 0);
            let actual = match anchor {
                StairNodeAnchor::Roof { height } | StairNodeAnchor::Terrain { height } => height,
                StairNodeAnchor::Wall(_) => panic!("意外墙锚点"),
            };
            assert_eq!(actual.to_bits(), height.to_bits());
        }
    }

    #[test]
    fn required_stair_point_failures_preserve_cache_before_extra_order() {
        use country_core::resources::walls::decorator::{DecoratorExtraInfo, WindowExtra};
        let mut storage = DecoratorStorage::default();
        let mut info = stair_info(3, 2.0, 1.0);
        info.set_extra(DecoratorExtraInfo::Window(WindowExtra(0)));
        storage.apply_decorator_place(DecoratorStorageAddr::Terrain, DecoratorId(7), &info);
        let mut stairs = StairsState::new();
        stairs.add_node(7);
        let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            collect_stair_points(&storage, &CachedDecoratorTransforms::default(), &PublicWalls::default(), &stairs);
        })).unwrap_err();
        let message = panic.downcast_ref::<String>().map(String::as_str)
            .or_else(|| panic.downcast_ref::<&str>().copied()).unwrap();
        assert!(message.contains("CachedDecoratorTransforms::transform"));
    }

    #[test]
    #[should_panic(expected = "missing storage address")]
    fn required_stair_point_without_authoritative_address_panics() {
        let mut stairs = StairsState::new();
        stairs.add_node(7);
        collect_stair_points(&DecoratorStorage::default(), &CachedDecoratorTransforms::default(),
                             &PublicWalls::default(), &stairs);
    }

    #[test]
    #[should_panic(expected = "internal error: entered unreachable code")]
    fn derive_stair_destination_is_unreachable() {
        derive_stair_node_params(&stair_info(4, 0.0, 1.0), &PublicWalls::default(),
                                 &StairsState::new(), DecoratorStorageAddr::Stair, DecoratorId(7));
    }

    #[test]
    fn segment_type_preserves_ordered_nan_and_both_original_return_bytes() {
        let line = [Vec2::ZERO, Vec2::X];
        for (heights, kind) in [([0.0, 0.25], 1), ([0.0, 2.5], 2),
                                ([0.0, 3.0], 3), ([0.0, f32::NAN], 1),
                                ([0.0, f32::INFINITY], 3)] {
            assert_eq!(determine_segment_type(&line, &heights), (kind, 1));
        }
        assert_eq!(determine_segment_type(&[Vec2::ZERO], &[4.0]), (1, 1));
        assert_eq!(determine_segment_type(&[Vec2::ZERO], &[0.0, 1.0]), (3, 1));
        assert_eq!(determine_segment_type(&line, &[0.0, 100.0, 0.25]), (1, 1));
    }

    #[test]
    #[should_panic(expected = "empty heights")]
    fn segment_type_checks_empty_heights_before_empty_points() {
        determine_segment_type(&[], &[]);
    }

    #[test]
    #[should_panic(expected = "empty points")]
    fn segment_type_requires_the_real_points_slice() {
        determine_segment_type(&[], &[1.0]);
    }

}
