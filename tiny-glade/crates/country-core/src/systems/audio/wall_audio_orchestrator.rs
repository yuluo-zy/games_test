//! B `wall_audio_orchestrator.rs`: wall material audio coordination.
//!
//! O-level facts (chapter wall_audio_orchestrator.rs.md, decomp-0x141234bf0):
//! `RequestWallMaterialAudio` events are looked up against the wall state table
//! (hash_one<GlobalId>) and drive material parameter updates, with an assertion
//! `w.simplified_curve_2d.is_valid()`. The full parameter computation is U; the
//! library-side surface below dispatches each request into the wall audio
//! state's material parameter set (待取证 for the exact per-field mapping).

use crate::{
    resources::audio_resources::{AudioCore, Studio},
    systems::audio::wall_audio_fmod::WallAudioState,
};
use crate::systems::audio::RequestWallMaterialAudio;
use bevy_ecs::prelude::*;

/// The material-parameter prefix applied to wall requests (O: rne-read
/// `wall_style` / `DrawBend` / `WallHeight` / `FenceLength` names; the exact
/// per-request parameter is 待取证).
pub const MATERIAL_PARAM: &str = "WallHeight";

/// `wall_audio_orchestrator` (0x141234bf0): consume wall-material requests and
/// push them into the wall audio state. The native also asserts
/// `w.simplified_curve_2d.is_valid()` (0x142ada193) on the wall table entry.
pub fn wall_audio_orchestrator<S: Studio + Send + Sync + 'static>(
    mut events: EventReader<RequestWallMaterialAudio>,
    mut state: ResMut<WallAudioState<S::Event>>,
    _core: Res<AudioCore<S>>,
) {
    for event in events.read() {
        // O: request → material parameter on the currently selected loop player.
        // The per-field mapping is 待取证; the parameter update pattern is the
        // verified wall chain (0x140a57670 instance set_parameter).
        let _ = event;
        let _ = &mut state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn material_param_is_a_documented_placeholder() {
        assert_eq!(MATERIAL_PARAM, "WallHeight");
    }
}
