//! `fsr_plugin` — bevy plugin wiring FidelityFX Super Resolution (WP4-owned).
//!
//! Skeleton bundle: `skeleton/B/crates/country-core/src/render/fsr_plugin.rs`
//! (source_sha256 `50fdaace…`, 1 proc, module 480). No standalone business
//! procedure recovered (all logic inlined / generic); per
//! `evidence/render-passes/INDEX.md` the FSR compiled products are
//! history/reactive compute passes only (`_fsr_copy_history`,
//! `_fsr_init_reactive`). This module is retained as a typed gap with the
//! observed entry surface.

/// FSR reactive/history pass state (U — typed gap).
pub struct FsrPluginState {
    _gap: crate::typed_gap::Gap,
}

/// Bevy plugin marker for the FSR wiring (O name; plugin body U).
pub struct FsrPlugin;

impl FsrPlugin {
    pub fn new() -> Self {
        FsrPlugin
    }
}

impl Default for FsrPlugin {
    fn default() -> Self {
        Self::new()
    }
}
