//! External acceptance for actual canonical snapshot application and original producer receipts.
//! Independent of unrelated country-core unit-test fixture availability.
use country_core::{
    resources::{roofs::roof_shape::Roof, terrain_editing::TerrainEditingData},
    systems::save::snapshot::{SnapshotApplyReport, SnapshotLoadResources, LoadedSnapshotData,
        TerrainSectionJson, SNAPSHOT_FORMAT_VERSION, apply_loaded_snapshot_to_world,
        validate_and_load_closest_snapshot_data},
};
use serde_json::{Value, json};

fn apply_section(
    world: &mut bevy_ecs::world::World,
    key: &'static str,
    value: Value,
) -> anyhow::Result<SnapshotApplyReport> {
    // Native apply always restores the stairs owner, including files omitting
    // the Stairs section. This is a real fixture prerequisite, not production fallback.
    world.init_resource::<country_core::resources::stairs::stairs_state::StairsState>();
    apply_loaded_snapshot_to_world(world, &SnapshotLoadResources {
        loaded: Some(LoadedSnapshotData {
            version: i64::from(SNAPSHOT_FORMAT_VERSION),
            sections: vec![(key, value)],
            terrain: Vec::new(),
            rasters: Vec::new(),
        }),
    })
}

fn decode_native_hex(text: &str) -> Vec<u8> {
    (0..text.len()).step_by(2)
        .map(|offset| u8::from_str_radix(&text[offset..offset + 2], 16).unwrap())
        .collect()
}

#[test]
fn snapshot_roof_requests_match_retained_original_instruction_ranges() {
    let fixtures: Value = serde_json::from_str(include_str!(
        "fixtures/save-snapshot-native-producers-b.json"
    )).unwrap();
    assert_eq!(fixtures["sha256"], "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03");
    let cases = fixtures["roofs"].as_array().unwrap();
    assert_eq!(cases.len(), 24);
    let mut checked = 0;
    for case in cases {
        for (source, expected) in case["sources"].as_array().unwrap().iter()
            .zip(case["payloads"].as_array().unwrap()) {
            let bytes = decode_native_hex(source.as_str().unwrap());
            let roof = Roof::from_observed(bytes.try_into().unwrap());
            let mut world = bevy_ecs::world::World::new();
            world.init_resource::<bevy_ecs::event::Events<
                country_core::resources::roofs::commands::CreateRoofCmd,
            >>();
            apply_section(&mut world, "Roofs", serde_json::json!([roof])).unwrap();
            let requests: Vec<_> = world.resource_mut::<bevy_ecs::event::Events<
                country_core::resources::roofs::commands::CreateRoofCmd,
            >>().drain().collect();
            assert_eq!(requests.len(), 1);
            assert_eq!(&requests[0].0.record()[8..61],
                decode_native_hex(expected.as_str().unwrap()), "native Roof projection {checked}");
            checked += 1;
        }
    }
    assert_eq!(checked, 32);
}

#[test]
fn snapshot_color_requests_match_retained_original_instruction_ranges() {
    use country_core::resources::color_id::{SetWallColorIdCmd, SetGateColorIdCmd, SetHalftimberColorIdCmd};
    use bevy_ecs::event::Events;
    let fixtures: Value = serde_json::from_str(include_str!(
        "fixtures/save-snapshot-native-producers-b.json"
    )).unwrap();
    let cases = fixtures["colors"].as_array().unwrap();
    assert_eq!(cases.len(), 24);
    let mut checked = 0;
    for case in cases {
        let mut world = bevy_ecs::world::World::new();
        world.init_resource::<Events<SetWallColorIdCmd>>();
        world.init_resource::<Events<SetGateColorIdCmd>>();
        world.init_resource::<Events<SetHalftimberColorIdCmd>>();
        for (family, key) in ["WallColors", "GateColors", "HalftimberColors"].into_iter().enumerate() {
            apply_section(&mut world, key, case["inputs"][family].clone()).unwrap();
        }
        macro_rules! compare {
            ($event:ty, $family:expr) => {{
                let events = world.resource_mut::<Events<$event>>().drain().collect::<Vec<_>>();
                let expected = case["payloads"][$family].as_array().unwrap();
                assert_eq!(events.len(), expected.len());
                for (event, expected) in events.iter().zip(expected) {
                    let mut fields = event.wall_id().0.to_le_bytes().to_vec();
                    fields.extend_from_slice(&event.color.to_le_bytes());
                    fields.push(event.custom);
                    assert_eq!(fields, decode_native_hex(expected.as_str().unwrap()));
                    checked += 1;
                }
            }};
        }
        compare!(SetWallColorIdCmd, 0);
        compare!(SetGateColorIdCmd, 1);
        compare!(SetHalftimberColorIdCmd, 2);
    }
    assert_eq!(checked, 96);
}

