
#[cfg(test)]
mod canonical_mode_backoff_tests {
    use super::*;
    use country_core::resources::events::{ModeJustChanged, WriteSettingsFileCmd};
    use country_core::systems::mode_manager::{
        send_mode_just_changed_event, AppMode, EditSubject, PreviousAppMode,
    };

    #[test]
    fn published_latest_mode_controls_backoff_once_across_both_event_buffers() {
        let mut world = World::new();
        world.init_resource::<country_core::game_profile::GameProfile>();
        world.init_resource::<GameTime>();
        world.init_resource::<PreviousAppMode>();
        world.init_resource::<crate::hint_deselect_tool::DeselectToolHintConditions>();
        world.init_resource::<crate::hint_focused_edit::FocusedEditHintConditions>();
        world.init_resource::<Events<ModeJustChanged>>();
        world.init_resource::<Events<WriteSettingsFileCmd>>();
        world.init_resource::<Events<country_core::resources::window::CursorMovedScreenSpace>>();
        world.init_resource::<Events<crate::DisplayHintRequest>>();
        world.resource_mut::<GameTime>().delta = 0.0;
        world.resource_mut::<country_core::game_profile::GameProfile>().tutorial_cooldowns.cooldown_b = 800.0;
        let mut publish = Schedule::default();
        publish.add_systems(send_mode_just_changed_event);
        let mut consume = Schedule::default();
        consume.add_systems(exponential_back_off);

        world.insert_resource(AppMode::Editing(EditSubject::Roof(0x1234_5678_90ab_cdef)));
        publish.run(&mut world);
        world.resource_mut::<Events<ModeJustChanged>>().update();
        world.insert_resource(AppMode::Path);
        publish.run(&mut world);
        consume.run(&mut world);
        assert_eq!(world.resource::<country_core::game_profile::GameProfile>().tutorial_cooldowns.cooldown_b, 800.0);
        assert_eq!(world.resource::<Events<WriteSettingsFileCmd>>().len(), 0);

        world.insert_resource(AppMode::Editing(EditSubject::Wall(0xfeed_dead_beef_1234)));
        publish.run(&mut world);
        consume.run(&mut world);
        assert_eq!(world.resource::<country_core::game_profile::GameProfile>().tutorial_cooldowns.cooldown_b, 1200.0);
        assert_eq!(world.resource::<Events<WriteSettingsFileCmd>>().len(), 1);
        consume.run(&mut world);
        assert_eq!(world.resource::<country_core::game_profile::GameProfile>().tutorial_cooldowns.cooldown_b, 1200.0);
        assert_eq!(world.resource::<Events<WriteSettingsFileCmd>>().len(), 1);
    }
}
