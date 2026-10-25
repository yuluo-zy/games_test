//! Real canonical Bevy system: no alternate slider dispatcher/input adapter.
use bevy_ecs::prelude::*;
use country_core::{
    components::{Ui2dExtent, Ui2dImage, UiDrawOrder},
    input::action_set::{ActionSetStates, DigitalActionPhase, DigitalActionState},
    resources::{
        brush_sizes::BrushSizes, image_formation::ImageFormationConfigs,
        mouse_terrain_raycast::CursorPositionSS, time::GameTime, walls::EraserSizeMultiplier,
    },
    systems::{
        camera_rig::main_camera::MainCamera,
        mode_manager::{AppMode, RequestToolbarCategoryChange},
        ui::resources::widget_params::{PlayUiSound, UiFocus, UiNavState},
    },
};
use glam::Vec2;
use toolbar::{
    photomode::ui_slider, resouces_and_components::{UiSlider, ToolbarSliderSession}, ui_interaction::UiDragEvent,
};

fn setup(mode: AppMode) -> (World, Schedule) {
    let mut world = World::new();
    world.init_resource::<Events<UiDragEvent>>();
    world.init_resource::<Events<PlayUiSound>>();
    world.init_resource::<Events<RequestToolbarCategoryChange>>();
    world.init_resource::<GameTime>();
    world.init_resource::<UiNavState>();
    world.init_resource::<ToolbarSliderSession>();
    world.init_resource::<ActionSetStates>();
    world.insert_resource(CursorPositionSS(Vec2::splat(-1000.)));
    world.insert_resource(MainCamera::new(1.));
    world.insert_resource(ImageFormationConfigs::default());
    world.insert_resource(BrushSizes {
        path: 0.8,
        water: 2.,
        garden: 3.,
    });
    world.insert_resource(EraserSizeMultiplier(1.));
    world.insert_resource(mode);
    let mut schedule = Schedule::default();
    schedule.add_systems(ui_slider);
    (world, schedule)
}

fn slider(world: &mut World, kind: u8, center: Vec2) -> Entity {
    let mut slider = UiSlider::default();
    slider.set_kind(kind);
    world
        .spawn((
            Ui2dExtent {
                pos: center,
                size: Vec2::new(400., 40.),
            },
            slider,
        ))
        .id()
}
fn drag(world: &mut World, entity: Entity, state: u8, x: f32) {
    world.resource_mut::<CursorPositionSS>().0 = Vec2::new(x, 100.);
    world
        .resource_mut::<Events<UiDragEvent>>()
        .send(UiDragEvent { entity, state });
}
fn focus(world: &mut World, entity: Entity) {
    world.resource_mut::<UiNavState>().focus = UiFocus {
        tag: 1,
        _pad: 0,
        payload: entity.to_bits(),
    };
}

#[test]
fn knob_is_recycled_every_frame_and_zero_extent_removes_it() {
    let (mut world, mut schedule) = setup(AppMode::Photomode);
    let entity = slider(&mut world, 0, Vec2::new(100., 100.));
    let mut generations = Vec::new();
    for _ in 0..5 {
        schedule.run(&mut world);
        let knob = world
            .get::<UiSlider>(entity)
            .unwrap()
            .knob_entity()
            .unwrap();
        for old in &generations {
            assert!(!world.entities().contains(*old));
        }
        assert!(!generations.contains(&knob));
        generations.push(knob);
        assert_eq!(
            world.get::<Ui2dImage>(knob).unwrap().name,
            "toolbar/slider_widget"
        );
        assert_eq!(world.get::<UiDrawOrder>(knob).unwrap().0, 1);
        let extent = world.get::<Ui2dExtent>(knob).unwrap();
        assert_eq!(extent.pos, Vec2::new(100., 100.));
        assert_eq!(extent.size, Vec2::splat(12.));
        assert_eq!(world.query::<&Ui2dImage>().iter(&world).count(), 1);
    }
    world.component_mut::<Ui2dExtent>(entity).unwrap().size = Vec2::ZERO;
    schedule.run(&mut world);
    assert!(world
        .get::<UiSlider>(entity)
        .unwrap()
        .knob_entity()
        .is_none());
    assert_eq!(world.query::<&Ui2dImage>().iter(&world).count(), 0);
}

#[test]
fn grab_preserves_offset_and_finish_and_cancel_write_the_release_position() {
    let (mut world, mut schedule) = setup(AppMode::Photomode);
    let entity = slider(&mut world, 0, Vec2::new(100., 100.));
    // Existing knob center=100. A start within the knob keeps the 4px grab offset.
    drag(&mut world, entity, 3, 104.);
    schedule.run(&mut world);
    assert_eq!(
        world
            .resource::<ImageFormationConfigs>()
            .current()
            .params_18[1],
        0.
    );
    // Original phase 1 still applies cursor+offset; it is not discarded.
    drag(&mut world, entity, 1, 289.);
    schedule.run(&mut world);
    assert_eq!(
        world
            .resource::<ImageFormationConfigs>()
            .current()
            .params_18[1],
        2.5
    );
    // The original Local retains the grab offset across finished events.
    drag(&mut world, entity, 0, -81.);
    schedule.run(&mut world);
    assert_eq!(
        world
            .resource::<ImageFormationConfigs>()
            .current()
            .params_18[1],
        -2.5
    );
    assert_eq!(world.get::<UiSlider>(entity).unwrap().hover_progress(), 1.);
}

