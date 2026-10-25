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

/// 原TerrainAttachment的pos_xz为Vec2<f32>，yaw_radians为f32。
/// 只准入auxiliary差分内该确切schema；其它数字、字段和数组不受影响。
fn is_native_terrain_attachment_f32_path(path: &str) -> bool {
    let Some((_, suffix)) = path.split_once("/decorators/diffs/") else {
        return false;
    };
    let parts: Vec<_> = suffix.split('/').collect();
    let (row, phase, id) = match parts.as_slice() {
        [row, "1", phase, id, "dst", "Terrain", "pos_xz", "0" | "1"]
        | [row, "1", phase, id, "dst", "Terrain", "yaw_radians"] => (*row, *phase, *id),
        _ => return false,
    };
    row.parse::<usize>().is_ok()
        && matches!(phase, "before" | "after")
        && id.parse::<u32>().is_ok()
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
        (Value::Number(_), Value::Number(_)) if is_native_terrain_attachment_f32_path(path) => {
            // Value先把f32扩为f64，默认JSON解析还可能再偏移一个f64 ULP。
            // 这里比较原字段的全部32位，绝不使用epsilon或把整数一并降精度。
            let original: f32 = serde_json::from_value(original.clone())
                .unwrap_or_else(|error| panic!("original f32 at {path}: {error}"));
            let saved: f32 = serde_json::from_value(saved.clone())
                .unwrap_or_else(|error| panic!("saved f32 at {path}: {error}"));
            assert!(original.is_finite() && saved.is_finite(), "finite f32 at {path}");
            assert_eq!(original.to_bits(), saved.to_bits(), "original f32 payload at {path}");
        }
        _ => assert_eq!(original, saved, "original payload at {path}"),
    }
}

#[test]
fn original_terrain_attachment_preserves_exact_f32_bits_across_value_and_compact_wire() -> Result<()>
{
    use country_core::resources::{
        history::serialized::SavedMultiEdit,
        walls::{decorator::DecoratorId, decorator_storage::DecoratorStorageAddr},
    };
    // 原03-halftimber-roof entry53的真实装饰物；保留原数字文本。
    let original: serde_json::Value = serde_json::from_str(
        r#"{
        "edits":[{"DecoratorV5":{"Place":{}}}],
        "decorators":{"diffs":[["Terrain",{"before":{},"after":{"0":{
            "dst":{"Terrain":{"pos_xz":[3.9437943,-5.4331703],"yaw_radians":2.5964363}},
            "ty":"Stairs","rank":{"x":1},"color_id":100003,"seed":0,"clv":5,
            "allow_connection":false,"stair_width":1.5
        }}}]]}
    }"#,
    )?;
    let entry: SavedMultiEdit = serde_json::from_value(original.clone())?;
    let source = entry
        .0
        .auxiliary
        .decorator_diff
        .as_ref()
        .unwrap()
        .get(&DecoratorStorageAddr::Terrain)
        .unwrap()
        .after
        .states
        .get(&DecoratorId(0))
        .unwrap();
    assert_eq!(source.dst_wall as u32, 0x407c_6720);
    let middle = serde_json::to_value(&entry)?;
    let saved: serde_json::Value = serde_json::from_slice(&serde_json::to_vec(&entry)?)?;
    let pointer = "/decorators/diffs/0/1/after/0/dst/Terrain/pos_xz/0";
    assert_eq!(
        middle.pointer(pointer).unwrap().as_f64().unwrap().to_bits(),
        0x400f_8ce4_0000_0000
    );
    // 当前锁定serde_json1.0.85/default解析分支的实际数值合同。
    assert_eq!(
        saved.pointer(pointer).unwrap().as_f64().unwrap().to_bits(),
        0x400f_8ce4_0000_0001
    );
    assert_ne!(original.pointer(pointer), saved.pointer(pointer));
    assert_original_fields_retained(&original, &saved, "fixture entry53");
    let restored: SavedMultiEdit = serde_json::from_value(saved)?;
    assert_eq!(restored.0.auxiliary, entry.0.auxiliary);
    Ok(())
}

#[test]
fn terrain_numeric_comparison_rejects_one_f32_bit_change_and_signed_zero_loss() {
    use serde_json::Value;
    let path = "fixture/decorators/diffs/0/1/after/0/dst/Terrain/pos_xz/0";
    for (original, corrupted) in [
        (f32::from_bits(0x407c_6720), f32::from_bits(0x407c_6721)),
        (-0.0_f32, 0.0_f32),
    ] {
        let failure = std::panic::catch_unwind(|| {
            assert_original_fields_retained(&Value::from(original), &Value::from(corrupted), path);
        });
        assert!(failure.is_err(), "不能放过实际f32位变化");
    }
}

