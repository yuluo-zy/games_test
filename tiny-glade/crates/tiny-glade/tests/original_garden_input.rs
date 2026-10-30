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
        world_raster_defs::GardenRaster,
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
    use country_core::systems::render_loop::draw_passes::{
        draw_callbacks_b::{env7, env26},
        draw_lists::{DrawListFn, DrawLists, list},
    };
    let world = driver.app_mut().world_mut();
    let mut lists = DrawLists::new();
    let ready = tiny_glade::production_renderer::prepare_headless_gameplay_frame_with_lists(
        bridge, world, &mut lists,
    )?;
    let count = |index, callback: DrawListFn| {
        lists
            .list(index)
            .iter()
            .filter(|entry| std::ptr::fn_addr_eq(entry.1, callback))
            .count()
    };
    let flower = count(list::GBUFFER, env7 as DrawListFn);
    let stone = count(list::GBUFFER, env26 as DrawListFn);
    ensure!(
        flower > 0 && stone > 0,
        "Garden draw owners absent from production G-buffer: flower={flower}, stone={stone}"
    );
    ensure!(
        count(list::SHADOW, env7 as DrawListFn) == 0
            && count(list::SHADOW, env26 as DrawListFn) > 0,
        "Garden draw owners have incorrect shadow-list membership"
    );
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

/// Inject the configured physical key through the same owner as the window
/// event path, then let the real mode manager and replay systems finish.
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
        systems::{
            garden::GardenDrawEvent,
            history::replay_raster::{RasterDomain, RasterReplayFeedback},
        },
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
    let mut draws = world.resource::<Events<GardenDrawEvent>>().get_cursor();
    samples
        .read(world.resource::<Events<RasterReplayFeedback>>())
        .for_each(drop);
    draws
        .read(world.resource::<Events<GardenDrawEvent>>())
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
            .filter(|event| event.domain == RasterDomain::Garden && event.replay)
            .count();
        draw_count += draws
            .read(world.resource::<Events<GardenDrawEvent>>())
            .filter(|event| event.observed_flag)
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
                sample_count > 0 && draw_count > 0,
                "{stage} completed without Garden replay feedback: samples={sample_count}, draw events={draw_count}"
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

