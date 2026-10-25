//! B `sheep_audio.rs`: sheep reaction sounds — kind 0..7 dispatch.
//!
//! O-level facts (chapter sheep_audio.rs.md, multi-baseline audio-extra §3e,
//! rne-read): `PlaySheepReactSound` slot stride 0x20, kind u8 @+0x18; jump
//! table 0x142ac2cc8 (kind 0–7). Kinds 0/1/2/3/5/6/7 instantiate one of
//! `event:/SFX/Sheep/{Aww,Think,Sob,Angry,Idea,Eyes,Confused}` + set_parameter
//! (FirstPersonOff) + start_if_not_playing; kind 4 sets the should-play flag
//! of a frame-end loop (no one-shot). Frame-end drives that loop by flag.
//!
//! DEEP-2026-10-17 (fresh disasm 0x1409d5ea0..0x1409d6201 + xrefs): the
//! "0x1b..0x51 大分派" once registered as 待补挖 is a **Ghidra function-merge
//! artifact** — the table entries 8..0x51 point into `signifier_audio_system`
//! (0x1409d6b90..) code, which is the same generic dispatch instantiated for
//! the signifier stream (`DAT_142ac2ce8`, byte-identical offsets, base+0x20).
//! `sheep_audio_system`'s own body handles only kinds 0..7; the Rust `_ => {}`
//! for kind ≥ 8 is behaviorally equivalent on the reachable producer range.

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    utils::audio_utils::AudioLoop,
};
use crate::systems::audio::PlaySheepReactSound;
use bevy_ecs::prelude::*;

/// Sheep reaction one-shots (O: rne-read; the chapter's kind-2 "So" is a
/// mis-read of "Sob" — the string table reads `event:/SFX/Sheep/Sob`).
pub const SHEEP_EVENTS: [&str; 7] = [
    "event:/SFX/Sheep/Aww",
    "event:/SFX/Sheep/Think",
    "event:/SFX/Sheep/Sob",
    "event:/SFX/Sheep/Angry",
    "event:/SFX/Sheep/Idea",
    "event:/SFX/Sheep/Eyes",
    "event:/SFX/Sheep/Confused",
];

/// Original inlined SheepAudioState constructors140b6a92c/1414d9b82 use
/// event name142ac2338 (20 bytes), separate from startup sheep's Baah event.
pub const KIND4_LOOP_EVENT: &str = "event:/SFX/Sheep/Pet";
/// Sheep reaction pitch parameter.
pub const SHEEP_PITCH_PARAM: &str = "Pitch";

/// SheepAudioState (O: the kind-4 frame-end loop player).
#[derive(Resource)]
pub struct SheepAudioState<E: EventInstance> {
    pub loop4: AudioLoop<AudioInstance<E>>,
    pub should_play4: bool,
}

impl<E: EventInstance> SheepAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        SheepAudioState {
            loop4: core.instantiate_loop(
                &crate::resources::audio_resources::AudioLoopDesc::new(KIND4_LOOP_EVENT),
            ),
            should_play4: false,
        }
    }
}

/// `sheep_audio_init_system` (O: init is inlined; state resource owns the loop).
pub fn sheep_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(SheepAudioState::<S::Event>::new(&core));
}

/// `sheep_audio_system` (0x1409d5ea0).
///
/// O (decomp-0x1409d5ea0 + disasm-0x1409d5ea0..0x1409d6201): kinds 0..3 →
/// Aww/Think/Sob/Angry, kind 4 → the continuous Baah loop, kinds 5..7 →
/// Idea/Eyes/Confused (kind 7 is Confused, not ignored). DEEP-2026-10-17:
/// the dispatch-table cases 8..0x51 belong to `signifier_audio_system`
/// (Ghidra merge artifact; see the module doc) and are unreachable for the
/// sheep event stream.
pub fn sheep_audio_system<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<SheepAudioState<S::Event>>,
    mut events: EventReader<PlaySheepReactSound>,
    core: Res<AudioCore<S>>,
) {
    let mut should_play4 = false;
    for event in events.read() {
        match event.kind {
            4 => {
                // O (0x1409d61b0): kind 4 only raises the frame-end loop flag;
                // it sets no parameter and plays no one-shot.
                should_play4 = true;
            }
            // O: kind 0..3 → Aww/Think/Sob/Angry, 5..7 → Idea/Eyes/Confused.
            kind if kind <= 3 => {
                let name = SHEEP_EVENTS[kind as usize];
                play_one_shot(&core, name, event);
            }
            kind if (5..=7).contains(&kind) => {
                let name = SHEEP_EVENTS[(kind - 1) as usize];
                play_one_shot(&core, name, event);
            }
            _ => {}
        }
    }
    state.should_play4 = should_play4;
    // Frame-end loop driving (O: 0x1409d61b0; flag → start_if_not_playing, else stop).
    if state.should_play4 {
        let _ = state.loop4.start_if_not_playing();
    } else {
        let _ = state.loop4.stop_now();
    }
}

/// One-shot path (O: 0x1409d60d4-0x1409d617d): instantiate →
/// `set_position` (payload +8) → `set_parameter("Pitch", pitch@+0x14)` →
/// `start_if_not_playing` → drop.
fn play_one_shot<S: Studio>(core: &AudioCore<S>, name: &str, event: &PlaySheepReactSound) {
    let mut instance = core.instantiate_audio(name);
    let _ = instance.set_position([event.position.x, event.position.y, event.position.z]);
    let _ = instance.set_parameter(SHEEP_PITCH_PARAM, event.pitch);
    let _ = instance.start_if_not_playing();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sheep_event_names_match_the_string_table() {
        assert_eq!(SHEEP_EVENTS[0], "event:/SFX/Sheep/Aww");
        assert_eq!(SHEEP_EVENTS[1], "event:/SFX/Sheep/Think");
        assert_eq!(SHEEP_EVENTS[2], "event:/SFX/Sheep/Sob");
        assert_eq!(SHEEP_EVENTS[3], "event:/SFX/Sheep/Angry");
        assert_eq!(SHEEP_EVENTS[4], "event:/SFX/Sheep/Idea");
        assert_eq!(SHEEP_EVENTS[5], "event:/SFX/Sheep/Eyes");
        assert_eq!(SHEEP_EVENTS[6], "event:/SFX/Sheep/Confused");
    }
}







