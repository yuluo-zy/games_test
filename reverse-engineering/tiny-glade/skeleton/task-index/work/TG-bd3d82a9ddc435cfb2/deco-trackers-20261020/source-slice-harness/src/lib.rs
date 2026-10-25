#![forbid(unsafe_code)]
// This alias provides the candidate's canonical dependency namespace. The
// definitions below are byte-for-byte current source spans, not mock types.
extern crate self as country_core;
pub mod resources {
    pub mod walls {
        include!("canonical/wall_id.rs");
        pub mod decorator {
            include!("canonical/decorator_import.rs");
            include!("canonical/decorator_id.rs");
        }
        pub mod decorator_storage {
            include!("canonical/storage_import.rs");
            include!("canonical/decorator_storage_addr.rs");
        }
    }
}
pub mod decorator_visual {
    #[path = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-bd3d82a9ddc435cfb2/deco-trackers-20261020/candidate/src/decorator_visual/deco_differential_state.rs"]
    pub mod deco_differential_state;
    #[path = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-917d6130810801d337/deco-trackers-20261020/candidate/src/decorator_visual/chimney_assemble_bricks.rs"]
    pub mod chimney_assemble_bricks;
    #[path = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-e10831b95b26b67a3c/deco-trackers-20261020/candidate/src/decorator_visual/door_stairs_assemble_bricks.rs"]
    pub mod door_stairs_assemble_bricks;
}

#[cfg(test)]
mod real_bevy_protocol_fixture {
    use bevy_ecs::prelude::*;
    use crate::decorator_visual::chimney_assemble_bricks::{chimney_assemble_bricks, ChimneyAssemblyParams};
    use crate::decorator_visual::door_stairs_assemble_bricks::{door_stairs_assemble_bricks, DoorStairsAssemblyParams};
    use crate::decorator_visual::deco_differential_state::WallAttachedDecoTracker;
    use crate::resources::walls::{WallId, decorator::DecoratorId, decorator_storage::DecoratorStorageAddr};

    // Test observations only; this does not stand in for any original host.
    #[derive(Resource, Default)]
    struct Observed { current: Vec<(WallId, DecoratorId)>, removed: Vec<(WallId, DecoratorId)> }

    fn chimney_fixture(tracker: ResMut<WallAttachedDecoTracker<ChimneyAssemblyParams>>, mut out: ResMut<Observed>) {
        chimney_assemble_bricks(tracker, |wall, log| {
            log.with_each_deco_diff(|id, diff| {
                if diff.current().is_some() { out.current.push((wall, id)); }
            }).with_removed(|id, _| out.removed.push((wall, id)))
        });
    }
    fn door_fixture(tracker: ResMut<WallAttachedDecoTracker<DoorStairsAssemblyParams>>, mut out: ResMut<Observed>) {
        door_stairs_assemble_bricks(tracker, |wall, log| {
            log.with_each_deco_diff(|id, diff| {
                if diff.current().is_some() { out.current.push((wall, id)); }
            }).with_removed(|id, _| out.removed.push((wall, id)))
        });
    }

    #[test]
    fn both_real_resmut_adapters_consume_current_then_removed() {
        let wall = WallId(12);
        let storage = DecoratorStorageAddr::Wall(wall);
        let mut chimney = WallAttachedDecoTracker::default();
        chimney.add(storage, DecoratorId(1), &ChimneyAssemblyParams { words: [0; 10] });
        let mut door = WallAttachedDecoTracker::default();
        door.add(storage, DecoratorId(2), &DoorStairsAssemblyParams { words: [0; 8] });
        let mut world = World::new();
        world.insert_resource(chimney);
        world.insert_resource(door);
        world.init_resource::<Observed>();
        let mut schedule = Schedule::default();
        schedule.add_systems((chimney_fixture, door_fixture));
        schedule.run(&mut world);
        assert_eq!(world.resource::<Observed>().current.len(), 2);
        assert!(world.resource::<Observed>().removed.is_empty());
        world.resource_mut::<WallAttachedDecoTracker<ChimneyAssemblyParams>>().remove(storage, DecoratorId(1));
        world.resource_mut::<WallAttachedDecoTracker<DoorStairsAssemblyParams>>().remove(storage, DecoratorId(2));
        schedule.run(&mut world);
        let out = world.resource::<Observed>();
        assert_eq!(out.current.len(), 2);
        assert_eq!(out.removed.len(), 2);
        assert!(out.removed.contains(&(wall, DecoratorId(1))));
        assert!(out.removed.contains(&(wall, DecoratorId(2))));
    }
}
