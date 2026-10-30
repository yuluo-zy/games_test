//! Original Tiny Glade host wall/session lifecycle. The complete unload body
//! @1402b1cb0 and Reload→Game PostUpdate consumer are recovered below.
//! The gameplay host calls WallSystems::add_wall_systems for the recovered
//! wall chain. Individual omitted original consumers are listed below.

/// `<bevy_app::App as tiny_glade::app_systems::WallSystems>`: `add_wall_systems` (0x140493120) and
/// `add_wall_events` (0x1404936b0, 39KB 事件/资源注册表)。
pub trait WallSystems {
    fn add_wall_systems(&mut self) -> &mut Self;
    fn add_wall_events(&mut self) -> &mut Self;
}

impl WallSystems for bevy_app::App {
    /// `add_wall_systems` 0x140493120：12 次 add_systems + 1 次 insert_resource，调度标签/set 号/成员
    /// 顺序取证见 reports/main-loop-render-recovery-2026-10-19/PLUGIN-WIRING-2026-10-20.md 附录 A。
    ///
    /// 支撑三个入口分别位于 set24 与 set36；set28 的同一墙构造父链同步生成半木，
    /// set32 消费抹灰、曲面半木与木塔结果。注册完成不替代原行为与联合运行验收。
    ///
    /// **宿主适配（原版链中没有，保持其原有相对位置）**：`synchronize_wall_publication_terrain`、
    /// `collect_wall_publication_dirty`、`prepare_wall_brick_assembly_owners`、
    /// `delete_constructor_output`、`finalize_wall_holes`。
    fn add_wall_systems(&mut self) -> &mut Self {
        use bevy_app::{Startup, Update};
        use bevy_ecs::schedule::IntoScheduleConfigs;
        use country_core::{
            resources::{
                history::MaybePendingSnapshotId, walls::decorator_archivist::LiveColliderPayload,
            },
            systems::{GameSystemSet, wall},
        };
        use system_wall_constructor as wc;

        // 1. 0x14049315b
        self.add_systems(
            Update,
            wc::ui_arch_controls::ui_arch_controls.in_set(GameSystemSet(17)),
        );
        // 2. 0x140493177
        self.add_systems(
            Update,
            system_decorator::restore_archived_decorators::restore_archived_decorators::<
                LiveColliderPayload,
            >
                .in_set(GameSystemSet(42)),
        );
        // 3. 0x1404932c7：外层 chain_ignore_deferred(chain1, chain2)，set 19。
        let chain1 = (
            wall::wall_splitter::wall_splitter::<MaybePendingSnapshotId>,
            wall::eraser::ui_bulldozer::wall_eraser_bulldoze
                .run_if(wall::eraser::ui_bulldozer::wall_eraser_bulldoze_condition),
            wall::eraser::wall_eraser::wall_eraser::<MaybePendingSnapshotId>,
            wall::eraser::animate_wall_bulldoze::animate_wall_bulldoze,
            wall::ui_draw_wall::ui_draw_wall,
            wall::ui_continue_wall::suppress_continue_wall,
            wall::ui_continue_wall::ui_continue_wall,
            wall::snapping::snapping_init::snapping_init,
            wall::snapping::snapping_to_ends::snapping_to_ends,
            wall::snapping::snapping_to_segments::snapping_to_segments,
            wall::snapping::snapping_maintain::check_for_disable_snapping,
            wall::snapping::snapping_audio::snapping_audio,
            wall::snapping::snapping_maintain::snapping_signifiers,
            wall::snapping::snapping_maintain::snapping_maintain,
            wall::draw_wall::draw_wall,
            wall::finish_wall::finish_wall::<MaybePendingSnapshotId>,
            wall::merge_walls::merge_walls,
            wall::snapping::telegraph_walls_are_merged::telegraph_walls_are_merged,
        )
            .chain_ignore_deferred();
        let chain2 = (
            wall::adjust_wall_height::signifier_animation::ui_adjust_wall_height_signifier_animation,
            wall::adjust_wall_height::ui_adjust_wall_height_mode::ui_adjust_wall_height_mode,
            // 宿主适配：原版没有，放在它们原有位置（adjust 之前/publish 之前）。
            wall::wall_state::publish_public_wall_state::synchronize_wall_publication_terrain,
            wall::adjust_wall_height::adjust_wall_height_impl::adjust_wall_height_impl,
            wall::wall_state::publish_public_wall_state::collect_wall_publication_dirty,
            wc::ui_arch_controls::do_ui_arch_controls,
            wc::ui_lintel_controls::ui_lintel_controls,
            wall::wall_highlight::wall_highlight,
            (wall::wall_state::publish_public_wall_state::publish_public_wall_state,
                wc::gpu_output::retire_republished_half_timber).chain_ignore_deferred(),
            wall::delete_wall::delete_wall,
            // 宿主适配（原版没有）。
            country_core::geometry::runtime::prepare_wall_brick_assembly_owners,
            wc::gpu_output::delete_constructor_output,
            wall::wall_state::wall_spatial_hash::spatial_hash_update,
            wall::wall_state::spatial_hash_debug::spatial_hash_debug,
            wall::calc_wall_bend::calc_wall_bend,
            wc::debug_force_update_arches,
            wc::gpu_clear_wall_gate_pieces,
            wc::segmentation::sample_wall_path_intersections::sample_wall_path_intersections,
            wc::segmentation::calculate_wall_path_segmentation::calculate_wall_path_segmentation,
            wc::init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height,
        )
            .chain_ignore_deferred();
        self.add_systems(
            Update,
            (chain1, chain2)
                .chain_ignore_deferred()
                .in_set(GameSystemSet(19)),
        );
        // 0x1404932eb + 0x1404932f6（Startup）
        self.init_resource::<wc::construct_elevation_supports::OverhangSupportDirtySubscriptions>();
        self.add_systems(Startup, wc::construct_elevation_supports::overhang_subscribe_to_tabloids);
        // 原set24顺序：脏区检测→圆支撑→矩形柱→柱碰撞→物理提交。
        // 托架由原set36另行消费，保持其晚于柱与主体墙生成的调度。
        self.add_systems(
            Update,
            (
                wc::construct_elevation_supports::detect_overhang_support_segments,
                wc::construct_elevation_supports::construct_circle_supports,
                wc::construct_elevation_supports::construct_rectangle_columns,
                wc::construct_elevation_supports::maintain_stone_column_colliders::maintain_stone_column_colliders,
                country_core::systems::collision::world::flush_rapier_colliders,
            ).chain_ignore_deferred().in_set(GameSystemSet(24)),
        );
        // 6. 原set28保留chain_ignore_deferred；楼板紧随墙构造、先于debug与outline。
        self.add_systems(
            Update,
            (
                wc::construct_gates::systems::update_gates_hashmap,
                wc::construct_gates::systems::construct_gates,
                wc::construct_fences::construct_fences,
                // 原set28：通用柱消费set22的拱／过梁缓存，追加各自通用实例槽。
                wc::place_pillars::place_pillars,
                wall::construct_wall_bricks::finalize_wall_holes,
                wc::construct_walls::construct_walls,
                wc::construct_floors::construct_floors,
                wall::wall_state::debug_viz_threshold_curve::debug_viz_threshold_curve,
                wall::structure_outline::structure_outline,
            )
                .chain_ignore_deferred()
                .in_set(GameSystemSet(28)),
        );
        // 原1404934e4/4e9：set32三项保持create→maintain→实体维护及ignore_deferred。
        // 同一父链生成的曲面半木及圆塔由这里维护实体。
        self.add_systems(Update,
            (wc::plaster_systems::create_plaster, wc::plaster_systems::maintain_plaster,
                wc::curved_halftimber::curved_halftimber_maintain_entities)
                .chain_ignore_deferred().in_set(GameSystemSet(32)));
        // 8. 0x1404934bf set 34
        self.add_systems(
            Update,
            (
                wall::adjust_wall_height::ui_adjust_wall_height_mode::history_record_adjust_wall_height::<
                    MaybePendingSnapshotId,
                >,
                wall::deferred_wall_history::history_record_wall_changed::<MaybePendingSnapshotId>,
                wall::deferred_wall_history::history_record_merge_walls::<MaybePendingSnapshotId>,
                country_core::systems::ui_clutter::defered_autoclutter_history::defered_autoclutter_history::<
                    MaybePendingSnapshotId,
                >,
            )
                .chain_ignore_deferred()
                .in_set(GameSystemSet(34)),
        );
        // 原140493423写0x24，140493425构造独立托架系统，140493430加入Update。
        self.add_systems(Update,
            wc::construct_elevation_supports::construct_rectangle_brackets.in_set(GameSystemSet(36)));
        // 9. 0x1404934db set 44：普通 tuple，非 chain。
        self.add_systems(
            Update,
            (
                wall::upload_instanced_wall_ssbo::gpu_flush_brick_buffers,
                wc::gpu_flush_plank_buffers,
            )
                .in_set(GameSystemSet(44)),
        );
        // 11. 0x140493574
        self.add_systems(
            bevy_state::state::OnExit(country_core::app_loading_state::AppLoadingState::Game),
            walls_on_glade_unload,
        );
        self
    }

