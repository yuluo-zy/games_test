//! B `roof_audio.rs`: roof edit sounds — 3 loops (RoofShape / RoofTipMove /
//! InterRoofMerlonsRearrange) plus the two one-shot helpers shared with the
//! wall audio systems (`instantiate_audio_w_position[_and_volume]`).
//!
//! O-level facts (chapter roof_audio.rs.md, audio-misc DIGEST §2.13):
//! - `roof_audio_init_system` 0x140aa2fd0: 3 × `instantiate_loop`:
//!   `event:/SFX/RoofShape`, `event:/SFX/RoofTipMove` (0x142adcb38),
//!   `event:/SFX/InterRoofMerlonsRearrange` (0x142adcb68).
//! - `instantiate_audio_w_position` 0x140aa3360 (4-arg `(core, name&str, pos)`):
//!   instantiate_audio (0x140a54440) → set_position (0x140a57da0) →
//!   start_if_not_playing (0x140a57300).
//! - `instantiate_audio_w_position_and_volume` 0x140aa3590: additionally
//!   `EventInstance::set_volume` when the instance is Active.
//! - `roof_audio_system` 0x140aa3860: drives the 3 loops' should-play /
//!   position / parameters plus one-shots per roof edit event.
//!
//! REBUILD-2026-10-17 (fresh decomp-0x140aa3860 + jump table 0x142adcef8): the
//! system accumulates the `+0x58` editing speed into acc[0]/acc[1]
//! (state+0xa8/+0xac), dispatches the 0x28-stride `RoofEditSound` stream
//! (kind@+8, selector byte@+9, position@+0x10), and frame-end drives the three
//! loops. The per-loop parameter names/values are the verified surface where
//! known, the exact FMOD parameter assignments 待对账.

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, Studio},
    systems::audio::audio_update::EditingSpeed,
    utils::audio_utils::AudioLoop,
};
use bevy_ecs::prelude::*;
use glam::Vec3;

/// The three roof edit loops (O: rne-read).
pub const ROOF_LOOP_EVENTS: [&str; 3] = [
    "event:/SFX/RoofShape",
    "event:/SFX/RoofTipMove",
    "event:/SFX/InterRoofMerlonsRearrange",
];

/// Roof edit one-shots (O: decomp-0x140aa3860 rne-read, lengths 21/21/24/26/
/// 24/22/25/30). `roof_audio_system` dispatches the roof edit event kinds:
/// 0→RoofAppear, 1→RoofRemove, 4→RoofTilesLand, 5→RoofTilesDetach, 6→MerlonsAppear,
/// 7→RoofSwitchDirection, 0xa→GableAppear, 0xb→GableDisappear.
pub const ROOF_ONE_SHOT_EVENTS: [&str; 8] = [
    "event:/SFX/RoofAppear",
    "event:/SFX/RoofRemove",
    "event:/SFX/RoofTilesLand",
    "event:/SFX/RoofTilesDetach",
    "event:/SFX/MerlonsAppear",
    "event:/SFX/GableAppear",
    "event:/SFX/GableDisappear",
    "event:/SFX/RoofSwitchDirection",
];

/// Roof loop parameters (O: rne-read; the "ridge"-related controls —
/// RoofHeight/RoofProfile/RoofTipMoveSpeed/RoofTipOffset — are set with the
/// f32 constants below).
pub const ROOF_PARAMS: [&str; 8] = [
    "editing_speed",
    "is_gable",
    "RoofHeight",
    "RoofProfile",
    "RoofTipMoveSpeed",
    "RoofTipOffset",
    "ImpactStrength",
    "isHalfTimbered",
];

/// Roof inverse_lerp / scale bounds (O: decomp f32 constants):
/// `0.2` (0x1429cd068), `0.05` (0x142930d70), `5.0` (0x142925988).
pub const ROOF_RIDGE_BOUNDS: (f32, f32, f32) = (0.2, 0.05, 5.0);

/// O (decomp-0x140aa3860): the 0x28-stride roof edit event stream consumed by
/// `roof_audio_system`. Payload: kind u8 @+8, selector byte @+9,
/// value0 f32 @+0xc, value1 f32 @+0x10, value2 f32 @+0x14, position Vec3
/// @+0x18. The producer is the roof domain (待接线).
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct RoofEditSound {
    /// kind u8 @+8.
    pub kind: u8,
    /// selector byte @+9 (RoofShape loop selector / is_gable).
    pub selector: u8,
    /// f32 @+0xc.
    pub value0: f32,
    /// f32 @+0x10.
    pub value1: f32,
    /// f32 @+0x14 (RoofShape loop `editing_speed` source).
    pub value2: f32,
    pub position: Vec3,
}

