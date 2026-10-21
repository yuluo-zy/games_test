//! B `decorator_visual/stairs_assemble.rs` — 玩家楼梯装配（D1）。
//!
//! 证据：fresh RE-MCP 反编译（18 业务 VA）+ 原字节手解（capdis）+ 骨架 callees（O）。
//!
//! **状态（2026-10-20）**：gen_stairs_structural_collider 955B 的八点凸包和
//! tiny-sphere fallback 已按原字节及独立执行恢复；其余主流程仍部分恢复。
//! `determine_ladder_support` 0x1407de800 只有原字节结构（a/b 高度选段 + 逐段
//! 校验分支），段阈值/flag 语义仍 U，函数体返回显式未恢复值 0；订阅/碰撞
//! 函数同样只有调用面结构。`playermade_stairs_assemble<true/false>`
//! （0x140008c90/0x140018040）是**硬阻塞 U**：Ghidra 两次 timeout，且 4 个
//! 发射闭包 `closure$8/9<true>`（0x140284b0/0x140275b0）、
//! `closure$24/25<false>`（0x140286d0/0x14027840）均无落盘反编译；在补齐
//! 闭包/P-code 前禁止写空体或伪造装配输出。
//!
//! 结构恢复（O 业务链，U 主体）：calculate_adjusted_ladder_positions /
//! construct_ladder / playermade_stairs_assemble<false,true> /
//! playermade_stairs_interpolate_and_compare /
//! playermade_stairs_assembly_propagate_dirty_flags /
//! playermade_stairs_add_wall_holes /
//! create_structural_colliders_and_initial_graph /
//! calculate_distortion_from_structural_graph /
//! split_segment_into_supported_and_unsupported_subsegments /
//! construct_stair_steps / gen_stair_floors / gen_stairs_collider_and_rt_geo /
//! create_stairs_collider。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator::DecoratorId;
use country_core::resources::stairs::stairs_assembly_params::{
    StairGraphEdgeIdentity, StairNodeAnchor, StairSubSegmentMap,
};
use country_core::resources::stairs::stairs_state::{StairSegment, StairsState};
use country_core::systems::collision::world::RaycastWorld;
use glam::{Vec2, Vec3};

use crate::decorator_visual_state::DecoratorTransientVisualState;

/// 楼梯种类判别（O：create_stairs_collider 0x1407deb60 首字节 '\x11'=17 =
/// 移除类；I 枚举面）。
const STAIR_KIND_REMOVE: u8 = 0x11;

/// 0x1407de800：梯支撑判定（**结构面，主体 U**）。
///
/// 4 参面 {a: {f32@4}, b: {f32@4}, segments: &[0x20B {u32@0, f32@4, u8@0x18}],
/// flag: {+0x14/+0 字节}} → u8 {0,1,2,3,4}。原字节可见：比较 a/b 高度选取两个
/// 0x20B 段（b<=a → 段1，否则段2）与后续逐段校验分支；段阈值常量与 flag 语义
/// 未闭合，返回 0 为显式未恢复值（不是“全文恢复”）。
pub fn determine_ladder_support(
    a: f32,
    b: f32,
    _segments: &[(u32, f32, u8)],
    _flag: u8,
) -> u8 {
    // O 数学：edx = (b<=a) ? 0x20 : 0；eax = (b>a) ? 0x20 : 0；逐段校验 + 返回
    // 分支。U：段数组/flag 精确语义 → 恒 0 为显式未恢复面。
    let _ = (a, b);
    0
}

/// 0x1407d4990（O，944B）：结构图订阅（事件回调注册）。
pub fn structural_graph_subscribe(
    _subscribers: &mut Vec<fn(u64)>,
    _callback_ctx: u64,
) {
    // U：回调注册面（FUN_1412926e0 订阅器）待取证。
}

