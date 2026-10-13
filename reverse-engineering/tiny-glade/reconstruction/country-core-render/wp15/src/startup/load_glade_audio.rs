//! `startup/load_glade_audio.rs` — per-glade ambience audio.
//!
//! Source: B skeleton `crates/country-core/src/startup/load_glade_audio.rs`.
//! PDB named entity: `GladeAudio`.
//! O evidence (Ghidra, B patched EXE):
//! - `GladeAudio::load` @0x1408bf460 → `GladeAudio::try_load` (inline).
//! - `stop_and_play_new` (inline in `load_glade_audio`).
//! - `load_glade_audio` @0x1408be360 (2 callers) — switches the glade
//!   ambience audio loops (via `AudioLoop`/`AudioCore::instantiate_loop`).

#![allow(dead_code, unused_imports)]

use crate::Gap;

/// `GladeAudio` (O named entity; per-glade audio bundle, layout U).
#[derive(Clone, Debug, Default)]
pub struct GladeAudio(pub Gap);

impl GladeAudio {
    /// `GladeAudio::load` @0x1408bf460 (O).
    pub fn load(_path: &str) -> anyhow::Result<GladeAudio> {
        todo!("recover GladeAudio::load")
    }
    /// `GladeAudio::try_load` (O inline).
    pub fn try_load(_path: &str) -> anyhow::Result<GladeAudio> {
        Self::load(_path)
    }
}

/// `stop_and_play_new` (O inline in `load_glade_audio`).
pub fn stop_and_play_new(_audio_core: &Gap, _audio: &GladeAudio) {
    todo!("recover stop_and_play_new")
}

/// `load_glade_audio` @0x1408be360 (O).
pub fn load_glade_audio(_session: &Gap, _audio_core: &Gap) {
    todo!("recover load_glade_audio")
}
