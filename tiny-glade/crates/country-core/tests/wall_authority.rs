use country_core::resources::walls::freeform_height_cache::PreviewRectangleObserved;
use country_core::resources::walls::freeform_height_cache::rectangle_curve;
use country_core::resources::walls::publication_fields::WallShapeSnapshot;
use country_core::resources::walls::{
    InnerWalls, WallId, WallState, inner_wall_state::is_default_arch_height,
};
use serde_json::Value;
use country_core::resources::roofs::geometry::surface::Curve2;

fn curve() -> Curve2 {
    Curve2::try_new(vec![[0.0, 0.0], [1.0, 0.0]], false).unwrap()
}

fn state(height: f32) -> WallState {
    WallState::new(curve(), height, 2, 5, false)
}

fn decode(s: &str) -> Vec<u8> {
    s.as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}

fn word(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap())
}

fn from_native(bytes: &[u8]) -> WallState {
    let mut snapshot = state(0.0).into_snapshot().0;
    snapshot.source.shape = WallShapeSnapshot::from_observed(bytes[0x38..0x54].try_into().unwrap());
    snapshot.source.curve_kind_tag = snapshot.source.shape.tag();
    snapshot.source.extra = f32::from_bits(word(bytes, 0x54));
    snapshot.source.terrain_floor = f32::from_bits(word(bytes, 0x58));
    snapshot.source.observed_path_scalar = f32::from_bits(word(bytes, 0x5c));
    snapshot.metadata.source_60_bits = word(bytes, 0x60);
    snapshot.source.observed_source_flag_64 = bytes[0x64];
    snapshot.source.clutter_version = bytes[0x65];
    snapshot.metadata.source_67 = bytes[0x67];
    snapshot.source.style = bytes[0x68];
    snapshot.metadata.source_69 = bytes[0x69];
    WallState::from_snapshot(snapshot, bytes[0x66])
}

fn assert_native_fields(state: &WallState, bytes: &[u8], compare_shape_payload: bool) {
    let snapshot = state.snapshot();
    assert_eq!(snapshot.source.shape.tag(), word(bytes, 0x38));
    if compare_shape_payload {
        assert_eq!(snapshot.source.shape.bytes(), &bytes[0x38..0x54]);
    }
    assert_eq!(state.height().to_bits(), word(bytes, 0x54));
    assert_eq!(state.elevation().to_bits(), word(bytes, 0x58));
    assert_eq!(state.arch_height().to_bits(), word(bytes, 0x5c));
    assert_eq!(state.plaster_peel_amount().to_bits(), word(bytes, 0x60));
    assert_eq!(state.arch_type(), bytes[0x64]);
    assert_eq!(snapshot.source.clutter_version, bytes[0x65]);
    assert_eq!(state.freehand_ty(), bytes[0x66]);
    assert_eq!(state.removed_supports(), bytes[0x67]);
    assert_eq!(state.style(), bytes[0x68]);
    assert_eq!(snapshot.metadata.source_69, bytes[0x69]);
}

#[test]
fn authority_stores_match_the_original_instruction_fixtures() {
    let fixtures: Value =
        serde_json::from_str(include_str!("fixtures/wall-authority-b.json")).unwrap();
    assert_eq!(
        fixtures["binary_sha256"],
        "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
    );
    for case in fixtures["setters"].as_array().unwrap() {
        let before = decode(case["before"].as_str().unwrap());
        let payload = decode(case["input"].as_str().unwrap());
        let after = decode(case["after"].as_str().unwrap());
        let mut walls = InnerWalls::new();
        walls.new_wall_with_id(WallId(42), from_native(&before));
        walls.take_changes();
        let value = f32::from_bits(word(&payload, 0));
        let changed = match case["operation"].as_str().unwrap() {
            "height" => walls.update_wall_state_height(WallId(42), value),
            "elevation" => walls.update_elevation(WallId(42), value),
            "arch_height" => walls.update_arch_height(WallId(42), value),
            "plaster" => walls.update_plaster_peel_amount(WallId(42), value),
            "style" => walls.update_style(WallId(42), payload[0]),
            "freehand" => walls.update_freehand_ty(WallId(42), payload[0]),
            "arch_type" => walls.update_arch_type(WallId(42), payload[0]),
            "supports" => walls.update_supports(WallId(42), payload[0]),
            "enclosure" => walls.update_wall_state_enclosure(
                WallId(42),
                WallShapeSnapshot::from_observed(payload.try_into().unwrap()),
            ),
            "non_enclosed" => walls.mark_as_non_enclosed(WallId(42)),
            op => panic!("unexpected native operation {op}"),
        };
        assert!(changed);
        assert_native_fields(walls.get_unwrapped(WallId(42)), &after, true);
        assert!(walls.changes().updated.contains(&WallId(42)));
        assert!(walls.changes().created.is_empty());
        assert!(walls.changes().removed.is_empty());
        assert_eq!(walls.max_id(), 0);
    }
}

