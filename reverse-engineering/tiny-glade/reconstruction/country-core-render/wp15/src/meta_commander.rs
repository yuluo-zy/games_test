//! `meta_commander.rs` — MetaCommander / MetaCommand / MetaCommandFence /
//! ProfilerSink / MemoryLog (queued "meta commands" processed each frame).
//!
//! Source: B skeleton `crates/country-core/src/meta_commander.rs`.
//! PDB named entities: `MetaCommand`, `MetaCommandFence`, `MetaCommander`,
//! `ProfilerSink`, `commander_update::MemoryLog`.
//!
//! O evidence (Ghidra, B patched EXE):
//! - `MetaCommand` element stride **0x38 (56 B)** in a VecDeque; empty sentinel
//!   `0x8000000000000014` (`consume` @0x14091f100).
//! - `MetaCommander::new` @0x14091eff0: `{ commands: VecDeque<MetaCommand>
//!   (4 qwords), profiler_sinks: HashMap<String,(FrameSinkId,
//!   Arc<Mutex<ProfilerSink>>)> (4 qwords) + RandomState (2 qwords) }` (80 B).
//! - `MetaCommandFence::new_pair_blocked` @0x14091ef90 — Arc<Mutex<(bool,bool)>>
//!   with both flags set (blocked); `is_blocked`/`unblock`; a fence is attached
//!   to `resources::session::NextSession::append_on_load` so a session-load
//!   command stays blocked until the load completes.
//! - `commander_update` @0x14091f180 — dispatches on the MetaCommand tag:
//!     tag 0  session load (`NextSession` + fence + `is_session_ok_to_load`),
//!     tag 1  re-queue (no-op frame),
//!     tag 2  `insert_resource<TimeUpdateStrategy>` (gated on RT BVH idle),
//!     tag 3  `insert_resource<InputRecordReplayState>`,
//!     tag 6  manual time (from_secs_f32) → `TimeUpdateStrategy`,
//!     tag 7  `insert_resource<InputRecordReplayState>` (default),
//!     tag 8  push a `LastKnownGraphicsDevice` into the device list,
//!     tag 9  create save directories,
//!     tag 0xc  start profiling (`puffin::set_scopes_on` + add sink),
//!     tag 0xd+ stop profiling / write profile file (`FrameView::save_to_writer`),
//!     tag 0xe  write GPU performance info,
//!     tag 0xf  write memory log (`MemoryLog` serialized to JSON),
//!     tag 0x10  despawn entities,
//!     tag 0x11  `WriteSettingsFileCmd`,
//!     tag 0x12  `insert_resource<ShutdownGame>`,
//!     tag 0x13  `std::process::exit()`.
//!
//! Variant *names* are U (PDB does not expose them); the recovered tags +
//! operations are recorded below as inferred (`I`) labels.

#![allow(dead_code, unused_imports)]

use std::sync::Arc;

use crate::Gap;

/// `MetaCommandFence` (O named entity). A pair of blocked flags
/// (`Arc<Mutex<(bool,bool)>>`, both true when created blocked).
#[derive(Clone, Debug, Default)]
pub struct MetaCommandFence {
    /// first blocked flag (I).
    pub first: Arc<Gap>,
    /// second blocked flag (I).
    pub second: Arc<Gap>,
}

impl MetaCommandFence {
    /// `MetaCommandFence::new_pair_blocked` @0x14091ef90 (O: both flags set).
    pub fn new_pair_blocked() -> Self {
        MetaCommandFence {
            first: Arc::new(Gap),
            second: Arc::new(Gap),
        }
    }
    /// `MetaCommandFence::is_blocked` (O inline in `commander_update`).
    pub fn is_blocked(&self) -> bool {
        false
    }
    /// `MetaCommandFence::unblock` (O inline in `load_game`).
    pub fn unblock(&self) {}
}

