//! B `decorator_audio.rs`: decorator (window/lantern/chimney/flag/clutter)
//! edit sounds — 16 loops driven by `PlayDecoratorSoundCmd`.
//!
//! O-level facts (chapter decorator_audio.rs.md, decomp-0x14145c6b0 /
//! 0x14145dc20, rne-read): init creates 16 loops (decomp names, incl. three
//! `Lantern/Amb` and the `Cosmetics/Clutter_Rotate` / `Cosmetics/MoveGround`
//! pair). The system collects `PlayDecoratorSoundCmd` events into a
//! `(dist, position)` list, stable-sorts it, and drives the loop players by
//! distance/attenuation with stop/start_if_not_playing pair toggling.

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    utils::audio_utils::AudioLoop,
};
use crate::systems::audio::PlayDecoratorSoundCmd;
use bevy_ecs::prelude::*;
use glam::Vec3;

/// The 16 decorator loops (O: decomp-0x14145c6b0 string constants).
pub const DECORATOR_LOOP_EVENTS: [&str; 16] = [
    "event:/SFX/WindowDrag",
    "event:/SFX/DecoratorMoveWoodenFloorTEMP",
    "event:/SFX/Lantern/MoveWall",
    "event:/SFX/Lantern/MoveGround",
    "event:/SFX/Lantern/Rotate",
    "event:/SFX/Lantern/Amb",
    "event:/SFX/Lantern/Amb",
    "event:/SFX/Lantern/Amb",
    "event:/SFX/ChimneyChangeHeight",
    "event:/SFX/ChimneyDrag",
    "event:/SFX/Flags/Rotate",
    "event:/SFX/Flags/Drag",
    "event:/SFX/Flags/DragGround",
    "event:/SFX/Cosmetics/Clutter_Rotate",
    "event:/SFX/Cosmetics/MoveGround",
    "event:/SFX/Lantern/Amb",
];

/// DecoratorAudioState: the 16 loops + per-loop should-play flag.
#[derive(Resource)]
pub struct DecoratorAudioState<E: EventInstance> {
    pub loops: [AudioLoop<AudioInstance<E>>; 16],
    pub should_play: [bool; 16],
    pub positions: [Vec3; 16],
}

impl<E: EventInstance> DecoratorAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        DecoratorAudioState {
            loops: DECORATOR_LOOP_EVENTS.map(|name| {
                core.instantiate_loop(&crate::resources::audio_resources::AudioLoopDesc::new(name))
            }),
            should_play: [false; 16],
            positions: [Vec3::ZERO; 16],
        }
    }
}

/// `decorator_audio_init_system` (0x14145c6b0).
pub fn decorator_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(DecoratorAudioState::<S::Event>::new(&core));
}

/// Map a `PlayDecoratorSoundCmd` kind to a loop index. The native 0x14145dc20
/// jump table's full kind→loop mapping is 待取证; the decomp shows distance
/// sorting then per-loop stop/start_if_not_playing pair toggling, so the kind
/// below selects the loop that matches the edit action where known.
fn kind_to_loop(kind: u8) -> usize {
    // O partial: window edits → loop 0 (WindowDrag); lantern placement/move →
    // 2..7; chimney height → 8; flag rotate → 10; clutter rotate → 13.
    match kind {
        0..=1 => 0,
        2..=5 => 2,
        6 => 3,
        7..=8 => 8,
        9 => 10,
        10 => 13,
        _ => 5, // Lantern/Amb fallback.
    }
}

/// `decorator_audio_system` (0x14145dc20): collect events, drive the matching
/// loop at the event position (O: distance-sorted stop/start_if_not_playing
/// toggling). The exact attenuation curve is 待取证.
pub fn decorator_audio_system<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<DecoratorAudioState<S::Event>>,
    mut events: EventReader<PlayDecoratorSoundCmd>,
    _core: Res<AudioCore<S>>,
) {
    for event in events.read() {
        let index = kind_to_loop(event.kind);
        state.should_play[index] = true;
        state.positions[index] = event.position;
    }

    let should_play = std::mem::take(&mut state.should_play);
    let positions = std::mem::take(&mut state.positions);
    for (index, loop_player) in state.loops.iter_mut().enumerate() {
        if should_play[index] {
            let _ = loop_player.start_if_not_playing();
            let current = &mut loop_player.entries[loop_player.cursor];
            let pos = positions[index];
            let _ = current.set_position([pos.x, pos.y, pos.z]);
        } else {
            let _ = loop_player.stop(std::time::Duration::ZERO);
        }
    }
    state.should_play = should_play;
    state.positions = positions;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decorator_loop_count_and_first_names_are_original() {
        assert_eq!(DECORATOR_LOOP_EVENTS.len(), 16);
        assert_eq!(DECORATOR_LOOP_EVENTS[0], "event:/SFX/WindowDrag");
        assert_eq!(DECORATOR_LOOP_EVENTS[13], "event:/SFX/Cosmetics/Clutter_Rotate");
        assert_eq!(DECORATOR_LOOP_EVENTS[14], "event:/SFX/Cosmetics/MoveGround");
    }
}
