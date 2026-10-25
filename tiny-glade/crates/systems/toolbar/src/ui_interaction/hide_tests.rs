use super::*;

fn fixture() -> (World, Schedule, Entity, Entity, Entity) {
    let mut world = World::new();
    world.insert_resource(EscMenuState::Closed);
    world.insert_resource(WelcomeScreenState::Closed);
    world.insert_resource(PhotomodeScreen::default());
    world.insert_resource(PhotomodeState::default());
    world.insert_resource(SteamdeckScreenshotMsgScreen::default());
    world.insert_resource(UiNavState::default());
    let initial = UiInteractionState {
        state: 77,
        keep: 88,
        selected: 99,
    };
    let normal = world.spawn((initial, HideIfMenuIsOpen)).id();
    let photo = world
        .spawn((initial, HideIfMenuIsOpen, ShowOnlyInPhotomode))
        .id();
    let unaffected = world.spawn(initial).id();
    let mut schedule = Schedule::default();
    schedule.add_systems(hide_ui);
    (world, schedule, normal, photo, unaffected)
}

#[test]
fn hide_ui_all_native_menu_and_photomode_gates() {
    let (mut world, mut schedule, normal, photo, unaffected) = fixture();
    // Truth table derives from the bounded native branches, not widget names.
    for gates in 0u8..32 {
        let esc_open = gates & 1 != 0;
        let welcome_open = gates & 2 != 0;
        let deck_message = gates & 4 != 0;
        let photo_enabled = gates & 8 != 0;
        let photo_hidden = gates & 16 != 0;
        *world.resource_mut::<EscMenuState>() = if esc_open {
            EscMenuState::Main
        } else {
            EscMenuState::Closed
        };
        *world.resource_mut::<WelcomeScreenState>() = if welcome_open {
            WelcomeScreenState::Pending
        } else {
            WelcomeScreenState::Closed
        };
        world.resource_mut::<SteamdeckScreenshotMsgScreen>().0 = deck_message;
        world.resource_mut::<PhotomodeScreen>().enabled = photo_enabled;
        world.resource_mut::<PhotomodeState>().hide_ui = photo_hidden;
        schedule.run(&mut world);
        let closed = !esc_open && !welcome_open && !deck_message;
        assert_eq!(
            world.get::<UiInteractionState>(normal).unwrap().state,
            u8::from(closed && !photo_enabled),
            "gates={gates}"
        );
        assert_eq!(
            world.get::<UiInteractionState>(photo).unwrap().state,
            u8::from(closed && photo_enabled && !photo_hidden),
            "gates={gates}"
        );
        for entity in [normal, photo, unaffected] {
            let state = world.get::<UiInteractionState>(entity).unwrap();
            assert_eq!((state.keep, state.selected), (88, 99));
        }
        assert_eq!(
            world.get::<UiInteractionState>(unaffected).unwrap().state,
            77
        );
    }
}

#[test]
fn hide_ui_invalidates_only_toolbar_focus_and_keeps_payloads() {
    let (mut world, mut schedule, normal, photo, _) = fixture();
    for tag in 0..4 {
        let f = UiFocus {
            tag,
            _pad: 0,
            payload: photo.to_bits(),
        };
        world.resource_mut::<UiNavState>().focus = f;
        world.resource_mut::<UiNavState>().staged_focus = f;
        schedule.run(&mut world);
        let nav = world.resource::<UiNavState>();
        assert_eq!(nav.focus.tag, if tag == 1 { 0 } else { tag });
        assert_eq!(nav.staged_focus.tag, if tag == 1 { 0 } else { tag });
        assert_eq!(nav.focus.payload, photo.to_bits());
        assert_eq!(nav.staged_focus.payload, photo.to_bits());
    }
    let visible = UiFocus {
        tag: 1,
        _pad: 0,
        payload: normal.to_bits(),
    };
    world.resource_mut::<UiNavState>().focus = visible;
    world.resource_mut::<UiNavState>().staged_focus = visible;
    schedule.run(&mut world);
    assert_eq!(world.resource::<UiNavState>().focus, visible);
    assert_eq!(world.resource::<UiNavState>().staged_focus, visible);
    // A menu opening clears even a current toolbar focus absent from the query;
    // only matching hidden entities clear the staged slot.
    let stale = UiFocus {
        tag: 1,
        _pad: 0,
        payload: 0x12345678,
    };
    world.resource_mut::<UiNavState>().focus = stale;
    world.resource_mut::<UiNavState>().staged_focus = stale;
    *world.resource_mut::<EscMenuState>() = EscMenuState::Main;
    schedule.run(&mut world);
    assert_eq!(world.resource::<UiNavState>().focus.tag, 0);
    assert_eq!(world.resource::<UiNavState>().focus.payload, stale.payload);
    assert_eq!(world.resource::<UiNavState>().staged_focus, stale);
}

#[test]
fn hide_ui_reentry_recomputes_visibility_without_reusing_entity_focus() {
    let (mut world, mut schedule, normal, _, _) = fixture();
    world.resource_mut::<UiNavState>().staged_focus = UiFocus {
        tag: 1,
        _pad: 0,
        payload: normal.to_bits(),
    };
    *world.resource_mut::<WelcomeScreenState>() = WelcomeScreenState::Showing { elapsed: 0.2 };
    schedule.run(&mut world);
    assert_eq!(world.resource::<UiNavState>().staged_focus.tag, 0);
    world.despawn(normal);
    let next = world
        .spawn((UiInteractionState::default(), HideIfMenuIsOpen))
        .id();
    *world.resource_mut::<WelcomeScreenState>() = WelcomeScreenState::Closed;
    schedule.run(&mut world);
    assert_eq!(world.get::<UiInteractionState>(next).unwrap().state, 1);
    assert_eq!(world.resource::<UiNavState>().staged_focus.tag, 0);
    assert_ne!(next, normal);
}
