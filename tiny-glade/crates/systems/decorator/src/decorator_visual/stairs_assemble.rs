//! B `decorator_visual/stairs_assemble.rs` — 玩家楼梯装配。
//!
//! 本轮原证闭包：calculate_adjusted_ladder_positions 598B及相邻603B回调；
//! playermade_stairs_interpolate_and_compare 8622B及真实六资源wrapper。
//! 插值消费同一请求/历史/可视状态owner，保旧XZ与输出XYZ的不同坐标语义。
//! 本轮插值的原指令差分与CPU历史验证已通过；原宿主注册和整条装配仍须分别验收，不能据此
//! 宣称完整楼梯渲染链已挂载。原gen_stairs_structural_collider 955B的八点
//! 凸包及tiny-sphere分支沿用已有恢复；另已补五路真实懒订阅、dirty传播与原7资源结构图阶段。
//! 另已恢复梯子64B写面、五态支撑判别、完整碰撞/RT采样及墙侧支撑分段叶。
//! 主装配、梯步、楼层等其余标U的函数仍是明确缺口；叶级验收不能替代主循环接线。

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


/// 原1407de800完整167B：按上下端锚点、邻接度及下端对齐输出五种支撑状态。
/// 高度无序时按原UCOMISS/setbe选端，不把NaN当作严格大于。
pub fn determine_ladder_support(
    from: Vec3,
    to: Vec3,
    nodes: &[(StairNodeAnchor, u8); 2],
    alignments: &[country_core::resources::stairs::stairs_assembly_params::AlignKind; 2],
) -> u8 {
    let lower = usize::from(!(to.y > from.y));
    let upper = usize::from(to.y > from.y);
    let low = &nodes[lower];
    let high = &nodes[upper];
    let touches_floor = |anchor: &StairNodeAnchor| match anchor {
        StairNodeAnchor::Terrain { height } | StairNodeAnchor::Roof { height } => *height <= 0.13,
        StairNodeAnchor::Wall(_) => false,
    };
    let lower_supported = low.1 != 1 || match &low.0 {
        StairNodeAnchor::Wall(_) => alignments[lower].tag == 0,
        anchor => touches_floor(anchor),
    };
    let upper_supported = high.1 != 1 || match &high.0 {
        StairNodeAnchor::Wall(_) => true,
        anchor => touches_floor(anchor),
    };
    if !lower_supported { return 4 - u8::from(upper_supported); }
    if upper_supported { return 0; }
    if touches_floor(&low.0) { 1 } else { 2 }
}

/// 原结构图唯一碰撞增量订阅，不把全图扫描或函数指针Vec当事件源。
#[derive(Resource)]
pub struct StructuralGraphSubscription {
    subscriber: tabloid::Subscriber<country_core::systems::collision::world::ColliderDiff>,
}
impl StructuralGraphSubscription {
    /// 必须在该游戏世界首批碰撞发布之前构造；旧订阅被替换时由同一owner自然退出。
    pub fn new(world: &mut RaycastWorld) -> Self {
        Self { subscriber: world.mesh_intersect_collider_diff_publisher().subscribe() }
    }
}

/// 原1407d4990完整56B：ResMut<RaycastWorld>取得Publisher，订阅后Commands插入真实Resource。
pub fn structural_graph_subscribe(mut world: ResMut<RaycastWorld>, mut commands: Commands) {
    commands.insert_resource(StructuralGraphSubscription::new(&mut world));
}

/// 原1407deb60：None卸载该边，Some以恒等外变换更新对应owner。
/// 原记录为24字节source及Arc网格；None使用source判别17作为niche。
pub fn create_stairs_collider(
    world: &mut RaycastWorld,
    colliders: &mut std::collections::HashMap<StairGraphEdge,
        country_core::systems::collision::world::ColliderMeshInstance>,
    mesh: Option<country_core::systems::collision::world::RaycastMesh>,
    edge: StairGraphEdge,
) {
    use country_core::systems::collision::world::{remove_collider, upsert_collider};
    match mesh {
        None => { remove_collider(world, colliders, &edge); }
        Some(mesh) => {
            upsert_collider(world, colliders, edge, mesh.source, mesh.data,
                Vec3::ZERO, glam::Quat::IDENTITY);
        }
    }
}

/// 原5个真实发布者的独立订阅；资源退出时按Vec owner释放，不能以空listener代替。
#[derive(Resource)]
pub struct StairsLazySubscriber {
    subscriptions: Vec<tabloid::Subscriber<country_core::utils::dirty_tiles::DirtyTiles>>,
}

/// 原1407cf210：先订阅collision slots0/1/9，再物理与可视地形高度，最后插入资源。
pub fn stairs_subscribe_lazy(
    mut world: ResMut<RaycastWorld>,
    mut terrain: ResMut<country_core::resources::terrain_height::TerrainHeightsData>,
    mut visual_terrain: ResMut<country_core::resources::terrain_height::TerrainHeightsData<
        country_core::resources::terrain_height::VisualTerrainHeightSsbo>>,
    mut commands: Commands,
) {
    let mut subscriptions = Vec::new();
    for slot in [0, 1, 9] { subscriptions.push(world.subscribe_to_changes(slot)); }
    subscriptions.push(terrain.subscribe());
    subscriptions.push(visual_terrain.subscribe());
    commands.insert_resource(StairsLazySubscriber { subscriptions });
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

/// 原1407db3e0完整梯子：两侧主梁、条件支撑和内侧横档直接写同一木板分配。
/// 原writer返回的NoFreeChunks不影响后续RNG/几何推进，不改成重试或备用网格。
pub fn construct_ladder(
    writer: &mut country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<'_,
        country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>,
    positions: &[Vec3],
    widths: &[f32],
    support: u8,
    wall_id: u32,
    rng: &mut fastrand::Rng,
) {
    emit_ladder_pieces(positions, widths, support, wall_id, rng, |piece| {
        let _ = writer.push(&piece);
    });
}

/// 纯发射内核保留原执行顺序；生产回调直接写Sparse，验收回调只收集原64B记录。
fn emit_ladder_pieces(
    positions: &[Vec3], widths: &[f32], support: u8, wall_id: u32,
    rng: &mut fastrand::Rng,
    mut emit: impl FnMut(country_core::resources::roofs::roof_buffers::WoodenPlankSsbo),
) {
    use country_core::resources::{render::Affine3Packed, roofs::roof_buffers::WoodenPlankSsbo};
    // 原先读宽度两端，再读位置两端；空输入不得生成默认梯子。
    let start_width = *widths.first().expect("ladder first width") + -0.593;
    let end_width = *widths.last().expect("ladder last width") + -0.593;
    let start = *positions.first().expect("ladder first position");
    let end = *positions.last().expect("ladder last position");
    let delta = end - start;
    let horizontal_inverse = 1.0 / (delta.x * delta.x + delta.z * delta.z).sqrt();
    let normal = Vec3::new(-delta.z * horizontal_inverse, 0.0, delta.x * horizontal_inverse);
    let mut piece = |transform, flags| emit(WoodenPlankSsbo {
        transform, gate_seed: 0, piece_seed: rng.i32(-100..100), wall_id, flags,
    });
    for side in [0.5, -0.5] {
        let from = start + (normal * start_width) * side;
        let to = end + (normal * end_width) * side;
        let side_delta = to - from;
        let horizontal_length = (side_delta.x * side_delta.x + side_delta.z * side_delta.z).sqrt();
        let inverse = 1.0 / horizontal_length;
        let forward = Vec3::new(side_delta.x * inverse, 0.0, side_delta.z * inverse);
        piece(ladder_beam_transform(from, to, forward), 24);
        if support < 5 && (0x16_u32 >> support) & 1 != 0 {
            // 距较高端最多半个水平单位；等高时仍选择第二端作为高端。
            let fraction = 0.5_f32.min(0.5 / horizontal_length);
            let (lower, higher) = if from.y > to.y { (to, from) } else { (from, to) };
            let top = lower * fraction + higher * (1.0 - fraction);
            let foot = Vec3::new(top.x, from.y.min(to.y), top.z);
            piece(ladder_beam_transform(foot, top, forward), 24);
        }
    }
    let reverse_delta = start - end;
    let length = (reverse_delta.z * reverse_delta.z
        + (reverse_delta.y * reverse_delta.y + reverse_delta.x * reverse_delta.x)).sqrt();
    let count = ((length / 0.65).ceil() as usize).max(3);
    let start_width = start_width.max(0.4);
    let end_width = end_width.max(0.4);
    let from = start - (normal * start_width) * 0.5;
    let to = end - (normal * end_width) * 0.5;
    // 原perp的零分量仍经过乘减，不能替成固定-Y/固定-Z轴，负零也属记录合同。
    let perpendicular = Vec3::new(0.0 * normal.y - normal.z,
        normal.z * 0.0 - normal.x * 0.0, normal.x - 0.0 * normal.y);
    let x = ladder_cross(normal, perpendicular) * 0.16;
    let z = perpendicular * 0.4;
    for index in 0..count {
        let fraction = index as f32 / (count - 1) as f32;
        if fraction == 0.0 || fraction == 1.0 { continue; }
        let reverse = 1.0 - fraction;
        let width = start_width * reverse + end_width * fraction;
        piece(Affine3Packed::from_cols(x, normal * width, z,
            from * reverse + to * fraction), 25);
    }
}

/// 原两次叉积的标量顺序；不把斜梁厚度重新归一化，保留原坡度缩放。
fn ladder_cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3::new(a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x)
}

fn ladder_beam_transform(from: Vec3, to: Vec3, forward: Vec3)
    -> country_core::resources::render::Affine3Packed
{
    use country_core::resources::render::Affine3Packed;
    let delta = to - from;
    let length = (delta.z * delta.z + (delta.y * delta.y + delta.x * delta.x)).sqrt();
    let unit = delta * (1.0 / length);
    let x = ladder_cross(unit, forward);
    let z = ladder_cross(x, unit);
    // 原Y列先归一化再乘长度；直接用delta会在斜梁出现真实1ULP差异。
    Affine3Packed::from_cols(x * 0.222, unit * length, z * 0.222, from)
}

