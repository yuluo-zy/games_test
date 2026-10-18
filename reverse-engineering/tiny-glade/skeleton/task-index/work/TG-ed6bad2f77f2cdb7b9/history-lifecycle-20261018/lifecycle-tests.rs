#[cfg(test)]
mod lifecycle_tests {
    use super::*;
    use crate::resources::{
        cursor_terrain_raycast::CursorTerrainRaycast,
        glade_border::{BorderShape, GladeBorder},
        terrain_editing::{StrokeId, TerrainEditingData},
        walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
        world_raster::{DirtyTilePublisher}, world_raster_defs::WaterRaster,
    };
    use crate::systems::{
        backend::input::input_record_replay::InputReplayCommand,
        history::HistoryCpuStage,
        mode_manager::AppMode,
        terrain::{self, deferred_terrain_history::{PendingTerrainHistory, TerrainEdit, history_record_terrain}, ui_draw_terrain::{ui_draw_terrain, DRAW_CANVAS_NAME}},
        ui::resources::{ui_state::{UiState, UiInteraction, WidgetId}, widget_params::{WidgetParams, WidgetFlags}},
    };
    use bevy_ecs::{prelude::*, system::{RunSystemOnce, IntoSystem, System}};
    use glam::{Vec2, Vec3};
    use std::sync::Arc;
    use utils::geometry::Circle2d;

    fn edit() -> EditType {
        EditType::Terrain(Arc::new(TerrainEdit::NewTerrainStroke {
            index: StrokeId(17), height_ws: 2.5, stroke_pts: vec![[3.0, 0.0, 4.0]],
        }))
    }

    fn state(hover: u8, phase: u8, flags: u8) -> UiState {
        let id = WidgetId(19);
        let ui = UiState::new(UiInteraction { native_tag: hover, native_phase: phase, widget: id });
        let mut params = WidgetParams::default();
        params.flags = WidgetFlags(flags);
        ui.add_with_id(id, params);
        ui
    }

    #[test]
    fn start_matches_241_original_inline_and_initializer_cases() {
        // Generated from native-history-lifecycle-fixtures.json: mode, hover,
        // phase, flags, existing group, original call to start_input_recording.
        let cases: &[(u32, u8, u8, u8, bool, bool)] = &[
            // NATIVE_START_ROWS
        ];
        for (index, &(mode, hover, phase, flags, was_recording, started)) in cases.iter().enumerate() {
            let mut world = World::new();
            world.insert_resource(if mode == 13 { AppMode::Replay } else if mode == 10 { AppMode::Terrain } else { AppMode::default() });
            world.insert_resource(state(hover, phase, flags));
            let mut history = History::<()>::empty_glade();
            if was_recording { history.start_input_recording(0).unwrap(); history.add(edit()).unwrap(); }
            world.insert_resource(history);
            world.run_system_once(history_start_recording::<()>).unwrap();
            let history = world.resource::<History>();
            assert_eq!(history.is_recording(), was_recording || started, "native case {index}");
            if started { assert!(history.pending().unwrap().edits.is_empty(), "explicit start replaces group {index}"); }
        }
    }

    fn first_frame_world() -> World {
        let mut world = World::new();
        terrain::register_terrain_events(&mut world);
        register_recording_events(&mut world);
        world.insert_resource(History::<()>::empty_glade());
        world.insert_resource(AppMode::Terrain);
        // Use the actual input Sync consumer, preserving its existing next-frame
        // Control phase rather than creating a cycle with late history Record.
        crate::systems::backend::input::register_input_events(&mut world);
        world.init_resource::<crate::resources::toolbar::Toolbar>();
        world.init_resource::<crate::systems::camera_rig::main_camera::MainCamera>();
        world.init_resource::<crate::resources::mouse_capture::CursorCapturedPosition>();
        world.insert_resource(CursorTerrainRaycast { tag: 1, pos: Vec3::new(3.0, -1.0, 4.0), ..Default::default() });
        world.insert_resource(WaterRaster::new(DirtyTilePublisher::unique()));
        world.insert_resource(TerrainEditingData(tg_terrain::stroke::TerrainEditingData::default()));
        world.insert_resource(GladeBorder(BorderShape::Circle(Circle2d::new(Vec2::ZERO, 100.0))));
        world.insert_resource(UiState::new(UiInteraction { native_tag: 2, native_phase: 3, widget: WidgetId::clearing_canvas(DRAW_CANVAS_NAME) }));
        let library = crate::asset_libraries::ssbo_library::AssetSsboLibrary::new();
        let device = crate::render::test_device::TestDevice(0);
        use crate::{resources::terrain_rock_mask::TerrainRockMaskSsbo, systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo};
        library.add_new::<MouseTerrainRaycastSsbo>(&device);
        library.add_new::<TerrainRockMaskSsbo>(&device);
        library.get_mut::<MouseTerrainRaycastSsbo>().unwrap().mapped[0].terrain_stroke_index = -1;
        world.insert_resource(library);
        world
    }

