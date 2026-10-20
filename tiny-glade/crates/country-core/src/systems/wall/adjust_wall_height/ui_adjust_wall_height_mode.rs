//! `country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode`
//!
//! 原版 4 个函数：`ui_adjust_wall_height_mode`（0x140ac6d60）、
//! `get_closest_point_u_to_cursor`（0x140ac7400，已在父模块 `adjust_wall_height.rs`）、
//! `history_record_adjust_wall_height`（0x140ac79e0）、`clamp_wall_height`（0x140ac7f30）。
//!
//! ## `ui_adjust_wall_height_mode` 形参（FunctionSystem 类型名串 @0x142982c00，12 个）
//!
//! | # | 原版形参 | 本实现 |
//! |---|---|---|
//! | 1 | `Res<CursorPositionSS>` | 同 |
//! | 2 | `Res<MainCamera>` | 同 |
//! | 3 | `Res<WindowSize>` | 同 |
//! | 4 | `Res<PublicWalls>` | 同 |
//! | 5 | `DecoratorArchivalResources`（SystemParam 结构） | 同（`decorator_archivist`） |
//! | 6 | `ResMut<AdjWallHeightModeCache>` | 同 |
//! | 7 | `EventReader<AudioCursorMoved>` | 同 |
//! | 8 | `EventReader<AdjustWallHeightUi>` | 同 |
//! | 9 | `EventWriter<AdjustWallHeightCmd>` | 同 |
//! | 10 | `Query<(&TestCube, &mut Transform)>` | 同（函数体不读写） |
//! | 11 | `Res<DecoratorStorage>` | 同 |
//! | 12 | `ResMut<DecoratorArchivist>` | `ResMut<LiveDecoratorArchivist>` |
//!
//! ## `history_record_adjust_wall_height` 形参（类型名串 @0x142985500，7 个）
//!
//! `ResMut<AdjWallHeightModeCache>`, `EventReader<FinishAdjWallHeightMode>`,
//! `Query<(&TestCube, &mut Transform)>`, `ResMut<History>`, `Res<PublicWalls>`,
//! `Query<&Roof>`, `EventWriter<NotifyHintSystemShapeEditWasPerformed>`。
//! `History` 在本仓库是 `History<S>`，所以系统对 `S` 泛型。

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};
use std::sync::Arc;

use crate::components::TestCube;
use crate::resources::events::FinishAdjWallHeightMode;
use crate::resources::history::wall_edit::{WallCompoundHistoryEdit, WallHistoryEdit};
use crate::resources::history::{EditType, History};
use crate::resources::mouse_terrain_raycast::CursorPositionSS;
use crate::resources::onboarding::NotifyHintSystemShapeEditWasPerformed;
use crate::resources::roofs::Roof;
use crate::resources::walls::decorator_archivist::{DecoratorArchivalResources, LiveDecoratorArchivist};
use crate::resources::walls::decorator_storage::{
    Decorators, DecoratorStorage, DecoratorStorageAddr,
};
use crate::resources::walls::height_rules::clamp_resolved_wall_height;
use crate::resources::walls::public_walls::PublicWalls;
use crate::resources::walls::publication_fields::WallId as RecoveredWallId;
use crate::resources::walls::WallId;
use crate::resources::window::{AudioCursorMoved, WindowSize};
use crate::systems::camera_rig::main_camera::MainCamera;
use crate::systems::wall::adjust_wall_height::{
    AdjustWallHeightCmd, CAMERA_PROJECTION_OFFSET, CAMERA_VIEW_OFFSET,
    get_closest_point_u_to_cursor,
};
use crate::systems::wall::height_command::AdjustWallHeightRequest;
use crate::utils::components::transform::Transform;
use crate::utils::project_cursor_onto_ray::{ProportionalScreenSpace, ProportionalScreenSpaceCoord};
use utils::geometry::Ray;

/// B `ui_adjust_wall_height_mode::AdjustWallHeightUi`：UI 控件每帧请求编辑某面墙的高度。
/// 事件实例 0x10 B = 8 B EventId + 8 B 载荷；载荷 qword 被当作 WallId 读取
/// （0x140ac7041 `[R15+8]`，随后做 PublicWalls 探针）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdjustWallHeightUi(pub WallId);