/// 0x140008c90 / 0x140018040（泛型 <true,false>）：玩家楼梯装配主流程。
///
/// **仍未完成，禁止注册为空生产阶段**：两原实例已按PDB范围排除EH函数并修复4张跳表，
/// 完整C/ASM保存在本轮m01-stairs-assemble-{true,false}-native证据中，工具超时不再是缺口。
/// 当前尚待恢复原请求类型完整尾部、reset返回缓存、梯步/楼层/栏杆和两阶段主分支；
/// 既有插值与本文件已验收的叶函数不能据此声明整条装配完成。
pub fn playermade_stairs_assemble<const X: bool>() {
    // U：主分支未完成；不以固定几何或空support生成伪造装配结果。
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

/// 原 1407cfe5a..1407cff72 的逐点内核；独立后仍由生产插值循环调用。
/// 保留 SSE 的 y+x 点积、z²+(y²+x²) 与原有限正倒数归一化门。
#[inline]
fn interpolate_stair_sample(
    position: &mut Vec3,
    tangent: &mut Vec2,
    old_position: Vec3,
    mut old_tangent: Vec2,
    factor: f32,
) -> bool {
    let delta = *position - old_position;
    *position = old_position + delta * factor;
    if tangent.y * old_tangent.y + tangent.x * old_tangent.x < 0.0 {
        old_tangent = -old_tangent;
    }
    let blended = old_tangent + (*tangent - old_tangent) * factor;
    let reciprocal = (blended.y * blended.y + blended.x * blended.x).sqrt().recip();
    *tangent = if reciprocal.is_finite() && reciprocal > 0.0 {
        blended * reciprocal
    } else {
        Vec2::X
    };
    delta.z * delta.z + (delta.y * delta.y + delta.x * delta.x)
        > f32::from_bits(0x3851_b717)
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
                    let (old_position, old_tangent) = if previous.points.len() == request.curve.points.len() {
                        (previous.points[index], previous.tangents[index])
                    } else {
                        let coord = old_curve.get_coord_at_u(request.curve.points_u[index]);
                        let a = previous.points[coord.segment];
                        let b = previous.points[coord.segment + 1];
                        let ta = previous.tangents[coord.segment];
                        let tb = previous.tangents[coord.segment + 1];
                        (a + (b - a) * coord.t, ta + (tb - ta) * coord.t)
                    };
                    // 原比较插值前 delta²，NaN 有序 > 为 false。
                    moved |= interpolate_stair_sample(position, tangent, old_position, old_tangent, factor);
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

/// 原1407d16b0：更新真实旧段表，消费角吸附操作的墙洞事件，再合并五路脏瓦片。
pub fn playermade_stairs_assembly_propagate_dirty_flags(
    mut subscriber: ResMut<StairsLazySubscriber>,
    (mut visual, mut requests): (ResMut<StairsVisualState>, ResMut<PlayermadeStairsAssemblyRequests>),
    _signifiers: Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,
    (mut events, mut recalculate, walls): (
        EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>,
        EventWriter<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallHoles>,
        Res<country_core::resources::walls::public_walls::PublicWalls>),
) {
    use country_core::resources::{walls::WallId, roofs::commands::RecalculateWallHoles};
    let requests = &mut *requests;
    requests.previous.sub_segments.extend(requests.requests.iter()
        .map(|request| (StairGraphEdgeIdentity::from(request.segment), request.sub_segments.clone())));
    for event in events.read() {
        let raw = &event.0;
        if raw[0x81] != b'?' { continue; }
        // 原先当前地址try_as_wall，再在disc<3的旧目的地址分支单独发事件；不可去重。
        let discriminant = u64::from_le_bytes(raw[..8].try_into().unwrap());
        let address_offset = if matches!(discriminant, 3 | 4) { 8 } else { 0 };
        let tag = u64::from_le_bytes(raw[address_offset..address_offset + 8].try_into().unwrap());
        let current = u64::from_le_bytes(raw[address_offset + 8..address_offset + 16].try_into().unwrap());
        let mut publish = |id| {
            let wall = WallId(id);
            if walls.get(wall).is_some_and(|state| state.geometry.metrics.constrained_extra <= 3.0) {
                recalculate.write(RecalculateWallHoles(wall));
            }
        };
        if tag == 0 { publish(current); }
        let disc = u32::from_le_bytes(raw[..4].try_into().unwrap());
        if disc < 3 && u64::from_le_bytes(raw[16..24].try_into().unwrap()) == 0 {
            publish(u64::from_le_bytes(raw[24..32].try_into().unwrap()));
        }
    }
    let mut dirty = country_core::utils::dirty_tiles::DirtyTiles::new_clear();
    for source in &mut subscriber.subscriptions {
        if let Some(publication) = source.take() { dirty.extend(&publication); }
    }
    // 原每个脏tile遍历视觉条目；空发布不会扫描视觉map或改变其ChangedTick。
    for index in dirty.tile_indices() {
        visual.force_rebuild_for_tile(country_core::LazyTile::from_index(index as u32));
    }
}

/// 原1407d1f80两组SystemParam：遍历所有边缓存，将24B切孔逐项发布为kind4。
pub fn playermade_stairs_add_wall_holes(
    visual: Res<StairsVisualState>,
    mut wall_holes: ResMut<country_core::resources::walls::wall_holes::WallHoles>,
) {
    use country_core::resources::walls::wall_holes::RawWallHole;
    for edge in visual.edges().values() {
        for cutout in &edge.wall_cutouts {
            wall_holes.add(cutout.wall, RawWallHole {center: cutout.center.to_array(),
                dimensions: cutout.dimensions.to_array(), kind: 4})
                .expect("楼梯墙洞必须在本帧WallHoles finalize之前发布");
        }
    }
}

/// 原1407d20c0：只重建dirty且具有插值曲线的请求，保留其它边的真实碰撞和图。
#[allow(clippy::too_many_arguments)]
pub fn create_structural_colliders_and_initial_graph(
    mut world: ResMut<RaycastWorld>,
    mut registry: ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,
    mut structural: ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,
    requests: Res<PlayermadeStairsAssemblyRequests>,
    removed: Res<StairsRemovedSupports>,
    stairs: Res<StairsState>,
    _signifiers: Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,
) {
    use country_core::resources::stairs::structural_graph::StructuralGraphConnection;
    for request in &requests.requests {
        if !request.dirty { continue; }
        let Some(curve) = &request.interpolated_curve else { continue; };
        // 原+60是widths长度；不是heights，也不能以zip截短掩盖损坏的请求。
        assert_eq!(request.widths.len(), request.terrain_contacts.len());
        let edge = StairGraphEdge::from(request.segment);
        let without_support = removed.contains(&edge);
        let mut start = 0usize;
        for sub in &request.sub_segments {
            let end = start.checked_add(usize::from(sub.count)).expect("stairs subsegment end overflow");
            let end_exclusive = end.checked_add(1).expect("stairs subsegment end overflow");
            // 原先按positions/tangents/widths/BitSlice顺序做闭区间检查，再发任何碰撞。
            let positions = &curve.points[start..end_exclusive];
            let directions = &request.tangents[start..end_exclusive];
            let widths = &request.widths[start..end_exclusive];
            let contacts = &request.terrain_contacts[start..end_exclusive];
            if sub.kind == 3 {
                assert_eq!(positions.len(), 2);
                let (a, b) = request.segment.nodes();
                let adjusted = calculate_adjusted_ladder_positions(&request.nodes,
                    positions[0], positions[1], &stairs, &requests.previous.sub_segments, [a, b]);
                // 原即使两端无地形接触，未移除支撑的边仍是tag15、height正零。
                let unsupported = without_support && !contacts[0] && !contacts[1];
                upsert_initial_structural_span(&mut world, &mut registry, &mut structural,
                    edge, start as u32, adjusted, [directions[0], directions[1]],
                    [widths[0], widths[1]], unsupported);
            } else {
                for index in 0..positions.len().saturating_sub(1) {
                    let unsupported = without_support && !contacts[index] && !contacts[index + 1];
                    upsert_initial_structural_span(&mut world, &mut registry, &mut structural,
                        edge, (start + index) as u32, [positions[index], positions[index + 1]],
                        [directions[index], directions[index + 1]], [widths[index], widths[index + 1]],
                        unsupported);
                }
            }
            start = end;
        }
        let count = curve.points.len().saturating_sub(1) as u32;
        if count != 0 {
            structural.parts_mut().segment_counts.insert(edge, count);
            // 原永久连接覆盖全部相邻sample索引；末端缺span时原helper自行跳过。
            for index in 0..count {
                structural.add_edge_by_span((edge, index), (edge, index + 1),
                    StructuralGraphConnection::new(false));
            }
        }
    }
    // 原独立第二阶段：所有当前图节点的邻边两两连接，不能只考虑本帧dirty请求。
    let mut neighbors = Vec::new();
    for node in stairs.graph().nodes() {
        neighbors.clear();
        neighbors.extend(stairs.graph().neighbors(node));
        for first in 0..neighbors.len() {
            for &other in &neighbors[first + 1..] {
                let a = StairGraphEdge::new(node, neighbors[first]);
                let b = StairGraphEdge::new(node, other);
                let endpoint = |edge: StairGraphEdge| {
                    if edge.nodes().0 == node { Some(0) }
                    else { structural.segment_counts().get(&edge).map(|count|
                        count.checked_sub(1).expect("called Option::unwrap() on zero stair segment count")) }
                };
                let Some(ia) = endpoint(a) else { continue; };
                let Some(ib) = endpoint(b) else { continue; };
                structural.add_edge_by_span((a, ia), (b, ib), StructuralGraphConnection::new(false));
            }
        }
    }
}

/// 原1407d2a8d/2dfe的source有效字段：tag15/16、sample+4、全边+8。
/// collider外变换原为(0,0.3,0)，结构高度只由移除支撑与两端接触决定。
fn upsert_initial_structural_span(
    world: &mut RaycastWorld,
    registry: &mut country_core::resources::stairs::stairs_collider::StairColliderRegistry,
    structural: &mut country_core::resources::stairs::structural_graph::StairsStructuralGraph,
    edge: StairGraphEdge,
    index: u32,
    positions: [Vec3; 2],
    directions: [Vec2; 2],
    widths: [f32; 2],
    unsupported: bool,
) {
    use country_core::systems::collision::world::{RaycastGeoSource, ColliderUpsert};
    let mut source = if unsupported { RaycastGeoSource::Slot16 } else { RaycastGeoSource::Slot15 };
    source.0[4..8].copy_from_slice(&index.to_le_bytes());
    let (a, b) = edge.nodes();
    source.0[8..12].copy_from_slice(&a.to_le_bytes()); source.0[12..16].copy_from_slice(&b.to_le_bytes());
    let result = upsert_structural_stair_segment(world, registry, (edge,index), source,
        positions[0],positions[1],directions[0],directions[1],widths[0],widths[1]);
    let (ColliderUpsert::Inserted { handle } | ColliderUpsert::Replaced { handle } | ColliderUpsert::Kept { handle }) = result;
    structural.upsert_span((edge, index), handle.into(), if unsupported { 1.0 } else { 0.0 });
}

/// 原1407d49d0：消费碰撞增量，重连临时支撑，传播结构距离并更新请求/历史。
/// 原六资源签名由140029600验证；3493行反编译尾部须分页读取，不能采用120KB截断件。
pub fn calculate_distortion_from_structural_graph(
    world: Res<RaycastWorld>,
    removed: Res<StairsRemovedSupports>,
    _signifiers: Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,
    mut subscription: ResMut<StructuralGraphSubscription>,
    mut structural: ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,
    mut requests: ResMut<PlayermadeStairsAssemblyRequests>,
) {
    use country_core::resources::stairs::structural_graph::{StairColliderHandle, StructuralGraphConnection};
    if let Some(diff) = subscription.subscriber.take() {
        let relevant = |slot: u8| (0x1a203u32 >> (slot & 31)) & 1 != 0;
        for &collider in &diff.removed {
            if relevant(collider.slot) { structural.remove_node_by_collider(collider.into()); }
        }
        for &collider in &diff.added {
            if relevant(collider.slot) { structural.disconnect_node_by_collider(collider.into()); }
        }
        let added: std::collections::HashSet<_> = diff.added.iter().copied().collect();
        for &handle in &diff.added {
            if !relevant(handle.slot) { continue; }
            let current = StairColliderHandle::from(handle);
            if handle.slot == 16 {
                let collider = world.get_collider(handle);
                let mesh = world.get_collider_mesh(collider.generation_low as u32, collider.generation);
                assert_eq!(mesh.source.0[0], 16);
                let edge = &mesh.source.0[8..16];
                world.iter_shape_overlaps_handle_filtered(handle, 0x1a203,
                    |other, source| {
                        if other.slot == 16 {
                            assert_eq!(source.0[0], 16);
                            if source.0[8..16] == *edge { return false; }
                        }
                        current < other.into() || !added.contains(&other)
                    },
                    |other, _| {
                        if other.slot.wrapping_sub(15) >= 2 {
                            structural.upsert_support_source_collider(other.into());
                        }
                        structural.add_edge_by_collider(current, other.into(), StructuralGraphConnection::new(true));
                        false
                    });
            } else {
                world.iter_shape_overlaps_handle_filtered(handle, 0x10000,
                    |other, _| current < other.into() || !added.contains(&other),
                    |other, _| {
                        structural.upsert_support_source_collider(current);
                        structural.add_edge_by_collider(current, other.into(), StructuralGraphConnection::new(true));
                        false
                    });
            }
        }
    }
    propagate_structural_distortion(&mut structural, &removed, &mut requests);
}

/// 原7d52bb后的算法只保留仍在图内的slot16脏端；永久支撑高度始终为零。
/// 失效沿“旧高度+3/64”的梯度扩散，再由边界最短路重新填充，不能全图重建。
fn propagate_structural_distortion(
    structural: &mut country_core::resources::stairs::structural_graph::StairsStructuralGraph,
    removed: &StairsRemovedSupports,
    requests: &mut PlayermadeStairsAssemblyRequests,
) {
    use country_core::resources::stairs::structural_graph::StructuralColliderSet;
    use ordered_float::OrderedFloat;
    use priority_queue::PriorityQueue;
    let parts = structural.parts_mut();
    parts.connections.retain(|collider| collider.slot == 16 && parts.graph.contains_node(*collider));
    let mut frontier: StructuralColliderSet = parts.connections.clone();
    let mut next = StructuralColliderSet::default();
    while !frontier.is_empty() {
        for &collider in &frontier {
            if collider.slot == 16 { parts.collider_to_height.insert(collider, 1.0); }
            let Some(&old) = parts.working_heights.get(&collider) else { continue; };
            let value = old + 0.046875;
            let value = if value >= 1.0 { 1.0 } else { value };
            for neighbor in parts.graph.neighbors(collider) {
                if !parts.connections.contains(&neighbor) && !frontier.contains(&neighbor)
                    && parts.working_heights.get(&neighbor).is_some_and(|&height| height == value)
                {
                    next.insert(neighbor);
                }
            }
        }
        parts.connections.extend(next.iter().copied());
        frontier.clear();
        std::mem::swap(&mut frontier, &mut next);
    }
    let mut queue = PriorityQueue::new();
    for &collider in parts.connections.iter() {
        for neighbor in parts.graph.neighbors(collider) {
            if neighbor.slot == 16 {
                if parts.connections.contains(&neighbor) { continue; }
                if let Some(&height) = parts.collider_to_height.get(&neighbor) {
                    queue.push(neighbor, OrderedFloat(-height));
                }
            } else {
                queue.push(neighbor, OrderedFloat(0.0));
            }
        }
    }
    let mut affected = hashbrown::HashSet::new();
    for collider in parts.connections.iter() {
        if let Some(&(edge, _)) = parts.collider_to_span.get(collider) { affected.insert(edge); }
    }
    while let Some((collider, OrderedFloat(priority))) = queue.pop() {
        let value = 0.046875 - priority;
        let value = if value >= 1.0 { 1.0 } else { value };
        for neighbor in parts.graph.neighbors(collider) {
            match parts.collider_to_height.entry(neighbor) {
                hashbrown::hash_map::Entry::Occupied(mut entry) => {
                    if value < *entry.get() {
                        *entry.get_mut() = value;
                        queue.push(neighbor, OrderedFloat(-value));
                    }
                }
                hashbrown::hash_map::Entry::Vacant(entry) => {
                    entry.insert(value);
                    queue.push(neighbor, OrderedFloat(-value));
                }
            }
            if let Some(&(edge, _)) = parts.collider_to_span.get(&neighbor) { affected.insert(edge); }
        }
    }
    for request in &mut requests.requests {
        let edge = StairGraphEdge::from(request.segment);
        let identity = StairGraphEdgeIdentity::from(request.segment);
        if !affected.contains(&edge) && !removed.contains(&edge) {
            if let Some(old) = requests.previous.sampled_points.get_mut(&identity) {
                old.auxiliary_words.clear();
            }
            continue;
        }
        request.auxiliary_samples.resize(request.curve.points.len(), 1.0);
        for (index, sample) in request.auxiliary_samples.iter_mut().enumerate() {
            for span_index in [index as u32, (index as u32).wrapping_sub(1)] {
                if let Some(collider) = parts.span_to_collider.get(&(edge, span_index)) {
                    if let Some(&height) = parts.collider_to_height.get(collider) {
                        *sample = structural_height_min(*sample, height);
                    }
                }
            }
            *sample = structural_distortion_factor(*sample);
        }
        let old = requests.previous.sampled_points.get_mut(&identity)
            .expect("structural distortion requires interpolated sample history");
        if old.auxiliary_words.len() != request.auxiliary_samples.len()
            || !old.auxiliary_words.iter().zip(&request.auxiliary_samples)
                .all(|(&word, &height)| f32::from_bits(word) == height)
        {
            request.dirty = true;
            old.auxiliary_words.clear();
            old.auxiliary_words.extend(request.auxiliary_samples.iter().map(|height| height.to_bits()));
        }
    }
    parts.working_heights.clone_from(parts.collider_to_height);
    parts.connections.clear();
}

/// 原7d7600..7d7648；按乘法顺序保留f32舍入，NaN经MAXSS/MINSS返回NaN。
fn structural_distortion_factor(height: f32) -> f32 {
    if height < 0.45 { return 0.0; }
    let t = (height + -0.45) / 0.55;
    let t = if t < 0.0 { 0.0 } else { t };
    let t = if t > 1.0 { 1.0 } else { t };
    let square = t * t;
    ((3.0 - (t + t)) * square) * 0.3 + 0.7
}

/// 原MINSS相等时取当前采样，额外掩码仅在采样NaN时取支撑高度。
fn structural_height_min(sample: f32, height: f32) -> f32 {
    if sample.is_nan() || height < sample { height } else { sample }
}

/// 原1407d94a0输出三张有序表；区间两端都是采样下标，交界点按原共享。
#[derive(Debug, PartialEq)]
pub struct StairSupportPartition {
    pub unsupported: Vec<std::ops::RangeInclusive<usize>>,
    pub supported: Vec<std::ops::RangeInclusive<usize>>,
    pub distances: Vec<Vec<f32>>,
}

/// 原1407d94a0：在墙法线侧逐内部采样探测真实World，保持支撑区间和逐点距离。
pub fn split_segment_into_supported_and_unsupported_subsegments(
    world: &RaycastWorld,
    curve: &utils::curve::Curve3,
    tangents: &[Vec2],
    widths: &[f32],
    anchor: country_core::resources::stairs::stairs_assembly_params::StairWallAnchor,
) -> StairSupportPartition {
    split_stair_support_samples(&curve.points, tangents, widths, || anchor.wall_normal(),
        |origin, direction| world.raycast_simple_with_max_toi(origin, direction, 1.78, 1))
}

fn split_stair_support_samples(
    positions: &[Vec3], tangents: &[Vec2], widths: &[f32],
    mut wall_normal: impl FnMut() -> Vec2,
    mut cast: impl FnMut(Vec3, Vec3) -> country_core::systems::collision::world::RaycastSimpleResult,
) -> StairSupportPartition {
    let first_width = widths[0];
    let last = positions.len().checked_sub(1).expect("stair support positions");
    let mut output = StairSupportPartition { unsupported: Vec::new(), supported: Vec::new(),
        distances: vec![vec![first_width * 0.5]] };
    let mut supported_start = 0;
    let mut unsupported_start = None;
    for index in 1..last {
        // 原先求墙法线，再读切线与两路带界检查的采样；不预先zip截断。
        let normal = wall_normal();
        let tangent = tangents[index];
        let sign = if normal.x * tangent.y < normal.y * tangent.x { -1.0 } else { 1.0 };
        let signed = Vec3::new(-tangent.y, 0.0, tangent.x) * sign;
        let reciprocal = 1.0 / (signed.z * signed.z
            + (signed.y * signed.y + signed.x * signed.x)).sqrt();
        let direction = if reciprocal.is_finite() && reciprocal > 0.0 {
            signed * reciprocal
        } else { Vec3::ZERO };
        let position = positions[index];
        let width = widths[index];
        let origin = Vec3::new(
            position.x + (direction.x * width) * -0.5,
            (position.y + -1.17) + (direction.y * width) * -0.5,
            (direction.z * width) * -0.5 + position.z,
        );
        let hit = cast(origin, direction);
        if hit.is_miss() {
            if unsupported_start.is_none() {
                output.supported.push(supported_start..=index - 1);
                unsupported_start = Some(index - 1);
            }
        } else {
            let dx = origin.x - hit.position.x;
            let dz = origin.z - hit.position.z;
            let distance = width * -0.5 + (dz * dz + dx * dx).sqrt();
            if let Some(start) = unsupported_start.take() {
                output.unsupported.push(start..=index);
                supported_start = index;
                output.distances.push(vec![distance]);
            } else {
                output.distances.last_mut().unwrap().push(distance);
            }
        }
    }
    if let Some(start) = unsupported_start {
        output.unsupported.push(start..=last);
    } else {
        output.supported.push(supported_start..=last);
        output.distances.last_mut().unwrap().push(widths.last().unwrap() * 0.5);
    }
    output
}

/// 原梯步四路借用采样；原迭代器按最短输入zip，但预读位置前两点。
#[derive(Clone, Copy)]
pub struct StairStepSamples<'a> {
    pub positions: &'a [Vec3],
    pub widths: &'a [f32],
    pub tangents: &'a [Vec2],
    pub auxiliary_samples: &'a [f32],
}

/// 原两个端标志与三个独立选项，不用模式默认值代替caller的分支结果。
#[derive(Clone, Copy)]
pub struct StairStepOptions {
    pub exclude_end_extra: [bool; 2],
    pub inset_width: bool,
    pub minimum_width: f32,
    /// 原最后bool：同时减半高度，并跳过aux均值小于.1的内部层。
    pub half_height: bool,
}

/// 原1407da0b0：原terrain采样、随机宽度分割、逐砖变形和96B稀疏写面。
pub fn construct_stair_steps(
    writer: &mut country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<'_,
        country_core::geometry::instanced_wall::BrickTransformSsbo>,
    samples: StairStepSamples<'_>,
    options: StairStepOptions,
    terrain: &country_core::resources::terrain_height::TerrainHeightsData,
    source_id: u32,
    rng: &fastrand::Rng,
) {
    emit_stair_steps(samples, options, source_id, rng,
        |point| terrain.sample_at_world_xz(point.x, point.y),
        |brick| { let _ = writer.push(&brick); });
}

fn emit_stair_steps(
    samples: StairStepSamples<'_>, options: StairStepOptions, source_id: u32,
    rng: &fastrand::Rng, mut sample_terrain: impl FnMut(Vec2) -> f32,
    mut emit: impl FnMut(country_core::geometry::instanced_wall::BrickTransformSsbo),
) {
    use country_core::geometry::instanced_wall::{BrickSourceId, BrickTransformSsbo};
    let positions = samples.positions;
    // 原不允许少于两位置点，不能因其它zip输入短而跳过该检查。
    let first = positions[0]; let second = positions[1];
    let first_delta = first - second;
    let extra_height = first_delta.y.abs();
    let front_samples = (0.56 / (first_delta.z * first_delta.z
        + first_delta.x * first_delta.x).sqrt()).floor() as usize;
    let end_threshold = positions.len().saturating_sub(front_samples.wrapping_add(2));
    let last_height = positions.last().unwrap().y;
    let count = positions.len().min(samples.widths.len()).min(samples.tangents.len())
        .min(samples.auxiliary_samples.len());
    let material = BrickSourceId::from_color_id(country_core::resources::color_id::ColorId(source_id)).0;
    for index in 0..count.saturating_sub(1) {
        let a = positions[index]; let b = positions[index + 1];
        let mut height = (b.y - a.y).abs().max(0.6);
        let extra = if first.y < last_height && !options.exclude_end_extra[1] && index > end_threshold
            || first.y > last_height && !options.exclude_end_extra[0] && index < front_samples
        { extra_height } else { 0.0 };
        height += extra;
        if options.half_height { height *= 0.5; }
        let point = Vec2::new(b.x * 0.5 + a.x * 0.5, b.z * 0.5 + a.z * 0.5);
        let top = (sample_terrain(point) + 0.15).max(a.y.max(b.y));
        let offset = Vec3::Y * (height * -0.5);
        let base_a = Vec3::new(a.x, top, a.z) + offset;
        let base_b = Vec3::new(b.x, top, b.z) + offset;
        let normal_a = stair_step_normal(samples.tangents[index]);
        let normal_b = stair_step_normal(samples.tangents[index + 1]);
        let inset = if options.inset_width { 0.25 } else { 0.0 };
        let width_a = (samples.widths[index] - inset).max(options.minimum_width);
        let width_b = (samples.widths[index + 1] - inset).max(options.minimum_width);
        let mean_width = width_a * 0.5 + width_b * 0.5;
        let factor = utils::scalar::inverse_lerp(2.0, 4.0, mean_width).clamp(0.0, 1.0);
        let mut jitter = 0.0;
        let parts = if mean_width >= 0.0 && mean_width < 0.66 { 1 }
        else if mean_width >= 0.66 && mean_width < 1.2 {
            // 原是u64高位的两项整数采样，Rng::bool使用其它位，不能替换。
            rng.usize(1..=2)
        } else {
            let random = rng.f32().clamp(0.0, 1.0);
            let smooth = (3.0 - (random + random)) * (random * random);
            let denominator = smooth * (factor * 1.3 + (1.0 - factor) * 0.7) + 0.46;
            jitter = (denominator * 0.3).max(0.32);
            ((mean_width / denominator).ceil() as usize).max(1)
        };
        let splits = utils::random_splits(parts.wrapping_add(1), jitter / mean_width, rng);
        let amplitude_random = rng.f32();
        let auxiliary = samples.auxiliary_samples[index] * 0.5
            + samples.auxiliary_samples[index + 1] * 0.5;
        if options.half_height && auxiliary < 0.1 { continue; }
        let left_a = base_a - (normal_a * width_a) * 0.5;
        let right_a = base_a + (normal_a * width_a) * 0.5;
        let left_b = base_b - (normal_b * width_b) * 0.5;
        let right_b = base_b + (normal_b * width_b) * 0.5;
        // 先平均两端深度中线，再沿宽度分割。交换两次lerp会使曲段真实相差ULP。
        let left = left_a * 0.5 + left_b * 0.5;
        let right = right_a * 0.5 + right_b * 0.5;
        let delta = left - right;
        let inverse = 1.0 / (delta.z * delta.z + delta.x * delta.x).sqrt();
        let direction = if inverse.is_finite() && inverse > 0.0 {
            Vec2::new(-delta.z * inverse, delta.x * inverse)
        } else { Vec2::ZERO };
        // 原caller规范化后，helper仍再次规范化，不能省略任何一层。
        let rotation = utils::xz_rotation_mat3_bad(direction);
        let depths = [stair_step_length(left_a - left_b), stair_step_length(right_a - right_b)];
        let vertical = (a.y - b.y).abs() * 0.8;
        let upper = vertical.min(factor * 0.3 + (1.0 - factor) * 0.15);
        let lower = vertical.min(factor * 0.15 + (1.0 - factor) * 0.05);
        let amplitude = upper * amplitude_random + (1.0 - amplitude_random) * lower;
        let variation = auxiliary * 0.42;
        for (piece_index, pair) in splits.windows(2).enumerate() {
            let [s0, s1] = [pair[0], pair[1]];
            let scale = 1.0 - rng.f32() * variation;
            let from = left * (1.0 - s0) + right * s0;
            let to = left * (1.0 - s1) + right * s1;
            let span = to - from;
            let width = (span.y * span.y + (span.z * span.z + span.x * span.x)).sqrt() * 1.13 * scale;
            let columns = [rotation.x_axis * width, rotation.y_axis * (height * scale),
                rotation.z_axis * scale, from * 0.5 + to * 0.5];
            let deform_a = rng.f32() * 0.05 - (1.0 - ((s0 - 0.5).abs() + (s0 - 0.5).abs())) * amplitude;
            let deform_b = rng.f32() * 0.05 - (1.0 - ((s1 - 0.5).abs() + (s1 - 0.5).abs())) * amplitude;
            let deformation = if index == 0 || index == positions.len() - 2 { [0.0; 4] }
                else { [deform_b, deform_a, deform_b, deform_a] };
            emit(BrickTransformSsbo { affine_columns: columns.map(|column| column.to_array()), deformation,
                surface: [depths[0] * (1.0 - s0) + depths[1] * s0,
                    depths[0] * (1.0 - s1) + depths[1] * s1, 0.0],
                seed: (index as u32).wrapping_add(piece_index as u32), material,
                wall_distance: [s0 * mean_width, s1 * mean_width], flags: 0x60 });
        }
    }
}

fn stair_step_normal(tangent: Vec2) -> Vec3 {
    let inverse = 1.0 / (tangent.x * tangent.x + tangent.y * tangent.y).sqrt();
    Vec3::new(-tangent.y * inverse, 0.0, tangent.x * inverse)
}

fn stair_step_length(vector: Vec3) -> f32 {
    (vector.z * vector.z + (vector.y * vector.y + vector.x * vector.x)).sqrt()
}

/// 原1407dc170：真实楼层双面裁剪、meshlet构建与实体所有权；构建失败仅记录原错误。
pub fn gen_stair_floors<D: country_core::render::ssbo::rhapsody_boundary::GraphicsDevice + Send + Sync + 'static>(
    device: std::sync::Arc<D>,
    widths: &[f32], tangents: &[Vec2], positions: &[Vec3],
    entities: &mut Vec<Entity>, commands: &mut Commands,
    source: u32, endpoint_adjustments: &[Option<f32>; 2], auxiliary: &[f32],
) {
    use country_core::{components::drawable::{NaniBundle, NaniInstanceMeshDesc},
        render::nani::{trimesh::Trimesh, instance_data::StairsFloorInstanceData,
            gpu_world::{InstanceDataMarker, GpuWorldSubsetKind}, systems::NaniInstanceDesc},
        resources::stairs::visual::{StairsFloorMeshChunk, StairsFloorBuffer}};
    let geometry = stair_floor_geometry(widths, tangents, positions, endpoint_adjustments, auxiliary);
    let mesh = Trimesh::builder(&geometry.indices).positions(&geometry.positions).attrib(&geometry.normals)
        .build(device.as_ref(), "playermade stairs");
    let Trimesh { chunks, buffer } = match mesh {
        Ok(mesh) => mesh,
        Err(error) => { log::error!(target:"system_decorator::decorator_visual::stairs_assemble", "Error spawning stair mesh: {error}"); return; }
    };
    let owner = std::sync::Arc::new(StairsFloorBuffer::new(device, buffer));
    for chunk in chunks {
        let part = floor_tri_count_to_atlas_idx(u32::from(chunk.index_count()) / 3);
        entities.push(commands.spawn((NaniBundle::<StairsFloorInstanceData> {
            desc: NaniInstanceDesc { mesh: NaniInstanceMeshDesc::AtlasPart(part), subset_kind: GpuWorldSubsetKind::StairsFloor },
            transform: country_core::utils::components::transform::Transform::identity(),
            marker: InstanceDataMarker::default(),
        }, StairsFloorMeshChunk { buffer: owner.clone(), chunk, source })).id());
    }
}

