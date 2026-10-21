//! Original toolbar registration/replay event gates, tested through real ECS.
//! Interaction is seeded at this leaf boundary. This does not verify device
//! translation, actual window input, drawing, or module/performance acceptance.
use bevy_ecs::event::Events;
use bevy_ecs::prelude::*;
use country_core::components::{Ui2dExtent, UiInteractionState, UiMouseInteractable};
use country_core::game_config::GameConfig;
use country_core::input::action_set::ActionSetStates;
use country_core::resources::mouse_terrain_raycast::CursorPositionSS;
use country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed;
use country_core::systems::mode_manager::AppMode;
use country_core::systems::ui::resources::ui_state::{UiInteraction, UiState, WidgetId};
use country_core::systems::ui::resources::widget_params::{UiNavState, WidgetGeometry};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierEntry;
use country_core::systems::ui::signifiers::UiSignifierStream;
use glam::Vec2;
use taffy::style::Style;
use toolbar::resouces_and_components::{HistoryButton, TaffyNode, UiPlaySelectAnimation};
use toolbar::toolbar_update::ToolbarSizeMult;
use toolbar::ui_interaction::{UiClickEvent, UiDragEvent};

fn button_name(node: taffy::node::Node) -> String {
    format!("toolbar_button_{:?}", node)
}

fn new_node() -> taffy::node::Node {
    let mut taffy = taffy::node::Taffy::default();
    taffy.new_leaf(Style::default()).expect("taffy leaf")
}

struct Harness {
    world: World,
    schedule: Schedule,
}

impl Harness {
    fn new(app_mode: AppMode, interaction: UiInteraction, multiplier: f32) -> Self {
        let mut world = World::new();
        world.init_resource::<Events<UiClickEvent>>();
        world.init_resource::<Events<UiDragEvent>>();
        world.init_resource::<Events<UiHistoryButtonPressed>>();
        world.insert_resource(UiState::new(interaction));
        world.insert_resource(UiNavState::default());
        world.insert_resource(app_mode);
        world.insert_resource(UiSignifierStream::default());
        world.insert_resource(CursorPositionSS(Vec2::new(12.5, -3.0)));
        world.insert_resource(GameConfig::default());
        world.insert_resource(ActionSetStates::default());
        world.insert_resource(ToolbarSizeMult { multiplier });
        let mut schedule = Schedule::default();
        schedule.add_systems(toolbar::ui_interaction::render_taffy_ui_to_widgets);
        Self { world, schedule }
    }

    fn spawn_widget(&mut self, node: taffy::node::Node, state: UiInteractionState) -> Entity {
        self.world
            .spawn((
                Ui2dExtent {
                    pos: Vec2::new(10.0, 20.0),
                    size: Vec2::new(100.0, 40.0),
                },
                UiMouseInteractable::default(),
                TaffyNode {
                    node,
                    ..TaffyNode::default()
                },
                state,
            ))
            .id()
    }

    fn run(&mut self) {
        self.schedule.run(&mut self.world);
        self.world.flush();
    }

    fn clicks(&mut self) -> Vec<Entity> {
        self.world
            .resource_mut::<Events<UiClickEvent>>()
            .drain()
            .map(|event| event.entity)
            .collect()
    }

    fn drags(&mut self) -> Vec<(Entity, u8)> {
        self.world
            .resource_mut::<Events<UiDragEvent>>()
            .drain()
            .map(|event| (event.entity, event.state))
            .collect()
    }

    fn history(&mut self) -> Vec<u8> {
        self.world
            .resource_mut::<Events<UiHistoryButtonPressed>>()
            .drain()
            .map(|event| event.0)
            .collect()
    }
}

fn active_interaction(name: &str, tag: u8, phase: u8) -> UiInteraction {
    UiInteraction {
        native_tag: tag,
        native_phase: phase,
        widget: WidgetId::from_name(name),
    }
}

