//! B `clutter_audio.rs`: clutter-placement sounds — 39-kind dispatch into
//! `event:/SFX/Cosmetics/*` one-shots.
//!
//! O-level facts (chapter clutter_audio.rs.md, decomp-0x1408c02f0): slot
//! stride 0x18, kind u8 @+0x14; jump table kind 0..0x26 (kind 0x12 is skipped).
//! Common block 0x1408c0990: instantiate_audio → set_position → set_parameter
//! → start_if_not_playing. The 39 kind→event mapping below is decoded from the
//! fresh decomp cases (each case loads a `.data (ptr,len)` into the common
//! block); the event names were rne-read from the string table.

use crate::resources::audio_resources::{AudioCore, Studio};
use crate::systems::audio::PlayClutterSoundCmd;
use bevy_ecs::prelude::*;

/// kind → `event:/SFX/Cosmetics/*` (O: decomp case table + rne-read). Kind
/// 0x12 has no entry (the native jump table skips it).
pub const CLUTTER_EVENTS: [&str; 39] = [
    "event:/SFX/Cosmetics/BirdsNest",               // 0x00
    "event:/SFX/Cosmetics/Box",                     // 0x01
    "event:/SFX/Cosmetics/BoxSack",                 // 0x02
    "event:/SFX/Cosmetics/BoxWaterCan",             // 0x03
    "event:/SFX/Cosmetics/Bucket",                  // 0x04
    "event:/SFX/Cosmetics/Cart",                    // 0x05
    "event:/SFX/Cosmetics/Chairs",                  // 0x06
    "event:/SFX/Cosmetics/ClayPot",                 // 0x07
    "event:/SFX/Cosmetics/FlowerPot",               // 0x08
    "event:/SFX/Cosmetics/Ladder",                  // 0x09
    "event:/SFX/Cosmetics/Wheel",                   // 0x0a
    "event:/SFX/Cosmetics/Firewood",                // 0x0b
    "event:/SFX/Cosmetics/Pitchfork",               // 0x0c
    "event:/SFX/Cosmetics/Shovel",                  // 0x0d
    "event:/SFX/Cosmetics/Shutters",                // 0x0e
    "event:/SFX/Cosmetics/ClothlineAppear",         // 0x0f
    "event:/SFX/Cosmetics/ClothlineConnectionAppear", // 0x10
    "event:/SFX/Cosmetics/WaterPlant",              // 0x11
    "",                                             // 0x12 skipped
    "event:/SFX/Cosmetics/Bench",                   // 0x13
    "event:/SFX/Cosmetics/Birdhouse",               // 0x14
    "event:/SFX/Cosmetics/PotatoSack",              // 0x15
    "event:/SFX/Cosmetics/RoadSign",                // 0x16
    "event:/SFX/Cosmetics/Barrel",                  // 0x17
    "event:/SFX/Cosmetics/Haystack",                // 0x18
    "event:/SFX/Cosmetics/Basket",                  // 0x19
    "event:/SFX/Cosmetics/Pumpkin",                 // 0x1a
    "event:/SFX/Cosmetics/PumpkinWithFace",         // 0x1b
    "event:/SFX/Cosmetics/OldenLog",                // 0x1c
    "event:/SFX/Cosmetics/OldenStump",              // 0x1d
    "event:/SFX/Cosmetics/OldenRock",               // 0x1e
    "event:/SFX/Cosmetics/WindowFlowerbed",         // 0x1f
    "event:/SFX/Cosmetics/Anvil",                   // 0x20
    "event:/SFX/Cosmetics/FabricStack",             // 0x21
    "event:/SFX/Cosmetics/BreadStall",              // 0x22
    "event:/SFX/Cosmetics/Veggiestall",             // 0x23
    "event:/SFX/Cosmetics/BoxFish",                 // 0x24
    "event:/SFX/Cosmetics/RackBrooms",              // 0x25
    "event:/SFX/Cosmetics/DoorBell",                // 0x26
];

/// `clutter_audio_system` (0x1408c02f0).
pub fn clutter_audio_system<S: Studio + Send + Sync + 'static>(
    mut events: EventReader<PlayClutterSoundCmd>,
    core: Res<AudioCore<S>>,
) {
    for event in events.read() {
        let kind = usize::from(event.kind);
        if kind >= CLUTTER_EVENTS.len() {
            continue; // O: kind > 0x26 out of bounds → skip.
        }
        let name = CLUTTER_EVENTS[kind];
        if name.is_empty() {
            continue; // O: kind 0x12 is skipped by the jump table.
        }
        let position = [event.position.x, event.position.y, event.position.z];
        // O: instantiate_audio → set_position → set_parameter → start_if_not_playing.
        let mut instance = core.instantiate_audio(name);
        let _ = instance.set_position(position);
        // O: the common block sets a parameter on the instance before start
        // (parameter name 待取证 — decomp shows impl$10::set_parameter with an
        // unresolved .data name).
        let _ = instance.set_parameter("editing_speed", 1.0);
        let _ = instance.start_if_not_playing();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clutter_kind_table_has_39_entries_and_kind_0x12_is_skipped() {
        assert_eq!(CLUTTER_EVENTS.len(), 0x27);
        assert!(CLUTTER_EVENTS[0x12].is_empty());
        assert!(CLUTTER_EVENTS[0x00].ends_with("BirdsNest"));
        assert!(CLUTTER_EVENTS[0x26].ends_with("DoorBell"));
    }
}
