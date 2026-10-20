//! Cross-crate consumers for the existing production history stages.
//! B color141c7f270/tree141bc34f0 run on the shared front bucket; all their
//! writes must reach the real domain owners before the single set41 Pop.
use super::GameSystemSet;
use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use country_core::{resources as res, systems as sys};

pub(super) fn install(app: &mut App) {
    system_color::register_color_events(app.world_mut());
    // Tree-tool events/TreeStorage/systems come from TreePlugin140650520 and the roof
    // events/resources/systems from RoofSystems (run_game::add_original_plugins).
    app.init_resource::<res::color_id::WallColorIds>();
    app.init_resource::<res::color_id::GateColorIds>();
    app.init_resource::<res::color_id::RoofColorIds>();
    app.init_resource::<res::color_id::HalftimberColorIds>();
    app.init_resource::<res::stairs::stairs_removed_supports::StairsRemovedSupports>();
    app.init_resource::<res::walls::cached_decorator_transforms::CachedDecoratorTransforms>();
    app.init_resource::<sys::save::snapshot::PendingRoofAssociationRebuild>();
    app.init_resource::<res::roofs::associations::RoofAssociations>();
    app.init_resource::<sys::prefab::runtime::PrefabsToDelete>();
    app.init_resource::<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>(
    );
    // Same owning water raster constructor as WaterPlugin140b68a20, retained
    // for both load_game's actual raster application and tree water sampling.
    if !app
        .world()
        .contains_resource::<res::world_raster_defs::WaterRaster>()
    {
        app.insert_resource(res::world_raster_defs::WaterRaster::new(
            res::world_raster::DirtyTilePublisher::unique(),
        ));
    }
    // B GardenPlugin140b6d290 initializes unique Publisher{0,8,0,..}
    // before WorldRaster::new140a15530 and inserts that actual GardenRaster.
    if !app
        .world()
        .contains_resource::<res::world_raster_defs::GardenRaster>()
    {
        app.insert_resource(res::world_raster_defs::GardenRaster::new(
            res::world_raster::DirtyTilePublisher::unique(),
        ));
    }
    app.init_resource::<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>();
    // B run_game_thread @14059fc85 selects GameSystemSet14 (HistoryStart)
    // for the real raster staging consumer via converter1405c0400. Snapshot
    // loading runs in set12; rendering reads these same staging owners later.
    // A CPU-only host has no mapped buffers. Gameplay's original PreStartup
    // creates all six; the consumer retains its own missing-buffer failures.
    app.add_systems(
        Update,
        sys::world_raster::upload_world_raster_buffers_to_staging
            .in_set(GameSystemSet(14))
            .run_if(|device: Option<Res<country_core::render::device::GraphicsDeviceResource>>| {
                device.is_some()
            }),
    );
    // App registration supplies frame maintenance as well as the resource.
    // No second color replay/CPU apply is installed alongside this domain path.
    app.add_event::<res::color_id::SetWallColorIdCmd>();
    app.add_event::<res::color_id::SetGateColorIdCmd>();
    app.add_event::<res::color_id::SetHalftimberColorIdCmd>();
    app.add_event::<system_color::SetRoofColorIdCmd>();
    app.add_event::<system_color::SetDecoratorColorIdCmd>();
    app.add_event::<system_color::SetDecoratorBrightnessCmd>();
    app.add_event::<system_color::SetLadderColorIdCmd>();
    app.add_event::<system_color::SetStairRailingCmd>();
    app.add_event::<system_color::SetMagicColorIdCmd>();
    app.add_event::<system_color::SetWindowLightCmd>();
    app.add_event::<system_color::SetFlagConnectionCmd>();
    app.add_event::<system_color::SetChimneySmokeCmd>();
    app.add_event::<sys::prefab::SpawnPrefabCmd>();
    app.add_event::<sys::prefab::DespawnPrefabCmd>();
    app.add_event::<sys::audio::PlayTreeSoundCmd>();
    app.add_event::<sys::audio::PlayStairsAudioCmd>();
    app.add_event::<sys::audio::PlayWallAudio>();
    app.add_event::<sys::audio::PlayWallUiSound>();
    app.add_event::<res::walls::plaster::CreatePlasterCmd>();
    app.add_event::<res::walls::plaster::DeletePlasterCmd>();
    app.add_event::<res::roofs::commands::SetFlatRoofStyleCmd>();
    app.add_event::<res::roofs::commands::RecalculateWallHoles>();

    install_domain_lifecycles(app);
    app.add_systems(
        Update,
        sys::save::snapshot::rebuild_pending_roof_associations
            .after(GameSystemSet(19))
            .before(GameSystemSet(20)),
    );
    app.add_systems(
        Update,
        (
            sys::history::replay_decorator::replay_decorator,
            sys::history::replay_stairs::replay_stairs,
        )
            .chain()
            .in_set(GameSystemSet(40)),
    );
    app.add_systems(Update, (
        route_wall_replay_colors.after(sys::history::replay_wall::replay_wall),
        sys::history::replay_wall::apply_wall_replay_audio
            .after(sys::history::replay_wall::replay_wall),
        system_roof::edit_roof::set_flat_roof_style
            .after(system_color::replay_color::replay_color::<res::history::MaybePendingSnapshotId>),
    ).in_set(GameSystemSet(40)));
    // OPEN (not closed): the original registration point of the prefab command
    // runtime (spawn/update/despawn_prefab_commands) is unlocated; it stays here. The
    // tree systems that used to be interleaved (tree_move_bend, despawn_trees,
    // process_tree_cmds, recalc_tree_position_on_terrain_height_change) now live in
    // TreePlugin's set 0x1b chain.
    app.add_systems(Update, (
        sys::prefab::runtime::spawn_prefab_commands,
        sys::prefab::runtime::update_prefab_commands,
        sys::prefab::runtime::despawn_prefab_commands,
    ).chain().after(GameSystemSet(40)).before(GameSystemSet(41))
        .run_if(|world: &World| {
            world.contains_resource::<res::prefab::PrefabLibrary>()
                && world.contains_resource::<res::render::mesh_atlas_library::MeshAtlasLibrary>()
                && world.contains_resource::<country_core::render::nani::gpu_world::GpuWorld>()
                && world.contains_resource::<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>()
                && world.contains_resource::<res::rt::RtWorld>()
                && world.contains_resource::<res::session::ActiveSession>()
                && world.contains_resource::<sys::camera_rig::MainCamera>()
        }));
}

