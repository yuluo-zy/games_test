//! Local CPU animation registration; global growth/spawn scheduling is separate.
use super::{
    ivy_branch_thickness_animation::ivy_branch_thickness_animation,
    ivy_grower::ivy_grower,
    ivy_leaf_animation::{IvyLeafIsGrowing, ivy_leaf_animation},
    ivy_leaf_spawner::{IvySegmentDoneGrowingMsg, ivy_leaf_spawner},
    ivy_pruner::{RemoveIvyCmd, ivy_pruner, ivy_pruner_cmd},
    ivy_spawner::ivy_spawner,
};
use crate::input::action_set::MainActionSet;
use crate::resources::ivy::{
    IvyDirectionProposer, IvyLeafRng, IvyLeafStorage, IvySpawnNoise, IvyStorage,
};
use crate::resources::critters::sheep::CmdCheckForSheepInsideBuilding;
use crate::resources::events::OnCircleRotated;
use crate::resources::walls::wall_holes::PrevWallHoles;
use bevy_ecs::prelude::*;

pub fn register_ivy_storage(world: &mut World) {
    world.init_resource::<IvyStorage>();
    world.init_resource::<IvyLeafStorage>();
    world.init_resource::<IvyLeafRng>();
    world.init_resource::<IvySpawnNoise>();
    world.init_resource::<IvyDirectionProposer>();
    // Ivy systems consume the previous-frame wall holes (grower hole gate) and
    // the shared input action set (spawner pressed gate); init_resource is
    // idempotent and does not replace a resource the input plugin installed.
    world.init_resource::<PrevWallHoles>();
    world.init_resource::<MainActionSet>();
    world.init_resource::<Events<IvySegmentDoneGrowingMsg>>();
    world.init_resource::<Events<IvyLeafIsGrowing>>();
    world.init_resource::<Events<RemoveIvyCmd>>();
    // 原两个独立reader共用生产者的真实事件身份。
    world.init_resource::<Events<CmdCheckForSheepInsideBuilding>>();
    world.init_resource::<Events<OnCircleRotated>>();
}

/// B `systems/ivy/mod.rs::impl$0::insert_ivy_resources` (0x1409440c0): inserts
/// IvyStorage/IvyLeafStorage/IvySpawnNoise/IvyDirectionProposer into the app
/// world. The original is Bevy framework resource-insert machinery; this crate
/// exposes the same insertion through `World::init_resource`.
pub fn insert_ivy_resources(world: &mut World) {
    register_ivy_storage(world);
}

/// B `systems/ivy/mod.rs::impl$0::add_ivy_events` (0x140944280): registers the
/// ivy event types. The original registers the system-set event buffers; this
/// crate initializes them as resources.
pub fn add_ivy_events(world: &mut World) {
    world.init_resource::<Events<IvySegmentDoneGrowingMsg>>();
    world.init_resource::<Events<IvyLeafIsGrowing>>();
    world.init_resource::<Events<RemoveIvyCmd>>();
}

pub fn add_ivy_animation_systems(schedule: &mut Schedule) {
    schedule.add_systems((ivy_branch_thickness_animation, ivy_leaf_animation).chain());
}

/// Local consumer order: pruning updates points/leaves before thickness and
/// leaf animation read them. The caller supplies GameTime, the loaded
/// GladeSettings owner and event maintenance.
pub fn add_ivy_cpu_systems(schedule: &mut Schedule) {
    schedule.add_systems(
        (
            ivy_pruner_cmd,
            ivy_leaf_spawner,
            ivy_branch_thickness_animation,
            ivy_leaf_animation,
        )
            .chain(),
    );
}

