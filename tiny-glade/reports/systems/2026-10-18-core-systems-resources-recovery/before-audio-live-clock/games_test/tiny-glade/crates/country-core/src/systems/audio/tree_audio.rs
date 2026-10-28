//! B `tree_audio.rs`: tree edit / ambience sounds — 4 loops + TreePlace /
//! TreeDelete one-shots.
//!
//! Fresh evidence (decomp-0x1414633e0 / 0x1414638e0, disasm-0x141463b80..
//! 0x141464130, map closure `next` 0x1412f0680):
//! - init: 4 × `instantiate_loop` — `event:/SFX/Tree/TreeMove`, then three
//!   `event:/SFX/Tree/Amb`.
//! - the distance closure maps `&TreeAmbience` → `(distance_to_camera_eye,
//!   position)`; the vector is stable-sorted by distance.
//! - frame start: the three loops at state+0x38/+0x70/+0xa8 (indices 1..=3)
//!   follow the three nearest trees (`start_if_not_playing` + `set_position`);
//!   the remaining loops are stopped.
//! - `PlayTreeSoundCmd` (stride 0x20: kind@+8, selector@+9, extra@+0xa,
//!   value@+0xc, position@+0x10):
//!   kind 0 → TreePlace, kind 2 → TreeDelete, kind 1 → frame-end TreeMove
//!   loop (selector byte == 2 stops it). One-shot parameter order:
//!   `tree_size` (= value) → `set_position` → `start_if_not_playing` →
//!   (`in_water` = selector != 0, `is_naked` = extra != 0 for TreePlace).

use crate::{
    resources::audio_resources::{AudioCore, AudioInstance, EventInstance, Studio},
    systems::audio::PlayTreeSoundCmd,
    systems::camera_rig::main_camera::MainCamera,
    systems::mode_manager::AppMode,
    resources::cursor_terrain_raycast::CursorTerrainRaycast,
    utils::audio_utils::AudioLoop,
};
use bevy_ecs::prelude::*;
use glam::Vec3;

/// Loop events (O: init string constants; the 2nd–4th are the ambience loop).
pub const TREE_LOOP_EVENTS: [&str; 4] = [
    "event:/SFX/Tree/TreeMove",
    "event:/SFX/Tree/Amb",
    "event:/SFX/Tree/Amb",
    "event:/SFX/Tree/Amb",
];

/// One-shot events (O: decomp string assembly + rne-read).
pub const TREE_PLACE_EVENT: &str = "event:/SFX/Tree/TreePlace";
pub const TREE_DELETE_EVENT: &str = "event:/SFX/Tree/TreeDelete";

/// One-shot / loop parameters (O: rne-read).
pub const TREE_SIZE_PARAM: &str = "tree_size";
pub const IN_WATER_PARAM: &str = "in_water";
pub const IS_NAKED_PARAM: &str = "is_naked";
pub const EDITING_SPEED_PARAM: &str = "editing_speed";

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
#[allow(clippy::too_many_arguments)]
pub fn tree_audio_system<S: Studio + Send + Sync + 'static>(
    mut state: ResMut<TreeAudioState<S::Event>>,
    mut events: EventReader<PlayTreeSoundCmd>,
    core: Res<AudioCore<S>>,
    camera: Res<MainCamera>,
    mode: Res<AppMode>,
    raycast: Res<CursorTerrainRaycast>,
    trees: Query<&TreeAmbience>,
) {
    // O (map closure `next` 0x1412f0680): distance to the maintained camera
    // matrix eye position, stable-sorted ascending.
    let eye = camera.eye_position();
    let mut distances: Vec<(f32, Vec3)> = trees
        .iter()
        .map(|tree| (tree.position.distance(eye), tree.position))
        .collect();
    distances.sort_by(|a, b| a.0.total_cmp(&b.0));

    // O: the three ambience loops (state+0x38/+0x70/+0xa8 = indices 1..=3) follow
    // the three nearest trees.
    for index in 1..=3 {
        let loop_player = &mut state.loops[index];
        if let Some((_, position)) = distances.get(index - 1) {
            let _ = loop_player.start_if_not_playing();
            let current = &mut loop_player.entries[loop_player.cursor];
            let _ = current.set_position([position.x, position.y, position.z]);
        } else {
            let _ = loop_player.stop(std::time::Duration::ZERO);
        }
    }

    // O: TreeMove loop (state+0) driven by the last kind-1 event: selector == 2
    // stops it, otherwise start + position + editing_speed / in_water / tree_size.
    let mut move_selector = 2u8;
    let mut move_position = Vec3::ZERO;
    let mut move_value = 0.0f32;

    // O: editing speed source: AppMode == Replay (0xd) → 1.0, else the shared
    // `CursorTerrainRaycast.speed` (+0x1c) — no /60 normalization here.
    let editing_speed = if mode.native_tag() == 0xd {
        1.0
    } else {
        raycast.speed
    };

    for event in events.read() {
        match event.kind {
            0 => {
                let mut instance = core.instantiate_audio(TREE_PLACE_EVENT);
                let _ = instance.set_parameter(TREE_SIZE_PARAM, event.value);
                let _ = instance.set_position([
                    event.position.x,
                    event.position.y,
                    event.position.z,
                ]);
                let _ = instance.start_if_not_playing();
                let in_water = if event.selector != 0 { 1.0 } else { 0.0 };
                let _ = instance.set_parameter(IN_WATER_PARAM, in_water);
                let is_naked = if event.extra != 0 { 1.0 } else { 0.0 };
                let _ = instance.set_parameter(IS_NAKED_PARAM, is_naked);
            }
            2 => {
                let mut instance = core.instantiate_audio(TREE_DELETE_EVENT);
                let _ = instance.set_parameter(TREE_SIZE_PARAM, event.value);
                let _ = instance.set_position([
                    event.position.x,
                    event.position.y,
                    event.position.z,
                ]);
                let _ = instance.start_if_not_playing();
            }
            1 => {
                move_selector = event.selector;
                move_position = event.position;
                move_value = event.value;
            }
            _ => {}
        }
    }

    let loop_player = &mut state.loops[0];
    if move_selector == 2 {
        let _ = loop_player.stop(std::time::Duration::ZERO);
    } else {
        let _ = loop_player.start_if_not_playing();
        let current = &mut loop_player.entries[loop_player.cursor];
        let _ = current.set_position([move_position.x, move_position.y, move_position.z]);
        let _ = current.set_parameter(EDITING_SPEED_PARAM, editing_speed);
        let in_water = if move_selector & 1 != 0 { 1.0 } else { 0.0 };
        let _ = current.set_parameter(IN_WATER_PARAM, in_water);
        let _ = current.set_parameter(TREE_SIZE_PARAM, move_value);
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

    #[test]
    fn tree_parameter_names_are_original() {
        assert_eq!(TREE_SIZE_PARAM, "tree_size");
        assert_eq!(IN_WATER_PARAM, "in_water");
        assert_eq!(IS_NAKED_PARAM, "is_naked");
    }
}
