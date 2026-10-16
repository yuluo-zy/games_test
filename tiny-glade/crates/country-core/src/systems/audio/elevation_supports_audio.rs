//! B `elevation_supports_audio.rs` 0x140ad51c0: elevated-shape supports
//! (wooden brackets / brick / wooden columns) one-shot placement sounds.
//!
//! O-level facts (chapter elevation_supports_audio.rs.md, audio-misc DIGEST
//! §2.13): `PlayElevationSupportSoundCmd`'s kind byte selects one of three
//! one-shot event names (rne-read: `event:/SFX/Shapes/WoodenBracketsAppear` /
//! `BrickColumnAppear` / `WoodenColumnAppear`) → instantiate_audio + start.
//! No loops, no frame-end dispatch — pure kind → one-shot.

use crate::{
    resources::audio_resources::{AudioCore, Studio},
    systems::audio::roof_audio::instantiate_audio_w_position_and_volume,
};
use crate::systems::audio::PlayElevationSupportSoundCmd;
use bevy_ecs::prelude::*;

/// The three support one-shot events (O: rne-read, contiguous .data).
pub const SUPPORT_EVENTS: [&str; 3] = [
    "event:/SFX/Shapes/WoodenBracketsAppear",
    "event:/SFX/Shapes/BrickColumnAppear",
    "event:/SFX/Shapes/WoodenColumnAppear",
];

/// `elevation_supports_audio` (0x140ad51c0): kind → one of the three
/// support events, played at the event position. Kind ≥ 3 clamps to the last
/// event (native jump-table bounds beyond the 3-entry table are not reached in
/// the original producers; 待取证 for exact kind→name order).
pub fn elevation_supports_audio<S: Studio + Send + Sync + 'static>(
    mut events: EventReader<PlayElevationSupportSoundCmd>,
    core: Res<AudioCore<S>>,
) {
    for event in events.read() {
        let index = usize::from(event.kind).min(SUPPORT_EVENTS.len() - 1);
        let name = SUPPORT_EVENTS[index];
        let position = [event.position.x, event.position.y, event.position.z];
        // O: `roof_audio::instantiate_audio_w_position` (0x140aa3360) which
        // instantiates, positions and starts the one-shot.
        let _ = instantiate_audio_w_position_and_volume(&core, name, position, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn support_event_names_match_the_original_string_table() {
        assert_eq!(SUPPORT_EVENTS[0], "event:/SFX/Shapes/WoodenBracketsAppear");
        assert_eq!(SUPPORT_EVENTS[1], "event:/SFX/Shapes/BrickColumnAppear");
        assert_eq!(SUPPORT_EVENTS[2], "event:/SFX/Shapes/WoodenColumnAppear");
    }
}
