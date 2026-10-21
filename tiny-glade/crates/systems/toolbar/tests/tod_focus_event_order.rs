//! Narrow native contract: B 0x14234316b and 0x14234322b..26c.
//! These checks start at the actual button consumer, not physical input.
use bevy_ecs::prelude::*;
use country_core::{
    components::UiInteractionState,
    resources::{events::ModeJustChanged, gui::TimeOfDayScreen},
    systems::ui::resources::widget_params::{UiFocus, UiNavState},
};
use toolbar::{resouces_and_components::TimeOfDayButtonV2, ui_interaction::UiClickEvent};

fn fixture() -> (World, Schedule, Entity) {
    let mut world = World::new();
    world.init_resource::<Events<UiClickEvent>>();
    world.init_resource::<Events<ModeJustChanged>>();
    world.init_resource::<TimeOfDayScreen>();
    let mut nav = UiNavState::default();
    nav.focus = UiFocus { tag: 1, _pad: 0x11223344, payload: 0x123456789abcdef0 };
    nav.staged_focus = UiFocus { tag: 2, _pad: 9, payload: 101 };
    nav.hold = true;
    nav.timer = Some(std::time::Instant::now());
    world.insert_resource(nav);
    let button = world.spawn((TimeOfDayButtonV2, UiInteractionState::default())).id();
    let mut schedule = Schedule::default();
    schedule.add_systems(toolbar::toolbar_button::tod_v2_button);
    (world, schedule, button)
}

fn mode(world: &mut World, current: country_core::systems::mode_manager::AppMode) {
    world.init_resource::<country_core::systems::mode_manager::PreviousAppMode>();
    world.insert_resource(current);
    let mut publish = Schedule::default();
    publish.add_systems(country_core::systems::mode_manager::send_mode_just_changed_event);
    publish.run(world);
}

#[test]
fn opening_clears_only_focus_header_and_closing_preserves_it() {
    let (mut world, mut schedule, button) = fixture();
    let before = world.resource::<UiNavState>().clone();
    world.resource_mut::<Events<UiClickEvent>>().send(UiClickEvent { entity: button });
    schedule.run(&mut world);
    let nav = world.resource::<UiNavState>();
    assert!(world.resource::<TimeOfDayScreen>().visible);
    assert_eq!(nav.focus.tag, 0);
    assert_eq!(nav.focus._pad, 0);
    assert_eq!(nav.focus.payload, before.focus.payload);
    assert_eq!(nav.staged_focus, before.staged_focus);
    assert_eq!(nav.timer, before.timer);
    assert_eq!(nav.hold, before.hold);

    *world.resource_mut::<UiNavState>() = before.clone();
    world.resource_mut::<Events<UiClickEvent>>().send(UiClickEvent { entity: button });
    schedule.run(&mut world);
    assert!(!world.resource::<TimeOfDayScreen>().visible);
    assert_eq!(world.resource::<UiNavState>(), &before);
}

#[test]
fn latest_unread_mode_across_both_event_buffers_controls_visibility() {
    let (mut world, mut schedule, _) = fixture();
    world.resource_mut::<TimeOfDayScreen>().visible = true;
    mode(&mut world, country_core::systems::mode_manager::AppMode::Path);
    world.resource_mut::<Events<ModeJustChanged>>().update();
    mode(&mut world, country_core::systems::mode_manager::AppMode::Replay);
    schedule.run(&mut world);
    assert!(world.resource::<TimeOfDayScreen>().visible, "newest exempt mode wins");

    mode(&mut world, country_core::systems::mode_manager::AppMode::Replay);
    world.resource_mut::<Events<ModeJustChanged>>().update();
    mode(&mut world, country_core::systems::mode_manager::AppMode::Path);
    schedule.run(&mut world);
    assert!(!world.resource::<TimeOfDayScreen>().visible, "newest normal mode closes TOD");

    world.resource_mut::<TimeOfDayScreen>().visible = true;
    schedule.run(&mut world);
    assert!(world.resource::<TimeOfDayScreen>().visible, "consumed events cannot close it twice");
}