#[test]
fn snapshot_tree_commands_preserve_every_input_and_resolve_only_absent_names() {
    use country_core::resources::history::tree_edit::{InsertTreeCmd, missing_prefab};
    use bevy_ecs::event::Events;
    let mut world = bevy_ecs::world::World::new();
    world.init_resource::<Events<InsertTreeCmd>>();
    let report = apply_section(&mut world, "Trees", json!([
        [u64::MAX, {"position": [1.25, -2.5, 3.75], "scale": 0.625,
            "rotation_degrees": -135.5, "id": -7, "prefab_name": "oak/source", "is_preplaced": true}],
        [73, {"position": [4.0, 5.0, 6.0], "scale": 0.75,
            "rotation_degrees": 90.0, "id": 12}]
    ])).unwrap();
    assert_eq!(report.trees, 2);
    let events: Vec<_> = world.resource_mut::<Events<InsertTreeCmd>>().drain().collect();
    assert_eq!(events[0].id, u64::MAX);
    assert_eq!(events[0].position, glam::Vec3::new(1.25, -2.5, 3.75));
    assert_eq!(events[0].scale, 0.625);
    assert_eq!(events[0].rotation_degrees, -135.5);
    assert_eq!(events[0].variant, -7);
    assert!(events[0].is_preplaced);
    assert_eq!(events[0].prefab_name, "oak/source");
    assert_eq!(events[1].id, 73);
    assert_eq!(events[1].prefab_name, missing_prefab());
    assert!(!events[1].is_preplaced);
    assert_eq!(world.entities().len(), 0, "entity creation belongs to actual tree consumer");
}

#[test]
fn auto_clutter_snapshot_replaces_previous_session_ids_including_empty() {
    use country_core::systems::ui_clutter::DeletedAutoClutter;
    let mut world = bevy_ecs::world::World::new();
    world.insert_resource(DeletedAutoClutter([11, 22].into_iter().collect()));
    let report = apply_section(&mut world, "AutoClutter", json!([22, 73, 73, u64::MAX]))
        .unwrap();
    assert_eq!(report.auto_clutter, 3);
    assert_eq!(world.resource::<DeletedAutoClutter>().0,
        [22, 73, u64::MAX].into_iter().collect::<std::collections::HashSet<u64>>());
    assert!(report.gaps.is_empty());
    let report = apply_section(&mut world, "AutoClutter", json!([])).unwrap();
    assert_eq!(report.auto_clutter, 0);
    assert!(world.resource::<DeletedAutoClutter>().0.is_empty());
}

#[test]
fn absent_auto_clutter_section_preserves_existing_owner() {
    use country_core::systems::ui_clutter::DeletedAutoClutter;
    let mut world = bevy_ecs::world::World::new();
    world.insert_resource(DeletedAutoClutter([73].into_iter().collect()));
    apply_section(&mut world, "Version", json!(SNAPSHOT_FORMAT_VERSION)).unwrap();
    assert!(world.resource::<DeletedAutoClutter>().is_deleted(73));
}

#[test]
fn invalid_auto_clutter_is_rejected_before_replacing_existing_owner() {
    use country_core::systems::ui_clutter::DeletedAutoClutter;
    for value in [json!(null), json!({}), json!([-1]), json!([1.5]), json!(["73"])] {
        let document = json!({"Version": SNAPSHOT_FORMAT_VERSION, "AutoClutter": value});
        let error = validate_and_load_closest_snapshot_data(&serde_json::to_vec(&document).unwrap())
            .unwrap_err();
        assert!(error.to_string().contains("AutoClutter"));
        let mut world = bevy_ecs::world::World::new();
        world.insert_resource(DeletedAutoClutter([73].into_iter().collect()));
        assert!(apply_section(&mut world, "AutoClutter", value).is_err());
        assert_eq!(world.resource::<DeletedAutoClutter>().0,
            [73].into_iter().collect::<std::collections::HashSet<u64>>());
    }
}

#[test]
fn auto_clutter_snapshot_requires_the_real_resource_owner() {
    let mut world = bevy_ecs::world::World::new();
    let error = apply_section(&mut world, "AutoClutter", json!([73])).unwrap_err();
    assert!(error.to_string().contains("DeletedAutoClutter constructor missing"));
    assert!(!world.contains_resource::<country_core::systems::ui_clutter::DeletedAutoClutter>());
}

#[test]
fn snapshot_terrain_invalidates_incoming_strokes_before_replacement() {
    use country_core::systems::terrain::draw_terrain_stroke::EvTerrainStrokeChanged;
    use bevy_ecs::event::Events;
    let mut world = bevy_ecs::world::World::new();
    let old: TerrainSectionJson = serde_json::from_value(json!({
        "next_stroke_index": 10, "strokes": {
            "9": {"falloff": 0.5, "height_ws": 2.0, "index": 9,
                "points": [[0.0, 1.0, 2.0]]}
        }
    })).unwrap();
    world.insert_resource(old.into_editing_data());
    world.init_resource::<Events<EvTerrainStrokeChanged>>();
    let report = apply_section(&mut world, "Terrain", json!({
        "next_stroke_index": 4, "strokes": {
            "1": {"falloff": 0.5, "height_ws": 2.0, "index": 1,
                "points": [[0.0, 1.0, 2.0]]},
            "3": {"falloff": 0.25, "height_ws": 4.0, "index": 3,
                "points": [[3.0, 2.0, 1.0]]}
        }
    })).unwrap();
    assert_eq!(report.terrain_strokes, 2);
    let mut ids: Vec<_> = world.resource_mut::<Events<EvTerrainStrokeChanged>>()
        .drain().map(|event| event.0.0).collect();
    ids.sort_unstable();
    assert_eq!(ids, [1, 3], "old-world stroke 9 is not the snapshot source");
    assert_eq!(world.resource::<TerrainEditingData>().next_index, 4);
}

#[test]
fn empty_snapshot_raster_still_requests_real_staging_upload() {
    use country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd;
    use bevy_ecs::event::Events;
    let mut world = bevy_ecs::world::World::new();
    world.init_resource::<Events<UploadWorldRasterBuffersToStagingCmd>>();
    let report = apply_section(&mut world, "Raster", json!([])).unwrap();
    assert!(report.gaps.is_empty());
    assert_eq!(world.resource::<Events<UploadWorldRasterBuffersToStagingCmd>>().len(), 1);
}