#[test]
fn terrain_numeric_scope_keeps_ids_unknown_fields_and_other_domains_strict() {
    use serde_json::json;
    let a: serde_json::Value = serde_json::from_str("3.9437943").unwrap();
    let b = serde_json::to_value(f32::from_bits(0x407c_6720)).unwrap();
    for path in [
        "fixture/decorators/diffs/0/1/after/0/seed",
        "fixture/decorators/diffs/0/1/after/0/dst/Terrain/unknown",
        "fixture/decorators/diffs/0/1/after/0/dst/Terrain/pos_xz/2",
        "fixture/decorators/diffs/0/1/after/0/dst/Roof/mesh_uv/0",
        "fixture/other/dst/Terrain/pos_xz/0",
    ] {
        assert!(!is_native_terrain_attachment_f32_path(path));
        let failure = std::panic::catch_unwind(|| assert_original_fields_retained(&a, &b, path));
        assert!(failure.is_err(), "未知字段仍必须精确相等：{path}");
    }
    let failure = std::panic::catch_unwind(|| {
        assert_original_fields_retained(
            &json!(16_777_216_u32),
            &json!(16_777_217_u32),
            "fixture/decorators/diffs/0/1/after/0/seed",
        );
    });
    assert!(failure.is_err(), "不能将u32 ID降为f32后误判相等");
    for suffix in [
        "before/0/dst/Terrain/pos_xz/0",
        "after/42/dst/Terrain/pos_xz/1",
        "after/0/dst/Terrain/yaw_radians",
    ] {
        let path = format!("fixture/decorators/diffs/0/1/{suffix}");
        assert!(is_native_terrain_attachment_f32_path(&path));
        assert_original_fields_retained(&a, &b, &path);
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
                // 原 Value 序列化先将 f32 扩为 f64，独立验证该中间表示的往返。
                let mut value = serde_json::to_value(&entry)?;
                let value_loaded: SavedMultiEdit = serde_json::from_value(value.clone())?;
                let mut value_resaved = serde_json::to_value(value_loaded)?;
                canonicalize_decorator_diff_map(&mut value);
                canonicalize_decorator_diff_map(&mut value_resaved);
                assert_eq!(value_resaved, value, "{}", path.display());
                // 原资产文本及实际 SerializedJson::new 调用使用 CompactFormatter。
                let mut saved: serde_json::Value =
                    serde_json::from_slice(&serde_json::to_vec(&entry)?)?;
                let loaded: SavedMultiEdit = serde_json::from_value(saved.clone())?;
                let mut resaved: serde_json::Value =
                    serde_json::from_slice(&serde_json::to_vec(&loaded)?)?;
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

/// Everything the shipped-session tests share: the real gameplay App loaded to the shipped intro
/// session and settled, the device/bridge tuple of the production window path.
struct BootedSession {
    driver: tiny_glade::game_loop::GameFrameDriver,
    context: egui::Context,
    bridge: CoreRenderBridge,
    shaders: std::sync::Arc<country_core::asset_libraries::shader_library::AssetShaderLibrary>,
    device: std::sync::Arc<rhapsody::device::Device>,
    _boot: std::sync::MutexGuard<'static, ()>,
}

/// 显式诊断已完成但尚待销毁的对象，不改变逐帧采样或后面的稳定性断言。
fn diagnose_completed_retirement(device: &rhapsody::device::Device, stage: &str) -> Result<()> {
    if !std::env::var_os("TG_RETIREMENT_DRAIN_DIAGNOSTIC").is_some_and(|v| v == "1") {
        return Ok(());
    }
    let before = device.retirement_stats();
    let bytes_before = device.allocator_stats().totals().total_allocated_bytes;
    let destroyed = device.try_collect_all_completed()?;
    let after = device.retirement_stats();
    let bytes_after = device.allocator_stats().totals().total_allocated_bytes;
    eprintln!(
        "RETIREMENT_DRAIN_DIAGNOSTIC stage={stage} pending={}→{} destroyed={destroyed} allocated_bytes={bytes_before}→{bytes_after} submitted={:?} completed={:?} idle_waits={}→{}",
        before.pending, after.pending, after.submitted, after.completed,
        before.idle_waits, after.idle_waits,
    );
    Ok(())
}

fn boot_shipped_session() -> Result<BootedSession> {
    let installed = std::env::var_os("TG_INSTALL_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../ljxsj_92385/Tiny Glade")
        })
        .canonicalize()?;
    // Both shipped-session tests boot a whole App in this process: the account paths are process
    // global, so they are initialised once and the boots are serialised.
    static BOOT: std::sync::Mutex<()> = std::sync::Mutex::new(());
    static ACCOUNT: std::sync::OnceLock<tempfile::TempDir> = std::sync::OnceLock::new();
    let boot_guard = BOOT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if ACCOUNT.get().is_none() {
        let account = tempfile::tempdir()?;
        paths::initialize_paths_with_installation(0, Some(account.path()), &installed)?;
        let _ = ACCOUNT.set(account);
    }
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
        rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig {
            // Same budget as the production window path (the original allocator has no cap).
            memory_budget_bytes: tiny_glade::production_renderer::UNBOUNDED_DEVICE_BUDGET_BYTES,
            ..rhapsody::device::DeviceConfig::headless()
        })
        .context("strict gameplay physical device")?;
    let shaders = app.world().resource::<ApplicationShaderLibrary>().0.clone();
    let bridge = CoreRenderBridge::new(device.clone(), installed.clone(), shaders.clone())?;
    let mut final_post = None;
    country_core::startup::load_shaders::load_shaders(
        &shaders,
        bridge.shader_backend(),
        &mut final_post,
    );
    // Same shader set as ProductionRenderer::new: the world tail (ui_2d, text, vermeer,
    // painterly, blit) is part of the compiled production graph.
    country_core::startup::load_shaders::load_ui_shaders(&shaders, bridge.shader_backend());
    country_core::startup::startup_blit::startup_blit(&shaders, bridge.shader_backend());
    struct TailShaders<'a> {
        library: &'a country_core::asset_libraries::shader_library::AssetShaderLibrary,
        backend: &'a dyn country_core::render::shader::boundary::ShaderBackend,
    }
    impl country_core::systems::render_loop::painterly::ShaderLoader for TailShaders<'_> {
        fn load_shader_into_library(&mut self, path: &'static str, name: &'static str) {
            country_core::startup::load_shaders::load_shader_into_library(
                self.library,
                self.backend,
                path,
                name,
            );
        }
    }
    country_core::systems::render_loop::painterly::startup_painterly(&mut TailShaders {
        library: &shaders,
        backend: bridge.shader_backend(),
    });
    country_core::systems::render_loop::vermeer::startup_vermeer(&mut TailShaders {
        library: &shaders,
        backend: bridge.shader_backend(),
    });
    bridge.install_asset_libraries(app.world_mut())?;
    tiny_glade::gameplay_startup::install_gameplay_device_runtime(
        &mut app,
        device.clone(),
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
    Ok(BootedSession { driver, context, bridge, shaders, device, _boot: boot_guard })
}

