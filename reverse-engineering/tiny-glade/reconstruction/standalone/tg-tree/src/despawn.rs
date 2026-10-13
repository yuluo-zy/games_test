//! `despawn_trees` (VA 0x1422686d0, process_tree_cmds.rs:203-214).
//! Evidence: tree-tool-rest-analysis.md §1.

use crate::types::*;
use bevy_ecs::prelude::*;

/// Threshold 0.01f (0x3c23d70a @ 0x142960674).
pub const DESPAWN_THRESHOLD: f32 = 0.01;

/// Query<(Entity,&TreeAnimation,&TreeMarkedForDeletion)>: every marked tree with
/// animation f32@+4 strictly below 0.01 (NaN never) gets a DespawnPrefabCmd{entity}.
/// Re-sent every frame until the (external) consumer despawns the entity.
pub fn despawn_trees(
    trees: Query<(Entity, &TreeAnimation, &TreeMarkedForDeletion)>,
    mut despawn_prefab: EventWriter<DespawnPrefabCmd>,
) {
    for (entity, anim, _) in &trees {
        if anim.value_at_4 < DESPAWN_THRESHOLD {
            despawn_prefab.write(DespawnPrefabCmd::for_entity(entity));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;
    use bevy_app::prelude::*;

    fn world(v: f32, marked: bool, animated: bool) -> (App, Entity) {
        let mut app = App::new();
        app.add_event::<DespawnPrefabCmd>()
            .init_resource::<Seen<DespawnPrefabCmd>>()
            .add_systems(Update, (despawn_trees, collect::<DespawnPrefabCmd>).chain());
        let mut e = app.world_mut().spawn_empty();
        if animated {
            e.insert(TreeAnimation { value_at_4: v });
        }
        if marked {
            e.insert(TreeMarkedForDeletion);
        }
        let id = e.id();
        (app, id)
    }

    #[test]
    fn sends_only_below_threshold() {
        for (v, expect) in [(0.0, 1), (0.0099, 1), (0.01, 0), (0.5, 0), (f32::NAN, 0), (-1.0, 1)] {
            let (mut app, e) = world(v, true, true);
            app.update();
            let seen = &app.world().resource::<Seen<DespawnPrefabCmd>>().0;
            assert_eq!(seen.len(), expect, "v={v}");
            if expect == 1 {
                assert_eq!(seen[0].entity_bits, e.to_bits());
            }
        }
    }

    #[test]
    fn unmarked_trees_ignored() {
        let (mut app, _) = world(0.0, false, true);
        app.update();
        assert!(app.world().resource::<Seen<DespawnPrefabCmd>>().0.is_empty());
    }

    #[test]
    fn marked_without_animation_ignored() {
        let (mut app, _) = world(0.0, true, false);
        app.update();
        assert!(app.world().resource::<Seen<DespawnPrefabCmd>>().0.is_empty());
    }

    #[test]
    fn resent_every_frame_until_consumed() {
        let (mut app, _) = world(0.0, true, true);
        app.update();
        app.update();
        assert_eq!(app.world().resource::<Seen<DespawnPrefabCmd>>().0.len(), 2);
    }
}
