//! TB-E (round 11): `render_taffy_ui_to_widgets` real Signifier payload flow.
//!
//! Contract `tb-toolbar-CONTRACT.md` §2.14 (#65-#75) / §4.3: the no-HistoryButton
//! branch of `render_taffy_ui_to_widgets` must write the 0x20-byte Signifier
//! payload into `UiSignifierStream` instead of computing and dropping it
//! (TB delivery §6.3). This test drives the public system through a real ECS
//! world and reads the stream back with `take_all()`.
use bevy_ecs::event::Events;
use bevy_ecs::prelude::*;
use country_core::components::{Ui2dExtent, UiInteractionState, UiMouseInteractable};
use country_core::game_config::GameConfig;
use country_core::input::action_set::ActionSetStates;
use country_core::resources::mouse_terrain_raycast::CursorPositionSS;
use country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed;
use country_core::systems::mode_manager::AppMode;
use country_core::systems::ui::resources::ui_state::{UiInteraction, UiState, WidgetId};
use country_core::systems::ui::resources::widget_params::UiNavState;
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
    taffy
        .new_leaf(Style::default())
        .expect("taffy leaf")
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
fn signifier_payload_is_written_into_the_stream_and_not_dropped() {
    let node = new_node();
    let name = button_name(node);
    // native_tag != 0 selects this widget as the active widget.
    let mut harness = Harness::new(AppMode::Path, active_interaction(&name, 1, 0), 1.0);
    let entity = harness.spawn_widget(
        node,
        UiInteractionState {
            state: 1,
            keep: 0,
            selected: 0,
        },
    );

    // The widget registration carries the expanded rect and the interaction
    // layer; expand = icon_size_mult(1.0) * 10 * size_mult(1.0) = 10.
    harness.run();
    let widget_id = WidgetId::from_name(&name);
    let params = harness
        .world
        .resource::<UiState>()
        .widget(widget_id)
        .expect("render_taffy registers the widget by its native name");
    assert_eq!(params.volume.tag, 3);
    assert_eq!(params.volume.layer, 1);
    assert_eq!(params.volume.size, Vec2::new(110.0, 50.0));
    assert!(
        params.flags.no_history_recording(),
        "fail-closed history flag"
    );
    assert!(params.flags.allow_negative_distance());
    assert!(params.occlude_by_world);
    assert_eq!(params.extra_78, [0, 0]);

    let records = harness
        .world
        .resource_mut::<UiSignifierStream>()
        .take_all();
    assert_eq!(records.len(), 1, "one active widget -> exactly one record");
    let record = records[0];
    // 0x20-byte payload: {1u64 tag, 1u64 kind, cursor Vec2, icon_size_mult*60}.
    assert_eq!(record.tag, 1);
    assert_eq!(record.kind, 1);
    assert_eq!(record.position.x, 12.5);
    assert_eq!(record.position.y, -3.0);
    assert_eq!(record.position.z, 60.0);
    assert_eq!(record.scale, 0.0);
    assert_eq!(record.name, 0);

    // The native `keep` byte is cleared unless the nav focus points at it.
    assert_eq!(
        harness.world.get::<UiInteractionState>(entity).unwrap().keep,
        0
    );
    // A non-clicked, non-dragged widget emits no transient events.
    assert!(harness.clicks().is_empty());
    assert!(harness.drags().is_empty());

    // A second frame appends another complete record (never aggregated away).
    harness.run();
    let records = harness
        .world
        .resource_mut::<UiSignifierStream>()
        .take_all();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].tag, 1);
    assert_eq!(records[0].position.z, 60.0);
}

#[test]
fn signifier_scale_uses_icon_size_mult_and_toolbar_size_mult() {
    let node = new_node();
    let name = button_name(node);
    let mut harness = Harness::new(AppMode::Path, active_interaction(&name, 1, 0), 2.5);
    harness.spawn_widget(node, UiInteractionState::default());
    harness.run();
    let records = harness
        .world
        .resource_mut::<UiSignifierStream>()
        .take_all();
    assert_eq!(records.len(), 1);
    // 0x20B payload f32 = game_config.ui.icon_size_mult * 60, independent of
    // ToolbarSizeMult; the widget rect expansion uses *10*size_mult.
    assert_eq!(records[0].position.z, 60.0);
    let params = harness
        .world
        .resource::<UiState>()
        .widget(WidgetId::from_name(&name))
        .expect("widget");
    assert_eq!(params.volume.size, Vec2::new(125.0, 65.0));
}

#[test]
fn clicked_and_started_drag_write_shared_tail_events() {
    let node = new_node();
    let name = button_name(node);
    // native_tag 2 + phase 1: clicked, drag state 1 (finished variant).
    let mut harness = Harness::new(AppMode::Path, active_interaction(&name, 2, 1), 1.0);
    let entity = harness.spawn_widget(node, UiInteractionState::default());
    harness.run();
    assert_eq!(harness.clicks(), vec![entity]);
    assert_eq!(harness.drags(), vec![(entity, 1)]);
    // The no-HistoryButton branch still emits the signifier payload.
    assert_eq!(
        harness
            .world
            .resource_mut::<UiSignifierStream>()
            .take_all()
            .len(),
        1
    );
}

#[test]
fn history_button_branch_writes_press_event_and_select_animation() {
    let node = new_node();
    let name = button_name(node);
    // phase 3 -> DragState::STARTED (is_some && is_started).
    let mut harness = Harness::new(AppMode::Path, active_interaction(&name, 2, 3), 1.0);
    let entity = harness
        .world
        .spawn((
            Ui2dExtent {
                pos: Vec2::new(5.0, 5.0),
                size: Vec2::new(20.0, 20.0),
            },
            UiMouseInteractable::default(),
            TaffyNode {
                node,
                ..TaffyNode::default()
            },
            UiInteractionState::default(),
            HistoryButton::REDO,
        ))
        .id();
    harness.run();

    assert_eq!(harness.history(), vec![HistoryButton::REDO.kind]);
    let animation = *harness
        .world
        .get::<UiPlaySelectAnimation>(entity)
        .expect("started drag queues the select animation");
    assert_eq!(animation.a, 0.2);
    assert_eq!(animation.b, 0.2);
    assert_eq!(animation.progress, 0.0);
    // HistoryButton entities do not take the Signifier branch.
    assert!(
        harness
            .world
            .resource_mut::<UiSignifierStream>()
            .take_all()
            .is_empty()
    );
}

#[test]
fn photomode_skips_the_signifier_payload() {
    let node = new_node();
    let name = button_name(node);
    let mut harness = Harness::new(AppMode::Photomode, active_interaction(&name, 1, 0), 1.0);
    harness.spawn_widget(node, UiInteractionState::default());
    harness.run();
    assert!(
        harness
            .world
            .resource_mut::<UiSignifierStream>()
            .take_all()
            .is_empty(),
        "the native branch gates the signifier on AppMode != Photomode"
    );
}