fn copy_session_tree(from: &std::path::Path, to: &std::path::Path) -> Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_session_tree(&entry.path(), &target)?;
        } else {
            std::fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

// Inspect the same entity/VAO/shader query used by the production renderer.
fn garden_draw_owners(
    driver: &mut tiny_glade::game_loop::GameFrameDriver,
    shaders: &country_core::asset_libraries::shader_library::AssetShaderLibrary,
) -> Result<Vec<bevy_ecs::entity::Entity>> {
    use country_core::systems::render_loop::draw_passes::draw_lists::{
        DrawLists, build_draw_lists, list,
    };
    let mut lists = DrawLists::new();
    build_draw_lists(driver.app_mut().world_mut(), shaders, &mut lists);
    let entries = lists.list(list::GBUFFER);
    let mut owners = Vec::new();
    for name in ["solid_vertex_color_garden_stone", "garden_lowpoly_flower"] {
        let matching: Vec<_> = entries
            .iter()
            .filter(|entry| entry.0.params.shader_name == name)
            .map(|entry| entry.0.params.entity)
            .collect();
        ensure!(
            !matching.is_empty(),
            "Garden drawable {name} did not enter the G-buffer"
        );
        owners.extend(matching);
    }
    Ok(owners)
}

/// Observe the actual preset through the same resolve/transfer path used by
/// original_scene_compute_gpu; the production texture's usage is unchanged.
fn read_garden_mask(
    driver: &mut tiny_glade::game_loop::GameFrameDriver,
    bridge: &mut CoreRenderBridge,
    shaders: &country_core::asset_libraries::shader_library::AssetShaderLibrary,
    device: &std::sync::Arc<rhapsody::device::Device>,
) -> Result<Vec<u8>> {
    use ash::vk::{self, Handle};
    use country_core::{
        asset_libraries::texture_library::AssetTextureLibrary,
        render::pass_builder::{draw_stream::PassHost, framebuffer_binding::AttachmentTexture},
        resources::texture_presets::TexturePreset,
        systems::render_loop::draw_passes::{
            d_main_support::{DrawPassCtx, ShaderLookup},
            final_post,
        },
    };
    struct Shaders<'a>(&'a country_core::asset_libraries::shader_library::AssetShaderLibrary);
    impl ShaderLookup for Shaders<'_> {
        fn raster_handle_by_name(
            &self,
            name: &str,
        ) -> Option<
            country_core::rhapsody_contract::Handle<
                country_core::render::shader::RasterShaderProgram,
            >,
        > {
            self.0.get_raster_handle_by_name(name).map(|handle| {
                country_core::rhapsody_contract::Handle(
                    country_core::rhapsody_contract::ManagedHandle(handle.id().raw()),
                    std::marker::PhantomData,
                )
            })
        }
    }
    let world = driver.app_mut().world_mut();
    let (output, graph) = bridge.build_frame(|frame| {
        tiny_glade::frame_setup::with_frame_resources(frame, world, |frame, world| {
            let source = world
                .resource::<AssetTextureLibrary>()
                .get_rg_texture(TexturePreset(0x14));
            let [width, height] = source.desc.extent_2d();
            let target = std::sync::Arc::new(rhapsody::resource::Texture::new(
                device.clone(),
                &rhapsody::resource::TextureDesc::new_2d(width, height, vk::Format::R8G8B8A8_UNORM)
                    .usage(
                        vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
                    ),
            )?);
            let output = std::sync::Arc::new(rhapsody::resource::Buffer::new(
                device.clone(),
                &rhapsody::resource::BufferDesc::new_gpu_to_cpu(
                    u64::from(width * height * 4),
                    vk::BufferUsageFlags::TRANSFER_DST,
                ),
            )?);
            let target_id = frame.import_native_texture(target.clone(), 0)?;
            let output_id = frame.import_native_buffer(output.clone(), 0)?;
            frame.with_raster(|host| {
                let mut factory = || -> Box<dyn PassHost> { Box::new(host.sibling()) };
                let lookup = Shaders(shaders);
                let mut ctx =
                    DrawPassCtx::new(world, std::time::Duration::ZERO, &mut factory, &lookup);
                final_post::resolve_to_swapchain(
                    &mut ctx,
                    &AttachmentTexture {
                        id: source.resource as u32,
                        extent: (width, height),
                        format: source.desc.format,
                        sample_count: 1,
                    },
                    &AttachmentTexture {
                        id: target_id.raw(),
                        extent: (width, height),
                        format: 37,
                        sample_count: 1,
                    },
                    0,
                );
                Ok(())
            })?;
            frame.with_graph(|graph| {
                let device = device.clone();
                let buffer = output.clone();
                let mut pass = graph.add_pass("garden reload mask readback");
                pass.add_resource_access(target_id, 0x25, 0)?;
                pass.add_resource_access(output_id, 0x34, 0)?;
                pass.render_with_result_context(move |ctx| {
                    unsafe {
                        device.raw_device().cmd_copy_image_to_buffer(
                            vk::CommandBuffer::from_raw(ctx.command.handle()),
                            target.raw_handle(),
                            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                            buffer.raw_handle(),
                            &[vk::BufferImageCopy::default()
                                .image_subresource(
                                    vk::ImageSubresourceLayers::default()
                                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                                        .layer_count(1),
                                )
                                .image_extent(vk::Extent3D {
                                    width,
                                    height,
                                    depth: 1,
                                })],
                        );
                    }
                    Ok(())
                });
                graph.export(output_id, 0x26)?;
                Ok(())
            })?;
            Ok((output, width, height))
        })
    })?;
    bridge.execute_test_graph(graph)?;
    bridge.wait_submits_done(120_000_000_000)?;
    let (buffer, width, height) = output;
    let mut bytes = vec![0; (width * height * 4) as usize];
    buffer.read_bytes(0, &mut bytes)?;
    Ok(bytes)
}

