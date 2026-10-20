//! Independent lifecycle acceptance against the complete production App/device
//! installer. No systems are replaced, removed, or repaired by this test.
use anyhow::{Context, Result};
use bevy_ecs::{event::EventCursor, prelude::*};
use clap::Parser;
use country_core::{
    app_loading_state::AppLoadingState,
    asset_libraries::placement_data_library::AssetPlacementDataLibrary,
    components::KeepOnLevelUnload,
    resources::{
        image_formation::{ImageFormationConfig, ImageFormationConfigs},
        session::{
            ActiveSession, GladeType, NewSessionStartedCmd, NextSession, SessionId,
            SessionLoadedCmd, SessionMeta,
        },
        time::GameTime,
        window::WindowSize,
    },
    systems::{
        mode_manager::{AppMode, AppModeChanged, ClearCanvasCmd},
        save::load_game::{LoadGameManager, LoadGameState, LoadingGameComplete},
    },
};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    time::Duration,
};
use tiny_glade::{
    app_config::{CliArgs, get_app_config},
    render_bridge::CoreRenderBridge,
    run_game::{ApplicationShaderLibrary, run_game_thread},
};

#[derive(Component)]
struct MustSurvive;
#[derive(Component)]
struct MustUnload;

#[derive(Default)]
struct Seen {
    new: EventCursor<NewSessionStartedCmd>,
    legacy: EventCursor<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,
    sessions: EventCursor<SessionLoadedCmd>,
    complete: EventCursor<LoadingGameComplete>,
    clear: EventCursor<ClearCanvasCmd>,
    session_count: usize,
    complete_count: usize,
    clear_false: usize,
    new_count: usize,
    legacy_count: usize,
}
fn step(driver: &mut tiny_glade::game_loop::GameFrameDriver, ui: &egui::Context, seen: &mut Seen) {
    ui.begin_frame(egui::RawInput::default());
    driver.app_mut().update();
    let _ = ui.end_frame();
    let world = driver.app().world();
    seen.new_count += seen
        .new
        .read(world.resource::<Events<NewSessionStartedCmd>>())
        .count();
    seen.legacy_count += seen.legacy.read(world.resource::<Events<country_core::resources::history::tree_edit::SpawnPreplacedTrees>>()).count();
    seen.session_count += seen
        .sessions
        .read(world.resource::<Events<SessionLoadedCmd>>())
        .count();
    seen.complete_count += seen
        .complete
        .read(world.resource::<Events<LoadingGameComplete>>())
        .count();
    seen.clear_false += seen
        .clear
        .read(world.resource::<Events<ClearCanvasCmd>>())
        .filter(|e| !e.0)
        .count();
}
fn load_until_complete(
    driver: &mut tiny_glade::game_loop::GameFrameDriver,
    ui: &egui::Context,
    seen: &mut Seen,
    expected: Option<SessionId>,
    previous_complete: usize,
) -> Result<()> {
    for _ in 0..900 {
        step(driver, ui, seen);
        let world = driver.app().world();
        let load = world.resource::<LoadGameManager>();
        anyhow::ensure!(
            load.state != LoadGameState::Failed,
            "actual load state failed, not completed"
        );
        if world
            .get_resource::<ActiveSession>()
            .is_some_and(|s| expected.is_none_or(|id| s.id == id))
            && load.state == LoadGameState::Loaded
            && load.options.is_none()
            && seen.complete_count == previous_complete + 1
            && *world.resource::<AppMode>() == AppMode::default()
        {
            anyhow::ensure!(
                !world.contains_resource::<NextSession>(),
                "request owner not consumed"
            );
            anyhow::ensure!(
                world
                    .resource::<country_core::resources::walls::InnerWalls>()
                    .wall_ids()
                    .next()
                    .is_some(),
                "complete event without actual loaded wall state"
            );
            return Ok(());
        }
    }
    anyhow::bail!("actual session {expected:?} never completed exactly once");
}

