//! `systems/time_update.rs` — the game-clock system.
//!
//! PDB: `time_update @0x140922c30` (886 B).
//!
//! O: advances a 56 B game clock
//! `{Duration secs+nanos, total_f32@+0x10, total_60ths_u64@+0x18,
//! frac_seconds@+0x20, flag@+0x30, frame_count@+0x2c}`. During input replay
//! (`InputRecordReplayState.mode == Replaying`) the clock delta comes from the
//! recorded frame's game-delta field (@+0xc8 of the 0xd0 B frame); otherwise
//! it comes from `GameTime` (Duration @+0x10/+0x18). Frame deltas for the
//! first two frames are clamped to 1/60 s.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::input::input_stream::InputRecordReplayState;
use crate::resources::time::GameTime;
use crate::Gap;

/// `GameClock` (O: 56 B game-clock resource written by `time_update`).
#[derive(Resource, Clone, Debug, Default)]
pub struct GameClock {
    pub raw: Gap,
}

/// `time_update` @0x140922c30 (O).
pub fn time_update(
    _time: Res<GameTime>,
    _replay: Res<InputRecordReplayState>,
    mut _clock: ResMut<GameClock>,
    _flags: Res<Gap>,
) {
    todo!("recover time_update (clock advance + replay-frame delta + 1/60 clamp)")
}
