
#[cfg(test)]
mod canonical_mode_publication_tests {
    use super::*;
    use country_core::resources::events::ModeJustChanged;
    use country_core::systems::mode_manager::{
        send_mode_just_changed_event, AppMode, EditSubject, PreviousAppMode,
    };
    use crate::hint_clutter_focus_edit::{
        hint_clutter_focused_edit_condition, FocusedClutterHintConditions,
    };
    use crate::tutorial_completion::TutorialHintComplete;

    #[test]
    fn published_latest_mode_controls_both_hint_consumers_and_drains_old_modes() {
        for (target, expected_hint) in [
            (AppMode::Editing(EditSubject::Wall(0xfedc_ba98_7654_3210)), 1),
            (AppMode::Editing(EditSubject::DecoClutter { decorator_id: 0xdead_beef }), 9),
        ] {
            let mut world = World::new();
            world.init_resource::<country_core::game_profile::GameProfile>();
            world.init_resource::<FocusedEditHintConditions>();
            world.init_resource::<FocusedClutterHintConditions>();
            world.init_resource::<PreviousAppMode>();
            world.init_resource::<Events<ModeJustChanged>>();
            world.init_resource::<Events<TutorialHintComplete>>();
            world.init_resource::<Events<country_core::systems::ui::resources::widget_params::PlayUiSound>>();
            let mut publish = Schedule::default();
            publish.add_systems(send_mode_just_changed_event);
            let mut consume = Schedule::default();
            consume.add_systems((hint_focused_edit_condition, hint_clutter_focused_edit_condition));

            // A qualifying old-buffer mode must not override the newest Path.
            world.insert_resource(target);
            publish.run(&mut world);
            world.resource_mut::<Events<ModeJustChanged>>().update();
            world.insert_resource(AppMode::Path);
            publish.run(&mut world);
            consume.run(&mut world);
            assert!(!world.resource::<FocusedEditHintConditions>().hint_done);
            assert!(!world.resource::<FocusedClutterHintConditions>().hint_done);
            assert_eq!(world.resource::<Events<TutorialHintComplete>>().len(), 0);
            consume.run(&mut world);
            assert_eq!(world.resource::<Events<TutorialHintComplete>>().len(), 0);

            // Re-enter through the real publisher; only the latest target qualifies.
            world.insert_resource(target);
            publish.run(&mut world);
            consume.run(&mut world);
            let completed: Vec<_> = world.resource_mut::<Events<TutorialHintComplete>>()
                .drain().map(|event| event.id).collect();
            assert_eq!(completed, vec![expected_hint]);
            consume.run(&mut world);
            assert_eq!(world.resource::<Events<TutorialHintComplete>>().len(), 0);
        }
    }
}
