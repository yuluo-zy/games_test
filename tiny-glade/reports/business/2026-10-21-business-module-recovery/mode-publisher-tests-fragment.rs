
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