#[test]
fn shipped_session_through_actual_app_and_production_world_graph() -> Result<()> {
    let BootedSession { mut driver, context: _context, bridge, shaders, device, _boot } = boot_shipped_session()?;
    let world = driver.app_mut().world_mut();
    assert_eq!(world.resource::<InnerWalls>().wall_ids().count(), 3);
    assert_nonempty_original_walls(world, shaders.as_ref(), &bridge);
    // Preserve downstream failures in the acceptance log. A successful graph
    // export alone cannot establish visible wall construction or full play.
    eprintln!(
        "wall construction residuals: {:?}",
        world
            .resource::<country_core::systems::wall::construct_wall_bricks::WallBrickBuildState>()
            .failures
    );
    // Original "register shader + spawn entity" Startup / OnEnter(Game) producers must have run on this App.
    for name in [
        "cloud_shader", "star_shader", "billboard_forest_shader", "loading_screen", "sparkles_shader",
        "leaf_particle", "texture_test_shader", "instanced_ivy_branch_shader", "instanced_ivy_leaf_shader",
        "instanced_ivy_flower_shader", "water_plane", "water_plane_refraction_copy", "lilypad_shader",
        "lilypad_flower_shader", "reed_shader", "water_x_paths",
    ] {
        assert!(shaders.get_raster_handle_by_name(name).is_some(), "shader {name} was never registered");
    }
    {
        use country_core::systems::render_loop::draw_passes::draw_lists::DesignatedRenderPass;
        let mut passes = world.query::<&DesignatedRenderPass>();
        let mut count = |pass: u32| passes.iter(world).filter(|p| p.0 == pass).count();
        assert_eq!(count(0x10), 2, "clouds + stars (list 16)");
        assert_eq!(count(0x12), 1, "loading_screen (list 18)");
        assert_eq!(count(15), 1, "water surface (list 15)");
        assert_eq!(count(14), 1, "water refraction copy (list 14)");
    }
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
    assert!(world.contains_resource::<tiny_glade::loading_graph::LoadingGpuResources>());
    // Real pixel consumer on the shipped screenshot owner: the T4 MenuBackground opportunity of the
    // actual world graph reads the world color back through the original capture chain
    // (after final_post, before any overlay). The pixels are checked below, not just the export.
    anyhow::ensure!(
        tiny_glade::screenshot::shared_texture_capture::installed(world),
        "the gameplay App has no screenshot owner"
    );
    let captured: std::sync::Arc<parking_lot::Mutex<Option<(u32, u32, Vec<u8>)>>> =
        Default::default();
    let sink = captured.clone();
    tiny_glade::screenshot::gather_screenshot_requests(
        world,
        [tiny_glade::screenshot::ScreenshotRequest {
            purpose: tiny_glade::screenshot::ScreenshotPurpose::MenuBackground,
            dimensions: None,
            sink: tiny_glade::screenshot::ScreenshotSink::Rgba(std::sync::Arc::new(
                move |w, h, rgba| *sink.lock() = Some((w, h, rgba)),
            )),
            completion: None,
        }],
    )?;
    tiny_glade::production_renderer::warm_up_gameplay(bridge.warmup_owners()?, world)
        .context("actual production world graph compile, execute and exported owner")?;
    // The capture retires after its original frame budget (3); drive the owner past it.
    for _ in 0..8 {
        tiny_glade::screenshot::shared_texture_capture::finish(world)?;
    }
    tiny_glade::screenshot::shared_texture_capture::shutdown(world)?;
    diagnose_completed_retirement(&device, "single-world-complete")?;
    let (w, h, rgba) = captured
        .lock()
        .take()
        .context("world color capture never delivered pixels")?;
    anyhow::ensure!(
        rgba.len() == (w * h * 4) as usize,
        "captured pixel buffer does not match {w}x{h}"
    );
    if let Some(path) = std::env::var_os("TG_PIXEL_OUT") {
        let mut ppm = format!("P6\n{w} {h}\n255\n").into_bytes();
        for px in rgba.chunks_exact(4) {
            ppm.extend_from_slice(&px[..3]);
        }
        std::fs::write(path, ppm)?;
    }
    // A black frame is the failure this guards: the world has terrain, grass and trees, so
    // the picture must carry real luminance and many distinct colors.
    let pixels = (w * h) as f64;
    let mean = rgba
        .chunks_exact(4)
        .map(|px| (u32::from(px[0]) + u32::from(px[1]) + u32::from(px[2])) as f64 / 3.0)
        .sum::<f64>()
        / pixels;
    let distinct = rgba
        .chunks_exact(4)
        .map(|px| u32::from_le_bytes([px[0], px[1], px[2], 0]))
        .collect::<std::collections::HashSet<_>>()
        .len();
    eprintln!("PIXELS {w}x{h} mean_luma={mean:.1} distinct_colors={distinct}");
    assert!(mean > 40.0, "world frame is (nearly) black: mean luma {mean}");
    assert!(distinct > 2000, "world frame lacks content: {distinct} distinct colors");
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
    bridge: &CoreRenderBridge,
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

    // An allocation/pending=false records the CPU write; a later device flush
    // may still have failed. Require the production publisher's outcome and
    // the exact physical namespace used by this App's bridge.
    let status = world.resource::<country_core::geometry::runtime::WallBrickUploadStatus>();
    assert!(status.gpu_ready, "brick publisher failed: {:?}", status.last_error);
    assert_eq!(status.capacity_failures, 0, "brick allocation capacity failure");
    assert!(status.last_error.is_none(), "brick upload error was retained");
    let storage = world.resource::<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>()
        .get::<WallBrickInstance>().expect("no canonical brick sparse SSBO");
    let ranges = storage.to_draw_multi_instanced();
    let count: u64 = ranges.iter().map(|(_, count)| u64::from(*count)).sum();
    assert!(count > 0, "allocated brick chunks contain no drawable instances");
    assert!(storage.dirty_bitset.iter().all(|word| word.load(std::sync::atomic::Ordering::Relaxed) == 0),
        "brick upload left dirty chunks");
    assert!(bridge.hosts().buffers.managed_buffer(&storage.buffer.buffer).is_some(),
        "brick SSBO does not resolve on the production App device");
    eprintln!("actual uploaded wall brick instances: {count}, draw ranges: {}", ranges.len());

    let ids: std::collections::HashSet<_> = world.query::<(bevy_ecs::entity::Entity, &WallBrickInstancedDrawProxy)>()
        .iter(world).map(|(entity, _)| entity).collect();
    assert_eq!(ids.len(), 7, "original seven global wall drawables were not produced exactly once");

    // A draw-list item only proves ECS admission. Its indexed mesh must also
    // resolve to a nonempty VAO and retained physical index-buffer owner.
    for entity in &ids {
        let handle = world.get::<country_core::asset_libraries::Handle<country_core::render::vao::Vao>>(*entity)
            .expect("wall drawable has no published VAO");
        let vao = world.resource::<country_core::asset_libraries::vao_library::AssetVAOLibrary>()
            .get_by_handle(*handle).expect("wall VAO handle is stale");
        assert!(vao.index_count > 0, "wall drawable has no mesh indices");
        assert!(bridge.hosts().buffers.managed_buffer(&vao.vao.index_buffer).is_some(),
            "wall VAO index buffer does not resolve on the production App device");
    }

    let mut lists = DrawLists::new();
    draw_lists::build_draw_lists(world, shaders, &mut lists);
    for (list, expected) in [(draw_lists::list::SHADOW, 7),
        (draw_lists::list::GBUFFER, 7), (draw_lists::list::GBUFFER_OC2, 4)] {
        assert_eq!(lists.list(list).iter().filter(|entry| ids.contains(&entry.0.params.entity)).count(), expected,
            "physical asset maintenance did not admit all original wall drawables to list {list}");
    }
}