/// B `ui_adjust_wall_height_mode::AdjWallHeightModeCache`（资源，0x30 B，偏移取自
/// 0x140ac6d60 / 0x140ac79e0 的读写面；字段名为描述性，偏移证据见各字段）。
///
/// 默认值全 `None`：`history_record_adjust_wall_height` 收尾时 `+0x00/+0x10/+0x18/
/// +0x20/+0x28` 五个标签全部写 0。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct AdjWallHeightModeCache {
    /// +0x00 qword 标签 / +0x08 WallId：编辑会话锁定的墙（init 分支写入）。
    pub wall: Option<WallId>,
    /// +0x10 / +0x14：光标命中的曲线参数 u（`cache.u` 为 `None` 才走 init 分支）。
    pub u: Option<f32>,
    /// +0x18 / +0x1c：光标投影 y 与墙高之间的抓取偏移（首个更新帧写入，之后读取）。
    pub grab_offset: Option<f32>,
    /// +0x20 / +0x24：会话起始高度（init 写 `constrained_extra`，历史记录的 `from`）。
    pub original_height: Option<f32>,
    /// +0x28 / +0x2c：当前高度（init 写 `constrained_extra`，每个更新帧写新高度，历史记录的 `to`）。
    pub current_height: Option<f32>,
}

/// B `utils::mouse_ss_to_cs`（0x140c906d0）：`(x, y, w, h) → (2·(x/w) − 1, 2·((1 − y)/h) + 1)`。
/// 机器码逐条：`x/w; +x/w; + (−1.0)` 与 `(1.0 − y)/h; 加倍; + 1.0`；本函数保持同一运算次序。
pub fn mouse_ss_to_cs(x: f32, y: f32, width: f32, height: f32) -> Vec2 {
    let nx = x / width;
    let cx = nx + nx + -1.0;
    let ny = (1.0 - y) / height;
    let cy = ny + ny + 1.0;
    Vec2::new(cx, cy)
}

/// B `Curve::get_coord_at_u`（0x141467ce0）在 `points_u` 切片上的同式实现：
/// `u >= 1 → (len − 2, 1)`，`u <= 0 → (0, 0)`，否则右偏二分 + 夹紧的 `t`。
fn coord_at_u(points_u: &[f32], u: f32) -> (usize, f32) {
    assert!(u.is_finite());
    if u >= 1.0 {
        return (points_u.len() - 2, 1.0);
    }
    if u <= 0.0 {
        return (0, 0.0);
    }
    let next = points_u.partition_point(|&value| value <= u);
    assert!(next > 0 && next < points_u.len());
    let segment = next - 1;
    let (a, b) = (points_u[segment], points_u[next]);
    assert!(a != b);
    let t = (u - a) / (b - a);
    (segment, if t < 0.0 { 0.0 } else if t > 1.0 { 1.0 } else { t })
}

/// B `clamp_wall_height`（0x140ac7f30）：`PublicWalls` 中无该墙（含整表为空）→ `None`
///（调用方保持原值）；命中则先做 `count > 1 && length > 0` 断言（`PublicWalls::get`），
/// 再按屋顶高度/特殊标志夹紧。
pub fn clamp_wall_height(
    value: f32,
    wall: WallId,
    public: &PublicWalls,
    roofs: &Query<&Roof>,
) -> Option<f32> {
    let state = public.get(wall)?;
    let roof_height = public
        .roof_component(wall, |entity| roofs.get(entity).ok())
        .map(Roof::height_01);
    Some(clamp_resolved_wall_height(
        value,
        roof_height,
        state.authority.source_flag_65(),
    ))
}

