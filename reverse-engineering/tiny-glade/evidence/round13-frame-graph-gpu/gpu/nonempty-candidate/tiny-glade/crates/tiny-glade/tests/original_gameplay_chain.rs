//! Real gameplay App, shipped initial session and the production device tuple.
//! No individual systems are removed or replaced to make this fixture pass.
//! A temporary account prevents writes to the player's actual saves.
use std::{path::PathBuf, time::Duration};

use anyhow::{Context, Result};
use clap::Parser;
use country_core::{
    resources::{session::ActiveSession, walls::InnerWalls, window::WindowSize},
    systems::save::load_game::LoadGameManager,
};
use tiny_glade::{
    app_config::{CliArgs, get_app_config},
    render_bridge::CoreRenderBridge,
    run_game::{ApplicationShaderLibrary, run_game_thread},
};

/// Only this array is the `vectorize` representation of a HashMap (original
/// DecoratorHistoryEdit::serialize @1404f46e0, vectorize @1404be820). Its address
/// keys identify entries; every other history array retains its original order.
fn canonicalize_decorator_diff_map(value: &mut serde_json::Value) {
    let Some(diffs) = value.pointer_mut("/decorators/diffs") else {
        return;
    };
    let rows = diffs
        .as_array_mut()
        .expect("decorator diffs are key/value rows");
    let mut keys = std::collections::HashSet::new();
    for row in rows.iter() {
        let pair = row.as_array().expect("decorator map row is an array");
        assert_eq!(pair.len(), 2, "decorator map row is a key/value pair");
        assert!(
            keys.insert(pair[0].to_string()),
            "duplicate decorator address"
        );
    }
    rows.sort_by_cached_key(|row| row[0].to_string());
}

/// Compare the original v27 payload, rather than only checking that our encoder
/// and decoder agree with each other. These are the documented wire aliases and
/// omitted defaults of WallState and the shared snapshot decorator serializer.
/// Additional serialized defaults are permitted; an original nondefault field,
/// variant, map key or ordered array element must survive exactly.
fn canonicalize_original_history_aliases(value: &mut serde_json::Value) {
    use serde_json::{Value, json};
    match value {
        Value::Array(values) => {
            for value in values {
                canonicalize_original_history_aliases(value);
            }
        }
        Value::Object(object) => {
            if object.len() == 1 {
                if let Some(wall) = object.remove("WallNewSystem") {
                    object.insert("Wall".into(), wall);
                }
            }
            // Original SavedWallState field visitor aliases has_skirts to style.
            if object.contains_key("curve") && object.contains_key("is_enclosed") {
                if let Some(skirts) = object.remove("has_skirts") {
                    let style = match skirts {
                        Value::Bool(false) => json!("Stone"),
                        Value::Bool(true) => json!("StoneWithSkirts"),
                        other => panic!("invalid original has_skirts: {other}"),
                    };
                    assert!(object.insert("style".into(), style).is_none());
                }
            }
            if object.contains_key("dst") && object.contains_key("ty") {
                if let Some(offset) = object.remove("chimney_h_offset") {
                    assert!(object.insert("h_offset".into(), offset).is_none());
                }
                // take_snapshot's named-state writer skips these exact defaults.
                for (field, default) in [
                    ("color_id", json!(0)),
                    ("window_light_color", json!(0)),
                    ("ladder_color", json!(0)),
                    ("magic_color", json!(0)),
                    ("clv", json!(0)),
                    ("h_offset", json!(0.0)),
                    ("lantern_brightness", json!(1.0)),
                    ("stair_width", json!(1.0)),
                    ("allow_connection", json!(true)),
                    ("chimney_smoke", json!(true)),
                ] {
                    if object.get(field) == Some(&default) {
                        object.remove(field);
                    }
                }
            }
            if object.contains_key("coord") && object.contains_key("anchor") {
                let anchor = object["anchor"].as_object_mut().expect("wall anchor map");
                for (field, default) in [
                    ("wall_height_relative_y", json!(0.0)),
                    ("anchor_ws", json!([0.0, 0.0, 0.0])),
                    ("platform", json!(0)),
                    ("is_bottom_door", json!(false)),
                    ("side", json!("Outer")),
                ] {
                    if anchor.get(field) == Some(&default) {
                        anchor.remove(field);
                    }
                }
            }
            for value in object.values_mut() {
                canonicalize_original_history_aliases(value);
            }
        }
        _ => {}
    }
}

fn assert_original_fields_retained(
    original: &serde_json::Value,
    saved: &serde_json::Value,
    path: &str,
) {
    use serde_json::Value;
    match (original, saved) {
        (Value::Object(original), Value::Object(saved)) => {
            for (field, original) in original {
                let path = format!("{path}/{field}");
                let saved = saved.get(field).unwrap_or_else(|| panic!("dropped {path}"));
                assert_original_fields_retained(original, saved, &path);
            }
        }
        (Value::Array(original), Value::Array(saved)) => {
            assert_eq!(original.len(), saved.len(), "array length at {path}");
            for (index, (original, saved)) in original.iter().zip(saved).enumerate() {
                assert_original_fields_retained(original, saved, &format!("{path}/{index}"));
            }
        }
        _ => assert_eq!(original, saved, "original payload at {path}"),
    }
}

