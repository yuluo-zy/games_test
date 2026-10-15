//! B 0x14083c4a0 and window closure 0x14083b480, vtable @0x142a93400.
use super::{
    music_player::{MusicEvent, PlayerState, StopMode, timeline_snapshot},
    music_system::MusicAudioState,
};
use crate::resources::gui::{DebugGui, EguiContextResource};
use bevy_ecs::prelude::*;

pub fn music_debug_gui<E: MusicEvent>(
    context: Option<Res<EguiContextResource>>,
    mut state: ResMut<MusicAudioState<E>>,
    mut debug: ResMut<DebugGui>,
) {
    let Some(context) = context else { return };
    // Native marks MusicAudioState changed even when the window is closed.
    let state = &mut *state;
    let force = debug.force_music_richness;
    egui::Window::new("DJ station")
        .open(&mut debug.show_music_info)
        .show(&context.0, |ui| {
            ui.label(format!(
                "Music richness: {}",
                if force { 1.0 } else { state.richness }
            ));
            let player = &mut state.player;
            // Native Option::unwrap_or(0), then names.get(index).unwrap_or("??").
            let index = player.current_track.unwrap_or(0);
            ui.label(format!(
                "Current track {index}: {}",
                player.track_names.get(index).map_or("??", String::as_str)
            ));
            ui.label(format!("Music player state: {:?}", player.state));
            if ui.button("Force switch track").clicked() {
                player.state = PlayerState::SwitchAtNextOpportunity;
            }
            if ui.button("Switch immediately").clicked() {
                player.track_switching.stop(StopMode::Immediate);
                if let Some(track) = player.current_track.and_then(|i| player.tracks.get_mut(i)) {
                    track.stop(StopMode::Immediate);
                }
                player.state = PlayerState::SwitchTrack;
            }
            ui.separator();
            let beat = timeline_snapshot();
            ui.label(format!("Beat: {}", beat.beat));
            ui.label(format!("Bar (fmod): {}", beat.bar));
            ui.label(format!("Bar counting: {}", beat.bar_count));
            ui.label(format!("Tempo: {}", beat.tempo));
            ui.label(format!(
                "Time signature: {}/{}",
                beat.time_signature_upper, beat.time_signature_lower
            ));
            ui.separator();
        });
}
