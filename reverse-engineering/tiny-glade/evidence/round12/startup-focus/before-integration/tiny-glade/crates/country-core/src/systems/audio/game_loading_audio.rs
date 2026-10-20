//! B `game_loading_audio.rs`: loading-screen audio — `snapshot:/Loading`
//! snapshot + `event:/UI/Loading` one-shots.
//!
//! O-level facts (chapter game_loading_audio.rs.md, decomp-0x140b1ae70 /
//! 0x140b1b1b0, rne-read):
//! - `init_game_loading_audio` 0x140b1ae70: `AudioCoreInner::instantiate_audio`
//!   (`"snapshot:/Loading"`, 0x142ae94a8) → libfmod start → state resource.
//! - `loading_audio` 0x140b1b1b0: while loading work is pending the system
//!   instantiates + starts `event:/UI/Loading`; on completion it stops the
//!   snapshot (state→2) and clears the loading flag, with a 1 s timer
//!   (`state+0x34` accumulating `time+0x28`) as a second-stop guard.

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
};
use bevy_ecs::prelude::*;

/// `snapshot:/Loading` (O: 0x142ae94a8, len 0x11).
pub const LOADING_SNAPSHOT: &str = "snapshot:/Loading";
/// `event:/UI/Loading` (O: 0x142ae9510, len 0x11).
pub const LOADING_EVENT: &str = "event:/UI/Loading";

/// GameLoadingAudioState (O: snapshot instance @+0, loading flag @+0x28,
/// 1 s timer @+0x34).
#[derive(Resource)]
pub struct GameLoadingAudioState<E: EventInstance> {
    pub snapshot: AudioInstance<E>,
    pub loading: bool,
    pub timer: f32,
}

impl<E: EventInstance> GameLoadingAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        let mut snapshot = core.instantiate_audio(LOADING_SNAPSHOT);
        let _ = snapshot.start();
        GameLoadingAudioState {
            snapshot,
            loading: false,
            timer: 0.0,
        }
    }
}

/// `init_game_loading_audio` (0x140b1ae70).
pub fn init_game_loading_audio<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(GameLoadingAudioState::<S::Event>::new(&core));
}

/// `loading_audio` (0x140b1b1b0). `loading` is set by the host while loading
/// work is pending (the native consumes several resource/event streams —
/// 待接线 to the loading domain); the audio-side behavior (play the loading
/// one-shot, stop the snapshot, 1 s guard) is complete here.
pub fn loading_audio<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<GameLoadingAudioState<S::Event>>,
    core: Res<AudioCore<S>>,
    time: Res<bevy_time::Time>,
) {
    if state.loading {
        let mut event = core.instantiate_audio(LOADING_EVENT);
        let _ = event.start();
    } else {
        let _ = state.snapshot.stop();
    }
    // O: 1 s second-stop guard.
    if state.loading {
        state.timer += time.delta_secs();
        if state.timer > 1.0 {
            let _ = state.snapshot.stop();
            state.loading = false;
            state.timer = 0.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loading_event_names_are_original() {
        assert_eq!(LOADING_SNAPSHOT, "snapshot:/Loading");
        assert_eq!(LOADING_EVENT, "event:/UI/Loading");
    }
}