/// Exercise the public history wire through the same linked production core.
/// This runs explicitly, without rebuilding the entire internal unit harness
/// or treating its external-assets ignored diagnostic as an executed test.
#[test]
fn all_shipped_histories_keep_saved_aux_and_bidirectional_replay_commands() -> Result<()> {
    use country_core::resources::history::{
        replay_cmd_buffer::ReplayBucket, serialized::SavedMultiEdit,
    };
    let root = PathBuf::from(
        std::env::var_os("TG_INSTALL_ROOT")
            .context("TG_INSTALL_ROOT names the original B installation")?,
    )
    .join("assets/starting-builds");
    let (mut files, mut entries_count, mut decorator_aux, mut stairs_aux) = (0, 0, 0, 0);
    for edition in std::fs::read_dir(root)? {
        for build in std::fs::read_dir(edition?.path())? {
            let path = build?.path().join("history.json");
            if !path.is_file() {
                continue;
            }
            let asset: serde_json::Value = serde_json::from_slice(&std::fs::read(&path)?)?;
            let entries: Vec<SavedMultiEdit> =
                serde_json::from_value(asset["History"]["edit_chain"].clone())
                    .with_context(|| path.display().to_string())?;
            files += 1;
            entries_count += entries.len();
            for (index, entry) in entries.into_iter().enumerate() {
                decorator_aux += usize::from(entry.0.auxiliary.decorator_diff.is_some());
                stairs_aux += usize::from(entry.0.auxiliary.stairs_diff.is_some());
                let mut saved = serde_json::to_value(&entry)?;
                let loaded: SavedMultiEdit = serde_json::from_value(saved.clone())?;
                let mut resaved = serde_json::to_value(&loaded)?;
                canonicalize_decorator_diff_map(&mut saved);
                canonicalize_decorator_diff_map(&mut resaved);
                assert_eq!(resaved, saved, "{}", path.display());
                assert_eq!(loaded.0.auxiliary, entry.0.auxiliary);
                if asset["Version"] == 27 {
                    let mut original = asset["History"]["edit_chain"][index].clone();
                    let mut encoded = saved.clone();
                    canonicalize_original_history_aliases(&mut original);
                    canonicalize_original_history_aliases(&mut encoded);
                    canonicalize_decorator_diff_map(&mut original);
                    canonicalize_decorator_diff_map(&mut encoded);
                    assert_original_fields_retained(
                        &original,
                        &encoded,
                        &format!("{} entry {index}", path.display()),
                    );
                }
                for undo in [false, true] {
                    ReplayBucket::expand_with_aux(
                        0,
                        &loaded.0,
                        undo,
                        true,
                        loaded.0.auxiliary.for_replay(undo),
                    )
                    .map_err(|error| {
                        anyhow::anyhow!("{} undo={undo}: {error:?}", path.display())
                    })?;
                }
            }
        }
    }
    assert_eq!(
        (files, entries_count, decorator_aux, stairs_aux),
        (33, 2120, 408, 63)
    );
    eprintln!(
        "PASS: {files} original histories / {entries_count} entries, {decorator_aux} decorator / {stairs_aux} stairs auxiliary owners"
    );
    Ok(())
}

