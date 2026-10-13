//! `systems/ui/resources/widget_params.rs` — `WidgetParams` / `WidgetVolumeShape` /
//! `UiSounds` / `UiFocus` / `WidgetFlags` / `UiNavState` / `WorldCanvasLayer` +
//! `PlayUiSound` / `PlayPhotomodeSound` events.
//!
//! Source: B skeleton `crates/country-core/src/systems/ui/resources/widget_params.rs`.
//! PDB named entities: `PlayPhotomodeSound`, `PlayUiSound`, `UiFocus`,
//! `UiNavState`, `UiSounds`, `WidgetFlags`, `WidgetParams`, `WidgetVolumeShape`,
//! `WorldCanvasLayer`, and flag constants `ALLOW_NEGATIVE_DISTANCE`,
//! `DECORATOR`, `HIDE_OS_CURSOR`.
//!
//! O evidence (Ghidra, B patched EXE):
//! - `WidgetParams::new` @0x1409a8e10: layout **0xb0 B**; defaults write
//!   u32 `{0,8,0,1,0,2,3,3}` at `{0,8,0x10,0x18,0x20,0x30,0x48,0x60}`, copies a
//!   16 B sound pair to `0x78..0x88` and a 32 B volume to `0x88..0xa8`, and
//!   sets `u16 @0xa8 = 0x101`.
//! - `occlude_by_world` @0x1409a8ee0 → byte **@0xa8** (bit0).
//! - `no_history_recording` @0x1409a8f10 → clear bit0 of byte **@0xa9**;
//!   `allow_negative_distance` @0x1409a8f40 → bit2 of **@0xa9**.
//! - `WidgetVolumeShape::clearing_canvas` @0x1409a9110 → 32 B:
//!   `{ tag u8 @0 (=4), layer u8 @1, size Vec2 @0x10 = (130,130)
//!   (=0x43020000,0x43020000) }`.
//! - `UiSounds` = 4 sound slots × 24 B (96 B); the builder setters map
//!   `on_hover_start`→slots 0-2, `on_drag_start`→3-5, `on_drag_release`→6-8,
//!   `on_click`→9-11 (each slot 3 qwords); `empty` @0x1409a8fb0 zeroes the
//!   first u32 of each slot (0,0x18,0x30,0x48).
//! - `WidgetParams::signifier` @0x1422bc980 → `into_signifier_factory`.

#![allow(dead_code, unused_imports)]

use crate::Gap;
use crate::systems::ui::signifiers::SignifierFactory;

/// `WorldCanvasLayer` (O named entity): world-canvas layer id.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct WorldCanvasLayer(pub u32);

/// One 24 B sound slot inside `UiSounds` (O: 3 qwords; layout U).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UiSoundSlot(pub Gap);

/// `UiSounds` (O: 4 × 24 B slots). Setter slot mapping is O (see header).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UiSounds {
    pub hover_start: UiSoundSlot,
    pub drag_start: UiSoundSlot,
    pub drag_release: UiSoundSlot,
    pub click: UiSoundSlot,
}

impl UiSounds {
    /// `UiSounds::empty` @0x1409a8fb0 (O; folds with `boop`).
    pub fn empty() -> Self {
        UiSounds::default()
    }
    /// `UiSounds::boop` (O).
    pub fn boop(&self) -> Self {
        *self
    }
    /// `on_hover_start` @0x1409a8fd0 (O: slots 0-2).
    pub fn on_hover_start(&self, _sound: &UiSoundSlot) -> Self {
        *self
    }
    /// `on_click` @0x1409a9020 (O: slots 9-11).
    pub fn on_click(&self, _sound: &UiSoundSlot) -> Self {
        *self
    }
    /// `on_drag_start` @0x1409a9070 (O: slots 3-5).
    pub fn on_drag_start(&self, _sound: &UiSoundSlot) -> Self {
        *self
    }
    /// `on_drag_release` @0x1409a90c0 (O: slots 6-8).
    pub fn on_drag_release(&self, _sound: &UiSoundSlot) -> Self {
        *self
    }
}

/// `WidgetVolumeShape` (O: 32 B `{tag, layer, size@0x10}`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WidgetVolumeShape {
    /// tag @0 (O; 4 = clearing-canvas).
    pub tag: u8,
    /// layer @1 (O).
    pub layer: u8,
    /// size @0x10 (O; clearing canvas = 130×130).
    pub size: glam::Vec2,
}

impl Default for WidgetVolumeShape {
    fn default() -> Self {
        WidgetVolumeShape {
            tag: 0,
            layer: 0,
            size: glam::Vec2::ZERO,
        }
    }
}

