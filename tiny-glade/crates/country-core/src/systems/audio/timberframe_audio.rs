//! B `timberframe_audio.rs`: timberframe edit sounds.
//!
//! O-level facts (chapter timberframe_audio.rs.md, decomp-0x140a67810 /
//! 0x140a67b90, rne-read): `timberframe_audio_init_system` dispatches
//! `PlayTimberframeSound` kinds to `TimberframeMove/Length/Width` loops and
//! the `Timberframe{Rectangle,Circle}Appear` / `Disappear` / `StartEdit` /
//! `Appear` one-shots, with `editing_speed` set on the loop instances.
//! `timberframe_audio` (0x140a67b90) is the state assembly.
//!
//! The exact kind→loop/one-shot table is 待取证 (chapter notes the pattern but
//! not every case); the dispatch below follows the event-name ordering.

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    systems::audio::roof_audio::instantiate_audio_w_position,
    utils::audio_utils::AudioLoop,
};
use crate::systems::audio::PlayTimberframeSound;
use bevy_ecs::prelude::*;

/// Timberframe loop events (O: rne-read).
pub const TIMBERFRAME_LOOP_EVENTS: [&str; 3] = [
    "event:/SFX/TimberframeMove",
    "event:/SFX/TimberframeLength",
    "event:/SFX/TimberframeWidth",
];
/// Timberframe one-shots (O: rne-read).
pub const TIMBERFRAME_ONE_SHOTS: [&str; 5] = [
    "event:/SFX/TimberframeRectangleAppear",
    "event:/SFX/TimberframeCircleAppear",
    "event:/SFX/TimberframeDisappear",
    "event:/SFX/TimberframeStartEdit",
    "event:/SFX/TimberframeAppear",
];

/// TimberframeAudioState: the three loops + the five one-shot event names.
#[derive(Resource)]
pub struct TimberframeAudioState<E: EventInstance> {
    pub loops: [AudioLoop<AudioInstance<E>>; 3],
    pub one_shot_events: [&'static str; 5],
}

impl<E: EventInstance> TimberframeAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        TimberframeAudioState {
            loops: TIMBERFRAME_LOOP_EVENTS.map(|name| {
                core.instantiate_loop(&crate::resources::audio_resources::AudioLoopDesc::new(name))
            }),
            one_shot_events: TIMBERFRAME_ONE_SHOTS,
        }
    }
}

/// `timberframe_audio` (0x140a67b90): state assembly + one-shot dispatch.
pub fn timberframe_audio<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<TimberframeAudioState<S::Event>>,
    mut events: EventReader<PlayTimberframeSound>,
    core: Res<AudioCore<S>>,
) {
    for event in events.read() {
        // O: kind ≤ 2 → loops (Move/Length/Width), else the one-shot table.
        let kind = usize::from(event.kind);
        let position = [event.position.x, event.position.y, event.position.z];
        if kind < 3 {
            let cursor = state.loops[kind].cursor;
            state.loops[kind]
                .entries
                .get_mut(cursor)
                .map(|current| current.set_position(position));
            let _ = state.loops[kind].start_if_not_playing();
        } else if let Some(name) = state.one_shot_events.get(kind - 3) {
            let _ = instantiate_audio_w_position(&core, name, position);
        }
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
        assert_eq!(TIMBERFRAME_ONE_SHOTS[0], "event:/SFX/TimberframeRectangleAppear");
    }
}
