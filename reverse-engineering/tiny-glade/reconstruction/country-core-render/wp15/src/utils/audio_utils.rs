//! `utils/audio_utils.rs` — `AudioLoop` / `AudioLoopDesc` (rotating looped
//! sound instances with cooldown-gated stop).
//!
//! Source: B skeleton `crates/country-core/src/utils/audio_utils.rs`.
//! PDB named entities: `AudioLoop`, `AudioLoopDesc`.
//!
//! O evidence (Ghidra, B patched EXE):
//! - `AudioCore::instantiate_loop` @0x140a59220: `AudioLoop` =
//!   `{ instances: Vec<AudioInstance> (stride 0x30), last-stop Instant @+0x18,
//!   +0x20 u32 (=1000000000 marker), current_index @+0x28, cooldown_ms @+0x30,
//!   is_playing bool @+0x34, has_parameter bool @+0x35, bool @+0x36 }`.
//!   Cooldown read from `AudioLoopDesc @+0x18`.
//! - `AudioLoop::current` @0x140a593f0 (panics OOB),
//!   `start_if_not_playing` @0x140a596b0 (fmod `EventInstance::start`),
//!   `stop` @0x140a59430 (cooldown-gated `EventInstance::stop` +
//!   advance `current_index` modulo instance count).

#![allow(dead_code, unused_imports)]

use crate::Gap;

/// `AudioLoopDesc` (O named entity; cooldown @+0x18).
#[derive(Clone, Copy, Debug, Default)]
pub struct AudioLoopDesc {
    /// cooldown ms @+0x18 (O).
    pub cooldown_ms: u32,
    pub _gap: Gap,
}

impl AudioLoopDesc {
    /// `AudioLoopDesc::new` (O inline into `water_audio_init_system`).
    pub fn new() -> Self {
        AudioLoopDesc::default()
    }
}

/// `AudioInstance` (O: fmod EventInstance wrapper, stride 0x30; state int @0
/// `== 1` = not-started in the decompile; layout U).
#[derive(Clone, Debug, Default)]
pub struct AudioInstance(pub Gap);

/// `AudioLoop` (O named entity; rotating ambience loop).
#[derive(Clone, Debug, Default)]
pub struct AudioLoop {
    pub instances: Vec<AudioInstance>,
    /// current instance index (O @+0x28).
    pub current_index: usize,
    /// cooldown ms (O @+0x30).
    pub cooldown_ms: u32,
    /// is_playing (O @+0x34).
    pub is_playing: bool,
    /// has a set-parameter (O @+0x35).
    pub has_parameter: bool,
    pub _gap: Gap,
}

impl AudioLoop {
    /// `current` @0x140a593f0 (O; panics out-of-bounds).
    pub fn current(&self) -> &AudioInstance {
        &self.instances[self.current_index]
    }
    /// `is_playing` (O inline into `terrain_stairs_audio_system`).
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }
    /// `start_if_not_playing` @0x140a596b0 (O).
    pub fn start_if_not_playing(&mut self) {
        todo!("recover AudioLoop::start_if_not_playing (EventInstance::start)")
    }
    /// `stop` @0x140a59430 (O; cooldown-gated, advances index).
    pub fn stop(&mut self, _now: std::time::Instant) {
        todo!("recover AudioLoop::stop")
    }
}