/// B `DecoratorArchivist::archive_current_decorators_in_storages`（0x1413e77f0）以
/// `[Wall(wall), Terrain]` 调用的归档语义：存储中有桶则在归档里尚无时克隆入；存储缺桶
/// 则以空 `Decorators` 走 `archive_storage_decorators`（原版 `LAB_1413e7e60` 分支）。
/// 碰撞体载荷重建（`DecoratorArchivalResources`）属 collision 域，沿用本仓库
/// `archive_storage_decorators` 的登记缺口。
fn archive_wall_and_terrain(
    archivist: &mut LiveDecoratorArchivist,
    storage: &DecoratorStorage,
    wall: WallId,
) {
    let empty = Decorators::default();
    let wall_source = storage.get(DecoratorStorageAddr::Wall(wall)).unwrap_or(&empty);
    archivist.archive_storage_decorators(0, wall_source, Some(wall));
    let terrain_source = storage.get(DecoratorStorageAddr::Terrain).unwrap_or(&empty);
    archivist.archive_storage_decorators(1, terrain_source, None);
}

/// 0x140ac7180 的 `Map::fold`：存储 `+0x90` 的地形桶（若存在）里的装饰 id 全部放行。
fn cull_allow_terrain(archivist: &mut LiveDecoratorArchivist, storage: &DecoratorStorage) {
    if let Some(decorators) = storage.get(DecoratorStorageAddr::Terrain) {
        for id in decorators.decorators().keys() {
            archivist.culling.allowed_ids.insert(*id);
        }
    }
}

/// B `ui_adjust_wall_height_mode`（0x140ac6d60）。
///
/// 分支：
/// 1. `AdjustWallHeightUi` 队列为空 → 返回（队列整体消费）。
/// 2. `AudioCursorMoved` 队列为空 → 返回（同样整体消费）。
/// 3. 取 UI 队列**最后一个**事件的墙；`PublicWalls` 缺该墙 → `Option::unwrap` panic，
///    命中则做曲线有效性断言。
/// 4. `cache.u == None`（init 帧）：`get_closest_point_u_to_cursor` 求 u；写
///    `cache.{u, original_height, current_height} = constrained_extra`、`cache.wall = wall`；
///    归档墙与地形装饰、放行该墙与地形装饰 id 的剔除。
/// 5. 公共尾部：由 `cache.u` 求曲线上的 3D 点，以 `(点, +Y)` 为射线把光标投影到射线上，
///    取投影 y；`cache.grab_offset == None` → 写 `p.y + constrained_extra − y`（不发命令）；
///    否则发 `AdjustWallHeightCmd { anchor: Some(p), mode: 1, value: y + grab − p.y, wall,
///    flag 0 }` 并把 `cache.current_height` 写为同一值。
#[allow(clippy::too_many_arguments)]
pub fn ui_adjust_wall_height_mode(
    cursor: Res<CursorPositionSS>,
    camera: Res<MainCamera>,
    window: Res<WindowSize>,
    public: Res<PublicWalls>,
    _archival: DecoratorArchivalResources,
    mut cache: ResMut<AdjWallHeightModeCache>,
    mut cursor_moved: EventReader<AudioCursorMoved>,
    mut ui_requests: EventReader<AdjustWallHeightUi>,
    mut commands: EventWriter<AdjustWallHeightCmd>,
    _test_cubes: Query<(&TestCube, &mut Transform)>,
    storage: Res<DecoratorStorage>,
    mut archivist: ResMut<LiveDecoratorArchivist>,
) {
    let Some(request) = ui_requests.read().last().copied() else {
        return;
    };
    if cursor_moved.read().count() == 0 {
        return;
    }
    let wall = request.0;
    let state = public.get(wall).unwrap();

    let (width, height) = (window.width as f32, window.height as f32);
    let aspect = width / height;
    let screen = ProportionalScreenSpace {
        size: Vec2::new(width, height),
        aspect,
        unit: 1.0,
    };
    let cs = mouse_ss_to_cs(cursor.0.x, cursor.0.y, width, height);
    let (canvas_x, canvas_y) = (cs.x * aspect, cs.y);
    let constrained_extra = state.geometry.metrics.constrained_extra;
    let outline: Vec<Vec3> = state
        .geometry
        .outline
        .iter()
        .map(|point| Vec3::from_array(*point))
        .collect();

    if cache.u.is_none() {
        // 原生 0x140ac705f：光标对以 `(y, x·aspect)` 序传入。
        let hit = get_closest_point_u_to_cursor(
            &camera,
            &outline,
            &state.geometry.outline_u,
            Vec2::new(canvas_y, canvas_x),
            constrained_extra,
        );
        cache.u = Some(hit.u);
        cache.original_height = Some(constrained_extra);
        cache.current_height = Some(constrained_extra);
        cache.wall = Some(wall);
        archive_wall_and_terrain(&mut archivist, &storage, wall);
        archivist.decorator_cull_allow_wall(wall, &storage);
        cull_allow_terrain(&mut archivist, &storage);
    }

    let u = cache.u.expect("AdjWallHeightModeCache::u is set by the init branch");
    let (segment, t) = coord_at_u(&state.geometry.outline_u, u);
    // 原生两次边界检查：`len <= i` 与 `i + 1 >= len` 均 panic_bounds_check。
    let p0 = outline[segment];
    let p1 = outline[segment + 1];
    let point = p1 * t + p0 * (1.0 - t);

    let projected = ProportionalScreenSpaceCoord(Vec2::new(canvas_x, canvas_y)).project_onto_ray(
        &screen,
        &camera.read_matrix(CAMERA_VIEW_OFFSET),
        &camera.read_matrix(CAMERA_PROJECTION_OFFSET),
        Ray {
            origin: point,
            direction: Vec3::Y,
        },
        [0.0, f32::MAX],
    );

    match cache.grab_offset {
        Some(grab) => {
            let new_height = projected.y + grab - point.y;
            commands.write(AdjustWallHeightCmd(AdjustWallHeightRequest {
                anchor: Some(point.to_array()),
                mode: 1,
                value: new_height,
                wall: RecoveredWallId(wall.0),
                observed_audio_flag: 0,
            }));
            cache.current_height = Some(new_height);
        }
        None => {
            cache.grab_offset = Some(point.y + constrained_extra - projected.y);
        }
    }
}