fn garden_history(
    entries: &[country_core::resources::history::transactions::HistoryEntry<EditType>],
) -> Vec<(
    usize,
    usize,
    country_core::resources::raster_stroke::RasterStroke,
)> {
    entries
        .iter()
        .enumerate()
        .flat_map(|(entry_index, entry)| {
            entry
                .edits
                .iter()
                .enumerate()
                .filter_map(move |(edit_index, edit)| match edit {
                    EditType::Garden(stroke) => {
                        Some((entry_index, edit_index, stroke.as_ref().clone()))
                    }
                    _ => None,
                })
        })
        .collect()
}

fn history_wire(
    entries: &[country_core::resources::history::transactions::HistoryEntry<EditType>],
) -> Result<Vec<serde_json::Value>> {
    use country_core::resources::history::serialized::SavedMultiEdit;
    entries
        .iter()
        .cloned()
        .map(|entry| {
            let mut value = serde_json::to_value(SavedMultiEdit(entry))?;
            // This one map is represented as unordered key/value rows; edit and
            // all other array ordering remains significant.
            if let Some(rows) = value
                .pointer_mut("/decorators/diffs")
                .and_then(|v| v.as_array_mut())
            {
                rows.sort_by_cached_key(|row| row[0].to_string());
            }
            Ok(value)
        })
        .collect()
}