#[derive(Debug)]
struct StairFloorGeometry { indices: Vec<u32>, positions: Vec<Vec3>, normals: Vec<Vec3> }

fn stair_floor_geometry(widths: &[f32], tangents: &[Vec2], positions: &[Vec3], endpoint_adjustments: &[Option<f32>; 2], auxiliary: &[f32]) -> StairFloorGeometry {
    let result = stair_floor_geometry_inner::<false>(widths, tangents, positions, endpoint_adjustments, auxiliary);
    if result.positions.iter().chain(&result.normals).any(|p| !p.is_finite()) {
        stair_floor_geometry_inner::<true>(widths, tangents, positions, endpoint_adjustments, auxiliary)
    } else { result }
}

/// 原几何阶段不补侧面或端盖：两层带状面沿auxiliary=0.15的交点剪裁，各自保持上一对索引。
fn stair_floor_geometry_inner<const SSE: bool>(widths: &[f32], tangents: &[Vec2], positions: &[Vec3], endpoint_adjustments: &[Option<f32>; 2], auxiliary: &[f32]) -> StairFloorGeometry {
    #[inline]
    fn original<const SSE: bool>(a: f32, b: f32, result: f32) -> f32 {
        if !SSE { return result; }
        if a.is_nan() { f32::from_bits(a.to_bits() | 0x0040_0000) }
        else if b.is_nan() { f32::from_bits(b.to_bits() | 0x0040_0000) }
        else if result.is_nan() { f32::from_bits(0xffc0_0000) }
        else { result }
    }
    let add = |a: f32, b: f32| original::<SSE>(a, b, a + b);
    let sub = |a: f32, b: f32| original::<SSE>(a, b, a - b);
    let mul = |a: f32, b: f32| original::<SSE>(a, b, a * b);
    let div = |a: f32, b: f32| original::<SSE>(a, b, a / b);
    assert!(positions.len() >= 2, "原楼层生成要求至少两个位置");
    let correction = endpoint_adjustments.map(|value| value.map_or(0.0, |height| {
        // 原1407dc218/241 MAXSS的第二操作数为+0，无序和相等均选它；C伪码的<=会漏NaN。
        let offset = sub(0.15, height); if offset > 0.0 { offset } else { 0.0 }
    }));
    let reciprocal = div(1.0, (positions.len() - 1) as f32);
    let points: Vec<_> = positions.iter().enumerate().map(|(index, point)| {
        let u = mul(index as f32, reciprocal);
        let offset = add(mul(u, correction[1]), mul(sub(1.0, u), correction[0]));
        Vec3::new(add(mul(offset, 0.0), point.x), add(offset, point.y), add(point.z, mul(offset, 0.0)))
    }).collect();
    let face_normal = |from: Vec3, to: Vec3| {
        let d = Vec3::from_array(std::array::from_fn(|i| sub(to[i], from[i])));
        let left = Vec3::new(-d.z, 0.0, d.x);
        let cross = Vec3::new(sub(mul(left.z, d.y), mul(d.z, left.y)),
            sub(mul(left.x, d.z), mul(d.x, left.z)), sub(mul(d.x, left.y), mul(d.y, left.x)));
        let squared = add(mul(cross.z, cross.z), add(mul(cross.y, cross.y), mul(cross.x, cross.x)));
        let length = original::<SSE>(squared, 0.0, squared.sqrt());
        let inverse = div(1.0, length);
        if inverse.is_finite() && inverse > 0.0 {
            Vec3::new(mul(inverse, -cross.x), mul(inverse, -cross.y), mul(-cross.z, inverse))
        } else { Vec3::splat(-0.0) }
    };
    let mut normals = Vec::with_capacity(points.len());
    normals.push(face_normal(points[0], points[1]));
    for window in points.windows(3) { normals.push(face_normal(window[0], window[2])); }
    // 原末端方向反向，代数上同面但有符号零不同，不能一律改成向前差分。
    normals.push(face_normal(points[points.len()-1], points[points.len()-2]));
    let mut output = StairFloorGeometry { indices: Vec::with_capacity(points.len() * 12), positions: Vec::with_capacity(points.len() * 4), normals: Vec::with_capacity(points.len() * 4) };
    let mut previous_cut = sub(0.15, *auxiliary.first().expect("原楼层生成要求auxiliary首项"));
    let mut previous_left = Vec3::ZERO; let mut previous_right = Vec3::ZERO; let mut previous_normal = Vec3::ZERO;
    let mut top_pair = None; let mut bottom_pair = None;
    let append_pair = |output: &mut StairFloorGeometry, pair: [Vec3; 2], normal: Vec3| {
        let base = output.positions.len() as u32; output.positions.extend(pair); output.normals.extend([normal;2]); [base, base+1]
    };
    let append_quad = |indices: &mut Vec<u32>, previous: [u32; 2], current: [u32; 2]| indices.extend([previous[0],previous[1],current[0],current[0],previous[1],current[1]]);
    let lower = |point: Vec3| Vec3::new(point.x, add(point.y, -0.2), point.z);
    for (((point, normal), (width, tangent)), auxiliary) in points.iter().zip(&normals).zip(widths.iter().zip(tangents)).zip(auxiliary) {
        let cut = sub(0.15, *auxiliary);
        let lateral = Vec3::new(-tangent.y, 0.0, tangent.x).map(|v| mul(mul(*width, v), 0.5));
        let left = Vec3::from_array(std::array::from_fn(|i| sub(point[i], lateral[i])));
        let right = Vec3::from_array(std::array::from_fn(|i| add(lateral[i], point[i])));
        let inside = cut > 0.0;
        if (previous_cut > 0.0) != inside {
            let t = div(previous_cut, sub(previous_cut, cut)); let complement = sub(1.0, t);
            let interpolate = |before: Vec3, current: Vec3| Vec3::from_array(std::array::from_fn(|i| add(mul(t,current[i]),mul(before[i],complement))));
            let crossing_left = interpolate(previous_left,left); let crossing_right = interpolate(previous_right,right);
            let crossing_normal = Vec3::new(add(mul(t,normal.x),mul(previous_normal.x,complement)), add(mul(t,normal.y),mul(previous_normal.y,complement)), add(mul(previous_normal.z,complement),mul(normal.z,t)));
            let current_top = [output.positions.len() as u32,output.positions.len() as u32+1];
            if let Some(previous) = top_pair {append_quad(&mut output.indices,previous,current_top);}
            append_pair(&mut output,[crossing_left,crossing_right],crossing_normal);
            let current_bottom = [output.positions.len() as u32,output.positions.len() as u32+1];
            if let Some(previous) = bottom_pair {append_quad(&mut output.indices,previous,current_bottom);}
            append_pair(&mut output,[lower(crossing_right),lower(crossing_left)],-crossing_normal);
            top_pair = inside.then_some(current_top); bottom_pair = inside.then_some(current_bottom);
        }
        if inside {
            let current_top = [output.positions.len() as u32,output.positions.len() as u32+1];
            if let Some(previous) = top_pair {append_quad(&mut output.indices,previous,current_top);}
            top_pair = Some(append_pair(&mut output,[left,right],*normal));
            let current_bottom = [output.positions.len() as u32,output.positions.len() as u32+1];
            if let Some(previous) = bottom_pair {append_quad(&mut output.indices,previous,current_bottom);}
            bottom_pair = Some(append_pair(&mut output,[lower(right),lower(left)],-*normal));
        }
        previous_left=left; previous_right=right; previous_normal=*normal; previous_cut=cut;
    }
    output
}