/// `EditType::new_height_changed(wall, from, to)`（0x140a83fb0）：单组单编辑的墙复合记录。
fn new_height_changed(wall: WallId, from: f32, to: f32) -> EditType {
    EditType::Wall(Arc::new(WallCompoundHistoryEdit {
        groups: vec![vec![WallHistoryEdit::AdjustHeight {
            wall: RecoveredWallId(wall.0),
            from,
            to,
        }]],
    }))
}

/// B `history_record_adjust_wall_height`（0x140ac79e0）。
///
/// 1. `FinishAdjWallHeightMode` 队列为空 → 返回（整体消费，不动缓存）。
/// 2. 缓存 `{wall, original_height, current_height}` 任一为 `None` → 只做第 5 步。
/// 3. `from/to` 各自经 [`clamp_wall_height`]（`PublicWalls` 缺墙则不夹紧）；有序相等
///    （`UCOMISS` 的 ZF&&!PF）→ 只做第 5 步，NaN 视为不等。
/// 4. `History::add(EditType::new_height_changed(wall, from, to))`；随后再次探针
///    `PublicWalls`：缺墙 → 跳过；命中做曲线断言，仅 `shape.tag <= 1`（预制形状）才写
///    `NotifyHintSystemShapeEditWasPerformed`。
/// 5. 缓存整体复位为 `None`。
///
/// `History::add`（0x140a84ec0）失败原版走 `panic_fmt`；这里把 `Err` 同样 panic。
pub fn history_record_adjust_wall_height<S: Send + Sync + 'static>(
    mut cache: ResMut<AdjWallHeightModeCache>,
    mut finish: EventReader<FinishAdjWallHeightMode>,
    _test_cubes: Query<(&TestCube, &mut Transform)>,
    mut history: ResMut<History<S>>,
    public: Res<PublicWalls>,
    roofs: Query<&Roof>,
    mut hint: EventWriter<NotifyHintSystemShapeEditWasPerformed>,
) {
    if finish.read().count() == 0 {
        return;
    }
    if let (Some(wall), Some(original), Some(current)) =
        (cache.wall, cache.original_height, cache.current_height)
    {
        let from = clamp_wall_height(original, wall, &public, &roofs).unwrap_or(original);
        let to = clamp_wall_height(current, wall, &public, &roofs).unwrap_or(current);
        if !(from == to) {
            if let Err(error) = history.add(new_height_changed(wall, from, to)) {
                panic!("History::add failed for wall height edit: {error:?}");
            }
            if let Some(state) = public.get(wall) {
                if state.is_premade_shape() {
                    hint.write(NotifyHintSystemShapeEditWasPerformed);
                }
            }
        }
    }
    *cache = AdjWallHeightModeCache::default();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::history::transactions::HistoryTransactions;
    use crate::resources::roofs::geometry::surface::Curve2;
    use crate::resources::walls::decorator::{DecoratorId, DecoratorInfo};
    use crate::resources::walls::inner_walls::InnerWalls;
    use crate::resources::walls::publication_fields::WallShapeSnapshot;
    use crate::resources::walls::publication_source::TerrainHeightGrid512;
    use crate::resources::walls::WallState;
    use bevy_ecs::system::RunSystemOnce;

    type Hist = HistoryTransactions<EditType, ()>;

    fn flat_terrain() -> TerrainHeightGrid512 {
        TerrainHeightGrid512 {
            encoded: vec![0u16; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: true,
        }
    }

    fn wall_state(premade: bool) -> WallState {
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let mut state = WallState::new(curve, 2.0, 0, 0, false);
        if premade {
            state.snapshot.source.shape = WallShapeSnapshot::from_observed([0; 28]);
        }
        state
    }

    fn base_world(premade: bool) -> (World, WallId) {
        let mut world = World::new();
        let mut inner = InnerWalls::new();
        let wall = inner.new_wall_from_curve(wall_state(premade));
        let mut public = PublicWalls::default();
        public.consume(&mut inner, &flat_terrain());
        world.insert_resource(inner);
        world.insert_resource(public);
        world.init_resource::<AdjWallHeightModeCache>();
        world.init_resource::<Events<AdjustWallHeightUi>>();
        world.init_resource::<Events<AudioCursorMoved>>();
        world.init_resource::<Events<AdjustWallHeightCmd>>();
        world.init_resource::<Events<FinishAdjWallHeightMode>>();
        world.init_resource::<Events<NotifyHintSystemShapeEditWasPerformed>>();
        (world, wall)
    }

    fn ui_world() -> (World, WallId) {
        let (mut world, wall) = base_world(false);
        let mut camera = MainCamera::default();
        camera.write_matrix(CAMERA_PROJECTION_OFFSET, &glam::Mat4::IDENTITY);
        camera.write_matrix(CAMERA_VIEW_OFFSET, &glam::Mat4::IDENTITY);
        world.insert_resource(camera);
        world.insert_resource(WindowSize::new(100, 100));
        world.insert_resource(CursorPositionSS(Vec2::new(150.0, 51.0)));
        world.init_resource::<crate::systems::collision::world::RaycastWorld>();
        #[cfg(feature = "native-physics")]
        world.init_resource::<crate::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
        world.init_resource::<LiveDecoratorArchivist>();
        let mut storage = DecoratorStorage::default();
        storage
            .get_or_insert_default(DecoratorStorageAddr::Wall(wall))
            .insert(
                DecoratorId(11),
                DecoratorInfo {
                    dst_wall: wall.0,
                    decorator_id: 11,
                    ..DecoratorInfo::default()
                },
            );
        storage
            .get_or_insert_default(DecoratorStorageAddr::Terrain)
            .insert(
                DecoratorId(77),
                DecoratorInfo {
                    decorator_id: 77,
                    ..DecoratorInfo::default()
                },
            );
        world.insert_resource(storage);
        (world, wall)
    }

    fn send_ui(world: &mut World, wall: WallId) {
        world.resource_mut::<Events<AdjustWallHeightUi>>().send(AdjustWallHeightUi(wall));
        world.resource_mut::<Events<AudioCursorMoved>>().send(AudioCursorMoved);
    }

    fn cmds(world: &World) -> Vec<AdjustWallHeightCmd> {
        let events = world.resource::<Events<AdjustWallHeightCmd>>();
        events.get_cursor().read(events).copied().collect()
    }

    fn extra(world: &World, wall: WallId) -> f32 {
        world
            .resource::<PublicWalls>()
            .get(wall)
            .unwrap()
            .geometry
            .metrics
            .constrained_extra
    }

    #[test]
    fn mouse_ss_to_cs_keeps_the_machine_code_formula() {
        let cs = mouse_ss_to_cs(150.0, 51.0, 100.0, 100.0);
        assert!((cs.x - 2.0).abs() < 1e-6);
        assert!((cs.y - 0.0).abs() < 1e-6);
        let cs = mouse_ss_to_cs(0.0, 0.0, 200.0, 100.0);
        assert_eq!(cs.x, -1.0);
        assert!((cs.y - 1.02).abs() < 1e-6);
    }

    #[test]
    fn coord_at_u_matches_the_curve_rules() {
        let u = [0.0, 0.25, 1.0];
        assert_eq!(coord_at_u(&u, 0.0), (0, 0.0));
        assert_eq!(coord_at_u(&u, 1.0), (1, 1.0));
        let (segment, t) = coord_at_u(&u, 0.625);
        assert_eq!(segment, 1);
        assert!((t - 0.5).abs() < 1e-6);
    }

    #[test]
    fn empty_ui_queue_does_nothing() {
        let (mut world, _) = ui_world();
        world.resource_mut::<Events<AudioCursorMoved>>().send(AudioCursorMoved);
        world.run_system_once(ui_adjust_wall_height_mode).unwrap();
        assert_eq!(*world.resource::<AdjWallHeightModeCache>(), AdjWallHeightModeCache::default());
        assert!(cmds(&world).is_empty());
    }

    #[test]
    fn missing_cursor_moved_returns_but_consumes_the_ui_queue() {
        let (mut world, wall) = ui_world();
        // 读者游标要跨帧保留，必须用同一个 Schedule（run_system_once 每次新建游标）。
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_adjust_wall_height_mode);
        world.resource_mut::<Events<AdjustWallHeightUi>>().send(AdjustWallHeightUi(wall));
        schedule.run(&mut world);
        assert_eq!(*world.resource::<AdjWallHeightModeCache>(), AdjWallHeightModeCache::default());
        // UI 事件已被整体消费：补一个光标事件也没有待处理请求。
        world.resource_mut::<Events<AudioCursorMoved>>().send(AudioCursorMoved);
        schedule.run(&mut world);
        assert_eq!(*world.resource::<AdjWallHeightModeCache>(), AdjWallHeightModeCache::default());
    }

    #[test]
    fn init_frame_locks_the_session_archives_and_sets_grab_offset_without_command() {
        let (mut world, wall) = ui_world();
        // cs.y = 5 → y 像素 −199：投影射线 y = 5。
        world.insert_resource(CursorPositionSS(Vec2::new(150.0, -199.0)));
        send_ui(&mut world, wall);
        world.run_system_once(ui_adjust_wall_height_mode).unwrap();
        let extra = extra(&world, wall);
        let cache = *world.resource::<AdjWallHeightModeCache>();
        assert_eq!(cache.wall, Some(wall));
        assert!((cache.u.unwrap() - 0.5).abs() < 1e-4, "{cache:?}");
        assert_eq!(cache.original_height, Some(extra));
        assert_eq!(cache.current_height, Some(extra));
        assert!((cache.grab_offset.unwrap() - (extra - 5.0)).abs() < 1e-4, "{cache:?}");
        assert!(cmds(&world).is_empty());

        let archivist = world.resource::<LiveDecoratorArchivist>();
        assert!(archivist.walls.contains_key(&RecoveredWallId(wall.0)));
        assert!(archivist.special1.is_some());
        assert!(archivist.culling.allowed_ids.contains(&11));
        assert!(archivist.culling.allowed_ids.contains(&77));
    }

    #[test]
    fn update_frame_emits_the_height_command_and_tracks_the_current_height() {
        let (mut world, wall) = ui_world();
        world.insert_resource(CursorPositionSS(Vec2::new(150.0, -199.0)));
        send_ui(&mut world, wall);
        world.run_system_once(ui_adjust_wall_height_mode).unwrap();
        // cs.y = 7 → y 像素 −299：投影射线 y = 7，新高度 = 7 + (extra − 5) − 0。
        world.insert_resource(CursorPositionSS(Vec2::new(150.0, -299.0)));
        send_ui(&mut world, wall);
        world.run_system_once(ui_adjust_wall_height_mode).unwrap();
        let extra = extra(&world, wall);
        let sent = cmds(&world);
        assert_eq!(sent.len(), 1);
        let request = sent[0].0;
        assert_eq!(request.mode, 1);
        assert_eq!(request.wall, RecoveredWallId(wall.0));
        assert_eq!(request.observed_audio_flag, 0);
        assert!((request.value - (extra + 2.0)).abs() < 1e-4, "{request:?}");
        let anchor = request.anchor.unwrap();
        assert!((anchor[0] - 2.0).abs() < 1e-4 && anchor[1].abs() < 1e-4 && anchor[2].abs() < 1e-4);
        let cache = *world.resource::<AdjWallHeightModeCache>();
        assert!((cache.current_height.unwrap() - (extra + 2.0)).abs() < 1e-4);
        assert_eq!(cache.original_height, Some(extra));
    }

    #[test]
    fn only_the_last_ui_request_selects_the_wall() {
        let (mut world, wall) = ui_world();
        world.insert_resource(CursorPositionSS(Vec2::new(150.0, -199.0)));
        world
            .resource_mut::<Events<AdjustWallHeightUi>>()
            .send(AdjustWallHeightUi(WallId(9999)));
        send_ui(&mut world, wall);
        world.run_system_once(ui_adjust_wall_height_mode).unwrap();
        assert_eq!(world.resource::<AdjWallHeightModeCache>().wall, Some(wall));
    }

    #[test]
    #[should_panic]
    fn unknown_wall_panics_like_the_original_unwrap() {
        let (mut world, _) = ui_world();
        send_ui(&mut world, WallId(4242));
        let _ = world.run_system_once(ui_adjust_wall_height_mode);
    }

    fn history_world(premade: bool) -> (World, WallId) {
        let (mut world, wall) = base_world(premade);
        let mut history = Hist::after_observed_future_clear();
        history.start_input_recording(0).unwrap();
        world.insert_resource(history);
        (world, wall)
    }

    fn set_cache(world: &mut World, wall: WallId, from: f32, to: f32) {
        *world.resource_mut::<AdjWallHeightModeCache>() = AdjWallHeightModeCache {
            wall: Some(wall),
            u: Some(0.5),
            grab_offset: Some(0.0),
            original_height: Some(from),
            current_height: Some(to),
        };
    }

    fn finish(world: &mut World) {
        world.resource_mut::<Events<FinishAdjWallHeightMode>>().send(FinishAdjWallHeightMode);
    }

    fn pending_edits(world: &World) -> Vec<EditType> {
        world.resource::<Hist>().pending().unwrap().edits.clone()
    }

    fn hints(world: &World) -> usize {
        let events = world.resource::<Events<NotifyHintSystemShapeEditWasPerformed>>();
        events.get_cursor().read(events).count()
    }

    #[test]
    fn no_finish_event_leaves_the_cache_alone() {
        let (mut world, wall) = history_world(true);
        set_cache(&mut world, wall, 2.0, 3.0);
        world.run_system_once(history_record_adjust_wall_height::<()>).unwrap();
        assert!(world.resource::<AdjWallHeightModeCache>().wall.is_some());
        assert!(pending_edits(&world).is_empty());
    }

    #[test]
    fn finish_records_the_clamped_height_change_and_resets_the_cache() {
        let (mut world, wall) = history_world(true);
        // 无屋顶、非 special：夹紧区间 [0.5, 14.0]。
        set_cache(&mut world, wall, 0.1, 20.0);
        finish(&mut world);
        world.run_system_once(history_record_adjust_wall_height::<()>).unwrap();
        let edits = pending_edits(&world);
        assert_eq!(edits.len(), 1);
        let EditType::Wall(compound) = &edits[0] else {
            panic!("expected a wall edit");
        };
        assert_eq!(compound.groups.len(), 1);
        match &compound.groups[0][0] {
            WallHistoryEdit::AdjustHeight { wall: w, from, to } => {
                assert_eq!(*w, RecoveredWallId(wall.0));
                assert_eq!(*from, 0.5);
                assert_eq!(*to, 14.0);
            }
            other => panic!("unexpected edit {other:?}"),
        }
        assert_eq!(hints(&world), 1);
        assert_eq!(*world.resource::<AdjWallHeightModeCache>(), AdjWallHeightModeCache::default());
    }

    #[test]
    fn freehand_wall_records_history_without_the_shape_hint() {
        let (mut world, wall) = history_world(false);
        set_cache(&mut world, wall, 2.0, 3.0);
        finish(&mut world);
        world.run_system_once(history_record_adjust_wall_height::<()>).unwrap();
        assert_eq!(pending_edits(&world).len(), 1);
        assert_eq!(hints(&world), 0);
    }

    #[test]
    fn equal_heights_after_clamping_record_nothing_but_still_reset() {
        let (mut world, wall) = history_world(true);
        // 两端夹紧后同为 0.5。
        set_cache(&mut world, wall, 0.1, 0.2);
        finish(&mut world);
        world.run_system_once(history_record_adjust_wall_height::<()>).unwrap();
        assert!(pending_edits(&world).is_empty());
        assert_eq!(hints(&world), 0);
        assert_eq!(*world.resource::<AdjWallHeightModeCache>(), AdjWallHeightModeCache::default());
    }

    #[test]
    fn incomplete_cache_only_resets() {
        let (mut world, wall) = history_world(true);
        *world.resource_mut::<AdjWallHeightModeCache>() = AdjWallHeightModeCache {
            wall: Some(wall),
            u: Some(0.5),
            original_height: Some(2.0),
            ..Default::default()
        };
        finish(&mut world);
        world.run_system_once(history_record_adjust_wall_height::<()>).unwrap();
        assert!(pending_edits(&world).is_empty());
        assert_eq!(*world.resource::<AdjWallHeightModeCache>(), AdjWallHeightModeCache::default());
    }

    #[test]
    fn wall_missing_from_public_walls_records_unclamped_heights_without_hint() {
        let (mut world, _) = history_world(true);
        let ghost = WallId(31337);
        set_cache(&mut world, ghost, 0.1, 20.0);
        finish(&mut world);
        world.run_system_once(history_record_adjust_wall_height::<()>).unwrap();
        let edits = pending_edits(&world);
        let EditType::Wall(compound) = &edits[0] else {
            panic!("expected a wall edit");
        };
        match &compound.groups[0][0] {
            WallHistoryEdit::AdjustHeight { from, to, .. } => {
                assert_eq!((*from, *to), (0.1, 20.0));
            }
            other => panic!("unexpected edit {other:?}"),
        }
        assert_eq!(hints(&world), 0);
    }

    #[test]
    fn clamp_wall_height_is_none_for_an_unknown_wall() {
        let (mut world, wall) = base_world(false);
        let result = world
            .run_system_once(move |public: Res<PublicWalls>, roofs: Query<&Roof>| {
                (
                    clamp_wall_height(1.0, WallId(999), &public, &roofs),
                    clamp_wall_height(100.0, wall, &public, &roofs),
                )
            })
            .unwrap();
        assert_eq!(result.0, None);
        assert_eq!(result.1, Some(14.0));
    }

    #[test]
    fn cache_default_is_all_none_like_the_original_reset() {
        let cache = AdjWallHeightModeCache::default();
        assert!(cache.wall.is_none() && cache.u.is_none() && cache.grab_offset.is_none());
        assert!(cache.original_height.is_none() && cache.current_height.is_none());
    }
}
