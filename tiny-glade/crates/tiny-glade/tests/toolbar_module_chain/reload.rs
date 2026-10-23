//! Host lifecycle repair exercised through the real named OnExit schedule,
//! physical device pipeline, toolbar constructor, picker and category consumer.
use super::*;
use bevy_state::state::OnExit;
use country_core::app_loading_state::AppLoadingState;
use std::sync::Arc;

#[test]
fn held_mouse_reload_releases_old_owner_and_requires_a_fresh_physical_press() {
    let mut h = Harness::new();
    let outside = Vec2::ZERO;
    for _ in 0..24 {
        h.frame(false, outside);
    }
    h.frame(false, h.position());
    h.frame(true, h.position());
    h.frame(true, h.position());
    assert_eq!(h.world.resource::<UiState>().click_byte(), 4);
    let old_entity = h.entity;
    let old_widget = h.widget;
    let old_params = {
        let owned = h.world.resource::<UiState>().active_params().unwrap();
        Arc::downgrade(&owned)
    };
    let mut mei = toolbar::MeiContext::default();
    let id = toolbar::GlobalId::new("old-session-node");
    mei.nodes.insert(
        id,
        toolbar::Node::new(Box::new(toolbar::mei::core::EmptyContent)),
    );
    mei.input_events
        .push(toolbar::mei::core::MeiInputEvent::scroll(Vec2::ONE));
    let fonts = Arc::clone(&mei.font_table);
    h.world.insert_resource(mei);
    let mut navigation = h.world.resource_mut::<UiNavState>();
    navigation.timer = Some(std::time::Instant::now());
    navigation.hold = true;
    // Photo -> Replay can retain these transient owners until session exit.
    h.world.insert_resource(country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen(true));
    h.world.insert_resource(country_core::resources::gui::PhotomodeScreen {
        enabled: true, was_enabled: true,
    });

    let mut exit = Schedule::new(OnExit(AppLoadingState::Game));
    exit.add_systems(
        (
            tiny_glade::toolbar_plugin::clear_toolbar_session,
            country_core::startup::clearing::despawn_glade_entities,
        )
            .chain(),
    );
    h.world.add_schedule(exit);
    h.world.run_schedule(OnExit(AppLoadingState::Game));
    h.world.flush();
    let ui = h.world.resource::<UiState>();
    assert!(ui.registry_snapshot().is_empty());
    assert!(ui.hover_intersections().is_empty());
    assert!(ui.active_params().is_none());
    assert_eq!(ui.hover_byte(), 0);
    assert_eq!(ui.intersection_tag(), 2);
    assert_eq!(*h.world.resource::<UiNavState>(), UiNavState::default());
    assert!(!h.world.resource::<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>().0);
    let photo_screen = h.world.resource::<country_core::resources::gui::PhotomodeScreen>();
    assert!(!photo_screen.enabled && !photo_screen.was_enabled);
    assert!(h.world.get_entity(old_entity).is_err());
    let mei = h.world.resource::<toolbar::MeiContext>();
    assert!(mei.nodes.is_empty() && mei.input_events.is_empty());
    assert!(Arc::ptr_eq(&fonts, &mei.font_table));
    let cursor = h.world.resource::<Events<OsCursorVisibilityCmd>>();
    assert_eq!(
        cursor.get_cursor().read(cursor).last(),
        Some(&OsCursorVisibilityCmd::Unsuppress(
            country_core::resources::mouse_capture::OsCursorVisibilityCmdOrigin(1)
        ))
    );

    let mut startup = Schedule::default();
    startup.add_systems(toolbar::toolbar_startup::toolbar_setup);
    startup.run(&mut h.world);
    h.world.flush();
    h.entity = h
        .world
        .query::<(Entity, &SwitchToolbarCategoryButton)>()
        .iter(&h.world)
        .find(|(_, button)| button.category == ToolbarCategory::Water)
        .unwrap()
        .0;
    let node = h.world.get::<TaffyNode>(h.entity).unwrap().node;
    h.widget = WidgetId::from_name(&format!("toolbar_button_{node:?}"));
    assert_ne!(h.entity, old_entity);
    assert_eq!(
        h.widget, old_widget,
        "recreated Taffy IDs must not resurrect captured ownership"
    );
    // Physical state remains Held across the actual unload and new constructor.
    // Settle layout while held; ui_update must not turn it into a new press.
    for _ in 0..24 {
        h.frame(true, outside);
    }
    assert!(
        old_params.upgrade().is_none(),
        "picker's frame cache must also release the old Arc"
    );
    h.frame(true, h.position());
    h.frame(false, h.position());
    assert_eq!(*h.world.resource::<AppMode>(), AppMode::Path);
    assert!(h.world.resource::<Events<UiClickEvent>>().is_empty());
    h.frame(true, h.position());
    h.frame(false, h.position());
    assert_eq!(*h.world.resource::<AppMode>(), AppMode::Water);
    let clicks = h.world.resource::<Events<UiClickEvent>>();
    assert_eq!(
        clicks
            .get_cursor()
            .read(clicks)
            .map(|event| event.entity)
            .collect::<Vec<_>>(),
        [h.entity]
    );
}
