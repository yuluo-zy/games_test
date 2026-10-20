//! Original B 0x14233c8b0 click producer and deferred animation lifecycle.
use bevy_ecs::prelude::*;
use bevy_time::Time;
use toolbar::{
    resouces_and_components::{TaffyNode, UiLocked, UiPlayLockedAnimation},
    ui_interaction::{UiClickEvent, ui_locked_animation},
};

#[test]
fn clicks_insert_only_on_live_locked_entities_after_existing_animation_tick() {
    let mut world = World::new();
    world.insert_resource(Time::<()>::default());
    world.resource_mut::<Time>().advance_by(std::time::Duration::from_secs_f32(0.03));
    world.init_resource::<Events<UiClickEvent>>();
    let locked = world.spawn((UiLocked, TaffyNode::default(), UiPlayLockedAnimation {a: 0.1, b: 0.3, progress: 0.0})).id();
    let no_node = world.spawn(UiLocked).id();
    let ordinary = world.spawn(TaffyNode::default()).id();
    let dead = world.spawn(UiLocked).id();
    world.despawn(dead);
    for entity in [locked, ordinary, dead, no_node, locked] {
        world.resource_mut::<Events<UiClickEvent>>().send(UiClickEvent {entity});
    }
    let mut schedule = Schedule::default();
    schedule.add_systems(ui_locked_animation);
    schedule.run(&mut world);
    // The pre-existing animation already ran, then deferred clicks reset it.
    assert!(world.get::<TaffyNode>(locked).unwrap().scale() > 1.0);
    for entity in [locked, no_node] {
        let animation = world.get::<UiPlayLockedAnimation>(entity).unwrap();
        assert_eq!([animation.a, animation.b, animation.progress], [0.1, 0.3, 0.0]);
    }
    assert!(world.get::<UiPlayLockedAnimation>(ordinary).is_none());
    assert!(world.get::<UiPlayLockedAnimation>(dead).is_none());
    // EventReader must not reprocess the same clicks on the next frame.
    schedule.run(&mut world);
    assert!(world.get::<UiPlayLockedAnimation>(locked).unwrap().progress > 0.0);
    assert_eq!(world.get::<UiPlayLockedAnimation>(no_node).unwrap().progress, 0.0);
    world.resource_mut::<Time>().advance_by(std::time::Duration::from_secs_f32(0.3));
    schedule.run(&mut world);
    assert!(world.get::<UiPlayLockedAnimation>(locked).is_none());
}

#[test]
fn original_sinc_zero_reaches_the_real_taffy_multiplier_without_special_case() {
    let mut world = World::new();
    world.insert_resource(Time::<()>::default());
    world.init_resource::<Events<UiClickEvent>>();
    let entity = world.spawn((TaffyNode::default(), UiPlayLockedAnimation {a: 0.1, b: 1.0, progress: 0.2})).id();
    let mut schedule = Schedule::default();
    schedule.add_systems(ui_locked_animation);
    schedule.run(&mut world);
    assert_eq!(world.get::<UiPlayLockedAnimation>(entity).unwrap().progress, 0.2);
    assert!(world.get::<TaffyNode>(entity).unwrap().scale().is_nan());
}
