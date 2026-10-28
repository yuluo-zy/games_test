//! UNMOUNTED host integration candidate, not a production registration.
//! Proposed destination: crates/tiny-glade/tests/toolbar_physical_chain.rs.
//! Requires core picker/ui_update and toolbar producer candidates to be landed,
//! and ModeJustChanged to alias the canonical full AppModeChanged event.
//! plus host [dev-dependencies] bevy_tasks.workspace = true (existing lock version).
//!
//! Boundary: real canonical MouseButtonInput + InputCursorMoved -> existing
//! input CPU installer -> picker -> ui_update -> toolbar producer -> button
//! consumer -> canonical mode manager. No action, interaction, click, or mode
//! request is manufactured. A real Taffy node with a supplied computed extent
//! bounds the test: layout/startup/MEI/OS event dispatch/GPU are not exercised.
//! Warmup registers widgets AFTER picking, exactly as frame N's set7 supplies
//! frame N+1's set6. Actual MEI pre must follow ui_update within set6; do not
//! reorder MEI ahead of picking to make a same-frame registration test pass.
//!
//! This CPU test is not FPS evidence. Real 960x540 visible small-window Release
//! acceptance needs complete logic/quality: sustained >=60 FPS minimum, >=120
//! FPS qualified, plus 1% low and p95/p99 frame times. Never lower fidelity or
//! omit business systems to obtain those figures (D:/game/PERFORMANCE-STANDARD.md).
use bevy_ecs::prelude::*;
use country_core::{
    components::{Ui2dExtent, UiInteractionState, UiMouseInteractable},
    game_config::GameConfig,
    game_profile::GameProfile,
    input::action_set::ActionSetStates,
    resources::{
        cursor_terrain_raycast::CursorTerrainRaycast,
        events::{ModeJustChanged, ShutdownGameCmd},
        gui::{EscMenuState, EscapeMenuJustOpened, TimeOfDayScreen},
        history::{EditType, History, MaybePendingSnapshotId},
        mouse_capture::{CursorCapturedPosition, OsCursorVisibilityCmd},
        mouse_terrain_raycast::CursorPositionSS,
        read_mouse_depth_value::MouseDepthValue,
        shader_refactoring_state::ShaderRefactoringState,
        toolbar::{Toolbar, ToolbarCategory},
        window::{WindowFocusEvent, WindowSize},
    },
    systems::{
        backend::input::{
            self, InputSystemSet, button_input::MouseButtonInput,
            calculate_input_delta::UiHistoryButtonPressed,
            translate_input_to_cursor_moved::InputCursorMoved,
        },
        brush_preview::OverwriteBrushPreview,
        camera_rig::MainCamera,
        mode_manager::{
            self, AppMode, AppModeChanged, ClearCanvasCmd, HistoryCommand, LoadingGameComplete,
            ModeManagerSound, ModeManagerUiContext, PreviousAppMode, RequestAppModeChange,
            RequestHistoryCommand, RequestToolbarCategoryChange, RestoreAppTo,
            UiHistoryPlaySelectAnimation,
        },
        ui::{
            hover_intersection::hover_intersection,
            resources::{
                ui_state::{UiState, WidgetId},
                widget_params::{PlayUiSound, UiNavState},
            },
            signifiers::UiSignifierStream,
            ui_update::ui_update,
        },
    },
};
use glam::Vec2;
use toolbar::{
    resouces_and_components::{
        HistoryButton, SwitchAppModeButton, TaffyNode, UiPlaySelectAnimation,
        taffy_layout::TaffyLayout,
    },
    toolbar_button::switch_app_mode_button,
    toolbar_update::ToolbarSizeMult,
    ui_interaction::{UiClickEvent, UiDragEvent, render_taffy_ui_to_widgets},
};

struct Harness {
    world: World,
    schedule: Schedule,
    entity: Entity,
    widget: WidgetId,
}