fn stable_frames(
    driver: &mut tiny_glade::game_loop::GameFrameDriver,
    ui: &egui::Context,
    seen: &mut Seen,
) {
    let old = *driver.app().world().resource::<GameTime>();
    let counts = (
        seen.session_count,
        seen.new_count,
        seen.complete_count,
        seen.legacy_count,
    );
    *driver
        .app_mut()
        .world_mut()
        .resource_mut::<country_core::resources::gui::EscMenuState>() =
        country_core::resources::gui::EscMenuState::Closed;
    for _ in 0..90 {
        step(driver, ui, seen);
        assert_eq!(
            *driver.app().world().resource::<AppMode>(),
            AppMode::default()
        );
        assert_eq!(
            driver.app().world().resource::<LoadGameManager>().state,
            LoadGameState::Loaded
        );
    }
    let now = driver.app().world().resource::<GameTime>();
    assert!(now.frame_count > old.frame_count && now.total_secs > old.total_secs);
    assert_eq!(
        (
            seen.session_count,
            seen.new_count,
            seen.complete_count,
            seen.legacy_count
        ),
        counts,
        "idle main loop republished one-shot session events"
    );
}

fn inspect_original_intro_trees(world: &mut World, installed: &Path) -> Result<Vec<Entity>> {
    use country_core::{
        components::{ConvertedPreplacedTree, TreeComponent},
        resources::{prefab::PrefabInstance, rt::RtWorld},
        systems::prefab::PrefabInstanceData,
    };
    // Read shipped placement data independently of the production conversion
    // helper. Original140862270 skips only the literal clearing/tree/bush and
    // accepts strictly |x|,|z|<65 before assigning sequential IDs100001+.
    let root = installed.join("assets/glade/autumn");
    let groups: Vec<serde_json::Value> =
        serde_json::from_slice(&std::fs::read(root.join("trees.json"))?)?;
    let mut expected = Vec::new();
    for g in groups {
        let name = g["prefab"].as_str().unwrap();
        if name == "clearing/tree/bush" {
            continue;
        }
        let records: Vec<serde_json::Value> = serde_json::from_slice(&std::fs::read(
            root.join(g["placements"].as_str().unwrap()),
        )?)?;
        for p in records {
            let pos = p["position"].as_array().unwrap();
            if pos[0].as_f64().unwrap().abs() < 65.0 && pos[2].as_f64().unwrap().abs() < 65.0 {
                expected.push((
                    name.to_owned(),
                    [
                        pos[0].as_f64().unwrap() as f32,
                        pos[1].as_f64().unwrap() as f32,
                        pos[2].as_f64().unwrap() as f32,
                    ],
                ));
            }
        }
    }
    assert_eq!(expected.len(), 281, "original B autumn fixture changed");
    let storage = world.resource::<country_core::resources::tree_storage::TreeStorage>();
    let rt = world.resource::<RtWorld>();
    let mut entities = Vec::new();
    for (i, (name, position)) in expected.iter().enumerate() {
        let id = 100001 + i as u64;
        let entry = storage
            .get(country_core::resources::tree_storage::TreeStorageIndex(id))
            .context("legacySpawn did not populate real TreeStorage")?;
        let entity = world.entity(entry.entity);
        let tree = entity
            .get::<TreeComponent>()
            .context("legacy tree component missing")?;
        assert_eq!((tree.id, tree.is_preplaced), (id, true));
        assert_eq!(
            entity
                .get::<ConvertedPreplacedTree>()
                .context("conversion identity missing")?
                .0
                .as_str(),
            name
        );
        let placement = entity
            .get::<PrefabInstanceData>()
            .context("real48B placement missing")?;
        assert_eq!(placement.0.position.to_array(), *position);
        let contents = entity
            .get::<PrefabInstance>()
            .context("real88B owned prefab contents missing")?;
        assert!(
            !contents.gpu_instances.is_empty(),
            "legacySpawn stopped before actual GPU instances"
        );
        assert!(
            !contents.rt_meshes.is_empty(),
            "legacySpawn stopped before actual RT meshes"
        );
        for handle in &contents.rt_meshes {
            assert!(rt.get(*handle).is_some());
        }
        entities.push(entry.entity);
    }
    Ok(entities)
}
fn request(
    installed: &Path,
    theme: GladeType,
    override_image: bool,
) -> Result<(NextSession, SessionId, SessionMeta)> {
    // Use an unchanged original Version27 history, isolating this lifecycle
    // acceptance from unrelated old-version migration coverage.
    let source = installed.join("assets/starting-builds/full/03-halftimber-roof");
    let asset: serde_json::Value =
        serde_json::from_slice(&std::fs::read(source.join("history.json"))?)?;
    anyhow::ensure!(asset["Version"] == 27, "fixture version must stay explicit");
    let id = SessionId::new();
    let destination = id.save_folder_path();
    country_core::startup::startup_session::copy_dir_all(&source, &destination)?;
    let mut meta = SessionMeta {
        glade: theme,
        ..Default::default()
    };
    if override_image {
        let mut image = ImageFormationConfig::default();
        image.params_18 = [0.37, -0.2, 1.3, 0.8];
        image.params_30 = [0.04, 0.02];
        meta.image = Some(image);
    }
    meta.write_to_path(destination.join("meta.json"))?;
    let settings = country_core::resources::glade_settings::GladeSettings::load(meta.assets_root());
    let parsed = country_core::systems::save::load_game::parse_saved_game_data(
        &destination.join("history.json"),
        &settings,
    )?;
    Ok((NextSession::from_parsed(id, parsed), id, meta))
}
fn inspect_theme(
    world: &mut World,
    installed: &Path,
    meta: &SessionMeta,
) -> Result<(HashSet<u64>, Vec<Entity>)> {
    use country_core::systems::render_loop::draw_passes::draw_callbacks_b::InstancedDrawFromPlacementData;
    let root = installed.join(meta.assets_root());
    let plants: Vec<serde_json::Value> =
        serde_json::from_slice(&std::fs::read(root.join("plants.json"))?)?;
    let mut expected = Vec::new();
    let mut total = 0;
    for plant in &plants {
        let path = root.join(plant["placements"].as_str().unwrap());
        let records: Vec<country_core::resources::placement_data::PlacementDataSsbo> =
            serde_json::from_slice(&std::fs::read(&path)?)?;
        let library = world.resource::<AssetPlacementDataLibrary>();
        let handle = library.paths[&path];
        let (buffer, count) = &library.buffers[&handle];
        assert_eq!(*count, records.len());
        assert_eq!(buffer.buffer.byte_size, records.len() as u64 * 48);
        total += count;
        expected.push(handle.id().raw());
    }
    match meta.glade {
        GladeType::Flowery => assert_eq!((plants.len(), total), (1, 66)),
        GladeType::Summer => assert_eq!((plants.len(), total), (4, 197)),
        GladeType::Winter => assert_eq!(plants.len(), 0),
        _ => {}
    }
    let mut query = world.query::<(Entity, &InstancedDrawFromPlacementData)>();
    let entities: Vec<_> = query
        .iter(world)
        .filter(|(_, d)| expected.contains(&d.0))
        .map(|(e, _)| e)
        .collect();
    assert_eq!(
        entities.len(),
        expected.len(),
        "real per-placement draw entities must exist"
    );
    // Read the original asset rather than use load_session_grading as its own
    // reference implementation. Saved override must not contaminate slot0.
    let grading: serde_json::Value =
        serde_json::from_slice(&std::fs::read(root.join("grading.json"))?)?;
    let formation = world.resource::<ImageFormationConfigs>();
    let baseline = &formation.configs[0];
    assert_eq!(
        baseline.name,
        grading["current_grading_lut"].as_str().unwrap()
    );
    assert_eq!(
        baseline.params_18,
        ["grading_strength", "ev_shift", "contrast", "saturation"]
            .map(|k| grading[k].as_f64().unwrap() as f32)
    );
    assert_eq!(
        baseline.flag_28,
        u8::from(grading["bypass_color"].as_bool().unwrap())
    );
    for actual in &formation.configs[1..] {
        if let Some(saved) = &meta.image {
            assert_eq!(actual, saved);
        } else {
            assert_eq!(actual, baseline);
        }
    }
    let storage = world.resource::<country_core::resources::tree_storage::TreeStorage>();
    for (_, entry) in storage.iter() {
        assert!(
            world.entities().contains(entry.entity),
            "TreeStorage retained a dead prior-session entity"
        );
    }
    Ok((expected.into_iter().collect(), entities))
}

