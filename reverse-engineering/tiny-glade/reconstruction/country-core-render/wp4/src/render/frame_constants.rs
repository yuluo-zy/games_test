//! `FrameConstants` — per-frame GPU constant block (WP4-owned).
//!
//! Written to the `frame_constants` UBO (set1/b0) each frame, see
//! `evidence/render-passes/INDEX.md` descriptor conventions.
//!
//! Skeleton bundle: `skeleton/B/crates/country-core/src/render/frame_constants.rs`
//! (source_sha256 `abf41da9…`, 1 proc). Decompile `A_0x140976880.c`.

use crate::render::view_constants::ViewConstants;

/// Render-view slot index inside `FrameConstants`.
///
/// `is_shadow_view` / `sun_cascade` (O names; semantics I).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderViewIndex(pub u32);

impl RenderViewIndex {
    pub const MAIN_VIEW: RenderViewIndex = RenderViewIndex(0);
    pub const SUN_CASCADE_0: RenderViewIndex = RenderViewIndex(1);

    /// `is_shadow_view` — true for shadow/sun-cascade views (I).
    pub fn is_shadow_view(self) -> bool {
        self.0 != Self::MAIN_VIEW.0
    }

    /// `sun_cascade` — cascade index of a shadow view (I; -1 for main view).
    pub fn sun_cascade(self) -> i32 {
        if self.is_shadow_view() {
            (self.0 - 1) as i32
        } else {
            -1
        }
    }
}

/// Per-frame constants uploaded to the `frame_constants` UBO.
///
/// Members observed from PDB names: `main_view`, `sun_view`,
/// `should_reset_temporal`, `RenderViewIndex`. Full UBO field list (light
/// direction, time, frame index, …) is a typed gap informed by shader
/// reflection conventions.
#[derive(Debug, Clone)]
pub struct FrameConstants {
    /// Main camera view (O name).
    pub main_view: ViewConstants,
    /// Sun light shadow-cascade views (O name).
    pub sun_view: ViewConstants,
    /// Reset temporal accumulation this frame (O: getter 0x140976880; e.g. on
    /// camera teleport / material change).
    pub should_reset_temporal: bool,
    /// U: additional UBO fields (frame index, timestamps, …).
    _gap: crate::typed_gap::Gap,
}

impl FrameConstants {
    /// `main_view` accessor (O name).
    pub fn main_view(&self) -> &ViewConstants {
        &self.main_view
    }

    /// `sun_view` accessor (O name).
    pub fn sun_view(&self) -> &ViewConstants {
        &self.sun_view
    }

    /// `should_reset_temporal` getter (O: 0x140976880, 11B trivial read).
    pub fn should_reset_temporal(&self) -> bool {
        self.should_reset_temporal
    }

    /// `TREE_SEE_THROUGH` — display/format constant for the tree-see-through
    /// toggle (O: `impl$15::fmt` string; value U).
    pub const TREE_SEE_THROUGH: &'static str = "TREE_SEE_THROUGH";
}
