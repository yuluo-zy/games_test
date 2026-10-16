//! B `wall_audio_path.rs`: wall path-draw loop driving.
//!
//! O-level facts (chapter wall_audio_path.rs.md, decomp-0x140a89bb0): the
//! decomp shows the event/log write path (`FUN_140562c30` at `+0x20`) but the
//! full loop should-play/parameter chain is U. The system drives the wall
//! "Wall Draw" loop (loop 0) while a wall path is being drawn. 待取证: exact
//! event source and parameter wiring.

use crate::{
    resources::audio_resources::{AudioCore, Studio},
    systems::audio::wall_audio_fmod::WallAudioState,
};
use bevy_ecs::prelude::*;

/// `wall_audio_path` (0x140a89bb0): while `path_active` is set, request the
/// wall-draw loop; otherwise frame-end the loop player.
pub fn wall_audio_path<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<WallAudioState<S::Event>>,
    _core: Res<AudioCore<S>>,
) {
    // O (partial): path drawing sets the wall-draw loop's should-play flag.
    // The producer's exact flag is 待取证; loop 0 is the "Wall Draw" player.
    // Frame-end drives start/stop by that flag.
    let _ = state.loops[0].frame_end();
}