/// `instantiate_audio_w_position` (0x140aa3360): instantiate → set_position →
/// start_if_not_playing.
pub fn instantiate_audio_w_position<S: Studio>(
    core: &AudioCore<S>,
    name: &str,
    position: [f32; 3],
) -> AudioInstance<S::Event> {
    let mut instance = core.instantiate_audio(name);
    let _ = instance.set_position(position);
    let _ = instance.start_if_not_playing();
    instance
}

/// `instantiate_audio_w_position_and_volume` (0x140aa3590): instantiate →
/// set_position → start_if_not_playing → set_volume (Active only). Used by the
/// wall one-shot kinds and elevation supports.
pub fn instantiate_audio_w_position_and_volume<S: Studio>(
    core: &AudioCore<S>,
    name: &str,
    position: [f32; 3],
    volume: f32,
) -> AudioInstance<S::Event> {
    let mut instance = core.instantiate_audio(name);
    let _ = instance.set_position(position);
    let _ = instance.start_if_not_playing();
    let _ = instance.set_volume(volume);
    instance
}

/// RoofAudioState (O: 3 ring loops + the acc[0]/acc[1] @+0xa8/+0xac speed accumulators).
#[derive(Resource)]
pub struct RoofAudioState<E: crate::resources::audio_resources::EventInstance> {
    pub loops: [AudioLoop<AudioInstance<E>>; 3],
    /// Accumulated edit-strength per loop (native `+0xa8/+0xac` speed sources).
    pub acc: [f32; 2],
}

impl<E: crate::resources::audio_resources::EventInstance> RoofAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        let loops = ROOF_LOOP_EVENTS.map(|name| {
            core.instantiate_loop(&crate::resources::audio_resources::AudioLoopDesc::new(name))
        });
        RoofAudioState { loops, acc: [0.0, 0.0] }
    }
}

/// `roof_audio_init_system` (0x140aa2fd0).
pub fn roof_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    let state = RoofAudioState::<S::Event>::new(&core);
    commands.insert_resource(state);
}