    #[test]
    fn first_actual_draw_registers_permission_then_records_and_commits_on_release() {
        let mut world = first_frame_world();
        let widget = WidgetId::clearing_canvas(DRAW_CANVAS_NAME);
        // This must be absent: pre-registering the canvas would hide a broken
        // start-before-UI schedule and lose the first native drag.
        assert!(world.resource::<UiState>().clearing_canvas_params(widget).is_none());
        assert!(!world.resource::<History>().is_recording());
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_draw_terrain);
        schedule.add_systems(crate::systems::backend::input::input_record_replay::control_input_replay.before(ui_draw_terrain));
        schedule.add_systems(history_record_terrain::<()>.in_set(HistoryCpuStage::Record));
        add_recording_systems::<()>(&mut schedule);
        // Exercise a real next-frame storage write after the permission prefix.
        schedule.add_systems((|mut storage: ResMut<DecoratorStorage>| {
            storage.get_or_insert_default(DecoratorStorageAddr::Terrain).clear_modification_tracking();
        }).after(HistoryCpuStage::Cursor).before(ui_draw_terrain));
        schedule.run(&mut world);
        assert!(world.resource::<UiState>().clearing_canvas_params(widget).is_some());
        assert!(world.resource::<History>().is_recording());
        assert!(world.resource::<History>().pending().unwrap().edits.is_empty());
        assert_eq!(world.resource::<TerrainEditingData>().strokes.len(), 1);

