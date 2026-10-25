use super::*;
use crate::toolbar_update::ToolbarSizeMult;
use bevy_ecs::prelude::*;
use country_core::{
    components::Ui2dExtent,
    game_config::GameConfig,
    input::action_set::ActionSetStates,
    resources::mouse_terrain_raycast::CursorPositionSS,
    systems::{
        backend::input::calculate_input_delta::UiHistoryButtonPressed,
        mode_manager::AppMode,
        ui::{
            resources::{ui_state::UiState, widget_params::WidgetGeometry},
            signifiers::signifier_create::UiSignifierStream,
        },
    },
};
use glam::Vec2;

#[test]
fn toolbar_registration_keeps_screen_center_depth_and_occlusion_policy() {
    let mut world = World::new();
    world.insert_resource(UiState::at_startup());
    world.insert_resource(UiNavState::default());
    world.insert_resource(ActionSetStates::default());
    world.insert_resource(AppMode::Photomode);
    world.insert_resource(UiSignifierStream::default());
    world.insert_resource(CursorPositionSS(Vec2::ZERO));
    let mut config = GameConfig::default();
    config.ui.icon_size_mult = 1.5;
    world.insert_resource(config);
    world.insert_resource(ToolbarSizeMult { multiplier: 0.75 });
    world.init_resource::<Events<UiClickEvent>>();
    world.init_resource::<Events<UiDragEvent>>();
    world.init_resource::<Events<UiHistoryButtonPressed>>();
    let entity = world
        .spawn((
            Ui2dExtent {
                pos: Vec2::new(300., 210.),
                size: Vec2::new(60., 40.),
            },
            UiMouseInteractable {
                bytes: (-0.125f32).to_le_bytes(),
            },
            TaffyNode::default(),
            UiInteractionState {
                state: 1,
                keep: 0,
                selected: 0,
            },
        ))
        .id();
    let mut schedule = Schedule::default();
    schedule.add_systems(render_taffy_ui_to_widgets);
    schedule.run(&mut world);
    let name = format!(
        "toolbar_button_{:?}",
        world.get::<TaffyNode>(entity).unwrap().node
    );
    let state = world.resource::<UiState>();
    let params = state
        .widget(country_core::systems::ui::resources::ui_state::WidgetId::from_name(&name))
        .unwrap();
    assert!(!params.occlude_by_world);
    assert!(params.flags.no_history_recording());
    assert!(params.flags.allow_negative_distance());
    let half = (Vec2::new(60., 40.) + Vec2::splat(1.5 * 10. * 0.75)) * 0.5;
    match params.resolved_geometry().as_ref() {
        WidgetGeometry::ScreenAabb { min, max, distance } => {
            assert_eq!(*min, Vec2::new(300., 210.) - half);
            assert_eq!(*max, Vec2::new(300., 210.) + half);
            assert_eq!(distance.to_bits(), (-0.125f32).to_bits());
        }
        other => panic!("wrong original toolbar volume: {other:?}"),
    }
    assert!(world.resource::<Events<UiClickEvent>>().is_empty());
    assert!(world
        .resource::<Events<UiHistoryButtonPressed>>()
        .is_empty());
}