impl Harness {
    fn new() -> Self {
        bevy_tasks::ComputeTaskPool::get_or_init(|| {
            bevy_tasks::TaskPoolBuilder::new().num_threads(2).build()
        });
        let mut world = World::new();
        world.insert_resource(AppMode::Path);
        let mut toolbar = Toolbar::default();
        toolbar.switch_mode(&AppMode::Path);
        world.insert_resource(toolbar);
        input::register_input_events(&mut world);
        // These are owned by the real input installer, not fabricated by this
        // fixture. Assert ownership so a future registrar regression is visible.
        assert!(world.contains_resource::<MouseDepthValue>());
        assert!(world.contains_resource::<PreviousAppMode>());
        assert!(world.contains_resource::<Events<AppModeChanged>>());
        assert_eq!(world.resource::<PreviousAppMode>().0, AppMode::Path);
        assert_eq!(
            std::any::TypeId::of::<ModeJustChanged>(),
            std::any::TypeId::of::<AppModeChanged>(),
            "requires the canonical full event alias, never a tool-only bridge"
        );
        world.init_resource::<GameProfile>();
        world.init_resource::<GameConfig>();
        world.init_resource::<ShaderRefactoringState>();
        world.init_resource::<EscMenuState>();
        world.init_resource::<TimeOfDayScreen>();
        world.init_resource::<CursorCapturedPosition>();
        world.init_resource::<CursorTerrainRaycast>();
        world.insert_resource(CursorPositionSS(Vec2::ZERO));
        world.insert_resource(WindowSize::new(100, 100));
        world.insert_resource(MainCamera::new(1.0));
        world.insert_resource(UiState::at_startup());
        world.init_resource::<UiNavState>();
        world.init_resource::<UiSignifierStream>();
        world.insert_resource(ToolbarSizeMult { multiplier: 1.0 });
        world.insert_resource(History::<MaybePendingSnapshotId>::empty_glade());
        world.init_resource::<RestoreAppTo>();
        world.insert_resource(ModeManagerUiContext::from_ui_state(
            world.resource::<UiState>(),
        ));
        world.init_resource::<Events<WindowFocusEvent>>();
        world.init_resource::<Events<EscapeMenuJustOpened>>();
        world.init_resource::<Events<ShutdownGameCmd>>();
        world.init_resource::<Events<OverwriteBrushPreview>>();
        world.init_resource::<Events<OsCursorVisibilityCmd>>();
        world.init_resource::<Events<PlayUiSound>>();
        world.init_resource::<Events<UiClickEvent>>();
        world.init_resource::<Events<UiDragEvent>>();
        world.init_resource::<Events<ModeJustChanged>>();
        world.init_resource::<Events<RequestAppModeChange>>();
        world.init_resource::<Events<RequestToolbarCategoryChange>>();
        world.init_resource::<Events<RequestHistoryCommand>>();
        world.init_resource::<Events<LoadingGameComplete>>();
        world.init_resource::<Events<HistoryCommand>>();
        world.init_resource::<Events<UiHistoryPlaySelectAnimation>>();
        world.init_resource::<Events<ModeManagerSound>>();
        world.init_resource::<Events<ClearCanvasCmd>>();

        let mut layout = TaffyLayout::default();
        let node = layout.add_node(Default::default()).root();
        let widget = WidgetId::from_name(&format!("toolbar_button_{node:?}"));
        world.insert_resource(layout);
        let entity = world
            .spawn((
                TaffyNode {
                    node,
                    ..Default::default()
                },
                Ui2dExtent {
                    pos: Vec2::splat(50.0),
                    size: Vec2::splat(20.0),
                },
                UiMouseInteractable {
                    bytes: (-0.011_f32).to_le_bytes(),
                },
                UiInteractionState {
                    state: 1,
                    keep: 0,
                    selected: 0,
                },
                SwitchAppModeButton {
                    mode: AppMode::Water,
                },
            ))
            .id();
        let mut schedule = Schedule::default();
        // Reuse the existing input_pipeline fixture's real installer. Its
        // Consume slot runs after translate/resolve/record/derive and before
        // replay advancement. The relative consumer order matches host 6/7/8/10.
        input::add_input_cpu_consumers(&mut schedule);
        schedule.add_systems(
            (
                hover_intersection,
                ui_update,
                render_taffy_ui_to_widgets,
                switch_app_mode_button,
                mode_manager::synchronize_mode_manager_ui_context,
                mode_manager::mode_manager::<EditType, MaybePendingSnapshotId>,
                mode_manager::send_mode_just_changed_event,
            )
                .chain_ignore_deferred()
                .in_set(InputSystemSet::Consume),
        );
        Self {
            world,
            schedule,
            entity,
            widget,
        }
    }