#[test]
fn constructor_counter_defaults_and_notification_match_native() {
    let fixtures: Value =
        serde_json::from_str(include_str!("fixtures/wall-authority-b.json")).unwrap();
    for case in fixtures["constructors"].as_array().unwrap() {
        let mut walls = InnerWalls::new();
        walls.set_max_id(case["counter"].as_u64().unwrap());
        let id = walls.new_wall_w_params(
            curve(),
            f32::from_bits(case["height_bits"].as_u64().unwrap() as u32),
            case["style"].as_u64().unwrap() as u8,
            case["clutter_version"].as_u64().unwrap() as u8,
            case["demo"].as_bool().unwrap(),
        );
        assert_eq!(id.0, case["id"].as_u64().unwrap());
        assert_eq!(walls.max_id(), case["counter_after"].as_u64().unwrap());
        assert_eq!(case["notice"][0], 0x30);
        assert_eq!(case["notice"][1], id.0);
        assert!(walls.changes().created.contains(&id));
        assert!(walls.changes().updated.is_empty());
        assert_native_fields(
            walls.get_unwrapped(id),
            &decode(case["state"].as_str().unwrap()),
            false,
        );
    }
    for case in fixtures["default_arch_height"].as_array().unwrap() {
        assert_eq!(
            is_default_arch_height(f32::from_bits(case["bits"].as_u64().unwrap() as u32)),
            case["result"].as_bool().unwrap()
        );
    }
}

#[test]
fn explicit_restore_duplicate_panic_keeps_the_native_replacement_side_effect() {
    let mut walls = InnerWalls::new();
    walls.set_max_id(7);
    walls.new_wall_with_id(WallId(99), state(2.0));
    walls.take_changes();
    let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        walls.new_wall_with_id(WallId(99), state(3.0));
    }));
    assert!(panic.is_err());
    assert_eq!(walls.get_unwrapped(WallId(99)).height(), 3.0);
    assert_eq!(walls.max_id(), 7);
    assert!(walls.changes().created.is_empty());
}

#[test]
fn equal_writes_notify_missing_writes_do_not_and_reset_only_clears_counter() {
    let mut walls = InnerWalls::new();
    let id = walls.new_wall(state(-0.0));
    walls.take_changes();
    assert!(walls.update_wall_state_height(id, -0.0));
    assert!(!walls.update_wall_state_height(WallId(88), 1.0));
    assert_eq!(walls.changes().updated.len(), 1);
    assert_eq!(walls.get_unwrapped(id).height().to_bits(), 0x80000000);
    walls.set_max_id(999);
    walls.reset_max_id();
    assert_eq!(walls.max_id(), 0);
    assert!(walls.contains(id));
    assert!(walls.changes().updated.contains(&id));
    let removed = walls.remove(id);
    assert_eq!(removed.height().to_bits(), 0x80000000);
    assert!(walls.changes().removed.contains(&id));
    let count = walls.changes().removed.len();
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| walls.remove(id))).is_err());
    assert_eq!(walls.changes().removed.len(), count);
}

#[test]
fn removing_all_preserves_counter_and_both_prior_notice_sets() {
    let mut walls = InnerWalls::new();
    let a = walls.new_wall(state(1.0));
    let b = walls.new_wall(state(2.0));
    walls.update_style(a, 1);
    walls.remove_all();
    assert_eq!(walls.wall_ids().count(), 0);
    assert_eq!(walls.max_id(), 2);
    assert_eq!(walls.changes().created.len(), 2);
    assert!(walls.changes().updated.contains(&a));
    assert!(walls.changes().removed.contains(&a));
    assert!(walls.changes().removed.contains(&b));
}

fn publication_fixture() -> (bevy_ecs::world::World, bevy_ecs::schedule::Schedule) {
    use country_core::resources::{walls::{publication_source::TerrainHeightGrid512}, world_raster::DirtyTilePublisher, world_raster_defs::{PathRaster, Cobblestone1Raster, Cobblestone2Raster, Cobblestone3Raster}};
    use country_core::systems::wall::{registration::register_wall_cpu_resources, wall_state::publish_public_wall_state::{WallPublicationTerrain, publish_public_wall_state}};
    let mut world = bevy_ecs::world::World::new();
    register_wall_cpu_resources(&mut world);
    world.insert_resource(WallPublicationTerrain(TerrainHeightGrid512 { encoded: vec![0;512*512].into(), minimum_y:0.0, maximum_y:0.0, sampling_fix:true }));
    world.insert_resource(PathRaster::new(DirtyTilePublisher::unique()));
    world.insert_resource(Cobblestone1Raster::new(DirtyTilePublisher::unique()));
    world.insert_resource(Cobblestone2Raster::new(DirtyTilePublisher::unique()));
    world.insert_resource(Cobblestone3Raster::new(DirtyTilePublisher::unique()));
    let mut schedule = bevy_ecs::schedule::Schedule::default();
    schedule.add_systems(publish_public_wall_state);
    (world,schedule)
}

