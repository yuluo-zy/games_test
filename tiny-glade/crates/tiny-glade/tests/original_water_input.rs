//! Shipped session: actual toolbar and pointer input through the gameplay App.
#[path = "support/mod.rs"]
mod test_support;
use test_support::BridgeTestDriver as _;

use anyhow::{Context, Result, ensure};
use clap::Parser;
use country_core::{
    components::Ui2dExtent,
    resources::{
        cursor_terrain_raycast::CursorTerrainRaycast,
        history::{EditType, History, ReplayBuffer},
        mouse_terrain_raycast::CursorPositionSS,
        session::ActiveSession,
        toolbar::ToolbarCategory,
        walls::InnerWalls,
        window::WindowSize,
        world_raster_defs::WaterRaster,
    },
    systems::{
        backend::input::{
            button_input::{KeyButtonInput, MouseButtonInput},
            translate_input_to_cursor_moved::InputCursorMoved,
        },
        history::PendingHistoryCommands,
        mode_manager::AppMode,
        save::load_game::LoadGameManager,
        ui::resources::ui_state::{UiState, WidgetId},
    },
};
use tiny_glade::{
    app_config::{CliArgs, get_app_config},
    render_bridge::CoreRenderBridge,
    run_game::{ApplicationShaderLibrary, run_game_thread},
};

use bevy_ecs::event::{EventCursor, Events};
use country_core::{
    resources::{
        glade_border::{DisplayOutOfBorder, FeedbackInputOutOfBorder},
        water::WaterDrawEvent,
    },
    systems::{
        render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd,
        terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback,
    },
};
use std::sync::atomic::Ordering;

struct WaterObservation {
    draws: EventCursor<WaterDrawEvent>,
    recalcs: EventCursor<RecalculateHeightMapCmd>,
    borders: EventCursor<DisplayOutOfBorder>,
    feedback: EventCursor<FeedbackInputOutOfBorder>,
    draw_events: Vec<WaterDrawEvent>,
    recalc_count: usize,
    border_count: usize,
    feedback_count: usize,
}
impl WaterObservation {
    fn new(world: &bevy_ecs::world::World) -> Self {
        Self {
            draws: world
                .resource::<Events<WaterDrawEvent>>()
                .get_cursor_current(),
            recalcs: world
                .resource::<Events<RecalculateHeightMapCmd>>()
                .get_cursor_current(),
            borders: world
                .resource::<Events<DisplayOutOfBorder>>()
                .get_cursor_current(),
            feedback: world
                .resource::<Events<FeedbackInputOutOfBorder>>()
                .get_cursor_current(),
            draw_events: Vec::new(),
            recalc_count: 0,
            border_count: 0,
            feedback_count: 0,
        }
    }
    fn sample(&mut self, world: &bevy_ecs::world::World) {
        self.draw_events.extend(
            self.draws
                .read(world.resource::<Events<WaterDrawEvent>>())
                .copied(),
        );
        self.recalc_count += self
            .recalcs
            .read(world.resource::<Events<RecalculateHeightMapCmd>>())
            .count();
        self.border_count += self
            .borders
            .read(world.resource::<Events<DisplayOutOfBorder>>())
            .count();
        self.feedback_count += self
            .feedback
            .read(world.resource::<Events<FeedbackInputOutOfBorder>>())
            .count();
    }
}

fn tick(
    driver: &mut tiny_glade::game_loop::GameFrameDriver,
    context: &egui::Context,
    position: glam::Vec2,
    held: bool,
) {
    let world = driver.app_mut().world_mut();
    let mut mouse = world.resource_mut::<MouseButtonInput>();
    mouse.clear();
    if held {
        mouse.press(0);
    } else {
        mouse.release(0);
    }
    world.send_event(InputCursorMoved::Absolute(position));
    context.begin_frame(egui::RawInput::default());
    driver.app_mut().update();
    let _ = context.end_frame();
}