#[cfg(test)]
mod native_stair_floor_recovery_tests {
    use super::*;
    use std::sync::{Arc, Mutex, atomic::{AtomicU64, Ordering}};
    use country_core::render::ssbo::rhapsody_boundary::{GraphicsDevice, Buffer, BufferDesc, BackendError};
    #[derive(Default)]
    struct RecordingDevice {
        next: AtomicU64,
        buffers: Mutex<std::collections::HashMap<u64, Vec<u8>>>,
        released: Mutex<Vec<u64>>,
        writes: Mutex<Vec<usize>>,
        fail_creation: bool,
    }
    impl GraphicsDevice for RecordingDevice {
        fn create_buffer(&self, desc: &BufferDesc, _: &str, data: Option<&[u8]>) -> Result<Buffer, BackendError> {
            if self.fail_creation { return Err(BackendError::invalid_usage("原floor构建失败路径的明确测试设备错误")); }
            let id=self.next.fetch_add(1,Ordering::Relaxed)+1;
            let mut bytes=vec![0;desc.size]; if let Some(data)=data {bytes.copy_from_slice(data);}
            self.buffers.lock().unwrap().insert(id,bytes);
            Ok(Buffer {id,byte_size:desc.size as u64,..Default::default()})
        }
        fn release_buffer(&self, buffer:&Buffer) {
            assert!(self.buffers.lock().unwrap().remove(&buffer.id).is_some(),"floor物理回执只释放一次");
            self.released.lock().unwrap().push(buffer.id);
        }
        fn update_buffer_bytes(&self, buffer:&Buffer, data:&[u8], ranges:&[std::ops::Range<usize>], offset:usize) -> Result<(),BackendError> {
            let mut buffers=self.buffers.lock().unwrap(); let out=buffers.get_mut(&buffer.id).unwrap();
            for range in ranges {out[offset+range.start..offset+range.end].copy_from_slice(&data[range.clone()]); self.writes.lock().unwrap().push(range.len());}
            Ok(())
        }
        fn device_address(&self,buffer:&Buffer)->u64 {buffer.id<<24}
    }

    #[test]
    fn actual_floor_meshlets_share_one_buffer_update_real_nani_and_release_after_last_entity() {
        use country_core::{resources::stairs::visual::StairsFloorMeshChunk,
            render::nani::{gpu_world::{GpuWorld, GpuWorldSubset, GpuWorldSubsetKind, OwnedGpuWorldSubset, InstanceDataBuffer, InstanceKey},
                instance_data::StairsFloorInstanceData,
                systems::{nani_maintain_instances_ecs,nani_update_stairs_floor_ecs,nani_flush_instance_updates}}};
        use country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary;
        use bevy_ecs::{system::SystemState,schedule::{Schedule,ApplyDeferred}};
        assert_eq!(std::mem::size_of::<StairsFloorMeshChunk>(),48);
        assert_eq!(std::mem::offset_of!(StairsFloorMeshChunk,chunk),8);
        assert_eq!(std::mem::offset_of!(StairsFloorMeshChunk,source),40);
        let device=Arc::new(RecordingDevice::default()); let mut world=World::new();
        let mut gpu=GpuWorld::new(device.as_ref());
        gpu.assign_subset(GpuWorldSubsetKind::StairsFloor,GpuWorldSubset::Owned(OwnedGpuWorldSubset::new::<StairsFloorInstanceData>(device.as_ref(),GpuWorldSubsetKind::StairsFloor,5)));
        world.insert_resource(gpu);world.insert_resource(MeshAtlasLibrary::default());
        let points:Vec<_>=(0..129).map(|i|Vec3::new(i as f32*0.7,i as f32*0.03,0.0)).collect();
        let mut entities=Vec::new();let mut commands=SystemState::<Commands>::new(&mut world);
        gen_stair_floors(device.clone(),&vec![1.;129],&vec![Vec2::X;129],&points,&mut entities,&mut commands.get_mut(&mut world),100003,&[Some(0.1),None],&vec![0.;129]);
        commands.apply(&mut world); assert!(entities.len()>1,"必须实际跨越64三角meshlet边界");
        let owner=Arc::downgrade(&world.get::<StairsFloorMeshChunk>(entities[0]).unwrap().buffer);
        let buffer_id=world.get::<StairsFloorMeshChunk>(entities[0]).unwrap().buffer.handle().id;
        for &entity in &entities {let chunk=world.get::<StairsFloorMeshChunk>(entity).unwrap();assert_eq!(chunk.buffer.handle().id,buffer_id);assert!((1..=192).contains(&chunk.chunk.index_count));}
        let mut schedule=Schedule::default();schedule.add_systems((nani_maintain_instances_ecs,ApplyDeferred,nani_update_stairs_floor_ecs).chain_ignore_deferred());
        schedule.run(&mut world);
        for &entity in &entities {
            let key=*world.get::<InstanceKey>(entity).unwrap(); let chunk=world.get::<StairsFloorMeshChunk>(entity).unwrap();
            let subset=world.resource::<GpuWorld>().owned_subset_mut(GpuWorldSubsetKind::StairsFloor);
            let data=subset.data().as_any().downcast_ref::<InstanceDataBuffer<StairsFloorInstanceData>>().unwrap();
            let mut expected=[0;12];expected[..9].copy_from_slice(&chunk.gpu_words());assert_eq!(data.shadow.data[key.index as usize].raw,expected);
        }
        nani_flush_instance_updates(&mut world.resource_mut::<GpuWorld>(),device.as_ref());world.clear_trackers();
        let before=device.writes.lock().unwrap().len();schedule.run(&mut world);
        nani_flush_instance_updates(&mut world.resource_mut::<GpuWorld>(),device.as_ref());
        assert_eq!(device.writes.lock().unwrap().len(),before,"原Changed过滤下静止帧不重复上传floor");
        world.get_mut::<StairsFloorMeshChunk>(entities[0]).unwrap().source=0x7fc1_2345;schedule.run(&mut world);
        let key=*world.get::<InstanceKey>(entities[0]).unwrap();
        {let subset=world.resource::<GpuWorld>().owned_subset_mut(GpuWorldSubsetKind::StairsFloor);
         let data=subset.data().as_any().downcast_ref::<InstanceDataBuffer<StairsFloorInstanceData>>().unwrap();assert_eq!(data.shadow.data[key.index as usize].raw[8],0x7fc1_2345);}
        for &entity in &entities[..entities.len()-1] {world.despawn(entity);}
        assert!(owner.upgrade().is_some());assert!(!device.released.lock().unwrap().contains(&buffer_id));
        world.despawn(*entities.last().unwrap());assert!(owner.upgrade().is_none());
        assert_eq!(device.released.lock().unwrap().iter().filter(|&&id|id==buffer_id).count(),1);
        schedule.run(&mut world);assert_eq!(world.resource::<GpuWorld>().owned_subset_mut(GpuWorldSubsetKind::StairsFloor).instance_count(),0);
    }

    #[test]
    fn actual_floor_buffer_creation_failure_spawns_nothing_and_does_not_fake_a_gpu_owner() {
        use bevy_ecs::system::SystemState;
        let device=Arc::new(RecordingDevice {fail_creation:true,..Default::default()});let mut world=World::new();
        let mut commands=SystemState::<Commands>::new(&mut world);let mut entities=Vec::new();
        gen_stair_floors(device.clone(),&[1.,1.],&[Vec2::X;2],&[Vec3::ZERO,Vec3::X],&mut entities,&mut commands.get_mut(&mut world),100003,&[None,None],&[0.,0.]);
        commands.apply(&mut world);assert!(entities.is_empty());assert_eq!(world.entities().len(),0);
        assert!(device.buffers.lock().unwrap().is_empty()&&device.released.lock().unwrap().is_empty());
    }

    #[test]
    fn fully_clipped_floor_keeps_original_trimesh_empty_attribute_failure() {
        use bevy_ecs::system::SystemState;
        let device=Arc::new(RecordingDevice::default());let mut world=World::new();
        let mut commands=SystemState::<Commands>::new(&mut world);let mut entities=Vec::new();
        // 原几何阶段允许全裁空，但后续Trimesh::positions/attrib仍拒绝空属性；不能改成假成功。
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
            gen_stair_floors(device.clone(),&[1.,1.],&[Vec2::X;2],&[Vec3::ZERO,Vec3::X],&mut entities,&mut commands.get_mut(&mut world),100003,&[None,None],&[0.2,0.2]))).is_err());
        commands.apply(&mut world);assert!(entities.is_empty());assert!(device.buffers.lock().unwrap().is_empty());
    }
    #[test]
    fn original_floor_geometry_preserves_clipping_both_faces_and_all_float_bits() {
        let report: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m01-stair-floors-rosetta.json"))).unwrap();
        assert_eq!(report["binary_sha256"], "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03");
        let rows = report["rows"].as_array().unwrap(); assert_eq!(rows.len(),388);
        let mut differences = Vec::new();
        for (case,row) in rows.iter().enumerate() {
            let input = &row["input"]; let f = |v:&serde_json::Value| f32::from_bits(v.as_u64().unwrap() as u32);
            let floats = |key:&str| input[key].as_array().unwrap().iter().map(f).collect::<Vec<_>>();
            let widths = floats("widths"); let auxiliary = floats("auxiliary");
            let positions: Vec<_> = input["positions"].as_array().unwrap().iter().map(|p|Vec3::from_array(std::array::from_fn(|i|f(&p[i])))).collect();
            let tangents: Vec<_> = input["tangents"].as_array().unwrap().iter().map(|p|Vec2::from_array(std::array::from_fn(|i|f(&p[i])))).collect();
            let endpoints = std::array::from_fn(|i|(!input["endpoints"][i].is_null()).then(||f(&input["endpoints"][i])));
            let actual = std::panic::catch_unwind(||stair_floor_geometry(&widths,&tangents,&positions,&endpoints,&auxiliary));
            if row["panic"].as_bool().unwrap() {assert!(actual.is_err(),"原floor拒绝case{case}");continue;}
            let actual=actual.unwrap();
            for (name,actual) in [("indices",actual.indices),
                ("positions",actual.positions.iter().flat_map(|v|v.to_array()).map(f32::to_bits).collect()),
                ("normals",actual.normals.iter().flat_map(|v|v.to_array()).map(f32::to_bits).collect())] {
                let expected: Vec<_> = row[name].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u32).collect();
                if actual != expected {differences.push(format!("case{case} {name} len {}/{} first {:?}",actual.len(),expected.len(),actual.iter().zip(&expected).enumerate().filter(|(_, (a,b))|a!=b).take(3).collect::<Vec<_>>()));}
            }
        }
        assert!(differences.is_empty(),"原floor三数组严格逐位差异：{differences:?}");
    }
}

/// 原1407de8b0：三路采样按最短输入zip，再step_by、相邻两点生成碰撞与RT几何。
/// 原141fe5f70在凸包失败前已追加两路几何，因此不能用filter_map提前丢掉它们。
pub fn gen_stairs_collider_and_rt_geo(
    positions: &[Vec3],
    tangents: &[Vec2],
    widths: &[f32],
    step: usize,
    colliders: &mut Vec<country_core::systems::collision::world::RaycastMeshData>,
    rt_positions: &mut Vec<Vec3>,
    top_positions: &mut Vec<glam::Vec4>,
) {
    // step_by原本就在迭代器构造时拒绝零步长，包括空输入。
    let mut samples = positions.iter().zip(tangents).zip(widths).step_by(step);
    let Some(mut previous) = samples.next() else { return; };
    for current in samples {
        let corners = stair_collider_corners(
            *previous.0.0, *current.0.0, *previous.0.1, *current.0.1,
            *previous.1, *current.1,
        );
        // 原两套索引顺序及绕向分别属于闭合侧面和独立顶面；不补端盖。
        const RT: [usize; 24] = [0, 4, 1, 1, 4, 5, 2, 3, 6, 3, 7, 6,
            0, 2, 4, 2, 6, 4, 1, 5, 3, 3, 5, 7];
        const TOP: [usize; 6] = [2, 3, 6, 3, 7, 6];
        rt_positions.extend(RT.map(|index| corners[index]));
        top_positions.extend(TOP.map(|index| corners[index].extend(1.0)));
        if let Some(collider) = country_core::systems::collision::world::RaycastMeshData::new_convex(&corners) {
            colliders.push(collider);
        }
        previous = current;
    }
}

