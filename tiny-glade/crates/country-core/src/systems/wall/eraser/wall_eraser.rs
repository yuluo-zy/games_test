//! B `systems/wall/eraser/wall_eraser.rs`: freehand wall eraser.
//!
//! Evidence: `chapters/systems/wall/eraser/wall_eraser.rs.md` (O, 0x140accab0).
//! Each stroke slices the wall curve, marks the enclosure freehand, clamps the
//! height by the remaining length (`wall_height_based_on_length`, 0.75..14.0)
//! and applies the flat-roof `-0.79` compensation; the original snapshot is
//! cached in the archivist for the finish history. The ChildWall split /
//! rewiring path is 待取证.

use bevy_ecs::{prelude::*, system::SystemParam};

use super::archivist::{ErasedRange, WallEraserArchivist};
use crate::resources::history::wall_edit::slice_source_curve;
use crate::resources::walls::publication_fields::WallShapeSnapshot;
use crate::resources::{
    roofs::Roof,
    walls::{InnerWalls, WallState, public_walls::PublicWalls},
};

/// Height lerp range for `wall_height_based_on_length`
/// (0.75 @0x1429edb90, 14.0 @0x142a7de00).
const MIN_HEIGHT: f32 = 0.75;
const MAX_HEIGHT: f32 = 14.0;
/// Flat-roof erase compensation (-0.79, _UNK_142a7e320).
const FLAT_ROOF_HEIGHT_DELTA: f32 = -0.79;

fn freehand_shape() -> WallShapeSnapshot {
    let mut bytes = [0; 28];
    bytes[..4].copy_from_slice(&2u32.to_le_bytes());
    WallShapeSnapshot::from_observed(bytes)
}

/// 原 140aceb8d：以切片长度在 [0.5, 6.0] 内逆插值，再限制到当前墙高。
/// 比较顺序保留 SSE 对无序值及相等值的选择，不用 Rust 的 clamp 替代。
fn wall_height_based_on_length(length: f32, current: f32) -> f32 {
    let t = (length - 0.5) / (6.0 - 0.5);
    let t = if 0.0 > t { 0.0 } else { t };
    let t = if 1.0 < t { 1.0 } else { t };
    let height = t * MAX_HEIGHT + (1.0 - t) * MIN_HEIGHT;
    if height.is_nan() || current < height { current } else { height }
}

/// 原 1405349b0 定型的最后一个参数：沿用现有真实事件 owner。
#[derive(SystemParam)]
#[allow(dead_code)]
struct EraserEventWriters<'w> {
    material: EventWriter<'w, crate::systems::audio::RequestWallMaterialAudio>,
    audio: EventWriter<'w, crate::systems::audio::PlayWallAudio>,
    wall_color: EventWriter<'w, crate::resources::color_id::SetWallColorIdCmd>,
    timber_color: EventWriter<'w, crate::resources::color_id::SetHalftimberColorIdCmd>,
    tool_used: EventWriter<'w, crate::resources::onboarding::NotifyHintSystemToolWasJustUsed>,
    erased: EventWriter<'w, crate::resources::onboarding::NotifyHintSystemSomethingWasErased>,
    reanchor: EventWriter<'w, crate::resources::events::ReanchorDecorators>,
    ivy: EventWriter<'w, crate::systems::ivy::ivy_pruner::RemoveIvyCmd>,
}

/// 原 clearing canvas 的相位；Start 同时落入活动刷子分支。
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EraserUiPhase {
    Inactive,
    Start,
    Continue,
    Finish,
}

/// 原 140accab0..140accd98：只有 All/Wall 注册画布，inactive 的 widget
/// `dragged()` 返回 2；3 重置归档器后继续刷，4 继续，其他相位结束。
#[allow(dead_code)]
fn eraser_ui_phase(
    mode: &crate::systems::mode_manager::AppMode,
    ui: &crate::systems::ui::resources::ui_state::UiState,
    highlight: &mut crate::resources::editing_highlight::EditingHighlight,
) -> EraserUiPhase {
    use crate::systems::mode_manager::{AppMode, EraserLayer};
    if !matches!(mode, AppMode::Eraser(EraserLayer::All | EraserLayer::Wall)) {
        return EraserUiPhase::Inactive;
    }
    let widget = ui.clearing_canvas_w_layer_and_sound("eraser", 6, 3);
    if widget.is_active() {
        // 原 140accd30：仅活动 widget 写入高亮，RGB 为原常量 0.1、0.03、0。
        highlight.color = glam::Vec3::new(0.1, 0.03, 0.0);
        highlight.enabled = true;
    }
    match widget.dragged() {
        2 => EraserUiPhase::Inactive,
        3 => EraserUiPhase::Start,
        4 => EraserUiPhase::Continue,
        _ => EraserUiPhase::Finish,
    }
}

/// 原 140acceeb..140accfa0：结束时先按 map3 发擦除提示，随后同步写
/// History；成功才发工具使用提示。原 finish 成功后清 map1，后续 reanchor 遍历为空。
#[allow(dead_code)]
fn finish_eraser_input<S>(
    eraser: &mut WallEraserArchivist,
    inner: &InnerWalls,
    colors: &crate::resources::color_id::WallColorIds,
    timber_colors: &crate::resources::color_id::HalftimberColorIds,
    history: &mut crate::resources::history::History<S>,
    events: &mut EraserEventWriters<'_>,
) {
    if eraser.has_pending_child_walls() {
        events.erased.write(crate::resources::onboarding::NotifyHintSystemSomethingWasErased);
    }
    eraser.finish_recorded(colors, timber_colors, history, inner)
        .expect("墙擦除历史写入失败");
    events.tool_used.write(crate::resources::onboarding::NotifyHintSystemToolWasJustUsed);
}