fn render(
    driver: &mut tiny_glade::game_loop::GameFrameDriver,
    bridge: &mut CoreRenderBridge,
) -> Result<()> {
    let world = driver.app_mut().world_mut();
    let ready = tiny_glade::production_renderer::prepare_headless_gameplay_frame(bridge, world)?;
    bridge.prepare_test_execution()?;
    let frame = ready.execute(bridge, world)?;
    world
        .resource_mut::<country_core::render::RenderTemporalState>()
        .frame_render_returned();
    bridge.with_session(|session| session.wait_frame(frame.receipt.frame, 120_000_000_000))?;
    drop(frame);
    bridge.collect_completed_frames()?;
    Ok(())
}

fn replay_with_key(
    driver: &mut tiny_glade::game_loop::GameFrameDriver,
    context: &egui::Context,
    bridge: &mut CoreRenderBridge,
    position: glam::Vec2,
    binding: usize,
    expected_cursor: i32,
    stage: &str,
) -> Result<()> {
    use bevy_ecs::event::Events;
    use country_core::{
        game_profile::GameProfile,
        systems::history::replay_raster::{RasterDomain, RasterReplayFeedback},
    };
    let key = driver
        .app()
        .world()
        .resource::<GameProfile>()
        .key_bindings
        .bindings[binding]
        .with_context(|| format!("{stage} key binding missing"))?;
    let code = country_core::game_profile::desktop_keycode_index(key)?;
    let world = driver.app().world();
    let mut samples = world
        .resource::<Events<RasterReplayFeedback>>()
        .get_cursor();
    let mut draws = world.resource::<Events<WaterDrawEvent>>().get_cursor();
    samples
        .read(world.resource::<Events<RasterReplayFeedback>>())
        .for_each(drop);
    draws
        .read(world.resource::<Events<WaterDrawEvent>>())
        .for_each(drop);
    let mut sample_count = 0usize;
    let mut draw_count = 0usize;
    let mut stable = 0usize;
    for frame in 0..600 {
        {
            let mut keys = driver
                .app_mut()
                .world_mut()
                .resource_mut::<KeyButtonInput>();
            keys.release_all();
            keys.clear();
            if frame == 0 {
                keys.press(code);
            }
        }
        tick(driver, context, position, false);
        render(driver, bridge)?;
        let world = driver.app().world();
        sample_count += samples
            .read(world.resource::<Events<RasterReplayFeedback>>())
            .filter(|event| event.domain == RasterDomain::Water && event.replay)
            .count();
        draw_count += draws
            .read(world.resource::<Events<WaterDrawEvent>>())
            .filter(|event| event.replay)
            .count();
        let history = world.resource::<History>();
        let done = history.ledger().cursor == expected_cursor
            && !history.is_recording()
            && world.resource::<ReplayBuffer>().is_empty()
            && world.resource::<PendingHistoryCommands>().0.is_empty()
            && world.resource::<AppMode>().native_tag() != 13;
        stable = if done { stable + 1 } else { 0 };
        if stable >= 3 {
            ensure!(
                binding == 21 || (sample_count > 0 && draw_count > 0),
                "{stage} completed without Water replay feedback: samples={sample_count}, draw events={draw_count}"
            );
            return Ok(());
        }
    }
    anyhow::bail!(
        "{stage} did not finish: cursor={}, replay_empty={}, pending={}, mode={:?}, samples={sample_count}, draw events={draw_count}",
        driver.app().world().resource::<History>().ledger().cursor,
        driver.app().world().resource::<ReplayBuffer>().is_empty(),
        driver
            .app()
            .world()
            .resource::<PendingHistoryCommands>()
            .0
            .len(),
        driver.app().world().resource::<AppMode>()
    )
}

