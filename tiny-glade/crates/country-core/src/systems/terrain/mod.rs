//! Recovered CPU terrain event consumers, tool UI producers and delayed
//! mapped-height readback. GPU execution and original global schedule remain separate.
pub mod deferred_terrain_history;
pub mod delayed_terrain_changed_msg;
pub mod draw_terrain_stroke;
pub mod edit_terrain_stroke;
pub mod replay_terrain;
pub mod terrain_hover_idx;
#[cfg(test)]
mod tests;
pub mod ui_draw_terrain;
pub mod ui_edit_terrain;
pub mod ui_terrain_focus_mode;

pub use crate::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd;
use bevy_ecs::prelude::*;
use glam::Vec3;

use crate::systems::render_loop::compute_passes::read_terrain_data::ScheduledTerrainReadback;

/// Original terrain sound observations. The SystemParam identity and audio
/// consumer determine the type; folded grow_one names are not event identities.
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub enum PlayTerrainSound {
    StrokeChanged,
    Falloff {
        position: Vec3,
        falloff: f32,
        magnitude: f32,
    },
    Height {
        position: Vec3,
        falloff: f32,
        height: f32,
        magnitude: f32,
    },
    Points {
        position: Vec3,
        falloff: f32,
        height: f32,
        magnitude: f32,
    },
    CreateChunk {
        position: Vec3,
        height: f32,
    },
    Removed {
        position: Vec3,
        falloff: f32,
        magnitude: f32,
    },
}
impl PlayTerrainSound {
    pub fn native_tag(&self) -> u32 {
        match self {
            Self::StrokeChanged => 0,
            Self::Falloff { .. } => 1,
            Self::Height { .. } => 2,
            Self::Points { .. } => 3,
            Self::Removed { .. } => 4,
            Self::CreateChunk { .. } => 5,
        }
    }
}

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TerrainCpuStage {
    Edit,
    Changed,
    Readback,
}

pub fn register_terrain_events(world: &mut World) {
    world.init_resource::<Events<edit_terrain_stroke::EditTerrainStrokeCmd>>();
    world.init_resource::<Events<edit_terrain_stroke::SetTerrainStrokePtsCmd>>();
    world.init_resource::<Events<draw_terrain_stroke::EvTerrainStrokeChanged>>();
    world.init_resource::<Events<delayed_terrain_changed_msg::OnTerrainChange>>();
    world.init_resource::<Events<RecalculateHeightMapCmd>>();
    world.init_resource::<Events<PlayTerrainSound>>();
    world.init_resource::<Events<crate::resources::onboarding::NotifyHintSystemTerrain>>();
    world.init_resource::<Events<crate::systems::mode_manager::RequestAppModeChange>>();
    world.init_resource::<Events<crate::resources::onboarding::NotifyHintSystemTerrainSteepness>>();
    world.init_resource::<Events<crate::resources::onboarding::NotifyHintSystemToolWasJustUsed>>();
    world.init_resource::<Events<crate::resources::glade_border::DisplayOutOfBorder>>();
    world.init_resource::<Events<crate::resources::glade_border::FeedbackInputOutOfBorder>>();
    world.init_resource::<Events<crate::resources::mouse_capture::CursorCaptureCmd>>();
    world.init_resource::<Events<crate::systems::arrow_gizmo_animation::EnableArrowGizmoStretch>>();
    world.init_resource::<Events<crate::systems::render_loop::compute_passes::highlight_active_terrain_stroke::AddTerrainHighlightCmd>>();
    world.init_resource::<Events<crate::systems::camera_rig::main_camera::MainCameraMoved>>();
    world.init_resource::<crate::systems::backend::input::mod_key::Modifiers>();

    world.init_resource::<Events<deferred_terrain_history::DebugHistoryMsg>>();
    world.init_resource::<Events<crate::resources::window::CursorMovedScreenSpace>>();

    world.init_resource::<deferred_terrain_history::PendingTerrainHistory>();
    world.init_resource::<crate::resources::walls::decorator_archivist::LiveDecoratorArchivist>();
    world.init_resource::<terrain_hover_idx::HoveredTerrainStrokeId>();
    world.init_resource::<terrain_hover_idx::MouseTerrainHoverReadback>();
    world.init_resource::<terrain_hover_idx::TerrainWaterSample>();
    // 台阶放置消费面（terrain→stairs/decorator 链；stairs 域消费后清空）。
    world.init_resource::<ui_edit_terrain::StairsPlacementBucket>();
    // 地形编辑历史 pending（ui_edit_terrain case 3 写 TerrainEdit 哨兵 0x…0001）。
    world.init_resource::<ui_edit_terrain::TerrainEditHistoryPending>();
    // 宿主兼容注册：保留已存在的原计时资源。
    world.init_resource::<ui_edit_terrain::MouseActivityTimer>();
    world.init_resource::<ui_edit_terrain::TerrainEditUiState>();
    world.init_resource::<ui_edit_terrain::TerrainWidgetResponseTable>();
    // 宿主 UI 悬停/交互适配（ui_edit 的 cursor tag / hovered id 由宿主写入）。
    world.init_resource::<ui_draw_terrain::TerrainHoverInput>();
    world.init_resource::<ui_draw_terrain::TerrainDrawState>();
    world.init_resource::<Events<ui_edit_terrain::TerrainWindowFocus>>();
}