/// One row of the multi-frame table: what the render owners report after frame `index`.
struct FrameRow {
    index: usize,
    frame_counter: u32,
    history_valid: bool,
    stats: rhapsody::rg::ExecutionStats,
    cache_hits: u64,
    cache_misses: u64,
    cache_diagnostics: String,
    update: Duration,
    draw_lists: Duration,
    declare: Duration,
    compile: Duration,
    execute: Duration,
    shader_heap: rhapsody::bindless::BindlessStats,
    texture_heap: rhapsody::bindless::BindlessStats,
    constants: rhapsody::resource::DynamicConstantsStats,
    allocated_bytes: u64,
    native_blocks: usize,
    parked_resources: usize,
    passes: Vec<(String, u64, u64)>,
}

fn percentile(sorted: &[f64], q: f64) -> f64 {
    sorted[(((sorted.len() - 1) as f64) * q).round() as usize]
}

fn luma(rgba: &[u8]) -> Vec<f32> {
    rgba.chunks_exact(4)
        .map(|px| 0.2126 * f32::from(px[0]) + 0.7152 * f32::from(px[1]) + 0.0722 * f32::from(px[2]))
        .collect()
}

/// Steady-state rendering: the shipped session rendered for 30 consecutive frames through the
/// production gameplay graph on one session (warm-up history adopted, plan cache, named/temporal
/// ping-pong, constants ring and descriptor leases recycled across frames). A camera that does not
/// move must converge: TAA history is valid from the second frame and frames stay stable.
#[test]
fn shipped_session_steady_state_multi_frame() -> Result<()> {
    const FRAMES: usize = 30;
    const CAPTURES: [usize; 7] = [1, 2, 3, 4, 9, 19, 29]; // 2nd, 10th and 30th frame plus the convergence ramp // 2nd, 10th and 30th frame plus the convergence ramp
    let BootedSession { mut driver, context, mut bridge, shaders: _shaders, device, _boot } =
        boot_shipped_session()?;
    let world = driver.app_mut().world_mut();
    anyhow::ensure!(
        tiny_glade::screenshot::shared_texture_capture::installed(world),
        "the gameplay App has no screenshot owner"
    );
    // Frame 0 of the production path: the worker warm-up renders the first frame and hands its
    // named/temporal owners to the main-thread session.
    tiny_glade::production_renderer::warm_up_gameplay(bridge.warmup_owners()?, world)
        .context("worker warm-up frame")?;
    {
        let completed = world.resource::<tiny_glade::render_bridge::CompletedGameplayWarmup>();
        bridge.accept_gameplay_warmup(completed)?;
    }
    let captured: std::sync::Arc<parking_lot::Mutex<Vec<(usize, u32, u32, Vec<u8>)>>> =
        Default::default();
    let out_dir = std::env::var_os("TG_MULTIFRAME_OUT").map(PathBuf::from);
    let gpu_timing = std::env::var_os("TG_GPU_TIMING").is_some_and(|value| value == "1");
    let mut gpu_samples = Vec::<serde_json::Value>::new();
    if gpu_timing {
        anyhow::ensure!(out_dir.is_some(), "TG_GPU_TIMING 需要 TG_MULTIFRAME_OUT 保存真实计时证据");
        bridge.with_session(|session| session.backend.configure_gpu_timing(Some(
            rhapsody::rg::GpuTimingConfig::default(),
        )))?;
    }
    let mut rows: Vec<FrameRow> = Vec::new();
    let mut in_flight = std::collections::VecDeque::<u64>::new();
    for index in 0..FRAMES {
        let update_started = std::time::Instant::now();
        context.begin_frame(egui::RawInput::default());
        driver.app_mut().update();
        let _ = context.end_frame();
        let update = update_started.elapsed();
        // Window path pacing (`acquire_swapchain_image`): at most `frames_in_flight` submitted
        // frames are outstanding before the next frame starts recording.
        while bridge.with_session(|session| session.pending_frame_count())
            >= device.frames_in_flight() as usize
        {
            let oldest = in_flight.pop_front().context("pending frame without receipt")?;
            bridge.with_session(|session| session.wait_frame(oldest, 120_000_000_000))?;
        }
        let world = driver.app_mut().world_mut();
        let frame_counter = world
            .resource::<country_core::render::frame_constants::FrameConstants>()
            .frame_counter;
        if CAPTURES.contains(&index) {
            let sink = captured.clone();
            tiny_glade::screenshot::gather_screenshot_requests(
                world,
                [tiny_glade::screenshot::ScreenshotRequest {
                    purpose: tiny_glade::screenshot::ScreenshotPurpose::MenuBackground,
                    dimensions: None,
                    sink: tiny_glade::screenshot::ScreenshotSink::Rgba(std::sync::Arc::new(
                        move |w, h, rgba| sink.lock().push((index, w, h, rgba)),
                    )),
                    completion: None,
                }],
            )?;
        }
        let frame = tiny_glade::production_renderer::render_headless_gameplay_frame(&mut bridge, world)
            .with_context(|| {
                format!(
                    "steady-state frame {index}: constants {:?}, shader heap {:?}, pending {}",
                    { let stats = bridge.raster_resources().dynamic_constants().lock().stats(); stats },
                    bridge.shader_backend().bindless_stats(),
                    bridge.with_session(|session| session.pending_frame_count()),
                )
            })?;
        let history_valid = world
            .resource::<tiny_glade::world_graph::WorldHistoryObservation>()
            .color_history_valid;
        tiny_glade::screenshot::finish_screenshot_capture(world)?;
        // GameLoop @1404de63a: the temporal frame index advances after render returns.
        world
            .resource_mut::<country_core::render::RenderTemporalState>()
            .frame_render_returned();
        let stats = frame.receipt.stats;
        eprintln!(
            "MULTIFRAME progress frame {index}: shader heap {:?}, record {:.0} ms, retirement {:?}",
            bridge.shader_backend().bindless_stats(),
            stats.record_ns as f64 / 1e6,
            device.retirement_stats(),
        );
        in_flight.push_back(frame.receipt.frame);
        let (cache, passes) = bridge.with_session(|session| {
            let passes = session
                .chain
                .pgo
                .previous
                .iter()
                .map(|(name, ns, count)| (name.to_owned(), ns, count))
                .collect::<Vec<_>>();
            (session.chain.plan_cache_stats(), passes)
        });
        let graph_caches = bridge.with_session(|session| session.cache_stats());
        let usage = device.memory_usage_snapshot();
        let texture_heap = world
            .resource::<country_core::asset_libraries::texture_library::AssetTextureLibrary>()
            .dset()
            .heap()?
            .stats();
        let constants = bridge.raster_resources().dynamic_constants().lock().stats();
        rows.push(FrameRow {
            index,
            frame_counter,
            history_valid,
            stats,
            cache_hits: cache.hits,
            cache_misses: cache.misses,
            cache_diagnostics: format!("{:?}", cache.diagnostics),
            update,
            draw_lists: frame.draw_lists,
            declare: frame.declare,
            compile: frame.compile,
            execute: frame.execute,
            shader_heap: bridge.shader_backend().bindless_stats(),
            texture_heap,
            constants,
            allocated_bytes: device.allocator_stats().totals().total_allocated_bytes,
            native_blocks: usage.native_memory_blocks,
            parked_resources: graph_caches.parked_resources,
            passes,
        });
        drop(frame);
        bridge.collect_completed_frames()?;
        if gpu_timing {
            let (stats, samples) = bridge.with_session(|session| {
                let stats = session.backend.poll_gpu_timings()?;
                let samples = session.backend.drain_gpu_timings().collect::<Vec<_>>();
                Ok::<_, rhapsody::Error>((stats, samples))
            })?;
            gpu_samples.push(serde_json::json!({"observed_after_frame": index, "stats": stats, "samples": samples}));
        }
        if let (Some(dir), true) = (&out_dir, index == 5 || index == FRAMES - 1) {
            let mut by_name = std::collections::BTreeMap::<String, (u64, u64)>::new();
            for record in device.allocator_stats().report().allocations {
                let entry = by_name.entry(format!("heap{} props{:x} size{} {}", record.heap, record.properties, record.size, record.name)).or_default();
                entry.0 += 1;
                entry.1 += record.size;
            }
            let text = by_name
                .iter()
                .map(|(name, (count, bytes))| format!("{count:>6} {bytes:>12} {name}"))
                .collect::<Vec<_>>()
                .join("\n");
            std::fs::create_dir_all(dir)?;
            std::fs::write(dir.join(format!("allocations-frame{index:02}.txt")), text)?;
        }
    }
    let world = driver.app_mut().world_mut();
    for _ in 0..8 {
        tiny_glade::screenshot::shared_texture_capture::finish(world)?;
    }
    tiny_glade::screenshot::shared_texture_capture::shutdown(world)?;
    bridge.wait_submits_done(10_000_000_000)?;
    diagnose_completed_retirement(&device, "thirty-world-complete")?;
    if gpu_timing {
        let (stats, samples) = bridge.with_session(|session| {
            let stats = session.backend.poll_gpu_timings()?;
            let samples = session.backend.drain_gpu_timings().collect::<Vec<_>>();
            Ok::<_, rhapsody::Error>((stats, samples))
        })?;
        gpu_samples.push(serde_json::json!({"observed_after_frame": FRAMES, "stats": stats, "samples": samples}));
        let dir = out_dir.as_ref().expect("计时启动前已检查证据目录");
        std::fs::create_dir_all(dir)?;
        // 每项是同一物理队列上单 command 的时间跨度，不能相加当作跨队列帧延迟。
        std::fs::write(dir.join("gpu-command-timings.json"), serde_json::to_vec_pretty(&gpu_samples)?)?;
    }

    // 先保存真实回读证据；后面的任一稳定性断言失败也不能丢失世界图像。
    // 所有原断言与阈值仍在下方执行。
    if let Some(dir) = &out_dir {
        std::fs::create_dir_all(dir)?;
        let artifact_shots = captured.lock().clone();
        for (index, w, h, rgba) in &artifact_shots {
            tiny_glade::screenshot::write_image_with_callback_impl(
                &dir.join(format!("multiframe-frame{:02}.png", index + 1)), *w, *h, rgba, 90,
                &|result| assert!(result.is_ok(), "{result:?}"));
        }
    }

    // ---- report -------------------------------------------------------------------------------
    eprintln!("MULTIFRAME idx counter hist hit compile_ms record_ms submit_ms barriers/trans cmds batches par chunks | update declare compile exec (ms) | shaderheap live/ret/peak  texheap live | constants pages/inflight/creat/reuse | alloc_MiB blocks parked");
    for r in &rows {
        eprintln!(
            "MULTIFRAME {:>2} {:>2} {} {} {:>8.1} {:>8.1} {:>8.1} {}/{} {} {} {} {} | {:>7.1} {:>7.1} {:>7.1} {:>7.1} | {}/{}/{} {} | {}/{}/{}/{} | {:.1} {} {}",
            r.index, r.frame_counter, u8::from(r.history_valid), u8::from(r.stats.plan_cache_hit),
            r.stats.compile_ns as f64 / 1e6, r.stats.record_ns as f64 / 1e6, r.stats.submit_ns as f64 / 1e6,
            r.stats.barrier_commands, r.stats.barrier_transitions, r.stats.commands_recorded,
            r.stats.batches, r.stats.parallel_batches, r.stats.chunks,
            r.update.as_secs_f64() * 1e3, r.declare.as_secs_f64() * 1e3,
            r.compile.as_secs_f64() * 1e3, r.execute.as_secs_f64() * 1e3,
            r.shader_heap.live, r.shader_heap.retiring, r.shader_heap.peak_live, r.texture_heap.live,
            r.constants.pages, r.constants.in_flight_pages, r.constants.page_creations, r.constants.page_reuses,
            r.allocated_bytes as f64 / (1024.0 * 1024.0), r.native_blocks, r.parked_resources,
        );
    }
    if let Some(dir) = &out_dir {
        std::fs::create_dir_all(dir)?;
        let last = rows.last().expect("frames");
        let mut passes = last.passes.clone();
        passes.sort_by_key(|(_, ns, _)| std::cmp::Reverse(*ns));
        let mut text = String::new();
        for (name, ns, count) in &passes {
            text.push_str(&format!("{:>10.3} ms  x{:<3} {}\n", *ns as f64 / 1e6, count, name));
        }
        std::fs::write(dir.join("pass-record-last-frame.txt"), text)?;
    }

    // ---- assertions ---------------------------------------------------------------------------
    for r in &rows {
        assert_eq!(r.frame_counter as usize, r.index, "FrameConstants.frame_counter follows frame_render_returned");
        if r.index >= 1 {
            assert!(r.history_valid, "frame {}: TAA/TGSR color history must be valid after the first frame", r.index);
        }
    }
    for r in rows.iter() {
        eprintln!("MULTIFRAME plan cache after frame {}: hits {} misses {} {}", r.index, r.cache_hits, r.cache_misses, r.cache_diagnostics);
    }
    // Plan cache: the declaration of a steady frame is identical to an earlier one except for the
    // temporal ping-pong parity (SSGI/TAA texture pairs swap physical sides every frame) and the
    // screenshot-capture passes, so the structurally different plans are few and every later frame
    // must hit. Chunk offsets of the dynamic-constants pages are part of the key (buffer access
    // ranges), which only works because used pages are sealed and restart at offset 0. Any other miss
    // reason is a new defect.
    {
        use rhapsody::rg::CacheReason;
        let diagnostics = bridge.with_session(|session| session.chain.plan_cache_stats().diagnostics);
        for reason in [CacheReason::Disabled, CacheReason::UnknownDependencies, CacheReason::DependencyVersion,
            CacheReason::Parameters, CacheReason::ShaderImplementation, CacheReason::ShaderInterface,
            CacheReason::Descriptor, CacheReason::Sharing, CacheReason::DeviceGeneration,
            CacheReason::QueuePolicy, CacheReason::SubgraphInterface, CacheReason::CompilerVersion] {
            assert_eq!(diagnostics.misses(reason), 0, "plan cache miss reason {reason:?}");
        }
        assert_eq!(diagnostics.misses(CacheReason::Cold), 1, "only the very first plan is a cold miss");
        let structural = diagnostics.misses(CacheReason::GraphStructure) + diagnostics.misses(CacheReason::AccessSemantics);
        assert!(structural <= 4, "more than parity x capture structural plan variants: {structural}");
        let stable = rows.iter().skip(6).filter(|r| !r.stats.plan_cache_hit).map(|r| r.index).collect::<Vec<_>>();
        assert!(stable.is_empty(), "plan cache must hit on every frame once all variants are cached; misses at {stable:?}");
    }
    // Descriptor/ring recycling: live counts and ring pages stay flat in the steady state.
    let steady = &rows[2..];
    let live_min = steady.iter().map(|r| r.shader_heap.live).min().unwrap();
    let live_max = steady.iter().map(|r| r.shader_heap.live).max().unwrap();
    assert!(live_max - live_min <= 16, "shader bindless live drifts: {live_min}..{live_max}");
    assert!(steady.iter().all(|r| r.shader_heap.peak_live == steady[0].shader_heap.peak_live || r.shader_heap.peak_live <= steady[0].shader_heap.peak_live + 16),
        "shader bindless peak keeps growing");
    assert!(steady.iter().all(|r| r.texture_heap.live == steady[0].texture_heap.live), "texture heap live drifts");
    assert!(steady.iter().all(|r| r.constants.pages <= 4), "dynamic-constants pages keep growing");
    assert!(steady.last().unwrap().constants.page_reuses > steady[0].constants.page_reuses, "constants pages are never reused");
    // Device memory: a steady state must not grow. Frames right after a screenshot request still
    // hold its readback storage, so compare frames that are neither capture nor capture + 1.
    {
        let clean = |index: usize| {
            !CAPTURES.contains(&index) && (index == 0 || !CAPTURES.contains(&(index - 1)))
        };
        let settled = rows.iter().filter(|r| r.index >= 12 && clean(r.index)).collect::<Vec<_>>();
        let (first, last) = (settled.first().expect("settled frames"), settled.last().expect("settled frames"));
        eprintln!(
            "MULTIFRAME device memory frame {} -> {}: {} -> {} bytes, native blocks {} -> {}",
            first.index, last.index, first.allocated_bytes, last.allocated_bytes, first.native_blocks, last.native_blocks
        );
        assert!(last.allocated_bytes <= first.allocated_bytes, "device memory grows in the steady state: {} -> {}", first.allocated_bytes, last.allocated_bytes);
        assert!(last.native_blocks <= first.native_blocks, "native memory blocks grow in the steady state");
        let peak = settled.iter().map(|r| r.allocated_bytes).max().unwrap();
        assert!(peak <= first.allocated_bytes, "device memory fluctuates upwards in the steady state: {peak}");
    }
    let late_hits = rows.iter().skip(2).filter(|r| r.stats.plan_cache_hit).count();
    eprintln!("MULTIFRAME plan_cache_hit frames>=2: {late_hits}/{}", FRAMES - 2);
    let mut times = rows.iter().skip(2).map(|r| (r.update + r.draw_lists + r.declare + r.compile + r.execute).as_secs_f64() * 1e3).collect::<Vec<_>>();
    times.sort_by(f64::total_cmp);
    eprintln!("MULTIFRAME frame ms (frames>=2): median {:.1} p95 {:.1}", percentile(&times, 0.5), percentile(&times, 0.95));

    let mut shots = captured.lock().clone();
    shots.sort_by_key(|shot| shot.0);
    anyhow::ensure!(shots.len() == CAPTURES.len(), "expected {} readbacks, got {}", CAPTURES.len(), shots.len());
    let lumas = shots.iter().map(|(_, _, _, rgba)| luma(rgba)).collect::<Vec<_>>();
    for (n, (index, w, h, _rgba)) in shots.iter().enumerate() {
        let mean = lumas[n].iter().sum::<f32>() / lumas[n].len() as f32;
        eprintln!("MULTIFRAME shot frame {index}: {w}x{h} mean_luma {mean:.3}");
    }
    for pair in (1..shots.len()).map(|n| (n - 1, n)) {
        let (a, b) = (&lumas[pair.0], &lumas[pair.1]);
        let mean_delta = (a.iter().sum::<f32>() - b.iter().sum::<f32>()).abs() / a.len() as f32;
        let diffs = a.iter().zip(b).map(|(x, y)| (x - y).abs()).collect::<Vec<_>>();
        let mean_abs = diffs.iter().sum::<f32>() / diffs.len() as f32;
        let over16 = diffs.iter().filter(|d| **d > 16.0).count() as f64 / diffs.len() as f64;
        eprintln!(
            "MULTIFRAME stability frames {}->{}: mean_luma_delta {mean_delta:.3} mean_abs_diff {mean_abs:.3} frac(|d|>16) {over16:.5}",
            shots[pair.0].0 + 1, shots[pair.1].0 + 1
        );
        // The picture settles: the last interval (frames 20 -> 30) must be near-identical.
        if pair.1 == shots.len() - 1 {
            assert!(mean_delta < 4.0 && mean_abs < 6.0 && over16 < 0.05,
                "frames {}->{} are not stable: {mean_delta} {mean_abs} {over16}", shots[pair.0].0 + 1, shots[pair.1].0 + 1);
        }
    }
    // Every readback carries real content (not black).
    for (n, l) in lumas.iter().enumerate() {
        assert!(l.iter().sum::<f32>() / l.len() as f32 > 40.0, "readback {n} is (nearly) black");
    }
    Ok(())
}