    fn frame(&mut self, pressed: bool, position: Vec2) {
        let mut mouse = self.world.resource_mut::<MouseButtonInput>();
        mouse.clear(); // Same per-frame edge clearing as the existing device harness.
        if pressed {
            mouse.press(0);
        } else {
            mouse.release(0);
        }
        self.world.send_event(InputCursorMoved::Absolute(position));
        self.world
            .resource_mut::<bevy_time::Time>()
            .advance_by(std::time::Duration::from_millis(16));
        self.schedule.run(&mut self.world);
        self.world.flush();
        assert_eq!(self.world.resource::<CursorPositionSS>().0, position);
    }

    fn requests(&self) -> Vec<AppMode> {
        let events = self.world.resource::<Events<RequestAppModeChange>>();
        events.get_cursor().read(events).map(|e| e.0).collect()
    }
}

#[test]
fn canonical_mouse_press_hold_release_reaches_toolbar_and_mode_owner() {
    let mut h = Harness::new();
    h.frame(false, Vec2::splat(50.0));
    assert_eq!(
        h.world.resource::<UiState>().hover_byte(),
        0,
        "first set6 sees no registration; set7 supplies the next frame"
    );
    assert!(h.world.resource::<UiState>().widget(h.widget).is_some());
    assert!(h.requests().is_empty());

    h.frame(true, Vec2::splat(50.0));
    assert!(h.world.resource::<ActionSetStates>().digital[0].just_pressed());
    assert_eq!(h.world.resource::<UiState>().active_widget(), h.widget);
    assert_eq!(h.world.resource::<UiState>().click_byte(), 3);
    assert!(h.requests().is_empty());
    assert_eq!(*h.world.resource::<AppMode>(), AppMode::Path);
    h.frame(true, Vec2::splat(50.0));
    assert!(!h.world.resource::<ActionSetStates>().digital[0].just_pressed());
    assert_eq!(h.world.resource::<UiState>().click_byte(), 4);
    assert!(h.requests().is_empty());

    h.frame(false, Vec2::splat(50.0));
    assert!(h.world.resource::<ActionSetStates>().digital[0].just_released());
    assert_eq!(h.world.resource::<UiState>().click_byte(), 1);
    assert_eq!(h.requests(), [AppMode::Water]);
    assert_eq!(*h.world.resource::<AppMode>(), AppMode::Water);
    assert_eq!(
        h.world.resource::<Toolbar>().current,
        ToolbarCategory::Water
    );
    let changes = h.world.resource::<Events<AppModeChanged>>();
    assert_eq!(
        changes
            .get_cursor()
            .read(changes)
            .copied()
            .collect::<Vec<_>>(),
        [AppModeChanged {
            current: AppMode::Water,
            previous: AppMode::Path
        }]
    );
    let clicks = h.world.resource::<Events<UiClickEvent>>();
    assert_eq!(
        clicks
            .get_cursor()
            .read(clicks)
            .map(|e| e.entity)
            .collect::<Vec<_>>(),
        [h.entity]
    );
    h.frame(false, Vec2::splat(50.0));
    assert_eq!(
        h.requests(),
        [AppMode::Water],
        "release must produce only one request"
    );
    // set10 published the complete event last frame; the next set8 consumer
    // must update both selection and animation without a hand-written event.
    assert_eq!(
        h.world
            .get::<UiInteractionState>(h.entity)
            .unwrap()
            .selected,
        1
    );
    assert!(h.world.get::<UiPlaySelectAnimation>(h.entity).is_some());
}

