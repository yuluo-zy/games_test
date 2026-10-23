//! Independent CPU chain: production toolbar_setup and Taffy geometry ->
//! canonical device inputs -> real picker/UI owner -> category consumer -> mode owner.
//! No target component, extent, hit, click, action or mode event is manufactured.
//! The host provides closed-menu state and camera/window resources. OS dispatch,
//! MEI menu production, GPU drawing, save/history replay and performance remain
//! separate acceptance boundaries; passing these tests cannot close the module.
#[path = "toolbar_module_chain/reload.rs"]
mod reload;
#[path = "toolbar_module_chain/history.rs"]
mod history;

use bevy_ecs::prelude::*;
use country_core::{
    components::{Ui2dExtent, UiInteractionState},
    game_config::GameConfig,
    game_profile::GameProfile,
    input::action_set::ActionSetStates,
    resources::{
        cursor_terrain_raycast::CursorTerrainRaycast,
        events::{EscapeMenuJustOpened, ModeJustChanged, ShutdownGameCmd},
        gui::{DebugGui, EscMenuState, TimeOfDayScreen},
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
            self, InputSystemSet,
            button_input::{KeyButtonInput, MouseButtonInput},
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
        SwitchToolbarCategoryButton, TaffyNode, UiPlaySelectAnimation, taffy_layout::TaffyLayout,
    },
    toolbar_button::{switch_app_mode_button, switch_toolbar_category_button},
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
        assert_eq!(
            std::any::TypeId::of::<UiInteractionState>(),
            std::any::TypeId::of::<toolbar::resouces_and_components::UiInteractionState>()
        );
        assert_eq!(
            std::any::TypeId::of::<country_core::components::UiMouseInteractable>(),
            std::any::TypeId::of::<toolbar::resouces_and_components::UiMouseInteractable>()
        );
        world.init_resource::<DebugGui>();
        world.init_resource::<GameProfile>();
        world.init_resource::<GameConfig>();
        world.init_resource::<ShaderRefactoringState>();
        world.init_resource::<EscMenuState>();
        world.init_resource::<TimeOfDayScreen>();
        world.init_resource::<CursorCapturedPosition>();
        world.init_resource::<CursorTerrainRaycast>();
        world.insert_resource(CursorPositionSS(Vec2::ZERO));
        world.insert_resource(WindowSize::new(960, 540));
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
        world.init_resource::<toolbar::MeiContext>();
        world.init_resource::<Events<bevy_input::mouse::MouseWheel>>();
        world.init_resource::<Events<country_core::systems::audio::PlayMenuUiSound>>();

        // Register required components exactly as the application does.
        toolbar::resouces_and_components::register_local_components(&mut world);
        world.init_resource::<TaffyLayout>();
        world.init_resource::<country_core::resources::gui::PhotomodeScreen>();
        world.init_resource::<country_core::startup::init_photomode_state::PhotomodeState>();
        world.init_resource::<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>();
        world.insert_resource(country_core::resources::onboarding::WelcomeScreenState::Closed);
        let mut startup = Schedule::default();
        startup.add_systems(toolbar::toolbar_startup::toolbar_setup);
        startup.run(&mut world);
        world.flush();
        let entity = world
            .query::<(Entity, &SwitchToolbarCategoryButton)>()
            .iter(&world)
            .find(|(_, button)| button.category == ToolbarCategory::Water)
            .expect("real full-edition Water category")
            .0;
        let node = world.get::<TaffyNode>(entity).unwrap().node;
        let widget = WidgetId::from_name(&format!("toolbar_button_{node:?}"));
        let mut schedule = Schedule::default();
        // Reuse the existing input_pipeline fixture's real installer. Its
        // Consume slot runs after translate/resolve/record/derive and before
        // replay advancement. The relative consumer order matches host 6/7/8/10.
        input::add_input_cpu_consumers(&mut schedule);
        schedule.add_systems(
            (
                hover_intersection,
                ui_update,
                toolbar::mei::systems::mei_pre_update,
                toolbar::ui_interaction::hide_ui,
                render_taffy_ui_to_widgets,
                toolbar::ui_interaction::ui_focus_parent_if_self_in_focus,
                toolbar::ui_interaction::ui_show_if_parent_in_focus,
                (
                    toolbar::ui_interaction::ui_animation_state_update,
                    toolbar::ui_presentation::ui_focus_animation,
                    toolbar::ui_presentation::ui_appear_animation,
                )
                    .chain_ignore_deferred(),
                toolbar::ui_interaction::ui_nav_update,
                toolbar::toolbar_systems::toolbar_steal_mouse_input,
                switch_toolbar_category_button,
                switch_app_mode_button,
                mode_manager::synchronize_mode_manager_ui_context,
                mode_manager::mode_manager::<EditType, MaybePendingSnapshotId>,
                mode_manager::send_mode_just_changed_event,
                toolbar::toolbar_systems::toolbar_alignment,
                toolbar::taffy_update::taffy_finalize_element_size,
                toolbar::taffy_update::taffy_perform_layout,
                toolbar::taffy_update::taffy_update_drawables,
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

    fn position(&self) -> Vec2 {
        let extent = self.world.get::<Ui2dExtent>(self.entity).unwrap();
        assert!(extent.size.x > 0.0 && extent.size.y > 0.0);
        extent.pos
    }

    fn key_frame(&mut self, binding: Option<usize>) {
        let code = binding.map(|slot| {
            let key = self.world.resource::<GameProfile>().key_bindings.bindings[slot]
                .expect("default navigation binding");
            country_core::game_profile::desktop_keycode_index(key).unwrap()
        });
        let mut keyboard = self.world.resource_mut::<KeyButtonInput>();
        keyboard.release_all();
        keyboard.clear();
        if let Some(code) = code {
            keyboard.press(code);
        }
        self.frame(false, Vec2::ZERO);
    }

    fn requests(&self) -> Vec<AppMode> {
        let events = self.world.resource::<Events<RequestAppModeChange>>();
        events.get_cursor().read(events).map(|e| e.0).collect()
    }
}

#[test]
fn startup_layout_and_canonical_device_edges_switch_real_water_category() {
    let mut h = Harness::new();
    let outside = Vec2::ZERO;
    // Settle actual visibility/animation/layout; never provide a hand-built extent.
    for _ in 0..24 {
        h.frame(false, outside);
    }
    let position = h.position();
    assert!(position.x >= 0. && position.x < 960. && position.y >= 0. && position.y < 540.);
    h.frame(false, position);
    assert_eq!(h.world.resource::<UiState>().active_widget(), h.widget);
    h.frame(true, h.position());
    assert_eq!(h.world.resource::<UiState>().click_byte(), 3);
    assert_eq!(*h.world.resource::<AppMode>(), AppMode::Path);
    h.frame(true, h.position());
    h.frame(false, h.position());
    assert_eq!(*h.world.resource::<AppMode>(), AppMode::Water);
    assert_eq!(
        h.world.resource::<Toolbar>().current,
        ToolbarCategory::Water
    );
    let clicks = h.world.resource::<Events<UiClickEvent>>();
    assert_eq!(
        clicks
            .get_cursor()
            .read(clicks)
            .map(|event| event.entity)
            .collect::<Vec<_>>(),
        [h.entity]
    );
    h.frame(false, h.position());
    assert_eq!(
        h.world
            .get::<UiInteractionState>(h.entity)
            .unwrap()
            .selected,
        1
    );
    assert!(h.world.get::<UiPlaySelectAnimation>(h.entity).is_some());
    assert!(
        h.requests().is_empty(),
        "category switching uses its actual category request, not a fabricated mode click"
    );
}

#[test]
fn startup_button_release_outside_cancels_real_category_change() {
    let mut h = Harness::new();
    let outside = Vec2::ZERO;
    for _ in 0..24 {
        h.frame(false, outside);
    }
    h.frame(false, h.position());
    h.frame(true, h.position());
    assert_eq!(h.world.resource::<UiState>().active_widget(), h.widget);
    h.frame(false, outside);
    assert_eq!(h.world.resource::<UiState>().click_byte(), 0);
    assert_eq!(*h.world.resource::<AppMode>(), AppMode::Path);
    assert!(h.world.resource::<Events<UiClickEvent>>().is_empty());
}

#[test]
fn configured_keyboard_navigates_real_toolbar_ring_and_confirms_water() {
    let mut h = Harness::new();
    for _ in 0..24 {
        h.key_frame(None);
    }
    // The first key selects the first eligible real widget; subsequent right
    // edges follow the actual startup-produced horizontal links.
    h.key_frame(Some(31));
    for _ in 0..11 {
        let focus = h.world.resource::<UiNavState>().focus;
        assert!(focus.is_toolbar());
        if focus.payload == h.entity.to_bits() {
            break;
        }
        let from = Entity::from_bits(focus.payload);
        let next = h.world.get::<toolbar::UiNav>(from).unwrap().links[1];
        assert_eq!(
            next.tag, 3,
            "startup ring must provide an explicit right link"
        );
        h.key_frame(None);
        h.key_frame(Some(31));
        assert_eq!(h.world.resource::<UiNavState>().focus.payload, next.value);
    }
    assert_eq!(
        h.world.resource::<UiNavState>().focus.payload,
        h.entity.to_bits()
    );
    assert_eq!(*h.world.resource::<AppMode>(), AppMode::Path);
    h.key_frame(None);
    h.key_frame(Some(29));
    assert_eq!(*h.world.resource::<AppMode>(), AppMode::Water);
    assert_eq!(
        h.world.resource::<Toolbar>().current,
        ToolbarCategory::Water
    );
    h.key_frame(None);
    assert_eq!(
        h.world
            .get::<UiInteractionState>(h.entity)
            .unwrap()
            .selected,
        1
    );
}