/// 原141fe5f70：先取水平切线的左法线，再依次乘宽度、半宽。
/// 保留零Y参与乘法及顶点的加零，避免改变负零和非有限输入的传播。
fn stair_collider_corners(
    start: Vec3, end: Vec3, start_tangent: Vec2, end_tangent: Vec2,
    start_width: f32, end_width: f32,
) -> [Vec3; 8] {
    fn endpoint(position: Vec3, tangent: Vec2, width: f32) -> [Vec3; 4] {
        let dx = (-tangent.y * width) * 0.5;
        let dy = (width * 0.0) * 0.5;
        let dz = (width * tangent.x) * 0.5;
        let left_y = (position.y + -0.375) - dy;
        let right_y = (dy + position.y) + -0.375;
        let left_x = position.x - dx;
        let right_x = position.x + dx;
        let left_z = position.z - dz;
        let right_z = position.z + dz;
        [Vec3::new(left_x, left_y + -0.5, left_z),
         Vec3::new(right_x, right_y + -0.5, right_z),
         Vec3::new(left_x + 0.0, left_y + 0.5, left_z + 0.0),
         Vec3::new(right_x + 0.0, right_y + 0.5, right_z + 0.0)]
    }
    let a = endpoint(start, start_tangent, start_width);
    let b = endpoint(end, end_tangent, end_width);
    [a[0], a[1], a[2], a[3], b[0], b[1], b[2], b[3]]
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

/// 原1407d2b79/1407d2ede的真实registry交接；生产请求遍历复用本入口。
/// 原IsoTransform::from_translation明确传(0,0.3,0)，不能恢复成零偏移。
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
        Vec3::new(0.0, f32::from_bits(0x3e99_999a), 0.0),
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

/// 原内联1407dd80c..832/930..949：floor(log2(tri_count-1))-1，减法分别饱和。
/// index_count/3属于调用者；不能在这里再除三或把低三角数映射到负索引。
pub fn floor_tri_count_to_atlas_idx(tri_count: u32) -> u32 {
    30_u32.saturating_sub(tri_count.saturating_sub(1).leading_zeros())
}

#[cfg(test)]
mod dirty_structural_tests {
    use super::*;
    use bevy_ecs::system::RunSystemOnce;
    use country_core::{resources::{stairs::{stairs_collider::StairColliderRegistry,
        structural_graph::StairsStructuralGraph, stairs_assembly_params::{PlayermadeStairsAssemblyParams, StairSubSegment}},
        terrain_height::{TerrainHeightsData, VisualTerrainHeightSsbo}, walls::public_walls::PublicWalls},
        systems::ui::signifiers::signifier_create::UiSignifierStream};
    use std::panic::{AssertUnwindSafe, catch_unwind};

    struct OriginalSpan { index: u32, source: [u32; 4], height: u32, convex: [u32; 24] }
    struct OriginalStructural {
        n: usize, removed: bool, bits: usize, dirty: bool, curve: bool, widths: usize,
        sub_segments: &'static [(u8, u16)], panic: bool, count: Option<u32>, spans: &'static [OriginalSpan],
    }
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m01-stair-structural-stage-native-cases.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m01-stair-dirty-event-native-cases.rs"));
    struct OriginalJunction {
        center:u32, neighbors:&'static [u32], counts:&'static [(u32,u32,u32)], panic:bool, edges:&'static [[u32;6]],
    }
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m01-stair-structural-junction-native-cases.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m01-stair-lazy-tile-native-cases.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m01-stair-structural-wall-anchor-native-cases.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m01-stair-distortion-scalar-native-cases.rs"));

    #[test]
    fn original_distortion_scalar_instruction_blocks_match_at_boundaries_and_nan() {
        for &[input, expected] in ORIGINAL_DISTORTION_FACTORS {
            let actual = structural_distortion_factor(f32::from_bits(input));
            assert_eq!(actual.to_bits(), expected, "原factor输入{input:#x}");
        }
        for &[sample, height, expected] in ORIGINAL_DISTORTION_MIN {
            let actual = structural_height_min(f32::from_bits(sample), f32::from_bits(height));
            assert_eq!(actual.to_bits(), expected, "原min输入{sample:#x}/{height:#x}");
        }
    }

    #[test]
    fn distortion_consumes_real_world_support_add_remove_and_keeps_same_edge_unconnected() {
        use country_core::systems::collision::world::{RaycastGeoSource, RaycastMeshData, upsert_collider, remove_collider};
        use country_core::resources::stairs::structural_graph::StructuralGraphConnection;
        let mut world = world();
        world.run_system_once(structural_graph_subscribe).unwrap();
        let mut native = RaycastWorld::new();
        let subscription = StructuralGraphSubscription::new(&mut native);
        let mut receipts = std::collections::HashMap::new();
        let first_edge = StairGraphEdge::new(3, 8);
        let second_edge = StairGraphEdge::new(10, 11);
        let mut handles = Vec::new();
        for (id, source_tag, edge, index) in [(0, 16, first_edge, 0), (1, 16, first_edge, 1), (2, 16, second_edge, 0), (3, 0, first_edge, 0)] {
            let mut source = RaycastGeoSource::Slot0;
            source.0[0] = source_tag;
            source.0[4..8].copy_from_slice(&(index as u32).to_le_bytes());
            let (a, b) = edge.nodes();
            source.0[8..12].copy_from_slice(&a.to_le_bytes());
            source.0[12..16].copy_from_slice(&b.to_le_bytes());
            let upsert = upsert_collider(&mut native, &mut receipts, id, source,
                std::sync::Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO)), Vec3::ZERO, glam::Quat::IDENTITY);
            let country_core::systems::collision::world::ColliderUpsert::Inserted { handle } = upsert else { panic!("fresh collider") };
            handles.push(handle);
        }
        // 订阅先于upsert的真实增量发布；flush只同步Rapier查询树。
        native.flush_colliders();
        world.insert_resource(native); world.insert_resource(subscription);
        {
            let mut graph = world.resource_mut::<StairsStructuralGraph>();
            graph.upsert_span((first_edge,0),handles[0].into(),1.0);
            graph.upsert_span((first_edge,1),handles[1].into(),1.0);
            graph.upsert_span((second_edge,0),handles[2].into(),1.0);
        }
        {
            let mut requests = world.resource_mut::<PlayermadeStairsAssemblyRequests>();
            for (a,b,n) in [(3,8,3),(10,11,2)] {
                let r = request(n,a,b);
                requests.previous.sampled_points.insert(StairGraphEdgeIdentity::from(r.segment), SampledStairsPoints {
                    points:r.interpolated_curve.as_ref().unwrap().points.clone(), tangents:r.tangents.clone(),
                    widths:r.widths.clone(), auxiliary_words:vec![], metadata:StairSamplesMetadata {
                        info_28:0, extra_08:0, extra_0c:0, removed_support:false, extra_10:0,
                    },
                });
                requests.requests.push(r);
            }
        }
        world.run_system_once(calculate_distortion_from_structural_graph).unwrap();
        {
            let graph = world.resource::<StairsStructuralGraph>();
            assert!(graph.graph().contains_edge(handles[0].into(),handles[2].into()));
            assert!(!graph.graph().contains_edge(handles[0].into(),handles[1].into()), "同边不能产生临时重叠连接");
            for &handle in &handles[..3] { assert_eq!(graph.collider_to_height()[&country_core::resources::stairs::structural_graph::StairColliderHandle::from(handle)],0.046875); }
            assert!(graph.connections().is_empty());
            assert_eq!(graph.collider_to_height(),graph.working_heights());
        }
        let removed = world.resource::<StairsRemovedSupports>().clone();
        {
            // 永久span关系在原结构阶段建立，必须跨临时支撑删除继续存在。
            let mut graph = world.resource_mut::<StairsStructuralGraph>();
            graph.add_edge_by_collider(handles[0].into(),handles[1].into(),StructuralGraphConnection::new(false));
        }
        {
            let mut physics = world.resource_mut::<RaycastWorld>();
            remove_collider(&mut physics,&mut receipts,&3).unwrap(); physics.flush_colliders();
        }
        world.run_system_once(calculate_distortion_from_structural_graph).unwrap();
        let graph = world.resource::<StairsStructuralGraph>();
        assert!(!graph.graph().contains_node(handles[3].into()));
        assert!(!graph.graph().edge_weight(handles[0].into(),handles[1].into()).unwrap().transient);
        for &handle in &handles[..3] { assert_eq!(graph.collider_to_height()[&country_core::resources::stairs::structural_graph::StairColliderHandle::from(handle)],1.0); }
        assert_eq!(*world.resource::<StairsRemovedSupports>(),removed);
        for request in &world.resource::<PlayermadeStairsAssemblyRequests>().requests {
            // 原resize保留同长旧值；没有请求生产者重建时会再次变换旧factor。
            assert!(request.auxiliary_samples.iter().all(|&x| x == 0.0));
        }
    }

    fn request(n: usize, a: u32, b: u32) -> PlayermadeStairsAssemblyParams {
        let points: Vec<_> = (0..n).map(|i| Vec3::new(i as f32 * 1.25, i as f32 * 0.375 + 0.25,
            if i % 2 == 0 { 0.5 } else { -0.5 })).collect();
        PlayermadeStairsAssemblyParams {
            curve: utils::curve::Curve2::crashy_new_from_points(points.iter().map(|p| Vec2::new(p.x, p.z)).collect(), false),
            heights: points.iter().map(|p| p.y).collect(), widths: (0..n).map(|i| 1.0 + i as f32 * 0.125).collect(),
            tangents: (0..n).map(|i| Vec2::new(1.0, i as f32 * 0.125)).collect(), auxiliary_samples: vec![],
            interpolated_curve: Some(utils::curve::Curve3::try_new_from_points(points, true).unwrap()),
            nodes: [(StairNodeAnchor::Terrain { height: 0.0 }, 1); 2],
            terrain_contacts: bitvec::vec::BitVec::repeat(false, n), segment: StairSegment::new(DecoratorId(a), DecoratorId(b)),
            sub_segments: [StairSubSegment { kind: 0, flag: 0, count: (n - 1) as u16 }].into_iter().collect(), dirty: true,
        }
    }

    fn world() -> World {
        let mut world = World::new();
        world.insert_resource(RaycastWorld::new()); world.init_resource::<StairColliderRegistry>();
        world.init_resource::<StairsStructuralGraph>(); world.init_resource::<PlayermadeStairsAssemblyRequests>();
        world.init_resource::<StairsRemovedSupports>(); world.init_resource::<StairsState>();
        world.init_resource::<StairsVisualState>(); world.init_resource::<UiSignifierStream>();
        world.init_resource::<PublicWalls>();
        world.init_resource::<Events<country_core::resources::walls::decorator::DecoratorOpEvent>>();
        world.init_resource::<Events<country_core::resources::roofs::commands::RecalculateWallHoles>>();
        world.insert_resource(TerrainHeightsData::<country_core::resources::terrain_height::TerrainHeightSsbo>::new());
        world.insert_resource(TerrainHeightsData::<VisualTerrainHeightSsbo>::new());
        world.run_system_once(stairs_subscribe_lazy).unwrap();
        assert_eq!(world.resource::<StairsLazySubscriber>().subscriptions.len(), 5);
        world
    }

    #[test]
    fn original_127_full_entry_cases_reach_real_collider_and_structural_owners() {
        for (case_index, original) in ORIGINAL_STRUCTURAL.iter().enumerate() {
            let mut world = world(); let edge = StairGraphEdge::new(3, 8);
            let mut request = request(original.n, 3, 8); request.dirty = original.dirty;
            if !original.curve { request.interpolated_curve = None; }
            request.widths.truncate(original.widths);
            for i in 0..original.n { request.terrain_contacts.set(i, original.bits & (1 << i) != 0); }
            request.sub_segments = original.sub_segments.iter().map(|&(kind, count)| StairSubSegment { kind, flag: 0, count }).collect();
            world.resource_mut::<PlayermadeStairsAssemblyRequests>().requests.push(request);
            if original.removed { world.resource_mut::<StairsRemovedSupports>().record(edge); }
            let result = catch_unwind(AssertUnwindSafe(|| world.run_system_once(create_structural_colliders_and_initial_graph).unwrap()));
            assert_eq!(result.is_err(), original.panic, "原case{case_index}失败分支");
            let registry = world.resource::<StairColliderRegistry>();
            assert_eq!(registry.structural_colliders.len(), original.spans.len(), "原case{case_index}碰撞数");
            let structural = world.resource::<StairsStructuralGraph>();
            assert_eq!(structural.segment_counts().get(&edge).copied(), original.count, "原case{case_index}段数");
            for span in original.spans {
                let instance = registry.structural_colliders.get(&(edge, span.index)).unwrap();
                let mesh = world.resource::<RaycastWorld>().get_collider_mesh(instance.registry_index, instance.generation);
                let words: Vec<_> = mesh.source.0.chunks_exact(4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).collect();
                assert_eq!(&words[..4], &span.source, "原case{case_index}来源有效字段");
                let expected_points: Vec<_> = span.convex.chunks_exact(3).map(|p| glam::UVec3::new(p[0], p[1], p[2])).collect();
                assert_eq!(mesh.data.hash, country_core::render::utils::calculate_hash(&expected_points.as_slice()),
                    "原case{case_index}实际mesh几何hash");
                let handle = *structural.span_to_collider().get(&(edge, span.index)).unwrap();
                assert_eq!(handle.as_subworld(), instance.handle());
                assert_eq!(structural.collider_to_height()[&handle].to_bits(), span.height, "原case{case_index}结构高度");
            }
        }
    }

    #[test]
    fn real_dirty_publishers_mark_only_dependent_visuals_once_and_drop_with_resource() {
        use country_core::{geometry::instanced_wall::BrickTransformSsbo, render::{sparse_instance_buffer::{MeshAtlasAllocation, SparseInstanceBuffer}, test_device::TestDevice}};
        let mut world = world(); let hit = StairGraphEdge::new(1, 2); let miss = StairGraphEdge::new(4, 5);
        let mut buffer = SparseInstanceBuffer::<BrickTransformSsbo>::try_new(&TestDevice(91)).unwrap();
        for edge in [hit, miss] { world.resource_mut::<StairsVisualState>().reset_edge(edge); }
        {
            let mut visual = world.resource_mut::<StairsVisualState>(); visual.clear_dirty();
            let state = visual.get_mut(&hit).unwrap(); state.affected_tiles.insert((0, 0));
            let (first, _) = state.allocations_mut(); *first = Some(MeshAtlasAllocation::new(&buffer));
            buffer.writer_for(first.as_ref().unwrap().allocation()).push(&BrickTransformSsbo::default()).unwrap();
        }
        let free = buffer.free_chunks.read().len();
        // 真高度owner的copy_from_gpu发布，consumer不能靠人工填充Subscriber待收队列。
        let mut heights = vec![0; 512 * 512]; heights[0] = 7;
        world.resource_mut::<TerrainHeightsData>().copy_from_gpu(&heights);
        world.run_system_once(playermade_stairs_assembly_propagate_dirty_flags).unwrap();
        let visual = world.resource::<StairsVisualState>();
        assert!(visual.get(&hit).unwrap().force_rebuild_requested()); assert!(!visual.get(&miss).unwrap().force_rebuild_requested());
        assert!(!visual.is_dirty()); assert_eq!(buffer.free_chunks.read().len(), free); assert!(buffer.waste.lock().is_empty());
        assert!(world.resource_mut::<StairsLazySubscriber>().subscriptions.iter_mut().all(|sub| sub.take().is_none()));
        let tick = world.get_resource_ref::<StairsVisualState>().unwrap().last_changed();
        world.increment_change_tick(); world.run_system_once(playermade_stairs_assembly_propagate_dirty_flags).unwrap();
        assert_eq!(world.get_resource_ref::<StairsVisualState>().unwrap().last_changed(), tick);
        world.remove_resource::<StairsLazySubscriber>();
        heights[1] = 9; world.resource_mut::<TerrainHeightsData>().copy_from_gpu(&heights);
        // 退出后允许重新订阅；新边界不接收订阅前已经发布的旧批次。
        world.run_system_once(stairs_subscribe_lazy).unwrap();
        assert!(world.resource_mut::<StairsLazySubscriber>().subscriptions.iter_mut().all(|sub| sub.take().is_none()));
    }

    #[test]
    fn original_420_event_variants_use_true_public_walls_and_event_writer_without_duplicates_suppression() {
        use country_core::resources::walls::{WallId, WallState, inner_walls::InnerWalls,
            public_walls::WallPublicationDirty, publication_source::TerrainHeightGrid512};
        use country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;
        use country_core::resources::{walls::decorator::DecoratorOpEventKind,
            roofs::commands::RecalculateWallHoles};
        let mut world = world(); let mut inner = InnerWalls::new();
        for id in [0,1,2,41] {
            let curve = country_core::resources::roofs::geometry::surface::Curve2::try_new(vec![[0.0,0.0],[1.0,0.0]], false).unwrap();
            inner.new_wall_with_id(WallId(id), WallState::new(curve, 2.0, 0, 0, false));
        }
        let terrain = TerrainHeightGrid512 { encoded: vec![0;512*512].into(), minimum_y: 0.0, maximum_y: 0.0, sampling_fix: true };
        world.resource_mut::<PublicWalls>().consume_with_paths(&mut inner, &terrain, None,
            &WallPublicationDirty::default(), &WallSpatialHash::default());
        let mut schedule = Schedule::default(); schedule.add_systems(playermade_stairs_assembly_propagate_dirty_flags);
        for (index, &(disc, tag, oldtag, height, corner, expected)) in ORIGINAL_DIRTY_EVENTS.iter().enumerate() {
            for id in [0,1,2,41] {
                world.resource_mut::<PublicWalls>().get_mut(WallId(id)).unwrap().geometry.metrics.constrained_extra = f32::from_bits(height);
            }
            world.resource_mut::<Events<country_core::resources::walls::decorator::DecoratorOpEvent>>().clear();
            let mut raw = [0;136]; raw[..8].copy_from_slice(&disc.to_le_bytes());
            raw[8..16].copy_from_slice(&tag.to_le_bytes()); raw[16..24].copy_from_slice(&oldtag.to_le_bytes());
            raw[24..32].copy_from_slice(&41u64.to_le_bytes()); raw[129] = if corner {63} else {62};
            world.resource_mut::<Events<country_core::resources::walls::decorator::DecoratorOpEvent>>().send(DecoratorOpEventKind(raw));
            schedule.run(&mut world);
            let actual:Vec<_> = world.resource_mut::<Events<RecalculateWallHoles>>().drain().map(|e| e.0.0).collect();
            assert_eq!(&actual, expected, "原eventcase{index}");
            schedule.run(&mut world);
            assert!(world.resource_mut::<Events<RecalculateWallHoles>>().drain().next().is_none(), "同一reader不能重发已读批次");
        }
        // 同一角吸附事件的新旧地址相等时，原两个独立分支均发通知。
        let mut raw = [0;136]; raw[8..16].copy_from_slice(&41u64.to_le_bytes());
        raw[24..32].copy_from_slice(&41u64.to_le_bytes()); raw[129] = 63;
        world.resource_mut::<PublicWalls>().get_mut(WallId(41)).unwrap().geometry.metrics.constrained_extra = 3.0;
        world.resource_mut::<Events<country_core::resources::walls::decorator::DecoratorOpEvent>>().send(DecoratorOpEventKind(raw));
        schedule.run(&mut world);
        assert_eq!(world.resource_mut::<Events<RecalculateWallHoles>>().drain().map(|e|e.0).collect::<Vec<_>>(), vec![WallId(41);2]);
    }

    #[test]
    fn all_five_original_publishers_reach_the_same_tile_dependency_consumer() {
        use country_core::systems::collision::world::{RaycastGeoSource, RaycastMeshData, upsert_collider};
        for source in 0..5 {
            let mut world = world(); let edge = StairGraphEdge::new(1,2); let unrelated = StairGraphEdge::new(4,5);
            {
                let mut visual = world.resource_mut::<StairsVisualState>();
                for entry in [edge, unrelated] { visual.reset_edge(entry); }
                visual.get_mut(&edge).unwrap().affected_tiles.insert((32,32));
                visual.get_mut(&unrelated).unwrap().affected_tiles.insert((0,0));
                visual.clear_dirty();
            }
            if source < 3 {
                // 原SOURCE_TO_SLOT：tag0/2/1分别落真实slot0/1/9，并经真实flush发布。
                let geo = [RaycastGeoSource::Slot0, RaycastGeoSource::Slot2, RaycastGeoSource::Slot1][source];
                let mut map = std::collections::HashMap::new();
                let mut physics = world.resource_mut::<RaycastWorld>();
                upsert_collider(&mut physics, &mut map, 1u32, geo,
                    std::sync::Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE,Vec3::ZERO)),Vec3::ZERO,glam::Quat::IDENTITY);
                physics.flush_colliders();
            } else {
                let mut heights = vec![0;512*512]; heights[256*512+256] = 1;
                if source == 3 { world.resource_mut::<TerrainHeightsData>().copy_from_gpu(&heights); }
                else { world.resource_mut::<TerrainHeightsData<VisualTerrainHeightSsbo>>().copy_from_gpu(&heights); }
            }
            world.run_system_once(playermade_stairs_assembly_propagate_dirty_flags).unwrap();
            let visual = world.resource::<StairsVisualState>();
            assert!(visual.get(&edge).unwrap().force_rebuild_requested(), "原publisher{source}");
            assert!(!visual.get(&unrelated).unwrap().force_rebuild_requested(), "原publisher{source}不误标其它tile");
            assert!(!visual.is_dirty());
        }
    }

    #[test]
    fn recovered_request_producer_interpolation_invalidation_propagation_reaches_nonempty_real_structure() {
        let mut world = super::interpolate_history_tests::generated_world(2.0);
        world.insert_resource(RaycastWorld::new()); world.init_resource::<StairColliderRegistry>();
        world.init_resource::<StairsStructuralGraph>(); world.init_resource::<country_core::resources::rt::RtWorld>();
        world.insert_resource(TerrainHeightsData::<country_core::resources::terrain_height::TerrainHeightSsbo>::new());
        world.init_resource::<Events<country_core::resources::walls::decorator::DecoratorOpEvent>>();
        world.init_resource::<Events<country_core::resources::roofs::commands::RecalculateWallHoles>>();
        world.run_system_once(stairs_subscribe_lazy).unwrap();
        world.run_system_once(playermade_stairs_interpolate_and_compare).unwrap();
        world.run_system_once(country_core::resources::stairs::invalidate_dirty_stair_artifacts).unwrap();
        world.run_system_once(playermade_stairs_assembly_propagate_dirty_flags).unwrap();
        world.run_system_once(create_structural_colliders_and_initial_graph).unwrap();
        let requests = world.resource::<PlayermadeStairsAssemblyRequests>();
        let request = &requests.requests[0];
        let edge = StairGraphEdge::from(request.segment);
        let sample_count = request.interpolated_curve.as_ref().unwrap().points.len() - 1;
        assert!(sample_count > 0); assert!(!request.sub_segments.is_empty());
        assert_eq!(world.resource::<StairColliderRegistry>().structural_colliders.len(), sample_count);
        assert_eq!(world.resource::<StairsStructuralGraph>().segment_counts()[&edge] as usize, sample_count);
        assert_eq!(world.resource::<StairsStructuralGraph>().graph().node_count(), sample_count);
        assert!(requests.previous.sub_segments.contains_key(&StairGraphEdgeIdentity::from(request.segment)));
        // 本case止于原slot5，尚未执行的assemble/calculate_distortion不能用伪造画面或清空图补齐。
    }

    #[test]
    fn original_16_junction_cases_reuse_actual_registered_spans_and_keep_missing_count_failure_order() {
        for (case_index, original) in ORIGINAL_JUNCTIONS.iter().enumerate() {
            let mut world = world();
            for &node in original.neighbors {
                let edge = StairGraphEdge::new(original.center,node);
                let count = original.counts.iter().find(|&&(a,b,_)| StairGraphEdge::new(a,b)==edge).map_or(2,|x| x.2.max(1));
                world.resource_mut::<StairsState>().add_edge(original.center,node);
                world.resource_mut::<PlayermadeStairsAssemblyRequests>().requests.push(request(count as usize+1,original.center,node));
            }
            // 第一轮由真实producer建立全部collider/span，第二轮只改变明确的旧计数输入。
            world.run_system_once(create_structural_colliders_and_initial_graph).unwrap();
            {
                let mut graph = world.resource_mut::<StairsStructuralGraph>();
                let cross:Vec<_> = graph.graph().all_edges().filter_map(|(a,b,_)|
                    (graph.collider_to_span()[&a].0!=graph.collider_to_span()[&b].0).then_some((a,b))).collect();
                for (a,b) in cross { graph.parts_mut().graph.remove_edge(a,b); }
                graph.parts_mut().segment_counts.clear();
                for &(a,b,count) in original.counts { graph.parts_mut().segment_counts.insert(StairGraphEdge::new(a,b),count); }
            }
            for request in &mut world.resource_mut::<PlayermadeStairsAssemblyRequests>().requests { request.dirty=false; }
            let result=catch_unwind(AssertUnwindSafe(||world.run_system_once(create_structural_colliders_and_initial_graph).unwrap()));
            assert_eq!(result.is_err(),original.panic,"原junction{case_index}失败分支");
            let graph=world.resource::<StairsStructuralGraph>();
            let cross:Vec<_>=graph.graph().all_edges().filter(|(a,b,_)|graph.collider_to_span()[a].0!=graph.collider_to_span()[b].0).collect();
            assert_eq!(cross.len(),original.edges.len(),"原junction{case_index}跨边连接数");
            for native in original.edges {
                let a=graph.span_to_collider()[&(StairGraphEdge::new(native[0],native[1]),native[2])];
                let b=graph.span_to_collider()[&(StairGraphEdge::new(native[3],native[4]),native[5])];
                assert!(!graph.graph().edge_weight(a,b).unwrap().transient,"原junction{case_index}");
            }
        }
    }

    #[test]
    fn original_144_lazy_tile_words_keep_u32_wrap_and_visual_u8_coordinate_contract() {
        use country_core::LazyTile;
        assert_eq!(std::mem::size_of::<LazyTile>(),4);
        for &(x,y,index,key) in ORIGINAL_LAZY_TILES {
            let tile=LazyTile::from(glam::UVec2::new(x,y));
            assert_eq!(tile,LazyTile::from_index(index));
            assert_eq!(tile.xy(),glam::UVec2::new(tile.x(),tile.y()));
            let mut visual=StairsVisualState::default(); let edge=StairGraphEdge::new(1,2);
            visual.reset_edge(edge); visual.clear_dirty();
            visual.get_mut(&edge).unwrap().affected_tiles.insert((key[0],key[1]));
            visual.force_rebuild_for_tile(tile);
            assert!(visual.get(&edge).unwrap().force_rebuild_requested(),"原{x},{y}→{index}→{key:?}");
            assert!(!visual.is_dirty());
        }
    }

    #[test]
    fn propagation_clones_current_segment_records_without_clearing_unrelated_previous_owners() {
        let mut world=world(); let mut current=request(3,3,5); current.dirty=false;
        current.sub_segments=[StairSubSegment {kind:2,flag:1,count:65535}].into_iter().collect();
        let key=StairGraphEdgeIdentity::from(current.segment);
        let other=StairGraphEdgeIdentity::from(StairSegment::new(DecoratorId(8),DecoratorId(9)));
        {
            let mut owner=world.resource_mut::<PlayermadeStairsAssemblyRequests>();
            owner.previous.sub_segments.insert(other,[StairSubSegment {kind:4,flag:0,count:19}].into_iter().collect());
            owner.requests.push(current);
        }
        world.run_system_once(playermade_stairs_assembly_propagate_dirty_flags).unwrap();
        let mut owner=world.resource_mut::<PlayermadeStairsAssemblyRequests>();
        assert_eq!(owner.previous.sub_segments[&other][0].count,19);
        assert_eq!(owner.previous.sub_segments[&key][0],StairSubSegment {kind:2,flag:1,count:65535});
        assert!(!owner.requests[0].dirty);
        owner.requests[0].sub_segments[0].kind=3;
        assert_eq!(owner.previous.sub_segments[&key][0].kind,2,"历史SmallVec必须真实clone，不能借用本轮可变段");
    }

    #[test]
    fn original_12_inline_ladder_wall_anchor_branches_reach_real_structural_meshes() {
        use country_core::resources::stairs::stairs_assembly_params::StairWallAnchor;
        use country_core::resources::walls::WallId;
        for &(anchors,y_mode,convex) in ORIGINAL_STRUCTURAL_WALL_ANCHORS {
            let mut world=world(); let mut current=request(2,3,8);
            current.sub_segments[0].kind=3;
            for (i,node) in current.nodes.iter_mut().enumerate() {
                if anchors & (1<<i)!=0 { node.0=StairNodeAnchor::Wall(StairWallAnchor::new(WallId(91),0.0)); }
            }
            let mut points=current.interpolated_curve.take().unwrap().points;
            if y_mode==1 { for (i,p) in points.iter_mut().enumerate() { p.y=(1-i)as f32*0.375+0.25; } }
            else if y_mode==2 { for p in &mut points { p.y=0.25; } }
            current.heights=points.iter().map(|p|p.y).collect();
            current.interpolated_curve=Some(utils::curve::Curve3::try_new_from_points(points,true).unwrap());
            world.resource_mut::<PlayermadeStairsAssemblyRequests>().requests.push(current);
            world.run_system_once(create_structural_colliders_and_initial_graph).unwrap();
            let registry=world.resource::<StairColliderRegistry>();let instance=registry.structural_colliders[&(StairGraphEdge::new(3,8),0)];
            let mesh=world.resource::<RaycastWorld>().get_collider_mesh(instance.registry_index,instance.generation);
            let expected:Vec<_>=convex.chunks_exact(3).map(|p|glam::UVec3::new(p[0],p[1],p[2])).collect();
            assert_eq!(mesh.data.hash,country_core::render::utils::calculate_hash(&expected.as_slice()),"原梯段anchors{anchors}y{y_mode}");
        }
    }

    #[test]
    fn real_colliders_are_reused_with_original_offset_and_permanent_junction_connections() {
        let mut world = world();
        for (a, b) in [(1, 3), (3, 5), (3, 8)] {
            world.resource_mut::<StairsState>().add_edge(a, b);
            world.resource_mut::<PlayermadeStairsAssemblyRequests>().requests.push(request(3, a, b));
        }
        world.run_system_once(create_structural_colliders_and_initial_graph).unwrap();
        let first: Vec<_> = world.resource::<StairColliderRegistry>().structural_colliders.iter().map(|(&key, entry)| (key, entry.handle())).collect();
        let graph = world.resource::<StairsStructuralGraph>();
        let endpoint = |a,b,i| graph.span_to_collider()[&(StairGraphEdge::new(a,b),i)];
        for (a, b) in [(endpoint(1,3,1), endpoint(3,5,0)), (endpoint(1,3,1), endpoint(3,8,0)), (endpoint(3,5,0), endpoint(3,8,0))] {
            assert!(!graph.graph().edge_weight(a,b).unwrap().transient);
        }
        for &(_, handle) in &first {
            let instance = world.resource::<StairColliderRegistry>().structural_colliders.values().find(|x| x.handle()==handle).copied().unwrap();
            let mut physics = world.resource_mut::<RaycastWorld>();
            let collider = physics.get_rapier_collider(instance.slot, instance.collider).unwrap();
            assert_eq!(collider.translation().y.to_bits(), 0.3f32.to_bits());
        }
        world.run_system_once(create_structural_colliders_and_initial_graph).unwrap();
        for (key, handle) in first { assert_eq!(world.resource::<StairColliderRegistry>().structural_colliders[&key].handle(), handle); }
        // 干净帧保留真实三图owner，不把它们清成空图；现有永久连接仍完整。
        for request in &mut world.resource_mut::<PlayermadeStairsAssemblyRequests>().requests { request.dirty = false; }
        world.run_system_once(create_structural_colliders_and_initial_graph).unwrap();
        assert_eq!(world.resource::<StairColliderRegistry>().structural_colliders.len(), 6);
        assert_eq!(world.resource::<StairsStructuralGraph>().graph().edge_count(), 6);
    }
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

    pub(super) fn generated_world(length: f32) -> World {
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
    fn generated_requests_replace_auxiliary_before_distortion_next_frame() {
        use country_core::resources::stairs::structural_graph::StairsStructuralGraph;
        let _policy = LazyPolicyGuard::enabled();
        let mut world = generated_world(1.0);
        let mut raycast = RaycastWorld::new();
        let subscription = StructuralGraphSubscription::new(&mut raycast);
        world.insert_resource(raycast);
        world.insert_resource(subscription);
        world.init_resource::<StairsStructuralGraph>();
        let edge = StairGraphEdge::from(world.resource::<PlayermadeStairsAssemblyRequests>().requests[0].segment);
        world.resource_mut::<StairsRemovedSupports>().record(edge);
        run(&mut world);
        world.run_system_once(calculate_distortion_from_structural_graph).unwrap();
        assert!(world.resource::<PlayermadeStairsAssemblyRequests>().requests[0].auxiliary_samples.iter().all(|&x| x == 1.0));
        // 模拟上帧写出的非单位factor；下一帧由真实producer替换整个请求。
        world.resource_mut::<PlayermadeStairsAssemblyRequests>().requests[0].auxiliary_samples.fill(0.0);
        world.run_system_once(crate::decorator_visual::stairs::generate_playermade_stair_requests::<false>).unwrap();
        assert!(world.resource::<PlayermadeStairsAssemblyRequests>().requests[0].auxiliary_samples.is_empty());
        run(&mut world);
        world.run_system_once(calculate_distortion_from_structural_graph).unwrap();
        assert!(world.resource::<PlayermadeStairsAssemblyRequests>().requests[0].auxiliary_samples.iter().all(|&x| x == 1.0));
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

    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reports/module-recovery-2026-10-23/m01-stair-interpolation-native-vectors.rs"));

    #[test]
    fn interpolation_kernel_matches_128_original_sse_vectors() {
        for &(old, current, old_tangent, current_tangent, factor, expected, moved) in STAIR_INTERPOLATION_NATIVE {
            let mut position = Vec3::from_array(current.map(f32::from_bits));
            let mut tangent = Vec2::from_array(current_tangent.map(f32::from_bits));
            let actual_moved = interpolate_stair_sample(&mut position, &mut tangent,
                Vec3::from_array(old.map(f32::from_bits)), Vec2::from_array(old_tangent.map(f32::from_bits)),
                f32::from_bits(factor));
            let actual = [position.x.to_bits(), position.y.to_bits(), position.z.to_bits(), tangent.x.to_bits(), tangent.y.to_bits()];
            assert_eq!(actual, expected, "original interpolation inputs: {old:x?}, {current:x?}, {old_tangent:x?}, {current_tangent:x?}, {factor:x}");
            assert_eq!(actual_moved, moved);
        }
    }

}

#[cfg(test)]
mod original_ladder_support_tests {
    use super::*;
    use country_core::resources::stairs::stairs_assembly_params::{AlignKind,StairWallAnchor};
    #[test]
    fn original_ladder_support_all_five_states_keep_anchors_degree_alignment_and_unordered_height() {
        type Case=([u32;2],[u32;2],[u8;2],[u32;2],[u8;2],u8);
        let cases:&[Case]=include!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../reports/module-recovery-2026-10-23/m01-ladder-support-native-fixtures.rs"));
        for (index,(height,tags,degree,anchor_height,flags,expected)) in cases.iter().enumerate() {
            let nodes=std::array::from_fn(|i| {
                let height=f32::from_bits(anchor_height[i]);
                let anchor=match tags[i] {0=>StairNodeAnchor::Terrain{height},1=>StairNodeAnchor::Roof{height},2=>StairNodeAnchor::Wall(StairWallAnchor::new(country_core::resources::walls::WallId(7),0.0)),_=>unreachable!()};
                (anchor,degree[i])
            });
            let alignments=std::array::from_fn(|i|AlignKind{tag:flags[i],..Default::default()});
            assert_eq!(determine_ladder_support(Vec3::new(0.0,f32::from_bits(height[0]),0.0),Vec3::new(0.0,f32::from_bits(height[1]),0.0),&nodes,&alignments),*expected,"原167B case {index}");
        }
    }
}

#[cfg(test)]
mod original_stair_collider_geometry_tests {
    use super::*;

    #[test]
    fn collider_owner_creates_keeps_replaces_and_removes() {
        use country_core::systems::collision::world::{RaycastGeoSource, RaycastMesh, RaycastMeshData};
        use std::sync::Arc;
        let mut world = RaycastWorld::new();
        let mut owner = std::collections::HashMap::new();
        let edge = StairGraphEdge::new(3, 8);
        let mut source = RaycastGeoSource::Slot13;
        source.0[4..8].copy_from_slice(&3_u32.to_le_bytes());
        source.0[8..12].copy_from_slice(&8_u32.to_le_bytes());
        let first = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO));
        let record = RaycastMesh::new(first.clone(), source);
        create_stairs_collider(&mut world, &mut owner, Some(record.clone()), edge);
        let handle = owner[&edge].handle();
        create_stairs_collider(&mut world, &mut owner, Some(record), edge);
        assert_eq!(owner[&edge].handle(), handle);
        let changed = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(2.0), Vec3::ZERO));
        create_stairs_collider(&mut world, &mut owner, Some(RaycastMesh::new(changed, source)), edge);
        assert_ne!(owner[&edge].handle(), handle);
        create_stairs_collider(&mut world, &mut owner, None, edge);
        assert!(owner.is_empty());
        create_stairs_collider(&mut world, &mut owner, None, edge);
        assert!(owner.is_empty());
    }

    struct NativeCase {
        positions: &'static [[u32; 3]],
        tangents: &'static [[u32; 2]],
        widths: &'static [u32],
        step: usize,
        panic: bool,
        corners: &'static [[[u32; 3]; 8]],
        rt: &'static [[u32; 3]],
        top: &'static [[u32; 4]],
    }

    #[test]
    fn original_collider_rt_wrapper_keeps_stride_zip_corners_triangle_order_and_top_w() {
        let cases: &[NativeCase] = include!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m01-stair-collider-native-fixtures.rs"));
        for (index, case) in cases.iter().enumerate() {
            let positions: Vec<_> = case.positions.iter().map(|p| Vec3::from_array(p.map(f32::from_bits))).collect();
            let tangents: Vec<_> = case.tangents.iter().map(|p| Vec2::from_array(p.map(f32::from_bits))).collect();
            let widths: Vec<_> = case.widths.iter().copied().map(f32::from_bits).collect();
            let mut colliders = Vec::new();
            // 原为append合同，既有写面不能被清空。
            let sentinel = Vec3::new(11.0, -13.0, 17.0);
            let mut rt = vec![sentinel];
            let mut top = vec![sentinel.extend(19.0)];
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                gen_stairs_collider_and_rt_geo(&positions, &tangents, &widths,
                    case.step, &mut colliders, &mut rt, &mut top)));
            assert_eq!(result.is_err(), case.panic, "原wrapper case {index}");
            if case.panic { continue; }
            assert_eq!(rt[0], sentinel);
            assert_eq!(top[0], sentinel.extend(19.0));
            assert_eq!(rt[1..].iter().map(|p| p.to_array().map(f32::to_bits)).collect::<Vec<_>>(), case.rt,
                "原RT逐字 case {index}");
            assert_eq!(top[1..].iter().map(|p| p.to_array().map(f32::to_bits)).collect::<Vec<_>>(), case.top,
                "原顶面逐字 case {index}");
            let sampled: Vec<_> = positions.iter().zip(&tangents).zip(&widths).step_by(case.step).collect();
            for (segment, expected) in sampled.windows(2).zip(case.corners) {
                let a = segment[0]; let b = segment[1];
                let corners = stair_collider_corners(*a.0.0, *b.0.0, *a.0.1, *b.0.1, *a.1, *b.1);
                assert_eq!(corners.map(|p| p.to_array().map(f32::to_bits)), *expected,
                    "原凸包输入逐字 case {index}");
            }
            if !case.corners.is_empty() {
                // 原oracle只桥接Parry并返回None；这里独立验证当前真实凸包构造。
                // 普通输入逐段保留真实凸包；零宽可能仍被Parry接受为退化多面体，
                // 不凭宽度推断None，更不能在该边界追加替代球体。
                let nondegenerate = widths.iter().any(|width| *width != 0.0);
                if nondegenerate { assert_eq!(colliders.len(), case.corners.len(),
                    "真实Parry结果 case {index}"); }
                assert!(colliders.iter().all(|mesh| mesh.vertices_ptr == 4 && mesh.shape.is_some()));
            }
        }
    }
}