/// 0x1407deb60（O 结构）：创建/移除楼梯碰撞体。
/// kind==0x11 → RaycastWorld::remove_collider<StairGraphEdge>；否则碰撞记录
/// 构建（常量 0x142a88770/78 载荷，U）。
pub fn create_stairs_collider(
    kind: u8,
    _p2: u64,
    _world: &mut RaycastWorld,
    _p4: u64,
    _p5: u32,
) {
    if kind == STAIR_KIND_REMOVE {
        // O：remove_collider<StairGraphEdge>（0x140252ad0）。
        let _ = _world;
    }
    // U：创建面待取证。
}

/// 0x1407cf210（O 结构，4 参面）：惰性订阅（回调闭包 {3, 0x90000} 注册）。
/// U：订阅语义。
pub fn stairs_subscribe_lazy(
    _subscribers: &mut Vec<fn(u64)>,
    _p2: u64,
    _p3: u64,
    _p4: u64,
) {
    // U。
}

/// B 0x1407d8fe0：按墙端与 junction 的真实旧段类型调整梯子两端。
/// 锚点和邻接度来自同一 request；旧段表借用 canonical owner，不创建镜像。
pub fn calculate_adjusted_ladder_positions(
    nodes: &[(StairNodeAnchor, u8); 2],
    from: Vec3,
    to: Vec3,
    stairs: &StairsState,
    previous_segments: &StairSubSegmentMap,
    endpoint_ids: [DecoratorId; 2],
) -> [Vec3; 2] {
    let from_above = from.y > to.y;
    let (higher, lower) = if from_above { (&nodes[0], &nodes[1]) } else { (&nodes[1], &nodes[0]) };
    let wall_offset = if higher.0.try_as_wall().is_some()
        && higher.1 == 1
        && lower.0.try_as_wall().is_none()
    {
        f32::from_bits(0x3e8f_5c29)
    } else {
        0.0
    };
    // 原先算 from-to，再按 dz²+dx² 求倒长度；零水平长度保留 NaN。
    let dx = from.x - to.x;
    let dz = from.z - to.z;
    let reciprocal = 1.0 / (dz * dz + dx * dx).sqrt();
    let direction = Vec3::new(dx * reciprocal, 0.0, dz * reciprocal);
    // from/to 的身份顺序与几何一致，只有排除当前无向边时才规范化。
    let [a, b] = endpoint_ids;
    let segment = StairSegment::new(a, b);
    let needs_offset = if to.y > from.y && nodes[0].1 > 1 {
        !neighbor_segment_is_ladder(stairs, previous_segments, a, segment, true)
    } else {
        from.y > to.y && nodes[1].1 > 1
            && !neighbor_segment_is_ladder(stairs, previous_segments, b, segment, false)
    };
    let joint_offset = if needs_offset { f32::from_bits(0x3e99_999a) } else { 0.0 };
    if from_above {
        [from - direction * wall_offset, to - direction * joint_offset]
    } else {
        [from + direction * joint_offset, to + direction * wall_offset]
    }
}

/// B 0x1407d9240：第一条命中旧表的相邻边决定结果，空列表也立即 false。
fn neighbor_segment_is_ladder(
    stairs: &StairsState,
    previous_segments: &StairSubSegmentMap,
    node: DecoratorId,
    current: StairSegment,
    last: bool,
) -> bool {
    for neighbor in stairs.graph().neighbors(node.0) {
        if stairs.graph().edge_weight(node.0, neighbor).is_none() {
            unreachable!();
        }
        let other = StairSegment::new(node, DecoratorId(neighbor));
        if other == current {
            continue;
        }
        let Some(segments) = previous_segments.get(&StairGraphEdgeIdentity::from(other)) else {
            continue;
        };
        let endpoint = if last { segments.last() } else { segments.first() };
        return endpoint.is_some_and(|segment| segment.kind == 3);
    }
    false
}

/// 0x1407db3e0（O 结构）：构建梯（梯段几何）。
pub fn construct_ladder(
    _positions: &[Vec3],
) -> Vec<u8> {
    Vec::new() // U。
}

