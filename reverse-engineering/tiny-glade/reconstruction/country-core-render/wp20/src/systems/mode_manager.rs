//! `systems/mode_manager.rs` — the app-mode state machine.
//!
//! PDB: `mode_manager @0x140a86b60` (2680 B), `send_mode_just_changed_event
//! @0x140a875e0`, `AppMode::is_tool_selected @0x140a8a5d0`, 15
//! `check_if_can_switch<closure_env$0..14>` (each ~280-373 B),
//! `AppMode` serde glue. Named entities: `AppMode`, `PrevAppMode`,
//! `RequestAppModeChange`, `RequestToolbarCategoryChange`,
//! `RequestHistoryCommand`, `EditSubject`, `WallShape`, `CobblestoneVariant`,
//! `EraserLayer`, `UiHistoryPlaySelectAnimation`.
//!
//! O: `mode_manager` early-returns when a stored hash @+0x38 matches the
//! current mode-name hash and mode @+0x28 != 2; trims the input stream; when
//! un-consumed mode-change requests remain runs
//! `check_if_can_switch<closure_env_0>`; then dispatches a jumptable indexed
//! by the pending mode u32 @+0x80. `send_mode_just_changed_event` compares the
//! `AppMode` (2-qword enum) before/after and pushes an event `{prev, current}`
//! (stride 0x28) + updates the current mode.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::Gap;

/// `AppMode` (O named entity; 2-qword enum, serde round-trip).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum AppMode {
    #[default]
    None,
    /// Timelapse mode (O: `check_if_can_switch` for timelapse).
    Timelapse,
    /// Wall-shape mode (O: `WallShape` payload).
    Wall(WallShape),
    /// Focus mode (O: `EditSubject` payload).
    Focus(EditSubject),
    /// Other toolbar tool modes (O: `CobblestoneVariant` / `EraserLayer`
    /// payloads; variant set U).
    Tool(Gap),
}

/// `WallShape` (O named entity).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct WallShape(pub Gap);

/// `EditSubject` (O named entity; geometry-source → focus subject).
#[derive(
    bevy_ecs::resource::Resource, Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize,
)]
pub struct EditSubject(pub Gap);

/// `CobblestoneVariant` (O named entity).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct CobblestoneVariant(pub Gap);

/// `EraserLayer` (O named entity).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct EraserLayer(pub Gap);

/// `PrevAppMode` (O named entity; resource holding the previous mode).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct PrevAppMode {
    pub mode: AppMode,
}

/// `RequestAppModeChange` (O named entity; event).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct RequestAppModeChange(pub AppMode);

/// `RequestToolbarCategoryChange` (O named entity).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct RequestToolbarCategoryChange(pub Gap);

/// `RequestHistoryCommand` (O named entity).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct RequestHistoryCommand(pub Gap);

/// `UiHistoryPlaySelectAnimation` (O named entity).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct UiHistoryPlaySelectAnimation(pub Gap);

/// `AppMode::is_tool_selected` @0x140a8a5d0 (O; folded with `default`).
impl AppMode {
    pub fn is_tool_selected(&self) -> bool {
        false
    }
}

/// `mode_manager` @0x140a86b60 (O).
pub fn mode_manager(
    mut _prev: ResMut<PrevAppMode>,
    mut _requests: EventReader<RequestAppModeChange>,
    mut _history_requests: EventReader<RequestHistoryCommand>,
    mut _toolbar_requests: EventReader<RequestToolbarCategoryChange>,
    _state: Res<Gap>,
    mut _commands: Commands,
) {
    todo!("recover mode_manager (early-return hash + per-mode check_if_can_switch dispatch)")
}

/// `send_mode_just_changed_event` @0x140a875e0 (O).
pub fn send_mode_just_changed_event(
    _prev: Res<PrevAppMode>,
    _current: ResMut<Gap>,
    mut _events: EventWriter<RequestAppModeChange>,
) {
    todo!("recover send_mode_just_changed_event (AppMode ne + event push)")
}

/// `check_if_can_switch` (O: 15 monomorphizations, one per mode). Returns
/// whether the pending mode switch is allowed.
pub fn check_if_can_switch(
    _pending: RequestAppModeChange,
    _state: Res<Gap>,
) -> bool {
    todo!("recover per-mode check_if_can_switch conditions")
}