#[cfg(test)]
mod original_stair_ladder_tests {
    use super::*;

    struct LadderNativeCase {
        start: [u32; 3], end: [u32; 3], widths: [u32; 2], support: u8,
        seed: u64, final_seed: u64, records: &'static [[u32; 16]],
    }

    #[test]
    fn original_ladder_all_supports_slope_widths_rng_and_full_64_byte_records() {
        let cases: &[LadderNativeCase] = include!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m01-stair-ladder-native-fixtures.rs"));
        for (index, case) in cases.iter().enumerate() {
            let positions = [Vec3::from_array(case.start.map(f32::from_bits)),
                Vec3::from_array(case.end.map(f32::from_bits))];
            let widths = case.widths.map(f32::from_bits);
            let mut rng = fastrand::Rng::with_seed(case.seed);
            let mut records = Vec::new();
            emit_ladder_pieces(&positions, &widths, case.support, 42, &mut rng,
                |piece| records.push(piece));
            assert_eq!(records.len(), case.records.len(), "原梯子记录数 case {index}");
            assert_eq!(rng.get_seed(), case.final_seed, "原梯子最终RNG case {index}");
            for (record_index, (record, expected)) in records.iter().zip(case.records).enumerate() {
                let mut actual = [0; 16];
                for (column_index, column) in record.transform.cols.iter().enumerate() {
                    actual[column_index * 3..column_index * 3 + 3]
                        .copy_from_slice(&column.to_array().map(f32::to_bits));
                }
                actual[12..].copy_from_slice(&[record.gate_seed as u32, record.piece_seed as u32,
                    record.wall_id, record.flags]);
                for (word, (&actual, &expected)) in actual.iter().zip(expected).enumerate() {
                    // 原等高且要求竖撑的输入产生NaN，不跨ISA要求其符号/载荷一致。
                    assert!(actual == expected || (word < 12 && f32::from_bits(actual).is_nan()
                        && f32::from_bits(expected).is_nan()),
                        "原梯子case {index} record {record_index} word {word}: {actual:08x} != {expected:08x}");
                }
            }
        }
    }

