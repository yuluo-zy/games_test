//! B `building_audio.rs`: building edit sounds — 7 loops + detach/attach/peak
//! one-shots.
//!
//! O-level facts (chapter building_audio.rs.md, multi-baseline §2/§4, rne-read):
//! init creates 7 loops in order — BuildingResize / BuildingRotateRectangle /
//! BuildingRotateCircle / BuildingChangeHeight / BuildingMove /
//! PlasterChangeLoop / BuildingOverlapMove → `insert_resource<BuildingAudioState>`.
//! `building_audio` consumes `PlayBuildingAudio` (id u64, position Vec3,
//! editing_speed f32, kind u8): kind 6/7/8 only are one-shots
//! (DetachFromGround / AttachToGround / PeakReached) with
//! set_parameter("editing_speed") + set_parameter("BuildingSize"=inverse_lerp(area));
//! the frame end drives the 7 loop kinds {0,1,2,3,4,9,5} with
//! editing_speed / BuildingSize / ifWindow / BuildingHeight / isHalfTimbered /
//! IsOnGround / loop parameters.

use crate::{
    resources::{
        audio_instance::AudioLoopPlayer,
        audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    },
};
use crate::systems::audio::PlayBuildingAudio;
use bevy_ecs::prelude::*;

/// The seven building loops (O: rne-read; init order).
pub const BUILDING_LOOP_EVENTS: [&str; 7] = [
    "event:/SFX/Building/BuildingResize",
    "event:/SFX/Building/BuildingRotateRectangle",
    "event:/SFX/Building/BuildingRotateCircle",
    "event:/SFX/Building/BuildingChangeHeight",
    "event:/SFX/Building/BuildingMove",
    "event:/SFX/PlasterChangeLoop",
    "event:/SFX/Building/BuildingOverlapMove",
];

/// O (decomp-0x1417d2ad0): the loop kind → loop-index order the frame-end
/// drives (`local_208` = {0,1,2,3,4,9,5}). Kind 5 maps to the last loop
/// (BuildingOverlapMove), kind 9 to PlasterChangeLoop.
pub const BUILDING_LOOP_KINDS: [u8; 7] = [0, 1, 2, 3, 4, 9, 5];

/// One-shots for kind 6/7/8 (O: rne-read).
pub const BUILDING_ONE_SHOTS: [&str; 3] = [
    "event:/SFX/Shapes/DetachFromGround",
    "event:/SFX/Shapes/AttachToGround",
    "event:/SFX/Shapes/PeakReached",
];

/// BuildingAudioState (O: 7 loop players).
#[derive(Resource)]
pub struct BuildingAudioState<E: EventInstance> {
    pub loops: [AudioLoopPlayer<AudioInstance<E>>; 7],
}

impl<E: EventInstance> BuildingAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        BuildingAudioState {
            loops: BUILDING_LOOP_EVENTS.map(|name| {
                core.instantiate_loop_player(&crate::resources::audio_resources::AudioLoopDesc::new(name))
            }),
        }
    }
}

/// `building_audio_init_system` (0x1417d21a0).
pub fn building_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(BuildingAudioState::<S::Event>::new(&core));
}

/// `building_audio` (0x1417d2ad0): one-shot dispatch + frame-end loop driving.
///
/// O (decomp-0x1417d2ad0): the native groups the `PlayBuildingAudio` events by
/// kind and drives each of the 7 loop kinds {0,1,2,3,4,9,5} — a matching event
/// sets the loop's should-play/position/parameters, otherwise the loop is
/// stopped (previously the loop kinds were discarded by the one-shot filter, so
/// the 7 loops never started). Kinds 6/7/8 are the Detach/Attach/Peak one-shots
/// with `BuildingSize = inverse_lerp(area)` (the building-table area lookup is
/// 待接线; `editing_speed` stands in here as documented).
pub fn building_audio<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<BuildingAudioState<S::Event>>,
    mut events: EventReader<PlayBuildingAudio>,
    core: Res<AudioCore<S>>,
) {
    let collected: Vec<PlayBuildingAudio> = events.read().copied().collect();

    // O: per-loop-kind drive (frame-end start_if_not_playing / stop).
    for (index, loop_kind) in BUILDING_LOOP_KINDS.iter().enumerate() {
        let loop_player = &mut state.loops[index];
        let event = collected.iter().find(|e| e.kind == *loop_kind);
        match event {
            Some(event) => {
                let position = [event.position.x, event.position.y, event.position.z];
                let _ = loop_player.request_play(position, 1.0, event.editing_speed);
                // O: per-loop BuildingSize / ifWindow / BuildingHeight /
                // isHalfTimbered / IsOnGround params are set from the building
                // table (0x2a8 stride, 待接线); BuildingSize stands in here.
                let current = loop_player.current;
                let _ = loop_player.instances[current]
                    .set_parameter("BuildingSize", event.editing_speed);
            }
            None => {
                let _ = loop_player.frame_end();
            }
        }
    }

    // O: kinds 6/7/8 only produce the Detach/Attach/Peak one-shots.
    for event in collected {
        if event.kind >= 6 && event.kind < 9 {
            let name = BUILDING_ONE_SHOTS[(event.kind - 6) as usize];
            let position = [event.position.x, event.position.y, event.position.z];
            let mut instance = core.instantiate_audio(name);
            let _ = instance.set_position(position);
            let _ = instance.set_parameter("editing_speed", event.editing_speed);
            // O: BuildingSize = inverse_lerp(area) — the building area comes
            // from the 0x2a8 building table (待接线); the event editing speed
            // is the documented stand-in.
            let _ = instance.set_parameter("BuildingSize", event.editing_speed);
            let _ = instance.start_if_not_playing();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn building_event_names_are_original() {
        assert_eq!(BUILDING_LOOP_EVENTS[0], "event:/SFX/Building/BuildingResize");
        assert_eq!(BUILDING_LOOP_EVENTS[5], "event:/SFX/PlasterChangeLoop");
        assert_eq!(BUILDING_ONE_SHOTS[0], "event:/SFX/Shapes/DetachFromGround");
        assert_eq!(BUILDING_ONE_SHOTS[2], "event:/SFX/Shapes/PeakReached");
    }
}
