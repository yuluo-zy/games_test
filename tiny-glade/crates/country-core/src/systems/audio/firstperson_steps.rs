//! B `firstperson_steps.rs`: first-person footstep sounds — a walking loop +
//! the `FirstPersonOff` global parameter + terrain-raster material sampling.
//!
//! O-level facts (chapter firstperson_steps.rs.md, decomp-0x140a889b0,
//! procs.tsv signature): the system is
//! `(Res<AudioCore>, Res<MainCamera>, Res<AssetSsboLibrary>,
//!  PathRastersResources, ResMut<FirstPersonViewAudioState>, Res<WaterRaster>,
//!  Res<GardenRaster>, Res<ActionSetStates>)`. It maps the ground material
//! (`GroundType::to_fmod_param`), sets the `FirstPersonOff` global parameter,
//! and when on raster terrain (GroundType==3) and moving, bilinear-samples the
//! terrain grid to pick the walking material. GroundType==0 plays the walking
//! loop; otherwise it stops. The cross-domain inputs (ActionSetStates /
//! MainCamera / raster resources) 待接线; the audio-side logic is complete here.

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    utils::audio_utils::AudioLoop,
};
use bevy_ecs::prelude::*;

/// `FirstPerson` global parameter (O: 0x142ad8eb0, len 0xb; the chapter's
/// "FirstPersonOff" is the signifier-side camera event, not this param).
pub const FIRST_PERSON_PARAM: &str = "FirstPerson";
/// `GroundType` global parameter (O: 0x142ad8ed8) — the walking material.
pub const GROUND_TYPE_PARAM: &str = "GroundType";

/// Walking loop event (O: 0x142ad8e28, len 0x19).
pub const WALK_LOOP_EVENT: &str = "event:/SFX/Player/Walking";

/// Ground material driving the footstep material parameter (O: native
/// `GroundType::to_fmod_param`; the exact variant set/values 待取证).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum GroundType {
    #[default]
    Grass = 0,
    Water = 1,
    Garden = 2,
    /// Raster terrain — needs bilinear sampling of the terrain grid.
    Terrain = 3,
}

impl GroundType {
    /// `GroundType::to_fmod_param` (native line 128): material parameter value.
    pub fn to_fmod_param(self) -> f32 {
        match self {
            Self::Grass => 0.0,
            Self::Water => 1.0,
            Self::Garden => 2.0,
            Self::Terrain => 3.0,
        }
    }

    pub fn from_tag(tag: u8) -> Self {
        match tag {
            1 => Self::Water,
            2 => Self::Garden,
            3 => Self::Terrain,
            _ => Self::Grass,
        }
    }
}

/// FirstPersonViewAudioState (O: the walking loop).
#[derive(Resource)]
pub struct FirstPersonViewAudioState<E: EventInstance> {
    pub walking_loop: AudioLoop<AudioInstance<E>>,
    pub ground: GroundType,
}

impl<E: EventInstance> FirstPersonViewAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        FirstPersonViewAudioState {
            walking_loop: core.instantiate_loop(
                &crate::resources::audio_resources::AudioLoopDesc::new(WALK_LOOP_EVENT),
            ),
            ground: GroundType::Grass,
        }
    }
}

/// `firstperson_steps_init_system` (native inline): build the walking loop.
pub fn firstperson_steps_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(FirstPersonViewAudioState::<S::Event>::new(&core));
}

/// `firstperson_steps` (0x140a889b0): set the FirstPerson / GroundType global
/// parameters and drive the walking loop by ground material (raster sampling
/// 待接线).
pub fn firstperson_steps<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut state: ResMut<FirstPersonViewAudioState<S::Event>>,
) {
    // O: set_parameter("FirstPerson", value) at 0x140a88a27 (value source
    // 待接线 — the camera/view state); and set_parameter("GroundType",
    // material) at 0x140a88ec1.
    let _ = core.set_parameter(FIRST_PERSON_PARAM, 1.0);
    let _ = core.set_parameter(GROUND_TYPE_PARAM, state.ground.to_fmod_param());

    // O: GroundType==0 → play walking loop, else stop.
    if state.ground == GroundType::Grass {
        let _ = state.walking_loop.start_if_not_playing();
    } else {
        let _ = state.walking_loop.stop(std::time::Duration::ZERO);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_person_param_and_walk_loop_are_original() {
        assert_eq!(FIRST_PERSON_PARAM, "FirstPerson");
        assert_eq!(WALK_LOOP_EVENT, "event:/SFX/Player/Walking");
        assert_eq!(GROUND_TYPE_PARAM, "GroundType");
    }
}