fn recover_garden_in_child(
    driver: &mut tiny_glade::game_loop::GameFrameDriver,
    context: &egui::Context,
    bridge: &mut CoreRenderBridge,
    shaders: &country_core::asset_libraries::shader_library::AssetShaderLibrary,
    device: &std::sync::Arc<rhapsody::device::Device>,
) -> Result<()> {
    use country_core::{
        app_loading_state::AppLoadingState,
        meta_commander::{MetaCommand, MetaCommander},
        resources::session::SessionId,
        systems::save::load_game::HistorySectionJson,
    };
    let session: SessionId = serde_json::from_str(&std::env::var("TG_GARDEN_CHILD_SESSION")?)?;
    let source = std::path::PathBuf::from(
        std::env::var_os("TG_GARDEN_CHILD_SAVE").context("Garden child source session missing")?,
    );
    let expected_pixels = std::fs::read(
        std::env::var_os("TG_GARDEN_CHILD_RASTER")
            .context("Garden child raster baseline missing")?,
    )?;
    let saved: serde_json::Value =
        serde_json::from_slice(&std::fs::read(source.join("history.json"))?)?;
    let section: HistorySectionJson = serde_json::from_value(
        saved
            .get("History")
            .cloned()
            .context("parent save lacks History section")?,
    )?;
    let expected_entries: Vec<_> = section
        .edit_chain
        .iter()
        .map(|edit| edit.0.clone())
        .collect();
    let expected_garden = garden_history(&expected_entries);
    ensure!(
        !expected_garden.is_empty(),
        "saved session has no Garden edits"
    );
    let old_owners = garden_draw_owners(driver, shaders)?;

    // Child paths were initialized to a distinct tempfile account before App
    // construction. Copy the actual saved files, including any snapshots.
    copy_session_tree(&source, &session.save_folder_path())?;
    driver
        .app_mut()
        .world_mut()
        .resource_mut::<MetaCommander>()
        .extend_script([MetaCommand::load_session(session, false)]);
    let mut saw_reload = false;
    let mut saw_owner_cleanup = false;
    let mut settled = 0;
    for _ in 0..1200 {
        tick(driver, context, glam::Vec2::ZERO, false);
        bridge.prepare_test_execution()?;
        let world = driver.app().world();
        let state = *world
            .resource::<bevy_state::prelude::State<AppLoadingState>>()
            .get();
        saw_reload |= state == AppLoadingState::Reload;
        if state == AppLoadingState::Reload
            && old_owners
                .iter()
                .all(|entity| world.get_entity(*entity).is_err())
        {
            saw_owner_cleanup = true;
        }
        let ready = saw_reload
            && state == AppLoadingState::Game
            && world
                .get_resource::<ActiveSession>()
                .is_some_and(|active| active.id == session)
            && world.resource::<LoadGameManager>().options.is_none()
            && world.resource::<ReplayBuffer>().is_empty()
            && world.resource::<PendingHistoryCommands>().0.is_empty()
            && world.resource::<AppMode>().native_tag() != 13
            && !world.resource::<History>().is_recording();
        settled = if ready { settled + 1 } else { 0 };
        if settled >= 3 {
            break;
        }
    }
    ensure!(
        settled >= 3 && saw_owner_cleanup,
        "Garden child production Reload did not settle or clear old owners (saw_reload={saw_reload}, cleanup={saw_owner_cleanup})"
    );
    for _ in 0..4 {
        tick(driver, context, glam::Vec2::ZERO, false);
        render(driver, bridge)?;
    }
    let new_owners = garden_draw_owners(driver, shaders)?;
    ensure!(
        old_owners
            .iter()
            .all(|entity| driver.app().world().get_entity(*entity).is_err()),
        "Garden drawable from the previous session survived Reload"
    );
    ensure!(
        new_owners.iter().all(|entity| !old_owners.contains(entity)),
        "Garden Reload reused an old drawable entity"
    );
    let world = driver.app().world();
    ensure!(
        *world
            .resource::<bevy_state::prelude::State<AppLoadingState>>()
            .get()
            == AppLoadingState::Game
            && world.resource::<LoadGameManager>().options.is_none()
            && world.resource::<ReplayBuffer>().is_empty()
            && world.resource::<PendingHistoryCommands>().0.is_empty()
            && world.resource::<AppMode>().native_tag() != 13
            && !world.resource::<History>().is_recording(),
        "Garden child became unsettled during post-load rendering"
    );
    let ledger = world.resource::<History>().ledger();
    ensure!(
        ledger.cursor == section.chain_index
            && ledger.undo_anchor == section.history_start
            && ledger.entries.len() == expected_entries.len(),
        "reloaded History differs: cursor={} expected={}, start={} expected={}, entries={} expected={}",
        ledger.cursor,
        section.chain_index,
        ledger.undo_anchor,
        section.history_start,
        ledger.entries.len(),
        expected_entries.len()
    );
    ensure!(
        garden_history(&ledger.entries) == expected_garden,
        "reloaded Garden edit sequence differs from the saved History section"
    );
    ensure!(
        history_wire(&ledger.entries)? == history_wire(&expected_entries)?,
        "reloaded full History edit sequence differs from the saved section"
    );
    let actual = world.resource::<GardenRaster>().composite();
    ensure!(
        actual == expected_pixels.as_slice(),
        "reloaded Garden raster differs: expected {} bytes, actual {} bytes, mismatched pixels {}",
        expected_pixels.len(),
        actual.len(),
        actual
            .iter()
            .zip(&expected_pixels)
            .filter(|(a, b)| a != b)
            .count()
    );
    let expected_mask = std::fs::read(
        std::env::var_os("TG_GARDEN_CHILD_MASK")
            .context("Garden child GPU mask baseline missing")?,
    )?;
    let actual_mask = read_garden_mask(driver, bridge, shaders, device)?;
    // The garden upload and blur own red and green. Other channels are not
    // written by this business path and are not a stable recovery oracle.
    ensure!(
        actual_mask.len() == expected_mask.len() && actual_mask.len() % 4 == 0,
        "Garden GPU mask readback dimensions differ"
    );
    ensure!(
        expected_mask.chunks_exact(4).any(|pixel| pixel[0] != 0),
        "parent Garden GPU mask is empty"
    );
    let mismatches = actual_mask
        .chunks_exact(4)
        .zip(expected_mask.chunks_exact(4))
        .filter(|(a, b)| a[0..2] != b[0..2])
        .count();
    ensure!(
        mismatches == 0,
        "reloaded Garden GPU mask red/green differs at {mismatches} pixels"
    );
    Ok(())
}