/// 0x140008c90 / 0x140018040（泛型 <true,false>）：玩家楼梯装配主流程。
///
/// **硬阻塞 U**（禁止空体冒充）：主体两实例栈帧 0x948、开头 48B 逐字节相同；
/// Ghidra 两次 timeout。缺 4 个发射闭包的反编译/P-code：
/// `closure$8<true>` 0x140284b0、`closure$9<true>` 0x140275b0、
/// `closure$24<false>` 0x140286d0、`closure$25<false>` 0x14027840。
/// 还需 `PlayermadeStairsAssemblyRequests`/`AdjustedStairEdge`/
/// `StairSubSegment`/`SampledStairsPoints` core 类型（契约 §4.2）。
pub fn playermade_stairs_assemble<const X: bool>() {
    // U：见上；未取证前不写装配输出。
}

/// 0x1407cf500（O 结构）：插值与比较（新旧装配对比）。
pub fn playermade_stairs_interpolate_and_compare(
    _old: &[f32],
    _new: &[f32],
) -> bool {
    false // U。
}

/// 0x1407d16b0（O 结构）：脏标记传播。
pub fn playermade_stairs_assembly_propagate_dirty_flags(
    _dirty: &mut Vec<u32>,
) {
    // U。
}

/// 0x1407d1f80（O 结构，6 参面）：添加墙洞。
pub fn playermade_stairs_add_wall_holes(
    _wall_holes: &mut country_core::resources::walls::wall_holes::WallHoles,
    _p2: u64,
) {
    // U。
}

/// 0x1407d20c0（O 结构）：创建结构碰撞体与初始图。
pub fn create_structural_colliders_and_initial_graph(
    _storage: Res<DecoratorStorage>,
    _stairs: ResMut<country_core::resources::stairs::stairs_state::StairsState>,
    _transient: ResMut<DecoratorTransientVisualState>,
) {
    // U。
}

/// 0x1407d49d0（O 结构）：结构图变形计算。
pub fn calculate_distortion_from_structural_graph(
    _graph: &[(u32, u32)],
) -> Vec<f32> {
    Vec::new() // U。
}

/// 0x1407d94a0（O 结构）：段分割为支撑/非支撑子段。
pub fn split_segment_into_supported_and_unsupported_subsegments(
    _segment: &[(Vec2, f32)],
) -> Vec<(usize, usize)> {
    Vec::new() // U。
}

/// 0x1407da0b0（O 结构）：构建梯步。
pub fn construct_stair_steps(
    _steps: &[(Vec3, f32)],
) -> Vec<u8> {
    Vec::new() // U。
}

/// 0x1407dc170（O 结构）：生成梯层（floor_tri_count_to_atlas_idx 消费）。
pub fn gen_stair_floors(
    _segments: &[(u32, Vec3)],
) -> Vec<u8> {
    Vec::new() // U。
}

/// 0x1407de8b0（O 结构，10 参面）：生成梯碰撞 + 实时几何
/// （配置数组 {2,3,6,...} + 碰撞生成器链）。
pub fn gen_stairs_collider_and_rt_geo(
    _config: &[u64],
    _p2: u64,
) {
    // U。
}

/// B 1407dec90/955B: construct the eight original structural hull vertices,
/// then use the original tiny-sphere fallback only if convex construction fails.
/// The caller's actual per-segment registry/graph handoff remains separate.
pub fn gen_stairs_structural_collider(
    start: Vec3,
    end: Vec3,
    start_direction: Vec2,
    end_direction: Vec2,
    start_width: f32,
    end_width: f32,
) -> country_core::systems::collision::world::RaycastMeshData {
    use country_core::systems::collision::world::RaycastMeshData;
    let (points, fallback_center) = structural_stair_collider_geometry(
        start, end, start_direction, end_direction, start_width, end_width,
    );
    RaycastMeshData::new_convex(&points).unwrap_or_else(|| {
        RaycastMeshData::new_sphere(
            f32::from_bits(0x3727_c5ac),
            glam::UVec3::new(
                fallback_center.x.to_bits(),
                fallback_center.y.to_bits(),
                fallback_center.z.to_bits(),
            ),
        )
    })
}

