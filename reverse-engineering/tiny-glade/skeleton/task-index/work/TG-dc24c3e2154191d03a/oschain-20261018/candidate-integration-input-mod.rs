//! 有明确阶段的输入流水线。宿主游戏消费者放在 Consume，回放推进放在所有消费者之后。
pub mod button_input;
pub mod calculate_input_delta;
pub mod input_record_replay;
pub mod input_replay_for_trailers;
pub mod mod_key;
pub mod reset_input_state;
pub mod translate_input;
pub mod translate_input_to_cursor_moved;
pub mod update_cursor_position;

use crate::input::{
    action_set::{ActionSetStates, ActionSetValues},
    ActiveInputModeTracker,
};
use bevy_ecs::{prelude::*, schedule::common_conditions::resource_exists};
use input_record_replay::InputReplayCommand;
use translate_input_to_cursor_moved::{InputCursorDelta, InputCursorMoved};

#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InputSystemSet {
    Prepare,
    Control,
    Clock,
    Translate,
    Resolve,
    RecordReplay,
    Derive,
    Consume,
    Finish,
}

/// 注册输入自身拥有的资源。窗口、相机、工具栏、菜单由宿主提供。
fn register_event<T: Event>(world: &mut World) {
    if !world.contains_resource::<Events<T>>() {
        bevy_ecs::event::EventRegistry::register_event::<T>(world);
    }
}

pub fn register_input_events(world: &mut World) {
    register_event::<InputCursorMoved>(world);
    register_event::<InputCursorDelta>(world);
    register_event::<InputReplayCommand>(world);
    register_event::<crate::resources::walls::decorator::DecoratorOpEvent>(world);
    register_event::<crate::resources::mouse_capture::CursorCaptureCmd>(world);
    register_event::<crate::resources::mouse_capture::OsCursorCaptureCmd>(world);
    register_event::<crate::resources::mouse_capture::OsCursorVisibilityCmd>(world);
    world.init_resource::<crate::resources::mouse_capture::CursorCapturePolicy>();
    register_event::<crate::systems::mode_manager::AppModeChanged>(world);
    if !world.contains_resource::<crate::systems::mode_manager::PreviousAppMode>() {
        let current = world
            .get_resource::<crate::systems::mode_manager::AppMode>()
            .copied()
            .unwrap_or_default();
        world.insert_resource(crate::systems::mode_manager::PreviousAppMode(current));
    }
    register_event::<calculate_input_delta::UiHistoryButtonPressed>(world);
    register_event::<translate_input::MouseWheel>(world);
    register_event::<crate::resources::window::CursorMovedScreenSpace>(world);
    register_event::<crate::resources::session::NewSessionStartedCmd>(world);
    register_event::<crate::resources::session::SessionLoadedCmd>(world);
    world.init_resource::<ActionSetStates>();
    world.init_resource::<ActionSetValues>();
    world.init_resource::<bevy_time::Time>();
    world.init_resource::<crate::resources::time::GameTime>();
    if !world.contains_resource::<ActiveInputModeTracker>() {
        world.insert_resource(ActiveInputModeTracker::new_default_mouse_keyboard(
            std::time::Duration::ZERO,
        ));
    }
    world.init_resource::<input_record_replay::InputRecordReplayState>();
    world.init_resource::<update_cursor_position::CursorPosition>();
    world.init_resource::<mod_key::Modifiers>();
    world.init_resource::<button_input::KeyButtonInput>();
    world.init_resource::<button_input::MouseButtonInput>();
    world.init_resource::<translate_input::SteamInputActive>();
    world.init_resource::<translate_input::DemoModeEnabled>();
    world.init_resource::<input_replay_for_trailers::InputReplayForTrailers>();
}

pub fn add_input_cpu_consumers(schedule: &mut Schedule) {
    use InputSystemSet::*;
    schedule.configure_sets(
        (
            Prepare,
            Control,
            Clock,
            Translate,
            Resolve,
            RecordReplay,
            Derive,
            Consume,
            Finish,
        )
            .chain(),
    );
    schedule.add_systems(
        (
            input_record_replay::begin_input_frame,
            reset_input_state::reset_input_state,
        )
            .chain()
            .in_set(Prepare),
    );
    schedule.add_systems(
        (
            input_replay_for_trailers::record_and_replay_input_for_trailers,
            input_replay_for_trailers::do_input_replay_ui
                .run_if(resource_exists::<crate::resources::gui::EguiContextResource>),
            input_record_replay::control_input_replay,
            crate::systems::mode_manager::send_mode_just_changed_event,
        )
            .chain()
            .in_set(Control),
    );
    schedule.add_systems(crate::systems::time_update::time_update.in_set(Clock));
    // 先产生本帧光标增量，再判断右键手势，避免读取上一帧增量。
    schedule.add_systems(
        (
            translate_input_to_cursor_moved::translate_input_to_cursor_moved,
            translate_input::translate_input,
        )
            .chain()
            .in_set(Translate),
    );
    schedule.add_systems(calculate_input_delta::calculate_input_delta.in_set(Resolve));
    schedule.add_systems(input_record_replay::input_record_replay.in_set(RecordReplay));
    schedule.add_systems(
        (
            update_cursor_position::update_cursor_position,
            mod_key::update_mod_key_state,
        )
            .in_set(Derive),
    );
    schedule.add_systems(input_record_replay::advance_input_replay_frame.in_set(Finish));
}

#[cfg(test)]
mod registration_tests {
    use super::*;
    #[test]
    fn original_input_events_rotate_once_per_tick_and_registration_is_idempotent() {
        let mut world = World::new();
        register_input_events(&mut world);
        register_input_events(&mut world);
        world.send_event(translate_input::MouseWheel {
            unit: 0,
            x: 0.0,
            y: 1.0,
        });
        let update = |world: &mut World| {
            world.resource_scope(|world, mut registry: Mut<bevy_ecs::event::EventRegistry>| {
                registry.run_updates(world, bevy_ecs::component::Tick::new(0));
            });
        };
        update(&mut world);
        assert_eq!(
            world
                .resource::<Events<translate_input::MouseWheel>>()
                .len(),
            1,
            "the same event type must not be registered twice"
        );
        update(&mut world);
        assert!(
            world
                .resource::<Events<translate_input::MouseWheel>>()
                .is_empty(),
            "initializing Events alone does not enroll it in EventRegistry"
        );
    }
}
