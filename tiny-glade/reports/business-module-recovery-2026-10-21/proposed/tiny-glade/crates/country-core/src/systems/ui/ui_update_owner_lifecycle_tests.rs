//! Additional owner-lifecycle test, not a duplicate of the three input/history
//! chain tests. The tiny ECS registration producer is a fixture boundary; the
//! picker, ui_update, Bevy state transitions and glade entity deletion are real.
//! No test call clears the registry or replaces UiState.interaction.
use super::*;
use crate::{
    app_loading_state::AppLoadingState,
    components::KeepOnLevelUnload,
    input::action_set::{DigitalActionPhase, DigitalActionState},
    resources::{
        cursor_terrain_raycast::CursorTerrainRaycast,
        mouse_terrain_raycast::CursorPositionSS,
        read_mouse_depth_value::MouseDepthValue,
        window::WindowSize,
    },
    startup::clearing::despawn_glade_entities,
    systems::{camera_rig::MainCamera, ui::hover_intersection::hover_intersection},
};
use bevy_app::{App, Update};
use bevy_state::{app::{AppExtStates, StatesPlugin}, prelude::*};
use glam::{Mat4, Vec2};

// Toolbar registration derives a stable name from a rebuilt layout node, so a
// new session may register the same WidgetId for a different Entity generation.
const TARGET: WidgetId = WidgetId(0x5649_445f_5245_5553);

#[derive(Component)]
struct RegisteredTarget;

#[derive(Resource, Default)]
struct ObservedClicks(Vec<Entity>);

fn spawn_target(mut commands: Commands) {
    commands.spawn(RegisteredTarget);
}

fn register_live_target(targets: Query<Entity, With<RegisteredTarget>>, ui: Res<UiState>) {
    for _ in &targets {
        ui.add_with_id(
            TARGET,
            WidgetParams::default()
                .geometry(WidgetGeometry::ScreenAabb {
                    min: Vec2::splat(40.0),
                    max: Vec2::splat(60.0),
                    distance: 0.5,
                })
                .no_history_recording()
                .occlude_by_world(false),
        );
    }
}

fn observe_clicks(
    targets: Query<Entity, With<RegisteredTarget>>,
    ui: Res<UiState>,
    mut clicks: ResMut<ObservedClicks>,
) {
    if ui.widget_context(TARGET).clicked() {
        clicks.0.extend(targets.iter());
    }
}

fn setup() -> (App, Entity) {
    bevy_tasks::ComputeTaskPool::get_or_init(|| {
        bevy_tasks::TaskPoolBuilder::new().num_threads(2).build()
    });
    let mut app = App::new();
    let mut camera = MainCamera::new(1.0);
    for offset in [0x40, 0x80, 0xc0] {
        camera.write_matrix(offset, &Mat4::IDENTITY);
    }
    app.add_plugins(StatesPlugin)
        .init_state::<AppLoadingState>()
        .insert_resource(UiState::at_startup())
        .insert_resource(camera)
        .insert_resource(CursorTerrainRaycast::default())
        .insert_resource(CursorPositionSS(Vec2::splat(50.0)))
        .insert_resource(MouseDepthValue::default())
        .insert_resource(WindowSize::new(100, 100))
        .init_resource::<ActionSetStates>()
        .init_resource::<ObservedClicks>()
        .add_event::<WindowFocusEvent>()
        .add_event::<EscapeMenuJustOpened>()
        .add_event::<ShutdownGameCmd>()
        .add_event::<OverwriteBrushPreview>()
        .add_event::<OsCursorVisibilityCmd>()
        .add_event::<PlayUiSound>()
        .add_systems(OnEnter(AppLoadingState::Game), spawn_target)
        .add_systems(OnExit(AppLoadingState::Game), despawn_glade_entities)
        .add_systems(
            Update,
            // Original set6 consumes registrations from the preceding frame;
            // toolbar-like set7 producers register after arbitration/consumers.
            (hover_intersection, ui_update, observe_clicks, register_live_target).chain(),
        );
    let kept = app.world_mut().spawn(KeepOnLevelUnload).id();
    app.world_mut().resource_mut::<NextState<AppLoadingState>>()
        .set(AppLoadingState::Game);
    (app, kept)
}

