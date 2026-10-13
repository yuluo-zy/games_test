//! `game_config.rs` — GameConfig / DevSettings / GameSettings / ScreenshotFormat
//! (app-shell persisted settings) + the glade-shape clamping helpers.
//!
//! Source: B skeleton `crates/country-core/src/game_config.rs` (sha `crates/...`).
//! PDB named entities: `DevSettings`, `GameConfig`, `GameSettings`, `ScreenshotFormat`.
//!
//! O evidence (Ghidra, B patched EXE):
//! - `GameConfig::upgrade` @0x14090df90 — version migration: if byte@0x88 flag clear,
//!   zero int@0x80; when old int@0x80 != 0 && old float@0x84 is `>0.125 && !=1.0`,
//!   set byte@0x88 = 1 and move float@0x84 → float@0x8c (a scale-override migration).
//! - `is_within_glade_shape` @0x14090dfe0 (inlines `clamp_to_glade_shape` +
//!   `clamp_to_circle`) — **glade bounds**: non-demo mode clamps X/Y to a rounded
//!   box with half-extent `f32 0x42c28200 ≈ 97.2538` (release); demo mode
//!   (`_DEV_OVERRIDE_IS_DEMO_MODE`) clamps to a circle radius 65 (`4225 = 65²`).
//! - `DevSettings::live_link` @0x14090df40 / `developer_hotkeys_enabled`
//!   @0x14090e070 — tiny accessors (PDB `void()` placeholder; real ABI = bool read).
//! - `_::impl$0::{serialize,deserialize}` — RON (ron 0.8.1) persisted.
//!
//! Layouts of the settings structs are U (typed gap) except the offsets observed
//! in `upgrade`.

#![allow(dead_code, unused_imports)]

use serde::{Deserialize, Serialize};

use crate::Gap;

/// Screenshot capture format (O named entity; enum discriminant U).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScreenshotFormat {
    /// Png (I: first variant; discriminant U).
    Png,
    /// Jpeg (I; discriminant U).
    Jpeg,
}

/// `game_config::GameSettings` (O named entity; layout U — typed gap).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GameSettings {
    /// Typed gap: recovered-game-setting slots (layout U).
    pub _gap: Gap,
}

/// `game_config::DevSettings` (O named entity). Layout U.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DevSettings {
    pub _gap: Gap,
}

impl DevSettings {
    /// `DevSettings::live_link` @0x14090df40 (O method; reads a dev bool).
    pub fn live_link(&self) -> bool {
        false
    }
}

/// `game_config::GameConfig` (O named entity; persisted via ron 0.8.1).
///
/// Observed offsets (O, `GameConfig::upgrade`): an int @+0x80, a float @+0x84,
/// an upgrade-flag byte @+0x88, and a float @+0x8c written by the migration.
/// The remaining layout is a typed gap.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameConfig {
    /// `_gap` for the unrecovered settings blob.
    pub _gap: Gap,
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig { _gap: Gap }
    }
}

impl GameConfig {
    /// `GameConfig::upgrade` @0x14090df90 (O).
    /// Migration: when the flag byte @0x88 is clear, zero the int @0x80 and, if
    /// the legacy value was present (`int@0x80 != 0`) and its float @0x84 was
    /// `> 0.125 && != 1.0`, set the flag @0x88 and copy the float to @0x8c.
    /// (Old Option<(bool, f32)> encoding → new bool + f32 override.)
    pub fn upgrade(&mut self) {
        // O offsets documented; exact field mapping is a typed gap, so the
        // migration body is intentionally a no-op skeleton until the GameConfig
        // layout is recovered.
        let _ = self;
        todo!("recover GameConfig layout (int@0x80, float@0x84, flag@0x88, float@0x8c) to implement upgrade")
    }
}

// ---------------------------------------------------------------------------
// Glade-shape clamping (game-owned world bounds).
// ---------------------------------------------------------------------------

/// `_DEV_OVERRIDE_IS_DEMO_MODE` (O: read in `is_within_glade_shape`/`startup_terrain`).
/// A global override flag; treated as false (release) by default.
pub fn dev_override_is_demo_mode() -> bool {
    false
}

/// Release glade half-extent: `f32` constant `0x42c28200` (~97.2538) used by
/// the rounded-box clamp (O: `is_within_glade_shape`).
pub const GLADE_ROUNDED_BOX_HALF_EXTENT: f32 = f32::from_bits(0x42c28200);
/// Demo-mode glade circle radius: `65` (`4225 = 65²` checked in
/// `clamp_to_circle`, O).
pub const GLADE_DEMO_CIRCLE_RADIUS: f32 = 65.0;

/// `clamp_to_circle` (O inline into `is_within_glade_shape`): clamp a Vec2 to a
/// circle of `GLADE_DEMO_CIRCLE_RADIUS`.
pub fn clamp_to_circle(pos: glam::Vec2) -> glam::Vec2 {
    let len = pos.length();
    if len > GLADE_DEMO_CIRCLE_RADIUS && len != 0.0 {
        pos * (GLADE_DEMO_CIRCLE_RADIUS / len)
    } else {
        pos
    }
}

/// `clamp_to_rounded_box` (O inline into `is_within_glade_shape`): clamp a Vec2
/// to `±GLADE_ROUNDED_BOX_HALF_EXTENT` per axis.
pub fn clamp_to_rounded_box(pos: glam::Vec2) -> glam::Vec2 {
    glam::Vec2::new(
        pos.x.clamp(-GLADE_ROUNDED_BOX_HALF_EXTENT, GLADE_ROUNDED_BOX_HALF_EXTENT),
        pos.y.clamp(-GLADE_ROUNDED_BOX_HALF_EXTENT, GLADE_ROUNDED_BOX_HALF_EXTENT),
    )
}

/// `clamp_to_glade_shape` (O inline into `is_within_glade_shape`): pick
/// demo-circle or release-rounded-box by `_DEV_OVERRIDE_IS_DEMO_MODE`.
pub fn clamp_to_glade_shape(pos: glam::Vec2) -> glam::Vec2 {
    if dev_override_is_demo_mode() {
        clamp_to_circle(pos)
    } else {
        clamp_to_rounded_box(pos)
    }
}

/// `is_within_glade_shape` @0x14090dfe0 (O): the glade-boundary clamp used by
/// the camera rig (`systems::camera_rig::normal::impl$1::update`).
pub fn is_within_glade_shape(pos: glam::Vec2) -> glam::Vec2 {
    clamp_to_glade_shape(pos)
}

/// `game_config::developer_hotkeys_enabled` @0x14090e070 (O method; reads the
/// dev-settings hotkey flag).
pub fn developer_hotkeys_enabled() -> bool {
    false
}

/// `clamp_to_rounded_glade_shape` (O inline into camera rig): rounded-box clamp.
pub fn clamp_to_rounded_glade_shape(pos: glam::Vec2) -> glam::Vec2 {
    clamp_to_rounded_box(pos)
}
