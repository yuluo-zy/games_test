use bevy_ecs::prelude::*;
use country_core::resources::critters::Wren;
use country_core::utils::components::transform::Transform;
use system_critters::bird_spots::{BirdSpot, BirdSpotManager};
use system_critters::wren::{recycle::recycle_returned_bird_spots, SittableForBirds};

fn recycle(mut manager: ResMut<BirdSpotManager>,
    mut wrens: Query<(Entity, ComponentWrite<Transform>, &mut Wren), Without<SittableForBirds>>) {
    recycle_returned_bird_spots(&mut manager, &mut wrens);
}
#[test]
fn dropped_points_use_live_generation_query_membership_and_actual_wren_state() {
    let mut world = World::new();
    world.init_resource::<BirdSpotManager>();
    let mut actors = Vec::new();
    for state in [0, 1, 2, 3, 4, u32::MAX] {
        let mut wren = Wren::default(); wren.spot.state = state;
        actors.push(world.spawn((Transform::default(), wren)).id());
    }
    let stale = world.spawn((Transform::default(), Wren::default())).id();
    world.despawn(stale);
    let mut wren = Wren::default(); wren.spot.state = 0;
    let excluded = world.spawn((Transform::default(), wren, SittableForBirds::default())).id();
    let missing_transform = world.spawn(wren).id();
    let points = {
        let mut manager = world.resource_mut::<BirdSpotManager>();
        let mut records = Vec::new();
        for entity in actors.iter().copied().chain([stale, excluded, missing_transform]) {
            let id = manager.gen_id();
            records.push(BirdSpot { occupant: Some(entity), pos: glam::Vec3::ZERO, id });
        }
        records.push(BirdSpot::default());
        // Duplicate queued occupant must observe state 1 after its first callback.
        records.push(records[0]);
        manager.create_spots(records)
    };
    let mut schedule = Schedule::default(); schedule.add_systems(recycle);
    world.clear_trackers();
    schedule.run(&mut world);
    assert_eq!(world.get::<Wren>(actors[0]).unwrap().spot.state, 0);
    drop(points);
    world.clear_trackers();
    schedule.run(&mut world);
    for (&entity, state) in actors.iter().zip([1, 1, 2, 3, 4, u32::MAX]) {
        assert_eq!(world.get::<Wren>(entity).unwrap().spot.state, state);
    }
    assert_eq!(world.get::<Wren>(excluded).unwrap().spot.state, 0);
    assert_eq!(world.get::<Wren>(missing_transform).unwrap().spot.state, 0);
    let changed: Vec<Entity> = world.query_filtered::<Entity, Changed<Wren>>().iter(&world).collect();
    assert_eq!(changed, [actors[0]]);
    world.clear_trackers(); schedule.run(&mut world);
    assert_eq!(world.query_filtered::<Entity, Changed<Wren>>().iter(&world).count(), 0);
}
#[test]
fn spawned_wren_starts_in_original_airborne_state() {
    let wren = Wren::new_airborne(glam::Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(wren.spot.state, 2);
}

use country_core::render::extract::ComponentWrite;