    #[test]
    fn actual_ladder_sparse_edit_dirty_upload_stable_frames_and_owner_release() {
        use country_core::{render::{sparse_instance_buffer::{MeshAtlasAllocation,
            SparseInstanceBuffer, SparseSsbo}, ssbo::rhapsody_boundary::{BackendError, Buffer,
            BufferDesc, GraphicsDevice}}, resources::roofs::roof_buffers::WoodenPlankSsbo};
        use std::{ops::Range, sync::{Mutex, atomic::Ordering}};
        #[derive(Default)]
        struct UploadDevice(Mutex<Vec<Vec<Range<usize>>>>);
        impl GraphicsDevice for UploadDevice {
            fn create_buffer(&self, desc: &BufferDesc, _: &str, _: Option<&[u8]>) -> Result<Buffer, BackendError> {
                Ok(Buffer { byte_size: desc.size as u64, ..Default::default() })
            }
            fn update_buffer_bytes(&self, buffer: &Buffer, data: &[u8], ranges: &[Range<usize>], offset: usize)
                -> Result<(), BackendError>
            {
                for range in ranges { assert!(range.end <= data.len());
                    assert!(offset + range.end <= buffer.byte_size as usize); }
                self.0.lock().unwrap().push(ranges.to_vec());
                Ok(())
            }
        }
        let device = UploadDevice::default();
        let mut buffer = SparseInstanceBuffer::<WoodenPlankSsbo>::new_storage_buffer(&device, None);
        let allocation = MeshAtlasAllocation::new(&buffer);
        let positions = [Vec3::new(0.5, 1.0, -2.0), Vec3::new(4.0, 5.5, -0.25)];
        let widths = [0.75, 1.5];
        let mut original = Vec::new();
        emit_ladder_pieces(&positions, &widths, 4, 42, &mut fastrand::Rng::with_seed(19),
            |record| original.push(record));
        construct_ladder(&mut buffer.writer_for(allocation.allocation()), &positions, &widths,
            4, 42, &mut fastrand::Rng::with_seed(19));
        assert!(original.len() > 4);
        assert_eq!(&buffer.data[..original.len()], original);
        assert_eq!(buffer.byte_map[0] as usize, original.len());
        assert!(buffer.dirty_bitset.iter().any(|word| word.load(Ordering::Relaxed) != 0));
        buffer.gpu_flush(&device).unwrap();
        assert_eq!(device.0.lock().unwrap().len(), 1);
        for _ in 0..3 { buffer.gpu_flush(&device).unwrap(); }
        assert_eq!(device.0.lock().unwrap().len(), 1, "稳定帧不重复上传");
        {
            let mut writer = buffer.writer_for(allocation.allocation());
            writer.clear();
            construct_ladder(&mut writer, &[positions[0], Vec3::new(1.0, 2.0, -1.5)],
                &widths, 0, 42, &mut fastrand::Rng::with_seed(19));
        }
        assert_ne!(&buffer.data[..original.len()], original);
        assert!(usize::from(buffer.byte_map[0]) < original.len());
        buffer.gpu_flush(&device).unwrap();
        assert_eq!(device.0.lock().unwrap().len(), 2, "编辑只追加一次dirty上传");
        drop(allocation);
        assert_eq!(buffer.waste.lock().len(), 1);
        buffer.gpu_flush(&device).unwrap();
        assert_eq!(buffer.free_chunks.read().len(), WoodenPlankSsbo::CHUNK_COUNT);
        assert_eq!(buffer.byte_map[0], 0);
        assert!(buffer.active_words().all(|word| word == 0));
        assert!(buffer.waste.lock().is_empty());
    }
}