/// 原 140acd330..140acd351：使用真实地形命中投影和擦除倍率；
/// 未命中时 CursorTerrainRaycast::position 与原版一致返回零向量。
#[allow(dead_code)]
fn eraser_brush(
    cursor: &crate::resources::cursor_terrain_raycast::CursorTerrainRaycast,
    multiplier: &crate::resources::walls::EraserSizeMultiplier,
) -> utils::geometry::Circle2d {
    let position = cursor.position();
    utils::geometry::Circle2d::new(
        glam::Vec2::new(position.x, position.z),
        multiplier.0 * f32::from_bits(0x3ff1_eb84),
    )
}

/// 原 140acd55d..140ace2b2 的预制墙入口：先以最近点和半径筛选；
/// 可重接的闭合曲线依次写曲线与 freehand 外形，保留两次权威变更通知。
/// 返回 false 代表此墙位于刷圆外；None 重接则沿用原曲线进入后续相交。
#[allow(dead_code)]
fn prepare_enclosed_wall(
    inner: &mut InnerWalls,
    wall: crate::resources::walls::WallId,
    center: glam::Vec2,
    radius: f32,
) -> bool {
    let state = inner.get_unwrapped(wall);
    let source = &state.snapshot().source;
    if source.curve_kind_tag == 2 {
        return true;
    }
    let curve = utils::curve::Curve2::from_cached_parts(
        source.curve.points.iter().copied().map(glam::Vec2::from_array).collect(),
        source.curve.points_u.clone(), source.curve.length,
    );
    let coord = curve.get_approx_coord_from_pos(center);
    let point = curve.get_pos_at_coord(coord);
    let distance = (point - center).length();
    // 原 UCOMISS/JBE 对无序同样跳过该墙。
    if radius.is_nan() || distance.is_nan() || radius <= distance {
        return false;
    }
    let u0 = curve.points_u[coord.segment];
    let u1 = curve.points_u[coord.segment + 1];
    assert!((0.0..=1.0).contains(&coord.t), "原最近点参数超出区间");
    // 原 140c904d0 是 u0 + (u1 - u0) * t，与点坐标 lerp 的顺序不同。
    let u = utils::scalar::lerp(u0, u1, coord.t);
    if let Some(rewired) = crate::utils::geo::rewire_enclosed_curve(&curve, u) {
        let authority = crate::resources::roofs::geometry::surface::Curve2 {
            points: rewired.points.iter().map(|p| p.to_array()).collect(),
            points_u: rewired.points_u,
            length: rewired.length,
        };
        inner.update_wall_state_curve(wall, authority);
        inner.update_wall_state_enclosure(wall, freehand_shape());
    }
    true
}

/// 原 140ace441..140ace657：全擦先更新子墙追踪，再移除权威墙，最后发音频。
#[allow(dead_code)]
fn erase_entire_wall(
    wall: crate::resources::walls::WallId,
    cursor: &crate::resources::cursor_terrain_raycast::CursorTerrainRaycast,
    inner: &mut InnerWalls,
    archivist: &mut WallEraserArchivist,
    events: &mut EraserEventWriters<'_>,
) {
    archivist.add_fully_erased_event(super::archivist::OriginalWallId(wall.0));
    let removed = inner.remove(wall);
    events.audio.write(crate::systems::audio::PlayWallAudio {
        kind: if removed.source_flag_65() != 0 { 6 } else { 0 },
        // 原 kind 0/6 不携带 scalar；现有类型要求 f32，消费者对这两种不读取它。
        value: 0.0,
        position: cursor.position(),
        volume: 1.0,
        flag: 2,
    });
}

/// 原 140ace69f..140aced37 的单保留段：事件、裁曲线、子墙归档器、
/// 活墙查询和完整状态替换必须按此顺序。source 为同帧原始墙状态。
#[allow(dead_code)]
fn retain_single_wall_segment(
    wall: crate::resources::walls::WallId,
    source: &WallState,
    range: ErasedRange,
    was_premade: bool,
    cursor: &crate::resources::cursor_terrain_raycast::CursorTerrainRaycast,
    roof: Option<&Roof>,
    inner: &mut InnerWalls,
    archivist: &mut WallEraserArchivist,
    events: &mut EraserEventWriters<'_>,
) {
    events.material.write(crate::systems::audio::RequestWallMaterialAudio {
        id: wall.0, position: cursor.position(), kind: 2, flag: 2,
    });
    // 预制类型在重接前取值；source 则是重接后参与切片的活状态。
    let flat_roof = was_premade && roof.is_some_and(Roof::is_flat_w_no_rim);
    let sliced = slice_source_curve(&source.snapshot().source.curve, range.low, range.high)
        .expect("墙擦除保留段曲线切片失败");
    assert!(sliced.points.len() >= 2 && sliced.length > 0.0,
        "墙擦除保留段曲线无效");
    archivist.shrink_wall(super::archivist::OriginalWallId(wall.0), range.low, range.high);
    let live_height = inner.get_unwrapped(wall).height();
    let height = wall_height_based_on_length(sliced.length, live_height)
        + if flat_roof { FLAT_ROOF_HEIGHT_DELTA } else { 0.0 };
    let mut next = source.snapshot().clone();
    next.source.curve = sliced;
    next.source.shape = freehand_shape();
    next.source.curve_kind_tag = 2;
    next.source.extra = height;
    next.source.terrain_floor = 0.0;
    next.source.style = u8::from(next.source.style == 1);
    next.metadata.source_69 = 5; // 原入口的 DEV_OVERRIDE_IS_DEMO_MODE 启动值为 0。
    let freehand = source.freehand_ty() | u8::from(flat_roof);
    inner.update_wall_state(wall, WallState::from_snapshot(next, freehand));
}

