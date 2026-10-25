//! B `wall_audio_ui.rs`: wall UI sounds — `PlayWallUiSound` kind dispatch into
//! positioned one-shots (with the parameterized kinds 6/0xd/0xe/0x10).
//!
//! Rebuilt from fresh disasm (0x1408b7690, jump table 0x142a9fa44, radare2
//! continuation 0x1408b7b40+; REBUILD-2026-10-17). Native event layout:
//! `{kind u8 @+8, selector byte @+9, value f32 @+0xc, position Vec3 @+0x10}`,
//! stride 0x20. The jump table decodes as:
//! - kind 0: global `StructureSize` param = value@+0xc, then plays
//!   `event:/SFX/Bulldoze` (0x142a9f710/0x142a9f71d).
//! - kind 1: the default event `event:/SFX/WallSnapToEnd` (0x142a9f730).
//! - kinds 2..8, 0xa..0xc, 0x13/0x14: `WALL_UI_KINDS` (verified names).
//! - kind 6 (ColorBricks): + `isHalfTimbered` = selector != 0 → 1.0/0.0
//!   (0x142a9f810).
//! - kind 9: `event:/SFX/Flags/ToggleDisconnect|ToggleConnect` by selector
//!   (0x142a9f846/0x142a9f867) — **not** Chimney smoke.
//! - kind 0xd (ColorLantern): + `LanternColor` = value@+0xc (0x142a9f8f8).
//! - kind 0xe (Lantern/TurnOn): + `BrightnessValue` = value@+0xc (0x142a9f938).
//! - kind 0xf: `event:/SFX/Chimney/ToggleSmokeOff|On` by selector
//!   (0x142a9f947/0x142a9f968).
//! - kind 0x10 (ChangeWallStyle): + `wall_style` = `WALL_STYLE_VALUES[selector]`
//!   (0x142a9f9c0, table 0x142a9fa08 = {0.0, 1.0, 2.0}).
//! - kinds 0x11/0x12 (and any unlisted kind): `event:/SFX/WallSplit`
//!   (0x142a9f9ca, the jump-table default target).
//! - kinds 0x15/0x16: the table entries point into `.data` (original bad
//!   entries) — skipped here.

use crate::{
    resources::audio_resources::{AudioCore, Studio},
    systems::audio::{roof_audio::instantiate_audio_w_position, PlayWallUiSound},
};
use bevy_ecs::prelude::*;

/// kind → one-shot event (O: jump table 0x142a9fa44 + rne-read). Empty = handled
/// by the parameterized cases below (0/1/6/9/0xd/0xe/0xf/0x10/0x11/0x12/0x15/0x16).
pub const WALL_UI_KINDS: [&str; 0x17] = [
    "",                                             // 0   StructureSize param + Bulldoze
    "",                                             // 1   default WallSnapToEnd
    "event:/SFX/WallSnapToSegment",                 // 2
    "event:/SFX/ShapeSnap",                         // 3
    "event:/SFX/WallsMerged",                       // 4
    "event:/SFX/WallsMergedEnclosed",               // 5
    "",                                             // 6   ColorBricks + isHalfTimbered
    "event:/SFX/ColorGate",                         // 7
    "event:/SFX/ColorFlag",                         // 8
    "",                                             // 9   Flags ToggleDisconnect/Connect
    "event:/SFX/ColorWindow",                       // 0xa
    "event:/SFX/ColorRoof",                         // 0xb
    "event:/SFX/ColorHalftimber",                   // 0xc
    "",                                             // 0xd ColorLantern + LanternColor
    "",                                             // 0xe Lantern/TurnOn + BrightnessValue
    "",                                             // 0xf Chimney ToggleSmoke
    "",                                             // 0x10 ChangeWallStyle + wall_style
    "",                                             // 0x11 WallSplit (default target)
    "",                                             // 0x12 WallSplit
    "event:/SFX/BuildingSupportsRemoveWooden",      // 0x13
    "event:/SFX/BuildingSupportsRemoveWooden",      // 0x14
    "",                                             // 0x15 bad table entry (original)
    "",                                             // 0x16 bad table entry (original)
];