#[test]
fn first_nonempty_load_then_same_and_different_themes_use_actual_exit_enter() -> Result<()> {
    let installed = std::env::var_os("TG_INSTALL_ROOT")
        .map(PathBuf::from)
        .context("TG_INSTALL_ROOT must point to original B assets")?
        .canonicalize()?;
    let account = tempfile::tempdir()?;
    paths::initialize_paths_with_installation(0, Some(account.path()), &installed)?;
    std::env::set_current_dir(&installed)?;
    utils::local_time::init_local_time();
    tiny_glade::load_language_library::load_language_library(None, None);
    let cli = CliArgs::parse_from(["independent-startup-reload"]);
    let config = get_app_config(&cli)?;
    anyhow::ensure!(!config.dev.demo, "this test verifies all full-build themes");
    let mut app = run_game_thread(config, &cli, WindowSize::new(480, 270))?;
    app.insert_resource(
        country_core::resources::platform::RuntimePlatform::from_detection(false, false),
    );
    tiny_glade::game_loop::register_platform_events(&mut app);
    tiny_glade::game_loop::register_settings_application_system(&mut app);
    let device =
        rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig::headless())?;
    let shaders = app.world().resource::<ApplicationShaderLibrary>().0.clone();
    let bridge = CoreRenderBridge::new(device.clone(), installed.clone(), shaders.clone())?;
    let mut final_post = None;
    country_core::startup::load_shaders::load_shaders(
        &shaders,
        bridge.shader_backend(),
        &mut final_post,
    );
    bridge.install_asset_libraries(app.world_mut())?;
    tiny_glade::gameplay_startup::install_gameplay_device_runtime(
        &mut app,
        device,
        bridge.hosts().clone(),
        installed.clone(),
    )?;
    // Leave the real empty-account boot request in place: select/copy the
    // original v16 intro through normal startup, not a hand-built parsed input.
    let intro: serde_json::Value = serde_json::from_slice(&std::fs::read(
        installed.join("assets/starting-builds/full/intro/history.json"),
    )?)?;
    assert_eq!(intro["Version"], 16);
    let ui = egui::Context::default();
    let mut driver = tiny_glade::game_loop::GameFrameDriver::new(app, ui.clone());
    driver
        .app_mut()
        .insert_resource(bevy_time::TimeUpdateStrategy::ManualDuration(
            Duration::from_secs_f64(1.0 / 60.0),
        ));
    let mut seen = Seen::default();
    load_until_complete(&mut driver, &ui, &mut seen, None, 0)?;
    assert_eq!(
        (seen.new_count, seen.session_count, seen.legacy_count),
        (1, 0, 1)
    );
    stable_frames(&mut driver, &ui, &mut seen);
    let legacy_entities = inspect_original_intro_trees(driver.app_mut().world_mut(), &installed)?;
    let meta = driver
        .app()
        .world()
        .resource::<ActiveSession>()
        .meta
        .clone();
    assert_eq!(meta.glade, GladeType::Autumn);
    let (mut handles, mut entities) =
        inspect_theme(driver.app_mut().world_mut(), &installed, &meta)?;
    entities.extend(legacy_entities);
    let persistent = driver
        .app_mut()
        .world_mut()
        .spawn((KeepOnLevelUnload, MustSurvive))
        .id();
    for (pass, theme) in [
        GladeType::Flowery,
        GladeType::Flowery,
        GladeType::Summer,
        GladeType::Winter,
    ]
    .into_iter()
    .enumerate()
    {
        use country_core::{
            resources::photo_mode::PhotoModeCameraMode,
            startup::init_photomode_state::PhotomodeState, systems::camera_rig::MainCamera,
        };
        // A prior initialized free rig at a deliberately different position
        // exposes a missing per-session reset, unlike first-session defaults.
        {
            let world = driver.app_mut().world_mut();
            let mut camera = world.resource_mut::<MainCamera>();
            for (offset, value) in [(0x138, 500.0), (0x13c, 80.0), (0x140, -500.0)] {
                camera.write_f32(offset, value);
            }
            camera.match_free_camera_to_normal();
            let mut photo = world.resource_mut::<PhotomodeState>();
            photo.free_camera_initialized = true;
            photo.walk_camera_initialized = true;
            photo.camera_mode = PhotoModeCameraMode::Free;
            photo.hide_ui = true;
            photo.flag_08 = true;
        }
        let temporary = driver.app_mut().world_mut().spawn(MustUnload).id();
        let old_clock = *driver.app().world().resource::<GameTime>();
        let before_clear = seen.clear_false;
        let before_complete = seen.complete_count;
        let before_sessions = seen.session_count;
        let before_capacity = {
            let p = driver.app().world().resource::<AssetPlacementDataLibrary>();
            (p.paths.capacity(), p.buffers.capacity())
        };
        let (next, next_id, next_meta) = request(&installed, theme, pass == 0)?;
        driver.app_mut().world_mut().insert_resource(next);
        driver
            .app_mut()
            .world_mut()
            .resource_mut::<bevy_state::state::NextState<AppLoadingState>>()
            .set(AppLoadingState::Reload);
        step(&mut driver, &ui, &mut seen);
        let world = driver.app().world();
        assert_eq!(
            *world
                .resource::<bevy_state::state::State<AppLoadingState>>()
                .get(),
            AppLoadingState::Reload
        );
        assert_eq!(
            seen.clear_false,
            before_clear + 1,
            "the real OnExit must publish exactly one false ClearCanvas"
        );
        assert!(world.entities().contains(persistent));
        assert!(!world.entities().contains(temporary));
        for old in &entities {
            assert!(
                !world.entities().contains(*old),
                "old meadow entity survived OnExit"
            );
        }
        let placements = world.resource::<AssetPlacementDataLibrary>();
        assert!(
            placements.paths.is_empty() && placements.buffers.is_empty(),
            "OnExit must clear both placement owners"
        );
        assert_eq!(
            (placements.paths.capacity(), placements.buffers.capacity()),
            before_capacity,
            "clear replaced the retained placement allocations"
        );
        load_until_complete(&mut driver, &ui, &mut seen, Some(next_id), before_complete)?;
        assert_eq!(seen.session_count, before_sessions + 1);
        assert_eq!(
            seen.legacy_count, 1,
            "v27 reload replayed the prior v16 legacy event"
        );
        stable_frames(&mut driver, &ui, &mut seen);
        let (fresh, new_entities) =
            inspect_theme(driver.app_mut().world_mut(), &installed, &next_meta)?;
        assert!(
            handles.is_disjoint(&fresh),
            "a new session must receive fresh live placement handles"
        );
        let world = driver.app().world();
        let clock = world.resource::<GameTime>();
        assert!(
            clock.total_secs >= old_clock.total_secs && clock.frame_count >= old_clock.frame_count,
            "global clock was replaced during reload"
        );
        let photo = world.resource::<PhotomodeState>();
        assert!(!photo.free_camera_initialized && !photo.walk_camera_initialized && !photo.flag_08);
        assert_eq!(photo.camera_mode, PhotoModeCameraMode::TiltShift);
        if theme == GladeType::Winter {
            assert!(
                world
                    .resource::<country_core::asset_libraries::mesh_library::AssetMeshLibrary>()
                    .try_get_handle_by_name("ice_cracks")
                    .is_some(),
                "first winter entry was skipped by a run_once asset chain"
            );
        }
        // Enter actual photomode through the installed consumer; the current
        // normal camera comes from the newly loaded save, not the old free rig.
        {
            let world = driver.app_mut().world_mut();
            *world.resource_mut::<AppMode>() = AppMode::Photomode;
            world.send_event(AppModeChanged {
                current: AppMode::Photomode,
                previous: AppMode::default(),
            });
        }
        step(&mut driver, &ui, &mut seen);
        let camera = driver.app().world().resource::<MainCamera>();
        let rigs = camera.startup_rigs.as_ref().unwrap().lock().unwrap();
        let p = rigs.free.final_transform.position;
        assert!(
            (p.x - 500.0).abs() > 1.0 || (p.z + 500.0).abs() > 1.0,
            "free Dolly reused the previous session pose"
        );
        drop(rigs);
        // The loading timer must use the canonical GameTime and retain its
        // expired value. Do not replace audio or synthesize reveal events.
        *driver
            .app_mut()
            .world_mut()
            .resource_mut::<country_core::resources::gui::EscMenuState>() =
            country_core::resources::gui::EscMenuState::Closed;
        for _ in 0..90 {
            step(&mut driver, &ui, &mut seen);
        }
        #[cfg(target_os = "windows")]
        {
            type State = country_core::systems::audio::game_loading_audio::GameLoadingAudioState<
                country_core::resources::native_audio::NativeEvent,
            >;
            let loading = driver.app().world().resource::<State>();
            assert!(!loading.loading);
            assert!(loading.timer > 1.0);
        }
        handles = fresh;
        entities = new_entities;
    }
    Ok(())
}