#[cfg(test)]
mod original_stair_support_partition_tests {
    use super::*;
    use country_core::systems::collision::world::{RaycastSimpleResult, RaycastMeshData,
        RaycastGeoSource, upsert_collider, remove_collider};
    struct NativeQuery { ray: [[u32; 3]; 2], hit: bool, position: [u32; 3] }
    struct SupportNativeCase {
        normal: [u32; 2], positions: &'static [[u32; 3]], tangents: &'static [[u32; 2]],
        widths: &'static [u32], queries: &'static [NativeQuery],
        unsupported: &'static [[usize; 2]], supported: &'static [[usize; 2]],
        distances: &'static [&'static [u32]],
    }
    #[test]
    fn original_support_partition_all_interior_patterns_and_ray_distance_words() {
        let cases: &[SupportNativeCase] = include!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m01-stair-support-split-native-fixtures.rs"));
        for (case_index, case) in cases.iter().enumerate() {
            let positions: Vec<_> = case.positions.iter().map(|p|Vec3::from_array(p.map(f32::from_bits))).collect();
            let tangents: Vec<_> = case.tangents.iter().map(|p|Vec2::from_array(p.map(f32::from_bits))).collect();
            let widths: Vec<_> = case.widths.iter().copied().map(f32::from_bits).collect();
            let mut query_index = 0;
            let result = split_stair_support_samples(&positions, &tangents, &widths,
                ||Vec2::from_array(case.normal.map(f32::from_bits)), |origin, direction| {
                    let query = &case.queries[query_index]; query_index += 1;
                    assert_eq!([origin.to_array().map(f32::to_bits), direction.to_array().map(f32::to_bits)],
                        query.ray, "原ray逐字 case {case_index} query {query_index}");
                    if query.hit { RaycastSimpleResult { handle: [0; 3],
                        position: Vec3::from_array(query.position.map(f32::from_bits)), toi: 0.0 }
                    } else { RaycastSimpleResult::miss() }
                });
            assert_eq!(query_index, case.queries.len());
            let ranges = |spans: &[std::ops::RangeInclusive<usize>]| spans.iter()
                .map(|span| [*span.start(), *span.end()]).collect::<Vec<_>>();
            assert_eq!(ranges(&result.unsupported), case.unsupported, "原无支撑区间 case {case_index}");
            assert_eq!(ranges(&result.supported), case.supported, "原支撑区间 case {case_index}");
            let distances: Vec<Vec<_>> = result.distances.iter()
                .map(|distances|distances.iter().map(|v|v.to_bits()).collect()).collect();
            assert_eq!(distances.iter().map(Vec::as_slice).collect::<Vec<_>>(), case.distances,
                "原支撑距离逐字 case {case_index}");
        }
    }

    #[test]
    fn actual_support_partition_uses_world_mask_toi_edit_and_remove() {
        use country_core::resources::{stairs::stairs_assembly_params::StairWallAnchor, walls::WallId};
        let positions = vec![Vec3::new(-0.5, 1.5, 0.0), Vec3::new(0.0, 1.5, 0.0),
            Vec3::new(0.5, 1.5, 0.0), Vec3::new(1.0, 1.5, 0.0)];
        let curve = utils::curve::Curve3::crashy_new_from_points(positions, false);
        let tangents = [Vec2::X; 4]; let widths = [1.0; 4];
        let anchor = StairWallAnchor::new(WallId(3), 0.0);
        let mut world = RaycastWorld::new(); let mut registry = std::collections::HashMap::new();
        let query = |world: &RaycastWorld| split_segment_into_supported_and_unsupported_subsegments(
            world, &curve, &tangents, &widths, anchor);
        let missing = query(&world);
        assert_eq!(missing.unsupported, vec![0..=3]);
        assert_eq!(missing.supported, vec![0..=0]);
        // 此处是消费者的明确测试墙体，不替代任何生产楼梯/屋顶几何。
        let mesh = std::sync::Arc::new(RaycastMeshData::new_cuboid_at(Vec3::new(2.0, 2.0, 0.1), Vec3::ZERO));
        upsert_collider(&mut world, &mut registry, 3_u32, RaycastGeoSource::Slot0,
            mesh.clone(), Vec3::new(0.0, 0.0, 0.5), glam::Quat::IDENTITY);
        world.flush_colliders();
        let supported = query(&world);
        assert!(supported.unsupported.is_empty());
        assert_eq!(supported.supported, vec![0..=3]);
        assert_eq!(supported.distances[0].len(), 4);
        for _ in 0..3 { assert_eq!(query(&world), supported); }
        upsert_collider(&mut world, &mut registry, 3_u32, RaycastGeoSource::Slot0,
            mesh, Vec3::new(0.0, 0.0, 2.5), glam::Quat::IDENTITY);
        world.flush_colliders();
        assert_eq!(query(&world), missing, "编辑移出原1.78射线界限");
        remove_collider(&mut world, &mut registry, &3_u32);
        world.flush_colliders();
        assert!(registry.is_empty()); assert_eq!(query(&world), missing);
    }

    #[test]
    fn original_floor_atlas_inline_boundaries() {
        let cases: &[(u32,u32)] = include!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m01-stair-floor-atlas-native-fixtures.rs"));
        for &(tri_count, expected) in cases { assert_eq!(floor_tri_count_to_atlas_idx(tri_count), expected); }
    }
}

#[cfg(test)]
mod original_stair_wall_cutout_tests {
    use super::*;
    use bevy_ecs::system::RunSystemOnce;
    use country_core::resources::{stairs::visual::StairsWallCutout,
        walls::{WallId, wall_holes::{PrevWallHoles, WallHoles}}};
    struct HoleNativeCase {
        buckets: &'static [&'static [[u64; 5]]],
        expected: &'static [[u64; 6]],
    }
    fn outputs(world: &World) -> Vec<[u64; 6]> {
        let mut result = Vec::new();
        for (wall, storage) in &world.resource::<WallHoles>().storage {
            for hole in storage.raw_holes() { result.push([wall.0,
                u64::from(hole.center[0].to_bits()), u64::from(hole.center[1].to_bits()),
                u64::from(hole.dimensions[0].to_bits()), u64::from(hole.dimensions[1].to_bits()),
                u64::from(hole.kind)]); }
        }
        result.sort(); result
    }
    #[test]
    fn original_stair_wall_cutouts_all_buckets_float_bits_duplicates_world_frames_and_removal() {
        let cases: &[HoleNativeCase] = include!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m01-stair-hole-cache-native-fixtures.rs"));
        for case in cases {
            let mut visual = StairsVisualState::default();
            for (index, records) in case.buckets.iter().enumerate() {
                let edge = StairGraphEdge::new(index as u32, index as u32 + 100);
                visual.reset_edge(edge);
                visual.get_mut(&edge).unwrap().wall_cutouts = records.iter().map(|record| {
                    StairsWallCutout {wall: WallId(record[0]),
                        center: Vec2::new(f32::from_bits(record[1] as u32), f32::from_bits(record[2] as u32)),
                        dimensions: Vec2::new(f32::from_bits(record[3] as u32), f32::from_bits(record[4] as u32))}
                }).collect();
            }
            let mut world = World::new(); world.insert_resource(visual);
            world.insert_resource(WallHoles::default());
            let mut expected = case.expected.to_vec(); expected.sort();
            world.run_system_once(playermade_stairs_add_wall_holes).unwrap();
            // HashMap跨进程顺序不是业务合同；原每个缓存Vec中的重复记录必须全保留。
            assert_eq!(outputs(&world), expected);
            let mut previous = PrevWallHoles::default();
            world.resource_mut::<WallHoles>().clear_for_next_frame(&mut previous);
            world.run_system_once(playermade_stairs_add_wall_holes).unwrap();
            assert_eq!(outputs(&world), expected, "下一帧重新发布但不清边缓存");
            world.resource_mut::<WallHoles>().clear_for_next_frame(&mut previous);
            let edges: Vec<_> = world.resource::<StairsVisualState>().edges().keys().copied().collect();
            for edge in edges { world.resource_mut::<StairsVisualState>().remove_edge(edge); }
            world.run_system_once(playermade_stairs_add_wall_holes).unwrap();
            assert!(outputs(&world).is_empty(), "删除边后不能再发布旧切孔");
        }
    }
}

#[cfg(test)]
mod original_stair_step_tests {
    use super::*;
    use bevy_ecs::system::RunSystemOnce;
    struct StepNativeCase {
        positions: &'static [[u32; 3]], widths: &'static [u32], tangents: &'static [[u32; 2]],
        aux: &'static [u32], terrain: &'static [u32], end_flags: u16, inset: bool,
        half_height: bool, final_seed: u64, records: &'static [[u32; 24]],
    }

    #[test]
    fn actual_stair_step_resource_adapter_terrain_sparse_edit_stable_frames_and_release() {
        use country_core::{geometry::instanced_wall::BrickTransformSsbo,
            render::{sparse_instance_buffer::{MeshAtlasAllocation, SparseInstanceBuffer, SparseSsbo},
                ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc, GraphicsDevice}},
            resources::terrain_height::TerrainHeightsData};
        use std::{ops::Range, sync::{Mutex, atomic::Ordering}};
        #[derive(Default)]
        struct UploadDevice(Mutex<usize>);
        impl GraphicsDevice for UploadDevice {
            fn create_buffer(&self, desc: &BufferDesc, _: &str, _: Option<&[u8]>) -> Result<Buffer, BackendError> {
                Ok(Buffer { byte_size: desc.size as u64, ..Default::default() })
            }
            fn update_buffer_bytes(&self, buffer: &Buffer, data: &[u8], ranges: &[Range<usize>], offset: usize)
                -> Result<(), BackendError>
            {
                for range in ranges { assert!(range.end <= data.len());
                    assert!(offset + range.end <= buffer.byte_size as usize); }
                *self.0.lock().unwrap() += 1;
                Ok(())
            }
        }
        #[derive(Resource)]
        struct Output {
            buffer: SparseInstanceBuffer<BrickTransformSsbo>,
            allocation: MeshAtlasAllocation,
        }
        // 测试适配器只转交真实上游请求与terrain，不冒充尚未恢复的主装配system。
        fn write_steps(requests: Res<PlayermadeStairsAssemblyRequests>, terrain: Res<TerrainHeightsData>,
            mut output: ResMut<Output>)
        {
            let request = &requests.requests[0];
            let curve = request.interpolated_curve.as_ref().expect("真实插值曲线");
            let Output {buffer, allocation} = &mut *output;
            let mut writer = buffer.writer_for(allocation.allocation());
            writer.clear();
            construct_stair_steps(&mut writer, StairStepSamples {positions: &curve.points,
                widths: &request.widths, tangents: &request.tangents,
                auxiliary_samples: &request.auxiliary_samples},
                StairStepOptions {exclude_end_extra: [false;2], inset_width: false,
                    minimum_width: 0.1, half_height: false}, &terrain, 42,
                &fastrand::Rng::with_seed(19));
        }
        let device = UploadDevice::default();
        let buffer = SparseInstanceBuffer::<BrickTransformSsbo>::new_storage_buffer(&device, None);
        let allocation = MeshAtlasAllocation::new(&buffer);
        let mut world = super::interpolate_history_tests::generated_world(2.0);
        world.run_system_once(playermade_stairs_interpolate_and_compare).unwrap();
        {
            let mut requests = world.resource_mut::<PlayermadeStairsAssemblyRequests>();
            let request = &mut requests.requests[0];
            assert!(request.auxiliary_samples.is_empty());
            // 已恢复producer确实留空aux；后续原写入阶段的完整恢复仍待核验。
            // 本测试只验已恢复leaf的资源边界，显式给定aux输入，不能算全请求链验收。
            request.auxiliary_samples = vec![0.75; request.interpolated_curve.as_ref().unwrap().points.len()];
        }
        // 请求采样使用VisualTerrainHeightSsbo；原装配参数另读TerrainHeightSsbo，
        // 两者TypeId不同，测试必须分别提供真实资源，不能借前者冒充后者。
        let mut terrain: TerrainHeightsData = TerrainHeightsData::new();
        terrain.set_height_range(-20.0, -20.0);
        world.insert_resource(terrain);
        world.insert_resource(Output {buffer, allocation});
        world.run_system_once(write_steps).unwrap();
        let first = {
            let output = world.resource::<Output>();
            let count = usize::from(output.buffer.byte_map[0]);
            assert!(count > 0, "真实请求必须产生实际梯步实例");
            assert!(output.buffer.dirty_bitset.iter().any(|word| word.load(Ordering::Relaxed) != 0));
            output.buffer.data[..count].to_vec()
        };
        world.resource_mut::<Output>().buffer.gpu_flush(&device).unwrap();
        for _ in 0..3 { world.resource_mut::<Output>().buffer.gpu_flush(&device).unwrap(); }
        assert_eq!(*device.0.lock().unwrap(), 1, "静止帧不重复上传");
        world.resource_mut::<TerrainHeightsData>().set_height_range(1.5, 1.5);
        world.run_system_once(write_steps).unwrap();
        let output = world.resource::<Output>();
        assert_ne!(output.buffer.data[..first.len()], first, "真实地形变化应改变逐砖变换");
        world.resource_mut::<Output>().buffer.gpu_flush(&device).unwrap();
        assert_eq!(*device.0.lock().unwrap(), 2);
        let Output {mut buffer, allocation} = world.remove_resource::<Output>().unwrap();
        drop(allocation);
        assert_eq!(buffer.waste.lock().len(), 1);
        buffer.gpu_flush(&device).unwrap();
        assert_eq!(buffer.free_chunks.read().len(), BrickTransformSsbo::CHUNK_COUNT);
        assert!(buffer.byte_map[..BrickTransformSsbo::CHUNK_COUNT].iter().all(|&count| count == 0));
        assert_eq!(buffer.byte_map[BrickTransformSsbo::CHUNK_COUNT], 32, "原满块哨兵仍保留");
        assert!(buffer.active_words().all(|word| word == 0));
        assert!(buffer.waste.lock().is_empty());
    }

    #[test]
    fn original_stair_steps_curves_all_width_classes_auxiliary_gates_end_flags_and_full_records() {
        let cases: &[StepNativeCase] = include!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m01-stair-steps-native-fixtures.rs"));
        for (case_index, case) in cases.iter().enumerate() {
            let positions: Vec<_> = case.positions.iter().map(|p|Vec3::from_array(p.map(f32::from_bits))).collect();
            let tangents: Vec<_> = case.tangents.iter().map(|p|Vec2::from_array(p.map(f32::from_bits))).collect();
            let widths: Vec<_> = case.widths.iter().copied().map(f32::from_bits).collect();
            let auxiliary_samples: Vec<_> = case.aux.iter().copied().map(f32::from_bits).collect();
            let samples = StairStepSamples { positions: &positions, widths: &widths,
                tangents: &tangents, auxiliary_samples: &auxiliary_samples };
            let options = StairStepOptions { exclude_end_extra: [case.end_flags & 1 != 0,
                case.end_flags & 256 != 0], inset_width: case.inset, minimum_width: 0.1,
                half_height: case.half_height };
            let rng = fastrand::Rng::with_seed(0x1234_5678_9abc_def0);
            let mut records = Vec::new(); let mut sample_index = 0;
            emit_stair_steps(samples, options, 42, &rng, |point| {
                let a = positions[sample_index]; let b = positions[sample_index + 1];
                assert_eq!(point.to_array().map(f32::to_bits),
                    [b.x * 0.5 + a.x * 0.5, b.z * 0.5 + a.z * 0.5].map(f32::to_bits));
                let result = f32::from_bits(case.terrain[sample_index]); sample_index += 1; result
            }, |record| records.push(record));
            assert_eq!(sample_index, case.terrain.len(), "原地形采样数 case {case_index}");
            assert_eq!(rng.get_seed(), case.final_seed, "原最终RNG case {case_index}");
            assert_eq!(records.len(), case.records.len(), "原逐砖数量 case {case_index}");
            for (piece_index, (record, expected)) in records.iter().zip(case.records).enumerate() {
                let bytes = record.observed_bytes();
                let actual: [u32; 24] = std::array::from_fn(|i|
                    u32::from_le_bytes(bytes[i * 4..i * 4 + 4].try_into().unwrap()));
                assert_eq!(&actual, expected, "原96B case {case_index} piece {piece_index}");
            }
        }
    }
}