/// 原 140aceb8d..140acf112 与多段创建路径：每个新墙取自己的切片长度，
/// 对自由高度分支按活地形与公开墙顶计算上限；缺公开墙时沿用原高度。
#[allow(dead_code)]
fn split_piece_state(
    source: &WallState,
    sliced: crate::resources::roofs::geometry::surface::Curve2,
    was_premade: bool,
    roof: Option<&Roof>,
    public: &PublicWalls,
    wall: crate::resources::walls::WallId,
    terrain: &crate::resources::terrain_height::TerrainHeightsData,
) -> WallState {
    assert!(sliced.points.len() >= 2 && sliced.length > 0.0,
        "墙擦除子段曲线无效");
    let flat_roof = was_premade && roof.is_some_and(Roof::is_flat_w_no_rim);
    let freehand = source.freehand_ty() | u8::from(flat_roof);
    let mut height = source.height() + if flat_roof { FLAT_ROOF_HEIGHT_DELTA } else { 0.0 };
    if source.snapshot().source.curve_kind_tag >= 2 && freehand & 1 != 0 {
        let mut highest = -2.5f32;
        for point in &sliced.points {
            let sample = terrain.sample_at_world_xz(point[0], point[1]);
            // 原 MAXSS 的无序选择先取新样本，不能用普通 max 隐藏 NaN。
            highest = if highest.is_nan() || highest < sample { sample } else { highest };
        }
        if let Some(public) = public.get(wall) {
            height = public.max_y() - highest;
        }
    }
    let mut next = source.snapshot().clone();
    next.source.extra = wall_height_based_on_length(sliced.length, height);
    next.source.curve = sliced;
    next.source.curve_kind_tag = 2;
    next.source.shape = freehand_shape();
    next.source.terrain_floor = 0.0;
    next.source.style = u8::from(next.source.style == 1);
    next.metadata.source_69 = 5;
    WallState::from_snapshot(next, freehand)
}

/// 原 140ace6a2 之后的多保留段：先逐段建新墙，再删除原墙、记录重接并分发装饰。
/// 场景音与两种颜色命令使用同一帧的真实来源和原墙颜色表。
#[allow(dead_code)]
fn split_wall_segments(
    wall: crate::resources::walls::WallId,
    source: &WallState,
    ranges: &[[f32; 2]],
    was_premade: bool,
    roof: Option<&Roof>,
    public: &PublicWalls,
    terrain: &crate::resources::terrain_height::TerrainHeightsData,
    cursor: &crate::resources::cursor_terrain_raycast::CursorTerrainRaycast,
    colors: &crate::resources::color_id::WallColorIds,
    timber_colors: &crate::resources::color_id::HalftimberColorIds,
    inner: &mut InnerWalls,
    storage: &mut crate::resources::walls::decorator_storage::DecoratorStorage,
    resources: &crate::resources::walls::decorator_archivist::DecoratorArchivalResources<'_>,
    decorators: &mut crate::resources::walls::decorator_archivist::LiveDecoratorArchivist,
    eraser: &mut WallEraserArchivist,
    events: &mut EraserEventWriters<'_>,
) {
    use crate::resources::walls::decorator_storage::{DecoratorStorageAddr, Decorators};
    // 原先构造全部切片，全部成功后才查询颜色和创建任何子墙。
    let slices: Vec<_> = ranges.iter().map(|&[low, high]|
        slice_source_curve(&source.snapshot().source.curve, low, high)
            .expect("墙擦除子段曲线切片失败")).collect();
    let timber = source.source_flag_65() != 0;
    let color = if timber { timber_colors.0[wall.0 as usize] } else { colors.0[wall.0 as usize] };
    let mut pieces = Vec::with_capacity(ranges.len());
    for (&[low, high], slice) in ranges.iter().zip(slices) {
        let state = split_piece_state(source, slice, was_premade, roof, public, wall, terrain);
        let id = inner.new_wall_from_curve(state);
        assert!(low < high, "墙擦除子段区间无效");
        pieces.push((id.0, low, high));
        // 原新墙归档传入新构造的空桶，而非查询可能残留的存储桶。
        decorators.archive_storage_decorators(0, &Decorators::default(), Some(id));
        if timber {
            events.timber_color.write(crate::resources::color_id::SetHalftimberColorIdCmd::new(id, color, 0));
        } else {
            events.wall_color.write(crate::resources::color_id::SetWallColorIdCmd::new(id, color, 0));
        }
    }
    inner.remove(wall);
    eraser.add_split_event_pieces(wall, &pieces);
    decorators.archive_current_decorators_at_with_resources(
        DecoratorStorageAddr::Wall(wall), storage, resources, "wall_eraser",
    );
    decorators.decorator_cull_allow_wall(wall, storage);
    let first = pieces.first().expect("墙擦除多段结果为空").0;
    events.material.write(crate::systems::audio::RequestWallMaterialAudio {
        id: first, position: cursor.position(), kind: 3, flag: 2,
    });
    let curve = utils::curve::Curve2::from_cached_parts(
        source.snapshot().source.curve.points.iter().copied().map(glam::Vec2::from_array).collect(),
        source.snapshot().source.curve.points_u.clone(), source.snapshot().source.curve.length,
    );
    let children: Vec<_> = pieces.into_iter().map(|(id, low, high)|
        (crate::resources::walls::WallId(id), low, high)).collect();
    storage.on_wall_split(wall, &curve, &children);
}

/// 原 140acd98f..140acde08：从同一墙曲线的点和缓存 U 得到刷圆外的区间。
/// 只计算原始区间；原调用者须先处理完整保留早退与归档，再进行短段过滤。
#[allow(dead_code)]
fn original_maxss_u_end(u: f32, end: f32) -> f32 {
    if u > end { u } else { end }
}

