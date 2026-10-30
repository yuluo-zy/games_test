//! Original serializer/instant-dispatch branch fixtures, using the actual live
//! InnerWalls consumer. Values exercise branch semantics; they are not claimed
//! to be captured saves. Source identities are recorded in round12/history.
use anyhow::Result;
use bevy_ecs::prelude::*;
use country_core::{
    resources::{
        history::{ReplayBucket, ReplayBuffer, serialized::SavedMultiEdit},
        roofs::geometry::surface::Curve2,
        time::GameTime,
        walls::{InnerWalls, WallId, WallState, publication_source::TerrainHeightGrid512},
    },
    systems::{
        history::{
            HistoryReplayFailed,
            replay_wall::{register_wall_replay_events, replay_wall},
        },
        wall::wall_state::publish_public_wall_state::WallPublicationTerrain,
    },
};

#[test]
fn original_style_skirt_plaster_support_wire_reaches_live_authority_both_directions() -> Result<()>
{
    let rows = [
        (
            9,
            serde_json::json!({"ChangeStyle":{"wall_index":1,"before":"HalfTimbered","after":"StoneWithSkirts"}}),
        ),
        (
            16,
            serde_json::json!({"ChangeSkirt":{"wall_index":1,"before":true,"after":false}}),
        ),
        (
            10,
            serde_json::json!({"ChangePlasterPeelAmount":{"wall_index":1,"before":0.25,"after":0.75}}),
        ),
        (
            12,
            serde_json::json!({"ChangeSupports":{"wall_index":1,"before_removed_supports":false,"after_removed_supports":true}}),
        ),
    ];
    for (tag, row) in rows {
        let original = serde_json::json!({"edits":[{"Wall":[[row]]}]});
        let entry: SavedMultiEdit = serde_json::from_value(original.clone())?;
        assert_eq!(serde_json::to_value(&entry)?, original);
        let country_core::resources::history::EditType::Wall(wall) = &entry.0.edits[0] else {
            panic!("wall owner");
        };
        assert_eq!(wall.groups[0][0].operation_tag() as u64, tag);

        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let mut initial = WallState::new(curve, 2.0, if tag == 16 { 1 } else { 2 }, 0, false)
            .snapshot()
            .clone();
        initial.metadata.source_60_bits = 0.25_f32.to_bits();
        let mut world = World::new();
        register_wall_replay_events(&mut world);
        world.init_resource::<Events<HistoryReplayFailed>>();
        let mut walls = InnerWalls::default();
        walls.new_wall_with_id(WallId(1), WallState::from_snapshot(initial.clone(), 0));
        world.insert_resource(walls);
        world.insert_resource(GameTime {
            delta: 0.1,
            ..Default::default()
        });
        world.insert_resource(WallPublicationTerrain(TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 4.0,
            sampling_fix: true,
        }));
        world.insert_resource(ReplayBuffer::new(1.0));
        let mut schedule = Schedule::default();
        schedule.add_systems(replay_wall);

        for undo in [false, true] {
            let bucket = ReplayBucket::expand_with_aux(
                0,
                &entry.0,
                undo,
                false,
                entry.0.auxiliary.for_replay(undo),
            )
            .unwrap();
            world
                .resource_mut::<ReplayBuffer>()
                .push_with_aux(bucket, entry.0.auxiliary.for_replay(undo));
            schedule.run(&mut world);
            assert!(world.resource::<Events<HistoryReplayFailed>>().is_empty());
            let state = world
                .resource::<InnerWalls>()
                .get_unwrapped(WallId(1))
                .snapshot();
            match tag {
                9 => assert_eq!(state.source.style, if undo { 2 } else { 1 }),
                16 => assert_eq!(state.source.style, u8::from(undo)),
                10 => assert_eq!(
                    state.metadata.source_60_bits,
                    if undo { 0.25_f32 } else { 0.75_f32 }.to_bits()
                ),
                12 => assert_eq!(state.metadata.source_67, u8::from(!undo)),
                _ => unreachable!(),
            }
            assert_eq!(state.source.curve.points, initial.source.curve.points);
            assert_eq!(state.source.curve.points_u, initial.source.curve.points_u);
            assert_eq!(state.metadata.source_66, initial.metadata.source_66);
            assert_eq!(state.metadata.source_69, initial.metadata.source_69);
            let (finished, _) = world
                .resource_mut::<ReplayBuffer>()
                .pop_front_bucket()
                .unwrap();
            assert!(finished.commands.is_empty(), "tag{tag} undo={undo}");
        }
    }
    Ok(())
}