#[test]
fn shipped_session_through_actual_app_and_production_world_graph() -> Result<()> {
    let installed = std::env::var_os("TG_INSTALL_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../ljxsj_92385/Tiny Glade")
        })
        .canonicalize()?;
    let account = tempfile::tempdir()?;
    paths::initialize_paths_with_installation(0, Some(account.path()), &installed)?;
    std::env::set_current_dir(&installed)?;
    utils::local_time::init_local_time();
    tiny_glade::load_language_library::load_language_library(None, None);
    let cli = CliArgs::parse_from(["original-gameplay-chain"]);
    let config = get_app_config(&cli)?;
    assert!(
        !config.dev.demo,
        "this acceptance requires the full original build"
    );
    let mut app = run_game_thread(config, &cli, WindowSize::new(480, 270))?;
    app.insert_resource(
        country_core::resources::platform::RuntimePlatform::from_detection(false, false),
    );
    tiny_glade::game_loop::register_platform_events(&mut app);
    tiny_glade::game_loop::register_settings_application_system(&mut app);
    let device =
        rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig::headless())
            .context("strict gameplay physical device")?;
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
        installed,
    )?;
    // The window path uses this driver to install the same asset maintenance
    // and canonical egui Context before its first App::update.
    let context = egui::Context::default();
    let mut driver = tiny_glade::game_loop::GameFrameDriver::new(app, context.clone());
    driver
        .app_mut()
        .insert_resource(bevy_time::TimeUpdateStrategy::ManualDuration(
            Duration::from_secs_f64(1.0 / 60.0),
        ));
    let mut loaded = false;
    for frame in 0..600 {
        context.begin_frame(egui::RawInput::default());
        driver.app_mut().update();
        let _ = context.end_frame();
        let world = driver.app().world();
        if frame % 60 == 0 {
            eprintln!(
                "gameplay App frame {frame}: session={}, walls={}",
                world.contains_resource::<ActiveSession>(),
                world.resource::<InnerWalls>().wall_ids().count()
            );
        }
        if world.contains_resource::<ActiveSession>()
            && world.resource::<LoadGameManager>().options.is_none()
            && world.resource::<InnerWalls>().wall_ids().count() > 0
        {
            loaded = true;
            break;
        }
    }
    anyhow::ensure!(
        loaded,
        "shipped intro did not finish through the real gameplay schedule"
    );
    // Let actual asynchronous assets, constructor uploads and intro migration
    // settle through their normal schedules before asserting visible owners.
    for _ in 0..90 {
        context.begin_frame(egui::RawInput::default());
        driver.app_mut().update();
        let _ = context.end_frame();
    }
    let world = driver.app_mut().world_mut();
    assert_eq!(world.resource::<InnerWalls>().wall_ids().count(), 3);
    assert_nonempty_original_walls(world, shaders.as_ref());
    // Preserve downstream failures in the acceptance log. A successful graph
    // export alone cannot establish visible wall construction or full play.
    eprintln!(
        "wall construction residuals: {:?}",
        world
            .resource::<country_core::systems::wall::construct_wall_bricks::WallBrickBuildState>()
            .failures
    );
    let mut lists = country_core::systems::render_loop::draw_passes::draw_lists::DrawLists::new();
    country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists(
        world,
        shaders.as_ref(),
        &mut lists,
    );
    eprintln!(
        "actual draw-list entries before warmup: {}",
        lists.total_len()
    );
    anyhow::ensure!(
        lists.total_len() > 0,
        "shipped gameplay acceptance requires actual nonempty DrawLists before production GPU warmup"
    );
    assert!(world.contains_resource::<tiny_glade::loading_graph::LoadingGpuResources>());
    tiny_glade::production_renderer::warm_up_gameplay(bridge.warmup_owners()?, world)
        .context("actual production world graph compile, execute and exported owner")?;
    assert!(world.contains_resource::<tiny_glade::render_bridge::CompletedGameplayWarmup>());
    eprintln!(
        "PASS: shipped session -> actual gameplay App -> production graph -> physical export"
    );
    Ok(())
}

// Central acceptance fragment. Merge after real startup producers are installed.
// This checks actual loaded state, per-wall storage and global draw admission.
fn assert_nonempty_original_walls(
    world: &mut bevy_ecs::world::World,
    shaders: &country_core::asset_libraries::shader_library::AssetShaderLibrary,
) {
    use country_core::{
        geometry::instanced_wall::{InstancedWall, WallBrickInstancedDrawProxy},
        resources::walls::{InnerWalls, public_walls::{PublicWalls, WallEntities}},
        systems::{wall::{construction::bricks::WallBrickInstance,
            construct_wall_bricks::WallBrickBuildState},
            render_loop::draw_passes::draw_lists::{self, DrawLists}},
    };
    let failures = &world.resource::<WallBrickBuildState>().failures;
    assert!(failures.is_empty(), "startup construction failures: {failures:?}");
    let public = world.resource::<PublicWalls>();
    assert!(public.failures().is_empty(), "publication failed: {:?}", public.failures());
    assert!(public.path_failures().is_empty(), "path preparation failed: {:?}", public.path_failures());
    let walls: Vec<_> = world.resource::<InnerWalls>().wall_ids().collect();
    assert_eq!(walls.len(), 3, "original intro has three authoritative walls");
    let mut nonempty_allocations = 0;
    for id in walls {
        let entity = world.resource::<WallEntities>().0.get(&id)
            .expect("loaded wall never acquired an owning entity");
        let instance = world.get::<InstancedWall<WallBrickInstance>>(*entity)
            .expect("wall entity has no real brick instance owner");
        assert!(!instance.is_upload_pending(), "startup bricks stayed in CPU pending state");
        if let Some(allocation) = instance.allocation() {
            if allocation.handle.0.as_ref().is_some_and(|a| !a.chunk_ids.lock().is_empty()) {
                nonempty_allocations += 1;
            }
        }
    }
    assert!(nonempty_allocations > 0, "no loaded wall produced real allocated brick chunks");
    let ids: std::collections::HashSet<_> = world.query::<(bevy_ecs::entity::Entity, &WallBrickInstancedDrawProxy)>()
        .iter(world).map(|(entity, _)| entity).collect();
    assert_eq!(ids.len(), 7, "original seven global wall drawables were not produced exactly once");
    let mut lists = DrawLists::new();
    draw_lists::build_draw_lists(world, shaders, &mut lists);
    for (list, expected) in [(draw_lists::list::SHADOW, 7),
        (draw_lists::list::GBUFFER, 7), (draw_lists::list::GBUFFER_OC2, 4)] {
        assert_eq!(lists.list(list).iter().filter(|entry| ids.contains(&entry.0.params.entity)).count(), expected,
            "physical asset maintenance did not admit all original wall drawables to list {list}");
    }
}