/// Bind lifecycle work to the same front-bucket stage and finish before Pop.
/// GPU-dependent consumers borrow canonical startup owners; no second rasters,
/// placeholder assets, or independent history ledger are constructed here.
fn install_domain_lifecycles(app: &mut App) {
    // Roof events are registered by startup_event_runtime (cross-domain ones) and
    // RoofSystems (the original add_roof_events_and_resources set).
    app.init_resource::<sys::ui::signifiers::signifier_create::UiSignifierStream>();
    // Preserve already existing nonempty command adapters. Their individual native
    // caller stages remain separately adjudicated; they are not native20 items.
    // OPEN (not closed): switch_roof_direction, set_flat_roof_rim_style and
    // update_wall_gable_heights (and set_flat_roof_style above) are not members of the
    // original add_roof_systems@14062fcb0; their original registration points are
    // unlocated. switch_roof_ridge_direction IS original (RoofSystems set 0x12).
    app.add_systems(Update, (
        system_roof::edit_roof::switch_roof_direction,
        system_roof::edit_roof::set_flat_roof_rim_style,
        system_roof::update_roof_spatial::update_wall_gable_heights,
    ).chain().after(system_roof::replay_roof::replay_roof).in_set(GameSystemSet(40)));
    app.add_systems(
        Update,
        (
            system_color::write_roof_color_id_to_gpu::write_wall_color_ids_to_gpu,
        )
            .after(GameSystemSet(40))
            .before(GameSystemSet(41))
            .run_if(
                |device: Option<Res<country_core::render::device::GraphicsDeviceResource>>| {
                    device.is_some()
                },
            ),
    );

    // EditTerrainStrokeCmd / SetTerrainStrokePtsCmd / EvTerrainStrokeChanged /
    // RecalculateHeightMapCmd / OnTerrainChange events, ScheduledTerrainReadback and the
    // edit_terrain_stroke -> terrain_stroke_changed -> delayed_terrain_changed_msg chain
    // (set 0x0f) come from TerrainSystems (run_game::add_original_plugins).
    app.add_event::<sys::terrain::PlayTerrainSound>();
    app.add_event::<sys::history::replay_raster::RasterReplayFeedback>();
    // OPEN (not closed): original registration points of replay_terrain,
    // terrain_hover_idx and ui_edit_focus_terrain are NOT in the recovered
    // add_terrain_systems@1409455f0; they stay mounted here (arrow_gizmo_animation is
    // mounted in run_game_thread set 7). Constraints are those of country-core's
    // add_terrain_tool_systems: hover -> focus before mode_manager; replay in set 40.
    app.add_systems(
        Update,
        sys::terrain::replay_terrain::replay_terrain
            .in_set(GameSystemSet(40))
            .run_if(|data: Option<Res<res::terrain_editing::TerrainEditingData>>| data.is_some()),
    );
    app.add_systems(
        Update,
        (
            sys::terrain::terrain_hover_idx::terrain_hover_idx,
            sys::ui_edit_focus::ui_edit_focus_terrain,
        )
            .chain()
            .before(sys::mode_manager::mode_manager::<res::history::EditType, res::history::MaybePendingSnapshotId>),
    );
    app.add_systems(
        Update,
        sys::history::replay_raster::replay_rasters::<res::history::MaybePendingSnapshotId>
            .in_set(GameSystemSet(40))
            .run_if(|world: &World| {
                world.contains_resource::<res::world_raster_defs::PathRaster>()
                    && world.contains_resource::<res::world_raster_defs::Cobblestone1Raster>()
                    && world.contains_resource::<res::world_raster_defs::Cobblestone2Raster>()
                    && world.contains_resource::<res::world_raster_defs::Cobblestone3Raster>()
                    && world.contains_resource::<res::world_raster_defs::GardenRaster>()
            }),
    );
}