/// kind 0 global parameter (O: 0x142a9f710, len 0xd).
pub const STRUCTURE_SIZE_PARAM: &str = "StructureSize";
/// kind 0 one-shot (O: 0x142a9f71d, len 0x13).
pub const BULLDOZE_EVENT: &str = "event:/SFX/Bulldoze";
/// kind 1 default event (O: 0x142a9f730, len 0x18).
pub const WALL_SNAP_TO_END_EVENT: &str = "event:/SFX/WallSnapToEnd";
/// kind 9 selectors (O: 0x142a9f846 / 0x142a9f867).
pub const FLAGS_TOGGLE_DISCONNECT: &str = "event:/SFX/Flags/ToggleDisconnect";
pub const FLAGS_TOGGLE_CONNECT: &str = "event:/SFX/Flags/ToggleConnect";
/// kind 0xf selectors (O: 0x142a9f947 / 0x142a9f968).
pub const CHIMNEY_SMOKE_ON: &str = "event:/SFX/Chimney/ToggleSmokeOn";
pub const CHIMNEY_SMOKE_OFF: &str = "event:/SFX/Chimney/ToggleSmokeOff";
/// kinds 0x11/0x12 default target (O: 0x142a9f9ca, len 0x14).
pub const WALL_SPLIT_EVENT: &str = "event:/SFX/WallSplit";
/// kind 6 parameter (O: 0x142a9f810, len 0xe).
pub const IS_HALF_TIMBERED_PARAM: &str = "isHalfTimbered";
/// kind 0xd parameter (O: 0x142a9f8f8, len 0xc).
pub const LANTERN_COLOR_PARAM: &str = "LanternColor";
/// kind 0xe parameter (O: 0x142a9f938, len 0xf).
pub const BRIGHTNESS_VALUE_PARAM: &str = "BrightnessValue";
/// kind 0x10 parameter (O: 0x142a9f9c0, len 0xa).
pub const WALL_STYLE_PARAM: &str = "wall_style";
/// kind 0x10 selector value table (O: 0x142a9fa08 = {0.0, 1.0, 2.0}).
pub const WALL_STYLE_VALUES: [f32; 3] = [0.0, 1.0, 2.0];