/// B `systems/ivy/mod.rs::impl$0::add_ivy_systems` (0x140944070): registers the
/// grower/spawner/pruner/leaf-spawner/animation system family. The original
/// places the spawner after GameSystemSet(22), followed by the CPU chain.
/// The GPU upload passes
/// (`ivy_send_to_gpu_leaf`/`ivy_send_to_gpu_segments`, whose bodies live in
/// the modules of the same names) and `systems/audio/ivy_audio::ivy_audio_init_system`
/// are render/audio-domain wiring that this crate's render world must attach
/// (待接线) — their business logic is implemented and tested.
pub fn add_ivy_systems(schedule: &mut Schedule) {
    schedule.add_systems(
        (
            ivy_spawner.after_ignore_deferred(crate::systems::GameSystemSet(22)),
            ivy_grower,
            ivy_leaf_spawner,
            ivy_pruner,
            ivy_pruner_cmd,
            ivy_branch_thickness_animation,
            ivy_leaf_animation,
        )
            .chain_ignore_deferred(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::{
        glade_settings::GladeSettings,
        ivy::{IvyGrowthSegment, IvyLeafInput},
        time::GameTime,
    };
    use glam::Vec3;

    #[test]
    fn created_ivy_points_and_leaves_are_pruned_then_animated() {
        let mut world = World::new();
        register_ivy_storage(&mut world);
        world.insert_resource(GladeSettings::default());
        world.insert_resource(GameTime {
            delta: 0.125,
            ..Default::default()
        });
        let id;
        let ids;
        {
            let mut storage = world.resource_mut::<IvyStorage>();
            id = storage.spawn_ivy(Vec3::ZERO, Vec3::X, 1.0..2.0, 4..5, 42);
            let ivy = storage.get_mut(id).unwrap();
            ids = [
                0,
                ivy.add_point(Vec3::X, Vec3::X),
                ivy.add_point(Vec3::X * 2.0, Vec3::X),
                ivy.add_point(Vec3::X * 3.0, Vec3::X),
            ];
            assert_eq!(ids, [0, 2, 3, 4]);
            ivy.pending_growth = Some(IvyGrowthSegment {
                from: Vec3::X * 3.0,
                to: Vec3::X * 4.0,
            });
            ivy.growth_progress = 0.5;
        }
        for segment in ids.windows(2) {
            world.resource_mut::<IvyLeafStorage>().add(IvyLeafInput {
                position: Vec3::ZERO,
                vector_2c: Vec3::Y,
                vector_58: Vec3::Z,
                branch_direction: Vec3::X,
                ivy: id,
                point_a: segment[0],
                point_b: segment[1],
                size: 1.0,
                growth_delay: 0.0,
                frame: 7,
                source_18: 42,
            });
        }
        world
            .resource_mut::<Events<RemoveIvyCmd>>()
            .send(RemoveIvyCmd::at_location(Vec3::X * 2.0, 0.5, 0.5, 19, 20));
        let mut schedule = Schedule::default();
        add_ivy_cpu_systems(&mut schedule);
        schedule.run(&mut world);
        let storage = world.resource::<IvyStorage>();
        let ivy = storage.get(id).unwrap();
        assert_eq!(
            ivy.points.iter().map(|point| point.id).collect::<Vec<_>>(),
            vec![0, 2]
        );
        assert_eq!(ivy.root.id, 0);
        assert_eq!(ivy.next_point_id, 4);
        assert_eq!(ivy.pending_growth, None);
        assert_eq!(ivy.growth_progress, 0.0);
        assert_eq!(storage.total_segment_count(), 1);
        let leaves = world.resource::<IvyLeafStorage>().get(id).unwrap();
        assert_eq!(leaves.len(), 1);
        assert_eq!((leaves[0].point_a, leaves[0].point_b), (0, 2));
        assert_eq!(leaves[0].growth, 0.25);
        let root_thickness = 1.0 - (-0.2f32).exp();
        assert!((ivy.points[0].thickness - root_thickness).abs() < 1e-6);
        assert_eq!(ivy.points[1].thickness, 0.0);
        assert!((leaves[0].branch_thickness - (0.5 + root_thickness * 0.25)).abs() < 1e-6);
        assert_eq!(world.resource::<Events<IvyLeafIsGrowing>>().len(), 1);
    }
}