#[test]
fn shipped_app_garden_pointer_stroke_records_authoritative_raster_and_history() -> Result<()> {
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
    let cli = CliArgs::parse_from(["original-garden-input"]);
    let config = get_app_config(&cli)?;
    ensure!(
        !config.dev.demo,
        "garden input fixture requires the full release"
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
        device.clone(),
        bridge.hosts().clone(),
        installed.clone(),
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
    if std::env::var_os("TG_GARDEN_CHILD_SESSION").is_some() {
        return recover_garden_in_child(&mut driver, &context, &mut bridge, &shaders, &device);
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
        .find(|(_, b)| b.category == ToolbarCategory::Garden)
        .map(|(entity, _)| entity)
        .context("Garden category button missing")?;
    // Layout and picker consume different frames; follow the moving button
    // only after it is visible, on screen and has nonzero bounds.
    for _ in 0..48 {
        let world = driver.app().world();
        let extent = world
            .get::<Ui2dExtent>(button)
            .context("Garden button extent missing")?;
        let state = world
            .get::<UiInteractionState>(button)
            .context("Garden button interaction missing")?;
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
            .context("Garden button Taffy node missing")?
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
        .context("Garden button Taffy node missing")?
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
        "Garden button was not visible and picked: extent={extent:?}, state={:?}, welcome={:?}, cursor={:?}, active={:?}, reveal={reveal:?}",
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
        *driver.app().world().resource::<AppMode>() == AppMode::Garden,
        "toolbar click did not select Garden"
    );
    ensure!(
        driver
            .app()
            .world()
            .resource::<country_core::resources::toolbar::Toolbar>()
            .current
            == ToolbarCategory::Garden
            && driver
                .app()
                .world()
                .get::<UiInteractionState>(button)
                .is_some_and(|state| state.selected == 1),
        "Garden toolbar category/button did not become selected"
    );

    // Search actual screen pixels using the real picker and terrain ray system.
    // Require a live Garden canvas and an in-border terrain hit before pressing.
    let mut target = None;
    for y in (35..225).step_by(12) {
        for x in (35..445).step_by(12) {
            let p = glam::Vec2::new(x as f32, y as f32);
            for _ in 0..2 {
                tick(&mut driver, &context, p, false);
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
                .garden;
            let inside = world
                .resource::<country_core::resources::glade_border::GladeBorder>()
                .is_shape_inside(country_core::resources::glade_border::BorderShape::Circle(
                    utils::geometry::Circle2d::new(
                        glam::Vec2::new(cursor.pos.x, cursor.pos.z),
                        (0.4 * brush) * 0.8,
                    ),
                ));
            if hit.is_some()
                && cursor.tag == 1
                && inside
                && world.resource::<UiState>().active_widget()
                    == WidgetId::clearing_canvas("garden")
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
    let target = target.context("no live Garden canvas with in-border terrain hit")?;
    render(&mut driver, &mut bridge)?;
    let world = driver.app().world();
    ensure!(
        world.resource::<ReplayBuffer>().is_empty()
            && world.resource::<PendingHistoryCommands>().0.is_empty(),
        "intro replay still pending"
    );
    let before = world.resource::<GardenRaster>().composite().to_vec();
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
    tick(&mut driver, &context, target, true);
    for _ in 0..5 {
        tick(
            &mut driver,
            &context,
            target + glam::Vec2::new(3.0, 0.0),
            true,
        );
    }
    tick(
        &mut driver,
        &context,
        target + glam::Vec2::new(3.0, 0.0),
        false,
    );
    for _ in 0..30 {
        tick(&mut driver, &context, target, false);
        let history = driver.app().world().resource::<History>();
        if !history.is_recording() && history.ledger().entries.len() > count {
            break;
        }
    }
    let world = driver.app().world();
    let history = world.resource::<History>();
    ensure!(!history.is_recording(), "Garden transaction did not finish");
    ensure!(
        history.ledger().cursor == cursor + 1 && history.ledger().entries.len() == count + 1,
        "Garden stroke did not commit exactly one history entry"
    );
    let entry = history
        .ledger()
        .entries
        .last()
        .context("missing committed Garden edit")?;
    let strokes: Vec<_> = entry
        .edits
        .iter()
        .filter_map(|edit| match edit {
            EditType::Garden(stroke) => Some(stroke),
            _ => None,
        })
        .collect();
    ensure!(
        strokes.len() == 1 && !strokes[0].points.is_empty() && strokes[0].draw,
        "committed entry lacks its actual additive Garden stroke"
    );
    ensure!(
        strokes[0].flags == 1
            && strokes[0].strength == 0.5
            && strokes[0].radius
                == world
                    .resource::<country_core::resources::brush_sizes::BrushSizes>()
                    .garden,
        "committed Garden stroke lost its live brush parameters"
    );
    ensure!(
        world.resource::<GardenRaster>().composite() != before.as_slice(),
        "Garden raster did not change"
    );
    let expected_stroke = strokes[0].as_ref().clone();
    let after = world.resource::<GardenRaster>().composite().to_vec();
    let committed_cursor = history.ledger().cursor;
    let session = world.resource::<ActiveSession>().id;
    let folder = session.save_folder_path();
    render(&mut driver, &mut bridge)?;

    // Binding slots 21/22 map through translate_input to the original
    // undo/redo actions. No HistoryCommand or cursor is injected here.
    replay_with_key(
        &mut driver,
        &context,
        &mut bridge,
        target,
        21,
        cursor,
        "Garden undo",
    )?;
    ensure!(
        driver.app().world().resource::<GardenRaster>().composite() == before.as_slice(),
        "Garden undo did not restore the complete pre-stroke raster"
    );
    replay_with_key(
        &mut driver,
        &context,
        &mut bridge,
        target,
        22,
        committed_cursor,
        "Garden redo",
    )?;
    ensure!(
        driver.app().world().resource::<GardenRaster>().composite() == after.as_slice(),
        "Garden redo did not restore the complete committed raster"
    );

    // Request the real asynchronous save worker and decode the on-disk History
    // section with the production schema.
    use bevy_ecs::event::Events;
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
        "Garden production save did not complete for {}",
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
        "saved History cursor or entry count differs from the committed Garden history"
    );
    let saved_entry = section
        .edit_chain
        .last()
        .context("saved Garden entry missing")?;
    let saved_strokes: Vec<_> = saved_entry
        .0
        .edits
        .iter()
        .filter_map(|edit| match edit {
            EditType::Garden(stroke) => Some(stroke.as_ref()),
            _ => None,
        })
        .collect();
    ensure!(
        saved_strokes.len() == 1 && *saved_strokes[0] == expected_stroke,
        "saved History did not preserve the committed Garden stroke"
    );
    let recovery = tempfile::tempdir()?;
    let baseline = recovery.path().join("garden-raster.bin");
    std::fs::write(&baseline, &after)?;
    let mask = read_garden_mask(&mut driver, &mut bridge, &shaders, &device)?;
    let mask_baseline = recovery.path().join("garden-mask.rgba");
    std::fs::write(&mask_baseline, &mask)?;
    bridge.wait_submits_done(120_000_000_000)?;
    drop(driver);
    drop(bridge);
    drop(shaders);
    drop(device);
    let status = std::process::Command::new(std::env::current_exe()?)
        .args([
            "--exact",
            "shipped_app_garden_pointer_stroke_records_authoritative_raster_and_history",
            "--nocapture",
            "--test-threads=1",
        ])
        .env("TG_INSTALL_ROOT", &installed)
        .env("TG_GARDEN_CHILD_SESSION", serde_json::to_string(&session)?)
        .env("TG_GARDEN_CHILD_SAVE", &folder)
        .env("TG_GARDEN_CHILD_RASTER", &baseline)
        .env("TG_GARDEN_CHILD_MASK", &mask_baseline)
        .status()?;
    ensure!(
        status.success(),
        "Garden new-process production reload failed: {status}"
    );
    Ok(())
}
