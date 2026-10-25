#[cfg(test)]
mod lifecycle_integration_tests {
    use super::*;
    use crate::{resources::history::{EditType, History, auto_clutter_edit::AutoClutterEdit}, systems::{history::recording::{register_recording_events, history_finish_recording_system}, backend::input::input_record_replay::InputReplayCommand, mode_manager::ClutterMeshTy, ui_clutter::{AutoClutterRecord, AutoClutterEntityIndex, registration::register_ui_clutter_resources, rmb_autoclutter::handle_rmb_autoclutter}}};

    fn record() -> AutoClutterRecord {
        AutoClutterRecord {
            clutter_id: 73,
            // Source destination observed in native writer fixture deco[0].
            // Its placement enum semantics are outside this arrival test.
            destination: [0x75dd10c75433c8db, 0xe038d51dca8bf380, 0xecd9fb9341459e0e],
            position: glam::Vec3::new(1.0, 2.0, 3.0),
            w: 1.0, x: 0.0, y: 0.0, z: 0.0,
            entry_angle: 0.0, roof_space: false, local_x: 1.0, local_y: 0.0,
            ty: ClutterMeshTy::Bench,
        }
    }
    fn world() -> World {
        let mut world = World::new();
        register_ui_clutter_resources(&mut world);
        register_recording_events(&mut world);
        world.insert_resource(AppMode::default());
        world.insert_resource(History::<()>::empty_glade());
        world.resource_mut::<AutoClutterEntityIndex>().0.insert(42, record());
        world
    }

    #[test]
    fn actual_latest_right_click_emits_creation_and_commits_its_history() {
        let mut world = world();
        assert!(!world.resource::<History>().is_recording());
        world.send_event(EvRightClickedAutoClutter { entity: 42 });
        let mut schedule = Schedule::default();
        schedule.add_systems((ui_convert_autoclutter_to_decoclutter, handle_rmb_autoclutter::<()>, history_finish_recording_system::<()>).chain());
        schedule.run(&mut world);
        let history = world.resource::<History>();
        assert!(!history.is_recording());
        assert_eq!(history.ledger().entries.len(), 1);
        assert!(world.resource::<crate::systems::ui_clutter::DeletedAutoClutter>().is_deleted(73));
        assert!(!world.resource::<crate::systems::ui_clutter::DeletedAutoClutter>().is_deleted(42));
        assert!(matches!(&history.ledger().entries[0].edits[..], [EditType::AutoClutter(edit)] if **edit == (AutoClutterEdit::Delete { id: 73 })));
        let events = world.resource::<Events<InstaCreateDeco>>();
        let output = events.get_cursor().read(events).next().unwrap();
        assert_eq!(output.destination, record().destination);
        assert!(output.extra.is_none());
        assert_eq!(output.position, record().position);
        assert_eq!(output.ty, record().ty);
        let sync = world.resource::<Events<InputReplayCommand>>();
        assert_eq!(sync.get_cursor().read(sync).cloned().collect::<Vec<_>>(), [InputReplayCommand::Sync { record: true }]);
        // Actual placement consumption remains an explicit upper-module gap.
    }

    #[test]
    fn latest_invalid_entity_suppresses_earlier_valid_click_and_consumes_reader() {
        let mut world = world();
        // A Direct record has no entity identity and cannot bypass the native
        // generation-bearing entity guard on the ui_convert input path.
        world.resource_mut::<super::super::rmb_autoclutter::RmbAutoClutterDirect>().0 = Some(record());
        world.send_event(EvRightClickedAutoClutter { entity: 42 });
        world.send_event(EvRightClickedAutoClutter { entity: 999 });
        let mut schedule = Schedule::default();
        schedule.add_systems((ui_convert_autoclutter_to_decoclutter, handle_rmb_autoclutter::<()>).chain());
        schedule.run(&mut world);
        assert!(world.resource::<Events<InstaCreateDeco>>().is_empty());
        assert!(!world.resource::<History>().is_recording());
        schedule.run(&mut world);
        assert!(world.resource::<Events<InstaCreateDeco>>().is_empty());
        world.send_event(EvRightClickedAutoClutter { entity: 999 });
        world.send_event(EvRightClickedAutoClutter { entity: 42 });
        schedule.run(&mut world);
        assert_eq!(world.resource::<Events<InstaCreateDeco>>().len(), 1);
    }
}