fn step(app: &mut App, phase: DigitalActionPhase) {
    app.world_mut().resource_mut::<ActionSetStates>().digital[0] = DigitalActionState {
        phase,
        timer_seconds: 0.0,
    };
    app.update();
}

fn target_entity(app: &mut App) -> Entity {
    let mut query = app.world_mut().query_filtered::<Entity, With<RegisteredTarget>>();
    let targets: Vec<_> = query.iter(app.world()).collect();
    assert_eq!(targets.len(), 1);
    targets[0]
}

#[test]
fn forced_end_then_actual_reload_releases_old_capture_before_widget_id_reuse() {
    for shutdown in [false, true] {
        let (mut app, kept) = setup();
        // OnEnter creates the entity; its first frame registers the target for
        // the following frame, exactly as the production toolbar does.
        step(&mut app, DigitalActionPhase::Held);
        step(&mut app, DigitalActionPhase::JustPressed);
        let old_entity = target_entity(&mut app);
        assert_eq!(app.world().resource::<UiState>().hover_byte(), 2);
        assert_eq!(app.world().resource::<UiState>().click_byte(), 3);
        let old_capture = {
            let params = app.world().resource::<UiState>().active_params().unwrap();
            Arc::downgrade(&params)
        };
        assert!(old_capture.upgrade().is_some());
        assert_eq!(app.world().resource::<UiState>().registry_snapshot().len(), 1);

        // The original forced-end rule compares current hit identity. Moving
        // outside makes this cancellation; a same-target forced finish is
        // already covered by menu_and_last_focus_payload_use_original_forced_end_rules.
        app.world_mut().resource_mut::<CursorPositionSS>().0 = Vec2::splat(90.0);
        if shutdown {
            app.world_mut().send_event(ShutdownGameCmd);
        } else {
            app.world_mut().send_event(EscapeMenuJustOpened);
        }
        step(&mut app, DigitalActionPhase::Held);
        assert_eq!(app.world().resource::<UiState>().click_byte(), 0);
        assert!(old_capture.upgrade().is_some(), "phase-end still owns its captured params");
        assert!(app.world().resource::<ObservedClicks>().0.is_empty());

        app.world_mut().resource_mut::<NextState<AppLoadingState>>()
            .set(AppLoadingState::Reload);
        step(&mut app, DigitalActionPhase::Held);
        assert!(app.world().get_entity(old_entity).is_err());
        assert!(app.world().get_entity(kept).is_ok());
        let ui = app.world().resource::<UiState>();
        assert_eq!(ui.hover_byte(), 0);
        assert!(ui.active_params().is_none());
        assert!(ui.registry_snapshot().is_empty());
        assert!(ui.hover_intersections().is_empty());
        assert!(old_capture.upgrade().is_none(), "capture and picker cache must release old Arc");

        app.world_mut().resource_mut::<CursorPositionSS>().0 = Vec2::splat(50.0);
        app.world_mut().resource_mut::<NextState<AppLoadingState>>()
            .set(AppLoadingState::Game);
        step(&mut app, DigitalActionPhase::Held);
        let new_entity = target_entity(&mut app);
        assert_eq!(old_entity.index(), new_entity.index(), "exercise allocator slot reuse");
        assert_ne!(old_entity, new_entity, "the reused slot has a new generation");
        assert_eq!(app.world().resource::<UiState>().hover_byte(), 0);
        assert!(app.world().resource::<ObservedClicks>().0.is_empty());
        step(&mut app, DigitalActionPhase::Held);
        let ui = app.world().resource::<UiState>();
        assert_eq!(ui.active_widget(), TARGET, "same widget name is deliberately reused");
        assert_eq!(ui.hover_byte(), 1, "held input cannot restart the old captured drag");
        assert!(!ui.widget_context(TARGET).clicked());
        assert!(app.world().resource::<ObservedClicks>().0.is_empty());
        assert!(old_capture.upgrade().is_none());

        step(&mut app, DigitalActionPhase::JustReleased);
        assert!(app.world().resource::<ObservedClicks>().0.is_empty());
        step(&mut app, DigitalActionPhase::JustPressed);
        assert_eq!(app.world().resource::<UiState>().click_byte(), 3);
        step(&mut app, DigitalActionPhase::JustReleased);
        assert_eq!(app.world().resource::<ObservedClicks>().0, [new_entity]);
        assert!(old_capture.upgrade().is_none());
    }
}
