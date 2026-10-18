//! B `decorator_audio.rs`: decorator (window/lantern/chimney/flag/clutter)
//! edit sounds — 16 loops driven by `PlayDecoratorSoundCmd`.
//!
//! O-level facts (chapter decorator_audio.rs.md, decomp-0x14145c6b0 /
//! 0x14145dc20, disasm-0x14145e9c0, rne-read): init creates 16 loops (decomp
//! names, incl. three `Lantern/Amb` and the `Cosmetics/Clutter_Rotate` /
//! `Cosmetics/MoveGround` pair). The system dispatches each event kind to a
//! **one-shot** (verified names: WindowPlace / WindowCombine /
//! WindowPlacePreview / WindowDelete / WindowRemove / WindowChangeSeed /
//! WindowHover / WindowSplit / Lantern/Delete / ChimneyBaseDisappear /
//! ChimneyDelete / Flags/ChangeSeed / Flags/Delete / Cosmetics/Clutter_Clone)
//! or to a **loop** slot (kinds 6/7/0xc/0xd/0x12/0x16/0x18/0x1c/0x1d/0x22/0x24),
//! collects distance records for kind 0xf, and frame-end drives the loops with
//! stop/start_if_not_playing pair toggling. The loop slot ↔ event-name
//! correspondence (native state +0x10/+0x38/+0x70/+0xb8/+0xf0/+0x128/+0x160/
//! +0x198/+0x1d0/+0x208/+0x240/+0x278/+0x2b0/+0x2d8/+0x310/+0x358) is 待对账.

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    systems::audio::roof_audio::instantiate_audio_w_position,
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

/// Decorator one-shot event by kind (O: disasm-0x14145dc20 string table;
/// each verified by the native string address + length. Kinds not listed are
/// either loop kinds (see [`kind_to_loop`]) or reserved).
pub fn decorator_one_shot(kind: u8) -> Option<&'static str> {
    Some(match kind {
        1 => "event:/SFX/WindowPlace",
        2 => "event:/SFX/WindowCombine",
        3 => "event:/SFX/WindowPlacePreview",
        4 => "event:/SFX/WindowDelete",
        5 => "event:/SFX/WindowRemove",
        8 => "event:/SFX/WindowChangeSeed",
        9 => "event:/SFX/WindowHover",
        10 => "event:/SFX/WindowSplit",
        0x10 => "event:/SFX/Lantern/Delete",
        0x14 => "event:/SFX/ChimneyBaseDisappear",
        0x17 => "event:/SFX/ChimneyDelete",
        0x19 => "event:/SFX/Flags/ChangeSeed",
        0x1e => "event:/SFX/Flags/Delete",
        0x1f => "event:/SFX/Cosmetics/Clutter_Clone",
        _ => return None,
    })
}

/// Map a `PlayDecoratorSoundCmd` loop kind to a loop index. The native
/// 0x14145dc20 jump table assigns each loop kind a state slot at
/// +0x10/+0x38/+0x70/+0xb8/+0xf0/+0x128/+0x160/+0x198/+0x1d0/+0x208/+0x240/
/// +0x278/+0x2b0/+0x2d8/+0x310/+0x358; the exact slot ↔ event-name
/// correspondence requires the full DecoratorAudioState layout and is 待对账.
/// The assignment below is the best-effort mapping kept from the pre-verification
/// code (window→0, lantern→2..7, chimney→8, flags→10..12, clutter→13).
fn kind_to_loop(kind: u8) -> Option<usize> {
    // O partial: loop kinds only (the one-shot kinds are handled first).
    match kind {
        0xc | 0xd => Some(8),
        6 => Some(1),
        7 => Some(2),
        0x12 => Some(4),
        0x16 => Some(13),
        0x18 => Some(10),
        0x1c | 0x1d => Some(12),
        0x22 => Some(14),
        0x24 => Some(9),
        _ => None,
    }
}

/// `decorator_audio_system` (0x14145dc20): dispatch each event to the verified
/// one-shot (kind→event) or loop (kind→loop slot), drive the flagged loops at
/// frame end (O: distance-sorted stop/start_if_not_playing toggling; the
/// distance collection for kind 0xf and the 3-nearest drive are 待对账).
pub fn decorator_audio_system<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<DecoratorAudioState<S::Event>>,
    mut events: EventReader<PlayDecoratorSoundCmd>,
    core: Res<AudioCore<S>>,
) {
    for event in events.read() {
        let position = [event.position.x, event.position.y, event.position.z];
        if let Some(name) = decorator_one_shot(event.kind) {
            // O: one-shot instantiate → set_position → start_if_not_playing.
            let _ = instantiate_audio_w_position(&core, name, position);
            continue;
        }
        let Some(index) = kind_to_loop(event.kind) else {
            continue;
        };
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
            let _ = loop_player.stop_now();
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

    #[test]
    fn decorator_one_shots_match_the_dispatch_string_table() {
        assert_eq!(decorator_one_shot(1), Some("event:/SFX/WindowPlace"));
        assert_eq!(decorator_one_shot(2), Some("event:/SFX/WindowCombine"));
        assert_eq!(decorator_one_shot(4), Some("event:/SFX/WindowDelete"));
        assert_eq!(decorator_one_shot(8), Some("event:/SFX/WindowChangeSeed"));
        assert_eq!(decorator_one_shot(0x17), Some("event:/SFX/ChimneyDelete"));
        assert_eq!(decorator_one_shot(0x19), Some("event:/SFX/Flags/ChangeSeed"));
        assert_eq!(decorator_one_shot(0x1e), Some("event:/SFX/Flags/Delete"));
    }
}