fn route_wall_replay_colors(
    mut requests: EventReader<sys::history::replay_wall::WallHistoryRestoreColor>,
    mut wall: EventWriter<res::color_id::SetWallColorIdCmd>,
    mut gate: EventWriter<res::color_id::SetGateColorIdCmd>,
) {
    for request in requests.read() {
        let request = &request.0;
        if request.observed_special_owner {
            gate.write(res::color_id::SetGateColorIdCmd {
                id: request.wall.0 as u32,
                _gap4: (request.wall.0 >> 32) as u32,
                color: request.color_id,
                custom: 1,
                _pad: [0; 3],
            });
        } else {
            wall.write(res::color_id::SetWallColorIdCmd {
                id: request.wall.0 as u32,
                _gap4: (request.wall.0 >> 32) as u32,
                color: request.color_id,
                custom: 1,
                _pad: [0; 3],
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use res::history::{
        History, ReplayBucket, ReplayBuffer, ReplayCommand,
        replay_cmd_buffer::{ColorReplayCmd, CommandTiming, ReplayBucketAux},
    };
    use std::sync::Arc;

    fn app() -> App {
        let mut app = App::new();
        for stage in 0..44 {
            app.configure_sets(
                Update,
                GameSystemSet(stage).before(GameSystemSet(stage + 1)),
            );
        }
        super::super::install_history_and_wall_runtime(&mut app, false);
        app.init_resource::<sys::mode_manager::AppMode>();
        app.init_resource::<sys::mode_manager::PreviousAppMode>();
        app.init_resource::<country_core::input::action_set::MainActionSet>();
        app.init_resource::<res::gui::TimeOfDayScreen>();
        app.init_resource::<country_core::meta_commander::MetaCommander>();
        app.add_event::<sys::save::load_game::LoadingGameComplete>();
        app.add_event::<sys::mode_manager::HistoryCommand>();
        app.insert_resource(res::walls::InnerWalls::new());
        app.insert_resource(res::time::GameTime {
            delta: 0.05,
            ran: true,
            ..Default::default()
        });
        app
    }

    #[test]
    fn production_pop_restores_nonempty_stair_aux_and_rebases_its_backup() {
        let mut app = app();
        let mut stairs = res::stairs::stairs_state::StairsState::new();
        stairs.add_edge(1, 2);
        stairs.make_backup();
        let mut before = stairs.take_snapshot();
        let mut after = res::stairs::stairs_state::StairsSnapshot::new(vec![(2, 3), (4, 4)]);
        let diff = Arc::new(res::stairs::stairs_state::StairsState::reduce_diff(
            &mut before,
            &mut after,
        ));
        // Replay has provisional mutations; Pop must discard these first.
        stairs.add_edge(9, 10);
        app.insert_resource(stairs);
        for (undo, expected) in [(false, vec![(2, 3), (4, 4)]), (true, vec![(1, 2)])] {
            app.world_mut()
                .resource_mut::<ReplayBuffer>()
                .push_with_aux(
                    ReplayBucket {
                        entry_index: 0,
                        undo,
                        commands: vec![],
                        timing: vec![],
                    },
                    ReplayBucketAux {
                        stairs_diff: Some(diff.clone()),
                        ..Default::default()
                    },
                );
            app.update();
            let mut stairs = app
                .world_mut()
                .resource_mut::<res::stairs::stairs_state::StairsState>();
            let mut edges = stairs.collect_edges();
            edges.sort_unstable();
            assert_eq!(edges, expected);
            assert!(stairs.is_backed_up());
            assert!(stairs.extract_diff_and_make_backup().is_empty());
            assert!(app.world().resource::<ReplayBuffer>().is_empty());
        }
    }

    #[test]
    fn production_mixed_color_tree_replay_preserves_timing_and_resolves_sentinel() {
        let mut app = app();
        app.insert_resource(res::session::ActiveSession::new(
            res::session::SessionId::new(),
            res::session::SessionMeta::default(),
        ));
        let wall = res::walls::WallId(3);
        let mut bytes = [0; 88];
        bytes[..8].copy_from_slice(&wall.0.to_le_bytes());
        bytes[0x14..0x18].copy_from_slice(&2.0f32.to_le_bytes());
        let entity = app
            .world_mut()
            .spawn(res::roofs::Roof::from_observed(bytes))
            .id();
        app.world_mut()
            .resource_mut::<res::walls::public_walls::PublicWalls>()
            .associate_roof(wall, entity);
        let mut style = [0; 12];
        style[0] = 1;
        let color = res::history::color_edit::ColorEdit::FlatRoof {
            wall_id: wall,
            from: res::roofs::FlatRoofStyle::from_observed([0; 12]),
            to: res::roofs::FlatRoofStyle::from_observed(style),
        };
        let tree =
            res::history::tree_edit::TreeEdit::Place(res::history::tree_edit::TreeEditData {
                storage_index: Some(res::tree_storage::TreeStorageIndex(73)),
                position: glam::Vec3::new(8.0, 2.0, 9.0),
                scale: 0.8,
                rotation_degrees: 14.0,
                id: 19,
                prefab_name: res::history::tree_edit::missing_prefab(),
                ..Default::default()
            });
        app.world_mut()
            .resource_mut::<ReplayBuffer>()
            .push(ReplayBucket {
                entry_index: 0,
                undo: false,
                commands: vec![
                    ReplayCommand::Color(ColorReplayCmd::expanded(Arc::new(color), false, true)),
                    ReplayCommand::Tree(res::history::tree_edit::TreeReplayCmd {
                        edit: tree,
                        undo: false,
                        instant: false,
                    }),
                ],
                timing: vec![
                    CommandTiming {
                        elapsed: 77.0,
                        ..Default::default()
                    },
                    CommandTiming {
                        flags: 1,
                        delay: 0.2,
                        ..Default::default()
                    },
                ],
            });
        app.update();
        assert_eq!(
            &app.world()
                .get::<res::roofs::Roof>(entity)
                .unwrap()
                .observed()[0x40..0x4c],
            &style
        );
        let entry = app
            .world()
            .resource::<res::tree_storage::TreeStorage>()
            .get(res::tree_storage::TreeStorageIndex(73))
            .unwrap();
        assert!(
            app.world()
                .get::<country_core::components::TreeComponent>(entry.entity)
                .is_some()
        );
        let events = app
            .world()
            .resource::<Events<sys::prefab::SpawnPrefabCmd>>();
        let mut cursor = events.get_cursor();
        let command = cursor.read(events).last().unwrap();
        assert_ne!(
            command.prefab_name.as_ref().unwrap().as_name_ref().as_str(),
            res::history::tree_edit::missing_prefab()
        );
        assert_eq!(command.rotation, 14.0);
        assert_eq!(command.variant, 19);
        let bucket = app
            .world()
            .resource::<ReplayBuffer>()
            .buckets
            .front()
            .unwrap();
        assert_eq!(bucket.commands.len(), 1);
        assert_eq!(bucket.timing.len(), 1);
        assert_eq!(bucket.timing[0].elapsed, 0.05);
        app.update();
        assert!(
            cursor
                .read(
                    app.world()
                        .resource::<Events<sys::prefab::SpawnPrefabCmd>>()
                )
                .next()
                .is_none()
        );
        // The original owner still owns the history; replay did not re-record.
        assert!(
            app.world()
                .resource::<History>()
                .ledger()
                .entries
                .is_empty()
        );
    }

    #[test]
    fn loaded_empty_tree_prefab_reaches_the_registered_resolver() {
        let mut app = app();
        app.insert_resource(res::session::ActiveSession::new(
            res::session::SessionId::new(),
            res::session::SessionMeta::default(),
        ));
        let resources = sys::save::snapshot::SnapshotLoadResources {
            loaded: Some(sys::save::snapshot::LoadedSnapshotData {
                sections: vec![(
                    "Trees",
                    serde_json::json!([[73, {
                        "position": [8.0, 2.0, 9.0], "scale": 0.8,
                        "rotation_degrees": 14.0, "id": 19,
                        "prefab_name": "", "is_preplaced": false
                    }]]),
                )],
                ..Default::default()
            }),
        };
        let report =
            sys::save::snapshot::apply_loaded_snapshot_to_world(app.world_mut(), &resources)
                .unwrap();
        assert_eq!(report.trees, 1);
        {
            let world = app.world_mut();
            let mut trees = world.query::<&country_core::components::TreeComponent>();
            assert_eq!(trees.iter(world).count(), 0);
        }
        app.update();
        assert!(
            app.world()
                .resource::<res::tree_storage::TreeStorage>()
                .get(res::tree_storage::TreeStorageIndex(73))
                .is_some()
        );
        let spawns = app
            .world()
            .resource::<Events<sys::prefab::SpawnPrefabCmd>>();
        let mut reader = spawns.get_cursor();
        let command = reader.read(spawns).last().unwrap();
        assert_ne!(
            command.prefab_name.as_ref().unwrap().as_name_ref().as_str(),
            res::history::tree_edit::missing_prefab()
        );
        assert_eq!(command.rotation, 14.0);
        assert_eq!(command.variant, 19);
    }

    #[test]
    fn two_committed_entries_restore_their_own_aux_and_directional_cull_ids() {
        use res::history::decorator_edit::{
            DecoratorHistoryDiffMap, DecoratorHistoryState, DecoratorStorageAddrHistoryState,
            DecoratorStorageHistoryDiff,
        };
        use res::stairs::stairs_state::{StairsSnapshot, StairsState};
        use res::walls::{
            decorator::{DecoratorId, DecoratorInfo},
            decorator_archivist::LiveDecoratorArchivist,
            decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
        };
        let mut app = app();
        let addr = DecoratorStorageAddr::Terrain;
        let state = |id| {
            let mut state = DecoratorStorageAddrHistoryState::default();
            state
                .states
                .insert(DecoratorId(id), DecoratorHistoryState::default());
            state
        };
        for (index, before_id, after_id, old_edge, new_edge) in
            [(0, 10, 11, (1, 2), (2, 3)), (1, 11, 12, (2, 3), (3, 4))]
        {
            let mut diff = DecoratorHistoryDiffMap::default();
            diff.insert(
                addr,
                DecoratorStorageHistoryDiff {
                    before: state(before_id),
                    after: state(after_id),
                },
            );
            let stairs = StairsState::reduce_diff(
                &mut StairsSnapshot::new(vec![old_edge]),
                &mut StairsSnapshot::new(vec![new_edge]),
            );
            let mut history = app.world_mut().resource_mut::<History>();
            history.start_input_recording(0).unwrap();
            let commit = sys::history::recording::history_finish_recording(
                &mut history,
                &diff,
                &stairs,
                &mut std::collections::HashSet::new(),
                false,
            )
            .expect("changed owner commits a real history entry");
            assert_eq!(commit.entry_index, index);
        }
        // Actual final live authorities. The global legacy fields still hold
        // entry1's latest diff and must never stand in for older entry0.
        {
            let mut storage = app.world_mut().resource_mut::<DecoratorStorage>();
            storage.get_or_insert_default(addr).insert(
                DecoratorId(12),
                DecoratorInfo {
                    decorator_id: 12,
                    ..Default::default()
                },
            );
            storage.decorator_to_storage.insert(DecoratorId(12), addr);
        }
        {
            let mut stairs = app.world_mut().resource_mut::<StairsState>();
            stairs.add_edge(3, 4);
            stairs.make_backup();
        }
        for (command, expected_id, expected_edge, allowed, cursor) in [
            (sys::mode_manager::HistoryCommand::UNDO, 11, (2, 3), 12, 0),
            (sys::mode_manager::HistoryCommand::UNDO, 10, (1, 2), 11, -1),
            (sys::mode_manager::HistoryCommand::REDO, 11, (2, 3), 10, 0),
            (sys::mode_manager::HistoryCommand::REDO, 12, (3, 4), 11, 1),
        ] {
            app.world_mut().send_event(command);
            app.update();
            let world = app.world();
            let storage = world.resource::<DecoratorStorage>();
            assert!(
                storage
                    .lookup_decorator_info(DecoratorId(expected_id))
                    .is_some()
            );
            assert_eq!(storage.get(addr).unwrap().decorators.len(), 1);
            assert_eq!(
                world.resource::<StairsState>().collect_edges(),
                vec![expected_edge]
            );
            let ids = &world
                .resource::<LiveDecoratorArchivist>()
                .culling
                .allowed_ids;
            assert_eq!(ids.len(), 1);
            assert!(ids.contains(&allowed));
            assert_eq!(world.resource::<History>().ledger().cursor, cursor);
            assert!(world.resource::<ReplayBuffer>().is_empty());
        }
        assert_eq!(app.world().resource::<History>().ledger().entries.len(), 2);
    }
    #[test]
    fn production_roof_create_edit_delete_and_undo_restore_actual_entities() {
        use res::history::building_transactions::RoofHistoryEdit;
        use res::history::{EditType, transactions::HistoryEntry};
        use sys::wall::valid_enclosure::{Shape, ShapeParameters};
        let mut app = app();
        let shape = Shape::Rectangle(utils::geometry::Rectangle2d {
            axis: glam::Vec2::X,
            center: glam::Vec2::ZERO,
            size: glam::Vec2::new(4.0, 6.0),
        });
        let curve = shape.as_curve2();
        let curve = res::roofs::geometry::surface::Curve2::try_new(
            curve.points.iter().map(|p| p.to_array()).collect(),
            false,
        )
        .unwrap();
        let state = res::walls::WallState::new(curve, 3.9, 0, 0, false);
        let wall = res::walls::WallId(1);
        app.world_mut()
            .resource_mut::<res::walls::InnerWalls>()
            .new_wall_with_id(wall, state);
        app.world_mut()
            .resource_mut::<res::walls::InnerWalls>()
            .update_wall_state_enclosure(wall, ShapeParameters::from_shape(shape).to_snapshot());
        app.update();
        let create: RoofHistoryEdit = serde_json::from_value(serde_json::json!({"CreateRoof":{
            "wall_index":1,"color_id":4,"params":{
                "eave_length_01":0.2,"height_01":0.4,"profile_01":0.5,
                "ridge_dir":"Width","ridge_length_01":0.4,"tip_offset_01":[0.0,0.0]
            }
        }}))
        .unwrap();
        let visual: RoofHistoryEdit = serde_json::from_value(serde_json::json!({"VisualEdit":{
            "wall_index":1,"prev_height":0.4,"new_height":0.6,"prev_profile":0.5,"new_profile":0.7,
            "prev_eave":0.2,"new_eave":0.3,"prev_ridge":0.4,"new_ridge":0.5,
            "prev_tip_offset":[0.0,0.0],"new_tip_offset":[0.2,-0.1]
        }}))
        .unwrap();
        let apply = |app: &mut App, edit: &RoofHistoryEdit, undo| {
            let entry = HistoryEntry::from_edits(vec![EditType::Roof(Arc::new(edit.clone()))]);
            app.world_mut()
                .resource_mut::<ReplayBuffer>()
                .push(ReplayBucket::expand(0, &entry, undo, true).unwrap());
            for _ in 0..24 {
                app.update();
            }
            assert!(app.world().resource::<ReplayBuffer>().is_empty());
        };
        apply(&mut app, &create, false);
        let entity = app
            .world()
            .resource::<res::walls::public_walls::PublicWalls>()
            .roof_entity(wall)
            .unwrap();
        assert_eq!(
            app.world()
                .get::<res::roofs::Roof>(entity)
                .unwrap()
                .height_01(),
            0.4
        );
        apply(&mut app, &visual, false);
        let after = app.world().get::<res::roofs::Roof>(entity).unwrap().clone();
        assert_eq!(after.height_01(), 0.6);
        let delete = RoofHistoryEdit::delete_from_roof(after.observed());
        apply(&mut app, &delete, false);
        assert!(app.world().get_entity(entity).is_err());
        assert!(
            app.world()
                .resource::<res::walls::public_walls::PublicWalls>()
                .roof_entity(wall)
                .is_none()
        );
        apply(&mut app, &delete, true);
        let restored = app
            .world()
            .resource::<res::walls::public_walls::PublicWalls>()
            .roof_entity(wall)
            .unwrap();
        assert_ne!(restored, entity);
        assert_eq!(
            app.world()
                .get::<res::roofs::Roof>(restored)
                .unwrap()
                .height_01(),
            0.6
        );
        apply(&mut app, &visual, true);
        assert_eq!(
            app.world()
                .get::<res::roofs::Roof>(restored)
                .unwrap()
                .height_01(),
            0.4
        );
        apply(&mut app, &create, true);
        assert!(app.world().get_entity(restored).is_err());
        assert!(
            app.world()
                .resource::<Events<sys::history::HistoryReplayFailed>>()
                .is_empty()
        );
    }
}
