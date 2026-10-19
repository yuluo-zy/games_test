//! B `playermade_stairs_audio.rs`: player-made stairs sounds — 6 loops +
//! kind ≤ 18 dispatch.
//!
//! O-level facts (chapter playermade_stairs_audio.rs.md, multi-baseline §5,
//! rne-read): init creates 6 loops — StairMove / StairChangeHeight /
//! StairChangeWidth / LadderMove / LadderChangeHeight / LadderChangeWidth.
//! `stairs_audio_system` frame-first stops all 6 loops, then dispatches
//! `PlayStairsAudioCmd` kind 0..18 (jump table 0x142bad9f0): loop kinds
//! {0,1,2,7,9,10} → `start_if_not_playing` + `set_parameter("stair_width")`;
//! one-shot kinds → instantiate + set_position + stair_width + start, with a
//! secondary parameter table (size / extend_amount / h_offset /
//! avg_arch_supports / height_above_ground / in_water / stair_edge_length /
//! is_stub_preview / move_all). Kind 16 ignored; 17/18 reuse StairSupportAppear.

use crate::{
    resources::{
        audio_instance::AudioLoopPlayer,
        audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    },
    systems::audio::roof_audio::instantiate_audio_w_position,
};
use crate::systems::audio::PlayStairsAudioCmd;
use bevy_ecs::prelude::*;

/// The six stairs loops (O: rne-read; init order).
pub const STAIRS_LOOP_EVENTS: [&str; 6] = [
    "event:/SFX/Stairs/StairMove",
    "event:/SFX/Stairs/StairChangeHeight",
    "event:/SFX/Stairs/StairChangeWidth",
    "event:/SFX/Stairs/LadderMove",
    "event:/SFX/Stairs/LadderChangeHeight",
    "event:/SFX/Stairs/LadderChangeWidth",
];

/// One-shot events by kind (O: multi-baseline §3a table; kind 16 = ignored).
pub const STAIRS_ONE_SHOTS: [&str; 16] = [
    "",                                             // 0  loop
    "",                                             // 1  loop
    "",                                             // 2  loop
    "event:/SFX/Stairs/StairPlace",                 // 3
    "event:/SFX/Stairs/StairDelete",                // 4
    "event:/SFX/Stairs/StairsMerge",                // 5
    "event:/SFX/Stairs/StairsMergeConfirm",         // 6
    "",                                             // 7  loop
    "event:/SFX/Stairs/LadderAppear",               // 8
    "",                                             // 9  loop
    "",                                             // 10 loop
    "event:/SFX/Stairs/LadderPlace",                // 11
    "event:/SFX/Stairs/StepsAppear",                // 12
    "event:/SFX/Stairs/WalkwayAppear",              // 13
    "event:/SFX/Stairs/StairSupportAppear",         // 14
    "event:/SFX/Stairs/StairPillarAppear",          // 15
];

/// Kind → loop index for the loop kinds {0,1,2,7,9,10}.
fn kind_to_loop(kind: u8) -> Option<usize> {
    match kind {
        0 => Some(0),
        1 => Some(1),
        2 => Some(2),
        7 => Some(3),
        9 => Some(4),
        10 => Some(5),
        _ => None,
    }
}

/// StairsAudioState (O: 6 loop players).
#[derive(Resource)]
pub struct StairsAudioState<E: EventInstance> {
    pub loops: [AudioLoopPlayer<AudioInstance<E>>; 6],
}

impl<E: EventInstance> StairsAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        StairsAudioState {
            loops: STAIRS_LOOP_EVENTS.map(|name| {
                core.instantiate_loop_player(&crate::resources::audio_resources::AudioLoopDesc::new(name))
            }),
        }
    }
}

/// `stairs_audio_init_system` (0x140f878d0).
pub fn stairs_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(StairsAudioState::<S::Event>::new(&core));
}

/// `stairs_audio_system` (0x140f880a0).
pub fn stairs_audio_system<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<StairsAudioState<S::Event>>,
    mut events: EventReader<PlayStairsAudioCmd>,
    core: Res<AudioCore<S>>,
) {
    // O: frame-first stop all six loops.
    for loop_player in state.loops.iter_mut() {
        let _ = loop_player.frame_end();
    }

    for event in events.read() {
        if event.kind > 18 {
            continue; // O: kind > 0x12 → skip.
        }
        let position = [event.position.x, event.position.y, event.position.z];
        let kind = event.kind as usize;
        if let Some(loop_index) = kind_to_loop(event.kind) {
            let _ = state.loops[loop_index].audio.current_mut()
                .set_parameter("stair_width", event.values[0]);
            let _ = state.loops[loop_index].frame_end();
            state.loops[loop_index].should_play = true;
        } else {
            // O: one-shot kinds; kind 17/18 reuse StairSupportAppear (14).
            let name = if kind == 17 || kind == 18 {
                STAIRS_ONE_SHOTS[14]
            } else {
                STAIRS_ONE_SHOTS.get(kind).copied().unwrap_or("")
            };
            if name.is_empty() {
                continue;
            }
            let _ = instantiate_audio_w_position(&core, name, position);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stairs_loop_names_are_original() {
        assert_eq!(STAIRS_LOOP_EVENTS[0], "event:/SFX/Stairs/StairMove");
        assert_eq!(STAIRS_LOOP_EVENTS[5], "event:/SFX/Stairs/LadderChangeWidth");
    }
}