/// ICC使用真实已加载世界、原final_post和原截图读回；原30帧验收与阈值保持不变。
mod icc_acceptance {
    use super::*;
    use std::sync::Arc;
    use country_core::systems::backend::icc::{ActiveIccProfile, IccProfileChanged};
    use tiny_glade::screenshot::{ScreenshotPurpose, ScreenshotRequest, ScreenshotSink};
    use tiny_glade::production_renderer::HeadlessGameplayFrame;

    type Pixels = (u32, u32, Vec<u8>);

    fn write_profile(path: &std::path::Path, linear: bool) -> Result<()> {
        let profile = if linear {
            let xy = |x, y| lcms2::CIExyY { x, y, Y: 1.0 };
            let primaries = lcms2::CIExyYTRIPLE {
                Red: xy(0.64, 0.33), Green: xy(0.30, 0.60), Blue: xy(0.15, 0.06),
            };
            let curve = lcms2::ToneCurve::new(1.0);
            lcms2::Profile::new_rgb(&xy(0.3127, 0.3290), &primaries, &[&curve; 3])?
        } else {
            lcms2::Profile::new_srgb()
        };
        std::fs::write(path, profile.icc()?)?;
        Ok(())
    }

    fn queue_capture_and_render(
        driver: &mut tiny_glade::game_loop::GameFrameDriver,
        context: &egui::Context,
        bridge: &mut CoreRenderBridge,
    ) -> Result<(HeadlessGameplayFrame, std::sync::mpsc::Receiver<Pixels>)> {
        // dt=0仍运行完整真实App，用其实际ICC事件消费者；不手改Active资源或省略系统。
        context.begin_frame(egui::RawInput::default());
        driver.app_mut().update();
        let _ = context.end_frame();
        let world = driver.app_mut().world_mut();
        let (sender, receiver) = std::sync::mpsc::channel();
        tiny_glade::screenshot::gather_screenshot_requests(world, [ScreenshotRequest {
            purpose: ScreenshotPurpose::MenuBackground,
            dimensions: None,
            sink: ScreenshotSink::Rgba(Arc::new(move |w, h, bytes| {
                let _ = sender.send((w, h, bytes));
            })),
            completion: None,
        }])?;
        let frame = tiny_glade::production_renderer::render_headless_gameplay_frame(bridge, world)?;
        world.resource_mut::<country_core::render::RenderTemporalState>().frame_render_returned();
        Ok((frame, receiver))
    }

