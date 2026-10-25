//! `input/input_stream.rs` — `InputStream` / `InputFrame` /
//! `InputStreamSyncPoint` / `InputTranslationState` / `InputRecordReplayState` /
//! `InputRecordReplayMode` / `InputReplayInitialState` (bounded input recording
//! ring for trailer replay).
//!
//! Source: B skeleton `crates/country-core/src/input/input_stream.rs`.
//! PDB named entities: `InputFrame`, `InputRecordReplayMode`,
//! `InputRecordReplayState`, `InputReplayInitialState`, `InputStream`,
//! `InputStreamSyncPoint`, `InputTranslationState`.
//!
//! O evidence (Ghidra, B patched EXE):
//! - `InputStream` = 64 B: `{ sync_points: VecDeque<InputStreamSyncPoint> (4
//!   qwords), frames: VecDeque<InputFrame> (4 qwords) }`.
//!   `InputFrame` stride **0xd0 (208 B)**; `InputStreamSyncPoint` stride
//!   **0x60 (96 B)** (`add_frame`/`drop_oldest_sync_point`/`on_history_sync`).
//! - `add_frame` @0x140949190: when recording and frame_count >= 9999, trim:
//!   if fewer than 2 sync points and frame_count >= 50000 drop the oldest frame,
//!   else `drop_oldest_sync_point`; then push the 0xd0 B `InputFrame`.
//! - `on_history_sync` @0x1409494c0: when not recording truncate both queues;
//!   when sync_points >= 9 or frames >= 10000 drop the oldest sync point; then
//!   push a sync point snapshotting `InputTranslationState` (11 qwords) + frame
//!   count.
//! - `drop_oldest_sync_point` @0x140949660: pop front sync point, drop frames
//!   up to its frame index, re-base remaining sync-point frame offsets.
//! - `InputRecordReplayState::try_load` @0x1409499c0: `read_to_string` +
//!   `serde_json::from_str<InputStream>` → 80 B state
//!   `{ stream: InputStream, mode: u64 @0x40 (=0x8000000000000002 on load),
//!   index: i64 @0x48 (= -1) }`.
//! - `InputRecordReplayState::new` / `InputReplayInitialState::new` (O inline
//!   into `record_and_replay_input_for_trailers`).

#![allow(dead_code, unused_imports)]

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::Gap;

/// `InputTranslationState` (O named entity; 88 B snapshot in each sync point).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct InputTranslationState {
    pub _gap: Gap,
}

/// `InputFrame` (O named entity; 0xd0 B per-frame input record).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct InputFrame {
    pub _gap: Gap,
}

/// `InputStreamSyncPoint` (O named entity; 0x60 B: `InputTranslationState` +
/// frame count).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct InputStreamSyncPoint {
    pub translation: InputTranslationState,
    /// recorded frame index (O: written from the frames len).
    pub frame_index: u64,
}

/// `InputStream` (O named entity; 64 B input-recording ring).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct InputStream {
    /// Sync-point queue (O; `VecDeque<InputStreamSyncPoint>`).
    pub sync_points: VecDeque<InputStreamSyncPoint>,
    /// Recorded frames (O; `VecDeque<InputFrame>`).
    pub frames: VecDeque<InputFrame>,
}

impl InputStream {
    /// `InputStream::new` (O inline in `record_and_replay_input_for_trailers`).
    pub fn new() -> Self {
        InputStream {
            sync_points: VecDeque::new(),
            frames: VecDeque::new(),
        }
    }

    /// `sync_point_count` @0x140949170 (O).
    pub fn sync_point_count(&self) -> usize {
        self.sync_points.len()
    }

    /// `frame_count` @0x140949180 (O).
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// `add_frame` @0x140949190 (O): record one frame with bounded retention
    /// (9999/50000-frame trim + sync-point trim).
    pub fn add_frame(&mut self, _frame: InputFrame, _record: bool) {
        todo!("recover add_frame trim + push (0xd0 B InputFrame)")
    }

    /// `last_frame_mut` @0x140949480 (O).
    pub fn last_frame_mut(&mut self) -> Option<&mut InputFrame> {
        self.frames.back_mut()
    }

    /// `on_history_sync` @0x1409494c0 (O): push a sync point snapshot.
    pub fn on_history_sync(
        &mut self,
        _translation: &InputTranslationState,
        _record: bool,
    ) {
        todo!("recover on_history_sync (9/10000 trim + 0x60 B sync point)")
    }

    /// `drop_oldest_sync_point` @0x140949660 (O).
    pub fn drop_oldest_sync_point(&mut self) {
        todo!("recover drop_oldest_sync_point frame pruning + re-base")
    }

    /// `sync_point` (O inline in `record_and_replay_input_for_trailers`).
    pub fn sync_point(&self, _i: usize) -> Option<&InputStreamSyncPoint> {
        self.sync_points.get(_i)
    }

    /// `seek_to_sync_point` (O inline).
    pub fn seek_to_sync_point(&mut self, _i: usize) {
        todo!("recover seek_to_sync_point")
    }
}

/// `InputRecordReplayMode` (O named entity; mode u64 tags include
/// `0x8000000000000002` = loaded-from-file, O in `try_load`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum InputRecordReplayMode {
    #[default]
    Idle,
    /// Loaded from a recorded input file (O tag `0x8000000000000002`).
    Replaying,
}

/// `InputRecordReplayState` (O named entity; 80 B resource).
#[derive(Clone, Debug, Default)]
pub struct InputRecordReplayState {
    /// The recorded input stream (O).
    pub stream: InputStream,
    /// Replay mode (O @+0x40).
    pub mode: InputRecordReplayMode,
    /// Current replay frame index (O @+0x48; -1 initially).
    pub index: i64,
}

impl InputRecordReplayState {
    /// `InputRecordReplayState::new` (O inline).
    pub fn new() -> Self {
        InputRecordReplayState {
            stream: InputStream::new(),
            mode: InputRecordReplayMode::Idle,
            index: -1,
        }
    }
    /// `try_load` @0x1409499c0 (O): parse `InputStream` from a JSON file.
    pub fn try_load(&mut self, _json: &str) -> anyhow::Result<()> {
        todo!("recover try_load serde_json parse")
    }
    /// `seek_to_sync_point` (O inline).
    pub fn seek_to_sync_point(&mut self, _i: usize) {
        todo!("recover seek_to_sync_point")
    }
}

/// `InputReplayInitialState` (O named entity; initial replay options).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct InputReplayInitialState {
    pub _gap: Gap,
}

impl InputReplayInitialState {
    /// `InputReplayInitialState::new` (O inline).
    pub fn new() -> Self {
        InputReplayInitialState { _gap: Gap }
    }
}
