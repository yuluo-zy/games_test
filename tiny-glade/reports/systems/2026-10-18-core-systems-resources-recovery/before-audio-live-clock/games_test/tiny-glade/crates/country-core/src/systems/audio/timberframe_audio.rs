//! B `timberframe_audio.rs`: timberframe edit sounds.
//!
//! O-level facts (chapter timberframe_audio.rs.md, decomp-0x140a67810 /
//! 0x140a67b90, disasm-0x140a67f60, rne-read): `timberframe_audio_init_system`
//! dispatches `PlayTimberframeSound` kinds to `TimberframeMove/Length/Width`
//! loops and the verified `TimberframeStartEdit` / `TimberframeAppear` /
//! `TimberframeDisappear` / `TimberFrameSnap` / `Building/BuildingCreate`
//! one-shots (string table 0x142ad4fe7..0x142ad505b), with `editing_speed` set
//! on the loop instances. `timberframe_audio` (0x140a67b90) is the state
//! assembly.

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    systems::audio::roof_audio::instantiate_audio_w_position,
    utils::audio_utils::AudioLoop,
};
use crate::systems::audio::PlayTimberframeSound;
use bevy_ecs::prelude::*;
use glam::Vec3;

/// Timberframe loop events (O: rne-read; frame-end slots: state+0x00 = Move,
/// +0x38 = Length, +0x70 = Width. Kind 3 arms Move, kind 1 arms Length,
/// kind 0 arms Width — disasm 0x140a67f1d/0x140a68008/0x140a67f92 and the
/// frame-end slot mapping).
pub const TIMBERFRAME_LOOP_EVENTS: [&str; 3] = [
    "event:/SFX/TimberframeMove",
    "event:/SFX/TimberframeLength",
    "event:/SFX/TimberframeWidth",
];

/// Timberframe one-shot event by kind (O: disasm-0x140a67b90 string table:
/// kind 2 → `event:/SFX/TimberframeStartEdit` (len 0x1f), kind 4 →
/// `TimberframeAppear` (0x1c), kind 5 → `TimberframeDisappear` (0x1f), kind 6 →
/// `TimberFrameSnap` (0x1a, note the capital F), kind 7 →
/// `event:/SFX/Building/BuildingCreate` (0x22). The prior
/// RectangleAppear/CircleAppear/... list did not match the native string
/// lengths and is dropped.
pub fn timberframe_one_shot(kind: u8) -> Option<&'static str> {
    Some(match kind {
        2 => "event:/SFX/TimberframeStartEdit",
        4 => "event:/SFX/TimberframeAppear",
        5 => "event:/SFX/TimberframeDisappear",
        6 => "event:/SFX/TimberFrameSnap",
        7 => "event:/SFX/Building/BuildingCreate",
        _ => return None,
    })
}

/// TimberframeAudioState: the three loops.
#[derive(Resource)]
pub struct TimberframeAudioState<E: EventInstance> {
    pub loops: [AudioLoop<AudioInstance<E>>; 3],
}

impl<E: EventInstance> TimberframeAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        TimberframeAudioState {
            loops: TIMBERFRAME_LOOP_EVENTS.map(|name| {
                core.instantiate_loop(&crate::resources::audio_resources::AudioLoopDesc::new(name))
            }),
        }
    }
}

