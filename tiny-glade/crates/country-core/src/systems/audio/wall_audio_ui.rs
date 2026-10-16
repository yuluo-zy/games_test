//! B `wall_audio_ui.rs`: wall UI sounds — `PlayWallUiSound` kind dispatch into
//! `instantiate_audio_w_position` one-shots.
//!
//! O-level facts (chapter wall_audio_ui.rs.md, decomp-0x1408b7690, rne-read):
//! the switch reads kind @+0x8. Fresh-decomp cases that build event strings:
//! kind 6 → `event:/SFX/ColorBricks`, 0xd → `event:/SFX/ColorLantern`,
//! 0xe → `event:/SFX/Lantern/TurnOn`, 0x10 → `event:/SFX/ChangeWallStyle`; the
//! chapter additionally lists Chimney/ToggleSmokeOn|Off (selected by the event
//! +0x9 byte), BuildingSupportsRemoveWooden, WallSplit. The exact remaining
//! kind→event mapping is 待取证.

use crate::{
    resources::audio_resources::{AudioCore, Studio},
    systems::audio::roof_audio::instantiate_audio_w_position,
};
use crate::systems::audio::PlayWallUiSound;
use bevy_ecs::prelude::*;

/// Verified kind → event strings (O: decomp cases + rne-read).
pub const WALL_UI_EVENTS: [(&str, &str); 4] = [
    ("ColorBricks", "event:/SFX/ColorBricks"),           // kind 6
    ("ColorLantern", "event:/SFX/ColorLantern"),         // kind 0xd
    ("LanternTurnOn", "event:/SFX/Lantern/TurnOn"),      // kind 0xe
    ("ChangeWallStyle", "event:/SFX/ChangeWallStyle"),   // kind 0x10
];

/// Additional events named by the chapter (O string table) whose kinds are
/// 待取证.
pub const WALL_UI_EXTRA_EVENTS: [&str; 3] = [
    "event:/SFX/Chimney/ToggleSmokeOn",
    "event:/SFX/Chimney/ToggleSmokeOff",
    "event:/SFX/BuildingSupportsRemoveWooden",
];

/// `wall_audio_ui` (0x1408b7690): dispatch `PlayWallUiSound` to positioned
/// one-shots. The verified kinds below are from the fresh decomp; the rest are
/// 待取证.
pub fn wall_audio_ui<S: Studio + Send + Sync + 'static>(
    mut events: EventReader<PlayWallUiSound>,
    core: Res<AudioCore<S>>,
) {
    for event in events.read() {
        let position = [event.position.x, event.position.y, event.position.z];
        let name = match event.kind {
            6 => WALL_UI_EVENTS[0].1,
            0xd => WALL_UI_EVENTS[1].1,
            0xe => {
                // O: Lantern/TurnOn.
                WALL_UI_EVENTS[2].1
            }
            0x10 => WALL_UI_EVENTS[3].1,
            // 待取证: remaining kinds (chapter lists the ToggleSmoke /
            // SupportsRemoveWooden / WallSplit names without kind numbers).
            _ => continue,
        };
        let _ = instantiate_audio_w_position(&core, name, position);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verified_wall_ui_events_are_original() {
        assert_eq!(WALL_UI_EVENTS[0].1, "event:/SFX/ColorBricks");
        assert_eq!(WALL_UI_EVENTS[1].1, "event:/SFX/ColorLantern");
        assert_eq!(WALL_UI_EVENTS[2].1, "event:/SFX/Lantern/TurnOn");
        assert_eq!(WALL_UI_EVENTS[3].1, "event:/SFX/ChangeWallStyle");
    }
}