fn outside_brush_ranges(
    curve: &crate::resources::roofs::geometry::surface::Curve2,
    brush: utils::geometry::Circle2d,
) -> Vec<[f32; 2]> {
    use glam::Vec2;
    use utils::geometry::Ray2;
    let mut ranges: Vec<[f32; 2]> = Vec::new();
    let (mut start, mut end) = (0.0f32, 0.0f32);
    let mut push = |start: f32, end: f32| {
        if start == 0.0 && end == 0.0 { return; }
        if let Some(last) = ranges.last_mut() {
            if (start - last[1]) * curve.length < 0.1 {
                last[1] = end;
                return;
            }
        }
        ranges.push([start, end]);
    };
    assert!(curve.points.len() >= 2 && curve.points_u.len() >= curve.points.len(),
        "原墙曲线点或参数缓存无效");
    let segment_count = curve.points.len() - 1;
    for i in 0..segment_count {
        let a = Vec2::from_array(curve.points[i]);
        let b = Vec2::from_array(curve.points[i + 1]);
        let u0 = curve.points_u[i];
        let u1 = curve.points_u[i + 1];
        let Some((near, far)) = brush.intersect_ray(Ray2 { origin: a, direction: b - a }) else {
            // MAXSS 的目的操作数为 u1，源为旧 end；无序或相等时取源。
            end = original_maxss_u_end(u1, end);
            continue;
        };
        if near > 1.0 || far < 0.0 {
            end = original_maxss_u_end(u1, end);
            continue;
        }
        // 原交点 U 的 UCOMISS/JB 也拒绝无序；不得让 NaN 静默跳过边界断言。
        assert!(!near.is_nan() && !(near < 0.0 && far.is_nan()), "原 CurveU 交点无序");
        if near >= 0.0 {
            assert!(near <= 1.0, "原 CurveU 交点超出区间");
            end = u0 + (u1 - u0) * near;
        }
        push(start, end);
        start = 0.0;
        end = 0.0;
        if far <= 1.0 {
            assert!(far >= 0.0, "原 CurveU 交点超出区间");
            start = u0 + (u1 - u0) * far;
            end = u1;
        }
    }
    push(start, end);
    ranges
}

/// 原 140acdf29..140acdf66：受影响墙先归档装饰与活碰撞载荷，
/// 再允许该墙的装饰剔除，最后缓存墙原态；三者不能移到短段过滤之后。
#[allow(dead_code)]
fn archive_affected_wall(
    wall: crate::resources::walls::WallId,
    snapshot: &crate::resources::history::wall_edit::WallAuthoritySnapshot,
    storage: &crate::resources::walls::decorator_storage::DecoratorStorage,
    resources: &crate::resources::walls::decorator_archivist::DecoratorArchivalResources<'_>,
    decorators: &mut crate::resources::walls::decorator_archivist::LiveDecoratorArchivist,
    eraser: &mut WallEraserArchivist,
) {
    use crate::resources::walls::decorator_storage::DecoratorStorageAddr;
    decorators.archive_current_decorators_at_with_resources(
        DecoratorStorageAddr::Wall(wall), storage, resources, "wall_eraser",
    );
    decorators.decorator_cull_allow_wall(wall, storage);
    eraser.cache_if_original_wall(wall, snapshot, ErasedRange { low: 0.0, high: 1.0 });
}

/// B 140acdf66–140ace406：小于0.2或无序的残段被删除，顺序保持不变。
fn remove_short_brush_ranges(ranges: &mut Vec<[f32; 2]>, length: f32) {
    ranges.retain(|range| (range[1] - range[0]) * length >= 0.2);
}

#[cfg(test)]
mod original_height_tests {
    use super::wall_height_based_on_length;

    #[test]
    fn interval_maxss_preserves_source_on_unordered_and_equal() {
        let nan = f32::from_bits(0x7fc12345);
        assert_eq!(super::original_maxss_u_end(1.0, nan).to_bits(), nan.to_bits());
        assert_eq!(super::original_maxss_u_end(nan, 1.0).to_bits(), 1.0f32.to_bits());
        assert_eq!(super::original_maxss_u_end(0.0, -0.0).to_bits(), (-0.0f32).to_bits());
    }

    fn init_eraser_events(world: &mut bevy_ecs::world::World) {
        use bevy_ecs::event::Events;
        world.init_resource::<Events<crate::systems::audio::RequestWallMaterialAudio>>();
        world.init_resource::<Events<crate::systems::audio::PlayWallAudio>>();
        world.init_resource::<Events<crate::resources::color_id::SetWallColorIdCmd>>();
        world.init_resource::<Events<crate::resources::color_id::SetHalftimberColorIdCmd>>();
        world.init_resource::<Events<crate::resources::onboarding::NotifyHintSystemToolWasJustUsed>>();
        world.init_resource::<Events<crate::resources::onboarding::NotifyHintSystemSomethingWasErased>>();
        world.init_resource::<Events<crate::resources::events::ReanchorDecorators>>();
        world.init_resource::<Events<crate::systems::ivy::ivy_pruner::RemoveIvyCmd>>();
    }

    #[test]
    fn sliced_length_uses_original_inverse_lerp_and_operation_order() {
        // 原 B 指令段 140aceb8d..140acebd9 的 x86_64 逐位输出；
        // 调用者对真实切片另有点数和正长度门，0 只验证该算术片段。
        for (length, expected) in [
            (0x00000000, 0x3f400000), (0x3f000000, 0x3f400000),
            (0x3f400000, 0x3fad1746), (0x3f800000, 0x3ffa2e8b),
            (0x40000000, 0x408ba2e9), (0x40400000, 0x40d8ba2f),
            (0x40b00000, 0x414cba2f), (0x40c00000, 0x41600000),
            (0x40e00000, 0x41600000),
        ] {
            assert_eq!(wall_height_based_on_length(f32::from_bits(length), 100.0).to_bits(), expected);
        }
        assert_eq!(wall_height_based_on_length(6.0, 3.0), 3.0);
    }

