//! `systems/timelapse_replay.rs` — timelapse record/replay state machine.
//!
//! PDB: `TimelapseState::is_active @0x14092e6b0`, `start_timelapse
//! @0x140a8a310`, `finish_timelapse @0x140a8a460`, `cleanup_timelapse
//! @0x140a8a5d0`. Named entities: `TimelapseState`, `StartTimelapseReplayCmd`,
//! `InterruptTimelapse`, `TimelapseReplayFinished`,
//! `ReplaySpeedMultiplierOverride`.
//!
//! O: byte state 0=idle / 1=running / 2=finished. `start_timelapse` trims the
//! input stream and, when un-consumed frames remain, requests the mode switch
//! (`mode_manager::check_if_can_switch`); `finish_timelapse` pushes a
//! `MetaCommand`, sets state=1, and when the command queue empties emits the
//! finished event; `cleanup_timelapse` sets state=2 when frames remain.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::meta_commander::MetaCommand;
use crate::Gap;

/// `TimelapseState` (O named entity; byte state 0/1/2).
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TimelapseState(pub u8);

impl TimelapseState {
    /// `is_active` @0x14092e6b0 (O): `self.0 == 1`.
    pub fn is_active(self) -> bool {
        self.0 == 1
    }
}

/// `StartTimelapseReplayCmd` (O named entity).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct StartTimelapseReplayCmd {
    pub _gap: Gap,
}

/// `InterruptTimelapse` (O named entity).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct InterruptTimelapse {
    pub _gap: Gap,
}

/// `TimelapseReplayFinished` (O named entity).
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct TimelapseReplayFinished {
    pub _gap: Gap,
}

/// `ReplaySpeedMultiplierOverride` (O named entity).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct ReplaySpeedMultiplierOverride {
    pub _gap: Gap,
}

/// `start_timelapse` @0x140a8a310 (O).
pub fn start_timelapse(
    mut _state: ResMut<TimelapseState>,
    mut _meta: ResMut<Gap>,
    _commands: Commands,
) {
    todo!("recover start_timelapse (stream trim + mode switch request)")
}

/// `finish_timelapse` @0x140a8a460 (O).
pub fn finish_timelapse(
    mut _state: ResMut<TimelapseState>,
    mut _meta: ResMut<Gap>,
    mut _finished: EventWriter<TimelapseReplayFinished>,
) {
    todo!("recover finish_timelapse (MetaCommand push + finished event)")
}

/// `cleanup_timelapse` @0x140a8a5d0 (O).
pub fn cleanup_timelapse(mut _state: ResMut<TimelapseState>) {
    todo!("recover cleanup_timelapse (state = 2 on remaining frames)")
}