/// Local CPU order; requires the actual editing state, game clock, readback
/// owner, SSBO library and both CPU height resources to be inserted explicitly.
pub fn add_terrain_cpu_systems(schedule: &mut Schedule) {
    schedule.configure_sets(
        (
            TerrainCpuStage::Edit,
            TerrainCpuStage::Changed,
            TerrainCpuStage::Readback,
        )
            .chain(),
    );
    schedule.add_systems(edit_terrain_stroke::edit_terrain_stroke.in_set(TerrainCpuStage::Edit));
    schedule
        .add_systems(draw_terrain_stroke::terrain_stroke_changed.in_set(TerrainCpuStage::Changed));
    schedule.add_systems(
        delayed_terrain_changed_msg::delayed_terrain_changed_msg.in_set(TerrainCpuStage::Readback),
    );
}

/// 工具 UI 与历史生产（原 add_terrain_systems 0x1409455f0 挂载面；宿主提供
/// AppMode/mode/hover/SSBO 回读/History）。独立于 [`add_terrain_cpu_systems`]，测试 World
/// 无需注入宿主资源。
pub fn add_terrain_tool_systems<S: Send + Sync + 'static>(schedule: &mut Schedule) {
    // The UI producers share one pending owner. Consume it after release, and
    // apply replay effects before the history bucket can finish this frame.
    schedule.add_systems(
        (
            terrain_hover_idx::terrain_hover_idx,
            crate::systems::ui_edit_focus::ui_edit_focus.before(
                crate::systems::mode_manager::mode_manager::<
                    crate::resources::history::EditType,
                    S,
                >,
            ),
            ui_terrain_focus_mode::ui_terrain_focus_mode
                .after(crate::systems::mode_manager::ModeChangePublication),
            ui_draw_terrain::ui_draw_terrain,
            ui_edit_terrain::ui_edit_terrain,
        )
            .chain()
            .after(crate::systems::backend::input::InputSystemSet::Derive)
            .before(TerrainCpuStage::Edit),
    );
    schedule.add_systems(
        crate::systems::arrow_gizmo_animation::arrow_gizmo_animation
            .after(ui_edit_terrain::ui_edit_terrain)
            .before(TerrainCpuStage::Edit),
    );
    schedule.add_systems(
        replay_terrain::replay_terrain
            .in_set(crate::systems::history::HistoryCpuStage::Replay)
            .before(TerrainCpuStage::Edit),
    );
    schedule.configure_sets(
        TerrainCpuStage::Changed.before(crate::systems::history::HistoryCpuStage::Pop),
    );
    schedule.configure_sets(
        crate::systems::history::HistoryCpuStage::Cursor
            .after(crate::systems::mode_manager::ModeChangePublication),
    );
    schedule.add_systems(
        deferred_terrain_history::history_record_terrain::<S>
            .after(ui_edit_terrain::ui_edit_terrain)
            .in_set(crate::systems::history::HistoryCpuStage::Record),
    );
}