#[test]
fn original_auto_clutter_saved_variant_keeps_id_and_both_replay_directions() -> Result<()> {
    use country_core::resources::history::{ReplayCommand, auto_clutter_edit::AutoClutterEdit};
    let original =
        serde_json::json!({"edits":[{"AutoClutter":{"Delete":{"id":18446744073709551615_u64}}}]});
    let entry: SavedMultiEdit = serde_json::from_value(original.clone())?;
    assert_eq!(serde_json::to_value(&entry)?, original);
    for undo in [false, true] {
        let bucket = ReplayBucket::expand(0, &entry.0, undo, false).unwrap();
        assert_eq!(bucket.commands.len(), 1);
        let ReplayCommand::AutoClutter(command) = &bucket.commands[0] else {
            panic!("original tag13 expands to the AutoClutter consumer");
        };
        assert_eq!(*command.edit, AutoClutterEdit::Delete { id: u64::MAX });
        assert_eq!(command.undo, undo);
        assert!(command.timed);
        assert_eq!(command.duration.to_bits(), 0x3dcc_cccd);
    }
    Ok(())
}

#[test]
fn deprecated_decorator_aliases_encode_with_the_actual_original_save_name() -> Result<()> {
    // Original EditType writer140316771 saves DecoratorOld. Visitor140b3b630
    // accepts Decorator/Decorator4/Decorator5 as legacy names for the same owner.
    for name in ["DecoratorOld", "Decorator", "Decorator4", "Decorator5"] {
        let entry: SavedMultiEdit = serde_json::from_value(serde_json::json!({
            "edits": [{name: {"diffs": []}}]
        }))?;
        assert_eq!(
            serde_json::to_value(entry)?,
            serde_json::json!({
                "edits": [{"DecoratorOld": {"diffs": []}}]
            })
        );
    }
    Ok(())
}

#[test]
fn explicit_decorator_nulls_keep_nan_while_missing_fields_keep_original_defaults() -> Result<()> {
    let state = |fields: serde_json::Value| {
        let mut state = serde_json::json!({
            "dst":{"Terrain":{"pos_xz":[1.0,2.0]}},
            "ty":"Stairs", "rank":{"x":2}, "seed":19
        });
        state
            .as_object_mut()
            .unwrap()
            .extend(fields.as_object().unwrap().clone());
        state
    };
    let make_entry = |state| {
        serde_json::json!({
            "edits":[{"DecoratorV5":{"Change":{}}}],
            "decorators":{"diffs":[["Terrain",{"before":{},"after":{"7":state}}]]}
        })
    };
    for (fields, expected_nan) in [
        (serde_json::json!({}), false),
        (
            serde_json::json!({"h_offset":null,"lantern_brightness":null,"stair_width":null}),
            true,
        ),
    ] {
        let entry: SavedMultiEdit = serde_json::from_value(make_entry(state(fields)))?;
        let diff = entry.0.auxiliary.decorator_diff.as_ref().unwrap();
        let state = diff
            .values()
            .next()
            .unwrap()
            .after
            .states
            .values()
            .next()
            .unwrap();
        for (bits, default) in [(state.u30, 0.0_f32), (state.u38, 1.0), (state.u3c, 1.0)] {
            if expected_nan {
                assert!(f32::from_bits(bits).is_nan());
            } else {
                assert_eq!(bits, default.to_bits());
            }
        }
        let saved = serde_json::to_value(&entry)?;
        let saved_state = &saved["decorators"]["diffs"][0][1]["after"]["7"];
        for field in ["h_offset", "lantern_brightness", "stair_width"] {
            assert_eq!(
                saved_state.get(field),
                if expected_nan {
                    Some(&serde_json::Value::Null)
                } else {
                    None
                }
            );
        }
    }
    Ok(())
}

#[test]
fn m06_actual_color_plugin_registers_one_synchronous_style_writer_in_original_chain() {
    use bevy_app::{App, Update};
    use bevy_ecs::schedule::Schedules;
    use system_color::ColorSystems;
    let mut app = App::new();
    app.add_color();
    let schedules = app.world().resource::<Schedules>();
    let graph = schedules.get(Update).unwrap().graph();
    let systems: Vec<_> = graph.systems().map(|(id, system, _)| (id, system.name().to_string())).collect();
    let find = |suffix: &str| {
        let matches: Vec<_> = systems.iter().filter(|(_, name)| name.contains(suffix)).collect();
        assert_eq!(matches.len(), 1, "系统必须唯一：{suffix}");
        matches[0].0
    };
    let previous = find("set_halftimber_color_id");
    let style = find("set_wall_style");
    let next = find("set_flag_connection_and_chimney_smoke");
    let dependencies = graph.dependency().graph();
    assert!(dependencies.contains_edge(previous, style));
    assert!(dependencies.contains_edge(style, next));
    let set = graph.system_sets().find(|(_, set, _)| format!("{set:?}") == "GameSystemSet(31)")
        .map(|(id, _, _)| id).expect("原 color 系统集31");
    assert!(graph.hierarchy().graph().contains_edge(set, style));
}