    fn finish_capture(
        world: &mut bevy_ecs::world::World,
        receiver: std::sync::mpsc::Receiver<Pixels>,
    ) -> Result<Pixels> {
        // 原remaining=3；第四次finish真实read_bytes等待对应GPU写入，再交实际writer线程。
        for _ in 0..4 { tiny_glade::screenshot::shared_texture_capture::finish(world)?; }
        receiver.recv_timeout(Duration::from_secs(30)).context("ICC original screenshot did not deliver pixels")
    }

    fn assert_final_post_reads_actual_icc(
        bridge: &mut CoreRenderBridge,
        frame: &HeadlessGameplayFrame,
        texture: &rhapsody::resource::ManagedTexture,
    ) -> Result<()> {
        let snapshot = &frame.receipt.snapshot;
        let pass = snapshot.pass_order.iter().position(|name| {
            name == country_core::systems::render_loop::draw_passes::final_post::FINAL_BLIT_PASS_NAME
        }).context("actual world omitted Final blit")?;
        let range = snapshot.pass_ranges.get(pass).context("Final blit has no access span")?;
        let native = texture.shared();
        let logicals = bridge.with_session(|session| {
            snapshot.physical_mapping.iter().filter_map(|mapping| {
                let token = mapping.token?;
                match session.backend.resource_clone(token) {
                    Some(rhapsody::rg::GpuResource::Image(actual)) if Arc::ptr_eq(&actual, &native) => Some(mapping.logical),
                    _ => None,
                }
            }).collect::<Vec<_>>()
        });
        anyhow::ensure!(!logicals.is_empty(), "ICC native owner is absent from the actual world graph");
        anyhow::ensure!(snapshot.accesses[range.clone()].iter().any(|access| {
            logicals.contains(&access.resource_index)
                && access.access_code == country_core::render::pass_builder::draw_builder::ACCESS_SAMPLED_IMAGE
        }), "Final blit did not sample the exact active ICC texture owner");
        Ok(())
    }