    #[test]
    fn brush_outside_ranges_match_original_segment_loop() {
        use super::outside_brush_ranges;
        use crate::resources::roofs::geometry::surface::Curve2;
        use glam::Vec2;
        use utils::geometry::Circle2d;
        let curve = Curve2::try_new(vec![[0., 0.], [2., 0.], [4., 0.], [6., 0.]], false).unwrap();
        for (center, radius, expected) in [
            ((3., 0.), 1., vec![[0x00000000, 0x3eaaaaab], [0x3f2aaaab, 0x3f800000]]),
            ((3., 3.), 1., vec![[0x00000000, 0x3f800000]]),
            ((0., 0.), 1., vec![[0x3e2aaaab, 0x3f800000]]),
            ((3., 0.), 4., vec![]),
            ((2., 0.), 0.15, vec![[0x00000000, 0x3e9dddde], [0x3eb77778, 0x3f800000]]),
            ((2., 0.), 0.02, vec![[0, 0x3f800000]]),
            ((3., 1.), 1., vec![[0, 0x3f800000]]),
            ((3., 0.), 0.05, vec![[0, 0x3f800000]]),
        ] {
            let actual: Vec<[u32; 2]> = outside_brush_ranges(&curve, Circle2d::new(Vec2::new(center.0, center.1), radius))
                .into_iter().map(|range| range.map(f32::to_bits)).collect();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn real_eraser_ui_drives_zero_one_and_multiple_segments_into_history() {
        use super::wall_eraser;
        use crate::resources::color_id::{HalftimberColorIds, WallColorIds};
        use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
        use crate::resources::editing_highlight::EditingHighlight;
        use crate::resources::history::{EditType, History};
        use crate::resources::roofs::geometry::surface::Curve2;
        use crate::resources::terrain_height::TerrainHeightsData;
        use crate::resources::walls::{EraserSizeMultiplier, InnerWalls, WallId, WallState};
        use crate::resources::walls::decorator_archivist::LiveDecoratorArchivist;
        use crate::resources::walls::decorator_storage::DecoratorStorage;
        use crate::resources::walls::public_walls::PublicWalls;
        use crate::systems::collision::world::RaycastWorld;
        use crate::systems::mode_manager::{AppMode, EraserLayer};
        use crate::systems::ui::resources::ui_state::{UiInteraction, UiState, WidgetId};
        use crate::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer;
        use crate::systems::wall::eraser::archivist::WallEraserArchivist;
        use bevy_ecs::prelude::*;
        for (center_x, radius, expected_walls) in [(3.0, 4.0, 0), (0.0, 1.0, 1), (3.0, 1.0, 2)] {
            let mut world = World::new();
            init_eraser_events(&mut world);
            world.insert_resource(AppMode::Eraser(EraserLayer::Wall));
            world.insert_resource(AnimateWallBulldozeBuffer::new());
            world.insert_resource(CursorTerrainRaycast { tag: 1, pos: glam::Vec3::new(center_x, 0.0, 0.0), ..Default::default() });
            world.insert_resource(RaycastWorld::new());
            #[cfg(feature = "native-physics")]
            world.init_resource::<crate::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
            world.insert_resource(WallColorIds::new());
            world.insert_resource(HalftimberColorIds::new());
            world.insert_resource(EditingHighlight::default());
            world.insert_resource(EraserSizeMultiplier(radius / f32::from_bits(0x3ff1eb84)));
            world.insert_resource(TerrainHeightsData::<crate::resources::terrain_height::TerrainHeightSsbo>::new());
            world.insert_resource(PublicWalls::default());
            world.insert_resource(DecoratorStorage::default());
            world.insert_resource(LiveDecoratorArchivist::default());
            world.insert_resource(WallEraserArchivist::new());
            let curve = Curve2::try_new(vec![[0., 0.], [2., 0.], [4., 0.], [6., 0.]], false).unwrap();
            let mut inner = InnerWalls::new();
            inner.new_wall_with_id(WallId(10), WallState::new(curve, 3.0, 0, 0, false));
            world.insert_resource(inner);
            let id = WidgetId::clearing_canvas("eraser");
            world.insert_resource(UiState::new(UiInteraction { native_tag: 2, native_phase: 3, widget: id }));
            let mut history = History::<()>::empty_glade();
            history.start_input_recording(9).unwrap();
            world.insert_resource(history);
            let mut schedule = Schedule::default();
            schedule.add_systems(wall_eraser::<()>);
            schedule.run(&mut world);
            assert_eq!(world.resource::<InnerWalls>().wall_ids().count(), expected_walls);
            world.resource_mut::<UiState>().replace_interaction(UiInteraction { native_tag: 2, native_phase: 1, widget: id });
            schedule.run(&mut world);
            let edits = &world.resource::<History<()>>().pending().unwrap().edits;
            assert!(matches!(edits.as_slice(), [EditType::Wall(_)]));
            assert!(!world.resource::<WallEraserArchivist>().has_pending_child_walls());
            assert_eq!(world.resource::<Events<crate::resources::onboarding::NotifyHintSystemSomethingWasErased>>().len(), 1);
        }
    }

    #[test]
    fn fully_erased_wall_archives_then_removes_and_plays_real_audio() {
        use super::{erase_entire_wall, EraserEventWriters};
        use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
        use crate::resources::roofs::geometry::surface::Curve2;
        use crate::resources::walls::{InnerWalls, WallId, WallState};
        use crate::systems::wall::eraser::archivist::{ErasedRange, WallEraserArchivist};
        use bevy_ecs::prelude::*;
        fn erase(mut inner: ResMut<InnerWalls>, mut eraser: ResMut<WallEraserArchivist>,
            cursor: Res<CursorTerrainRaycast>, mut events: EraserEventWriters) {
            erase_entire_wall(WallId(10), &cursor, &mut inner, &mut eraser, &mut events);
        }
        let mut world = World::new();
        init_eraser_events(&mut world);
        let curve = Curve2::try_new(vec![[0., 0.], [2., 0.]], false).unwrap();
        let state = WallState::new(curve, 3.0, 0, 1, false);
        let mut eraser = WallEraserArchivist::new();
        eraser.cache_if_original_wall(WallId(10), state.snapshot(), ErasedRange { low: 0., high: 1. });
        let mut inner = InnerWalls::new();
        inner.new_wall_with_id(WallId(10), state);
        world.insert_resource(inner);
        world.insert_resource(eraser);
        world.insert_resource(CursorTerrainRaycast::default());
        let mut schedule = Schedule::default();
        schedule.add_systems(erase);
        schedule.run(&mut world);
        assert!(!world.resource::<InnerWalls>().contains(WallId(10)));
        assert!(world.resource::<WallEraserArchivist>().has_pending_child_walls());
        let audio = world.resource::<Events<crate::systems::audio::PlayWallAudio>>();
        let event = audio.iter_current_update_events().next().unwrap();
        assert_eq!((event.kind, event.position, event.volume, event.flag), (6, glam::Vec3::ZERO, 1.0, 2));
    }

    #[test]
    fn single_segment_updates_real_wall_and_material_event() {
        use super::{retain_single_wall_segment, EraserEventWriters};
        use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
        use crate::resources::roofs::geometry::surface::Curve2;
        use crate::resources::walls::{InnerWalls, WallId, WallState};
        use crate::systems::wall::eraser::archivist::{ErasedRange, WallEraserArchivist};
        use bevy_ecs::prelude::*;
        fn apply(mut inner: ResMut<InnerWalls>, mut eraser: ResMut<WallEraserArchivist>,
            cursor: Res<CursorTerrainRaycast>, mut events: EraserEventWriters) {
            let wall = WallId(10);
            let source = inner.get_unwrapped(wall).clone();
            retain_single_wall_segment(wall, &source, ErasedRange { low: 0.25, high: 0.75 },
                false, &cursor, None, &mut inner, &mut eraser, &mut events);
        }
        let mut world = World::new();
        init_eraser_events(&mut world);
        let curve = Curve2::try_new(vec![[0., 0.], [2., 0.]], false).unwrap();
        let initial = WallState::new(curve, 3.0, 2, 0, false);
        let mut snapshot = initial.snapshot().clone();
        snapshot.source.terrain_floor = 2.5;
        snapshot.metadata.source_69 = 9;
        let state = WallState::from_snapshot(snapshot, 2);
        let mut eraser = WallEraserArchivist::new();
        eraser.cache_if_original_wall(WallId(10), state.snapshot(), ErasedRange { low: 0., high: 1. });
        let mut inner = InnerWalls::new();
        inner.new_wall_with_id(WallId(10), state);
        world.insert_resource(inner);
        world.insert_resource(eraser);
        world.insert_resource(CursorTerrainRaycast::default());
        let mut schedule = Schedule::default();
        schedule.add_systems(apply);
        schedule.run(&mut world);
        let inner = world.resource::<InnerWalls>();
        let next = inner.get_unwrapped(WallId(10));
        assert_eq!(next.snapshot().source.curve.length, 1.0);
        assert_eq!(next.height().to_bits(), 0x3ffa2e8b);
        assert_eq!(next.snapshot().source.terrain_floor, 0.0);
        assert_eq!(next.style(), 0);
        assert_eq!(next.freehand_ty(), 2);
        assert_eq!(next.snapshot().metadata.source_69, 5);
        assert_eq!(world.resource::<Events<crate::systems::audio::RequestWallMaterialAudio>>().len(), 1);
    }

    #[test]
    fn finishing_active_eraser_records_history_and_real_hint_events() {
        use super::{finish_eraser_input, EraserEventWriters};
        use crate::resources::color_id::{HalftimberColorIds, WallColorIds};
        use crate::resources::history::{EditType, History};
        use crate::resources::onboarding::{NotifyHintSystemSomethingWasErased, NotifyHintSystemToolWasJustUsed};
        use crate::resources::roofs::geometry::surface::Curve2;
        use crate::resources::walls::{InnerWalls, WallId, WallState};
        use crate::systems::wall::eraser::archivist::{ErasedRange, WallEraserArchivist};
        use bevy_ecs::prelude::*;
        fn finish(
            mut eraser: ResMut<WallEraserArchivist>, inner: Res<InnerWalls>,
            colors: Res<WallColorIds>, timber: Res<HalftimberColorIds>,
            mut history: ResMut<History<()>>, mut events: EraserEventWriters,
        ) {
            finish_eraser_input(&mut eraser, &inner, &colors, &timber, &mut history, &mut events);
        }
        let mut world = World::new();
        init_eraser_events(&mut world);
        let curve = Curve2::try_new(vec![[0., 0.], [2., 0.]], false).unwrap();
        let state = WallState::new(curve, 3.0, 0, 0, false);
        let mut eraser = WallEraserArchivist::new();
        eraser.cache_if_original_wall(WallId(10), state.snapshot(), ErasedRange { low: 0., high: 1. });
        let mut inner = InnerWalls::new();
        inner.new_wall_with_id(WallId(10), state);
        let mut history = History::<()>::empty_glade();
        history.start_input_recording(12).unwrap();
        world.insert_resource(eraser);
        world.insert_resource(inner);
        world.insert_resource(history);
        world.insert_resource(WallColorIds::new());
        world.insert_resource(HalftimberColorIds::new());
        let mut schedule = Schedule::default();
        schedule.add_systems(finish);
        schedule.run(&mut world);
        assert_eq!(world.resource::<Events<NotifyHintSystemSomethingWasErased>>().len(), 1);
        assert_eq!(world.resource::<Events<NotifyHintSystemToolWasJustUsed>>().len(), 1);
        assert!(matches!(world.resource::<History<()>>().pending().unwrap().edits.as_slice(), [EditType::Wall(_)]));
        assert!(!world.resource::<WallEraserArchivist>().has_pending_child_walls());
    }

    #[test]
    fn enclosed_wall_preparation_uses_real_authority_and_preserves_outside_wall() {
        use super::prepare_enclosed_wall;
        use crate::resources::roofs::geometry::surface::Curve2;
        use crate::resources::walls::{InnerWalls, WallId, WallState};
        use crate::resources::walls::publication_fields::WallShapeSnapshot;
        use glam::Vec2;
        let curve = Curve2::try_new(vec![[0., 0.], [2., 0.], [2., 2.], [0., 2.], [0., 0.]], false).unwrap();
        let base = WallState::new(curve, 3.0, 1, 0, false);
        let mut snapshot = base.snapshot().clone();
        snapshot.source.curve_kind_tag = 0;
        snapshot.source.shape = WallShapeSnapshot::from_observed([0; 28]);
        let mut inner = InnerWalls::new();
        inner.new_wall_with_id(WallId(10), WallState::from_snapshot(snapshot, 0));
        inner.take_changes();
        assert!(!prepare_enclosed_wall(&mut inner, WallId(10), Vec2::new(10., 10.), 0.5));
        assert!(inner.changes().updated.is_empty());
        assert!(!prepare_enclosed_wall(&mut inner, WallId(10), Vec2::new(1., -0.5), 0.5));
        assert!(!prepare_enclosed_wall(&mut inner, WallId(10), Vec2::new(1., 0.), f32::NAN));
        assert!(inner.changes().updated.is_empty());
        assert!(prepare_enclosed_wall(&mut inner, WallId(10), Vec2::new(1., 0.), 0.5));
        let next = inner.get_unwrapped(WallId(10));
        assert_eq!(next.snapshot().source.curve_kind_tag, 2);
        assert_eq!(next.snapshot().source.curve.points[0], [1., 0.]);
        assert!(inner.changes().updated.contains(&WallId(10)));
    }

    #[test]
    fn original_eraser_canvas_phase_uses_actual_active_widget() {
        use super::{eraser_ui_phase, EraserUiPhase};
        use crate::systems::mode_manager::{AppMode, EraserLayer};
        use crate::systems::ui::resources::ui_state::{UiInteraction, UiState, WidgetId};
        let id = WidgetId::clearing_canvas("eraser");
        let mut highlight = crate::resources::editing_highlight::EditingHighlight::default();
        for (byte, expected) in [(0, EraserUiPhase::Finish), (1, EraserUiPhase::Finish),
            (2, EraserUiPhase::Inactive), (3, EraserUiPhase::Start), (4, EraserUiPhase::Continue)] {
            let ui = UiState::new(UiInteraction { native_tag: 2, native_phase: byte, widget: id });
            assert_eq!(eraser_ui_phase(&AppMode::Eraser(EraserLayer::Wall), &ui, &mut highlight), expected);
            let canvas = ui.clearing_canvas_params(id).unwrap();
            assert_eq!((canvas.argument_79, canvas.layer), (6, 3));
            assert_eq!(highlight.color, glam::Vec3::new(0.1, 0.03, 0.0));
            assert!(highlight.enabled);
        }
        let ui = UiState::new(UiInteraction { native_tag: 2, native_phase: 3, widget: WidgetId(0) });
        assert_eq!(eraser_ui_phase(&AppMode::Eraser(EraserLayer::All), &ui, &mut highlight), EraserUiPhase::Inactive);
        let ui = UiState::at_startup();
        assert_eq!(eraser_ui_phase(&AppMode::Eraser(EraserLayer::Bulldoze), &ui, &mut highlight), EraserUiPhase::Inactive);
        assert!(ui.clearing_canvas_params(id).is_none());
    }

    #[test]
    fn enclosed_rewire_u_uses_original_lerp_bits() {
        // 原 RNE 的 140c904d0 执行结果，区别于点坐标的加权插值顺序。
        let u = utils::scalar::lerp(f32::from_bits(0x3f1af100),
            f32::from_bits(0x3f29fa8b), f32::from_bits(0x3f71e191));
        assert_eq!(u.to_bits(), 0x3f29263c);
    }

    #[test]
    fn brush_ranges_on_elbow_match_original_segment_loop() {
        use super::outside_brush_ranges;
        use crate::resources::roofs::geometry::surface::Curve2;
        use glam::Vec2;
        use utils::geometry::Circle2d;
        // 原段循环在两处直角的6米曲线上执行；位模式来自 x86_64 原指令。
        let curve = Curve2::try_new(vec![[0., 0.], [2., 0.], [2., 2.], [4., 2.]], false).unwrap();
        for (radius, expected) in [
            (1., vec![[0, 0x3eaaaaab], [0x3f2aaaab, 0x3f800000]]),
            (0.5, vec![[0, 0x3ed55556], [0x3f155556, 0x3f800000]]),
        ] {
            let actual: Vec<[u32; 2]> = outside_brush_ranges(&curve, Circle2d::new(Vec2::new(2., 1.), radius))
                .into_iter().map(|range| range.map(f32::to_bits)).collect();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn short_residual_ranges_match_original_compaction() {
        use super::remove_short_brush_ranges;
        let cases = [
            ([0, 0x3ccccccc, 0x3e800000, 0x3f000000, 0x3f400000, 0x3f800000], vec![[0x3e800000, 0x3f000000], [0x3f400000, 0x3f800000]]),
            ([0, 0x3d088889, 0x3e800000, 0x3f000000, 0x3f400000, 0x3f800000], vec![[0, 0x3d088889], [0x3e800000, 0x3f000000], [0x3f400000, 0x3f800000]]),
            ([0, 0x3e800000, 0x3f000000, 0x3f010000, 0x3f400000, 0x3f800000], vec![[0, 0x3e800000], [0x3f400000, 0x3f800000]]),
            ([0, 0x3e800000, 0x3f000000, 0x3f400000, 0x3f700000, 0x3f710000], vec![[0, 0x3e800000], [0x3f000000, 0x3f400000]]),
            ([0, 0x7fc12345, 0x3e800000, 0x3f000000, 0x3f400000, 0x3f800000], vec![[0x3e800000, 0x3f000000], [0x3f400000, 0x3f800000]]),
        ];
        for (input, expected) in cases {
            let mut ranges: Vec<[f32; 2]> = input.chunks_exact(2)
                .map(|v| [f32::from_bits(v[0]), f32::from_bits(v[1])]).collect();
            remove_short_brush_ranges(&mut ranges, 6.0);
            assert_eq!(ranges.into_iter().map(|v| v.map(f32::to_bits)).collect::<Vec<_>>(), expected);
        }
    }
}

/// 原 140accab0 的地形、公开墙和屋顶查询组成一个 SystemParam 组。
#[derive(SystemParam)]
struct EraserWorld<'w, 's> {
    terrain: Res<'w, crate::resources::terrain_height::TerrainHeightsData>,
    public: Res<'w, PublicWalls>,
    roofs: Query<'w, 's, &'static Roof>,
}

/// 原 140accab0 的系统参数；各字段借用正式资源与事件，不复制业务 owner。
#[derive(SystemParam)]
pub struct WallEraserParams<'w, 's, S: Send + Sync + 'static> {
    mode: Res<'w, crate::systems::mode_manager::AppMode>,
    bulldoze: Res<'w, super::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,
    cursor: Res<'w, crate::resources::cursor_terrain_raycast::CursorTerrainRaycast>,
    archival: crate::resources::walls::decorator_archivist::DecoratorArchivalResources<'w>,
    wall_colors: Res<'w, crate::resources::color_id::WallColorIds>,
    inner: ResMut<'w, InnerWalls>,
    ui: Res<'w, crate::systems::ui::resources::ui_state::UiState>,
    history: ResMut<'w, crate::resources::history::History<S>>,
    highlight: ResMut<'w, crate::resources::editing_highlight::EditingHighlight>,
    multiplier: Res<'w, crate::resources::walls::EraserSizeMultiplier>,
    world: EraserWorld<'w, 's>,
    eraser: ResMut<'w, WallEraserArchivist>,
    storage: ResMut<'w, crate::resources::walls::decorator_storage::DecoratorStorage>,
    decorators: ResMut<'w, crate::resources::walls::decorator_archivist::LiveDecoratorArchivist>,
    timber_colors: Res<'w, crate::resources::color_id::HalftimberColorIds>,
    events: EraserEventWriters<'w>,
}

/// 原墙擦除 UI 主入口：真实画布相位驱动刷圆，墙编辑和事务均写入正式资源。
pub fn wall_eraser<S: Send + Sync + 'static>(params: WallEraserParams<'_, '_, S>) {
    let WallEraserParams {
        mode, bulldoze, cursor, archival, wall_colors, mut inner, ui, mut history,
        mut highlight, multiplier, world, mut eraser, mut storage,
        mut decorators, timber_colors, mut events,
    } = params;
    let EraserWorld { terrain, public, roofs } = world;
    match eraser_ui_phase(&mode, &ui, &mut highlight) {
        EraserUiPhase::Inactive => return,
        EraserUiPhase::Finish => {
            finish_eraser_input(&mut eraser, &inner, &wall_colors, &timber_colors,
                &mut history, &mut events);
            return;
        }
        EraserUiPhase::Start => *eraser = WallEraserArchivist::new(),
        EraserUiPhase::Continue => {}
    }
    let brush = eraser_brush(&cursor, &multiplier);
    // 原版先复制 key 列表，允许后续删除原墙并插入新墙。
    let ids: Vec<_> = inner.wall_ids().collect();
    for wall in ids {
        if bulldoze.contains(wall) { continue; }
        let was_premade = inner.get_unwrapped(wall).is_premade_shape();
        if !prepare_enclosed_wall(&mut inner, wall, brush.center, brush.radius) { continue; }
        let source = inner.get_unwrapped(wall).clone();
        let mut ranges = outside_brush_ranges(&source.snapshot().source.curve, brush);
        if ranges.len() == 1 && ranges[0][0] == 0.0 && ranges[0][1] == 1.0 {
            continue;
        }
        archive_affected_wall(wall, source.snapshot(), &storage, &archival,
            &mut decorators, &mut eraser);
        remove_short_brush_ranges(&mut ranges, source.snapshot().source.curve.length);
        let roof = public.roof_entity(wall).and_then(|entity| roofs.get(entity).ok());
        match ranges.as_slice() {
            [] => erase_entire_wall(wall, &cursor, &mut inner, &mut eraser, &mut events),
            [[low, high]] => retain_single_wall_segment(wall, &source,
                ErasedRange { low: *low, high: *high }, was_premade, &cursor, roof,
                &mut inner, &mut eraser, &mut events),
            _ => split_wall_segments(wall, &source, &ranges, was_premade, roof,
                &public, &terrain, &cursor, &wall_colors, &timber_colors,
                &mut inner, &mut storage, &archival, &mut decorators, &mut eraser, &mut events),
        }
    }
    // 两个源槽均不被原藤蔓消费者读取；Rust 的安全投影不复制栈上的未定义字节。
    if eraser.rewired_wall_ids().next().is_some() {
        let p = cursor.position();
        let width = multiplier.0 * f32::from_bits(0x4071_eb84);
        events.ivy.write(crate::systems::ivy::ivy_pruner::RemoveIvyCmd::at_location(
            glam::Vec3::new(p.x, 0.0, p.z), width, 10.0, 0, 0));
    }
    for wall in eraser.rewired_wall_ids() {
        if inner.contains(wall) {
            events.reanchor.write(crate::resources::events::ReanchorDecorators(wall));
        }
    }
}