    /// 已落地的部分（逐项有原版证据）：
    /// * `insert_resource(EraserSizeMultiplier(1.0))`：调用点 0x1404940c5，值取 0x142925904（= 1.0，
    ///   与 `draw_water` 里 `t*4 + (1-t)` 用的是同一常量）；core 的 `Default` 给 0.0，不能用 init；
    /// * 墙域 `register_event` / 资源初始化中类型已在本仓库定位的那些（见下）。
    ///
    /// **未闭合**：原函数共约 60 个事件与 20 余个资源，其中 `InitAdjWallHeightMode`、
    /// `FinishAdjWallHeightMode`、`AdjustWallHeightUi`、`MergeWallsCmd`、`PlayLineAcrossWall`、
    /// `RecalculateHalfTimberMeshCmd`、`RecalculateWoodenTurretCmd`、`OnRectangleEdited`、
    /// `RecalculateRectangleSupports/CircleSupports`、`OnShapeMoved`、`ReanchorDecorators`、
    /// `OnFreehandWallEdited`、`OnCircleRotated`、`RecalculateHalftimberBuildings`、
    /// `DisplayBetaWIPWarning`、`InProgressWallResource` 在本仓库的定义位置尚未对上，故未注册；
    /// 注册顺序（各 `register_event`/`insert_resource` 的先后）也未按调用顺序还原。
    fn add_wall_events(&mut self) -> &mut Self {
        use country_core::{
            resources::{
                roofs::commands::{RecalculateWallHoles, RoofIsInvalid},
                walls::{
                    EraserSizeMultiplier, WallHighlight,
                    plaster::{AnimatedPlasterPeel, RecalculatePlasterCmd, CreatePlasterCmd, DeletePlasterCmd,
                        RecalculateHalfTimberMeshCmd, RecalculateWoodenTurretCmd},
                    public_walls::{OnWallChanged, OnWallDeleted},
                    wall_stone_columns::{StoneColumnCollisionStorage, StoneColumnsChanged},
                },
            },
            systems::{
                terrain::deferred_terrain_history::PendingTerrainHistory,
                wall::{
                    adjust_wall_height::AdjustWallHeightCmd,
                    calc_wall_bend::WallBendChangedEvent,
                    deferred_wall_history::PendingWallHistory,
                    draw_wall::{CurveNursery, RecordMouseInputIntoWall},
                    finish_wall::{ContinueWallOriginalState, FinishWall},
                    snapping::SnappingMode,
                    structure_outline::{OutlineStairsCmd, OutlineWallCmd},
                    ui_continue_wall::{SuppressContinueWall, WallContinueState},
                    ui_draw_wall::WallDrawState,
                    wall_highlight::HighlightWallCmd,
                    wall_state::wall_spatial_hash::WallSpatialHash,
                },
            },
        };
        self.init_resource::<ContinueWallOriginalState>();
        self.init_resource::<WallSpatialHash>();
        self.init_resource::<WallDrawState>();
        self.init_resource::<WallContinueState>();
        self.init_resource::<SuppressContinueWall>();
        self.init_resource::<AnimatedPlasterPeel>();
        self.init_resource::<WallHighlight>();
        self.init_resource::<SnappingMode>();
        self.init_resource::<CurveNursery>();
        self.init_resource::<PendingWallHistory>();
        self.init_resource::<PendingTerrainHistory>();
        self.init_resource::<StoneColumnCollisionStorage>();
        self.init_resource::<country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables>();
        self.insert_resource(EraserSizeMultiplier(1.0));

        self.add_event::<FinishWall>();
        self.add_event::<RecordMouseInputIntoWall>();
        self.add_event::<RoofIsInvalid>();
        self.add_event::<RecalculateWallHoles>();
        self.add_event::<WallBendChangedEvent>();
        self.add_event::<OnWallDeleted>();
        self.add_event::<OnWallChanged>();
        self.add_event::<country_core::resources::events::OnShapeMoved>();
        self.add_event::<country_core::resources::events::OnFreehandWallEdited>();
        self.add_event::<country_core::resources::events::OnCircleRotated>();
        self.add_event::<country_core::systems::wall::shapes::rectangle::finish_edit_rectangle::OnRectangleEdited>();
        self.add_event::<AdjustWallHeightCmd>();
        self.add_event::<StoneColumnsChanged>();
        self.add_event::<RecalculatePlasterCmd>();
        // 原add_wall_events调用14057bd00/140582c60注册这两个通道。
        self.add_event::<CreatePlasterCmd>();
        self.add_event::<DeletePlasterCmd>();
        self.add_event::<RecalculateHalfTimberMeshCmd>();
        self.add_event::<RecalculateWoodenTurretCmd>();
        self.add_event::<system_wall_constructor::construct_elevation_supports::RecalculateRectangleSupports>();
        self.add_event::<system_wall_constructor::construct_elevation_supports::RecalculateCircleSupports>();
        self.add_event::<HighlightWallCmd>();
        self.add_event::<OutlineWallCmd>();
        self.add_event::<OutlineStairsCmd>();
        // add_wall_events 0x1404936b0 的其余事件（上面“未闭合”清单里现已定位的）。
        self.add_event::<country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::AdjustWallHeightUi>();
        self.add_event::<country_core::resources::events::FinishAdjWallHeightMode>();
        self.add_event::<country_core::systems::wall::merge_walls::MergeWallsCmd>();
        self.add_event::<system_wall_constructor::ui_arch_controls::DoUiArchControls>();
        self
    }
}