/// `<bevy_app::App as country_core::systems::terrain::TerrainSystems>`（原版三个方法：
/// `add_terrain_systems` 0x1409455f0、`insert_terrain_resources` 0x1409457c0、
/// `add_terrain_events` 0x140945a20）。
///
/// `add_terrain_systems`：`SubApp::add_systems(Update, (chain_ignore_deferred((ui_terrain_focus_mode,
/// ui_draw_terrain, ui_edit_terrain, edit_terrain_stroke, terrain_stroke_changed,
/// delayed_terrain_changed_msg)).in_set(GameSystemSet(0x0f)),
/// history_record_terrain.in_set(GameSystemSet(0x22))))`（chain 0x140945613，`MOV byte,0xf` 0x140945618，
/// in_set_inner 0x14094563d；`DL=0x22` in_set 0x14094569c；add_systems 0x140945742；顺序取自单体
/// 0x141755b40 的 `into_configs` 调用序；无 `run_if`）。原版此函数不含 terrain_hover_idx /
/// replay_terrain / ui_edit_focus / arrow_gizmo_animation（本仓库 `add_terrain_tool_systems`
/// 另行挂载，原版挂载点未定位，见报告）。
///
/// `insert_terrain_resources`（顺序）：MouseActivityTimer（0x1409457f2，原初始值已恢复；消费者未定位）、
/// TerrainStrokeParamsCache（0x14094580c，由 TerrainEditUiState 承载编辑参数）、
/// `TerrainHeightsData<TerrainHeightSsbo>`（0x1409458a5）、`TerrainHeightsData<VisualTerrainHeightSsbo>`
/// （0x140945936）、`HoveredTerrainStrokeId`（0x140945940，全零）、`ScheduledTerrainReadback`（0x140945958，全零）。
///
/// `add_terrain_events`（6 次 register_event，顺序）：EvTerrainStrokeChanged（0x140945a84）、
/// EditTerrainStrokeCmd（0x140945ae0）、AddTerrainHighlightCmd（0x140945b3c）、RecalculateHeightMapCmd
/// （0x140945b98）、OnTerrainChange（0x140945bf4）、SetTerrainStrokePtsCmd（0x140945c50）。
pub trait TerrainSystems {
    fn add_terrain_systems(&mut self) -> &mut Self;
    fn insert_terrain_resources(&mut self) -> &mut Self;
    fn add_terrain_events(&mut self) -> &mut Self;
}

impl TerrainSystems for bevy_app::App {
    fn add_terrain_systems(&mut self) -> &mut Self {
        use crate::resources::history::MaybePendingSnapshotId as S;
        use crate::systems::GameSystemSet;
        self.add_systems(
            bevy_app::Update,
            (
                (
                    ui_terrain_focus_mode::ui_terrain_focus_mode,
                    ui_draw_terrain::ui_draw_terrain,
                    ui_edit_terrain::ui_edit_terrain,
                    edit_terrain_stroke::edit_terrain_stroke,
                    draw_terrain_stroke::terrain_stroke_changed,
                    delayed_terrain_changed_msg::delayed_terrain_changed_msg,
                )
                    .chain_ignore_deferred()
                    .in_set(GameSystemSet(0x0f)),
                deferred_terrain_history::history_record_terrain::<S>.in_set(GameSystemSet(0x22)),
            ),
        )
    }

    fn insert_terrain_resources(&mut self) -> &mut Self {
        use crate::resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData, VisualTerrainHeightSsbo};
        // 原构造先插入计时资源；字段语义及消费者尚未确认。
        self.insert_resource(ui_edit_terrain::MouseActivityTimer::default());
        // 原参数缓存由现有编辑状态承载。
        self.insert_resource(ui_edit_terrain::TerrainEditUiState::default());
        self.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
        self.insert_resource(TerrainHeightsData::<VisualTerrainHeightSsbo>::new());
        self.insert_resource(terrain_hover_idx::HoveredTerrainStrokeId::default());
        self.insert_resource(ScheduledTerrainReadback::default())
    }

    fn add_terrain_events(&mut self) -> &mut Self {
        use crate::systems::render_loop::compute_passes::highlight_active_terrain_stroke::AddTerrainHighlightCmd;
        self.add_event::<draw_terrain_stroke::EvTerrainStrokeChanged>();
        self.add_event::<edit_terrain_stroke::EditTerrainStrokeCmd>();
        self.add_event::<AddTerrainHighlightCmd>();
        self.add_event::<RecalculateHeightMapCmd>();
        self.add_event::<delayed_terrain_changed_msg::OnTerrainChange>();
        self.add_event::<edit_terrain_stroke::SetTerrainStrokePtsCmd>()
    }
}
