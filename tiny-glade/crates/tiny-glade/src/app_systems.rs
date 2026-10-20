//! Original Tiny Glade host wall/session lifecycle. The complete unload body
//! @1402b1cb0 and Reload→Game PostUpdate consumer are recovered below.
//! Broader normal wall-system/event registration remains a separate unmounted
//! entry until every original consumer in that tuple is integrated.

/// `<bevy_app::App as tiny_glade::app_systems::WallSystems>`: `add_wall_systems` (0x140493120) and
/// `add_wall_events` (0x1404936b0, 39KB 事件/资源注册表)。
pub trait WallSystems {
    fn add_wall_systems(&mut self) -> &mut Self;
    fn add_wall_events(&mut self) -> &mut Self;
}

impl WallSystems for bevy_app::App {
    /// 未实现：注册顺序已解（见 `run_game.rs` 内 add_wall_systems 片段注释），系统函数名/ResMut 面
    /// 未逐个锚定。
    fn add_wall_systems(&mut self) -> &mut Self {
        todo!("未实现: add_wall_systems 0x140493120")
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
                    plaster::{AnimatedPlasterPeel, RecalculatePlasterCmd},
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
        self.insert_resource(EraserSizeMultiplier(1.0));

        self.add_event::<FinishWall>();
        self.add_event::<RecordMouseInputIntoWall>();
        self.add_event::<RoofIsInvalid>();
        self.add_event::<RecalculateWallHoles>();
        self.add_event::<WallBendChangedEvent>();
        self.add_event::<OnWallDeleted>();
        self.add_event::<OnWallChanged>();
        self.add_event::<AdjustWallHeightCmd>();
        self.add_event::<StoneColumnsChanged>();
        self.add_event::<RecalculatePlasterCmd>();
        self.add_event::<HighlightWallCmd>();
        self.add_event::<OutlineWallCmd>();
        self.add_event::<OutlineStairsCmd>();
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
) {
    use country_core::resources::color_id::*;
    *public = Default::default();
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
