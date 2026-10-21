//! TB-E (round 11): `switch_app_mode_button` request/selected/animation.
//!
//! Contract `tb-toolbar-CONTRACT.md` §2.15 (#79): a click emits
//! `RequestAppModeChange(button.mode)`; a `ModeJustChanged` frame compares each
//! button's target with `Res<AppMode>`, writes the selected byte and queues
//! `UiPlaySelectAnimation{0.2, 0.2, 0}` for the matching button.
use bevy_ecs::event::Events;
use bevy_ecs::prelude::*;
use country_core::components::UiInteractionState;
use country_core::resources::events::ModeJustChanged;
use country_core::systems::mode_manager::{AppMode, RequestAppModeChange};
use toolbar::resouces_and_components::{SwitchAppModeButton, UiPlaySelectAnimation};
use toolbar::ui_interaction::UiClickEvent;

fn publish_mode_change(world: &mut World) {
    world.init_resource::<country_core::systems::mode_manager::PreviousAppMode>();
    let mut publish = Schedule::default();
    publish.add_systems(country_core::systems::mode_manager::send_mode_just_changed_event);
    publish.run(world);
}

#[test]
fn click_requests_the_button_mode_and_mode_change_selects_only_matching() {
    let mut world = World::new();
    world.init_resource::<Events<UiClickEvent>>();
    world.init_resource::<Events<RequestAppModeChange>>();
    world.init_resource::<Events<ModeJustChanged>>();
    world.insert_resource(AppMode::Water);

    let water = world
        .spawn((
            SwitchAppModeButton {
                mode: AppMode::Water,
            },
            UiInteractionState::default(),
        ))
        .id();
    let path = world
        .spawn((
            SwitchAppModeButton {
                mode: AppMode::Path,
            },
            UiInteractionState::default(),
        ))
        .id();
    let unrelated = world.spawn(UiInteractionState::default()).id();

    let mut schedule = Schedule::default();
    schedule.add_systems(toolbar::toolbar_button::switch_app_mode_button);

    // Click phase: only widgets carrying SwitchAppModeButton emit a request.
    for entity in [water, path, unrelated] {
        world
            .resource_mut::<Events<UiClickEvent>>()
            .send(UiClickEvent { entity });
    }
    schedule.run(&mut world);
    world.flush();
    let requests: Vec<AppMode> = world
        .resource_mut::<Events<RequestAppModeChange>>()
        .drain()
        .map(|event| event.0)
        .collect();
    assert_eq!(requests, vec![AppMode::Water, AppMode::Path]);
    // No ModeJustChanged yet: selected bytes stay untouched and no animation.
    assert_eq!(world.get::<UiInteractionState>(water).unwrap().selected, 0);
    assert_eq!(world.get::<UiInteractionState>(path).unwrap().selected, 0);
    assert!(world.get::<UiPlaySelectAnimation>(water).is_none());

    // ModeJustChanged phase: drains the event and compares targets.
    publish_mode_change(&mut world);
    schedule.run(&mut world);
    world.flush();

    let water_state = *world.get::<UiInteractionState>(water).unwrap();
    let path_state = *world.get::<UiInteractionState>(path).unwrap();
    assert_eq!(water_state.selected, 1, "target equals current AppMode::Water");
    assert_eq!(path_state.selected, 0, "target differs from current mode");
    let animation = *world
        .get::<UiPlaySelectAnimation>(water)
        .expect("matching button plays the select animation");
    assert_eq!(animation.a, 0.2);
    assert_eq!(animation.b, 0.2);
    assert_eq!(animation.progress, 0.0);
    assert!(world.get::<UiPlaySelectAnimation>(path).is_none());
}

#[test]
fn mode_change_is_ignored_when_no_event_was_written() {
    let mut world = World::new();
    world.init_resource::<Events<UiClickEvent>>();
    world.init_resource::<Events<RequestAppModeChange>>();
    world.init_resource::<Events<ModeJustChanged>>();
    world.insert_resource(AppMode::Path);

    let button = world
        .spawn((
            SwitchAppModeButton {
                mode: AppMode::Path,
            },
            UiInteractionState {
                state: 3,
                keep: 1,
                selected: 0,
            },
        ))
        .id();

    let mut schedule = Schedule::default();
    schedule.add_systems(toolbar::toolbar_button::switch_app_mode_button);
    schedule.run(&mut world);
    world.flush();

    // Without a ModeJustChanged event the per-widget comparison is skipped;
    // the keep byte and selected byte keep their previous values.
    let state = *world.get::<UiInteractionState>(button).unwrap();
    assert_eq!(state.selected, 0);
    assert_eq!(state.keep, 1);
    assert!(world.get::<UiPlaySelectAnimation>(button).is_none());
}

#[test]
fn canonical_publisher_retains_complete_modes_and_suppresses_unchanged_mode() {
    use country_core::systems::mode_manager::{AppModeChanged, EditSubject, PreviousAppMode};
    assert_eq!(std::any::TypeId::of::<ModeJustChanged>(), std::any::TypeId::of::<AppModeChanged>());
    let previous = AppMode::Editing(EditSubject::Wall(0xfedc_ba98_7654_3210));
    let current = AppMode::Editing(EditSubject::DecoClutter { decorator_id: 0xdead_beef });
    let mut world = World::new();
    world.init_resource::<Events<ModeJustChanged>>();
    world.insert_resource(PreviousAppMode(previous));
    world.insert_resource(current);
    publish_mode_change(&mut world);
    let events: Vec<_> = world.resource_mut::<Events<ModeJustChanged>>().drain().collect();
    assert_eq!(events, vec![AppModeChanged { current, previous }]);
    assert_eq!(world.resource::<PreviousAppMode>().0, current);
    publish_mode_change(&mut world);
    assert_eq!(world.resource::<Events<ModeJustChanged>>().len(), 0);
    world.insert_resource(previous);
    publish_mode_change(&mut world);
    let events: Vec<_> = world.resource_mut::<Events<ModeJustChanged>>().drain().collect();
    assert_eq!(events, vec![AppModeChanged { current: previous, previous: current }]);
}