#[test]
fn physical_release_outside_cancels_without_mode_request() {
    let mut h = Harness::new();
    h.frame(false, Vec2::splat(50.0));
    h.frame(true, Vec2::splat(50.0));
    h.frame(false, Vec2::splat(90.0));
    assert_eq!(h.world.resource::<UiState>().click_byte(), 0);
    assert!(h.requests().is_empty());
    assert_eq!(*h.world.resource::<AppMode>(), AppMode::Path);
    assert!(h.world.resource::<Events<UiClickEvent>>().is_empty());
}

#[test]
fn deleting_captured_button_cannot_dispatch_request_from_previous_frame_geometry() {
    let mut h = Harness::new();
    h.frame(false, Vec2::splat(50.0));
    h.frame(true, Vec2::splat(50.0));
    h.world.despawn(h.entity);
    // The registry intentionally still contains the previous frame's geometry;
    // the query/consumer must not manufacture a click for the removed entity.
    h.frame(false, Vec2::splat(50.0));
    assert!(h.requests().is_empty());
    assert!(h.world.resource::<UiState>().registry_snapshot().is_empty());
    h.frame(false, Vec2::splat(50.0));
    assert!(h.requests().is_empty());
    assert!(h.world.resource::<UiState>().active_params().is_none());
    assert_eq!(*h.world.resource::<AppMode>(), AppMode::Path);
}

#[test]
fn real_undo_and_redo_button_edges_reach_next_frame_translator_and_mode_manager() {
    for (kind, slot, command) in [(0, 16, HistoryCommand::UNDO), (1, 17, HistoryCommand::REDO)] {
        let mut h = Harness::new();
        h.world
            .entity_mut(h.entity)
            .remove::<SwitchAppModeButton>()
            .insert(HistoryButton { kind });
        h.frame(false, Vec2::splat(50.0));
        h.frame(true, Vec2::splat(50.0));
        assert!(h.world.get::<UiPlaySelectAnimation>(h.entity).is_some());
        assert!(
            !h.world.resource::<ActionSetStates>().digital[slot].pressed(),
            "the set7 history event cannot travel backward to set5 in the same frame"
        );
        let pulses = h.world.resource::<Events<UiHistoryButtonPressed>>();
        assert_eq!(
            pulses
                .get_cursor()
                .read(pulses)
                .map(|e| e.0)
                .collect::<Vec<_>>(),
            [kind]
        );
        h.frame(true, Vec2::splat(50.0));
        assert!(h.world.resource::<ActionSetStates>().digital[slot].just_pressed());
        assert_eq!(*h.world.resource::<AppMode>(), AppMode::Replay);
        let commands = h.world.resource::<Events<HistoryCommand>>();
        assert_eq!(
            commands
                .get_cursor()
                .read(commands)
                .copied()
                .collect::<Vec<_>>(),
            [command]
        );
        h.frame(false, Vec2::splat(50.0));
        assert_eq!(h.world.resource::<UiState>().click_byte(), 1);
        // History buttons remain interactive in Replay; their original
        // is_some gate includes Started, Continuing, and Finished frames.
        let pulses = h.world.resource::<Events<UiHistoryButtonPressed>>();
        assert_eq!(
            pulses
                .get_cursor()
                .read(pulses)
                .map(|e| e.0)
                .collect::<Vec<_>>(),
            [kind; 3]
        );
        h.frame(false, Vec2::splat(50.0));
        h.frame(false, Vec2::splat(50.0));
        assert!(h.world.resource::<ActionSetStates>().digital[slot].just_released());
        assert!(h.requests().is_empty());
        let commands = h.world.resource::<Events<HistoryCommand>>();
        assert_eq!(
            commands
                .get_cursor()
                .read(commands)
                .copied()
                .collect::<Vec<_>>(),
            [command]
        );
        // Empty fixture history means this proves command production only;
        // history_process/replay of actual edits belongs to separate acceptance.
    }
}
