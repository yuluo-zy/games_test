//! `TreeToolPlugin` (plugin build 0x140650520). Evidence: tree-tool-extra-analysis.md
//! (register_event / insert_resource<TreeStorage> / OnExit reset) and
//! tree-tool-rest-analysis.md §2 (replay_tree: `in_set<GameSystemSet>` + `chain_ignore_deferred`).

use crate::bend::tree_move_bend;
use crate::despawn::despawn_trees;
use crate::process_cmds::process_tree_cmds;
use crate::recalc::recalc_tree_position_on_terrain_height_change;
use crate::replay::replay_tree;
use crate::reset::reset_tree_tool;
use crate::storage::TreeStorage;
use crate::types::*;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

/// UNKNOWN(evidence: tree-tool-rest-analysis.md §2): the real `GameSystemSet` is defined in
/// country-core; only its name is evidenced. Minimal local stand-in so the documented
/// `in_set` registration can be expressed.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameSystemSet;

/// Resources read but not owned here (ActiveSession, WaterRaster, GameTime, ReplayBuffer,
/// TerrainHeights) must be inserted by their owning modules/the host app.
#[derive(Default)]
pub struct TreeToolPlugin;

impl Plugin for TreeToolPlugin {
    fn build(&self, app: &mut App) {
        // Evidenced events (PlaceTreeCmd/SpawnPreplacedTrees/Hide* are not reconstructed here).
        app.add_event::<MoveTreeCmd>()
            .add_event::<DeleteTreeCmd>()
            .add_event::<InsertTreeCmd>()
            .add_event::<SpawnPrefabCmd>()
            .add_event::<DespawnPrefabCmd>()
            .add_event::<PlayTreeSoundCmd>()
            // Owned by country-core; registered here only so the recalc system can run standalone.
            .add_event::<OnTerrainChange>()
            .init_resource::<TreeStorage>();

        // UNKNOWN: the original runs this on OnExit<AppLoadingState> (bevy_state state type
        // from country-core, not recovered). Plain Startup system as the nearest equivalent.
        app.add_systems(Startup, reset_tree_tool);

        // Evidenced: replay_tree in GameSystemSet + chain_ignore_deferred.
        // UNKNOWN: schedule label (Update assumed) and the ordering of the other systems
        // (after/before edges below are harness assumptions so a frame is deterministic).
        app.add_systems(
            Update,
            (
                replay_tree.in_set(GameSystemSet).chain_ignore_deferred(),
                process_tree_cmds.after(replay_tree),
                despawn_trees.after(process_tree_cmds),
                tree_move_bend,
                recalc_tree_position_on_terrain_height_change,
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::replay::*;
    use crate::resources::*;
    use crate::test_util::*;
    use glam::Vec3;

    fn app() -> App {
        let mut app = App::new();
        app.add_plugins(TreeToolPlugin)
            .insert_resource(ActiveSession::default())
            .insert_resource(WaterRaster::filled(4, 0.0))
            .insert_resource(GameTime { delta: 0.1 })
            .insert_resource(TerrainHeights::flat(0.0))
            .insert_resource(ReplayBuffer { front: vec![], divisor: 1.0 })
            .init_resource::<Seen<DespawnPrefabCmd>>()
            .add_systems(Update, (consume_spawns.after(process_tree_cmds),
                                  collect::<DespawnPrefabCmd>.after(despawn_trees)));
        app
    }

    #[test]
    fn replay_feeds_process_in_same_frame_and_reset_runs_on_startup() {
        let mut app = app();
        // Pre-existing storage content is wiped by the Startup reset.
        app.world_mut().resource_mut::<TreeStorage>().insert(
            9, crate::storage::TreeEntry::new(0, Entity::from_raw(1)));
        app.world_mut().resource_mut::<ReplayBuffer>().front.push(ReplayCmd {
            payload: ReplayPayload::TreeEdit(TreeEdit::Insert(InsertTreeCmd {
                name: "oak".into(), position: Vec3::ZERO, height: 1.0, field_28_unknown: 0.0,
                flags: 0, byte_30_unknown: 0, index: 2 })),
            delayed: false, delay: 0.0, elapsed: 0.0 });
        app.update();
        let s = app.world().resource::<TreeStorage>();
        assert!(s.get(9).is_none());
        assert_eq!((s.len(), s.next_index()), (1, 3));
    }

    #[test]
    fn full_lifecycle_insert_delete_despawn_event() {
        let mut app = app();
        app.world_mut().send_event(InsertTreeCmd { name: "oak".into(), position: Vec3::ZERO,
            height: 1.0, field_28_unknown: 0.0, flags: 0, byte_30_unknown: 0, index: 0 });
        app.update();
        app.world_mut().send_event(DeleteTreeCmd { index: 0 });
        app.update();
        // Marked this frame (deferred applied after process); animation 1.0 -> not yet despawned.
        let mut q = app.world_mut().query::<(&TreeMarkedForDeletion, &TreeAnimation)>();
        assert_eq!(q.iter(app.world()).count(), 1);
        assert!(app.world().resource::<Seen<DespawnPrefabCmd>>().0.is_empty());
        let mut q = app.world_mut().query::<&mut TreeAnimation>();
        for mut a in q.iter_mut(app.world_mut()) { a.value_at_4 = 0.0; }
        app.update();
        assert_eq!(app.world().resource::<Seen<DespawnPrefabCmd>>().0.len(), 1);
    }
}