/// `MetaCommand` (O named entity; 0x38 B element, tags 0x0..0x13; `0x8000..0014`
/// = empty sentinel). Variant names U → inferred labels from the
/// `commander_update` dispatch.
#[derive(Clone, Debug, Default)]
pub enum MetaCommand {
    /// tag 0: load the next session, gated on a `MetaCommandFence` (I).
    #[default]
    LoadNextSession,
    /// tag 1: per-frame no-op re-queue (I).
    Tick,
    /// tag 2: set time update strategy auto (I).
    SetTimeUpdateStrategy,
    /// tag 3: enable input record/replay state (I).
    EnableInputReplay,
    /// tag 4: (I; unobserved op).
    Variant4,
    /// tag 5: (I; unobserved op).
    Variant5,
    /// tag 6: manual time from `Duration` (from_secs_f32) (I).
    SetManualTime,
    /// tag 7: reset input record/replay state to default (I).
    ResetInputReplay,
    /// tag 8: push a `LastKnownGraphicsDevice` (I).
    AddGraphicsDevice,
    /// tag 9: create save directories (I).
    CreateSaveDirs,
    /// tag 0xa / 0xb: toggle a bool at +0x31 (I; e.g. photo mode flags).
    Variant10,
    Variant11,
    /// tag 0xc: start profiling (puffin sink) (I).
    StartProfiling,
    /// tag 0xd: stop profiling / write profile file (I).
    StopProfiling,
    /// tag 0xe: write GPU performance info (I).
    WriteGpuInfo,
    /// tag 0xf: write memory log (I).
    WriteMemoryInfo,
    /// tag 0x10: despawn entities (I).
    DespawnEntities,
    /// tag 0x11: emit `WriteSettingsFileCmd` (I).
    WriteSettings,
    /// tag 0x12: insert `ShutdownGame` resource (I).
    ShutdownGame,
    /// tag 0x13: `std::process::exit()` (I).
    ExitProcess,
}

/// `ProfilerSink` (O named entity): puffin frame sink wrapper.
#[derive(Clone, Debug, Default)]
pub struct ProfilerSink {
    pub _gap: Gap,
}

/// `commander_update::MemoryLog` (O named entity; serialized JSON). Layout U.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct MemoryLog {
    pub _gap: Gap,
}

/// `MetaCommander` (O named entity; 80 B resource).
#[derive(Clone, Debug, Default)]
pub struct MetaCommander {
    /// Queued `MetaCommand`s (VecDeque, O).
    pub commands: std::collections::VecDeque<MetaCommand>,
    /// Profiler sinks keyed by name (HashMap, O).
    pub profiler_sinks: std::collections::HashMap<String, Arc<ProfilerSink>>,
    /// Profiling enabled flag (I; `is_profiling`).
    pub profiling: bool,
}

impl MetaCommander {
    /// `MetaCommander::new` @0x14091eff0 (O: empty command queue + empty sinks).
    pub fn new() -> Self {
        MetaCommander {
            commands: std::collections::VecDeque::new(),
            profiler_sinks: std::collections::HashMap::new(),
            profiling: false,
        }
    }

    /// `MetaCommander::peek` @0x14091f0c0 (O: front of queue).
    pub fn peek(&self) -> Option<&MetaCommand> {
        self.commands.front()
    }

    /// `MetaCommander::consume` @0x14091f100 (O: pop front; sentinel when empty).
    pub fn consume(&mut self) -> Option<MetaCommand> {
        self.commands.pop_front()
    }

    /// `MetaCommander::undo_consume` (O inline in `commander_update`):
    /// push a command back to the front.
    pub fn undo_consume(&mut self, cmd: MetaCommand) {
        self.commands.push_front(cmd);
    }

    /// `MetaCommander::is_profiling` @0x14091f0f0 (O).
    pub fn is_profiling(&self) -> bool {
        self.profiling
    }

    /// `MetaCommander::is_empty` (O inline in `finish_timelapse`).
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    /// `MetaCommander::extend_script` (O inline in `finish_timelapse`).
    pub fn extend_script(&mut self, _commands: Vec<MetaCommand>) {
        self.commands.extend(_commands);
    }
}

/// `commander_update` @0x14091f180 (O): process one queued `MetaCommand`.
/// Per-tag operations documented above; the exact GPU/IO side effects are
/// recovered surfaces only (reused rhapsody/puffin/std are not re-implemented).
pub fn commander_update(commander: &mut MetaCommander) {
    let Some(_cmd) = commander.consume() else {
        return;
    };
    // Full per-variant handling: see contract (ui/app-shell). Side-effect wiring
    // (session load, profiling, gpu/memory logs, quit) is a typed-gap boundary.
    todo!("recover commander_update per-variant side effects")
}
