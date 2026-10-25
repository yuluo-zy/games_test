//! B `decorator_visual/stairs_assemble.rs` — 玩家楼梯装配。
//!
//! 本轮原证闭包：calculate_adjusted_ladder_positions 598B及相邻603B回调；
//! playermade_stairs_interpolate_and_compare 8622B及真实六资源wrapper。
//! 插值消费同一请求/历史/可视状态owner，保旧XZ与输出XYZ的不同坐标语义。
//! 本轮候选的实际CPU验证、原宿主注册和完整cache仍分别待验收，不能据此
//! 宣称完整楼梯渲染链已挂载。原gen_stairs_structural_collider 955B的八点
//! 凸包及tiny-sphere分支沿用已有恢复；其余标U的装配/订阅/碰撞/支撑函数
//! 仍是明确缺口，不以本轮类型或插值恢复替代它们的业务实现。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator::DecoratorId;
use country_core::resources::stairs::stairs_assembly_params::{
    PlayermadeStairsAssemblyRequests, SampledStairsPoints, StairGraphEdgeIdentity,
    StairNodeAnchor, StairSamplesMetadata, StairSubSegmentMap,
};
use country_core::resources::stairs::stairs_state::{StairGraphEdge, StairSegment, StairsState, USE_STAIRS_LAZINESS};
use country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports;
use country_core::resources::stairs::visual::StairsVisualState;
use country_core::resources::time::GameTime;
use country_core::systems::collision::world::RaycastWorld;
use glam::{Vec2, Vec3};

use crate::decorator_visual_state::DecoratorTransientVisualState;

/// 楼梯种类判别（O：create_stairs_collider 0x1407deb60 首字节 '\x11'=17 =
/// 移除类；I 枚举面）。
const STAIR_KIND_REMOVE: u8 = 0x11;