#[test]
fn only_last_drag_candidate_writes_and_sounds_are_once_per_final_bin() {
    let (mut world, mut schedule) = setup(AppMode::Photomode);
    let exposure = slider(&mut world, 0, Vec2::new(100., 100.));
    let contrast = slider(&mut world, 1, Vec2::new(-270., 100.));
    // Both see cursor x=-85. This is exposure's low end and contrast's high end.
    world.resource_mut::<CursorPositionSS>().0 = Vec2::new(-85., 100.);
    for entity in [exposure, contrast] {
        world
            .resource_mut::<Events<UiDragEvent>>()
            .send(UiDragEvent { entity, state: 3 });
    }
    schedule.run(&mut world);
    let config = world.resource::<ImageFormationConfigs>().current();
    assert_eq!(config.params_18[1], 0.);
    assert_eq!(config.params_18[2], 1.4);
    assert_eq!(
        world
            .resource_mut::<Events<PlayUiSound>>()
            .drain()
            .map(|e| e.kind)
            .collect::<Vec<_>>(),
        vec![0x1c]
    );
    // Same candidate/bin on the next event produces no repeated sound.
    drag(&mut world, contrast, 1, -85.);
    schedule.run(&mut world);
    assert_eq!(
        world.resource_mut::<Events<PlayUiSound>>().drain().count(),
        0
    );
}

#[test]
fn tool_completion_uses_canonical_eraser_and_requests_the_actual_category() {
    let (mut world, mut schedule) = setup(AppMode::Photomode);
    let entity = slider(&mut world, 9, Vec2::new(100., 100.));
    drag(&mut world, entity, 3, -85.);
    schedule.run(&mut world);
    // Photomode still prevents eraser writes, but normalized/bin state changes.
    assert_eq!(world.resource::<EraserSizeMultiplier>().0, 1.);
    assert_eq!(
        world.get::<UiSlider>(entity).unwrap().previous_value_01(),
        0.
    );
    world.insert_resource(AppMode::Path);
    drag(&mut world, entity, 0, 285.);
    schedule.run(&mut world);
    assert_eq!(world.resource::<EraserSizeMultiplier>().0, 2.);
    assert_eq!(world.resource::<BrushSizes>().path, 0.8);
    let categories = world
        .resource_mut::<Events<RequestToolbarCategoryChange>>()
        .drain()
        .map(|e| (e.0).0)
        .collect::<Vec<_>>();
    assert_eq!(categories, vec![4]);
}

#[test]
fn navigation_requires_previous_focus_and_uses_game_clock_delta() {
    let (mut world, mut schedule) = setup(AppMode::Photomode);
    let entity = slider(&mut world, 0, Vec2::new(100., 100.));
    focus(&mut world, entity);
    world.resource_mut::<GameTime>().delta = 0.1;
    world.resource_mut::<ActionSetStates>().digital[21] = DigitalActionState {
        phase: DigitalActionPhase::JustPressed,
        timer_seconds: 0.,
    };
    schedule.run(&mut world);
    // Initial focus has not yet matched the system's persisted previous focus.
    assert_eq!(
        world
            .resource::<ImageFormationConfigs>()
            .current()
            .params_18[1],
        0.
    );
    world.resource_mut::<ActionSetStates>().digital[21] = DigitalActionState::default();
    schedule.run(&mut world);
    world.resource_mut::<ActionSetStates>().digital[21] = DigitalActionState {
        phase: DigitalActionPhase::JustPressed,
        timer_seconds: 0.,
    };
    schedule.run(&mut world);
    assert!(
        (world
            .resource::<ImageFormationConfigs>()
            .current()
            .params_18[1]
            - 0.5)
            .abs()
            < 1e-6
    );
    world.resource_mut::<GameTime>().delta = 0.;
    world.resource_mut::<ActionSetStates>().digital[21] = DigitalActionState {
        phase: DigitalActionPhase::Held,
        timer_seconds: 0.,
    };
    schedule.run(&mut world);
    assert!(
        (world
            .resource::<ImageFormationConfigs>()
            .current()
            .params_18[1]
            - 0.5)
            .abs()
            < 1e-6
    );
}

#[test]
fn nonfinite_final_candidate_skips_write_but_preserves_finish_category() {
    let (mut world, mut schedule) = setup(AppMode::Path);
    let entity = slider(&mut world, 11, Vec2::new(100., 100.));
    drag(&mut world, entity, 3, f32::NAN);
    schedule.run(&mut world);
    assert_eq!(world.resource::<BrushSizes>().water, 2.);
    assert_eq!(
        world.resource_mut::<Events<PlayUiSound>>().drain().count(),
        0
    );
    drag(&mut world, entity, 1, f32::NAN);
    schedule.run(&mut world);
    assert_eq!(
        world
            .resource_mut::<Events<RequestToolbarCategoryChange>>()
            .drain()
            .map(|e| (e.0).0)
            .collect::<Vec<_>>(),
        vec![8]
    );
    assert_eq!(world.resource::<BrushSizes>().water, 2.);
}