/// The native structural-leaf -> actual collider-registry handoff from
/// 1407d2b79/1407d2ede. The full assembly-request traversal must supply its
/// real source/segment geometry; this does not replace that unrecovered loop.
#[allow(clippy::too_many_arguments)]
pub fn upsert_structural_stair_segment(
    world: &mut RaycastWorld,
    registry: &mut country_core::resources::stairs::stairs_collider::StairColliderRegistry,
    segment: (country_core::resources::stairs::stairs_state::StairGraphEdge, u32),
    source: country_core::systems::collision::world::RaycastGeoSource,
    start: Vec3,
    end: Vec3,
    start_direction: Vec2,
    end_direction: Vec2,
    start_width: f32,
    end_width: f32,
) -> country_core::systems::collision::world::ColliderUpsert {
    let mesh = gen_stairs_structural_collider(
        start, end, start_direction, end_direction, start_width, end_width,
    );
    country_core::systems::collision::world::upsert_collider(
        world,
        &mut registry.structural_colliders,
        segment,
        source,
        std::sync::Arc::new(mesh),
        Vec3::ZERO,
        glam::Quat::IDENTITY,
    )
}

/// Pure argument construction from the same native leaf. Kept public so the
/// retained original SIMD oracle can independently check all float words.
pub fn structural_stair_collider_geometry(
    start: Vec3,
    end: Vec3,
    start_direction: Vec2,
    end_direction: Vec2,
    start_width: f32,
    end_width: f32,
) -> ([Vec3; 8], Vec3) {
    let mut points = [Vec3::ZERO; 8];
    let start_y = start.y + -0.5;
    let end_y = end.y + -0.5;
    for (index, (position, direction, width, shifted_y)) in [
        (start, start_direction, start_width, start_y),
        (end, end_direction, end_width, end_y),
    ].into_iter().enumerate() {
        // Native 140c9cdc0 expands (-direction.y, direction.x) to x0z;
        // preserve its separate multiply and addition order, including signed 0.
        let dx = (-direction.y * width) * 0.5;
        let dy = (0.0 * width) * 0.5;
        let dz = (direction.x * width) * 0.5;
        let left = Vec3::new(position.x - dx, shifted_y - dy, position.z - dz);
        let right = Vec3::new(position.x + dx, shifted_y + dy, position.z + dz);
        points[index * 4] = Vec3::new(left.x, left.y + -0.25, left.z);
        points[index * 4 + 1] = Vec3::new(right.x, right.y + -0.25, right.z);
        points[index * 4 + 2] = Vec3::new(left.x + 0.0, left.y + 0.25, left.z + 0.0);
        points[index * 4 + 3] = Vec3::new(right.x + 0.0, right.y + 0.25, right.z + 0.0);
    }
    // This unusual native fallback is an endpoint sum plus 0.5, not a midpoint.
    // Preserve it and the shifted-y addition order rather than "correcting" it.
    let fallback_center = Vec3::new(
        (start.x + end.x) + 0.5,
        (start_y + end_y) + 0.5,
        (start.z + end.z) + 0.5,
    );
    (points, fallback_center)
}

/// 梯三角计数 → 图集索引（内联，O callee 面；U 映射）。
pub fn floor_tri_count_to_atlas_idx(_tri_count: u32) -> u32 {
    0 // U。
}

#[cfg(test)]
mod ladder_position_tests {
    use super::*;
    use country_core::resources::stairs::stairs_assembly_params::{
        StairSubSegment, StairSubSegments, StairWallAnchor,
    };
    use country_core::resources::walls::WallId;