/// B 0x1407de800/167B：按两端锚点、邻接度与低端对齐种类决定梯支撑。
/// 返回原五种判别值；相等或无序高度都将第二端作为低端。
pub fn determine_ladder_support(
    from: Vec3,
    to: Vec3,
    nodes: &[(StairNodeAnchor, u8); 2],
    alignments: &[country_core::resources::stairs::stairs_assembly_params::AlignKind; 2],
) -> u8 {
    let (lower, higher) = if to.y > from.y { (0, 1) } else { (1, 0) };
    let near_ground = |anchor: &StairNodeAnchor| match anchor {
        StairNodeAnchor::Terrain { height } | StairNodeAnchor::Roof { height } =>
            *height <= f32::from_bits(0x3e05_1eb8),
        StairNodeAnchor::Wall(_) => false,
    };
    let lower_supported = nodes[lower].1 != 1 || match &nodes[lower].0 {
        StairNodeAnchor::Wall(_) => alignments[lower].tag == 0,
        anchor => near_ground(anchor),
    };
    let higher_supported = nodes[higher].1 != 1 || match &nodes[higher].0 {
        StairNodeAnchor::Wall(_) => true,
        anchor => near_ground(anchor),
    };
    if !lower_supported {
        return 4 - u8::from(higher_supported);
    }
    if higher_supported {
        return 0;
    }
    // 原最后一次锚点高度检查不受邻接度门控制，不能复用 lower_supported。
    if near_ground(&nodes[lower].0) { 1 } else { 2 }
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

/// 原 optional raw 判别；非 Stair 不先经过会 panic 的 enum parser。
fn optional_stair_extra(info: &country_core::resources::walls::decorator::DecoratorInfo)
    -> Option<country_core::resources::walls::decorator::StairExtra>
{
    (info.extra_disc() <= 1).then(|| info.extra().as_stair())
}

/// 原四次 optional 查找保持 +28、extra+8、extra+10、extra+c 的读取顺序。
fn stair_samples_metadata(storage: &DecoratorStorage, removed: &StairsRemovedSupports,
    segment: StairSegment) -> StairSamplesMetadata
{
    let first = segment.nodes().0;
    let info_28 = storage.lookup_decorator_info(first).map_or(0, |info| info.u28);
    let extra_08 = storage.lookup_decorator_info(first).and_then(optional_stair_extra)
        .map_or(0, |extra| u32::from_le_bytes(extra.0[8..12].try_into().unwrap()));
    let extra_10 = storage.lookup_decorator_info(first).and_then(optional_stair_extra)
        .map_or(0, |extra| extra.0[16]);
    let extra_0c = storage.lookup_decorator_info(first).and_then(optional_stair_extra)
        .map_or(0, |extra| u32::from_le_bytes(extra.0[12..16].try_into().unwrap()));
    StairSamplesMetadata { info_28, extra_08, extra_0c,
        removed_support: removed.contains(&StairGraphEdge::from(segment)), extra_10 }
}

/// 原1407cf500/8622B：插值当前请求，比较真实旧采样并维护同一历史 owner。
pub fn playermade_stairs_interpolate_and_compare(
    mut requests: ResMut<PlayermadeStairsAssemblyRequests>, time: Res<GameTime>,
    visual: Res<StairsVisualState>, removed: Res<StairsRemovedSupports>, storage: Res<DecoratorStorage>,
    _signifiers: Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,
) {
    let factor = utils::time_continuous_lerp_factor(time.delta, 9.0);
    let requests = &mut *requests;
    let mut old_samples = std::mem::take(&mut requests.previous.sampled_points);
    // 原 typed fold141293420：增量发布本轮段列表，不清除独立旧段表。
    requests.previous.sub_segments.extend(requests.requests.iter()
        .map(|request| (StairGraphEdgeIdentity::from(request.segment), request.sub_segments.clone())));
    for request in &mut requests.requests {
        let identity = StairGraphEdgeIdentity::from(request.segment);
        let mut positions: Vec<_> = request.curve.points.iter().zip(&request.heights)
            .map(|(point, &height)| Vec3::new(point.x, height, point.y)).collect();
        let metadata = stair_samples_metadata(&storage, &removed, request.segment);
        if let Some(previous) = old_samples.remove(&identity) {
            // 旧比较按XZ；最终输出的Curve3按XYZ，不能被同ICF getter名字混淆。
            if let Ok(old_curve) = utils::curve::Curve3XZ::try_new_from_points(previous.points.clone(), true) {
                let mut moved = false;
                for index in 0..request.curve.points.len().min(request.curve.points_u.len()) {
                    // 原先要求本轮位置，再要求本轮tangent，之后才读旧采样下标。
                    let position = &mut positions[index];
                    let tangent = &mut request.tangents[index];
                    let (old_position, mut old_tangent) = if previous.points.len() == request.curve.points.len() {
                        (previous.points[index], previous.tangents[index])
                    } else {
                        let coord = old_curve.get_coord_at_u(request.curve.points_u[index]);
                        let a = previous.points[coord.segment];
                        let b = previous.points[coord.segment + 1];
                        let ta = previous.tangents[coord.segment];
                        let tb = previous.tangents[coord.segment + 1];
                        (a + (b - a) * coord.t, ta + (tb - ta) * coord.t)
                    };
                    let delta = *position - old_position;
                    *position = old_position + delta * factor;
                    if tangent.x * old_tangent.x + tangent.y * old_tangent.y < 0.0 { old_tangent = -old_tangent; }
                    let blended = old_tangent + (*tangent - old_tangent) * factor;
                    let reciprocal = (blended.x * blended.x + blended.y * blended.y).sqrt().recip();
                    *tangent = if reciprocal.is_finite() && reciprocal > 0.0 { blended * reciprocal } else { Vec2::X };
                    // 原比较插值前delta²，NaN有序 > 为false。
                    moved |= (delta.x * delta.x + delta.y * delta.y) + delta.z * delta.z
                        > f32::from_bits(0x3851_b717);
                }
                if !moved && previous.metadata.removed_support == metadata.removed_support
                    && previous.metadata.extra_10 == metadata.extra_10 && previous.metadata.extra_0c == metadata.extra_0c
                    && previous.metadata.extra_08 == metadata.extra_08 && previous.metadata.info_28 == metadata.info_28
                    && request.widths.len() == previous.widths.len()
                    && request.widths.iter().zip(&previous.widths)
                        .all(|(&new, &old)| !((new - old).abs() > f32::from_bits(0x38d1_b717)))
                    && USE_STAIRS_LAZINESS.load(std::sync::atomic::Ordering::Relaxed)
                    && visual.get(&StairGraphEdge::from(request.segment)).is_some_and(|state| !state.force_rebuild_requested())
                {
                    // 原同数取旧点clone；异数才接管当前插值点。
                    let output = if previous.points.len() == positions.len() { previous.points.clone() } else { positions };
                    request.interpolated_curve = utils::curve::Curve3::try_new_from_points(output, true).ok();
                    requests.previous.sampled_points.insert(identity, previous);
                    // 旧dirty不被reuse清除，第四历史Vec也原样保留。
                    continue;
                }
            }
        }
        // 原fresh先写history，随后CurveTry即使失败仍保留history和dirty。
        requests.previous.sampled_points.insert(identity, SampledStairsPoints {
            points: positions.clone(), tangents: request.tangents.clone(), widths: request.widths.clone(),
            auxiliary_words: Vec::new(), metadata,
        });
        request.interpolated_curve = utils::curve::Curve3::try_new_from_points(positions, true).ok();
        request.dirty = true;
    }
    // 未被本轮消费的旧identity在这里drop，移除消失边的四路旧采样。
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


#[cfg(test)]
mod interpolate_history_tests {
    use super::*;
    use bevy_ecs::system::RunSystemOnce;
    use country_core::resources::stairs::stairs_assembly_params::StairArchWallSupportType;
    use country_core::resources::terrain_height::{TerrainHeightsData, VisualTerrainHeightSsbo};
    use country_core::resources::walls::cached_decorator_transforms::{CachedDecoratorTransform, CachedDecoratorTransforms};
    use country_core::resources::walls::decorator::{DecoratorExtraInfo, DecoratorInfo, FlagExtra, StairExtra};
    use country_core::resources::walls::decorator_storage::DecoratorStorageAddr;
    use country_core::resources::walls::public_walls::PublicWalls;
    use country_core::systems::backend::input::translate_input::DemoModeEnabled;
    use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
    use std::sync::{Mutex, MutexGuard, atomic::Ordering};

    // 本模块所有实际 consumer case 共用此锁，离开作用域始终恢复原全局值。
    static LAZY_TEST_LOCK: Mutex<()> = Mutex::new(());
    struct LazyPolicyGuard {
        previous: bool,
        _lock: MutexGuard<'static, ()>,
    }
    impl LazyPolicyGuard {
        fn enabled() -> Self {
            let lock = LAZY_TEST_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            let previous = USE_STAIRS_LAZINESS.swap(true, Ordering::Relaxed);
            Self { previous, _lock: lock }
        }
    }
    impl Drop for LazyPolicyGuard {
        fn drop(&mut self) { USE_STAIRS_LAZINESS.store(self.previous, Ordering::Relaxed); }
    }

    fn generated_world(length: f32) -> World {
        let mut storage = DecoratorStorage::default();
        let mut cached = CachedDecoratorTransforms::default();
        let mut stairs = StairsState::new();
        for (id, x) in [(1, 0.0), (2, length)] {
            let mut info = DecoratorInfo {
                u20: 3 << 16,
                dst_wall: u64::from(x.to_bits()),
                ..DecoratorInfo::default()
            };
            let mut extra = [0; 20];
            extra[4..8].copy_from_slice(&1.0f32.to_le_bytes());
            info.set_extra(DecoratorExtraInfo::Stair(StairExtra(extra)));
            storage.apply_decorator_place(DecoratorStorageAddr::Terrain, DecoratorId(id), &info);
            cached.insert(id, CachedDecoratorTransform {
                rotation: glam::Quat::IDENTITY, translation: Vec3::new(x, 0.0, 0.0), scale: Vec3::ONE,
            });
            stairs.add_node(id);
        }
        stairs.add_edge(1, 2);
        let mut terrain = TerrainHeightsData::<VisualTerrainHeightSsbo>::new();
        terrain.set_height_range(-20.0, -20.0);
        let mut world = World::new();
        world.insert_resource(storage); world.insert_resource(cached); world.insert_resource(stairs);
        world.insert_resource(PublicWalls::default()); world.insert_resource(terrain);
        world.insert_resource(PlayermadeStairsAssemblyRequests::default());
        world.insert_resource(UiSignifierStream::default()); world.insert_resource(DemoModeEnabled(false));
        // 使用已恢复的真实上游请求系统，不在测试中捏造 Default Params。
        world.run_system_once(crate::decorator_visual::stairs::generate_playermade_stair_requests::<false>).unwrap();
        let requests = world.resource::<PlayermadeStairsAssemblyRequests>();
        assert_eq!(requests.requests.len(), 1);
        let edge = StairGraphEdge::from(requests.requests[0].segment);
        let mut visual = StairsVisualState::default(); visual.reset_edge(edge);
        world.insert_resource(visual); world.insert_resource(StairsRemovedSupports::default());
        world.insert_resource(GameTime { delta: 0.25, ..Default::default() });
        world
    }

    fn run(world: &mut World) {
        world.run_system_once(playermade_stairs_interpolate_and_compare).unwrap();
    }

    fn key(world: &World) -> StairGraphEdgeIdentity {
        StairGraphEdgeIdentity::from(world.resource::<PlayermadeStairsAssemblyRequests>().requests[0].segment)
    }

    fn seed_previous(world: &mut World) -> StairGraphEdgeIdentity {
        // 第一帧由真实 consumer 发布历史；只给下一帧设置明确的旧缓存输入。
        run(world);
        let id = key(world);
        let mut requests = world.resource_mut::<PlayermadeStairsAssemblyRequests>();
        requests.requests[0].dirty = false;
        requests.previous.sampled_points.get_mut(&id).unwrap().auxiliary_words = vec![0x8000_0000, 0x7fc1_2345, 19];
        id
    }

    #[test]
    fn fresh_history_publishes_real_segments_and_disappeared_samples_are_dropped() {
        let _policy = LazyPolicyGuard::enabled();
        let mut world = generated_world(1.0);
        let id = key(&world);
        run(&mut world);
        let owner = world.resource::<PlayermadeStairsAssemblyRequests>();
        let current = &owner.requests[0];
        assert!(current.dirty);
        assert!(current.interpolated_curve.is_some());
        let sample = &owner.previous.sampled_points[&id];
        assert_eq!(sample.points.len(), current.curve.points.len());
        assert_eq!(sample.widths, current.widths);
        assert!(sample.auxiliary_words.is_empty());
        assert_eq!(owner.previous.sub_segments[&id], current.sub_segments);
        let orphan = StairGraphEdgeIdentity::from(StairSegment::new(DecoratorId(7), DecoratorId(8)));
        let old_sample = sample.clone(); let old_segments = current.sub_segments.clone();
        let mut owner = world.resource_mut::<PlayermadeStairsAssemblyRequests>();
        owner.previous.sampled_points.insert(orphan, old_sample);
        owner.previous.sub_segments.insert(orphan, old_segments);
        owner.previous.arch_wall_supports.insert(orphan, StairArchWallSupportType::Kind1);
        owner.requests.clear();
        drop(owner);
        run(&mut world);
        let owner = world.resource::<PlayermadeStairsAssemblyRequests>();
        assert!(owner.previous.sampled_points.is_empty());
        assert!(owner.previous.sub_segments.contains_key(&id));
        assert!(owner.previous.sub_segments.contains_key(&orphan));
        assert_eq!(owner.previous.arch_wall_supports[&orphan], StairArchWallSupportType::Kind1);
    }

    #[test]
    fn reuse_same_count_keeps_old_geometry_and_all_four_history_vectors() {
        let _policy = LazyPolicyGuard::enabled();
        let mut world = generated_world(1.0); let id = seed_previous(&mut world);
        let old = world.resource::<PlayermadeStairsAssemblyRequests>().previous.sampled_points[&id].clone();
        for height in &mut world.resource_mut::<PlayermadeStairsAssemblyRequests>().requests[0].heights { *height += 0.001; }
        run(&mut world);
        let owner = world.resource::<PlayermadeStairsAssemblyRequests>();
        assert!(!owner.requests[0].dirty);
        assert_eq!(owner.requests[0].interpolated_curve.as_ref().unwrap().points, old.points);
        let preserved = &owner.previous.sampled_points[&id];
        assert_eq!(preserved.points, old.points); assert_eq!(preserved.tangents, old.tangents);
        assert_eq!(preserved.widths, old.widths); assert_eq!(preserved.auxiliary_words, old.auxiliary_words);
        assert_eq!(preserved.metadata, old.metadata);
        world.resource_mut::<PlayermadeStairsAssemblyRequests>().requests[0].dirty = true;
        run(&mut world);
        assert!(world.resource::<PlayermadeStairsAssemblyRequests>().requests[0].dirty);
        assert_eq!(world.resource::<PlayermadeStairsAssemblyRequests>().previous.sampled_points[&id].auxiliary_words, old.auxiliary_words);
    }

    #[test]
    fn reuse_different_position_count_moves_interpolated_geometry_but_preserves_old_history() {
        let _policy = LazyPolicyGuard::enabled();
        let mut world = generated_world(1.0); let id = seed_previous(&mut world);
        {
            let mut owner = world.resource_mut::<PlayermadeStairsAssemblyRequests>();
            assert_eq!(owner.requests[0].curve.points.len(), 2);
            let old = owner.previous.sampled_points.get_mut(&id).unwrap();
            old.points.insert(1, (old.points[0] + old.points[1]) * 0.5);
            old.tangents.insert(1, old.tangents[0]);
            // 四个Vec的长度独立；原比较并没有用points长度替代widths长度。
            for height in &mut owner.requests[0].heights { *height += 0.001; }
        }
        run(&mut world);
        let owner = world.resource::<PlayermadeStairsAssemblyRequests>();
        assert!(!owner.requests[0].dirty);
        let output = owner.requests[0].interpolated_curve.as_ref().unwrap();
        assert_eq!(output.points.len(), 2);
        let expected_y = 0.001 * utils::time_continuous_lerp_factor(0.25, 9.0);
        assert!((output.points[0].y - expected_y).abs() < 1e-7);
        let old = &owner.previous.sampled_points[&id];
        assert_eq!(old.points.len(), 3); assert_eq!(old.points[0].y, 0.0);
        assert_eq!(old.auxiliary_words, [0x8000_0000, 0x7fc1_2345, 19]);
    }

    #[test]
    fn resampling_uses_old_xz_distance_and_motion_compares_before_time_factor() {
        let _policy = LazyPolicyGuard::enabled();
        let mut world = generated_world(2.0); let id = seed_previous(&mut world);
        world.resource_mut::<GameTime>().delta = 0.0;
        {
            let mut owner = world.resource_mut::<PlayermadeStairsAssemblyRequests>();
            assert_eq!(owner.requests[0].curve.points.len(), 4);
            let old = owner.previous.sampled_points.get_mut(&id).unwrap();
            old.points = vec![Vec3::ZERO, Vec3::new(0.25, 10.0, 0.0), Vec3::new(2.0, 0.0, 0.0)];
            old.tangents = vec![Vec2::X; 3];
        }
        run(&mut world);
        let owner = world.resource::<PlayermadeStairsAssemblyRequests>();
        let output = owner.requests[0].interpolated_curve.as_ref().unwrap();
        let x = 2.0 / 3.0;
        assert!((output.points[1].x - x).abs() < 1e-6);
        assert!((output.points[1].y - (10.0 + (0.0 - 10.0) * ((x - 0.25) / 1.75))).abs() < 1e-5);
        assert!(owner.requests[0].dirty);
        assert!(owner.previous.sampled_points[&id].auxiliary_words.is_empty());
        // dt=0使实际位置不动；超阈值仍必须标脏，不能比较插值后的delta。
        for (difference, dirty) in [(0.007, false), (0.0071, true)] {
            let mut world = generated_world(1.0); let id = seed_previous(&mut world);
            world.resource_mut::<GameTime>().delta = 0.0;
            for height in &mut world.resource_mut::<PlayermadeStairsAssemblyRequests>().requests[0].heights { *height += difference; }
            run(&mut world);
            let owner = world.resource::<PlayermadeStairsAssemblyRequests>();
            assert_eq!(owner.requests[0].dirty, dirty);
            assert_eq!(owner.requests[0].interpolated_curve.as_ref().unwrap().points[0].y, 0.0);
            assert_eq!(owner.previous.sampled_points[&id].auxiliary_words.is_empty(), dirty);
        }
    }

    #[test]
    fn tangent_sign_fallback_and_width_nan_keep_original_ordered_gates() {
        let _policy = LazyPolicyGuard::enabled();
        for zero_tangent in [false, true] {
            let mut world = generated_world(1.0); let id = seed_previous(&mut world);
            {
                let mut owner = world.resource_mut::<PlayermadeStairsAssemblyRequests>();
                owner.requests[0].tangents.fill(if zero_tangent { Vec2::ZERO } else { Vec2::Y });
                owner.requests[0].widths[0] = f32::NAN;
                owner.previous.sampled_points.get_mut(&id).unwrap().tangents.fill(if zero_tangent { Vec2::ZERO } else { -Vec2::Y });
            }
            run(&mut world);
            let owner = world.resource::<PlayermadeStairsAssemblyRequests>();
            assert!(!owner.requests[0].dirty);
            assert!(owner.requests[0].tangents.iter().all(|&direction| direction == if zero_tangent { Vec2::X } else { Vec2::Y }));
            assert!(!owner.previous.sampled_points[&id].auxiliary_words.is_empty());
            assert_eq!(owner.previous.sampled_points[&id].widths[0], 1.0);
        }
        let tolerance = f32::from_bits(0x38d1_b717);
        for (difference, dirty) in [(tolerance, false), (f32::from_bits(tolerance.to_bits() + 1), true)] {
            let mut world = generated_world(1.0); let id = seed_previous(&mut world);
            {
                let mut owner = world.resource_mut::<PlayermadeStairsAssemblyRequests>();
                owner.requests[0].widths.fill(0.0);
                owner.requests[0].widths[0] = difference;
                owner.previous.sampled_points.get_mut(&id).unwrap().widths.fill(0.0);
            }
            run(&mut world);
            assert_eq!(world.resource::<PlayermadeStairsAssemblyRequests>().requests[0].dirty, dirty);
        }
    }

    fn edit_first_info(world: &mut World, edit: impl FnOnce(&mut DecoratorInfo)) {
        let mut storage = world.resource_mut::<DecoratorStorage>();
        edit(storage.lookup_decorator_info_mut(DecoratorId(1)).unwrap());
    }

    #[test]
    fn metadata_compares_raw_signed_zero_nan_payload_and_each_optional_field() {
        let _policy = LazyPolicyGuard::enabled();
        let mut world = generated_world(1.0);
        edit_first_info(&mut world, |info| info.u28 = 0x7fc1_2345);
        let id = seed_previous(&mut world);
        run(&mut world);
        assert!(!world.resource::<PlayermadeStairsAssemblyRequests>().requests[0].dirty);
        assert!(!world.resource::<PlayermadeStairsAssemblyRequests>().previous.sampled_points[&id].auxiliary_words.is_empty());
        edit_first_info(&mut world, |info| info.u28 = 0x7fc1_2346);
        run(&mut world);
        assert!(world.resource::<PlayermadeStairsAssemblyRequests>().requests[0].dirty);
        assert_eq!(world.resource::<PlayermadeStairsAssemblyRequests>().previous.sampled_points[&id].metadata.info_28, 0x7fc1_2346);
        for field in 0..5 {
            let mut world = generated_world(1.0); let id = seed_previous(&mut world);
            if field == 4 {
                world.resource_mut::<StairsRemovedSupports>().record(StairGraphEdge::new(1, 2));
            } else {
                edit_first_info(&mut world, |info| {
                    if field == 0 { info.u28 = 0x8000_0000; }
                    else {
                        let mut extra = info.extra().as_stair();
                        match field {
                            1 => extra.0[8..12].copy_from_slice(&0x8000_0000u32.to_le_bytes()),
                            2 => extra.0[12..16].copy_from_slice(&0x7fc1_2345u32.to_le_bytes()),
                            3 => extra.0[16] = 1,
                            _ => unreachable!(),
                        }
                        info.set_extra(DecoratorExtraInfo::Stair(extra));
                    }
                });
            }
            run(&mut world);
            let owner = world.resource::<PlayermadeStairsAssemblyRequests>();
            assert!(owner.requests[0].dirty);
            assert!(owner.previous.sampled_points[&id].auxiliary_words.is_empty());
        }
    }

    #[test]
    fn optional_missing_non_stair_and_unknown_raw_extra_keep_zero_metadata() {
        let _policy = LazyPolicyGuard::enabled();
        for mode in 0..3 {
            let mut world = generated_world(1.0); let id = seed_previous(&mut world);
            if mode == 0 {
                world.resource_mut::<DecoratorStorage>().apply_decorator_remove(DecoratorStorageAddr::Terrain, DecoratorId(1));
            } else {
                edit_first_info(&mut world, |info| {
                    if mode == 1 { info.set_extra(DecoratorExtraInfo::Flag(FlagExtra(91))); }
                    else { info.u40 = (info.u40 & !0xff00) | 0xff00; }
                });
            }
            run(&mut world);
            let owner = world.resource::<PlayermadeStairsAssemblyRequests>();
            assert!(!owner.requests[0].dirty);
            assert_eq!(owner.previous.sampled_points[&id].metadata,
                StairSamplesMetadata { info_28: 0, extra_08: 0, extra_0c: 0, removed_support: false, extra_10: 0 });
            assert!(!owner.previous.sampled_points[&id].auxiliary_words.is_empty());
        }
    }

    #[test]
    fn actual_visual_missing_force_rebuild_and_global_off_each_prevent_reuse() {
        let _policy = LazyPolicyGuard::enabled();
        for gate in 0..3 {
            let mut world = generated_world(1.0); let id = seed_previous(&mut world);
            let edge = StairGraphEdge::from(world.resource::<PlayermadeStairsAssemblyRequests>().requests[0].segment);
            match gate {
                0 => world.resource_mut::<StairsVisualState>().remove_edge(edge),
                1 => world.resource_mut::<StairsVisualState>().force_rebuild(edge),
                2 => USE_STAIRS_LAZINESS.store(false, Ordering::Relaxed),
                _ => unreachable!(),
            }
            run(&mut world);
            let owner = world.resource::<PlayermadeStairsAssemblyRequests>();
            assert!(owner.requests[0].dirty);
            assert!(owner.previous.sampled_points[&id].auxiliary_words.is_empty());
            USE_STAIRS_LAZINESS.store(true, Ordering::Relaxed);
        }
    }

    #[test]
    fn old_xz_curve_accepts_nan_y_and_reuse_preserves_opaque_history_when_xyz_output_fails() {
        let _policy = LazyPolicyGuard::enabled();
        let mut world = generated_world(1.0); let id = seed_previous(&mut world);
        {
            let mut owner = world.resource_mut::<PlayermadeStairsAssemblyRequests>();
            let old = owner.previous.sampled_points.get_mut(&id).unwrap();
            assert_eq!(old.points.len(), 2);
            old.points = vec![Vec3::new(0.0, f32::NAN, 0.0), Vec3::new(1.0, f32::NAN, 0.0)];
        }
        run(&mut world);
        let owner = world.resource::<PlayermadeStairsAssemblyRequests>();
        assert!(!owner.requests[0].dirty);
        assert!(owner.requests[0].interpolated_curve.is_none());
        let old = &owner.previous.sampled_points[&id];
        assert!(old.points.iter().all(|point| point.y.is_nan()));
        assert_eq!(old.auxiliary_words, [0x8000_0000, 0x7fc1_2345, 19]);
    }

    #[test]
    fn fresh_xyz_curve_failure_still_inserts_history_and_marks_dirty() {
        let _policy = LazyPolicyGuard::enabled();
        let mut world = generated_world(1.0); let id = key(&world);
        world.resource_mut::<PlayermadeStairsAssemblyRequests>().requests[0].heights.fill(f32::NAN);
        run(&mut world);
        let owner = world.resource::<PlayermadeStairsAssemblyRequests>();
        assert!(owner.requests[0].dirty);
        assert!(owner.requests[0].interpolated_curve.is_none());
        assert!(owner.previous.sampled_points[&id].points.iter().all(|point| point.y.is_nan()));
        assert!(owner.previous.sampled_points[&id].auxiliary_words.is_empty());
    }
}

#[cfg(test)]
mod ladder_support_tests {
    use super::*;
    use country_core::resources::stairs::stairs_assembly_params::{AlignKind, StairWallAnchor};
    use country_core::resources::walls::WallId;

    fn terrain(height: f32, degree: u8) -> (StairNodeAnchor, u8) {
        (StairNodeAnchor::Terrain { height }, degree)
    }
    fn roof(height: f32, degree: u8) -> (StairNodeAnchor, u8) {
        (StairNodeAnchor::Roof { height }, degree)
    }
    fn wall(degree: u8) -> (StairNodeAnchor, u8) {
        (StairNodeAnchor::Wall(StairWallAnchor::new(WallId(7), 0.0)), degree)
    }
    fn support(nodes: [(StairNodeAnchor, u8); 2], tags: [u8; 2]) -> u8 {
        determine_ladder_support(Vec3::ZERO, Vec3::Y, &nodes,
            &tags.map(|tag| AlignKind { tag, ..Default::default() }))
    }

    #[test]
    fn all_five_native_support_results_use_actual_anchors() {
        assert_eq!(support([terrain(0.0, 1), roof(0.0, 1)], [0, 0]), 0);
        assert_eq!(support([terrain(0.0, 1), roof(1.0, 1)], [0, 0]), 1);
        assert_eq!(support([wall(1), roof(1.0, 1)], [0, 0]), 2);
        assert_eq!(support([terrain(1.0, 1), roof(0.0, 1)], [0, 0]), 3);
        assert_eq!(support([terrain(1.0, 1), roof(1.0, 1)], [0, 0]), 4);
    }

    #[test]
    fn reversing_both_real_endpoints_preserves_the_support_result() {
        let nodes = [wall(1), roof(1.0, 1)];
        let aligns = [AlignKind::default(), AlignKind { tag: 1, ..Default::default() }];
        assert_eq!(determine_ladder_support(Vec3::ZERO, Vec3::Y, &nodes, &aligns), 2);
        assert_eq!(determine_ladder_support(Vec3::Y, Vec3::ZERO,
            &[nodes[1], nodes[0]], &[aligns[1], aligns[0]]), 2);
    }

    #[test]
    fn equal_and_unordered_position_heights_choose_the_second_lower_endpoint() {
        let nodes = [roof(1.0, 1), wall(1)];
        let aligns = [AlignKind::default(), AlignKind { tag: 1, ..Default::default() }];
        assert_eq!(determine_ladder_support(Vec3::ZERO, Vec3::Y, &nodes, &aligns), 3);
        for (from, to) in [(Vec3::ZERO, Vec3::ZERO),
            (Vec3::new(0.0, f32::NAN, 0.0), Vec3::ZERO),
            (Vec3::ZERO, Vec3::new(0.0, f32::NAN, 0.0))] {
            assert_eq!(determine_ladder_support(from, to, &nodes, &aligns), 4);
        }
    }

    #[test]
    fn ground_height_threshold_is_inclusive_and_nan_is_unsupported() {
        let threshold = f32::from_bits(0x3e05_1eb8);
        let above = f32::from_bits(threshold.to_bits() + 1);
        assert_eq!(support([terrain(threshold, 1), roof(threshold, 1)], [0, 0]), 0);
        assert_eq!(support([terrain(above, 1), roof(0.0, 1)], [0, 0]), 3);
        assert_eq!(support([terrain(0.0, 1), roof(above, 1)], [0, 0]), 1);
        assert_eq!(support([terrain(f32::NAN, 1), roof(0.0, 1)], [0, 0]), 3);
        assert_eq!(support([terrain(0.0, 1), roof(f32::NAN, 1)], [0, 0]), 1);
        assert_eq!(support([terrain(f32::NEG_INFINITY, 1), roof(0.0, 1)], [0, 0]), 0);
    }

    #[test]
    fn lower_wall_checks_tag_only_and_higher_wall_has_no_alignment_gate() {
        let nodes = [wall(1), wall(1)];
        let aligns = [AlignKind { tag: 0, flag: 1, byte_2: 1, ..Default::default() },
            AlignKind { tag: 1, ..Default::default() }];
        assert_eq!(determine_ladder_support(Vec3::ZERO, Vec3::Y, &nodes, &aligns), 0);
        assert_eq!(support(nodes, [1, 0]), 3);
        assert_eq!(support([terrain(0.0, 1), wall(1)], [0, 2]), 0);
    }

    #[test]
    fn final_lower_height_check_is_independent_of_the_degree_gate() {
        for degree in [0, 2, u8::MAX] {
            assert_eq!(support([terrain(0.0, degree), roof(1.0, 1)], [0, 0]), 1);
            assert_eq!(support([terrain(1.0, degree), roof(1.0, 1)], [0, 0]), 2);
            assert_eq!(support([terrain(f32::NAN, degree), roof(1.0, 1)], [0, 0]), 2);
            assert_eq!(support([wall(degree), roof(0.0, 1)], [1, 0]), 0);
        }
    }
}