/// Original B @1402b1cb0,5354B; registered OnExit(Game) by
/// add_wall_systems@14049356f..574. Container resets use their current owners;
/// three actual GPU color buffers are reset in original CPU-reset/upload order.
#[allow(clippy::too_many_arguments)]
pub fn walls_on_glade_unload(
    device: bevy_ecs::prelude::Res<country_core::render::device::GraphicsDeviceResource>,
    ssbos: bevy_ecs::prelude::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,
    mut public: bevy_ecs::prelude::ResMut<
        country_core::resources::walls::public_walls::PublicWalls,
    >,
    mut plasters: bevy_ecs::prelude::ResMut<
        country_core::resources::walls::public_walls::WallPlasters,
    >,
    mut entities: bevy_ecs::prelude::ResMut<
        country_core::resources::walls::public_walls::WallEntities,
    >,
    mut inner: bevy_ecs::prelude::ResMut<country_core::resources::walls::InnerWalls>,
    mut spatial: bevy_ecs::prelude::ResMut<
        country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash,
    >,
    mut wall_colors: bevy_ecs::prelude::ResMut<country_core::resources::color_id::WallColorIds>,
    mut gate_colors: bevy_ecs::prelude::ResMut<country_core::resources::color_id::GateColorIds>,
    mut timber_colors: bevy_ecs::prelude::ResMut<
        country_core::resources::color_id::HalftimberColorIds,
    >,
    mut highlight: bevy_ecs::prelude::ResMut<country_core::resources::walls::WallHighlight>,
    mut platforms: bevy_ecs::prelude::ResMut<
        system_wall_constructor::construct_platform_stairs::PlatformStairsStorage,
    >,
    mut gates: bevy_ecs::prelude::ResMut<country_core::resources::walls::wooden_gates::Gates>,
    mut constructor: bevy_ecs::prelude::ResMut<system_wall_constructor::construct_arches::WallConstructorOutput>,
) {
    use country_core::resources::color_id::*;
    *public = Default::default();
    // 原PublicWalls析构同时释放其分配；当前sidecar沿同一退出阶段退役。
    constructor.gpu = Default::default();
    *entities = Default::default();
    *plasters = Default::default();
    // Original +10 byte only; retained id/entity payload is not zeroed.
    highlight.ty = 0;
    *platforms = Default::default();
    inner.remove_all();
    inner.reset_max_id();
    gates.clear();
    *spatial = Default::default();

    // Constructors allocate exactly0x5000 u32s and memset0 (native
    // 140b19890/140b19900/140b19970). GPU default_value returns Some(0):
    // low32=1 tag, upper32=0 value; it must not be misread as color1.
    *wall_colors = WallColorIds::new();
    reset_gpu_color_table::<WallColorIdSsbo>(&device, &ssbos);
    *gate_colors = GateColorIds::new();
    reset_gpu_color_table::<GateColorIdSsbo>(&device, &ssbos);
    *timber_colors = HalftimberColorIds::new();
    reset_gpu_color_table::<HalftimberColorIdSsbo>(&device, &ssbos);
}