/// `wall_audio_ui` (0x1408b7690): dispatch `PlayWallUiSound` to positioned
/// one-shots (parameterized kinds instantiate and set an instance parameter).
pub fn wall_audio_ui<S: Studio + Send + Sync + 'static>(
    mut events: EventReader<PlayWallUiSound>,
    core: Res<AudioCore<S>>,
) {
    for event in events.read() {
        let position = [event.position.x, event.position.y, event.position.z];
        match event.kind {
            // O: kind 0 sets the global "StructureSize" parameter to value@+0xc,
            // then plays "event:/SFX/Bulldoze".
            0 => {
                let _ = core.set_parameter(STRUCTURE_SIZE_PARAM, event.value);
                let _ = instantiate_audio_w_position(&core, BULLDOZE_EVENT, position);
            }
            // O: kind 6 instantiates ColorBricks and sets isHalfTimbered.
            6 => {
                let mut instance = core.instantiate_audio("event:/SFX/ColorBricks");
                let _ = instance.set_position(position);
                let timbered = if event.selector != 0 { 1.0 } else { 0.0 };
                let _ = instance.set_parameter(IS_HALF_TIMBERED_PARAM, timbered);
                let _ = instance.start_if_not_playing();
            }
            // O: kind 9 selects Flags ToggleDisconnect/Connect by the byte@+9.
            9 => {
                let name = if event.selector != 0 {
                    FLAGS_TOGGLE_CONNECT
                } else {
                    FLAGS_TOGGLE_DISCONNECT
                };
                let _ = instantiate_audio_w_position(&core, name, position);
            }
            // O: kind 0xd instantiates ColorLantern and sets LanternColor.
            0xd => {
                let mut instance = core.instantiate_audio("event:/SFX/ColorLantern");
                let _ = instance.set_position(position);
                // O (disasm 0x1408b7d25): `mov eax, esi; cvtsi2ss xmm3, rax`
                // — the raw 32-bit value bits are reinterpreted as an unsigned
                // index and converted to f32 (not passed through as a float).
                let lantern_color = event.value.to_bits() as f32;
                let _ = instance.set_parameter(LANTERN_COLOR_PARAM, lantern_color);
                let _ = instance.start_if_not_playing();
            }
            // O: kind 0xe instantiates Lantern/TurnOn and sets BrightnessValue.
            0xe => {
                let mut instance = core.instantiate_audio("event:/SFX/Lantern/TurnOn");
                let _ = instance.set_position(position);
                let _ = instance.set_parameter(BRIGHTNESS_VALUE_PARAM, event.value);
                let _ = instance.start_if_not_playing();
            }
            // O: kind 0xf selects Chimney ToggleSmokeOff/On by the byte@+9.
            0xf => {
                let name = if event.selector != 0 {
                    CHIMNEY_SMOKE_ON
                } else {
                    CHIMNEY_SMOKE_OFF
                };
                let _ = instantiate_audio_w_position(&core, name, position);
            }
            // O: kind 0x10 instantiates ChangeWallStyle and sets wall_style from
            // the selector value table {0.0, 1.0, 2.0}.
            0x10 => {
                let mut instance = core.instantiate_audio("event:/SFX/ChangeWallStyle");
                let _ = instance.set_position(position);
                let style = WALL_STYLE_VALUES
                    .get(event.selector as usize)
                    .copied()
                    .unwrap_or(0.0);
                let _ = instance.set_parameter(WALL_STYLE_PARAM, style);
                let _ = instance.start_if_not_playing();
            }
            kind => {
                let name = match kind {
                    1 => WALL_SNAP_TO_END_EVENT,
                    // O: kinds 0x11/0x12 (and any unlisted kind) reach the
                    // jump-table default target "event:/SFX/WallSplit".
                    0x11 | 0x12 => WALL_SPLIT_EVENT,
                    _ => {
                        let index = kind as usize;
                        if index >= WALL_UI_KINDS.len() {
                            continue;
                        }
                        WALL_UI_KINDS[index]
                    }
                };
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
        assert_eq!(WALL_UI_KINDS[0xa], "event:/SFX/ColorWindow");
        assert_eq!(WALL_UI_KINDS[0xc], "event:/SFX/ColorHalftimber");
        assert_eq!(WALL_UI_KINDS[0x13], "event:/SFX/BuildingSupportsRemoveWooden");
    }

    #[test]
    fn corrected_kinds_match_the_fresh_disasm_strings() {
        assert_eq!(STRUCTURE_SIZE_PARAM, "StructureSize");
        assert_eq!(BULLDOZE_EVENT, "event:/SFX/Bulldoze");
        assert_eq!(WALL_SNAP_TO_END_EVENT, "event:/SFX/WallSnapToEnd");
        assert_eq!(FLAGS_TOGGLE_DISCONNECT, "event:/SFX/Flags/ToggleDisconnect");
        assert_eq!(FLAGS_TOGGLE_CONNECT, "event:/SFX/Flags/ToggleConnect");
        assert_eq!(WALL_SPLIT_EVENT, "event:/SFX/WallSplit");
        assert_eq!(IS_HALF_TIMBERED_PARAM, "isHalfTimbered");
        assert_eq!(LANTERN_COLOR_PARAM, "LanternColor");
        assert_eq!(BRIGHTNESS_VALUE_PARAM, "BrightnessValue");
        assert_eq!(WALL_STYLE_PARAM, "wall_style");
        assert_eq!(WALL_STYLE_VALUES, [0.0, 1.0, 2.0]);
    }
}
