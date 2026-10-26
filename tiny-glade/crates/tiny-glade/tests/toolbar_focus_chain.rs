//! Real toolbar focus publisher -> canonical event -> onboarding consumer.
//! This CPU integration boundary starts at the toolbar interaction component;
//! it does not claim OS input, rendered hints, or full module acceptance.
use bevy_ecs::prelude::*;
use country_core::{
    components::UiInteractionState,
    resources::{gui::EscMenuState, toolbar::ToolbarUiIsInFocus},
    systems::mode_manager::AppMode,
};
use system_onboarding::hint_orchestrator::{
    hint_orchestrator, BindingHintDisplay, DisplayHintRequest, HintType, LingerHintRequest,
};

fn fixture() -> (World, Schedule, Entity) {
    let mut world = World::new();
    world.init_resource::<Events<DisplayHintRequest>>();
    world.init_resource::<Events<LingerHintRequest>>();
    // Exactly the event owner used by ToolbarPlugin; no local mirror/bridge.
    world.init_resource::<Events<ToolbarUiIsInFocus>>();
    world.init_resource::<AppMode>();
    world.init_resource::<EscMenuState>();
    let item = world.spawn(UiInteractionState { state: 1, keep: 1, selected: 0 }).id();
    let mut schedule = Schedule::default();
    schedule.add_systems((
        toolbar::ui_interaction::ui_notify_hints_if_toolbar_in_focus,
        hint_orchestrator,
    ).chain());
    (world, schedule, item)
}

fn request(world: &mut World, kind: HintType) {
    world.resource_mut::<Events<DisplayHintRequest>>().send(
        DisplayHintRequest::new(BindingHintDisplay::Key(None), kind, 0),
    );
}

fn hints(world: &mut World) -> Vec<Entity> {
    world.query_filtered::<Entity, With<HintType>>().iter(world).collect()
}

#[test]
fn real_toolbar_focus_suppresses_hint_then_release_consumes_only_new_requests() {
    let (mut world, mut schedule, item) = fixture();
    request(&mut world, HintType::Terrain);
    schedule.run(&mut world);
    assert!(hints(&mut world).is_empty());

    world.component_mut::<UiInteractionState>(item).unwrap().keep = 0;
    schedule.run(&mut world);
    assert!(hints(&mut world).is_empty(), "suppressed request must already be consumed");

    request(&mut world, HintType::Terrain);
    schedule.run(&mut world);
    let entities = hints(&mut world);
    assert_eq!(entities.len(), 1);
    assert_eq!(world.get::<HintType>(entities[0]), Some(&HintType::Terrain));
}

#[test]
fn focused_toolbar_does_not_block_menu_deletion_or_linger_special_hint() {
    let (mut world, mut schedule, _) = fixture();
    let old_hint = world.spawn(HintType::Terrain).id();
    *world.resource_mut::<EscMenuState>() = EscMenuState::Main;
    schedule.run(&mut world);
    assert!(world.get::<system_onboarding::hint_display_system::HintMarkedForDeletion>(old_hint).is_some());

    world.despawn(old_hint);
    *world.resource_mut::<EscMenuState>() = EscMenuState::Closed;
    world.resource_mut::<Events<LingerHintRequest>>().send(LingerHintRequest {
        display: BindingHintDisplay::Empty,
        hint_type: HintType::Rollercoaster,
        duration: 1.0,
    });
    request(&mut world, HintType::Rollercoaster);
    schedule.run(&mut world);
    assert_eq!(hints(&mut world).len(), 1);
}

use country_core::render::extract::WorldComponentWrite as _;