    fn terrain(degree: u8) -> (StairNodeAnchor, u8) {
        (StairNodeAnchor::Terrain { height: 0.0 }, degree)
    }
    fn wall(degree: u8) -> (StairNodeAnchor, u8) {
        (StairNodeAnchor::Wall(StairWallAnchor::new(WallId(91), 0.0)), degree)
    }
    fn edge(a: u32, b: u32) -> StairSegment {
        StairSegment::new(DecoratorId(a), DecoratorId(b))
    }
    fn record(kinds: &[u8]) -> StairSubSegments {
        kinds.iter().map(|&kind| StairSubSegment { kind, flag: 1, count: 1 }).collect()
    }
    fn place(
        nodes: [(StairNodeAnchor, u8); 2], from: Vec3, to: Vec3,
        stairs: &StairsState, previous: &StairSubSegmentMap,
    ) -> [Vec3; 2] {
        calculate_adjusted_ladder_positions(&nodes, from, to, stairs, previous, [DecoratorId(1), DecoratorId(2)])
    }

    #[test]
    fn terminal_wall_offset_preserves_both_directions_and_equal_height_choice() {
        let stairs = StairsState::new();
        let previous = StairSubSegmentMap::default();
        let delta = f32::from_bits(0x3e8f_5c29);
        let up = place([terrain(1), wall(1)], Vec3::ZERO, Vec3::new(2.0, 2.0, 0.0), &stairs, &previous);
        assert_eq!(up, [Vec3::ZERO, Vec3::new(2.0 - delta, 2.0, 0.0)]);
        let down = place([wall(1), terrain(1)], Vec3::new(2.0, 2.0, 0.0), Vec3::ZERO, &stairs, &previous);
        assert_eq!(down, [up[1], Vec3::ZERO]);
        let level = place([terrain(1), wall(1)], Vec3::ZERO, Vec3::new(2.0, 0.0, 0.0), &stairs, &previous);
        assert_eq!(level[1], Vec3::new(2.0 - delta, 0.0, 0.0));
        let both = place([wall(1), wall(1)], Vec3::ZERO, Vec3::new(2.0, 2.0, 0.0), &stairs, &previous);
        assert_eq!(both, [Vec3::ZERO, Vec3::new(2.0, 2.0, 0.0)]);
    }

    #[test]
    fn ascending_junction_uses_last_subsegment_and_spilled_smallvec() {
        let mut stairs = StairsState::new();
        stairs.add_edge(1, 2);
        stairs.add_edge(1, 3);
        let mut previous = StairSubSegmentMap::default();
        let key = StairGraphEdgeIdentity::from(edge(1, 3));
        let segments = record(&[1, 3]);
        assert!(segments.spilled());
        previous.insert(key, segments);
        let end = Vec3::new(2.0, 2.0, 0.0);
        assert_eq!(place([terrain(2), terrain(1)], Vec3::ZERO, end, &stairs, &previous), [Vec3::ZERO, end]);
        previous.insert(key, record(&[3, 1]));
        assert_eq!(place([terrain(2), terrain(1)], Vec3::ZERO, end, &stairs, &previous)[0], Vec3::new(-0.3, 0.0, 0.0));
    }

    #[test]
    fn descending_junction_uses_first_subsegment() {
        let mut stairs = StairsState::new();
        stairs.add_edge(1, 2);
        stairs.add_edge(2, 3);
        let mut previous = StairSubSegmentMap::default();
        let key = StairGraphEdgeIdentity::from(edge(2, 3));
        previous.insert(key, record(&[3, 1]));
        let start = Vec3::new(2.0, 2.0, 0.0);
        assert_eq!(place([terrain(1), terrain(2)], start, Vec3::ZERO, &stairs, &previous), [start, Vec3::ZERO]);
        previous.insert(key, record(&[1, 3]));
        assert_eq!(place([terrain(1), terrain(2)], start, Vec3::ZERO, &stairs, &previous)[1], Vec3::new(-0.3, 0.0, 0.0));
    }