impl WidgetVolumeShape {
    /// `clearing_canvas` @0x1409a9110 (O: 130×130 clearing volume).
    pub fn clearing_canvas(layer: WorldCanvasLayer) -> Self {
        WidgetVolumeShape {
            tag: 4,
            layer: layer.0 as u8,
            size: glam::Vec2::splat(130.0),
        }
    }
    /// `get_origin` (O inline into `signifier_animation`).
    pub fn get_origin(&self) -> glam::Vec3 {
        glam::Vec3::new(self.size.x, 0.0, self.size.y)
    }
}

/// `UiFocus` (O named entity).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum UiFocus {
    #[default]
    None,
    Toolbar,
    Mei,
    World,
}

impl UiFocus {
    /// `as_mei` @0x1409a9130 (O).
    pub fn as_mei(self) -> Self {
        UiFocus::Mei
    }
    /// `is_mei` @0x1409a9140 (O).
    pub fn is_mei(self) -> bool {
        matches!(self, UiFocus::Mei)
    }
    /// `is_toolbar` @0x1409a9150 (O).
    pub fn is_toolbar(self) -> bool {
        matches!(self, UiFocus::Toolbar)
    }
    /// `is_none` (O).
    pub fn is_none(self) -> bool {
        matches!(self, UiFocus::None)
    }
}

/// `WidgetFlags` (O named entity; bitmask; flag constants O names:
/// `ALLOW_NEGATIVE_DISTANCE`, `DECORATOR`, `HIDE_OS_CURSOR`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WidgetFlags(pub u32);

impl WidgetFlags {
    pub const ALLOW_NEGATIVE_DISTANCE: u32 = 1 << 2;
    pub const DECORATOR: u32 = 1 << 3;
    pub const HIDE_OS_CURSOR: u32 = 1 << 4;
}

/// `WidgetParams` (O: 0xb0 B resource stored per `WidgetId`).
#[derive(Clone, Debug, PartialEq)]
pub struct WidgetParams {
    /// `UiSounds` region @0x18..0x78 (O: `sounds` setter).
    pub sounds: UiSounds,
    /// volume @0x78..0xa8 (O: 32 B + 16 B region).
    pub volume: WidgetVolumeShape,
    /// occlude_by_world bit @0xa8 (O).
    pub occlude_by_world: bool,
    /// flags byte @0xa9: bit0 record_history_when_used, bit2
    /// allow_negative_distance (O).
    pub record_history_when_used: bool,
    pub allow_negative_distance: bool,
    /// typed gap for the remaining layout.
    pub _gap: Gap,
}

impl Default for WidgetParams {
    fn default() -> Self {
        WidgetParams::new(WidgetVolumeShape::default(), UiSounds::empty())
    }
}

impl WidgetParams {
    /// `WidgetParams::new` @0x1409a8e10 (O; builder defaults).
    pub fn new(volume: WidgetVolumeShape, sounds: UiSounds) -> Self {
        WidgetParams {
            sounds,
            volume,
            occlude_by_world: true,
            record_history_when_used: true,
            allow_negative_distance: false,
            _gap: Gap,
        }
    }
    /// `with_flags` (O inline into `ui_focus_stairs`).
    pub fn with_flags(mut self, _flags: WidgetFlags) -> Self {
        self
    }
    /// `sounds` @0x1409a8e80 (O builder).
    pub fn sounds(mut self, sounds: UiSounds) -> Self {
        self.sounds = sounds;
        self
    }
    /// `occlude_by_world` @0x1409a8ee0 (O builder).
    pub fn occlude_by_world(mut self, occlude: bool) -> Self {
        self.occlude_by_world = occlude;
        self
    }
    /// `no_history_recording` @0x1409a8f10 (O builder).
    pub fn no_history_recording(mut self) -> Self {
        self.record_history_when_used = false;
        self
    }
    /// `allow_negative_distance` @0x1409a8f40 (O builder).
    pub fn allow_negative_distance(mut self, allow: bool) -> Self {
        self.allow_negative_distance = allow;
        self
    }
    /// `signifier` @0x1422bc980 → `into_signifier_factory` (O).
    pub fn into_signifier_factory(&self) -> SignifierFactory {
        SignifierFactory::default()
    }
}

/// `UiNavState` (O named entity; resource with a 1e9-ns timeout, O in `add_ui`).
#[derive(Clone, Debug, Default)]
pub struct UiNavState {
    /// 1_000_000_000 ns timeout (O).
    pub nav_timeout_ns: u64,
    pub _gap: Gap,
}

/// `PlayUiSound` (O event; registered in `add_ui`).
#[derive(Clone, Copy, Debug, Default)]
pub struct PlayUiSound(pub UiSoundSlot);

/// `PlayPhotomodeSound` (O event; registered in `add_ui`).
#[derive(Clone, Copy, Debug, Default)]
pub struct PlayPhotomodeSound(pub UiSoundSlot);
