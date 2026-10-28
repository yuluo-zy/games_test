//! Recovered CPU terrain event consumers, tool UI producers and delayed
//! mapped-height readback. GPU execution and original global schedule remain separate.
pub mod deferred_terrain_history;
pub mod delayed_terrain_changed_msg;
pub mod draw_terrain_stroke;
pub mod edit_terrain_stroke;
pub mod replay_terrain;
pub mod terrain_hover_idx;
pub mod ui_draw_terrain;
pub mod ui_edit_terrain;
pub mod ui_terrain_focus_mode;
#[cfg(test)]
mod tests;

pub use crate::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd;
use bevy_ecs::prelude::*;
use glam::Vec3;

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
    CreateChunk { position: Vec3, height: f32 },
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
    world.init_resource::<ui_edit_terrain::TerrainEditUiState>();
    world.init_resource::<ui_edit_terrain::TerrainWidgetResponseTable>();
    // 宿主 UI 悬停/交互适配（ui_edit 的 cursor tag / hovered id 由宿主写入）。
    world.init_resource::<ui_draw_terrain::TerrainHoverInput>();
    world.init_resource::<ui_draw_terrain::TerrainDrawState>();
    // 聚焦模式适配（raycast/hover/selection 由宿主写入）。
    world.init_resource::<ui_terrain_focus_mode::TerrainFocusRaycast>();
    world.init_resource::<ui_terrain_focus_mode::TerrainFocusHover>();
    world.init_resource::<ui_terrain_focus_mode::TerrainFocusSelection>();
    world.init_resource::<Events<ui_edit_terrain::TerrainWindowFocus>>();
    world.init_resource::<Events<ui_terrain_focus_mode::SignifierDrawTopology>>();
    world.init_resource::<Events<ui_terrain_focus_mode::TerrainFocusTrigger>>();
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
            ui_terrain_focus_mode::ui_terrain_focus_mode,
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
    schedule.add_systems(
        deferred_terrain_history::history_record_terrain::<S>
            .after(ui_edit_terrain::ui_edit_terrain)
            .in_set(crate::systems::history::HistoryCpuStage::Record),
    );
}