    #[test]
    fn first_present_neighbor_decides_and_missing_entries_continue() {
        let mut stairs = StairsState::new();
        stairs.add_edge(1, 2);
        stairs.add_edge(1, 3);
        stairs.add_edge(1, 4);
        let others: Vec<_> = stairs.graph().neighbors(1).filter(|&id| id != 2).collect();
        let first = StairGraphEdgeIdentity::from(edge(1, others[0]));
        let second = StairGraphEdgeIdentity::from(edge(1, others[1]));
        let mut previous = StairSubSegmentMap::default();
        previous.insert(second, record(&[3]));
        assert!(neighbor_segment_is_ladder(&stairs, &previous, DecoratorId(1), edge(1, 2), true));
        previous.insert(first, record(&[1]));
        assert!(!neighbor_segment_is_ladder(&stairs, &previous, DecoratorId(1), edge(1, 2), true));
        previous.insert(first, record(&[]));
        assert!(!neighbor_segment_is_ladder(&stairs, &previous, DecoratorId(1), edge(1, 2), true));
        assert!(stairs.graph().contains_edge(1, 2));
        assert_eq!(previous.len(), 2);
    }

    #[test]
    fn absent_node_and_current_edge_only_do_not_find_a_neighbor_ladder() {
        let mut stairs = StairsState::new();
        let mut previous = StairSubSegmentMap::default();
        previous.insert(StairGraphEdgeIdentity::from(edge(1, 2)), record(&[3]));
        assert!(!neighbor_segment_is_ladder(&stairs, &previous, DecoratorId(1), edge(1, 2), false));
        stairs.add_edge(1, 2);
        assert!(!neighbor_segment_is_ladder(&stairs, &previous, DecoratorId(1), edge(1, 2), false));
    }

    #[test]
    fn zero_horizontal_span_retains_nan_and_unordered_height_does_not_add_joint_offset() {
        let stairs = StairsState::new();
        let previous = StairSubSegmentMap::default();
        let points = place([terrain(1), terrain(1)], Vec3::new(1.0, 0.0, 1.0), Vec3::new(1.0, 2.0, 1.0), &stairs, &previous);
        assert!(points.iter().all(|point| point.x.is_nan() && point.z.is_nan()));
        assert_eq!(points[0].y, 0.0);
        assert_eq!(points[1].y, 2.0);
        let unordered = place([terrain(7), wall(1)], Vec3::new(0.0, f32::NAN, 0.0), Vec3::new(2.0, 2.0, 0.0), &stairs, &previous);
        assert_eq!(unordered[0].x, 0.0);
        assert!(unordered[0].y.is_nan());
        assert_eq!(unordered[1].x, 2.0 - f32::from_bits(0x3e8f_5c29));
    }
    #[test]
    fn reversed_endpoint_ids_keep_their_geometry_and_neighbor_association() {
        let mut stairs = StairsState::new();
        stairs.add_edge(29, 7);
        stairs.add_edge(29, 3);
        let mut previous = StairSubSegmentMap::default();
        previous.insert(StairGraphEdgeIdentity::from(edge(29, 3)), record(&[1, 3]));
        let start = Vec3::ZERO;
        let end = Vec3::new(2.0, 2.0, 0.0);
        assert_eq!(calculate_adjusted_ladder_positions(
            &[terrain(2), terrain(1)], start, end, &stairs, &previous,
            [DecoratorId(29), DecoratorId(7)],
        ), [start, end]);
        let mut down_stairs = StairsState::new();
        down_stairs.add_edge(29, 7);
        down_stairs.add_edge(7, 3);
        previous.clear();
        previous.insert(StairGraphEdgeIdentity::from(edge(7, 3)), record(&[3, 1]));
        assert_eq!(calculate_adjusted_ladder_positions(
            &[terrain(1), terrain(2)], end, start, &down_stairs, &previous,
            [DecoratorId(29), DecoratorId(7)],
        ), [end, start]);
    }

}
