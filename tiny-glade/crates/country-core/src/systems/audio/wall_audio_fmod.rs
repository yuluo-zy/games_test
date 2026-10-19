//! B `wall_audio_fmod.rs`: the wall audio main system — 10 loops + kind 0..23
//! dispatch + frame-end start/stop.
//!
//! O-level facts (chapter wall_audio_fmod.rs.md, multi-baseline §1/§3, the
//! verified WALL_KIND_TABLE in systems::audio::wall_kinds):
//! - init: 10 × `instantiate_loop` (desc {count=3, max=50}) in the order
//!   Wall Draw / Wall Change Height / FenceExtend / LintelExtend /
//!   StoneArchExtend / WideningGate / PlatformStairsExtend / SewerGateWiden /
//!   FenceHeight / FenceMove → `insert_resource<WallAudioState>`.
//! - system: `PlayWallAudio` (slot stride 0x28, kind @+8) jump table 0x142ab4998
//!   kind 0..23: loop kinds → `request_play` + per-kind `set_parameter`
//!   (WallHeight / FenceLength / HasBricks); one-shot kinds →
//!   `instantiate_audio_w_position_and_volume`. Frame end: each loop starts /
//!   stops by its should-play flag and the state volume is modulated by
//!   `powf(0.1, speed × 5.0)`.

use crate::systems::audio::wall_kinds::WallKind;
use crate::systems::audio::wall_kinds::wall_kind;
use crate::resources::audio_instance::AudioLoopPlayer;
use crate::resources::audio_resources::AudioCore;
use crate::resources::audio_resources::AudioInstance;
use crate::resources::audio_resources::EventInstance;
use crate::resources::audio_resources::Studio;
use crate::systems::audio::roof_audio::instantiate_audio_w_position_and_volume;
use crate::systems::audio::wall_audio_utils::WALL_LOOP_EVENTS;
use crate::systems::mode_manager::AppMode;
use crate::systems::audio::PlayWallAudio;
use bevy_ecs::prelude::*;

/// WallAudioState (O: 10 × AudioLoopPlayer at stride 0x40 + state volume @+0x280).
#[derive(Resource)]
pub struct WallAudioState<E: EventInstance> {
    pub loops: [AudioLoopPlayer<AudioInstance<E>>; 10],
    /// Per-loop parameter vectors (native +0x48/+0x88/+0xc8/+0x148).
    pub loop_params: [Vec<AudioInstance<E>>; 4],
    pub volume: f32,
}

impl<E: EventInstance> WallAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        let loops = WALL_LOOP_EVENTS.map(|name| {
            core.instantiate_loop_player(&crate::resources::audio_resources::AudioLoopDesc::new(name))
        });
        WallAudioState {
            loops,
            loop_params: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            volume: 1.0,
        }
    }
}

/// `wall_audio_init_system` (0x14094fb80).
pub fn wall_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(WallAudioState::<S::Event>::new(&core));
}

/// `wall_audio_fmod` (0x140950920): the wall audio event system.
///
/// The native reads the `+0x58` editing speed (the shared
/// `bevy_time::Time::delta_secs` @+0x58) for the
/// frame-end volume modulation; an absent source keeps the previous behaviour.
pub fn wall_audio_fmod<S: Studio + Send + Sync + 'static>(
    mode: Res<AppMode>,
    core: Res<AudioCore<S>>,
    mut events: EventReader<PlayWallAudio>,
    mut state: ResMut<WallAudioState<S::Event>>,
    speed: Res<bevy_time::Time>,
) {
    // O: volume source — AppMode==0xd uses constant 1.0, else the stack
    // resource's +0x1c / 60.0 normalization. The per-event `volume` field feeds
    // `set_volume`; the editing-speed parameter is the normalized volume.
    let speed_factor = if mode.native_tag() == 0xd { 1.0 } else { 1.0 / 60.0 };

    for event in events.read() {
        let position = [event.position.x, event.position.y, event.position.z];
        let editing_speed = event.volume * speed_factor;
        match wall_kind(event.kind) {
            WallKind::Loop(loop_index) => {
                let loop_index = *loop_index;
                let _ = state.loops[loop_index].request_play(position, event.volume, editing_speed);
            }
            WallKind::LoopParam { loop_index, param, inverse_lerp_bounds } => {
                let loop_index = *loop_index;
                let _ = state.loops[loop_index].request_play(position, event.volume, editing_speed);
                let value = inverse_lerp(inverse_lerp_bounds.0, inverse_lerp_bounds.1, event.value);
                let _ = state.loops[loop_index].audio.current_mut()
                    .set_parameter(param, value);
            }
            WallKind::LoopBoolParam { loop_index, param, flag_offset: _ } => {
                let loop_index = *loop_index;
                let _ = state.loops[loop_index].request_play(position, event.volume, editing_speed);
                let value = if event.flag != 0 { 1.0 } else { 0.0 };
                let _ = state.loops[loop_index].audio.current_mut()
                    .set_parameter(param, value);
            }
            WallKind::OneShot(name) => {
                let _ = instantiate_audio_w_position_and_volume(&core, name, position, event.volume);
            }
            WallKind::Ignored => {}
        }
    }

    // O: frame-end — start/stop each loop by should-play flag.
    for loop_player in state.loops.iter_mut() {
        let _ = loop_player.frame_end();
    }
    // O (decomp-0x140950920 line 564): the state volume is modulated at frame
    // end by `powf(0.1, speed × 5.0)` where speed is the +0x58 editing speed.
    state.volume *= 0.1_f32.powf(speed.delta_secs() * 5.0);
}

/// `inverse_lerp` (shared utils, 0x140c904a0): value in [min,max] → [0,1]
/// **unclamped** — the native `inverse_lerp` extrapolates past the bounds (the
/// garden draw template documents the same native unclamped behaviour).
pub fn inverse_lerp(min: f32, max: f32, value: f32) -> f32 {
    (value - min) / (max - min)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverse_lerp_is_unclamped_like_the_native_0x140c904a0() {
        assert_eq!(inverse_lerp(0.0, 10.0, 5.0), 0.5);
        assert_eq!(inverse_lerp(0.0, 10.0, -5.0), -0.5);
        assert_eq!(inverse_lerp(0.0, 10.0, 15.0), 1.5);
    }
}
