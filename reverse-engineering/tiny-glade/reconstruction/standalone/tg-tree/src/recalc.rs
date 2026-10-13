//! `recalc_tree_position_on_terrain_height_change` (VA 0x142268dc0, source lines 12-35).
//! Evidence: tree-tool-rest-analysis.md §3.

use crate::resources::TerrainHeights;
use crate::types::*;
use bevy_ecs::prelude::*;

/// Bounds gate `|x| <= 65.0 && |z| <= 65.0` (0x42820000 @0x142a83814).
pub const BOUND: f32 = 65.0;

/// No-op without new OnTerrainChange events. y is written as `max(sample, 0.0)` only when
/// the raw sample differs from the stored y (so a negative sample rewrites 0 every run).
pub fn recalc_tree_position_on_terrain_height_change(
    mut prefab_instances: Query<&mut PrefabInstanceData>,
    terrain: Res<TerrainHeights>,
    mut terrain_changes: EventReader<OnTerrainChange>,
) {
    if terrain_changes.read().count() == 0 {
        return;
    }
    for mut inst in &mut prefab_instances {
        let (x, z) = (inst.position.x, inst.position.z);
        if x.abs() <= BOUND && z.abs() <= BOUND {
            let h = terrain.sample(x, z);
            if h != inst.position.y {
                inst.position.y = h.max(0.0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_app::prelude::*;
    use glam::Vec3;

    #[derive(Resource, Default)]
    struct ChangedCount(usize);

    fn count_changed(q: Query<(), Changed<PrefabInstanceData>>, mut n: ResMut<ChangedCount>) {
        n.0 = q.iter().count();
    }

    fn setup(h: f32, positions: &[Vec3]) -> (App, Vec<Entity>) {
        let mut app = App::new();
        app.add_event::<OnTerrainChange>()
            .insert_resource(TerrainHeights::flat(h))
            .init_resource::<ChangedCount>()
            .add_systems(
                Update,
                (recalc_tree_position_on_terrain_height_change, count_changed).chain(),
            );
        let es = positions
            .iter()
            .map(|p| app.world_mut().spawn(PrefabInstanceData::at(*p)).id())
            .collect();
        (app, es)
    }

    fn y(app: &App, e: Entity) -> f32 {
        app.world().get::<PrefabInstanceData>(e).unwrap().position.y
    }

    #[test]
    fn needs_event() {
        let (mut app, es) = setup(3.0, &[Vec3::ZERO]);
        app.update();
        assert_eq!(y(&app, es[0]), 0.0);
    }

    #[test]
    fn updates_y_clamps_and_skips_out_of_bounds() {
        let (mut app, es) = setup(
            2.5,
            &[Vec3::new(1.0, 0.0, 1.0), Vec3::new(65.0, 0.0, -65.0), Vec3::new(65.1, 0.0, 0.0),
              Vec3::new(0.0, 0.0, -66.0), Vec3::new(2.0, 5.0, 2.0)],
        );
        app.update(); // consume the "added" change flags
        app.world_mut().send_event(OnTerrainChange);
        app.update();
        assert_eq!(app.world().resource::<ChangedCount>().0, 3);
        assert_eq!(y(&app, es[0]), 2.5);
        assert_eq!(y(&app, es[1]), 2.5);
        assert_eq!(y(&app, es[2]), 0.0);
        assert_eq!(y(&app, es[3]), 0.0);
        assert_eq!(y(&app, es[4]), 2.5);
        // event consumed: second run no-ops
        app.world_mut().insert_resource(TerrainHeights::flat(9.0));
        app.update();
        assert_eq!(y(&app, es[0]), 2.5);
        assert_eq!(app.world().resource::<ChangedCount>().0, 0);
    }

    #[test]
    fn negative_sample_clamps_to_zero_and_unchanged_not_counted() {
        let (mut app, es) = setup(-1.0, &[Vec3::new(0.0, 4.0, 0.0), Vec3::new(1.0, 2.5, 1.0)]);
        app.update();
        app.world_mut().send_event(OnTerrainChange);
        app.update();
        assert_eq!(app.world().resource::<ChangedCount>().0, 2);
        assert_eq!(y(&app, es[0]), 0.0);
        // stored 0.0 vs raw -1.0 differs again -> rewritten (faithful to compare-raw)
        app.world_mut().send_event(OnTerrainChange);
        app.update();
        assert_eq!(app.world().resource::<ChangedCount>().0, 2);
        app.world_mut().insert_resource(TerrainHeights::flat(0.0));
        app.world_mut().send_event(OnTerrainChange);
        app.update();
        assert_eq!(app.world().resource::<ChangedCount>().0, 0);
    }
}