/// `timberframe_audio` (0x140a67b90): state assembly + one-shot dispatch.
///
/// Fresh disasm: event payload kind u8 @0, flag byte @+1, speed-source f32 @+4,
/// position @+8, value f32 @+0x14. Kinds 0/1 arm the Width/Length loops
/// (`editing_speed = clamp01(inverse_lerp(0, 30, speed_source / speed))`),
/// kind 3 the Move loop (`clamp01(inverse_lerp(0, 250, speed_source))`);
/// kinds 2/4/5/6/7 are the verified one-shots. Every loop gets
/// `BuildingSize = value@+0x14`; Move additionally gets
/// `is_on_ground = flag@+1`.
pub fn timberframe_audio<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<TimberframeAudioState<S::Event>>,
    mut events: EventReader<PlayTimberframeSound>,
    core: Res<AudioCore<S>>,
    speed: Option<Res<crate::systems::audio::audio_update::EditingSpeed>>,
) {
    let speed = speed.map_or(0.0, |s| s.0);
    // Per-frame loop flags and values (native local_130 = Move,
    // local_128 = Length, local_120 = Width).
    let mut move_flag = false;
    let mut move_pos = Vec3::ZERO;
    let mut move_speed = 0.0f32;
    let mut move_size = 0.0f32;
    let mut move_ground = 0.0f32;
    let mut length_flag = false;
    let mut length_pos = Vec3::ZERO;
    let mut length_speed = 0.0f32;
    let mut length_size = 0.0f32;
    let mut width_flag = false;
    let mut width_pos = Vec3::ZERO;
    let mut width_speed = 0.0f32;
    let mut width_size = 0.0f32;

    for event in events.read() {
        let position = [event.position.x, event.position.y, event.position.z];
        if let Some(name) = timberframe_one_shot(event.kind) {
            let _ = instantiate_audio_w_position(&core, name, position);
            continue;
        }
        match event.kind {
            0 => {
                // Width loop (slot 2): inverse_lerp(0, 30, speed_source / speed).
                width_flag = true;
                width_pos = event.position;
                width_speed = clamp01(event.speed_source / speed / 30.0);
                width_size = event.value;
            }
            1 => {
                // Length loop (slot 1): inverse_lerp(0, 30, speed_source / speed).
                length_flag = true;
                length_pos = event.position;
                length_speed = clamp01(event.speed_source / speed / 30.0);
                length_size = event.value;
            }
            3 => {
                // Move loop (slot 0): inverse_lerp(0, 250, speed_source).
                move_flag = true;
                move_pos = event.position;
                move_speed = clamp01(event.speed_source / 250.0);
                move_size = event.value;
                move_ground = if event.flag & 1 != 0 { 1.0 } else { 0.0 };
            }
            _ => {}
        }
    }

    // O: frame-end — drive the 3 loops by their flags.
    drive(
        &mut state.loops[0],
        move_flag,
        move_pos,
        move_speed,
        move_size,
        Some(move_ground),
    );
    drive(
        &mut state.loops[1],
        length_flag,
        length_pos,
        length_speed,
        length_size,
        None,
    );
    drive(
        &mut state.loops[2],
        width_flag,
        width_pos,
        width_speed,
        width_size,
        None,
    );
}

fn clamp01(value: f32) -> f32 {
    value.max(0.0).min(1.0)
}

/// O: `start_if_not_playing → set_position → set_parameter("editing_speed") →
/// set_parameter("BuildingSize") [→ set_parameter("is_on_ground")]`.
fn drive<E: EventInstance>(
    loop_player: &mut AudioLoop<AudioInstance<E>>,
    play: bool,
    position: Vec3,
    editing_speed: f32,
    building_size: f32,
    on_ground: Option<f32>,
) {
    if play {
        let _ = loop_player.start_if_not_playing();
        let cursor = loop_player.cursor;
        let _ = loop_player.entries[cursor].set_position([position.x, position.y, position.z]);
        let _ = loop_player.entries[cursor].set_parameter("editing_speed", editing_speed);
        let _ = loop_player.entries[cursor].set_parameter("BuildingSize", building_size);
        if let Some(on_ground) = on_ground {
            let _ = loop_player.entries[cursor].set_parameter("is_on_ground", on_ground);
        }
    } else {
        let _ = loop_player.stop(std::time::Duration::ZERO);
    }
}

/// `timberframe_audio_init_system` (0x140a67810).
pub fn timberframe_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(TimberframeAudioState::<S::Event>::new(&core));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timberframe_event_names_are_original() {
        assert_eq!(TIMBERFRAME_LOOP_EVENTS[0], "event:/SFX/TimberframeMove");
        assert_eq!(TIMBERFRAME_LOOP_EVENTS[1], "event:/SFX/TimberframeLength");
        assert_eq!(TIMBERFRAME_LOOP_EVENTS[2], "event:/SFX/TimberframeWidth");
        assert_eq!(timberframe_one_shot(2), Some("event:/SFX/TimberframeStartEdit"));
        assert_eq!(timberframe_one_shot(6), Some("event:/SFX/TimberFrameSnap"));
        assert_eq!(timberframe_one_shot(7), Some("event:/SFX/Building/BuildingCreate"));
    }
}