fn reset_gpu_color_table<T: country_core::asset_libraries::ssbo_library::SsboType>(
    device: &country_core::render::device::GraphicsDeviceResource,
    ssbos: &country_core::asset_libraries::ssbo_library::AssetSsboLibrary,
) {
    use country_core::render::ssbo::rhapsody_boundary::GraphicsDevice;
    use country_core::resources::color_id::{ColorId, MAX_NUMBER_OF_WALLS};
    let buffer = ssbos
        .get_buffer::<T>()
        .expect("original wall color SSBO is required on unload");
    let defaults = vec![ColorId::default_value().0; MAX_NUMBER_OF_WALLS];
    let bytes = bytemuck::cast_slice(&defaults);
    assert!(
        buffer.byte_size >= bytes.len() as u64,
        "wall color SSBO is too short"
    );
    GraphicsDevice::update_buffer_bytes(device.device(), buffer, bytes, &[0..bytes.len()], 0)
        .expect("resetting original wall color SSBO on unload");
}

/// Original PostUpdate closure@1403bf630/483B: State2 requests State1.
/// It runs during the Reload frame; the following state transition enters
/// Game and invokes the canonical NextSession consumer.
pub fn advance_reload_state(
    state: bevy_ecs::prelude::Res<
        bevy_state::state::State<country_core::app_loading_state::AppLoadingState>,
    >,
    mut next: bevy_ecs::prelude::ResMut<
        bevy_state::state::NextState<country_core::app_loading_state::AppLoadingState>,
    >,
) {
    if *state.get() == country_core::app_loading_state::AppLoadingState::Reload {
        next.set(country_core::app_loading_state::AppLoadingState::Game);
    }
}