#[test]
fn reset_chromatic_and_focus_use_the_original_parameter_defaults() {
    use toolbar::resouces_and_components::UiSliderReset;
    use toolbar::ui_interaction::UiClickEvent;
    let (mut world, _) = setup(AppMode::Photomode);
    world.init_resource::<Events<UiClickEvent>>();
    for row in world
        .resource_mut::<ImageFormationConfigs>()
        .iter_non_default_mut()
    {
        row.params_30[1] = 9.;
    }
    let focus = 0x170;
    {
        let mut camera = world.resource_mut::<MainCamera>();
        camera.write_f32(focus, f32::from_bits(1));
        camera.write_f32(focus + 0x10, 17.);
        camera.write_f32(focus + 0x14, -23.);
    }
    for flag in [5, 7] {
        let entity = world.spawn(UiSliderReset { flag }).id();
        world
            .resource_mut::<Events<UiClickEvent>>()
            .send(UiClickEvent { entity });
    }
    let mut schedule = Schedule::default();
    schedule.add_systems(toolbar::photomode::ui_slider_reset);
    schedule.run(&mut world);
    for row in world
        .resource::<ImageFormationConfigs>()
        .configs
        .iter()
        .skip(1)
    {
        assert_eq!(row.params_30[1].to_bits(), 0x3c75c28f);
    }
    let camera = world.resource::<MainCamera>();
    // Native reset 14234849a..4f8 produces exp2(.5-3), not exp2(-3).
    assert_eq!(camera.read_f32(focus + 0x18).to_bits(), 0x3e3504f3);
    assert_eq!(camera.read_f32(focus).to_bits(), 3);
    assert_eq!(
        [camera.read_f32(focus + 4), camera.read_f32(focus + 8)],
        [17., -23.]
    );
}


#[test]
fn session_reload_discards_slider_grab_and_armed_navigation_before_new_focus() {
    let (mut world, mut schedule) = setup(AppMode::Photomode);
    let original = slider(&mut world, 0, Vec2::new(100., 100.));
    focus(&mut world, original);
    world.resource_mut::<GameTime>().delta = 0.1;
    schedule.run(&mut world); // Establish the original previous focus.
    world.resource_mut::<ActionSetStates>().digital[21] = DigitalActionState {
        phase: DigitalActionPhase::JustPressed, timer_seconds: 0.,
    };
    drag(&mut world, original, 3, 104.);
    schedule.run(&mut world); // Both navigation and grab state are populated.
    assert_ne!(*world.resource::<ToolbarSliderSession>(), ToolbarSliderSession::default());
    world.resource_mut::<ActionSetStates>().digital[21] = DigitalActionState {
        phase: DigitalActionPhase::Held, timer_seconds: 0.,
    };
    let knob = world.get::<UiSlider>(original).unwrap().knob_entity().unwrap();
    assert!(world.despawn(knob));
    assert!(world.despawn(original));
    world.resource_mut::<Events<UiDragEvent>>().clear();
    *world.resource_mut::<UiNavState>() = UiNavState::default();
    world.resource_mut::<ToolbarSliderSession>().clear_session();
    assert_eq!(*world.resource::<ToolbarSliderSession>(), ToolbarSliderSession::default());
    let replacement = slider(&mut world, 0, Vec2::new(100., 100.));
    assert_ne!(original, replacement);
    focus(&mut world, replacement);
    let unchanged = world.resource::<ImageFormationConfigs>().current().params_18[1];
    for _ in 0..3 {
        schedule.run(&mut world);
        assert_eq!(world.resource::<ImageFormationConfigs>().current().params_18[1], unchanged,
            "Held direction must not inherit the old session's armed state");
    }
    // A continuation without a new start must not reuse the previous grab.
    drag(&mut world, replacement, 1, 285.);
    schedule.run(&mut world);
    assert_eq!(world.resource::<ImageFormationConfigs>().current().params_18[1], unchanged);
    // Fresh release/press and grab edges still drive the real consumer.
    world.resource_mut::<ActionSetStates>().digital[21] = DigitalActionState::default();
    schedule.run(&mut world);
    world.resource_mut::<ActionSetStates>().digital[21] = DigitalActionState {
        phase: DigitalActionPhase::JustPressed, timer_seconds: 0.,
    };
    schedule.run(&mut world);
    assert!(world.resource::<ImageFormationConfigs>().current().params_18[1] > unchanged);
    drag(&mut world, replacement, 3, 285.);
    schedule.run(&mut world);
    assert_eq!(world.resource::<ImageFormationConfigs>().current().params_18[1], 2.5);
}

use country_core::render::extract::WorldComponentWrite as _;
