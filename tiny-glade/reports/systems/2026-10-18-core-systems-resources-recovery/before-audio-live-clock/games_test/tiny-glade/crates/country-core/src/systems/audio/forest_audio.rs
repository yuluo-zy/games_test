//! B `forest_audio.rs`: forest-growth sound — a single `event:/SFX/ForestGrow`
//! loop driven by the presence of interactable forest trees.
//!
//! O-level facts (chapter forest_audio.rs.md, procs.tsv signature):
//! - `forest_audio_init_system` 0x140aa18f0: 1 × `instantiate_loop`
//!   (`event:/SFX/ForestGrow`).
//! - `forest_audio_system` 0x140aa1f00:
//!   `(ResMut<ForestAudioState>, Query<(&InteractableForestTree, &PrefabInstanceData)>, Res<EscMenuState>)`
//!   — drives the single loop start_if_not_playing / stop by tree presence and
//!   the menu gate, with set_position following the nearest tree.
//!
//! `InteractableForestTree` / `PrefabInstanceData` belong to the tree_storage /
//! prefab domains (未建); the query target is modeled here as a local
//! [`ForestTree`] marker so the FMOD/loop business is library-complete. 待接线:
//! substitute the real tree_storage component when that domain lands.

use crate::{
    resources::{
        audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
        gui::EscMenuState,
    },
    utils::audio_utils::AudioLoop,
};
use bevy_ecs::prelude::*;
use glam::Vec3;

/// The forest-growth loop event (O: rne-read).
pub const FOREST_GROW_EVENT: &str = "event:/SFX/ForestGrow";

/// Placeholder for `tree_storage::InteractableForestTree` (待接线).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ForestTree {
    pub position: Vec3,
}

/// ForestAudioState: the single grow loop.
#[derive(Resource)]
pub struct ForestAudioState<E: EventInstance> {
    pub loop_player: AudioLoop<AudioInstance<E>>,
}

impl<E: EventInstance> ForestAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        ForestAudioState {
            loop_player: core.instantiate_loop(
                &crate::resources::audio_resources::AudioLoopDesc::new(FOREST_GROW_EVENT),
            ),
        }
    }
}

/// `forest_audio_init_system` (0x140aa18f0).
pub fn forest_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(ForestAudioState::<S::Event>::new(&core));
}

/// `forest_audio_system` (0x140aa1f00): play the grow loop while interactable
/// forest trees exist, follow the first tree's position, and stop when the
/// escape menu is open.
pub fn forest_audio_system<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<ForestAudioState<S::Event>>,
    trees: Query<&ForestTree>,
    menu: Res<EscMenuState>,
) {
    // O: the native queries &InteractableForestTree; menu gate stops the loop.
    let playing = !menu.is_open() && trees.iter().next().is_some();
    if playing {
        let _ = state.loop_player.start_if_not_playing();
        if let Some(tree) = trees.iter().next() {
            let position = [tree.position.x, tree.position.y, tree.position.z];
            // set_position on the current entry (0x140a57da0).
            let cursor = state.loop_player.cursor;
            let current = &mut state.loop_player.entries[cursor];
            let _ = current.set_position(position);
        }
    } else {
        let _ = state.loop_player.stop(std::time::Duration::ZERO);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forest_grow_event_name_is_original() {
        assert_eq!(FOREST_GROW_EVENT, "event:/SFX/ForestGrow");
    }
}
