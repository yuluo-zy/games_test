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

fn initialize_menu_resources(world: &mut World) {
    world.insert_resource(AppMode::Path);
    let mut toolbar = Toolbar::default();
    toolbar.switch_mode(&AppMode::Path);
    world.insert_resource(toolbar);
    input::register_input_events(world);
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
    toolbar::resouces_and_components::register_local_components(world);
    world.init_resource::<TaffyLayout>();
    world.init_resource::<country_core::resources::gui::PhotomodeScreen>();
    world.init_resource::<country_core::startup::init_photomode_state::PhotomodeState>();
    world.init_resource::<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>();
    world.insert_resource(country_core::resources::onboarding::WelcomeScreenState::Closed);
}

impl MenuHarness {
    fn new() -> Self {
        bevy_tasks::ComputeTaskPool::get_or_init(|| {
            bevy_tasks::TaskPoolBuilder::new().num_threads(2).build()
        });
        let mut world = World::new();
        initialize_menu_resources(&mut world);
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

/// The production Welcome-page MEI frame: real device/bridge, original paper texture,
/// fonts and templates, `welcome::ui` + `mei_post_update` behind the real input consumer.
fn install_welcome_assets(world: &mut World) -> tiny_glade::render_bridge::CoreRenderBridge {
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
    let shaders = Arc::new(AssetShaderLibrary::new());
    let bridge = tiny_glade::render_bridge::CoreRenderBridge::new(
        device,
        installed.clone(),
        Arc::clone(&shaders),
    )
    .expect("real MEI core bridge");
    world.insert_resource(country_core::asset_libraries::shader_library::SharedAssetShaderLibrary(shaders));
    bridge.install_asset_libraries(world).unwrap();
    let paper = installed.join("compiled-assets/textures/menu/paper-bg.texture");
    let file = AlignedFile::read(&paper).unwrap();
    let image = parse_compiled_image(file.bytes(), &paper);
    let texture = bridge
        .hosts()
        .textures
        .create_texture("menu/paper-bg", &image.desc, image.data)
        .unwrap();
    world
        .resource_mut::<AssetTextureLibrary>()
        .add(Some("menu/paper-bg".to_owned()), texture);
    let mut renderer = tg_text::TextRenderer::new();
    tiny_glade::mei_host::load_text_fonts(&mut renderer, &installed).unwrap();
    renderer.set_pixels_per_point(0.54).unwrap();
    world.insert_resource(renderer);
    let mut mei = toolbar::MeiContext::default();
    mei.load_configs(&installed.join("assets/ui"));
    mei.load_font_table(&installed.join("assets/fonts/table.ron"))
        .unwrap();
    world.insert_resource(mei);
    let mut vermeer = VermeerState::default();
    vermeer.deactivate();
    world.insert_resource(vermeer);
    world.insert_resource(GameTime {
        delta: 1.0 / 60.0,
        ..Default::default()
    });
    world.init_resource::<Events<SaveGameProfileCmd>>();
    bridge
}

fn production_welcome_harness() -> (MenuHarness, tiny_glade::render_bridge::CoreRenderBridge) {
    let mut h = MenuHarness::new();
    let bridge = install_welcome_assets(&mut h.world);
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

    (h, bridge)
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
    let (mut h, _bridge) = production_welcome_harness();
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

// ---- M01 CPU frame cost (run explicitly: `--release --ignored --nocapture --test-threads=1`) ----
mod frame_cost {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

    /// Counts allocator calls so the per-frame allocation rate of the MEI path is measurable.
    pub struct Counting;
    pub static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);
    pub static BYTES: AtomicU64 = AtomicU64::new(0);
    /// When set, every `SAMPLE_EVERY`-th allocation records the innermost project frame of
    /// its backtrace (debug builds only: needs symbols).
    pub static SAMPLING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    pub static SITES: std::sync::Mutex<Vec<(String, u64)>> = std::sync::Mutex::new(Vec::new());
    const SAMPLE_EVERY: u64 = 16;
    thread_local! { static IN_HOOK: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }

    fn note(size: usize) {
        if IN_HOOK.with(|h| h.get()) {
            return;
        }
        let n = ALLOCATIONS.fetch_add(1, Relaxed);
        BYTES.fetch_add(size as u64, Relaxed);
        if SAMPLING.load(Relaxed) && n % SAMPLE_EVERY == 0 {
            IN_HOOK.with(|h| h.set(true));
            let text = std::backtrace::Backtrace::force_capture().to_string();
            let site = text
                .lines()
                .map(str::trim)
                .find(|l| {
                    ["toolbar::", "country_core::", "tiny_glade::", "taffy", "tg_text", "bevy_ecs", "system_"]
                        .iter()
                        .any(|p| l.contains(p))
                        && !l.contains("frame_cost")
                        && !l.contains("toolbar_menu_chain::measure")
                })
                .unwrap_or("(no project frame)")
                .to_owned();
            let site = site.split_once(": ").map_or(site.as_str(), |(_, rest)| rest);
            let site = site.rsplit_once("::h").map_or(site, |(head, _)| head).to_owned();
            let mut sites = SITES.lock().unwrap();
            match sites.iter_mut().find(|(s, _)| *s == site) {
                Some((_, count)) => *count += 1,
                None => sites.push((site, 1)),
            }
            drop(sites);
            IN_HOOK.with(|h| h.set(false));
        }
    }
    // SAFETY: every method forwards to the system allocator unchanged.
    unsafe impl GlobalAlloc for Counting {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            note(layout.size());
            unsafe { System.alloc(layout) }
        }
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            unsafe { System.dealloc(ptr, layout) }
        }
        unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
            note(new_size);
            unsafe { System.realloc(ptr, layout, new_size) }
        }
    }
}
#[global_allocator]
static COUNTING: frame_cost::Counting = frame_cost::Counting;