#[test]
fn screen_bounds_distance_and_native_input_flags_survive_registration() {
    let node = new_node();
    let name = button_name(node);
    let mut h = Harness::new(AppMode::Path, active_interaction(&name, 1, 0), 1.0);
    let e = h.spawn_widget(
        node,
        UiInteractionState {
            state: 1,
            keep: 1,
            selected: 0,
        },
    );
    h.world.get_mut::<UiMouseInteractable>(e).unwrap().bytes = (-0.125f32).to_le_bytes();
    h.run();
    let p = h
        .world
        .resource::<UiState>()
        .widget(WidgetId::from_name(&name))
        .unwrap();
    match p.resolved_geometry().as_ref() {
        WidgetGeometry::ScreenAabb { min, max, distance } => {
            assert_eq!(*min, Vec2::new(-45., -5.));
            assert_eq!(*max, Vec2::new(65., 45.));
            assert_eq!(distance.to_bits(), (-0.125f32).to_bits());
        }
        other => panic!("unexpected geometry {other:?}"),
    }
    assert!(!p.occlude_by_world);
    assert!(p.flags.no_history_recording());
    assert!(p.flags.allow_negative_distance());
    assert_eq!(p.extra_78, [0, 0]);
    assert_eq!(h.world.get::<UiInteractionState>(e).unwrap().keep, 0);
    assert!(h
        .world
        .resource_mut::<UiSignifierStream>()
        .take_entries()
        .is_empty());
}

#[test]
fn ordinary_and_photo_tools_emit_clicks_but_no_denied_input_signifier() {
    for mode in [AppMode::Path, AppMode::Photomode] {
        let node = new_node();
        let name = button_name(node);
        let mut h = Harness::new(mode, active_interaction(&name, 2, 1), 1.0);
        let e = h.spawn_widget(node, UiInteractionState::default());
        h.run();
        assert_eq!(h.clicks(), vec![e]);
        assert_eq!(h.drags(), vec![(e, 1)]);
        assert!(h
            .world
            .resource_mut::<UiSignifierStream>()
            .take_entries()
            .is_empty());
    }
}

#[test]
fn replay_ordinary_button_denies_events_and_enqueues_real_named_factory() {
    let node = new_node();
    let name = button_name(node);
    let mut h = Harness::new(AppMode::Replay, active_interaction(&name, 2, 1), 1.0);
    h.spawn_widget(
        node,
        UiInteractionState {
            state: 1,
            keep: 0,
            selected: 0,
        },
    );
    h.run();
    assert!(h.clicks().is_empty());
    assert!(h.drags().is_empty());
    assert!(h.history().is_empty());
    let entries = h.world.resource_mut::<UiSignifierStream>().take_entries();
    assert_eq!(entries.len(), 1);
    match &entries[0] {
        UiSignifierEntry::Factory(factory) => {
            assert!(factory.has_closure());
            assert_eq!(factory.name(), Some("deny_input_toolbar"));
        }
        UiSignifierEntry::Raw(_) => panic!("replay must retain the real cross factory"),
    }
    h.run();
    assert_eq!(
        h.world
            .resource_mut::<UiSignifierStream>()
            .take_entries()
            .len(),
        1
    );
}

#[test]
fn replay_inactive_button_does_not_emit_denied_marker_or_events() {
    let node = new_node();
    let name = button_name(node);
    let mut h = Harness::new(AppMode::Replay, active_interaction(&name, 0, 2), 1.0);
    h.spawn_widget(node, UiInteractionState::default());
    h.run();
    assert!(h.clicks().is_empty());
    assert!(h.drags().is_empty());
    assert!(h
        .world
        .resource_mut::<UiSignifierStream>()
        .take_entries()
        .is_empty());
}

#[test]
fn history_buttons_keep_press_animation_and_drag_including_during_replay() {
    for mode in [AppMode::Path, AppMode::Replay] {
        let node = new_node();
        let name = button_name(node);
        let mut h = Harness::new(mode, active_interaction(&name, 2, 3), 1.0);
        let e = h.spawn_widget(node, UiInteractionState::default());
        h.world.entity_mut(e).insert(HistoryButton::REDO);
        h.run();
        assert_eq!(h.history(), vec![HistoryButton::REDO.kind]);
        assert_eq!(h.drags(), vec![(e, 3)]);
        let a = h.world.get::<UiPlaySelectAnimation>(e).unwrap();
        assert_eq!((a.a, a.b, a.progress), (0.2, 0.2, 0.));
        assert!(h
            .world
            .resource_mut::<UiSignifierStream>()
            .take_entries()
            .is_empty());
    }
}
