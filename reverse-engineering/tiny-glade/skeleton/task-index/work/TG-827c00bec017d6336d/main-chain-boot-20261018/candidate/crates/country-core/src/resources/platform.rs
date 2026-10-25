//! B RuntimePlatform is one byte (insert_resource @0x14013a1d0).
//! run_game @0x140594c0d..4cc5 first computes 2 - GFN_is_running,
//! then overrides it with 0 when the actual Steam SDK reports Steam Deck.
//! Labels below describe these observed branches; original enum spelling is
//! unavailable. The explicit local macOS host has no GFN SDK and is desktop.

#[derive(bevy_ecs::resource::Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuntimePlatform(pub u8);

impl RuntimePlatform {
    pub fn from_detection(geforce_now: bool, steam_deck: bool) -> Self {
        Self(if steam_deck {
            0
        } else {
            2 - u8::from(geforce_now)
        })
    }

    pub fn native_tag(self) -> u8 {
        self.0
    }
}
