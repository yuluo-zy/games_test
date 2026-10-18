use bevy_ecs::prelude::*;
use country_core::components::{TreeAnimation, TreeMarkedForDeletion};
use country_core::systems::prefab::DespawnPrefabCmd;
use system_tree::process_tree_cmds::despawn_trees;

#[test]
fn deletion_reads_entity_progress_and_requires_the_marker() {
    let mut world = World::new();
    world.init_resource::<Events<DespawnPrefabCmd>>();
    // 很短的生长时长不应触发删除；已完成的进度仍为 1。
    let mut growing = TreeAnimation::new(0.001);
    growing.progress = 1.0;
    world.spawn((growing, TreeMarkedForDeletion));
    // 很长的生长时长不能阻止已衰减树的回收。
    let mut shrinking = TreeAnimation::new(10.0);
    shrinking.progress = 0.009;
    let expected = world.spawn((shrinking, TreeMarkedForDeletion)).id();
    world.spawn(TreeAnimation::new(1.0));
    let mut at_boundary = TreeAnimation::new(1.0);
    at_boundary.progress = 0.01;
    world.spawn((at_boundary, TreeMarkedForDeletion));
    let mut not_a_number = TreeAnimation::new(1.0);
    not_a_number.progress = f32::NAN;
    world.spawn((not_a_number, TreeMarkedForDeletion));

    let mut schedule = Schedule::default();
    schedule.add_systems(despawn_trees);
    schedule.run(&mut world);
    let events: Vec<_> = world
        .resource_mut::<Events<DespawnPrefabCmd>>()
        .drain()
        .map(|event| event.entity)
        .collect();
    assert_eq!(events, [Some(expected)]);
}
