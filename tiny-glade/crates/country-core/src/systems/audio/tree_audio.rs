//! B `tree_audio.rs`: tree edit / ambience sounds — 4 loops + TreePlace /
//! TreeDelete one-shots.
//!
//! O-level facts (chapter tree_audio.rs.md, decomp-0x1414633e0/0x1414638e0,
//! rne-read):
//! - init: 4 × `instantiate_loop` — `event:/SFX/Tree/TreeMove`, then three
//!   `event:/SFX/Tree/Amb` (decomp shows the 2nd–4th loop names all resolving
//!   to `event:/SFX/Tree/Amb`; the chapter's TreePlace/TreeGizmo events are the
//!   one-shot paths, not loops).
//! - system: `Query<&TreeAmbience>` → `Vec<(dist, pos)>` stable-sorted by
//!   distance; the first three loops follow the nearest trees (start + position
//!   when a tree exists, stop otherwise). `PlayTreeSoundCmd` kind 0 → TreePlace,
//!   kind 2 → TreeDelete one-shots (kind 1 ignored), each with parameters
//!   `tree_size` / `is_naked` (rne-read).

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    utils::audio_utils::AudioLoop,
};
use crate::systems::audio::PlayTreeSoundCmd;
use bevy_ecs::prelude::*;
use glam::Vec3;

/// Loop events (O: rne-read; the 2nd–4th are the ambience loop).
pub const TREE_LOOP_EVENTS: [&str; 4] = [
    "event:/SFX/Tree/TreeMove",
    "event:/SFX/Tree/Amb",
    "event:/SFX/Tree/Amb",
    "event:/SFX/Tree/Amb",
];

/// One-shot events (O: decomp string assembly + rne-read).
pub const TREE_PLACE_EVENT: &str = "event:/SFX/Tree/TreePlace";
pub const TREE_DELETE_EVENT: &str = "event:/SFX/Tree/TreeDelete";

/// `TreeAmbience` component (native `components::TreeAmbience`; the ECS
/// component is registered in the registry but the owning struct lives in the
/// tree-tool domain 待接线). Carries the world position for distance sorting.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct TreeAmbience {
    pub position: Vec3,
}

/// TreeAudioState: the four loops.
#[derive(Resource)]
pub struct TreeAudioState<E: EventInstance> {
    pub loops: [AudioLoop<AudioInstance<E>>; 4],
}

impl<E: EventInstance> TreeAudioState<E> {
    pub fn new<S: Studio<Event = E>>(core: &AudioCore<S>) -> Self {
        TreeAudioState {
            loops: TREE_LOOP_EVENTS.map(|name| {
                core.instantiate_loop(&crate::resources::audio_resources::AudioLoopDesc::new(name))
            }),
        }
    }
}

/// `tree_audio_init_system` (0x1414633e0).
pub fn tree_audio_init_system<S: Studio + Send + Sync + 'static>(
    core: Res<AudioCore<S>>,
    mut commands: Commands,
) {
    commands.insert_resource(TreeAudioState::<S::Event>::new(&core));
}

/// `tree_audio_system` (0x1414638e0).
pub fn tree_audio_system<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<TreeAudioState<S::Event>>,
    mut events: EventReader<PlayTreeSoundCmd>,
    core: Res<AudioCore<S>>,
    trees: Query<&TreeAmbience>,
) {
    // O: nearest-tree distance list, stable-sorted ascending.
    let mut distances: Vec<(f32, Vec3)> = trees
        .iter()
        .map(|tree| (tree.position.length(), tree.position))
        .collect();
    distances.sort_by(|a, b| a.0.total_cmp(&b.0));

    // O: first three loops follow the nearest three trees.
    for (index, loop_player) in state.loops.iter_mut().enumerate() {
        if let Some((_, position)) = distances.get(index) {
            let _ = loop_player.start_if_not_playing();
            let current = &mut loop_player.entries[loop_player.cursor];
            let _ = current.set_position([position.x, position.y, position.z]);
        } else {
            let _ = loop_player.stop(std::time::Duration::ZERO);
        }
    }

    // O: PlayTreeSoundCmd kind 0 → TreePlace, 2 → TreeDelete, 1 ignored.
    for event in events.read() {
        let name = match event.kind {
            0 => TREE_PLACE_EVENT,
            2 => TREE_DELETE_EVENT,
            _ => continue,
        };
        let mut instance = core.instantiate_audio(name);
        // O: parameters tree_size / is_naked (rne-read), then position + start.
        let _ = instance.set_parameter("tree_size", event.position.length());
        let _ = instance.set_parameter("is_naked", 0.0);
        let _ = instance.set_position([event.position.x, event.position.y, event.position.z]);
        let _ = instance.start_if_not_playing();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_one_shot_event_names_are_original() {
        assert_eq!(TREE_PLACE_EVENT, "event:/SFX/Tree/TreePlace");
        assert_eq!(TREE_DELETE_EVENT, "event:/SFX/Tree/TreeDelete");
        assert_eq!(TREE_LOOP_EVENTS[0], "event:/SFX/Tree/TreeMove");
        assert_eq!(TREE_LOOP_EVENTS[1], "event:/SFX/Tree/Amb");
    }
}
