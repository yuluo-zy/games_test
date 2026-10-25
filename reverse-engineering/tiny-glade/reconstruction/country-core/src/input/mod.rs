//! `input/mod.rs` — `ActiveInputMode` / `ActiveInputModeTracker` (controller vs
//! mouse+keyboard mode tracking with an input cooldown timestamp).
//!
//! Source: B skeleton `crates/country-core/src/input/mod.rs`.
//! PDB named entities: `ActiveInputMode`, `ActiveInputModeTracker`.
//!
//! O evidence (Ghidra, B patched EXE):
//! - `ActiveInputModeTracker::new_default_controller` @0x140958eb0 /
//!   `new_default_mouse_keyboard` @0x140958ef0 — builds the 32 B tracker
//!   (`std::time::Instant::now()` + duration math), writing a mode u32 + an
//!   Instant/Duration pair.
//! - `mode` @0x140958f30 — returns the mode word (@+0x00).
//! - `on_controller_used` / `on_mouse_keyboard_used` (O; inline into
//!   `systems::steam::update_steam_inputs`).
//! - `ActiveInputMode::is_controller` @0x1408c5680 (folds with
//!   `StructureType::is_fence` — identical code folding) — 31 callers.

#![allow(dead_code, unused_imports)]

use serde::{Deserialize, Serialize};

use crate::Gap;

/// `ActiveInputMode` (O named entity): input-mode discriminant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ActiveInputMode {
    /// Mouse + keyboard (I: default).
    #[default]
    MouseKeyboard,
    /// Controller (I).
    Controller,
}

impl ActiveInputMode {
    /// `ActiveInputMode::is_controller` (O; discriminant value U — inferred
    /// `Controller`).
    pub fn is_controller(self) -> bool {
        matches!(self, ActiveInputMode::Controller)
    }
}

/// `ActiveInputModeTracker` (O named entity; 32 B).
#[derive(Clone, Debug, Default)]
pub struct ActiveInputModeTracker {
    /// Current mode (@+0x00, O).
    pub mode: ActiveInputMode,
    /// Last-input timestamp / cooldown (Instant + Duration, O write pattern;
    /// layout U).
    pub _gap: Gap,
}

impl ActiveInputModeTracker {
    /// `new_default_controller` @0x140958eb0 (O).
    pub fn new_default_controller() -> Self {
        ActiveInputModeTracker {
            mode: ActiveInputMode::Controller,
            _gap: Gap,
        }
    }
    /// `new_default_mouse_keyboard` @0x140958ef0 (O).
    pub fn new_default_mouse_keyboard() -> Self {
        ActiveInputModeTracker {
            mode: ActiveInputMode::MouseKeyboard,
            _gap: Gap,
        }
    }
    /// `mode` @0x140958f30 (O).
    pub fn mode(&self) -> ActiveInputMode {
        self.mode
    }
    /// `on_controller_used` (O): record controller use.
    pub fn on_controller_used(&mut self) {
        self.mode = ActiveInputMode::Controller;
    }
    /// `on_mouse_keyboard_used` (O): record mouse/keyboard use.
    pub fn on_mouse_keyboard_used(&mut self) {
        self.mode = ActiveInputMode::MouseKeyboard;
    }
}

// WP15 input domain submodules (declared here; wp20 backend::input consumes).
pub mod action_set;
pub mod input_stream;

// WP20 backend::input event markers (O named entities; layout U).
/// `MouseMoved`（O：游标增量事件，回放路径产出；布局 U）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct MouseMoved(pub crate::Gap);

/// `InputCursorMoved`（O：合并后的游标移动事件；布局 U）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct InputCursorMoved(pub crate::Gap);
