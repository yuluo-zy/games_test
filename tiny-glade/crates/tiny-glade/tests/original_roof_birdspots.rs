use bevy_ecs::prelude::*;
use country_core::resources::{critters::Wren, roofs::{roof_shape::Roof, commands::RecalculateRoofCmd}, time::GameTime, walls::{WallId, public_walls::{PublicWalls, OnWallChanged}}};
use country_core::utils::components::transform::Transform;
use system_critters::{bird_spots::{BirdSpots, BirdSpotManager}, wren::{roof_spots::sample_roof_bird_spots, update_roof_bird_spots}};
fn fixtures() -> serde_json::Value { serde_json::from_str(include_str!("fixtures/roof_birdspot_original.json")).unwrap() }
fn roof(case: &serde_json::Value) -> Roof {
    let s=case["roof_hex"].as_str().unwrap(); let mut bytes=[0;88];
    for (i,b) in bytes.iter_mut().enumerate() { *b=u8::from_str_radix(&s[i*2..i*2+2],16).unwrap(); }
    Roof::from_observed(bytes)
}
#[test]
fn complete_roof_perches_match_92_original_instruction_cases() {
    let fixture=fixtures(); let cases=fixture["cases"].as_array().unwrap(); assert_eq!(cases.len(),92);
    let mut nonempty=0;
    for case in cases {
        let mut manager=BirdSpotManager::default();
        let points=sample_roof_bird_spots(&roof(case), case["frame_count"].as_u64().unwrap() as u32, &mut manager);
        let expected=case["native_points"].as_array().unwrap(); assert_eq!(points.len(),expected.len(),"{}",case["name"]);
        nonempty+=usize::from(!points.is_empty());
        for (point, expected) in points.iter().zip(expected) {
            assert!(point.occupant.is_none()); assert_eq!(point.id.0,expected["id"].as_u64().unwrap() as u32);
            for (axis, actual) in [point.pos.x,point.pos.y,point.pos.z].into_iter().enumerate() {
                assert_eq!(actual.to_bits(),expected["position_bits"][axis].as_u64().unwrap() as u32,"case {} axis {axis}",case["name"]);
            }
        }
    }
    assert_eq!(nonempty,55);
}
#[test]
fn direct_recalc_owns_points_and_replacement_drains_on_following_no_event_frame() {
    let fixture=fixtures(); let case=fixture["cases"].as_array().unwrap().iter().find(|c| !c["native_points"].as_array().unwrap().is_empty()).unwrap();
    let mut world=World::new();world.init_resource::<BirdSpotManager>();world.init_resource::<PublicWalls>();world.init_resource::<Events<RecalculateRoofCmd>>();world.init_resource::<Events<OnWallChanged>>();
    world.insert_resource(GameTime { frame_count:case["frame_count"].as_u64().unwrap() as u32, ..Default::default() });
    let target=world.spawn(roof(case)).id(); let unrelated=world.spawn(roof(case)).id();
    let wall=WallId(0x123);world.resource_mut::<PublicWalls>().associate_roof(wall,unrelated);
    let mut waiting=Wren::default();waiting.spot.state=0;let actor=world.spawn((Transform::default(),waiting)).id();
    let mut schedule=Schedule::default();schedule.add_systems(update_roof_bird_spots);
    world.resource_mut::<Events<RecalculateRoofCmd>>().send(RecalculateRoofCmd { entity:target,wall,kind:0 });
    schedule.run(&mut world);
    assert!(!world.get::<BirdSpots>(target).unwrap().spots.is_empty());assert!(world.get::<BirdSpots>(unrelated).is_none());
    world.get_mut::<BirdSpots>(target).unwrap().spots[0].occupant=Some(actor);
    world.resource_mut::<Events<RecalculateRoofCmd>>().send(RecalculateRoofCmd { entity:target,wall,kind:0 });
    schedule.run(&mut world);
    assert_eq!(world.get::<Wren>(actor).unwrap().spot.state,0);
    schedule.run(&mut world);
    assert_eq!(world.get::<Wren>(actor).unwrap().spot.state,1);
    assert_eq!(world.query::<&BirdSpots>().iter(&world).count(),1);
}
