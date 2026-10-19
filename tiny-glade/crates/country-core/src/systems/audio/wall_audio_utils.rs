//! B `wall_audio_utils.rs`: the wall loop player (`AudioLoopPlayer`).
//!
//! O-level facts (chapter wall_audio_utils.rs.md, DIGEST §2.5, decomp
//! 0x1409285a0): `AudioLoopPlayer::request_play` (0x1409285a0) sets the
//! should-play flag, then on the current instance does `set_position` →
//! `set_parameter("editing_speed")` → `set_volume`. The layout is
//! `{instances Vec, 参数 Vec, current usize, should_play u8}` (stride 0x40).
//!
//! The shared implementation lives in
//! [`crate::resources::audio_instance::AudioLoopPlayer`]; this module carries
//! the original file's contract and the wall-specific loop constants.

use crate::resources::audio_instance::AudioLoopPlayer;

/// The ten wall loop event names in init order (O: multi-baseline §3a).
pub const WALL_LOOP_EVENTS: [&str; 10] = [
    "event:/SFX/Wall Draw",
    "event:/SFX/Wall Change Height",
    "event:/SFX/FenceExtend",
    "event:/SFX/LintelExtend",
    "event:/SFX/StoneArchExtend",
    "event:/SFX/WideningGate",
    "event:/SFX/PlatformStairsExtend",
    "event:/SFX/SewerGateWiden",
    "event:/SFX/FenceHeight",
    "event:/SFX/FenceMove",
];

/// `AudioLoopPlayer::new` (native line 46, inlined): build from instances.
pub fn new_player<I>(instances: Vec<I>) -> AudioLoopPlayer<I>
where I: crate::resources::audio_instance::LoopInstance + crate::utils::audio_utils::LoopEntry<Error=<I as crate::resources::audio_instance::LoopInstance>::Error> {
    AudioLoopPlayer::new(instances)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wall_loop_names_are_in_init_order() {
        assert_eq!(WALL_LOOP_EVENTS[0], "event:/SFX/Wall Draw");
        assert_eq!(WALL_LOOP_EVENTS[1], "event:/SFX/Wall Change Height");
        assert_eq!(WALL_LOOP_EVENTS[9], "event:/SFX/FenceMove");
    }
}
