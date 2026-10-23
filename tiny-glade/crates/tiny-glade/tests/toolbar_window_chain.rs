//! Real OS input -> production dual-App GameLoop -> authoritative toolbar state.
//! No clicks, modes, hits, menu state, geometry or save commands are fabricated.
//! Requires X11/XTest; all configuration/save writes use a fresh local account.
use anyhow::{Context, Result, ensure};
use bevy_ecs::event::EventCursor;
use bevy_ecs::prelude::*;
use bevy_state::state::{OnExit, StateTransitionEvent};
use clap::Parser;
use country_core::{
    app_loading_state::AppLoadingState,
    components::{Ui2dExtent, UiInteractionState},
    game_profile::GameProfile,
    resources::{
        gui::EscMenuState,
        history::History,
        onboarding::WelcomeScreenState,
        session::{ActiveSession, SavedSessionList},
        toolbar::{Toolbar, ToolbarCategory},
        window::WindowSize,
    },
    systems::{
        mode_manager::AppMode,
        save::{
            save_backup_list::SaveBackupList,
            save_game::{AutoSaveState, SavingGameComplete},
            save_thread::SaveThreadHandle,
        },
        ui::resources::{ui_state::UiState, widget_params::UiNavState},
    },
};
use std::{
    cell::RefCell,
    path::PathBuf,
    process::{Child, Command},
    rc::Rc,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tiny_glade::{
    app_config::{CliArgs, get_app_config},
    game_loop::{GameLoop, GameLoopOptions},
    production_renderer::ProductionGraphKind,
    run_game::{ApplicationShaderLibrary, run_game_thread},
};
use toolbar::resouces_and_components::SwitchToolbarCategoryButton;
use toolbar::{
    MeiContext,
    mei::{
        core::GlobalId,
        widgets::{MenuButtonContent, PillButtonContent},
    },
};
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

struct Controller {
    title: String,
    script: PathBuf,
    child: Option<Child>,
}
impl Controller {
    fn spawn(&mut self, action: &str, point: Option<[i32; 2]>) -> Result<()> {
        self.spawn_input(action, point, "Return")
    }
    fn spawn_input(&mut self, action: &str, point: Option<[i32; 2]>, key: &str) -> Result<()> {
        ensure!(
            self.child.is_none(),
            "overlapping physical input controller"
        );
        let listed = Command::new("python3")
            .arg(&self.script)
            .arg("list")
            .output()?;
        ensure!(listed.status.success(), "X11 window enumeration failed");
        let windows = String::from_utf8(listed.stdout)?
            .lines()
            .map(serde_json::from_str::<serde_json::Value>)
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let matching: Vec<_> = windows
            .iter()
            .filter(|item| item["title"].as_str() == Some(&self.title))
            .collect();
        ensure!(
            matching.len() == 1,
            "unique test window missing or ambiguous: {:?}",
            matching
        );
        let window = matching[0]["window"].as_str().context("X11 window id")?;
        let mut command = Command::new("python3");
        command
            .arg(&self.script)
            .arg(action)
            .arg("--window")
            .arg(window)
            .arg("--hold")
            .arg("1.0");
        if let Some([x, y]) = point {
            command
                .arg("--x")
                .arg(x.to_string())
                .arg("--y")
                .arg(y.to_string());
        }
        if action == "key" {
            command.arg("--key").arg(key);
        }
        self.child = Some(command.spawn().context("spawn real XTest input")?);
        Ok(())
    }
    fn idle(&mut self) -> Result<bool> {
        if let Some(child) = self.child.as_mut() {
            if let Some(status) = child.try_wait()? {
                ensure!(
                    status.success(),
                    "physical input controller failed: {status}"
                );
                self.child = None;
            }
        }
        Ok(self.child.is_none())
    }
}

// Read the production layout; never resize nodes or set their input bits.
fn mei_point(world: &World, position: glam::Vec2) -> Option<[i32; 2]> {
    let size = world.resource::<WindowSize>();
    let point = position * (size.height as f32 / 1000.);
    (point.is_finite()
        && point.x >= 0.
        && point.y >= 0.
        && point.x < size.width as f32
        && point.y < size.height as f32)
        .then_some([point.x.round() as i32, point.y.round() as i32])
}

fn menu_label_point(world: &World, text: &str, root: GlobalId) -> Option<[i32; 2]> {
    let mei = world.resource::<MeiContext>();
    mei.nodes.iter().find_map(|(id, node)| {
        let content: &dyn std::any::Any = node.content.as_ref();
        let button = content.downcast_ref::<MenuButtonContent>()?;
        let label = button
            .pending_text
            .clone()
            .unwrap_or_else(|| button.text.text());
        (label == text
            && node.size.min_element() > 1.
            && node.opacity > 0.
            && toolbar::mei::core::find_node_path(&mei.parents, *id).contains(&root))
        .then(|| mei_point(world, node.pos + node.size * 0.5))
        .flatten()
    })
}

fn backup_button_point(world: &World) -> Option<[i32; 2]> {
    let mei = world.resource::<MeiContext>();
    mei.nodes.iter().find_map(|(id, node)| {
        let content: &dyn std::any::Any = node.content.as_ref();
        let button = content.downcast_ref::<PillButtonContent>()?;
        (button.image == "menu/glade_backups"
            && node.size.min_element() > 1.
            && node.opacity > 0.
            && toolbar::mei::core::find_node_path(&mei.parents, *id)
                .contains(&GlobalId::new("delete")))
        .then(|| mei_point(world, node.pos + node.size * 0.5))
        .flatten()
    })
}

fn backup_row_point(world: &World) -> Option<[i32; 2]> {
    let mei = world.resource::<MeiContext>();
    mei.nodes.iter().find_map(|(id, node)| {
        let content: &dyn std::any::Any = node.content.as_ref();
        let button = content.downcast_ref::<MenuButtonContent>()?;
        let label = button
            .pending_text
            .clone()
            .unwrap_or_else(|| button.text.text());
        // A real backup row has a timestamp label; Cancel and pagination are
        // distinct siblings. Scope to the actual backup overlay's list slot.
        let in_overlay = toolbar::mei::core::find_node_path(&mei.parents, *id)
            .contains(&GlobalId::new("glade backups menu"));
        (in_overlay
            && node.size.min_element() > 1.
            && node.opacity > 0.
            && label != lang_lib::lookup("Cancel").as_ref()
            && label != "←"
            && label != "→")
            .then(|| mei_point(world, node.pos + node.size * 0.5))
            .flatten()
    })
}
impl Drop for Controller {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

fn button(world: &mut World, category: ToolbarCategory) -> Option<(Entity, [i32; 2], u8)> {
    let size = world.resource::<WindowSize>();
    let bounds = [size.width as f32, size.height as f32];
    world
        .query::<(
            Entity,
            &SwitchToolbarCategoryButton,
            &Ui2dExtent,
            &UiInteractionState,
        )>()
        .iter(world)
        .find_map(|(entity, target, extent, state)| {
            (target.category == category
                && extent.size.x > 1.0
                && extent.size.y > 1.0
                && extent.pos.x >= 0.0
                && extent.pos.y >= 0.0
                && extent.pos.x < bounds[0]
                && extent.pos.y < bounds[1])
                .then_some((
                    entity,
                    [extent.pos.x.round() as i32, extent.pos.y.round() as i32],
                    state.state,
                ))
        })
}

fn activation_zone_point(world: &mut World) -> Option<[i32; 2]> {
    let size = *world.resource::<WindowSize>();
    world.query_filtered::<&Ui2dExtent,
        With<toolbar::resouces_and_components::DebugToolbarActivationZone>>()
        .iter(world).find_map(|zone| {
            let point = zone.pos;
            (zone.size.min_element() > 1. && point.is_finite()
                && point.x >= 0. && point.y >= 0.
                && point.x < size.width as f32 && point.y < size.height as f32)
                .then_some([point.x.round() as i32, point.y.round() as i32])
        })
}

fn toolbar_session_entities(world: &mut World) -> Vec<Entity> {
    let mut entities: Vec<_> = world
        .query_filtered::<Entity, With<toolbar::resouces_and_components::TaffyNode>>()
        .iter(world)
        .collect();
    entities.extend(
        world
            .query::<&toolbar::resouces_and_components::UiSlider>()
            .iter(world)
            .filter_map(|slider| slider.knob_entity()),
    );
    entities.sort_by_key(|entity| entity.to_bits());
    entities.dedup();
    entities
}

fn snapshot(world: &World, phase: &str, frames: u64) {
    let toolbar = world.resource::<Toolbar>();
    let nav = world.resource::<UiNavState>();
    let history = world.resource::<History>().ledger();
    let meshes = world.resource::<country_core::asset_libraries::mesh_library::AssetMeshLibrary>();
    let rss_kib = std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|status| {
            status
                .lines()
                .find(|line| line.starts_with("VmRSS:"))
                .and_then(|line| line.split_whitespace().nth(1))
                .and_then(|value| value.parse::<u64>().ok())
        });
    eprintln!(
        "TOOLBAR_WINDOW {}",
        serde_json::json!({
            "phase": phase, "frames": frames, "world": format!("{:?}", world.id()),
            "mode": format!("{:?}", world.resource::<AppMode>()),
            "toolbar": format!("{:?}", toolbar.current), "nav_tag": nav.focus.tag,
            "nav_payload": nav.focus.payload, "nav_hold": nav.hold,
            "welcome": format!("{:?}", world.resource::<WelcomeScreenState>()),
            "welcome_dismissed": world.resource::<GameProfile>().welcome_was_dismissed(),
            "history_cursor": history.cursor, "history_entries": history.entries.len(),
            "menu": format!("{:?}", world.get_resource::<EscMenuState>()),
            "session": format!("{:?}", world.get_resource::<ActiveSession>().map(|active| active.id)),
            "focus_tracking": world.get_resource::<toolbar::resouces_and_components::ToolbarFocusTracking>().map(|tracking| tracking.len()),
            "save_thread": world.contains_resource::<SaveThreadHandle>(),
            "entities": world.entities().len(),
            "meshes": meshes.len(), "mesh_revision": meshes.registration_revision(),
            "tracked_transient_meshes": meshes.transient_meshes().lock().unwrap().len(),
            "resident_kib": rss_kib,
        })
    );
}

fn main() -> Result<()> {
    ensure!(
        cfg!(target_os = "linux"),
        "toolbar OS test currently requires Linux X11/XTest"
    );
    struct TimingLogger;
    impl log::Log for TimingLogger {
        fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
            matches!(
                metadata.target(),
                "tiny_glade::frame_timing" | "tiny_glade::startup"
            ) || metadata.level() <= log::Level::Warn
        }
        fn log(&self, record: &log::Record<'_>) {
            if self.enabled(record.metadata()) {
                eprintln!("{} {}", record.target(), record.args());
            }
        }
        fn flush(&self) {}
    }
    static LOGGER: TimingLogger = TimingLogger;
    log::set_logger(&LOGGER)?;
    log::set_max_level(log::LevelFilter::Info);
    let installed =
        PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").context("TG_INSTALL_ROOT required")?)
            .canonicalize()?;
    let account = tempfile::Builder::new()
        .prefix("toolbar-window-account-")
        .tempdir_in("/tmp")?;
    paths::initialize_paths_with_installation(0, Some(account.path()), &installed)?;
    std::env::set_current_dir(&installed)?;
    utils::local_time::init_local_time();
    tiny_glade::load_language_library::load_language_library(None, None);
    let cli = CliArgs::parse_from(["toolbar-window-chain", "--window-size", "960x540"]);
    let mut config = get_app_config(&cli)?;
    ensure!(
        !config.dev.demo && !config.dev.skip_splash,
        "requires original full startup"
    );
    let launch_display = tiny_glade::app_config::LaunchDisplayOverride::resolve(
        &cli,
        &config.video.display,
        true,
        false,
        None,
    )?
    .context("explicit window policy")?;
    launch_display.apply(&mut config.video.display);
    let mut app = run_game_thread(config, &cli, WindowSize::new(960, 540))?;
    app.insert_resource(launch_display);
    app.insert_resource(
        country_core::resources::platform::RuntimePlatform::from_detection(false, false),
    );
    let shaders = app.world().resource::<ApplicationShaderLibrary>().0.clone();
    let event_loop = EventLoop::new()?;
    let title = format!("Tiny Glade toolbar OS chain {}", std::process::id());
    let window = Arc::new(
        WindowBuilder::new()
            .with_title(&title)
            .with_inner_size(winit::dpi::PhysicalSize::new(960, 540))
            .build(&event_loop)?,
    );
    let mut game = GameLoop::new(
        app,
        window.clone(),
        GameLoopOptions {
            compiled_shader_root: installed.join("compiled-shaders"),
            frame_limit: None,
            input_recording: None,
        },
        shaders,
    )?;
    let mut monitors = country_core::resources::window::AvailableMonitors::default();
    GameLoop::extract_available_monitors_info(&mut monitors, &event_loop);
    game.app_mut().world_mut().insert_resource(monitors);
    let mut controller = Controller {
        title,
        script: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/toolbar_window_chain/x11_controller.py"),
        child: None,
    };
    let timeout = Duration::from_secs(
        std::env::var("TG_TOOLBAR_WINDOW_TIMEOUT_SECONDS")
            .ok()
            .map(|v| v.parse())
            .transpose()?
            .unwrap_or(1800),
    );
    let required_frames = std::env::var("TG_TOOLBAR_WINDOW_FRAMES")
        .ok()
        .map(|v| v.parse::<u64>())
        .transpose()?
        .unwrap_or(30)
        .max(30);
    let result = Rc::new(RefCell::new(None::<Result<()>>));
    let observed = result.clone();
    let started = Instant::now();
    let mut phase = 0u8;
    let mut phase_frame = 0u64;
    let mut frames = 0u64;
    let mut native_frames = 0u64;
    let mut previous_native = None::<(u64, u64)>;
    let mut steady_baseline = None::<(u64, u64, Instant)>;
    let mut steady_main_loop_ms = Vec::<f64>::new();
    let mut last_receipt = None;
    let mut baseline = None::<(AppMode, ToolbarCategory, i32, usize)>;
    let mut target_category = ToolbarCategory::Path;
    let mut target_mode = None::<AppMode>;
    let mut mouse_releases = 0u64;
    let mut enter_releases = 0u64;
    let mut close_received = false;
    let mut saves = EventCursor::<SavingGameComplete>::default();
    let mut transitions = EventCursor::<StateTransitionEvent<AppLoadingState>>::default();
    let mut saved_after_escape = false;
    let mut reload_seen = false;
    let mut game_after_reload_seen = false;
    let mut old_toolbar_entities = Vec::<Entity>::new();
    let mut session_before_reload = None;
    let mut save_worker_before_reload = None;
    let mut history_identity_before_reload = None;
    let clear_observation = Arc::new(Mutex::new(None::<std::result::Result<(), String>>));
    let mut last_startup_diagnostic = Instant::now();
    let mut toolbar_reveal_requested = false;
    let mut reload_toolbar_reveal_requested = false;
    event_loop.run(move |event, target| {
        let redraw = matches!(&event, Event::WindowEvent { event: WindowEvent::RedrawRequested, .. });
        let exiting = matches!(&event, Event::LoopExiting);
        if let Event::WindowEvent { window_id, event: ref input } = event {
            if window_id == window.id() {
                match input {
                    WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Left, .. } => mouse_releases += 1,
                    WindowEvent::KeyboardInput { event, .. } if event.state == ElementState::Released
                        && event.physical_key == winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Enter) => enter_releases += 1,
                    WindowEvent::CloseRequested => close_received = true,
                    _ => {}
                }
            }
        }
        let step = (|| -> Result<()> {
            let redraw_started = Instant::now();
            game.main_loop(event, target)?;
            let main_loop_ms = redraw_started.elapsed().as_secs_f64() * 1000.0;
            if phase == 0 && last_startup_diagnostic.elapsed() >= Duration::from_secs(30) {
                eprintln!("TOOLBAR_WINDOW raw_startup_observation elapsed_wall_seconds={:.3} progress={:?} receipt={:?}",
                    started.elapsed().as_secs_f64(), game.frame_progress(),
                    game.frame_observation().map(|frame| (frame.kind, frame.world, frame.receipt_frame,
                        frame.native_present_successes, frame.present_generation, frame.present_point)));
                last_startup_diagnostic = Instant::now();
            }
            if let Some(events) = game.app().world().get_resource::<Events<SavingGameComplete>>() {
                for saved in saves.read(events) {
                    if phase == 5 {
                        ensure!(saved.path == game.app().world().resource::<ActiveSession>().id.save_folder_path(),
                            "Escape saved a different session");
                        saved_after_escape = true;
                        eprintln!("TOOLBAR_WINDOW actual SavingGameComplete path={}", saved.path.display());
                    }
                }
            }
            if let Some(events) = game.app().world().get_resource::<Events<StateTransitionEvent<AppLoadingState>>>() {
                for transition in transitions.read(events) {
                    if phase >= 9 {
                        if transition.exited == Some(AppLoadingState::Game)
                            && transition.entered == Some(AppLoadingState::Reload) { reload_seen = true; }
                        if reload_seen && transition.exited == Some(AppLoadingState::Reload)
                            && transition.entered == Some(AppLoadingState::Game) { game_after_reload_seen = true; }
                        eprintln!("TOOLBAR_WINDOW actual state transition {transition:?}");
                    }
                }
            }
            if redraw {
                let observation = game.frame_observation().cloned();
                if let Some(frame) = observation.filter(|frame| last_receipt != Some((frame.world, frame.receipt_frame))
                    && matches!(frame.kind, ProductionGraphKind::Main { .. })) {
                    ensure!(frame.world == game.app().world().id(), "render receipt belongs to a different App");
                    ensure!(frame.present_point.is_some() && frame.native_present_successes > 0, "no native gameplay present");
                    if matches!(frame.kind, ProductionGraphKind::Main { world_rendered: true, .. }) {
                        ensure!(frame.draw_list_lengths.iter().any(|&count| count > 0), "empty world cannot pass");
                    }
                    let generation = frame.present_generation.context("missing native present generation")?;
                    native_frames += match previous_native {
                        Some((old_generation, successes)) if old_generation == generation => frame.native_present_successes.saturating_sub(successes),
                        _ => 1,
                    };
                    previous_native = Some((generation, frame.native_present_successes));
                    last_receipt = Some((frame.world, frame.receipt_frame));
                    frames += 1;
                    if phase == 0 && frames % 10 == 0 {
                        snapshot(game.app().world(), "startup_wait", frames);
                        let world = game.app().world();
                        eprintln!("TOOLBAR_WINDOW startup elapsed_wall_seconds={:.3} menu={:?} vermeer={:?}",
                            started.elapsed().as_secs_f64(), world.get_resource::<EscMenuState>(),
                            world.get_resource::<country_core::systems::render_loop::vermeer::VermeerState>()
                                .map(|state| (state.phase, state.frame_count, state.progress, state.timer, state.hold)));
                    }
                    if phase == 4 {
                        steady_main_loop_ms.push(main_loop_ms);
                        eprintln!("TOOLBAR_WINDOW steady_frame={} main_loop_redraw_wall_ms={main_loop_ms:.3}",
                            frames - steady_baseline.context("missing steady baseline")?.0);
                    }
                    let idle = controller.idle()?;
                    match phase {
                        0 if idle && frames >= 4
                            && matches!(game.app().world().resource::<WelcomeScreenState>(), WelcomeScreenState::Showing { elapsed } if *elapsed > 0.)
                            && !game.app().world().resource::<country_core::systems::render_loop::vermeer::VermeerState>().is_active()
                            && game.app().world().resource::<MeiContext>().nodes.contains_key(&GlobalId::new("welcome")) => {
                            let world = game.app_mut().world_mut();
                            let mode = *world.resource::<AppMode>();
                            let toolbar = world.resource::<Toolbar>();
                            target_category = if toolbar.current == ToolbarCategory::Path { ToolbarCategory::Building } else { ToolbarCategory::Path };
                            target_mode = toolbar.categories[target_category as usize].last_selected_mode();
                            ensure!(target_mode.is_some_and(|candidate| candidate != mode), "target must change actual mode");
                            let history = world.resource::<History>().ledger();
                            baseline = Some((mode, toolbar.current, history.cursor, history.entries.len()));
                            if let Some((_, position, _)) = button(world, target_category) {
                                snapshot(world, "ready_blocked_click", frames);
                                eprintln!("TOOLBAR_WINDOW target={target_category:?} point={position:?}");
                                controller.spawn("click", Some(position))?;
                                phase = 1; phase_frame = frames;
                            } else if !toolbar_reveal_requested {
                                if let Some(point) = activation_zone_point(world) {
                                    eprintln!("TOOLBAR_WINDOW move into actual toolbar activation zone point={point:?}");
                                    controller.spawn("move", Some(point))?;
                                    toolbar_reveal_requested = true;
                                }
                            }
                        }
                        1 if idle && mouse_releases >= 1 && frames >= phase_frame + 3 => {
                            let world = game.app().world();
                            let (mode, category, cursor, entries) = baseline.context("missing baseline")?;
                            ensure!(*world.resource::<AppMode>() == mode && world.resource::<Toolbar>().current == category,
                                "Welcome allowed physical click through to toolbar");
                            ensure!(!world.resource::<WelcomeScreenState>().is_closed(), "blocked click dismissed Welcome");
                            let history = world.resource::<History>().ledger();
                            ensure!((history.cursor, history.entries.len()) == (cursor, entries), "blocked click mutated history");
                            snapshot(world, "blocked_click_verified", frames);
                            controller.spawn("key", None)?;
                            phase = 2; phase_frame = frames;
                        }
                        2 if idle && enter_releases >= 1 && frames >= phase_frame + 6
                            && game.app().world().resource::<WelcomeScreenState>().is_closed() => {
                            let world = game.app_mut().world_mut();
                            ensure!(world.resource::<GameProfile>().welcome_was_dismissed(), "physical Enter did not update profile");
                            if let Some((_, position, state)) = button(world, target_category).filter(|(_, _, state)| *state != 0) {
                                snapshot(world, "welcome_closed_ready_tool", frames);
                                eprintln!("TOOLBAR_WINDOW target={target_category:?} point={position:?} state={state}");
                                controller.spawn("click", Some(position))?;
                                phase = 3; phase_frame = frames;
                            }
                        }
                        3 if idle && mouse_releases >= 2 && frames >= phase_frame + 4 => {
                            let world = game.app_mut().world_mut();
                            ensure!(Some(*world.resource::<AppMode>()) == target_mode, "physical tool click did not change authoritative AppMode");
                            ensure!(world.resource::<Toolbar>().current == target_category, "Toolbar disagrees with AppMode");
                            let (entity, _, _) = button(world, target_category).context("target button disappeared")?;
                            ensure!(world.get::<UiInteractionState>(entity).unwrap().selected == 1, "real button did not become selected");
                            let (_, _, cursor, entries) = baseline.context("missing baseline")?;
                            let history = world.resource::<History>().ledger();
                            ensure!((history.cursor, history.entries.len()) == (cursor, entries), "tool selection changed edit history");
                            snapshot(world, "tool_selection_verified", frames);
                            session_before_reload = Some(world.resource::<ActiveSession>().id);
                            save_worker_before_reload = Some(world.resource::<SaveThreadHandle>().join.thread().id());
                            history_identity_before_reload = Some(world.resource::<History>().identity());
                            let observed_clear = clear_observation.clone();
                            // Read-only lifecycle probe, ordered after the real owners.
                            // It neither clears state nor injects a transition/event.
                            game.app_mut().add_systems(OnExit(AppLoadingState::Game),
                                (move |world: &mut World| {
                                    let checked = (|| -> Result<()> {
                                        let ui = world.resource::<UiState>();
                                        ensure!(ui.registry_snapshot().is_empty() && ui.hover_intersections().is_empty()
                                            && ui.active_params().is_none() && ui.hover_byte() == 0,
                                            "old picker registry/capture survived OnExit");
                                        let nav = world.resource::<UiNavState>();
                                        let default_nav = UiNavState::default();
                                        ensure!(nav.focus.tag == default_nav.focus.tag && nav.focus.payload == default_nav.focus.payload
                                            && nav.hold == default_nav.hold, "old navigation survived OnExit");
                                        let mei = world.resource::<MeiContext>();
                                        ensure!(mei.nodes.is_empty() && mei.previous_nodes.is_empty()
                                            && mei.parents.is_empty() && mei.input_events.is_empty(), "old MEI owners survived OnExit");
                                        ensure!(world.resource::<toolbar::resouces_and_components::ToolbarFocusTracking>().is_empty(),
                                            "old toolbar focus tracking survived OnExit");
                                        ensure!(world.resource::<toolbar::resouces_and_components::ToolbarSliderSession>()
                                            == &toolbar::resouces_and_components::ToolbarSliderSession::default(),
                                            "old slider grab/navigation/focus survived OnExit");
                                        let vermeer = world.resource::<country_core::systems::render_loop::vermeer::VermeerState>();
                                        ensure!(!vermeer.is_active() && vermeer.background.is_none(),
                                            "old Vermeer reveal/background survived OnExit");
                                        let photo = world.resource::<country_core::resources::gui::PhotomodeScreen>();
                                        ensure!(!photo.enabled && !photo.was_enabled
                                            && !world.resource::<country_core::startup::init_photomode_state::SteamdeckScreenshotMsgScreen>().0,
                                            "old photo screen/message survived OnExit");
                                        ensure!(world.resource::<Events<toolbar::ui_interaction::UiClickEvent>>().is_empty()
                                            && world.resource::<Events<toolbar::ui_interaction::UiDragEvent>>().is_empty()
                                            && world.resource::<Events<toolbar::ui_interaction::UiJustBecameInFocusEvent>>().is_empty(),
                                            "old toolbar input events survived OnExit");
                                        ensure!(world.contains_resource::<SaveThreadHandle>()
                                            && world.resource::<AutoSaveState>().is_command_attached(),
                                            "session unload shut down the process save worker");
                                        Ok(())
                                    })();
                                    eprintln!("TOOLBAR_WINDOW OnExit toolbar clear probe: {checked:?}");
                                    *observed_clear.lock().unwrap() = Some(checked.map_err(|error| format!("{error:#}")));
                                }).after(tiny_glade::toolbar_plugin::clear_toolbar_session)
                                    .after(country_core::startup::clearing::despawn_glade_entities));
                            let world = game.app().world();
                            snapshot(world, "steady_begin", frames);
                            steady_baseline = Some((frames, native_frames, Instant::now()));
                            phase = 4;
                        }
                        4 if idle && steady_baseline.is_some_and(|(first_frame, first_native, _)|
                            frames - first_frame >= required_frames && native_frames - first_native >= required_frames) => {
                            let (first_frame, first_native, clock) = steady_baseline.context("missing steady baseline")?;
                            snapshot(game.app().world(), "steady_end", frames);
                            let mut samples = steady_main_loop_ms.clone();
                            samples.sort_by(f64::total_cmp);
                            eprintln!("TOOLBAR_WINDOW {}", serde_json::json!({
                                "phase": "steady_observation", "frames": frames - first_frame,
                                "native_frames": native_frames - first_native,
                                "elapsed_wall_ms": clock.elapsed().as_secs_f64() * 1000.0,
                                "main_loop_redraw_mean_wall_ms": samples.iter().sum::<f64>() / samples.len() as f64,
                                "main_loop_redraw_p95_wall_ms": samples[(samples.len() - 1) * 95 / 100],
                                "main_loop_redraw_p99_wall_ms": samples[(samples.len() - 1) * 99 / 100],
                                "build_profile": if cfg!(debug_assertions) { "debug" } else { "release" },
                                "performance_verdict": "not_evaluated",
                                "timing_scope": "main-loop thread wall time including render handover and limiter; TG_FRAME_TIMING provides stage attribution",
                            }));
                            snapshot(game.app().world(), "ready_escape_save", frames);
                            controller.spawn_input("key", None, "Escape")?;
                            phase = 5;
                        }
                        5 if idle && saved_after_escape && matches!(game.app().world().resource::<EscMenuState>(), EscMenuState::Main) => {
                            let world = game.app().world();
                            let active = world.resource::<ActiveSession>();
                            let path = active.id.save_folder_path().join("history.json");
                            country_core::systems::save::load_game::parse_saved_game_data(
                                &path, world.resource::<country_core::resources::glade_settings::GladeSettings>())
                                .context("real synchronously saved history must parse")?;
                            let backups = SaveBackupList::iter_backup_files(&active.id.backup_folder_path())?
                                .filter(|backup| !backup.kind.is_corrupted()).count();
                            if backups > 0 {
                                if let Some(point) = menu_label_point(world, &lang_lib::lookup("Menu.Load").to_uppercase(), GlobalId::new("esc screen")) {
                                    eprintln!("TOOLBAR_WINDOW real saved history={} backup_count={backups} Load point={point:?}", path.display());
                                    controller.spawn("click", Some(point))?;
                                    phase = 6;
                                }
                            }
                        }
                        6 if idle => {
                            let world = game.app().world();
                            if let EscMenuState::Load(state) = world.resource::<EscMenuState>() {
                                let active = world.resource::<ActiveSession>().id;
                                let index = world.resource::<SavedSessionList>().sessions.iter()
                                    .position(|session| session.id == active).context("saved active session missing from actual Load list")? + 1;
                                let metrics = *state.thumbnail_metrics.lock();
                                if metrics.cells_per_line != 0 {
                                    if let Some(point) = metrics.positions().nth(index)
                                        .and_then(|position| mei_point(world, position * 1000.)) {
                                        eprintln!("TOOLBAR_WINDOW hover real active thumbnail index={index} point={point:?}");
                                        controller.spawn("move", Some(point))?;
                                        phase = 7;
                                    }
                                }
                            }
                        }
                        7 if idle => {
                            let world = game.app().world();
                            if let EscMenuState::Load(state) = world.resource::<EscMenuState>() {
                                if state.hovered_index.is_some_and(|index| index > 0) {
                                    if let Some(point) = backup_button_point(world) {
                                        eprintln!("TOOLBAR_WINDOW real view_backups point={point:?}");
                                        controller.spawn("click", Some(point))?;
                                        phase = 8;
                                    }
                                }
                            }
                        }
                        8 if idle => {
                            let world = game.app_mut().world_mut();
                            if let EscMenuState::Load(state) = world.resource::<EscMenuState>() {
                                if state.backups.as_ref().is_some_and(|backup| Some(backup.session) == session_before_reload) {
                                    if let Some(point) = backup_row_point(world) {
                                        old_toolbar_entities = toolbar_session_entities(world);
                                        ensure!(!old_toolbar_entities.is_empty(), "no real toolbar owner entities to track across Reload");
                                        eprintln!("TOOLBAR_WINDOW actual pre-Reload toolbar Taffy/knob entities={}", old_toolbar_entities.len());
                                        snapshot(world, "ready_actual_backup_restore_same_session", frames);
                                        controller.spawn("click", Some(point))?;
                                        phase = 9;
                                    }
                                }
                            }
                        }
                        9 if idle && reload_seen && game_after_reload_seen => {
                            if let Some(checked) = clear_observation.lock().unwrap().as_ref() {
                                checked.as_ref().map_err(|error| anyhow::anyhow!("OnExit clear failed: {error}"))?;
                            } else { anyhow::bail!("actual Reload did not execute the toolbar OnExit probe"); }
                            let world = game.app_mut().world_mut();
                            ensure!(Some(world.resource::<ActiveSession>().id) == session_before_reload,
                                "backup restore unexpectedly changed SessionId");
                            ensure!(old_toolbar_entities.iter().all(|entity| !world.entities().contains(*entity)),
                                "old toolbar entity generation survived Reload");
                            ensure!(Some(world.resource::<SaveThreadHandle>().join.thread().id()) == save_worker_before_reload
                                && world.resource::<AutoSaveState>().is_command_attached(), "Reload replaced or disconnected the process save worker");
                            let loaded = world.resource::<country_core::systems::save::load_game::LoadGameManager>().state
                                == country_core::systems::save::load_game::LoadGameState::Loaded;
                            if loaded && *world.resource::<AppMode>() != AppMode::Replay
                                && !world.resource::<MeiContext>().nodes.is_empty() {
                                let (_, _, cursor, entries) = baseline.context("missing history baseline")?;
                                let history = world.resource::<History>();
                                ensure!(history.ledger().cursor == cursor && history.ledger().entries.len() == entries
                                    && Some(history.identity()) == history_identity_before_reload,
                                    "real backup restore changed the unchanged toolbar history/identity");
                                let toolbar = world.resource::<Toolbar>();
                                target_category = if toolbar.current == ToolbarCategory::Path { ToolbarCategory::Building } else { ToolbarCategory::Path };
                                target_mode = toolbar.categories[target_category as usize].last_selected_mode();
                                ensure!(target_mode.is_some_and(|mode| mode != *world.resource::<AppMode>()),
                                    "rebuilt toolbar target must change actual mode");
                                if let Some((_, point, _)) = button(world, target_category).filter(|(_, _, state)| *state != 0) {
                                    snapshot(world, "same_session_reload_rebuilt", frames);
                                    controller.spawn("click", Some(point))?;
                                    phase = 10; phase_frame = frames;
                                } else if !reload_toolbar_reveal_requested {
                                    if let Some(point) = activation_zone_point(world) {
                                        eprintln!("TOOLBAR_WINDOW reveal rebuilt toolbar at actual activation zone point={point:?}");
                                        controller.spawn("move", Some(point))?;
                                        reload_toolbar_reveal_requested = true;
                                    }
                                }
                            }
                        }
                        10 if idle && frames >= phase_frame + 4 => {
                            let world = game.app_mut().world_mut();
                            ensure!(Some(*world.resource::<AppMode>()) == target_mode
                                && world.resource::<Toolbar>().current == target_category,
                                "real input failed to operate the rebuilt toolbar after Reload");
                            let (entity, _, _) = button(world, target_category).context("rebuilt toolbar missing")?;
                            ensure!(world.get::<UiInteractionState>(entity).unwrap().selected == 1,
                                "rebuilt toolbar selection disagrees with authoritative mode");
                            snapshot(world, "same_session_reload_input_verified", frames);
                                steady_baseline = Some((frames, native_frames, Instant::now()));
                            phase = 12;
                        }
                        12 if idle && steady_baseline.is_some_and(|(first_frame, first_native, _)|
                            frames - first_frame >= required_frames && native_frames - first_native >= required_frames) => {
                            snapshot(game.app().world(), "reload_steady_end_ready_os_close", frames);
                            controller.spawn("close", None)?;
                            phase = 11;
                        }
                        _ => {}
                    }
                }
            }
            if exiting {
                let world = game.app().world();
                ensure!(phase == 11 && close_received && reload_seen && game_after_reload_seen && steady_baseline.is_some_and(|(first_frame, first_native, _)|
                    frames - first_frame >= required_frames && native_frames - first_native >= required_frames),
                    "premature OS exit: phase={phase} frames={frames} native={native_frames}");
                ensure!(!world.contains_resource::<SaveThreadHandle>()
                    && world.get_resource::<AutoSaveState>().is_some_and(|state| !state.is_command_attached()), "save owner not released on real OS close");
                ensure!(!game.gameplay_preparation_pending(), "gameplay preparation owner leaked");
                snapshot(world, "closed_save_preparation_cleanup_verified", frames);
                eprintln!("PASS: actual XTest mouse/Enter -> production GameLoop Welcome arbitration/tool selection -> Escape synchronous save -> real Load/backups geometry -> same-SessionId backup Reload/OnExit/rebuild/30 native presents -> WM_DELETE_WINDOW/save/preparation shutdown; frames={frames}, native_frames={native_frames}, mouse_releases={mouse_releases}, enter_releases={enter_releases}; GPU teardown requires process exit and validation-layer evidence");
                *observed.borrow_mut() = Some(Ok(()));
            } else {
                ensure!(started.elapsed() < timeout, "OS toolbar chain timed out: phase={phase} frames={frames} mouse={mouse_releases} enter={enter_releases}");
            }
            Ok(())
        })();
        if let Err(error) = step {
            if observed.borrow().is_none() { *observed.borrow_mut() = Some(Err(error)); }
            target.exit();
        }
    })?;
    let outcome = result
        .borrow_mut()
        .take()
        .context("OS loop left no toolbar result")?;
    drop(account);
    outcome
}
