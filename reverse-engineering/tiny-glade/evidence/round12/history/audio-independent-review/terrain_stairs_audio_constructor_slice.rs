use crate::resources::audio_resources::{AudioCore,AudioInstance,EventInstance,Studio};
use crate::utils::audio_utils::AudioLoop;
use bevy_ecs::prelude::*;
use glam::Vec3;
pub const STAIR_DRAW_LOOP_EVENT: &str = "event:/SFX/StairDraw_LoopTrem";
/// Instance parameter (O: 0x142c62943).
pub const STAIR_EDITING_SPEED_PARAM: &str = "editing_speed";

/// Host face for the native stair-edit event stream (producer: terrain stairs
/// domain, 待接线). The audio handler only needs the per-frame presence.
#[derive(Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TerrainStairsAudioEvent;

/// O (native SSBO buffer first u32; the exact `SsboType::NAME`/id is 待取证 —
/// the terrain-domain writer fills this host face).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct StairsRasterValue(pub u32);

/// StairsCounter (native `+0x40` counter, incremented when a ring event is
/// pushed).
#[derive(Debug, Default, Clone, Copy)]
pub struct StairsCounter {
    pub count: u32,
}

/// TerrainStairsAudioState (O: ring + retained instances + last raster value).
#[derive(Resource)]
pub struct TerrainStairsAudioState<E: EventInstance> {
    pub ring: AudioLoop<AudioInstance<E>>,
    pub instances: Vec<AudioInstance<E>>,
    pub counter: StairsCounter,
    /// Last observed host SSBO value (O: the state comparison).
    pub last_raster: u32,
}

impl<E: EventInstance> TerrainStairsAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        TerrainStairsAudioState {
            ring: core.instantiate_loop(
                &crate::resources::audio_resources::AudioLoopDesc::new(STAIR_DRAW_LOOP_EVENT),
            ),
            instances: Vec::new(),
            counter: StairsCounter::default(),
            last_raster: 0,
        }
    }
}