#[test]
fn core_world_consumes_create_edit_delete_and_restoration_without_upper_runtime() {
    use bevy_ecs::prelude::*;
    use country_core::resources::walls::public_walls::PublicWalls;
    let (mut world, mut schedule) = publication_fixture();
    let rectangle = PreviewRectangleObserved {
        axes: [1.0, 0.0],
        center: [0.0, 0.0],
        dimensions: [4.0, 6.0],
    };
    let mut shape = [0; 28];
    shape[..4].copy_from_slice(&1u32.to_le_bytes());
    for (i, f) in [1.0f32, 0.0, 0.0, 0.0, 4.0, 6.0].into_iter().enumerate() {
        shape[4 + i * 4..8 + i * 4].copy_from_slice(&f.to_le_bytes());
    }
    let id = {
        let mut walls = world.resource_mut::<InnerWalls>();
        let id = walls.new_wall_w_params(rectangle_curve(rectangle).unwrap(), 3.0, 0, 0, false);
        walls.update_wall_state_enclosure(id, WallShapeSnapshot::from_observed(shape));
        id
    };
    schedule.run(&mut world);
    let output = world.resource::<PublicWalls>();
    let published = output.get(id).unwrap();
    assert_eq!(published.geometry.original_curve_length, 20.0);
    assert!(published.geometry.outline.len() > 5);
    let initial_max = published.regular_heights.as_ref().unwrap().maximum_y;
    assert!(output.last_delta.changed.contains(&id));
    schedule.run(&mut world);
    assert!(
        world
            .resource::<PublicWalls>()
            .last_delta
            .changed
            .is_empty()
    );
    assert!(
        world
            .resource::<PublicWalls>()
            .last_delta
            .removed
            .is_empty()
    );
    let static_tick = world.get_resource_ref::<PublicWalls>().unwrap().last_changed();
    schedule.run(&mut world);
    assert_eq!(world.get_resource_ref::<PublicWalls>().unwrap().last_changed(), static_tick);
    world
        .resource_mut::<InnerWalls>()
        .update_wall_state_height(id, 4.0);
    schedule.run(&mut world);
    let published = world
        .resource::<PublicWalls>()
        .get(id)
        .unwrap();
    assert!(published.regular_heights.as_ref().unwrap().maximum_y > initial_max);
    assert_eq!(published.authority.height(), 4.0);
    world.resource_mut::<InnerWalls>().mark_as_non_enclosed(id);
    schedule.run(&mut world);
    assert!(
        world
            .resource::<PublicWalls>()
            .get(id)
            .unwrap()
            .regular_heights
            .is_none()
    );
    let restored = world.resource_mut::<InnerWalls>().remove(id);
    schedule.run(&mut world);
    assert!(
        world
            .resource::<PublicWalls>()
            .get(id)
            .is_none()
    );
    assert!(
        world
            .resource::<PublicWalls>()
            .last_delta
            .removed
            .contains(&id)
    );
    world
        .resource_mut::<InnerWalls>()
        .new_wall_with_id(id, restored);
    schedule.run(&mut world);
    assert_eq!(world.resource::<InnerWalls>().max_id(), id.0);
    assert!(
        world
            .resource::<PublicWalls>()
            .get(id)
            .is_some()
    );
}

#[test]
fn a_failed_core_publication_is_visible_and_is_cleared_by_delete() {
    use country_core::resources::walls::public_walls::PublicWalls;
    let (mut world, mut schedule) = publication_fixture();
    let id = world.resource_mut::<InnerWalls>().new_wall(state(2.0));
    schedule.run(&mut world);
    assert!(world.resource::<PublicWalls>().get(id).is_some());
    let invalid = Curve2 { points: vec![], points_u: vec![], length: 0.0 };
    world.resource_mut::<InnerWalls>().update_wall_state_curve(id, invalid);
    schedule.run(&mut world);
    let output = world.resource::<PublicWalls>();
    assert!(output.failures().contains_key(&id));
    assert!(output.get(id).is_some(), "failed replacement preserves the last valid row");
    assert!(!output.last_delta.changed.contains(&id));
    world.resource_mut::<InnerWalls>().remove(id);
    schedule.run(&mut world);
    let output = world.resource::<PublicWalls>();
    assert!(output.get(id).is_none());
    assert!(output.failures().is_empty());
}
