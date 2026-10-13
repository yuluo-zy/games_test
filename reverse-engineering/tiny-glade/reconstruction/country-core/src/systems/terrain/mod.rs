//! systems::terrain — 地形系统模块（WP8 恢复）。
//!
//! 源：B skeleton `systems/terrain/mod.rs` + 子模块。反编译地址：
//! - `add_terrain_systems`@0x1409455f0（行 37）
//! - `insert_terrain_resources`@0x1409457c0（行 56）
//! - `add_terrain_events`@0x140945a20（行 65）
//!
//! `insert_terrain_resources`（O）：`insert_resource<MouseActivityTimer>`、
//! `insert_resource<TerrainStrokeParamsCache>`、`insert_resource<TerrainHeightsData<TerrainHeightSsbo>>`、
//! `insert_resource<TerrainHeightsData<VisualTerrainHeightSsbo>>`、`insert_resource<HoveredTerrainStrokeId>`、
//! `insert_resource<ScheduledTerrainReadback>`（含 `RtTriangleMetaSsbo` Vec 扩展）。
//!
//! `add_terrain_events`（O）：注册 `EvTerrainStrokeChanged` / `EditTerrainStrokeCmd` /
//! `AddTerrainHighlightCmd` / `RecalculateHeightMapCmd` / `OnTerrainChange` / `SetTerrainStrokePtsCmd`。

#![allow(dead_code, unused_imports)]

pub mod deferred_terrain_history;
pub mod delayed_terrain_changed_msg;
pub mod draw_terrain_stroke;
pub mod edit_terrain_stroke;
pub mod terrain_hover_idx;
pub mod ui_draw_terrain;
pub mod ui_edit_terrain;
pub mod ui_terrain_focus_mode;

use bevy_ecs::change_detection::Res;
use bevy_ecs::resource::Resource;

use crate::resources::terrain_height::{
    TerrainHeightsData, TerrainHeightSsbo, VisualTerrainHeightSsbo,
};
use crate::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback;
use crate::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId;
use crate::systems::terrain::ui_edit_terrain::{MouseActivityTimer, TerrainStrokeParamsCache};
use crate::typed_gap::Gap;

// ===========================================================================
// 依赖 typed-gap 门面（O 名；布局 U）。不臆造字段。
// ===========================================================================

/// `systems::mode_manager::AppMode`（O 名；ECS 签名使用）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AppMode(pub Gap);

/// `systems::mod_key::Modifiers`（O 名）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct Modifiers(pub Gap);

/// `systems::ui::resources::ui_state::UiState`（O 名）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct UiState(pub Gap);

/// `systems::terrain::deferred_terrain_history::PendingTerrainHistory`（O 名）。
#[derive(Resource, Clone, Debug, Default)]
pub struct PendingTerrainHistory(pub Gap);

/// `resources::walls::decorator_archivist::DecoratorArchivist`（O 名）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DecoratorArchivist(pub Gap);

/// `resources::glade_border::GladeBorder`（O 名）。
#[derive(Resource, Clone, Debug, Default)]
pub struct GladeBorder(pub Gap);

/// `resources::glade_border::DisplayOutOfBorder` / `FeedbackInputOutOfBorder`（O 名，事件）。
#[derive(Clone, Copy, Debug, Default)]
pub struct DisplayOutOfBorder(pub Gap);
#[derive(Clone, Copy, Debug, Default)]
pub struct FeedbackInputOutOfBorder(pub Gap);

/// `resources::mouse_capture::CursorCaptureCmd`（O 名，事件）。
#[derive(Clone, Copy, Debug, Default)]
pub struct CursorCaptureCmd(pub Gap);

/// `systems::arrow_gizmo_animation::EnableArrowGizmoStretch`（O 名，事件）。
#[derive(Clone, Copy, Debug, Default)]
pub struct EnableArrowGizmoStretch(pub Gap);

/// `resources::onboarding::NotifyHintSystemTerrain` / `NotifyHintSystemTerrainSteepness` /
/// `NotifyHintSystemToolWasJustUsed`（O 名，事件）。
#[derive(Clone, Copy, Debug, Default)]
pub struct NotifyHintSystemTerrain(pub Gap);
#[derive(Clone, Copy, Debug, Default)]
pub struct NotifyHintSystemTerrainSteepness(pub Gap);
#[derive(Clone, Copy, Debug, Default)]
pub struct NotifyHintSystemToolWasJustUsed(pub Gap);

/// `systems::main_camera::MainCameraMoved`（O 名，事件）。
#[derive(Clone, Copy, Debug, Default)]
pub struct MainCameraMoved(pub Gap);

/// `resources::window::CursorMovedScreenSpace`（O 名，事件）。
#[derive(Clone, Copy, Debug, Default)]
pub struct CursorMovedScreenSpace(pub Gap);

/// `resources::time::GameTime`（O 名）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct GameTime(pub Gap);

/// `systems::audio::terrain_audio::PlayTerrainSound`（O 名，事件；判别 0/1/…，prior edit.rs）。
#[derive(Clone, Copy, Debug, Default)]
pub struct PlayTerrainSound(pub Gap);

/// `systems::history::history_process::DebugHistoryMsg`（O 名，事件）。
#[derive(Clone, Copy, Debug, Default)]
pub struct DebugHistoryMsg(pub Gap);

// ===========================================================================
// 模块注册函数（O 反编译）。
// ===========================================================================

/// 添加地形系统（O：`add_terrain_systems`@0x1409455f0）。
/// 反编译显示向 `Update` 调度加入 `history_record_terrain`（in_set `GameSystemSet`）。
pub fn add_terrain_systems() {
    unimplemented!("recover add_terrain_systems schedule wiring")
}

/// 插入地形资源（O：`insert_terrain_resources`@0x1409457c0）。
pub fn insert_terrain_resources(_world: &mut Gap) {
    let _ = (
        MouseActivityTimer::default(),
        TerrainStrokeParamsCache::default(),
        TerrainHeightsData::<TerrainHeightSsbo>::default(),
        TerrainHeightsData::<VisualTerrainHeightSsbo>::default(),
        HoveredTerrainStrokeId::default(),
        ScheduledTerrainReadback::default(),
    );
    unimplemented!("recover insert_terrain_resources")
}

/// 注册地形事件（O：`add_terrain_events`@0x140945a20）。
pub fn add_terrain_events(_world: &mut Gap) {
    unimplemented!("recover add_terrain_events")
}

// 供 `add_terrain_systems` 引用的历史系统（O 签名；见 deferred_terrain_history.rs）。
pub use deferred_terrain_history::history_record_terrain;
