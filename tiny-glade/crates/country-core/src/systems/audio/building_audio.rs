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
pub fn building_audio<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<BuildingAudioState<S::Event>>,
    mut events: EventReader<PlayBuildingAudio>,
    core: Res<AudioCore<S>>,
) {
    for event in events.read() {
        // O: only kind 6/7/8 produce one-shots; others are skipped.
        if event.kind >= 6 && event.kind < 9 {
            let name = BUILDING_ONE_SHOTS[(event.kind - 6) as usize];
            let position = [event.position.x, event.position.y, event.position.z];
            let mut instance = core.instantiate_audio(name);
            let _ = instance.set_position(position);
            let _ = instance.set_parameter("editing_speed", event.editing_speed);
            // O: BuildingSize = inverse_lerp(area) — the building area comes
            // from the 0x2a8 building table (待接线); the loop's size param is
            // set here with the event editing speed as a documented stand-in.
            let _ = instance.set_parameter("BuildingSize", event.editing_speed);
            let _ = instance.start_if_not_playing();
        }
    }

    // O: frame-end — drive the 7 loop kinds by their should-play flags.
    for loop_player in state.loops.iter_mut() {
        let _ = loop_player.frame_end();
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
