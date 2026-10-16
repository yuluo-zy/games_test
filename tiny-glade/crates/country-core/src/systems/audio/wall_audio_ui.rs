//! B `wall_audio_ui.rs`: wall UI sounds — `PlayWallUiSound` kind dispatch into
//! `instantiate_audio_w_position` one-shots.
//!
//! O-level facts (chapter wall_audio_ui.rs.md, decomp-0x1408b7690, jump table
//! 0x142a9fa44, rne-read): slot stride 0x28, kind @+8. The jump table maps
//! kinds 0..0x16; the verified kind→event table below is decoded from the
//! case targets + `.data` strings. Kinds 9 / 0xf read the event byte @+9 to
//! select `Chimney/ToggleSmokeOn|Off`. Kind 0 sets the global `wall_style`
//! parameter (AudioCoreInner::set_parameter). Kinds 0x11/0x12/0x15/0x16 skip.

use crate::{
    resources::audio_resources::{AudioCore, Studio},
    systems::audio::{roof_audio::instantiate_audio_w_position, PlayWallUiSound},
};
use bevy_ecs::prelude::*;

/// kind → one-shot event (O: jump table 0x142a9fa44 + rne-read). Empty = skip.
pub const WALL_UI_KINDS: [&str; 0x17] = [
    "",                                             // 0   wall_style global param
    "",                                             // 1   pre-set common event
    "event:/SFX/WallSnapToSegment",                 // 2
    "event:/SFX/ShapeSnap",                         // 3
    "event:/SFX/WallsMerged",                       // 4
    "event:/SFX/WallsMergedEnclosed",               // 5
    "event:/SFX/ColorBricks",                       // 6
    "event:/SFX/ColorGate",                         // 7
    "event:/SFX/ColorFlag",                         // 8
    "",                                             // 9   ToggleSmoke selector
    "event:/SFX/ColorWindow",                       // 0xa
    "event:/SFX/ColorRoof",                         // 0xb
    "event:/SFX/ColorHalftimber",                   // 0xc
    "event:/SFX/ColorLantern",                      // 0xd
    "event:/SFX/Lantern/TurnOn",                    // 0xe
    "",                                             // 0xf ToggleSmoke selector
    "event:/SFX/ChangeWallStyle",                   // 0x10
    "",                                             // 0x11 skip
    "",                                             // 0x12 skip
    "event:/SFX/BuildingSupportsRemoveWooden",      // 0x13
    "event:/SFX/BuildingSupportsRemoveWooden",      // 0x14
    "",                                             // 0x15 skip
    "",                                             // 0x16 skip
];

/// Chimney smoke events (O: rne-read; kind 9/0xf select by the event +0x9 byte).
pub const CHIMNEY_SMOKE_ON: &str = "event:/SFX/Chimney/ToggleSmokeOn";
pub const CHIMNEY_SMOKE_OFF: &str = "event:/SFX/Chimney/ToggleSmokeOff";
/// `wall_style` global parameter (O: 0x142a9f9c0, kind 0 path).
pub const WALL_STYLE_PARAM: &str = "wall_style";

/// `wall_audio_ui` (0x1408b7690): dispatch `PlayWallUiSound` to positioned
/// one-shots.
pub fn wall_audio_ui<S: Studio + Send + Sync + 'static>(
    mut events: EventReader<PlayWallUiSound>,
    core: Res<AudioCore<S>>,
) {
    for event in events.read() {
        let position = [event.position.x, event.position.y, event.position.z];
        match event.kind {
            // O: kind 0 sets the wall_style global parameter (value from the
            // event +0xc scalar, 待取证 for the exact mapping).
            0 => {
                let _ = core.set_parameter(WALL_STYLE_PARAM, f32::from(event.flag));
            }
            // O: kinds 9/0xf select ToggleSmokeOn/Off by the event +0x9 byte.
            9 | 0xf => {
                let name = if event.flag != 0 {
                    CHIMNEY_SMOKE_ON
                } else {
                    CHIMNEY_SMOKE_OFF
                };
                let _ = instantiate_audio_w_position(&core, name, position);
            }
            kind => {
                let index = kind as usize;
                if index >= WALL_UI_KINDS.len() {
                    continue;
                }
                let name = WALL_UI_KINDS[index];
                if name.is_empty() {
                    continue;
                }
                let _ = instantiate_audio_w_position(&core, name, position);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wall_ui_kind_table_matches_the_jump_table() {
        assert_eq!(WALL_UI_KINDS[2], "event:/SFX/WallSnapToSegment");
        assert_eq!(WALL_UI_KINDS[4], "event:/SFX/WallsMerged");
        assert_eq!(WALL_UI_KINDS[7], "event:/SFX/ColorGate");
        assert_eq!(WALL_UI_KINDS[0xd], "event:/SFX/ColorLantern");
        assert_eq!(WALL_UI_KINDS[0xe], "event:/SFX/Lantern/TurnOn");
        assert_eq!(WALL_UI_KINDS[0x10], "event:/SFX/ChangeWallStyle");
        assert_eq!(WALL_UI_KINDS[0x13], "event:/SFX/BuildingSupportsRemoveWooden");
    }
}