/// `roof_audio_system` (0x140aa3860): consume the roof edit stream, drive the
/// 3 loops and dispatch the one-shots.
///
/// O (decomp-0x140aa3860): acc[0]/acc[1] accumulate the +0x58 editing speed;
/// kind 0/1 play RoofAppear/RoofRemove when their accumulator exceeds 0.2 (and
/// reset it); kind 2 selects the RoofShape loop (selector, position) and sets
/// two global params; kind 3 arms the RoofTipMove loop; kind 9 arms the
/// InterRoofMerlonsRearrange loop; kinds 4/6 instantiate the parameterized
/// one-shots RoofTilesLand / MerlonsAppear; kinds 5/7/0xa/0xb the plain
/// one-shots. Frame-end drives the loops (RoofShape by the selector, stop when
/// it is 2).
#[allow(clippy::too_many_arguments)]
pub fn roof_audio_system<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<RoofAudioState<S::Event>>,
    mut events: EventReader<RoofEditSound>,
    core: Res<AudioCore<S>>,
    speed: Option<Res<EditingSpeed>>,
) {
    // O: `+0x58` editing speed → acc[0]/acc[1] (state+0xa8/+0xac).
    let speed = speed.map_or(0.0, |s| s.0);
    state.acc[0] += speed;
    state.acc[1] += speed;
    // O: kind 2 stores value2 (+0x14) as the RoofShape loop's editing-speed
    // source (xmm6 in the native frame end).
    let mut shape_speed_source = 0.0f32;

    // Per-frame loop flags (native local_91/local_11c/local_130).
    let mut shape_selector = 2u8;
    let mut shape_pos = Vec3::ZERO;
    let mut tip_flag = false;
    let mut tip_pos = Vec3::ZERO;
    let mut merlons_flag = false;
    let mut merlons_pos = Vec3::ZERO;

    for event in events.read() {
        let position = [event.position.x, event.position.y, event.position.z];
        match event.kind {
            0 => {
                if state.acc[0] > 0.2 {
                    let _ = instantiate_audio_w_position(&core, ROOF_ONE_SHOT_EVENTS[0], position);
                    state.acc[0] = 0.0;
                }
            }
            1 => {
                if state.acc[1] > 0.2 {
                    let _ = instantiate_audio_w_position(&core, ROOF_ONE_SHOT_EVENTS[1], position);
                    state.acc[1] = 0.0;
                }
            }
            2 => {
                shape_selector = event.selector;
                shape_pos = event.position;
                shape_speed_source = event.value2;
                // O: two global params — RoofHeight = value0 (+0xc),
                // RoofProfile = value1 (+0x10) (disasm 0x140aa3d9a).
                let _ = core.set_parameter(ROOF_PARAMS[2], event.value0);
                let _ = core.set_parameter(ROOF_PARAMS[3], event.value1);
            }
            3 => {
                tip_flag = true;
                tip_pos = event.position;
                // O: RoofTipMoveSpeed = value1 (+0x10),
                // RoofTipOffset = value0 (+0xc) (disasm 0x140aa3df1).
                let _ = core.set_parameter(ROOF_PARAMS[4], event.value1);
                let _ = core.set_parameter(ROOF_PARAMS[5], event.value0);
            }
            4 => {
                let mut instance = core.instantiate_audio(ROOF_ONE_SHOT_EVENTS[2]);
                let _ = instance.set_position(position);
                // O: ImpactStrength = value0 (+0xc) (disasm 0x140aa3d09).
                let _ = instance.set_parameter(ROOF_PARAMS[6], event.value0);
                let _ = instance.start_if_not_playing();
            }
            5 => {
                let _ = instantiate_audio_w_position(&core, ROOF_ONE_SHOT_EVENTS[3], position);
            }
            6 => {
                let mut instance = core.instantiate_audio(ROOF_ONE_SHOT_EVENTS[4]);
                let _ = instance.set_parameter(
                    ROOF_PARAMS[1],
                    if event.selector != 0 { 1.0 } else { 0.0 },
                );
                let _ = instance.set_position(position);
                let _ = instance.start_if_not_playing();
            }
            7 => {
                let _ = instantiate_audio_w_position(&core, ROOF_ONE_SHOT_EVENTS[7], position);
            }
            8 => {}
            9 => {
                merlons_flag = true;
                merlons_pos = event.position;
            }
            0xa => {
                let _ = instantiate_audio_w_position(&core, ROOF_ONE_SHOT_EVENTS[5], position);
            }
            0xb => {
                let _ = instantiate_audio_w_position(&core, ROOF_ONE_SHOT_EVENTS[6], position);
            }
            _ => {}
        }
    }

    // O: frame-end — loop[0] (RoofShape) follows the selector (stop when 2).
    let loop0 = &mut state.loops[0];
    if shape_selector == 2 {
        let _ = loop0.stop_now();
    } else {
        let _ = loop0.start_if_not_playing();
        let current = &mut loop0.entries[loop0.cursor];
        let _ = current.set_position([shape_pos.x, shape_pos.y, shape_pos.z]);
        // O: editing_speed = clamp01(inverse_lerp(0, 5, value2 / speed))
        // (disasm 0x140aa42c4..0x140aa4327).
        let editing_speed = ((shape_speed_source / speed) / 5.0).max(0.0).min(1.0);
        let _ = current.set_parameter(ROOF_PARAMS[0], editing_speed);
        // O: is_gable = selector & 1 (disasm 0x140aa4350).
        let is_gable = if shape_selector & 1 != 0 { 1.0 } else { 0.0 };
        let _ = current.set_parameter(ROOF_PARAMS[1], is_gable);
    }
    // loop[1] (RoofTipMove).
    let loop1 = &mut state.loops[1];
    if tip_flag {
        let _ = loop1.start_if_not_playing();
        let current = &mut loop1.entries[loop1.cursor];
        let _ = current.set_position([tip_pos.x, tip_pos.y, tip_pos.z]);
    } else {
        let _ = loop1.stop_now();
    }
    // loop[2] (InterRoofMerlonsRearrange).
    let loop2 = &mut state.loops[2];
    if merlons_flag {
        let _ = loop2.start_if_not_playing();
        let current = &mut loop2.entries[loop2.cursor];
        let _ = current.set_position([merlons_pos.x, merlons_pos.y, merlons_pos.z]);
    } else {
        let _ = loop2.stop_now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roof_loop_names_are_in_init_order() {
        assert_eq!(ROOF_LOOP_EVENTS[0], "event:/SFX/RoofShape");
        assert_eq!(ROOF_LOOP_EVENTS[1], "event:/SFX/RoofTipMove");
        assert_eq!(ROOF_LOOP_EVENTS[2], "event:/SFX/InterRoofMerlonsRearrange");
    }

    #[test]
    fn roof_one_shots_match_the_dispatch_lengths() {
        assert_eq!(ROOF_ONE_SHOT_EVENTS[0], "event:/SFX/RoofAppear");
        assert_eq!(ROOF_ONE_SHOT_EVENTS[2], "event:/SFX/RoofTilesLand");
        assert_eq!(ROOF_ONE_SHOT_EVENTS[4], "event:/SFX/MerlonsAppear");
        assert_eq!(ROOF_ONE_SHOT_EVENTS[7], "event:/SFX/RoofSwitchDirection");
    }
}