#[test]
fn shipped_app_water_pointer_stroke_records_authoritative_raster_and_history() -> Result<()> {
    let installed = std::env::var_os("TG_INSTALL_ROOT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../../../ljxsj_92385/Tiny Glade")
        })
        .canonicalize()?;
    let account = tempfile::tempdir()?;
    paths::initialize_paths_with_installation(0, Some(account.path()), &installed)?;
    std::env::set_current_dir(&installed)?;
    utils::local_time::init_local_time();
    tiny_glade::load_language_library::load_language_library(None, None);
    let cli = CliArgs::parse_from(["original-water-input"]);
    let config = get_app_config(&cli)?;
    ensure!(
        !config.dev.demo,
        "water input fixture requires the full release"
    );
    let mut app = run_game_thread(config, &cli, WindowSize::new(480, 270))?;
    app.insert_resource(
        country_core::resources::platform::RuntimePlatform::from_detection(false, false),
    );
    tiny_glade::game_loop::register_platform_events(&mut app);
    tiny_glade::game_loop::register_settings_application_system(&mut app);
    let device = rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig {
        memory_budget_bytes: tiny_glade::production_renderer::UNBOUNDED_DEVICE_BUDGET_BYTES,
        ..rhapsody::device::DeviceConfig::headless()
    })?;
    let shaders = app.world().resource::<ApplicationShaderLibrary>().0.clone();
    let mut bridge = CoreRenderBridge::new(device.clone(), installed.clone(), shaders.clone())?;
    let mut final_post = None;
    country_core::startup::load_shaders::load_shaders(
        &shaders,
        bridge.shader_backend(),
        &mut final_post,
    );
    country_core::startup::load_shaders::load_ui_shaders(&shaders, bridge.shader_backend());
    country_core::startup::startup_blit::startup_blit(&shaders, bridge.shader_backend());
    struct Tail<'a> {
        library: &'a country_core::asset_libraries::shader_library::AssetShaderLibrary,
        backend: &'a dyn country_core::render::shader::boundary::ShaderBackend,
    }
    impl country_core::systems::render_loop::painterly::ShaderLoader for Tail<'_> {
        fn load_shader_into_library(&mut self, path: &'static str, name: &'static str) {
            country_core::startup::load_shaders::load_shader_into_library(
                self.library,
                self.backend,
                path,
                name,
            );
        }
    }
    country_core::systems::render_loop::painterly::startup_painterly(&mut Tail {
        library: &shaders,
        backend: bridge.shader_backend(),
    });
    country_core::systems::render_loop::vermeer::startup_vermeer(&mut Tail {
        library: &shaders,
        backend: bridge.shader_backend(),
    });
    bridge.install_asset_libraries(app.world_mut())?;
    tiny_glade::gameplay_startup::install_gameplay_device_runtime(
        &mut app,
        device,
        bridge.hosts().clone(),
        installed,
    )?;
    let context = egui::Context::default();
    let mut driver = tiny_glade::game_loop::GameFrameDriver::new(app, context.clone());
    driver
        .app_mut()
        .insert_resource(bevy_time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_secs_f64(1.0 / 60.0),
        ));
    let mut loaded = false;
    for _ in 0..600 {
        tick(&mut driver, &context, glam::Vec2::ZERO, false);
        let world = driver.app().world();
        if world.contains_resource::<ActiveSession>()
            && world.resource::<LoadGameManager>().options.is_none()
            && world.resource::<InnerWalls>().wall_ids().count() > 0
        {
            loaded = true;
            break;
        }
    }
    ensure!(loaded, "shipped intro did not load");
    for _ in 0..90 {
        tick(&mut driver, &context, glam::Vec2::ZERO, false);
    }
    let world = driver.app_mut().world_mut();
    tiny_glade::production_renderer::warm_up_gameplay(bridge.warmup_owners()?, world)?;
    bridge.accept_gameplay_warmup(
        world.resource::<tiny_glade::render_bridge::CompletedGameplayWarmup>(),
    )?;
    for _ in 0..4 {
        tick(&mut driver, &context, glam::Vec2::ZERO, false);
        render(&mut driver, &mut bridge)?;
    }

    use country_core::{
        components::UiInteractionState, game_profile::GameProfile,
        resources::onboarding::WelcomeScreenState, systems::render_loop::vermeer::VermeerState,
    };
    use toolbar::resouces_and_components::{
        DebugToolbarActivationZone, ShowOnlyInPhotomode, SwitchToolbarCategoryButton, TaffyNode,
        ToolbarButtonCooldown,
    };

    // Welcome is a real modal: its confirm handler is gated by the rendered
    // Vermeer reveal, and hide_ui keeps the toolbar's interaction state at 0
    // until Welcome closes. App-only ticks cannot advance that render pass.
    for _ in 0..240 {
        if !driver.app().world().resource::<VermeerState>().is_active() {
            break;
        }
        tick(&mut driver, &context, glam::Vec2::ZERO, false);
        render(&mut driver, &mut bridge)?;
    }
    ensure!(
        !driver.app().world().resource::<VermeerState>().is_active(),
        "Vermeer reveal did not finish: {:?}",
        driver.app().world().resource::<VermeerState>()
    );
    for _ in 0..12 {
        tick(&mut driver, &context, glam::Vec2::ZERO, false);
    }
    if !driver
        .app()
        .world()
        .resource::<WelcomeScreenState>()
        .is_closed()
    {
        ensure!(
            matches!(driver.app().world().resource::<WelcomeScreenState>(),
            WelcomeScreenState::Showing { elapsed } if *elapsed > 0.0),
            "Welcome did not reach its live confirm handler: {:?}",
            driver.app().world().resource::<WelcomeScreenState>()
        );
        let key = driver
            .app()
            .world()
            .resource::<GameProfile>()
            .key_bindings
            .bindings[29]
            .context("Welcome confirm key binding missing")?;
        let code = country_core::game_profile::desktop_keycode_index(key)?;
        for _ in 0..2 {
            {
                let mut keys = driver
                    .app_mut()
                    .world_mut()
                    .resource_mut::<KeyButtonInput>();
                keys.release_all();
                keys.clear();
                keys.press(code);
            }
            tick(&mut driver, &context, glam::Vec2::ZERO, false);
            {
                let mut keys = driver
                    .app_mut()
                    .world_mut()
                    .resource_mut::<KeyButtonInput>();
                keys.release_all();
                keys.clear();
            }
            tick(&mut driver, &context, glam::Vec2::ZERO, false);
            for _ in 0..4 {
                tick(&mut driver, &context, glam::Vec2::ZERO, false);
            }
            if driver
                .app()
                .world()
                .resource::<WelcomeScreenState>()
                .is_closed()
            {
                break;
            }
        }
        ensure!(
            driver
                .app()
                .world()
                .resource::<WelcomeScreenState>()
                .is_closed(),
            "physical confirm did not close Welcome: {:?}",
            driver.app().world().resource::<WelcomeScreenState>()
        );
        ensure!(
            driver
                .app()
                .world()
                .resource::<GameProfile>()
                .welcome_was_dismissed(),
            "Welcome closed without its profile dismissal"
        );
    }

    // Move into the same activation rectangle used by toolbar_auto_hide. The
    // debug zone may not exist, so compute its exact production geometry from
    // the live button query, config and size multiplier in that case.
    let reveal = {
        let world = driver.app_mut().world_mut();
        let zone = world
            .query_filtered::<&Ui2dExtent, bevy_ecs::query::With<DebugToolbarActivationZone>>()
            .iter(world)
            .find(|extent| extent.size.min_element() > 1.0 && extent.pos.is_finite())
            .map(|extent| extent.pos);
        zone.unwrap_or_else(|| {
            let count = world
                .query::<(
                    &ToolbarButtonCooldown,
                    &UiInteractionState,
                    Option<&ShowOnlyInPhotomode>,
                )>()
                .iter(world)
                .filter(|(_, _, photo)| photo.is_none())
                .count() as i32;
            let config = world.resource::<country_core::game_config::GameConfig>();
            let window = world.resource::<WindowSize>();
            let scale = config.ui.icon_size_mult
                * world
                    .resource::<toolbar::toolbar_update::ToolbarSizeMult>()
                    .multiplier;
            toolbar::toolbar_systems::toolbar_activation_geometry(
                config.ui.toolbar_alignment as u8,
                window.as_vec2(),
                scale,
                count,
            )
            .0
        })
    };
    let window = *driver.app().world().resource::<WindowSize>();
    ensure!(
        reveal.is_finite()
            && reveal.x >= 0.0
            && reveal.y >= 0.0
            && reveal.x < window.width as f32
            && reveal.y < window.height as f32,
        "invalid toolbar activation point: {reveal:?}"
    );
    for _ in 0..48 {
        tick(&mut driver, &context, reveal, false);
    }
    let world = driver.app_mut().world_mut();
    let button = world
        .query::<(bevy_ecs::entity::Entity, &SwitchToolbarCategoryButton)>()
        .iter(world)
        .find(|(_, b)| b.category == ToolbarCategory::Water)
        .map(|(entity, _)| entity)
        .context("Water category button missing")?;
    // Layout and picker consume different frames; follow the moving button
    // only after it is visible, on screen and has nonzero bounds.
    for _ in 0..48 {
        let world = driver.app().world();
        let extent = world
            .get::<Ui2dExtent>(button)
            .context("Water button extent missing")?;
        let state = world
            .get::<UiInteractionState>(button)
            .context("Water button interaction missing")?;
        if state.state != 0
            && extent.size.min_element() > 1.0
            && extent.pos.is_finite()
            && extent.pos.x >= 0.0
            && extent.pos.y >= 0.0
            && extent.pos.x < window.width as f32
            && extent.pos.y < window.height as f32
        {
            let position = extent.pos;
            tick(&mut driver, &context, position, false);
        } else {
            tick(&mut driver, &context, reveal, false);
        }
        let world = driver.app().world();
        let node = world
            .get::<TaffyNode>(button)
            .context("Water button Taffy node missing")?
            .node;
        if world.resource::<UiState>().active_widget()
            == WidgetId::from_name(&format!("toolbar_button_{node:?}"))
        {
            break;
        }
    }
    let world = driver.app().world();
    let extent = world.get::<Ui2dExtent>(button).unwrap();
    let button_pos = extent.pos;
    let node = world
        .get::<TaffyNode>(button)
        .context("Water button Taffy node missing")?
        .node;
    let button_widget = WidgetId::from_name(&format!("toolbar_button_{node:?}"));
    ensure!(
        world
            .get::<UiInteractionState>(button)
            .is_some_and(|state| state.state != 0)
            && extent.size.min_element() > 1.0
            && extent.pos.is_finite()
            && extent.pos.x >= 0.0
            && extent.pos.y >= 0.0
            && extent.pos.x < window.width as f32
            && extent.pos.y < window.height as f32
            && world.resource::<UiState>().active_widget() == button_widget,
        "Water button was not visible and picked: extent={extent:?}, state={:?}, welcome={:?}, cursor={:?}, active={:?}, reveal={reveal:?}",
        world.get::<UiInteractionState>(button),
        world.resource::<WelcomeScreenState>(),
        world.resource::<CursorPositionSS>(),
        world.resource::<UiState>().active_widget()
    );
    tick(&mut driver, &context, button_pos, true);
    tick(&mut driver, &context, button_pos, false);
    for _ in 0..3 {
        tick(&mut driver, &context, button_pos, false);
    }
    ensure!(
        *driver.app().world().resource::<AppMode>() == AppMode::Water,
        "toolbar click did not select Water"
    );
    ensure!(
        driver
            .app()
            .world()
            .resource::<country_core::resources::toolbar::Toolbar>()
            .current
            == ToolbarCategory::Water
            && driver
                .app()
                .world()
                .get::<UiInteractionState>(button)
                .is_some_and(|state| state.selected == 1),
        "Water toolbar category/button did not become selected"
    );

    // Search actual screen pixels using the real picker and terrain ray system.
    // Require a live Water canvas and an in-border terrain hit before pressing.
    let mut target = None;
    let mut target_search_points = 0usize;
    for y in (35..225).step_by(12) {
        for x in (35..445).step_by(12) {
            let p = glam::Vec2::new(x as f32, y as f32);
            for _ in 0..2 {
                tick(&mut driver, &context, p, false);
            }
            target_search_points += 1;
            if target_search_points % 8 == 0 {
                render(&mut driver, &mut bridge)?;
            }
            let world = driver.app().world();
            let cursor = world.resource::<CursorTerrainRaycast>();
            let ray =
                world.resource::<country_core::resources::mouse_terrain_raycast::CameraCursorRay>();
            let hit = world
                .resource::<country_core::resources::terrain_height::TerrainHeightsData>()
                .intersect_ray(ray.ray(), f32::MAX);
            let brush = world
                .resource::<country_core::resources::brush_sizes::BrushSizes>()
                .water;
            let inside = world
                .resource::<country_core::resources::glade_border::GladeBorder>()
                .is_shape_inside(country_core::resources::glade_border::BorderShape::Circle(
                    utils::geometry::Circle2d::new(
                        glam::Vec2::new(cursor.pos.x, cursor.pos.z),
                        brush * 0.8 + 5.0,
                    ),
                ));
            if hit.is_some()
                && cursor.tag == 1
                && inside
                && world.resource::<UiState>().active_widget()
                    == WidgetId::clearing_canvas("watereraser_water_")
                && (world.resource::<CursorPositionSS>().0 - p).length() < 1.0
            {
                target = Some(p);
                break;
            }
        }
        if target.is_some() {
            break;
        }
    }
    let target = target.context("no live Water canvas with in-border terrain hit")?;
    let world = driver.app().world();
    ensure!(
        world.resource::<ReplayBuffer>().is_empty()
            && world.resource::<PendingHistoryCommands>().0.is_empty(),
        "intro replay still pending"
    );
    let before = world.resource::<WaterRaster>().composite().to_vec();
    let count = world.resource::<History>().ledger().entries.len();
    let cursor = world.resource::<History>().ledger().cursor;
    ensure!(
        !world.resource::<History>().is_recording(),
        "history transaction already active"
    );
    ensure!(
        cursor + 1 == count as i32,
        "intro history is not at the end of its chain"
    );
    let pending_before = world
        .resource::<ScheduledTerrainReadback>()
        .pending
        .load(Ordering::SeqCst);
    let mut observed = WaterObservation::new(world);
    tick(&mut driver, &context, target, true);
    observed.sample(driver.app().world());
    ensure!(
        observed.recalc_count > 0,
        "water begin did not produce terrain recalculation"
    );
    ensure!(
        driver
            .app()
            .world()
            .resource::<ScheduledTerrainReadback>()
            .pending
            .load(Ordering::SeqCst)
            > pending_before,
        "water begin did not schedule terrain readback"
    );
    for _ in 0..5 {
        tick(
            &mut driver,
            &context,
            target + glam::Vec2::new(3.0, 0.0),
            true,
        );
        observed.sample(driver.app().world());
    }
    tick(
        &mut driver,
        &context,
        target + glam::Vec2::new(3.0, 0.0),
        false,
    );
    observed.sample(driver.app().world());
    for _ in 0..30 {
        tick(&mut driver, &context, target, false);
        let history = driver.app().world().resource::<History>();
        if !history.is_recording() && history.ledger().entries.len() > count {
            break;
        }
    }
    let world = driver.app().world();
    let history = world.resource::<History>();
    ensure!(!history.is_recording(), "Water transaction did not finish");
    ensure!(
        history.ledger().cursor == cursor + 1 && history.ledger().entries.len() == count + 1,
        "Water stroke did not commit exactly one history entry"
    );
    let entry = history
        .ledger()
        .entries
        .last()
        .context("missing committed Water edit")?;
    let strokes: Vec<_> = entry
        .edits
        .iter()
        .filter_map(|edit| match edit {
            EditType::Water(stroke) => Some(stroke),
            _ => None,
        })
        .collect();
    ensure!(
        strokes.len() == 1 && !strokes[0].points.is_empty() && strokes[0].draw,
        "committed entry lacks its actual additive Water stroke"
    );
    // The water strength word remains unproven; assert only the evidenced radius.
    ensure!(
        strokes[0].radius
            == world
                .resource::<country_core::resources::brush_sizes::BrushSizes>()
                .water,
        "committed Water stroke lost its live brush radius"
    );
    ensure!(
        world.resource::<WaterRaster>().composite() != before.as_slice(),
        "Water raster did not change"
    );
    ensure!(
        !observed.draw_events.is_empty(),
        "physical drag emitted no WaterDrawEvent"
    );
    ensure!(
        observed.draw_events.iter().all(|event| !event.replay
            && event.radius == strokes[0].radius
            && event.position.is_finite()),
        "live water feedback has incorrect replay flag, radius or position: {:?}",
        observed.draw_events
    );
    ensure!(
        observed.recalc_count >= 2,
        "water drag/end did not produce terrain recalculation"
    );
    ensure!(
        observed.border_count == 0 && observed.feedback_count == 0,
        "in-border water stroke emitted out-of-border feedback"
    );
    eprintln!(
        "water input feedback: draws={} recalcs={} border={} border_feedback={} pending_before={} pending_after={}",
        observed.draw_events.len(),
        observed.recalc_count,
        observed.border_count,
        observed.feedback_count,
        pending_before,
        world
            .resource::<ScheduledTerrainReadback>()
            .pending
            .load(Ordering::SeqCst)
    );

    let expected_stroke = strokes[0].as_ref().clone();
    let after = world.resource::<WaterRaster>().composite().to_vec();
    let committed_cursor = history.ledger().cursor;
    let folder = world.resource::<ActiveSession>().id.save_folder_path();
    // Probe a small viewport-relative grid, nearest the actual water canvas
    // first. This is a bounded sample, not an exhaustive visibility claim.
    // Render each probe so the real per-update uploads can be submitted and
    // completed instead of accumulating through thousands of App-only ticks.
    let mut outside = None;
    let mut hover_candidates = 0usize;
    let canvas = WidgetId::clearing_canvas("watereraser_water_");
    let mut probes = Vec::new();
    for y in [0.1_f32, 0.3, 0.5, 0.7, 0.9] {
        for x in [0.1_f32, 0.3, 0.5, 0.7, 0.9] {
            probes.push(glam::Vec2::new(
                window.width as f32 * x,
                window.height as f32 * y,
            ));
        }
    }
    probes.sort_by(|a, b| {
        a.distance_squared(target)
            .total_cmp(&b.distance_squared(target))
    });
    render(&mut driver, &mut bridge)?;
    'search: for p in probes.iter().copied() {
        for _ in 0..2 {
            tick(&mut driver, &context, p, false);
        }
        render(&mut driver, &mut bridge)?;
        let world = driver.app().world();
        let cursor = world.resource::<CursorTerrainRaycast>();
        let brush = world
            .resource::<country_core::resources::brush_sizes::BrushSizes>()
            .water;
        let inside = world
            .resource::<country_core::resources::glade_border::GladeBorder>()
            .is_shape_inside(country_core::resources::glade_border::BorderShape::Circle(
                utils::geometry::Circle2d::new(
                    glam::Vec2::new(cursor.pos.x, cursor.pos.z),
                    0.8 * brush,
                ),
            ));
        if cursor.tag == 1
            && !inside
            && world.resource::<UiState>().interaction().native_tag == 1
            && world.resource::<UiState>().active_widget() == canvas
            && (world.resource::<CursorPositionSS>().0 - p).length() < 1.0
        {
            hover_candidates += 1;
            let mut hover = WaterObservation::new(world);
            tick(&mut driver, &context, p, false);
            hover.sample(driver.app().world());
            ensure!(
                hover.border_count > 0,
                "outside water hover did not display border at {p:?}; interaction={:?}",
                driver.app().world().resource::<UiState>().interaction()
            );

            let mut pressed = WaterObservation::new(driver.app().world());
            let mut phases = Vec::new();
            let mut captured_outside = false;
            for _ in 0..3 {
                tick(&mut driver, &context, p, true);
                pressed.sample(driver.app().world());
                let world = driver.app().world();
                let ui = world.resource::<UiState>();
                let cursor = world.resource::<CursorTerrainRaycast>();
                let brush = world
                    .resource::<country_core::resources::brush_sizes::BrushSizes>()
                    .water;
                let inside = world
                    .resource::<country_core::resources::glade_border::GladeBorder>()
                    .is_shape_inside(country_core::resources::glade_border::BorderShape::Circle(
                        utils::geometry::Circle2d::new(
                            glam::Vec2::new(cursor.pos.x, cursor.pos.z),
                            0.8 * brush,
                        ),
                    ));
                let phase = ui.widget_context(canvas).dragged();
                captured_outside |= cursor.tag == 1
                    && !inside
                    && ui.interaction().native_tag >= 2
                    && ui.active_widget() == canvas
                    && phase != 2;
                phases.push((
                    ui.interaction(),
                    phase,
                    cursor.pos,
                    inside,
                    pressed.border_count,
                    pressed.feedback_count,
                ));
            }
            tick(&mut driver, &context, p, false);
            pressed.sample(driver.app().world());
            render(&mut driver, &mut bridge)?;
            ensure!(
                pressed.draw_events.is_empty(),
                "outside water press emitted a draw event at {p:?}: phases={phases:?}"
            );
            if captured_outside {
                ensure!(
                    pressed.border_count > 0,
                    "captured outside water press did not display border at {p:?}: phases={phases:?}"
                );
                ensure!(
                    pressed.feedback_count > 0,
                    "captured outside water press did not produce input feedback at {p:?}: phases={phases:?}"
                );
                outside = Some(p);
                eprintln!(
                    "water outside border exercised at {p:?}: displays={} feedback={}",
                    pressed.border_count, pressed.feedback_count
                );
                break 'search;
            }
        }
    }
    if outside.is_none() {
        eprintln!(
            "OPEN: no out-of-border point captured by the water canvas with a non-None drag phase in {} sampled viewport points; hover candidates={hover_candidates}; unprobed points remain",
            probes.len()
        );
    }
    for _ in 0..3 {
        tick(&mut driver, &context, target, false);
    }
    let world = driver.app().world();
    ensure!(
        *world.resource::<AppMode>() == AppMode::Water,
        "outside search changed the selected tool"
    );
    ensure!(
        !world.resource::<History>().is_recording()
            && world.resource::<History>().ledger().cursor == committed_cursor
            && world.resource::<History>().ledger().entries.len() == count + 1
            && world.resource::<WaterRaster>().composite() == after.as_slice(),
        "outside search changed the committed water raster or history before undo"
    );
    replay_with_key(
        &mut driver,
        &context,
        &mut bridge,
        target,
        21,
        cursor,
        "Water undo",
    )?;
    ensure!(
        driver.app().world().resource::<WaterRaster>().composite() == before.as_slice(),
        "Water undo did not restore the complete pre-stroke raster"
    );
    replay_with_key(
        &mut driver,
        &context,
        &mut bridge,
        target,
        22,
        committed_cursor,
        "Water redo",
    )?;
    ensure!(
        driver.app().world().resource::<WaterRaster>().composite() == after.as_slice(),
        "Water redo did not restore the complete committed raster"
    );

    // Request the real asynchronous save worker and decode the on-disk History
    // section with the production schema.
    use country_core::systems::save::{
        load_game::HistorySectionJson,
        save_game::{SaveGameCmd, SavingGameComplete},
    };
    let mut completions = driver
        .app()
        .world()
        .resource::<Events<SavingGameComplete>>()
        .get_cursor();
    completions
        .read(
            driver
                .app()
                .world()
                .resource::<Events<SavingGameComplete>>(),
        )
        .for_each(drop);
    driver.app_mut().world_mut().send_event(SaveGameCmd);
    let mut saved = false;
    for _ in 0..600 {
        tick(&mut driver, &context, target, false);
        render(&mut driver, &mut bridge)?;
        if completions
            .read(
                driver
                    .app()
                    .world()
                    .resource::<Events<SavingGameComplete>>(),
            )
            .any(|event| event.path == folder)
        {
            saved = true;
            break;
        }
    }
    ensure!(
        saved,
        "Water production save did not complete for {}",
        folder.display()
    );
    let saved: serde_json::Value =
        serde_json::from_slice(&std::fs::read(folder.join("history.json"))?)?;
    let section: HistorySectionJson = serde_json::from_value(
        saved
            .get("History")
            .cloned()
            .context("saved game lacks History section")?,
    )?;
    ensure!(
        section.chain_index == committed_cursor && section.edit_chain.len() == count + 1,
        "saved History cursor or entry count differs from the committed Water history"
    );
    let saved_entry = section
        .edit_chain
        .last()
        .context("saved Water entry missing")?;
    let saved_strokes: Vec<_> = saved_entry
        .0
        .edits
        .iter()
        .filter_map(|edit| match edit {
            EditType::Water(stroke) => Some(stroke.as_ref()),
            _ => None,
        })
        .collect();
    ensure!(
        saved_strokes.len() == 1 && *saved_strokes[0] == expected_stroke,
        "saved History did not preserve the committed Water stroke"
    );

    Ok(())
}
