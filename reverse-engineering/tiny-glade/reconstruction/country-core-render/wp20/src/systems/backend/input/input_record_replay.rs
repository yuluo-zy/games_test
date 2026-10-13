//! `backend/input/input_record_replay.rs` — record/replay frame driver.
//!
//! PDB: `input_record_replay` @0x1409423d0, `advance_input_replay_frame`
//! @0x140942280, `control_input_replay` @0x140941a30. Named entity
//! `InputReplayCommand`.
//!
//! O: mode is a u64 tag stored at `InputRecordReplayState @+0x40`; the switch
//! `mode ^ 0x8000000000000000` yields Idle=`0x8000000000000000`,
//! Replaying=`0x8000000000000002`, Recording=`0x8000000000000003`,
//! Finished=`0x8000000000000004`. `InputFrame` stride 0xd0.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::input::input_stream::*;
use crate::systems::backend::input::InputReplayCommand;
use crate::Gap;

/// `advance_input_replay_frame` @0x140942280 (O): bump the replay index; when
/// the index passes the recorded frame count, mark the replay Finished.
pub fn advance_input_replay_frame(mut _state: ResMut<InputRecordReplayState>) {
    // if mode == Replaying { index += 1; if frames.len() <= index { mode = Finished } }
    todo!("recover advance_input_replay_frame (index@+0x48, frames@+0x38, Finished tag)")
}

/// `input_record_replay` @0x1409423d0 (O): per-mode state machine — start
/// recording (store camera params, rebuild InputStream with one sync point,
/// add first frame), apply the current recorded frame during replay, reset the
/// translation state while recording, no-op when finished.
pub fn input_record_replay(
    mut _state: ResMut<InputRecordReplayState>,
    _translation: ResMut<Gap>,
    _recording_conditions: Res<Gap>,
    _outputs: (),
) {
    todo!("recover input_record_replay mode switch (Idle/Replaying/Recording/Finished)")
}

/// `control_input_replay` @0x140941a30 (O): keyboard / debug-command entry to
/// start/stop recording and start/stop replay.
pub fn control_input_replay(
    mut _state: ResMut<InputRecordReplayState>,
    _commands: Commands,
) {
    todo!("recover control_input_replay keyboard gates")
}