fn measure(h: &mut MenuHarness, position: Vec2, frames: usize) -> (Vec<f64>, f64, f64) {
    use std::sync::atomic::Ordering::Relaxed;
    let mut micros = Vec::with_capacity(frames);
    let (a0, b0) = (frame_cost::ALLOCATIONS.load(Relaxed), frame_cost::BYTES.load(Relaxed));
    for _ in 0..frames {
        let start = std::time::Instant::now();
        h.frame(false, position);
        micros.push(start.elapsed().as_secs_f64() * 1e6);
    }
    let (a1, b1) = (frame_cost::ALLOCATIONS.load(Relaxed), frame_cost::BYTES.load(Relaxed));
    micros.sort_by(f64::total_cmp);
    (micros, (a1 - a0) as f64 / frames as f64, (b1 - b0) as f64 / frames as f64)
}

/// Wall-clock and allocation cost of one full production MEI frame (input consume, widget
/// picker, Welcome `ui`, Taffy layout, drawable update, text primitives): the toolbar alone,
/// then with the Welcome page up. Numbers are printed and, with `TG_PERF_OUT`, written as JSON.
#[test]
#[ignore = "measurement, not an assertion of a budget; use --release"]
fn toolbar_menu_frame_cpu_time_and_allocations() {
    use country_core::resources::onboarding::WelcomeScreenState;
    let (mut h, _bridge) = production_welcome_harness();
    let outside = Vec2::ZERO;
    for _ in 0..24 {
        h.frame(false, outside);
    }
    let position = h.position();
    let pct = |v: &[f64], q: f64| v[((v.len() - 1) as f64 * q) as usize];
    let mut rows = Vec::new();
    for (name, welcome) in [("toolbar only", false), ("welcome page + toolbar", true)] {
        if welcome {
            *h.world.resource_mut::<WelcomeScreenState>() = WelcomeScreenState::Pending;
            for _ in 0..4 {
                h.frame(false, position);
            }
            assert!(matches!(*h.world.resource::<WelcomeScreenState>(), WelcomeScreenState::Showing { .. }));
        }
        let _ = measure(&mut h, position, 60); // warm-up: layout caches, glyph atlas
        let (micros, allocations, bytes) = measure(&mut h, position, 600);
        let nodes = h.world.resource::<toolbar::MeiContext>().nodes.len();
        eprintln!(
            "PERF {name}: {} frames, p50 {:.0} us, p95 {:.0} us, p99 {:.0} us, max {:.0} us; {allocations:.0} allocs/frame, {bytes:.0} bytes/frame; MEI nodes {nodes}",
            micros.len(), pct(&micros, 0.5), pct(&micros, 0.95), pct(&micros, 0.99), micros[micros.len() - 1]
        );
        rows.push(serde_json::json!({
            "scenario": name, "frames": micros.len(), "mei_nodes": nodes,
            "p50_us": pct(&micros, 0.5), "p95_us": pct(&micros, 0.95), "p99_us": pct(&micros, 0.99),
            "max_us": micros[micros.len() - 1], "allocations_per_frame": allocations, "bytes_per_frame": bytes,
            "profile": if cfg!(debug_assertions) { "debug" } else { "release" },
        }));
    }
    if let Some(path) = std::env::var_os("TG_PERF_OUT") {
        std::fs::write(path, serde_json::to_string_pretty(&rows).unwrap()).unwrap();
    }
}
/// Where the per-frame allocations come from: samples every 16th allocation of the steady-state
/// frames and prints the innermost project frame, most frequent first. Debug build only.
#[test]
#[ignore = "profiling aid; debug profile (needs symbols)"]
fn toolbar_menu_frame_allocation_sites() {
    use std::sync::atomic::Ordering::Relaxed;
    use country_core::resources::onboarding::WelcomeScreenState;
    let (mut h, _bridge) = production_welcome_harness();
    for _ in 0..24 {
        h.frame(false, Vec2::ZERO);
    }
    let position = h.position();
    for (name, welcome) in [("toolbar only", false), ("welcome page + toolbar", true)] {
        if welcome {
            *h.world.resource_mut::<WelcomeScreenState>() = WelcomeScreenState::Pending;
            for _ in 0..4 {
                h.frame(false, position);
            }
        }
        for _ in 0..60 {
            h.frame(false, position);
        }
        frame_cost::SITES.lock().unwrap().clear();
        frame_cost::SAMPLING.store(true, Relaxed);
        let frames = 200;
        let a0 = frame_cost::ALLOCATIONS.load(Relaxed);
        for _ in 0..frames {
            h.frame(false, position);
        }
        let a1 = frame_cost::ALLOCATIONS.load(Relaxed);
        frame_cost::SAMPLING.store(false, Relaxed);
        let mut sites = frame_cost::SITES.lock().unwrap().clone();
        sites.sort_by_key(|(_, n)| std::cmp::Reverse(*n));
        let total: u64 = sites.iter().map(|(_, n)| *n).sum();
        eprintln!("ALLOCSITES {name}: {:.0} allocs/frame, {total} samples", (a1 - a0) as f64 / frames as f64);
        for (site, n) in sites.iter().take(14) {
            eprintln!("ALLOCSITES   {:5.1}%  {}", *n as f64 * 100.0 / total.max(1) as f64, site);
        }
    }
}
#[test]
fn production_app_welcome_and_reload_use_real_toolbar_installers() {
    use bevy_app::{App, Update};
    use bevy_state::{app::{AppExtStates, StatesPlugin}, prelude::NextState};
    use country_core::app_loading_state::AppLoadingState as Loading;
    use tiny_glade::run_game::GameSystemSet;

    const CHILD: &str = "TG_TOOLBAR_REAL_APP_CHILD";
    if std::env::var_os(CHILD).is_none() {
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "production_app_welcome_and_reload_use_real_toolbar_installers", "--nocapture"])
            .env(CHILD, "1").status().unwrap();
        assert!(status.success(), "隔离子进程中的真实 App 验收失败: {status}");
        return;
    }

    let account = tempfile::tempdir().unwrap();
    let installed = std::path::PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").unwrap());
    paths::initialize_paths_with_installation(0, Some(account.path()), &installed).unwrap();
    struct RestoreCwd(std::path::PathBuf);
    impl Drop for RestoreCwd {
        fn drop(&mut self) { std::env::set_current_dir(&self.0).unwrap(); }
    }
    let _cwd = RestoreCwd(std::env::current_dir().unwrap());
    std::env::set_current_dir(&installed).unwrap();
    let mut app = App::new();
    app.add_plugins(StatesPlugin).init_state::<Loading>();
    tiny_glade::run_game::register_gameplay_events(&mut app);
    initialize_menu_resources(app.world_mut());
    app.add_event::<RequestAppModeChange>();
    app.add_event::<RequestToolbarCategoryChange>();
    app.add_event::<RequestHistoryCommand>();
    app.add_event::<HistoryCommand>();
    app.add_event::<PlayUiSound>();
    app.add_event::<LoadingGameComplete>();
    app.add_event::<EscapeMenuJustOpened>();
    app.add_event::<ShutdownGameCmd>();
    app.add_event::<WindowFocusEvent>();
    app.init_resource::<country_core::meta_commander::MetaCommander>();
    app.init_resource::<country_core::resources::rt::RtBvhBuilderGlueResource>();
    app.init_resource::<country_core::resources::rt::RtWorld>();
    app.insert_resource(country_core::asset_libraries::mesh_library::AssetMeshLibrary::new());
    app.init_resource::<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>();
    app.insert_resource(country_core::resources::terrain_height::TerrainHeightsData::<
        country_core::resources::terrain_height::TerrainHeightSsbo>::new());
    app.add_event::<country_core::resources::events::DisplayRollercoasterPrompt>();
    app.insert_resource(country_core::resources::game_clock::GameClock {
        ran: true, ..Default::default()
    });
    app.init_resource::<country_core::resources::image_formation::ImageFormationConfigs>();
    app.insert_resource(country_core::resources::platform::RuntimePlatform::from_detection(false, false));
    app.init_resource::<country_core::systems::collision::world::RaycastWorld>();
    app.init_resource::<country_core::resources::gui::NextEscMenuState>();
    app.insert_resource(country_core::resources::walls::EraserSizeMultiplier(1.0));
    app.insert_resource(country_core::resources::brush_sizes::BrushSizes {
        path: country_core::resources::brush_sizes::PATH_DEFAULT,
        water: country_core::resources::brush_sizes::WATER_DEFAULT,
        garden: country_core::resources::brush_sizes::GARDEN_DEFAULT,
    });
    app.add_event::<country_core::resources::window::OsWindowCmd>();
    app.init_resource::<country_core::resources::walls::public_walls::PublicWalls>();
    app.insert_resource(country_core::resources::session::ActiveSession::new(
        country_core::resources::session::SessionId::new(),
        country_core::resources::session::SessionMeta::default()));
    app.insert_resource(country_core::systems::timelapse_replay::TimelapseState::Inactive);
    app.init_resource::<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>();
    app.add_event::<country_core::systems::timelapse_replay::StartTimelapseReplayCmd>();
    app.add_event::<country_core::resources::walls::public_walls::OnWallChanged>();
    app.add_event::<country_core::resources::events::InitAdjWallHeightMode>();
    app.add_event::<country_core::resources::events::FinishAdjWallHeightMode>();
    app.add_event::<country_core::systems::timelapse_replay::TimelapseReplayFinished>();
    app.add_event::<country_core::resources::photo_mode::PhotoModeCmd>();
    app.add_event::<country_core::systems::ui::resources::widget_params::PlayPhotomodeSound>();
    app.add_event::<country_core::systems::backend::photomode::PhotoModeAudioParameter>();
    let camera = app.world().resource::<MainCamera>();
    let matrices = country_core::render::FrameCameraMatrices {
        projection: camera.read_matrix(0), inverse_projection: camera.read_matrix(0x40),
        view: camera.read_matrix(0x80), inverse_view: camera.read_matrix(0xc0),
    };
    app.insert_resource(country_core::render::RenderTemporalState::new(matrices));
    app.add_event::<country_core::systems::save::save_game::SaveGameCmd>();
    #[cfg(any(windows, feature = "native-audio"))]
    {
        // Match the production FMOD owner and complete its real metadata and
        // sample loader before Startup constructs the signifier audio state.
        let mut audio = country_core::resources::native_audio::create_audio_resources(&installed, false);
        assert!(audio.core.studio.is_some(), "shipped FMOD must initialize");
        assert!(!audio.core.banks.is_empty(), "shipped audio banks must be discovered");
        let samples = audio.samples.take().expect("native sample completion");
        let loader = audio.load_remaining.take().expect("native bank loader");
        std::thread::spawn(loader).join().expect("FMOD bank loading");
        assert_eq!(samples.recv(), Some(()));
        app.insert_resource(audio.core);
    }
    #[cfg(not(any(windows, feature = "native-audio")))]
    app.insert_resource(country_core::resources::audio_resources::AudioCore::<
        country_core::resources::audio_resources::DisabledStudio> {
        studio: None, banks: Default::default(),
    });
    let _bridge = install_welcome_assets(app.world_mut());
    use bevy_ecs::system::RunSystemOnce;
    app.world_mut().run_system_once(
        country_core::startup::init_populate_minimal_startup_ssbo_library::minimal_startup_ssbo_system
    ).unwrap();
    let device = _bridge.hosts().buffers.graphics_device().clone();
    tiny_glade::loading_graph::attach_loading_mei(app.world_mut(), device, _bridge.hosts()).unwrap();
    for stage in 0..44 {
        app.configure_sets(Update, GameSystemSet(stage).before(GameSystemSet(stage + 1)));
    }
    tiny_glade::run_game::install_gameplay_device_input(&mut app);
    app.add_systems(bevy_app::Startup, country_core::resources::glade_border::setup_glade_border);
    app.add_systems(Update, country_core::systems::time_update::time_update.in_set(GameSystemSet(2)));
    app.add_systems(Update, country_core::systems::backend::input::reset_input_state::reset_input_state.in_set(GameSystemSet(3)));
    tiny_glade::mei_host::install_mei_frame_systems(&mut app);
    app.add_plugins(tiny_glade::toolbar_plugin::ToolbarPlugin);
    tiny_glade::toolbar_plugin::install_photomode_host_systems(&mut app);
    tiny_glade::toolbar_plugin::install_mode_consumers(&mut app);
    app.add_systems(Update, toolbar::menu::welcome::ui.in_set(GameSystemSet(36)).run_if(
        bevy_state::condition::in_state(Loading::Game)));
    app.add_systems(Update, (|mut text: ResMut<tg_text::TextRenderer>| text.begin_frame())
        .in_set(GameSystemSet(1)));
    app.add_systems(bevy_state::state::OnExit(Loading::Game),
        country_core::startup::clearing::despawn_glade_entities);
    app.world_mut().resource_mut::<NextState<Loading>>().set(Loading::Game);
    fn frame(app: &mut App, pressed: bool, position: Vec2) {
        let mut mouse = app.world_mut().resource_mut::<MouseButtonInput>();
        mouse.clear();
        if pressed { mouse.press(0); } else { mouse.release(0); }
        app.world_mut().send_event(InputCursorMoved::Absolute(position));
        app.world_mut().resource_mut::<bevy_time::Time>()
            .advance_by(std::time::Duration::from_millis(16));
        app.update();
    }
    for _ in 0..24 { frame(&mut app, false, Vec2::ZERO); }
    let entity = app.world_mut().query::<(Entity, &SwitchToolbarCategoryButton)>()
        .iter(app.world()).find(|(_, b)| b.category == ToolbarCategory::Water).unwrap().0;
    let initial_count = app.world_mut().query_filtered::<Entity, With<TaffyNode>>().iter(app.world()).count();
    let position = app.world().get::<Ui2dExtent>(entity).unwrap().pos;
    *app.world_mut().resource_mut::<country_core::resources::onboarding::WelcomeScreenState>() =
        country_core::resources::onboarding::WelcomeScreenState::Pending;
    for _ in 0..4 { frame(&mut app, false, position); }
    let position = app.world().get::<Ui2dExtent>(entity).unwrap().pos;
    assert!(app.world().get::<Ui2dExtent>(entity).unwrap().size.x > 0.0);
    assert!(matches!(*app.world().resource::<country_core::resources::onboarding::WelcomeScreenState>(),
        country_core::resources::onboarding::WelcomeScreenState::Showing { .. }));
    assert_eq!(app.world().get::<UiInteractionState>(entity).unwrap().state, 0);
    frame(&mut app, true, position);
    let extent = app.world().get::<Ui2dExtent>(entity).unwrap();
    let cursor = app.world().resource::<CursorPositionSS>().0;
    assert!((cursor.x - extent.pos.x).abs() <= extent.size.x * 0.5);
    assert!((cursor.y - extent.pos.y).abs() <= extent.size.y * 0.5);
    let node = app.world().get::<TaffyNode>(entity).unwrap().node;
    assert_ne!(app.world().resource::<UiState>().interaction().widget,
        WidgetId::from_name(&format!("toolbar_button_{node:?}")));
    frame(&mut app, false, position);
    assert_eq!(*app.world().resource::<AppMode>(), AppMode::Path);
    let code = country_core::game_profile::desktop_keycode_index(
        app.world().resource::<GameProfile>().key_bindings.bindings[29].unwrap()).unwrap();
    app.world_mut().resource_mut::<KeyButtonInput>().press(code);
    frame(&mut app, false, Vec2::ZERO);
    app.world_mut().resource_mut::<KeyButtonInput>().release_all();
    app.world_mut().resource_mut::<KeyButtonInput>().clear();
    frame(&mut app, false, Vec2::ZERO);
    if !app.world().resource::<country_core::resources::onboarding::WelcomeScreenState>().is_closed() {
        assert!(matches!(*app.world().resource::<country_core::resources::onboarding::WelcomeScreenState>(),
            country_core::resources::onboarding::WelcomeScreenState::Showing { elapsed } if elapsed > 0.));
        app.world_mut().resource_mut::<KeyButtonInput>().press(code);
        frame(&mut app, false, Vec2::ZERO);
        app.world_mut().resource_mut::<KeyButtonInput>().release_all();
        app.world_mut().resource_mut::<KeyButtonInput>().clear();
        frame(&mut app, false, Vec2::ZERO);
    }
    assert!(app.world().resource::<country_core::resources::onboarding::WelcomeScreenState>().is_closed());
    for _ in 0..24 { frame(&mut app, false, position); }
    let position = app.world().get::<Ui2dExtent>(entity).unwrap().pos;
    frame(&mut app, true, position);
    frame(&mut app, false, position);
    assert_eq!(*app.world().resource::<AppMode>(), AppMode::Water);
    let mut category_events = app.world().resource::<Events<RequestToolbarCategoryChange>>().get_cursor();
    category_events.read(app.world().resource::<Events<RequestToolbarCategoryChange>>()).for_each(drop);
    let mut mode_events = app.world().resource::<Events<RequestAppModeChange>>().get_cursor();
    mode_events.read(app.world().resource::<Events<RequestAppModeChange>>()).for_each(drop);
    frame(&mut app, true, position);
    app.world_mut().resource_mut::<NextState<Loading>>().set(Loading::Reload);
    frame(&mut app, true, position);
    assert!(app.world().get_entity(entity).is_err());
    app.world_mut().resource_mut::<NextState<Loading>>().set(Loading::Game);
    frame(&mut app, true, position);
    assert_eq!(app.world_mut().query_filtered::<Entity, With<TaffyNode>>().iter(app.world()).count(), initial_count);
    assert_eq!(app.world_mut().query::<&SwitchToolbarCategoryButton>().iter(app.world())
        .filter(|b| b.category == ToolbarCategory::Water).count(), 1);
    assert_eq!(app.world().resource::<UiState>().interaction().native_tag, 0);
    assert_eq!(*app.world().resource::<AppMode>(), AppMode::Water);
    assert_eq!(category_events.read(app.world().resource::<Events<RequestToolbarCategoryChange>>()).count(), 0);
    assert_eq!(mode_events.read(app.world().resource::<Events<RequestAppModeChange>>()).count(), 0);
    frame(&mut app, false, position);
    assert_eq!(*app.world().resource::<AppMode>(), AppMode::Water);
    assert_eq!(category_events.read(app.world().resource::<Events<RequestToolbarCategoryChange>>()).count(), 0);
    let path = app.world_mut().query::<(Entity, &SwitchToolbarCategoryButton)>()
        .iter(app.world()).find(|(_, b)| b.category == ToolbarCategory::Path).unwrap().0;
    for _ in 0..24 { frame(&mut app, false, position); }
    let path_position = app.world().get::<Ui2dExtent>(path).unwrap().pos;
    for _ in 0..4 { frame(&mut app, false, path_position); }
    frame(&mut app, true, path_position);
    frame(&mut app, false, path_position);
    assert_eq!(*app.world().resource::<AppMode>(), AppMode::Path);
}