    fn rgb_delta(a: &Pixels, b: &Pixels) -> Result<f64> {
        anyhow::ensure!((a.0, a.1, a.2.len()) == (b.0, b.1, b.2.len()), "ICC captures have different dimensions");
        Ok(a.2.chunks_exact(4).zip(b.2.chunks_exact(4)).map(|(a, b)| {
            (0..3).map(|i| f64::from(a[i].abs_diff(b[i]))).sum::<f64>()
        }).sum::<f64>() / (f64::from(a.0) * f64::from(a.1) * 3.0))
    }

    #[test]
    fn shipped_session_icc_some_replacement_cancel_and_final_post_pixels() -> Result<()> {
        assert_eq!(lcms2::version(), 2150, "ICC acceptance must use the pinned native algorithm");
        let BootedSession { mut driver, context, mut bridge, shaders, device, _boot } = boot_shipped_session()?;
        // 隔离ICC对像素的影响：真实App继续更新，只有时间步长归零，不删除业务系统。
        driver.app_mut().insert_resource(bevy_time::TimeUpdateStrategy::ManualDuration(Duration::ZERO));
        let inputs = tempfile::tempdir()?;
        let linear_path = inputs.path().join("linear-display.icc");
        // 存盘sRGB含ICC固定点量化，属于恢复参考，不假设与内存sRGB逐位恒等。
        let identity_path = inputs.path().join("serialized-srgb-display.icc");
        write_profile(&linear_path, true)?;
        write_profile(&identity_path, false)?;
        let mut old_native = None::<std::sync::Weak<rhapsody::resource::Texture>>;
        let outcome = (|| -> Result<()> {
            let world = driver.app_mut().world_mut();
            anyhow::ensure!(world.resource::<ActiveIccProfile>().texture_and_device().is_none(), "default ICC was fabricated");
            tiny_glade::production_renderer::warm_up_gameplay(bridge.warmup_owners()?, world)?;
            bridge.accept_gameplay_warmup(world.resource::<tiny_glade::render_bridge::CompletedGameplayWarmup>())?;

            let (baseline_frame, receiver) = queue_capture_and_render(&mut driver, &context, &mut bridge)?;
            let baseline = finish_capture(driver.app_mut().world_mut(), receiver)?;
            bridge.wait_submits_done(30_000_000_000)?;
            drop(baseline_frame);

            driver.app_mut().world_mut().send_event(IccProfileChanged(linear_path.clone()));
            let (linear_frame, receiver) = queue_capture_and_render(&mut driver, &context, &mut bridge)?;
            let old_handle = driver.app().world().resource::<ActiveIccProfile>()
                .texture_and_device().context("linear profile did not create a real ICC LUT")?.0.clone();
            let managed = bridge.hosts().textures.get(&old_handle).context("linear ICC host owner missing")?;
            assert_final_post_reads_actual_icc(&mut bridge, &linear_frame, &managed)?;
            old_native = Some(Arc::downgrade(&managed.shared()));
            drop(managed);

            // 在线性帧已提交但尚未finish/collect时切换；旧slot注销不等于可以销毁图持有的资源。
            driver.app_mut().world_mut().send_event(IccProfileChanged(identity_path.clone()));
            context.begin_frame(egui::RawInput::default());
            driver.app_mut().update();
            let _ = context.end_frame();
            let new_handle = driver.app().world().resource::<ActiveIccProfile>()
                .texture_and_device().context("replacement ICC profile missing")?.0.clone();
            anyhow::ensure!(new_handle.id != old_handle.id, "ICC replacement reused a live host identity");
            anyhow::ensure!(bridge.hosts().textures.get(&old_handle).is_none(), "old ICC host slot was not released");
            anyhow::ensure!(old_native.as_ref().unwrap().upgrade().is_some(), "submitted frame lost the old native ICC owner");
            let linear = finish_capture(driver.app_mut().world_mut(), receiver)?;
            bridge.wait_submits_done(30_000_000_000)?;
            drop(linear_frame);

            let submitted_before = device.retirement_stats().submitted;
            let (_, built) = bridge.build_frame(|frame| {
                let imported = frame.import_texture(&new_handle, 0x1d)?;
                frame.with_graph(|graph| { graph.export(imported, 0x1d)?; Ok(()) })
            })?;
            let compiled = bridge.compile_built(built)?;
            drop(compiled); // 真实cancel_prepared；不能提交空替代帧或改Active。
            anyhow::ensure!(device.retirement_stats().submitted == submitted_before, "ICC cancellation submitted GPU work");
            anyhow::ensure!(driver.app().world().resource::<ActiveIccProfile>()
                .texture_and_device().is_some_and(|(texture, _)| texture.id == new_handle.id),
                "cancelled graph removed the active ICC profile");
            anyhow::ensure!(bridge.hosts().textures.get(&new_handle).is_some(), "cancelled graph destroyed the active host owner");

            let (restored_frame, receiver) = queue_capture_and_render(&mut driver, &context, &mut bridge)?;
            let current = bridge.hosts().textures.get(&new_handle).context("restored ICC host owner missing")?;
            assert_final_post_reads_actual_icc(&mut bridge, &restored_frame, &current)?;
            drop(current);
            let restored = finish_capture(driver.app_mut().world_mut(), receiver)?;
            bridge.wait_submits_done(30_000_000_000)?;
            drop(restored_frame);
            bridge.collect_completed_frames()?;

            let linear_delta = rgb_delta(&baseline, &linear)?;
            let restored_delta = rgb_delta(&baseline, &restored)?;
            eprintln!("ICC_WORLD_PIXELS extent={}x{} nonidentity_rgb_mad={linear_delta:.6} restored_serialized_srgb_rgb_mad={restored_delta:.6}", baseline.0, baseline.1);
            if let Some(directory) = std::env::var_os("TG_ICC_OUT").map(PathBuf::from) {
                std::fs::create_dir_all(&directory)?;
                std::fs::copy(&linear_path, directory.join("linear-display.icc"))?;
                std::fs::copy(&identity_path, directory.join("srgb-display.icc"))?;
                for (name, pixels) in [("default.png", &baseline), ("linear.png", &linear), ("restored.png", &restored)] {
                    let written = std::cell::RefCell::new(None);
                    tiny_glade::screenshot::write_image_with_callback_impl(&directory.join(name), pixels.0, pixels.1, &pixels.2, 90,
                        &|result| *written.borrow_mut() = Some(result));
                    written.into_inner().context("screenshot writer omitted its callback")?
                        .map_err(anyhow::Error::msg)?;
                }
            }
            anyhow::ensure!(linear_delta > 10.0, "nonidentity ICC did not materially change the actual world color");
            anyhow::ensure!(restored_delta < linear_delta * 0.25, "serialized-sRGB replacement did not restore original world colors");
            Ok(())
        })();
        // 失败先保留，再执行真实GPU/截图/保存线程清理；cleanup不能把失败转换成通过。
        let cleanup = bridge.wait_submits_done(30_000_000_000);
        let screenshots = tiny_glade::screenshot::shared_texture_capture::shutdown(driver.app_mut().world_mut());
        let save = country_core::systems::save::save_thread::shutdown_save_thread(driver.app_mut().world_mut());
        drop(driver);
        drop(bridge);
        drop(shaders);
        drop(context);
        outcome?;
        cleanup?;
        screenshots?;
        save?;
        anyhow::ensure!(old_native.as_ref().is_some_and(|owner| owner.upgrade().is_none()),
            "old ICC native owner remained after both App and graph/cache owners were dropped");
        eprintln!("PASS: actual shipped world ICC Some, nonidentity color, replacement, compiled cancellation and owner teardown");
        Ok(())
    }
}
