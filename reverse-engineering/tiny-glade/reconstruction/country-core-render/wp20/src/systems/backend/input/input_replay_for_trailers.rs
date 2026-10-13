//! `backend/input/input_replay_for_trailers.rs` — trailer-video input
//! record/replay (keyboard-driven) + egui debug panel.
//!
//! PDB: `record_and_replay_input_for_trailers` @0x14083c620,
//! `do_input_replay_ui` @0x14083cdd0.
//!
//! O: reads an enable byte @+0xfc of a config resource; on a key hit against
//! a `HashMap<KeyCode,()>` binding set, switches `InputRecordReplayState.mode`:
//!   - Idle/Finished → "Start recording": `MainCamera::store_parameters()`,
//!     rebuild InputStream with one sync point, mode = Idle tag.
//!   - Recording (default) → "Finished recording": mode = Finished tag.
//!   - Replaying key → "Replay...": mode = Replaying tag, index = -1, seek last
//!     sync point (frame_index-1), restore `InputTranslationState`,
//!     `MainCamera::load_parameters()` + `immediately_finish_interpolating()`.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::input::input_stream::*;
use crate::reused::egui;
use crate::Gap;

/// `record_and_replay_input_for_trailers` @0x14083c620 (O).
pub fn record_and_replay_input_for_trailers(
    mut _state: ResMut<InputRecordReplayState>,
    _config: Res<Gap>,
    _key_bindings: Res<Gap>,
    _commands: Commands,
) {
    todo!("recover record/replay keyboard gates + InputStream rebuild + camera param store/load")
}

/// `do_input_replay_ui` @0x14083cdd0 (O): egui panel — "Enable input
/// recording" checkbox, "Stream frame count: {}" label, "`O` - start/pause
/// recording", "`E` - replay", "Replay sheep" button.
pub fn do_input_replay_ui(
    mut _ui: egui::Ui,
    _state: ResMut<InputRecordReplayState>,
    _config: ResMut<Gap>,
) {
    todo!("recover egui input-record-replay panel")
}
