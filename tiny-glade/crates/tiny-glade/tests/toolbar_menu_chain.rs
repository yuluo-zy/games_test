//! Real welcome assets and MEI consumers arbitrate the physical toolbar input chain.
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

struct MenuHarness {
    world: World,
    schedule: Schedule,
    entity: Entity,
    widget: WidgetId,
}

impl MenuHarness {
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
fn real_welcome_layout_blocks_toolbar_until_its_own_confirm_handler_dismisses_it() {
    use country_core::{
        asset_libraries::{
            shader_library::AssetShaderLibrary, texture_library::AssetTextureLibrary,
        },
        game_profile::SaveGameProfileCmd,
        render::load_texture::{AlignedFile, boundary::TextureDevice, parse_compiled_image},
        resources::{onboarding::WelcomeScreenState, time::GameTime},
        systems::render_loop::vermeer::VermeerState,
    };
    use std::{path::PathBuf, sync::Arc};
    let installed = std::env::var_os("TG_INSTALL_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../ljxsj_92385/Tiny Glade")
        })
        .canonicalize()
        .expect("original packaged installation");
    // A real host device and original compiled image are required. This test
    // intentionally fails instead of substituting a mock device or texture.
    let device =
        rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig::headless())
            .expect("real MEI host Vulkan device");
    let bridge = tiny_glade::render_bridge::CoreRenderBridge::new(
        device,
        installed.clone(),
        Arc::new(AssetShaderLibrary::new()),
    )
    .expect("real MEI core bridge");
    let mut h = MenuHarness::new();
    bridge.install_asset_libraries(&mut h.world).unwrap();
    let paper = installed.join("compiled-assets/textures/menu/paper-bg.texture");
    let file = AlignedFile::read(&paper).unwrap();
    let image = parse_compiled_image(file.bytes(), &paper);
    let texture = bridge
        .hosts()
        .textures
        .create_texture("menu/paper-bg", &image.desc, image.data)
        .unwrap();
    h.world
        .resource_mut::<AssetTextureLibrary>()
        .add(Some("menu/paper-bg".to_owned()), texture);
    let mut renderer = tg_text::TextRenderer::new();
    tiny_glade::mei_host::load_text_fonts(&mut renderer, &installed).unwrap();
    renderer.set_pixels_per_point(0.54).unwrap();
    h.world.insert_resource(renderer);
    let mut mei = toolbar::MeiContext::default();
    mei.load_configs(&installed.join("assets/ui"));
    mei.load_font_table(&installed.join("assets/fonts/table.ron"))
        .unwrap();
    h.world.insert_resource(mei);
    let mut vermeer = VermeerState::default();
    vermeer.deactivate();
    h.world.insert_resource(vermeer);
    h.world.insert_resource(GameTime {
        delta: 1.0 / 60.0,
        ..Default::default()
    });
    h.world.init_resource::<Events<SaveGameProfileCmd>>();
    h.schedule.add_systems(
        (
            |mut text: ResMut<tg_text::TextRenderer>| text.begin_frame(),
            tiny_glade::mei_host::update_text_renderer_scale,
        )
            .chain()
            .before(InputSystemSet::Consume),
    );
    h.schedule.add_systems(
        (toolbar::menu::welcome::ui, toolbar::mei_post_update)
            .chain()
            .after(InputSystemSet::Consume),
    );

    let outside = Vec2::ZERO;
    for _ in 0..24 {
        h.frame(false, outside);
    }
    let toolbar_position = h.position();
    // Enter the original initial-state branch; it must produce and lay out its
    // own nodes before the next frame's picker can see its registrations.
    *h.world.resource_mut::<WelcomeScreenState>() = WelcomeScreenState::Pending;
    for _ in 0..4 {
        h.frame(false, toolbar_position);
    }
    assert!(matches!(
        *h.world.resource::<WelcomeScreenState>(),
        WelcomeScreenState::Showing { .. }
    ));
    assert!(!h.world.resource::<toolbar::MeiContext>().nodes.is_empty());
    assert!(!h.world.resource::<UiState>().registry_snapshot().is_empty());
    assert_eq!(
        h.world.get::<UiInteractionState>(h.entity).unwrap().state,
        0
    );
    h.frame(true, toolbar_position);
    h.frame(false, toolbar_position);
    assert_eq!(*h.world.resource::<AppMode>(), AppMode::Path);
    assert!(h.world.resource::<Events<UiClickEvent>>().is_empty());

    // Physical Enter reaches the real menu_hotkey/action handler. No node
    // state, fabricated UiFocus or profile dismissal is injected by this test.
    h.key_frame(Some(29));
    h.key_frame(None);
    if !h.world.resource::<WelcomeScreenState>().is_closed() {
        h.key_frame(Some(29));
        h.key_frame(None);
    }
    assert!(h.world.resource::<WelcomeScreenState>().is_closed());
    assert!(h.world.resource::<GameProfile>().welcome_was_dismissed());
    assert_eq!(h.world.resource::<Events<SaveGameProfileCmd>>().len(), 1);
    for _ in 0..4 {
        h.frame(false, toolbar_position);
    }
    assert_eq!(
        h.world.get::<UiInteractionState>(h.entity).unwrap().state,
        1
    );
    h.frame(true, toolbar_position);
    h.frame(false, toolbar_position);
    assert_eq!(*h.world.resource::<AppMode>(), AppMode::Water);
}