        world.resource_mut::<UiState>().replace_interaction(UiInteraction { native_tag: 2, native_phase: 4, widget });
        world.resource_mut::<CursorTerrainRaycast>().pos.x = 4.0;
        schedule.run(&mut world);
        assert!(world.resource::<History>().is_recording());
        assert!(world.resource::<DecoratorStorage>().immutable);
        world.resource_mut::<UiState>().replace_interaction(UiInteraction { native_tag: 2, native_phase: 1, widget });
        schedule.run(&mut world);
        let history = world.resource::<History>();
        assert!(!history.is_recording());
        assert_eq!(history.ledger().cursor, 0);
        assert_eq!(history.ledger().entries.len(), 1);
        assert!(matches!(&history.ledger().entries[0].edits[..], [EditType::Terrain(edit)] if matches!(edit.as_ref(), TerrainEdit::NewTerrainStroke {index: StrokeId(0), stroke_pts, ..} if stroke_pts == &[[3.0,-1.0,4.0],[4.0,-1.0,4.0]])));
        assert!(world.resource::<PendingTerrainHistory>().is_empty());
        let events = world.resource::<Events<InputReplayCommand>>();
        assert_eq!(events.get_cursor().read(events).cloned().collect::<Vec<_>>(), [InputReplayCommand::Sync { record: true }]);
        assert_eq!(world.resource::<crate::systems::backend::input::input_record_replay::InputRecordReplayState>().stream.sync_points().len(), 0);
        // This next start must permit the actual storage write above again,
        // despite finish having marked storage immutable in the previous frame.
        world.resource_mut::<UiState>().replace_interaction(UiInteraction { native_tag: 2, native_phase: 3, widget });
        schedule.run(&mut world);
        assert!(world.resource::<History>().is_recording());
        assert_eq!(world.resource::<History>().ledger().entries.len(), 1);
        assert_eq!(world.resource::<crate::systems::backend::input::input_record_replay::InputRecordReplayState>().stream.sync_points().len(), 1);
    }

    #[test]
    fn unstarted_group_preserves_finish_reader_and_empty_finish_emits_no_sync() {
        let mut world = World::new();
        register_recording_events(&mut world);
        world.insert_resource(History::<()>::empty_glade());
        world.resource_mut::<Events<FinishInputRecordingCmd>>().write(FinishInputRecordingCmd);
        let mut system = IntoSystem::into_system(history_finish_recording_system::<()>);
        system.initialize(&mut world);
        system.run((), &mut world).unwrap();
        // Original has no pending group: it has not consumed the event.
        assert!(!world.resource::<DecoratorStorage>().immutable);
        world.resource_mut::<History>().start_input_recording(0).unwrap();
        system.run((), &mut world).unwrap();
        assert!(!world.resource::<History>().is_recording());
        assert!(world.resource::<History>().ledger().entries.is_empty());
        assert!(world.resource::<Events<InputReplayCommand>>().is_empty());
    }
    fn instant_deco() -> crate::systems::ui_clutter::ui_autoclutter::InstaCreateDeco {
        // Exact destination transport from original writer fixture deco[0].
        // This test exercises arrival; placement enum semantics remain open.
        crate::systems::ui_clutter::ui_autoclutter::InstaCreateDeco {
            destination: [0x75dd10c75433c8db, 0xe038d51dca8bf380, 0xecd9fb9341459e0e],
            extra: None, rotation: 0.0, position: Vec3::ZERO,
            ty: crate::systems::mode_manager::ClutterMeshTy::Bench,
        }
    }
    fn instant_stair() -> crate::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint {
        // Complete original native writer fixture stair[0], excluding EventId.
        crate::systems::ui_clutter::ui_autoclutter::InstaCreateStairPoint::from_observed([0x49, 0xbd, 0xe4, 0x94, 0x63, 0x79, 0x36, 0xe2, 0x9b, 0x35, 0xda, 0xed, 0xb1, 0x4b, 0xe5, 0x72, 0x28, 0xd0, 0x49, 0x0a, 0x73, 0xe5, 0x46, 0x62, 0x0f, 0xf9, 0xb1, 0xbe, 0x61, 0x4e, 0xf3, 0x17, 0x68, 0xf6, 0x4e, 0x44, 0x78, 0x9d, 0xdf, 0x00, 0xb9, 0xd0, 0x4b, 0xed, 0x26, 0xf7, 0x8b, 0xca, 0x43, 0xd0, 0x56, 0xf7, 0xf6, 0x24, 0x34, 0x68, 0xf9, 0x99, 0xa0, 0x2e, 0xc0, 0x9a, 0xc0, 0x2e, 0x79, 0x85, 0x29, 0x84, 0x4e, 0x90, 0x6f, 0x05, 0xa2, 0xff, 0x15, 0x87, 0x04, 0xcf, 0x82, 0x2c, 0x03, 0x1a, 0x16, 0x4a, 0x1f, 0x0f, 0xf1, 0x3c, 0x3a, 0xb3, 0xe0, 0x34, 0xc3, 0x0d, 0xac, 0x36, 0x6d, 0xdf, 0xf1, 0x5d, 0x3e, 0xc6, 0x84, 0xc4, 0x3f, 0x20, 0x18, 0xa6, 0x70, 0x67, 0x77, 0xc0])
    }

    #[test]
    fn finish_matches_86_original_lifecycle_cases() {
        // hover, phase, flags, arrivals[Deco,Stair,Finish], pending, edit_count,
        // original recording_after, ledger_len, storage_immutable, sync_count.
        let cases: &[(u8,u8,u8,[usize;3],bool,usize,bool,usize,bool,usize)] = &[
            // NATIVE_FINISH_ROWS
        ];
        for (index, &(hover,phase,flags,events,pending,edits,after,ledger,immutable,syncs)) in cases.iter().enumerate() {
            let mut world = World::new();
            register_recording_events(&mut world);
            world.insert_resource(state(hover, phase, flags));
            let mut history = History::<()>::empty_glade();
            if pending { history.start_input_recording(0).unwrap(); for _ in 0..edits { history.add(edit()).unwrap(); } }
            world.insert_resource(history);
            for _ in 0..events[0] { world.send_event(instant_deco()); }
            for _ in 0..events[1] { world.send_event(instant_stair()); }
            for _ in 0..events[2] { world.send_event(FinishInputRecordingCmd); }
            world.run_system_once(history_finish_recording_system::<()>).unwrap();
            assert_eq!(world.resource::<History>().is_recording(), after, "native case {index}");
            assert_eq!(world.resource::<History>().ledger().entries.len(), ledger, "native case {index}");
            assert_eq!(world.resource::<DecoratorStorage>().immutable, immutable, "native case {index}");
            assert_eq!(world.resource::<Events<InputReplayCommand>>().len(), syncs, "native case {index}");
        }
    }

    #[test]
    fn instant_short_circuit_preserves_stair_reader_but_always_consumes_finish_reader() {
        let mut world = World::new();
        register_recording_events(&mut world);
        let mut history = History::<()>::empty_glade();
        history.start_input_recording(0).unwrap(); history.add(edit()).unwrap();
        world.insert_resource(history);
        world.send_event(instant_deco()); world.send_event(instant_stair()); world.send_event(FinishInputRecordingCmd);
        let mut system = IntoSystem::into_system(history_finish_recording_system::<()>);
        system.initialize(&mut world);
        system.run((), &mut world).unwrap();
        assert_eq!(world.resource::<History>().ledger().entries.len(), 1);
        // Reader2 remains unread on the first frame and commits this new group.
        world.resource_mut::<History>().start_input_recording(0).unwrap();
        world.resource_mut::<History>().add(edit()).unwrap();
        system.run((), &mut world).unwrap();
        assert_eq!(world.resource::<History>().ledger().entries.len(), 2);
        // Reader3 was consumed despite Reader1's trigger; it cannot finish this.
        world.resource_mut::<History>().start_input_recording(0).unwrap();
        system.run((), &mut world).unwrap();
        assert!(world.resource::<History>().is_recording());
        assert_eq!(world.resource::<Events<InputReplayCommand>>().len(), 2);
    }

}
