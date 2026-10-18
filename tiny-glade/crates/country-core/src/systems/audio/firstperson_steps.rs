//! B `firstperson_steps.rs`: first-person footstep sounds — the walking loop,
//! the `FirstPerson` / `GroundType` global parameters and the terrain-raster
//! material sampling.
//!
//! Fresh evidence (decomp-0x140a889b0, disasm head 0x140a889b0.. and tail
//! 0x140a88ec0..):
//! - `FirstPerson` = `camera.ground_type(@+0x4d0) == 3 ? 1.0 : 0.0`.
//! - Sampling gate: ground == 3 AND (movement state OR nonzero camera movement
//!   delta). The native then:
//!   1. reads `camera@+0x328` (Ssbo type); when it equals 11 it resolves the
//!      `TerrainRockMask` buffer (name assertion length 0x3f), bilinear-samples
//!      the terrain raster at the camera eye, accumulates the 4-neighbour
//!      samples (clamp01), and (when that is ≤ 0.1) tests the rock-mask bit grid;
//!      bit set → the sampled material stays at the current value, bit clear →
//!      samples the water/garden raster;
//!   2. the resulting `GroundType` value is `4.0` when the raster sample is
//!      > 0.1, else `0.0`; the default (no sampling) is `2.0`;
//!   3. `GroundType` is written as a global parameter.
//! - Loop gate: `camera@+0x344 == 0` starts the walking loop, otherwise stops it.
//!   (The previous "ground == Grass" gate and the hard-coded FirstPerson = 1.0
//!   were self-approximations and are dropped.)
//!
//! Cross-domain wiring: the SsboId 11 lookup / `TerrainRockMask` name
//! assertion / 0x3f-bit buffer and the InputDelta +0x150 movement source are
//! host-side; this file consumes the `MainCamera` raw bytes and a movement host
//! face, and registers the sampling inputs in the rebuild report.

use crate::{
    resources::{
        audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
        world_raster::RasterBufferRef,
        world_raster_defs::{GardenRaster, WaterRaster},
    },
    systems::{
        audio::garden_audio::GARDEN_RASTER_EXTENT,
        camera_rig::main_camera::MainCamera,
    },
    utils::audio_utils::AudioLoop,
};
use bevy_ecs::prelude::*;

/// `FirstPerson` global parameter (O: 0x142ad8eb0, len 0xb).
pub const FIRST_PERSON_PARAM: &str = "FirstPerson";
/// `GroundType` global parameter (O: 0x142ad8ed8).
pub const GROUND_TYPE_PARAM: &str = "GroundType";

/// Walking loop event (O: init 0x140a88fa0 string at 0x142ad8ee2? The walking
/// loop is the `event:/SFX/Garden/GardenDraw` loop created by the init system;
/// rne-read `event:/SFX/Player/Walking` at 0x142ad8e28 is the adjacent string).
pub const WALK_LOOP_EVENT: &str = "event:/SFX/Player/Walking";

/// Native `GroundType` global-parameter values (O: default 2.0, sampled 4.0 /
/// 0.0).
pub const GROUND_TYPE_DEFAULT: f32 = 2.0;
pub const GROUND_TYPE_SAMPLED: f32 = 4.0;
pub const GROUND_TYPE_NONE: f32 = 0.0;
/// Raster sample threshold (O: 0x142925970 = 0.1).
pub const GROUND_SAMPLE_THRESHOLD: f32 = 0.1;

/// FirstPersonViewAudioState (O: the walking loop + the last `GroundType`
/// global value).
#[derive(Resource)]
pub struct FirstPersonViewAudioState<E: EventInstance> {
    pub walking_loop: AudioLoop<AudioInstance<E>>,
    pub ground_type: f32,
}

impl<E: EventInstance> FirstPersonViewAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        FirstPersonViewAudioState {
            walking_loop: core.instantiate_loop(
                &crate::resources::audio_resources::AudioLoopDesc::new(WALK_LOOP_EVENT),
            ),
            ground_type: GROUND_TYPE_DEFAULT,
        }
    }
}

/// `firstperson_steps_init_system` (inline native init): build the walking loop.
pub fn firstperson_steps_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(FirstPersonViewAudioState::<S::Event>::new(&core));
}

/// Movement host face (O: `ActionSetStates +0x18 < 3 || |delta@+0x150| > 0`).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct PlayerMoving(pub bool);

/// `firstperson_steps` (0x140a889b0).
#[allow(clippy::too_many_arguments)]
pub fn firstperson_steps<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut state: ResMut<FirstPersonViewAudioState<S::Event>>,
    camera: Res<MainCamera>,
    moving: Option<Res<PlayerMoving>>,
    water: Option<Res<WaterRaster>>,
    garden: Option<Res<GardenRaster>>,
) {
    let bytes = camera.bytes();
    let ground = bytes[0x4d0];

    // O: FirstPerson = (ground type == 3) ? 1.0 : 0.0.
    let first_person = if ground == 3 { 1.0 } else { 0.0 };
    let _ = core.set_parameter(FIRST_PERSON_PARAM, first_person);

    // O: ground == 3 and moving → sample the terrain/water/garden rasters. The
    // SsboId-11 rock-mask bit grid is the host-side input; when the rasters are
    // absent the current GroundType is kept.
    if ground == 3 && moving.is_some_and(|m| m.0) {
        let eye = camera.eye_position();
        let sample = |raster: &crate::resources::world_raster::WorldRaster<512>| {
            RasterBufferRef::<512>::new(raster.current_buffer())
                .bilinear_sample_at_world_xz(
                    eye.x,
                    eye.z,
                    GARDEN_RASTER_EXTENT,
                    GARDEN_RASTER_EXTENT,
                )
        };
        let sampled = water
            .as_ref()
            .map(|w| sample(&w.0))
            .or_else(|| garden.as_ref().map(|g| sample(&g.0)));
        if let Some(value) = sampled {
            // O: the rock-mask bit (SsboId 11) would keep the current material
            // when set; without it the raster value decides.
            state.ground_type = if value > GROUND_SAMPLE_THRESHOLD {
                GROUND_TYPE_SAMPLED
            } else {
                GROUND_TYPE_NONE
            };
        }
    }

    // O: GroundType global parameter (default 2.0).
    let _ = core.set_parameter(GROUND_TYPE_PARAM, state.ground_type);

    // O: loop gate — camera@+0x344 == 0 starts the walking loop, else stops.
    if bytes[0x344] == 0 {
        let _ = state.walking_loop.start_if_not_playing();
    } else {
        let _ = state.walking_loop.stop_now();
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

    #[test]
    fn ground_type_values_match_the_native_constants() {
        assert_eq!(GROUND_TYPE_DEFAULT, 2.0);
        assert_eq!(GROUND_TYPE_SAMPLED, 4.0);
        assert_eq!(GROUND_TYPE_NONE, 0.0);
    }
}
