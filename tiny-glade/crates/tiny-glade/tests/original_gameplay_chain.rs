//! Real gameplay App, shipped initial session and the production device tuple.
//! No individual systems are removed or replaced to make this fixture pass.
//! A temporary account prevents writes to the player's actual saves.
#[path = "support/mod.rs"]
mod test_support;
use test_support::BridgeTestDriver as _;
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

/// Native terrain attachment coordinates and these three decorator scalar
/// slots are f32 (snapshot::DecoratorInfoJson). Compare all 32 bits after a
/// JSON round trip; preserve exact comparisons for other numbers and keys.
fn is_native_decorator_f32_path(path: &str) -> bool {
    let Some((_, suffix)) = path.split_once("/decorators/diffs/") else {
        return false;
    };
    let parts: Vec<_> = suffix.split('/').collect();
    let (row, phase, id) = match parts.as_slice() {
        [row, "1", phase, id, "dst", "Terrain", "pos_xz", "0" | "1"]
        | [row, "1", phase, id, "dst", "Terrain", "yaw_radians"]
        | [row, "1", phase, id, "dst", "Roof", "mesh_uv", "0" | "1"]
        | [row, "1", phase, id, "dst", "Roof", "relative_yaw_radians"]
        | [row, "1", phase, id, "dst", "Stair", "uv", "0" | "1"]
        | [row, "1", phase, id, "dst", "Stair", "yaw_radians"]
        | [row, "1", phase, id, "dst", "Wall", "coord", "x" | "y"]
        | [row, "1", phase, id, "dst", "Wall", "anchor", "anchor_ws", "0" | "1" | "2"]
        | [row, "1", phase, id, "dst", "Wall", "anchor", "wall_height_relative_y"]
        | [row, "1", phase, id, "h_offset" | "lantern_brightness" | "stair_width"] => (*row, *phase, *id),
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
        (Value::Number(_), Value::Number(_)) if is_native_decorator_f32_path(path) => {
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
        "fixture/decorators/diffs/0/1/after/0/dst/Roof/mesh_uv/2",
        "fixture/other/dst/Terrain/pos_xz/0",
    ] {
        assert!(!is_native_decorator_f32_path(path));
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
        "after/0/dst/Roof/mesh_uv/0",
        "after/0/dst/Roof/relative_yaw_radians",
        "after/0/dst/Stair/uv/1",
        "after/0/dst/Stair/yaw_radians",
        "after/0/dst/Wall/coord/x",
        "after/0/dst/Wall/anchor/anchor_ws/2",
        "after/0/dst/Wall/anchor/wall_height_relative_y",
        "after/0/h_offset",
        "after/0/lantern_brightness",
        "after/0/stair_width",
    ] {
        let path = format!("fixture/decorators/diffs/0/1/{suffix}");
        assert!(is_native_decorator_f32_path(&path));
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

/// `wait_frame` only accepts the session's oldest pending frame. Frames that
/// `collect_completed_frames` already retired are stale at the front of `in_flight`
/// (whether a frame has finished by the time pacing runs depends on GPU speed), and
/// the pending frames are always the newest `pending` receipts.
fn wait_oldest_pending(
    bridge: &mut CoreRenderBridge,
    in_flight: &mut std::collections::VecDeque<u64>,
) -> Result<()> {
    let pending = bridge.with_session(|session| session.pending_frame_count());
    while in_flight.len() > pending {
        in_flight.pop_front();
    }
    let oldest = in_flight.pop_front().context("pending frame without receipt")?;
    bridge.with_session(|session| session.wait_frame(oldest, 120_000_000_000))?;
    Ok(())
}

fn boot_shipped_session() -> Result<BootedSession> {
    boot_shipped_session_with_nani_readback(false)
}

fn boot_shipped_session_with_nani_readback(diagnostic_nani: bool) -> Result<BootedSession> {
    struct Diagnostics;
    impl log::Log for Diagnostics {
        fn enabled(&self, metadata: &log::Metadata<'_>) -> bool { metadata.level() <= log::Level::Error }
        fn log(&self, record: &log::Record<'_>) {
            if self.enabled(record.metadata()) { eprintln!("{} {}", record.target(), record.args()); }
        }
        fn flush(&self) {}
    }
    static DIAGNOSTICS: Diagnostics = Diagnostics;
    if log::set_logger(&DIAGNOSTICS).is_ok() { log::set_max_level(log::LevelFilter::Error); }

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
    let mut app = run_game_thread(config, &cli, {
        // 只用于出高分辨率验证图：TG_TEST_SIZE=宽x高，默认 480x270。
        let (w, h) = std::env::var("TG_TEST_SIZE")
            .ok()
            .and_then(|v| v.split_once('x').and_then(|(w, h)| Some((w.parse().ok()?, h.parse().ok()?))))
            .unwrap_or((480, 270));
        WindowSize::new(w, h)
    })?;
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
    // 独立诊断用例仅追加读回capability；仍使用同一BufferHost和真实GPU owner。
    let diagnostic = diagnostic_nani.then(|| std::sync::Arc::new(
        M6NaniDiagnosticDevice(bridge.hosts().buffers.clone())));
    if let Some(device) = diagnostic.as_ref() {
        app.insert_resource(country_core::render::nani::gpu_world::GpuWorld::new(device.as_ref()));
    }
    tiny_glade::gameplay_startup::install_gameplay_device_runtime(
        &mut app,
        device.clone(),
        bridge.hosts().clone(),
        installed,
    )?;
    if let Some(diagnostic) = diagnostic {
        app.insert_resource(country_core::render::device::GraphicsDeviceResource::new(
            country_core::rhapsody_contract::GraphicsDevice::new(device.clone())
                .with_buffer_host(diagnostic)));
    }
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
    shipped_session_production_graph(false, false)
}

#[test]
fn m06_wall_color_edit_reaches_plaster_and_actual_production_graph() -> Result<()> {
    shipped_session_production_graph(true, false)
}

#[test]
fn m06_diagnostic_plaster_indirect_command_contains_edited_wall() -> Result<()> {
    shipped_session_production_graph(true, true)
}

fn shipped_session_production_graph(edit_plaster: bool, diagnostic_nani: bool) -> Result<()> {
    shipped_session_production_graph_with_building(edit_plaster, diagnostic_nani, false)
}

#[test]
fn m06_created_building_reaches_half_timber_plaster_and_production_frame() -> Result<()> {
    shipped_session_production_graph_with_building(true, true, true)
}

fn shipped_session_production_graph_with_building(
    edit_plaster: bool, diagnostic_nani: bool, create_building: bool,
) -> Result<()> {
    let BootedSession { mut driver, context, mut bridge, shaders, device, _boot } =
        boot_shipped_session_with_nani_readback(diagnostic_nani)?;
    let created_wall = if create_building {
        use country_core::{resources::{history::History, walls::public_walls::PublicWalls},
            systems::wall::{shapes::CreateRectangleCmd, SetWallStyleCmd}};
        let before: std::collections::HashSet<_> = driver.app().world()
            .resource::<InnerWalls>().wall_ids().collect();
        // 创建命令来自正式工具的同一事件类型；这里只建立其输入事务，
        // 不声称覆盖矩形工具的真实OS交互，也不直接写入公开墙/网格/屋顶。
        driver.app_mut().world_mut().resource_mut::<History>().start_input_recording(0)
            .map_err(|error| anyhow::anyhow!("建筑验收事务不能开始: {error:?}"))?;
        driver.app_mut().world_mut().send_event(CreateRectangleCmd {
            center: glam::Vec2::ZERO, dimensions: glam::Vec2::new(6.0, 4.0),
            orientation: glam::Vec2::X, previous_wall: None, height: 4.0, elevation: 1.5,
        });
        for _ in 0..20 {
            context.begin_frame(egui::RawInput::default());
            driver.app_mut().update();
            let _ = context.end_frame();
        }
        let id = driver.app().world().resource::<InnerWalls>().wall_ids()
            .find(|id| !before.contains(id)).context("正式矩形命令没有创建墙")?;
        let from = driver.app().world().resource::<InnerWalls>().get_unwrapped(id).snapshot().source.style;
        driver.app_mut().world_mut().send_event(SetWallStyleCmd {
            wall_id: id, from, to: 3, record_history: 0, play_sound: 0,
        });
        for _ in 0..20 {
            context.begin_frame(egui::RawInput::default());
            driver.app_mut().update();
            let _ = context.end_frame();
        }
        let world = driver.app().world();
        let public = world.resource::<PublicWalls>();
        anyhow::ensure!(public.roof_entity(id).is_some(), "矩形创建没有形成真实屋顶关联");
        let library = world.resource::<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>();
        let storage = library.get::<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>()
            .context("正式半木木板SSBO缺失")?;
        let output = world.resource::<system_wall_constructor::construct_arches::WallConstructorOutput>();
        let allocation = output.gpu.half_timber_allocation(id.0).context("正式半木父链没有创建独立owner")?;
        let ranges = storage.allocation_to_draw_multi_instanced(allocation);
        let count: u64 = ranges.iter().map(|(_, count)| u64::from(*count)).sum();
        anyhow::ensure!(count > 0, "正式半木owner没有非空GPU可绘制范围");
        eprintln!("M06 created building: wall={} half_timber_instances={count} ranges={}", id.0, ranges.len());
        Some(id)
    } else { None };
    let plaster_wall = if edit_plaster {
        use country_core::resources::{color_id::{SetWallColorIdCmd, WallColorIds},
            walls::{plaster::Plaster, public_walls::{PublicWalls, WallPlasters}}};
        let wall = {
            let world = driver.app().world();
            created_wall.or_else(|| world.resource::<InnerWalls>().wall_ids().find(|id| world.resource::<PublicWalls>()
                .get(*id).is_some_and(|wall| !wall.authority.is_special()))
            )
                .context("原会话没有可用于抹灰编辑验收的墙")?
        };
        // 合成命令走正式setter／事件／生成器；不手造网格、颜色表或实体。
        if create_building { m6_observe_plaster_owners(driver.app_mut().world_mut(), "before-color"); }
        driver.app_mut().world_mut().send_event(SetWallColorIdCmd::new(wall, 4, 1));
        for _ in 0..16 {
            context.begin_frame(egui::RawInput::default());
            driver.app_mut().update();
            let _ = context.end_frame();
        }
        if create_building { m6_observe_plaster_owners(driver.app_mut().world_mut(), "after-color"); }
        let world = driver.app().world();
        assert_eq!(world.resource::<WallColorIds>().0[wall.0 as usize], 4);
        let entities = world.resource::<WallPlasters>().0.get(&wall)
            .context("正式颜色命令没有到达抹灰owner")?;
        let count = entities.iter().flatten().filter(|entity| world.get::<Plaster>(**entity).is_some()).count();
        anyhow::ensure!(count > 0, "抹灰owner没有实际非空生成实体");
        eprintln!("M06 actual color command -> plaster entities: wall={} count={count}", wall.0);
        Some(wall)
    } else { None };
    if let Some(id) = created_wall {
        use country_core::{resources::color_id::{HalftimberColorIds, SetHalftimberColorIdCmd},
            systems::save::save_game::{SaveGameCmd, SavingGameComplete}};
        // This tests the actual setter and snapshot persistence only. A setter event
        // does not record color history; the real menu path must cover undo/redo.
        driver.app_mut().world_mut().send_event(SetHalftimberColorIdCmd::new(id, 5, 1));
        for _ in 0..8 {
            context.begin_frame(egui::RawInput::default()); driver.app_mut().update(); let _ = context.end_frame();
        }
        anyhow::ensure!(driver.app().world().resource::<HalftimberColorIds>().0[id.0 as usize] == 5,
            "正式半木颜色命令没有更新权威颜色表");
        let session = driver.app().world().resource::<ActiveSession>().id;
        driver.app_mut().world_mut().send_event(SaveGameCmd);
        let mut saved = false;
        for _ in 0..600 {
            context.begin_frame(egui::RawInput::default()); driver.app_mut().update(); let _ = context.end_frame();
            if driver.app_mut().world_mut().resource_mut::<bevy_ecs::event::Events<SavingGameComplete>>()
                .drain().any(|event| event.path == session.save_folder_path()) { saved = true; break; }
        }
        anyhow::ensure!(saved, "半木颜色生产存档worker未完成");
    }
    let world = driver.app_mut().world_mut();
    if let Some(id) = created_wall {
        use country_core::{resources::walls::{Floor, public_walls::PublicWalls},
            render::nani::gpu_world::{InstanceKey, GpuWorldSubsetKind}};
        let published = world.resource::<PublicWalls>().get(id).context("建筑公开墙缺失")?;
        anyhow::ensure!(published.authority.elevation() == 1.5, "建筑权威海拔不一致");
        let mut floors = world.query::<(&Floor, &InstanceKey)>();
        let matching: Vec<_> = floors.iter(world).filter(|(floor, _)| floor.wall_id == id).collect();
        anyhow::ensure!(matching.len() == 1 && matching[0].1.kind == GpuWorldSubsetKind::RoofFloor,
            "建筑楼板未进入真实RoofFloor Nani owner");
        eprintln!("M06 building wall={} elevation=1.5 floor EntityKey={:?}", id.0, matching[0].1);
    }
    let expected_walls = if create_building { 4 } else { 3 };
    assert_eq!(world.resource::<InnerWalls>().wall_ids().count(), expected_walls);
    assert_nonempty_wall_owners(world, shaders.as_ref(), &bridge, expected_walls);
    // Preserve downstream failures in the acceptance log. A successful graph
    // export alone cannot establish visible wall construction or full play.
    eprintln!(
        "wall construction residuals: {:?}",
        world
            .resource::<country_core::systems::wall::construct_wall_bricks::WallBrickBuildState>()
            .failures
    );
    {
        let clutter = world.resource::<system_clutter::autoclutter::AutoClutterState>();
        eprintln!("actual App clutter: regions={} region_candidates={} output_candidates={}",
            clutter.regions.len(), clutter.region_candidates.len(), clutter.spawn_candidates.len());
        // 初始场景可以没有杂物；明确记录实际owner，不能由墙砖／像素推断杂物已验收。
        let mut live = world.query::<&country_core::components::drawable::AutoClutter>();
        eprintln!("actual App clutter entities={}", live.iter(world).count());
    }
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
    // The original worker pre-renders 32x32, then transfers its real completed
    // history owners. Content acceptance belongs to a normal-resolution frame,
    // not a reduced warmup screenshot with at most 1024 distinct pixels.
    tiny_glade::production_renderer::warm_up_gameplay(bridge.warmup_owners()?, world)
        .context("actual production warmup graph compile, execute and exported owner")?;
    bridge.accept_gameplay_warmup(
        world.resource::<tiny_glade::render_bridge::CompletedGameplayWarmup>(),
    )?;
    if let Some(id) = created_wall {
        // Exercise the production frame and its actual owners without a performance loop.
        for index in 0..3 {
            context.begin_frame(egui::RawInput::default());
            driver.app_mut().update();
            let _ = context.end_frame();
            let world = driver.app_mut().world_mut();
            let frame = tiny_glade::production_renderer::prepare_headless_gameplay_frame(&mut bridge, world)
                .with_context(|| format!("M06建筑功能帧{index}"))?;
            bridge.prepare_test_execution()?;
            let completed = frame.execute(&mut bridge, world)?;
            world.resource_mut::<country_core::render::RenderTemporalState>().frame_render_returned();
            bridge.with_session(|session| session.wait_frame(completed.receipt.frame, 120_000_000_000))?;
            drop(completed);
            bridge.collect_completed_frames()?;
            anyhow::ensure!(world.resource::<system_wall_constructor::construct_arches::WallConstructorOutput>()
                .gpu.half_timber_allocation(id.0).is_some(), "建筑功能帧丢失半木owner");
            anyhow::ensure!(world.resource::<country_core::resources::walls::public_walls::WallPlasters>()
                .0.get(&id).is_some_and(|entities| entities.iter().flatten().next().is_some()),
                "建筑功能帧丢失抹灰owner");
        }
    }
    context.begin_frame(egui::RawInput::default());
    driver.app_mut().update();
    let _ = context.end_frame();
    let world = driver.app_mut().world_mut();
    let display = world.resource::<country_core::render::render_config::FrameRenderConfig>().extent_0;
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
    let frame = tiny_glade::production_renderer::prepare_headless_gameplay_frame(&mut bridge, world)
        .context("actual normal-resolution world graph after warmup adoption")?;
    bridge.prepare_test_execution()?;
    let frame = frame.execute(&mut bridge, world)?;
    world.resource_mut::<country_core::render::RenderTemporalState>().frame_render_returned();
    bridge.with_session(|session| session.wait_frame(frame.receipt.frame, 120_000_000_000))?;
    drop(frame);
    bridge.collect_completed_frames()?;
    if diagnostic_nani {
        let observed = observe_plaster_indirect(&device, bridge.hosts().buffers.as_ref(), world,
            plaster_wall.expect("抹灰诊断必须包含正式颜色编辑"))?;
        eprintln!("M06 diagnostic descriptor variant: {observed:?}");
        let initial = probe_plaster_initial_cull(&mut bridge, &device, world,
            plaster_wall.expect("抹灰诊断必须包含正式颜色编辑"))?;
        eprintln!("M06 independent initial-cull diagnostic, no raster: {initial:?}");
        if create_building { probe_plaster_raster_depth(&mut bridge, &device, world, shaders.as_ref())?; }
    }
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
    assert_eq!([w, h], display, "content capture must use the normal display target");
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
    if create_building {
        use country_core::{components::KeepOnLevelUnload,
            resources::walls::plaster::Plaster,
            render::nani::gpu_world::{GpuWorld, GpuWorldSubsetKind},
            meta_commander::{MetaCommand, MetaCommander}};
        // 包括原重复Create后已不在WallPlasters表中的实体；最终卸载必须走
        // 原全局Without<KeepOnLevelUnload>查询，不能只检查当前映射。
        let mut query = world.query_filtered::<bevy_ecs::entity::Entity,
            bevy_ecs::query::Or<(bevy_ecs::query::With<Plaster>,
                bevy_ecs::query::With<system_wall_constructor::plaster_systems::PlasterBufferOwner>)>>();
        let old: Vec<_> = query.iter(world).collect();
        anyhow::ensure!(!old.is_empty(), "建筑卸载验收没有真实抹灰owner");
        for entity in &old {
            anyhow::ensure!(world.get::<KeepOnLevelUnload>(*entity).is_none(),
                "抹灰实体带有跨关卡保留标记: {entity:?}");
        }
        let session = world.resource::<ActiveSession>().id;
        world.resource_mut::<MetaCommander>().extend_script([MetaCommand::load_session(session, false)]);
        let mut unloaded = false;
        let mut reentered = false;
        for _ in 0..1200 {
            context.begin_frame(egui::RawInput::default());
            driver.app_mut().update();
            let _ = context.end_frame();
            let world = driver.app_mut().world_mut();
            if !unloaded && *world.resource::<bevy_state::prelude::State<country_core::app_loading_state::AppLoadingState>>()
                .get() == country_core::app_loading_state::AppLoadingState::Reload {
                for entity in &old {
                    anyhow::ensure!(world.get_entity(*entity).is_err(),
                        "正式卸载遗漏旧抹灰实体/缓冲owner: {entity:?}");
                }
                anyhow::ensure!(world.resource::<GpuWorld>().owned_subset_mut(GpuWorldSubsetKind::Plaster)
                    .instance_count() == 0, "正式卸载后仍保留抹灰GPU实例");
                anyhow::ensure!(world.resource::<system_wall_constructor::construct_arches::WallConstructorOutput>()
                    .gpu.half_timber_allocation(created_wall.unwrap().0).is_none(),
                    "正式卸载遗漏半木独立owner");
                eprintln!("M06 actual reload retired all {} plaster/owner entities and cleared plaster/half timber GPU owners", old.len());
                unloaded = true;
            }
            if unloaded && *world.resource::<bevy_state::prelude::State<country_core::app_loading_state::AppLoadingState>>()
                .get() == country_core::app_loading_state::AppLoadingState::Game
                && world.resource::<LoadGameManager>().options.is_none()
                && world.resource::<country_core::resources::history::ReplayBuffer>().is_empty()
                && world.resource::<InnerWalls>().wall_ids().next().is_some() {
                reentered = true;
                break;
            }
        }
        anyhow::ensure!(unloaded, "正式会话命令未进入Reload卸载阶段");
        anyhow::ensure!(reentered, "正式Reload后未恢复新会话");
        for index in 0..8 {
            context.begin_frame(egui::RawInput::default());
            driver.app_mut().update();
            let _ = context.end_frame();
            let world = driver.app_mut().world_mut();
            let frame = tiny_glade::production_renderer::prepare_headless_gameplay_frame(&mut bridge, world)
                .with_context(|| format!("M06卸载重入后生产渲染帧{index}"))?;
            bridge.prepare_test_execution()?;
            let completed = frame.execute(&mut bridge, world)?;
            world.resource_mut::<country_core::render::RenderTemporalState>().frame_render_returned();
            bridge.with_session(|session| session.wait_frame(completed.receipt.frame, 120_000_000_000))?;
            drop(completed);
            bridge.collect_completed_frames()?;
        }
        let world = driver.app().world();
        let id = created_wall.expect("created building must have a wall");
        anyhow::ensure!(world.resource::<country_core::resources::color_id::HalftimberColorIds>()
            .0[id.0 as usize] == 5, "正式重载未恢复半木颜色");
        let output = world.resource::<system_wall_constructor::construct_arches::WallConstructorOutput>();
        let allocation = output.gpu.half_timber_allocation(id.0)
            .context("正式重载后半木owner缺失")?;
        let storage = world.resource::<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>()
            .get::<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>()
            .context("正式重载后半木SSBO缺失")?;
        anyhow::ensure!(storage.allocation_to_draw_multi_instanced(allocation).iter()
            .any(|(_, count)| *count > 0), "正式重载后半木几何为空");
        eprintln!("M06 saved half-timber color and nonempty geometry restored after Reload");
        eprintln!("M06 actual Reload->Game resumed 8 production GPU frames");
    }
    Ok(())
}

// Central acceptance fragment. Merge after real startup producers are installed.
// This checks actual loaded state, per-wall storage and global draw admission.
fn assert_nonempty_original_walls(
    world: &mut bevy_ecs::world::World,
    shaders: &country_core::asset_libraries::shader_library::AssetShaderLibrary,
    bridge: &CoreRenderBridge,
) {
    assert_nonempty_wall_owners(world, shaders, bridge, 3);
}

fn assert_nonempty_wall_owners(
    world: &mut bevy_ecs::world::World,
    shaders: &country_core::asset_libraries::shader_library::AssetShaderLibrary,
    bridge: &CoreRenderBridge,
    expected_walls: usize,
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
    assert_eq!(walls.len(), expected_walls, "权威墙数量与验收场景不一致");
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
    shader_heap: rhapsody::bindless::BindlessOccupancy,
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
    // Construct all thread-local graph owners here, as the production worker
    // does. An opt-in parallel run must exercise actual shared-pool Collect.
    std::thread::Builder::new().name("tg-render".into())
        .spawn(run_shipped_session_steady_state_multi_frame)?
        .join().unwrap_or_else(|panic| std::panic::resume_unwind(panic))
}

fn run_shipped_session_steady_state_multi_frame() -> Result<()> {
    let collect_probe_enabled = std::env::var("TG_FRAME_TIMING").is_ok_and(|value| value == "1");
    let require_cross_pass = std::env::var_os("TG_REQUIRE_CROSS_PASS").is_some();
    anyhow::ensure!(!require_cross_pass || collect_probe_enabled, "Collect task acceptance requires TG_FRAME_TIMING=1");
    let frames: usize = std::env::var("TG_MULTIFRAME_FRAMES").ok()
        .map(|value| value.parse()).transpose()?.unwrap_or(30);
    anyhow::ensure!((30..=100_000).contains(&frames), "TG_MULTIFRAME_FRAMES must be 30..=100000");
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
    let _ = country_core::render::declare_probe::take();
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
    let mut allocation_peak_dumped = false;
    for index in 0..frames {
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
            wait_oldest_pending(&mut bridge, &mut in_flight)?;
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
        let frame = tiny_glade::production_renderer::prepare_headless_gameplay_frame(&mut bridge, world)
            .with_context(|| {
                format!(
                    "steady-state frame {index}: constants {:?}, shader heap {:?}, pending {}",
                    { let stats = bridge.raster_resources().dynamic_constants().lock().stats(); stats },
                    bridge.shader_backend().bindless_occupancy(),
                    bridge.with_session(|session| session.pending_frame_count()),
                )
            })?;
        bridge.prepare_test_execution()?;
        let frame = frame.execute(&mut bridge, world)?;
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
            bridge.shader_backend().bindless_occupancy(),
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
            shader_heap: bridge.shader_backend().bindless_occupancy(),
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
        let first_memory_growth = index > 12 && !allocation_peak_dumped
            && !CAPTURES.contains(&index) && !CAPTURES.contains(&(index - 1))
            && rows[index].allocated_bytes > rows[12].allocated_bytes;
        if let (Some(dir), true) = (&out_dir, index == 5 || index == 12 || index == frames - 1 || first_memory_growth) {
            allocation_peak_dumped |= first_memory_growth;
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
    // Recording page distribution and GPU overlap legitimately change the
    // instantaneous high-water mark. Drain real completion, then account for
    // every surviving descriptor through its intended frame-global/sampler
    // owner. A strong residency index or leaked past frame fails exactly here.
    let collection = country_core::render::declare_probe::take();
    assert!(rows.iter().all(|row| row.stats.chunks > 0), "native recording must execute shared-pool chunks");
    if collect_probe_enabled {
        assert!(collection.collect_tasks > 0, "enabled Collect probe recorded no shared-pool tasks");
        eprintln!("MULTIFRAME actual shared-pool Collect jobs: {}, cross-pass jobs: {}", collection.collect_tasks, collection.cross_pass_tasks);
        if require_cross_pass {
            assert!(collection.cross_pass_tasks > 0, "cross-pass acceptance ran only single-pass Collect");
        }
    }
    let drained = bridge.shader_backend().bindless_occupancy();
    let expected = bridge.shader_backend().retained_bindless_counts();
    let actual = drained.tables.map(|table| table.live);
    eprintln!("MULTIFRAME drained descriptor owners: actual={actual:?}, expected={expected:?}");
    assert_eq!(actual, expected, "descriptors outlive their intended owners after actual GPU completion");
    assert_eq!(drained.live, expected.iter().sum::<usize>());
    // Grass blades survive the steady state. The ground carries a fine blade texture only while
    // the blade draws land (see `plain_indirect_needs_count_form`); a bare ground, blades dropped
    // after the first frame, loses most of its horizontal luma gradient in the lower picture.
    // Checked before the timing-sensitive resource assertions so a failure names its own cause.
    for (index, w, h, rgba) in captured.lock().iter() {
        let energy = lower_gradient_energy(rgba, *w, *h);
        eprintln!("MULTIFRAME ground detail frame {}: {energy:.2}", index + 1);
        // Only the converged frames (20 and 30) separate the two pictures at 480x270: with the
        // blades 2.5 against 1.5 without; earlier frames still carry TAA's unsettled noise.
        assert!(
            *index < 19 || energy >= MIN_GROUND_DETAIL,
            "frame {}: the grass blades are gone (ground detail {energy:.2})",
            index + 1
        );
    }
    diagnose_completed_retirement(&device, "thirty-world-complete")?;
    if gpu_timing {
        let (stats, samples) = bridge.with_session(|session| {
            let stats = session.backend.poll_gpu_timings()?;
            let samples = session.backend.drain_gpu_timings().collect::<Vec<_>>();
            Ok::<_, rhapsody::Error>((stats, samples))
        })?;
        gpu_samples.push(serde_json::json!({"observed_after_frame": frames, "stats": stats, "samples": samples}));
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
    eprintln!("MULTIFRAME idx counter hist hit compile_ms record_ms submit_ms barriers/trans cmds batches par chunks | update declare compile exec (ms) | shaderpool sampled_live  texheap live | constants pages/inflight/creat/reuse | alloc_MiB blocks parked");
    for r in &rows {
        eprintln!(
            "MULTIFRAME {:>2} {:>2} {} {} {:>8.1} {:>8.1} {:>8.1} {}/{} {} {} {} {} | {:>7.1} {:>7.1} {:>7.1} {:>7.1} | {} {} | {}/{}/{}/{} | {:.1} {} {}",
            r.index, r.frame_counter, u8::from(r.history_valid), u8::from(r.stats.plan_cache_hit),
            r.stats.compile_ns as f64 / 1e6, r.stats.record_ns as f64 / 1e6, r.stats.submit_ns as f64 / 1e6,
            r.stats.barrier_commands, r.stats.barrier_transitions, r.stats.commands_recorded,
            r.stats.batches, r.stats.parallel_batches, r.stats.chunks,
            r.update.as_secs_f64() * 1e3, r.declare.as_secs_f64() * 1e3,
            r.compile.as_secs_f64() * 1e3, r.execute.as_secs_f64() * 1e3,
            r.shader_heap.live, r.texture_heap.live,
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
        use rhapsody::collections::cache::CacheReason;
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
        // Capture stops after frame 4. Both temporal parities of the graph
        // without capture first appear at frames 5 and 6.
        let stable = rows.iter().skip(7).filter(|r| !r.stats.plan_cache_hit).map(|r| r.index).collect::<Vec<_>>();
        assert!(stable.is_empty(), "plan cache must hit on every frame once all variants are cached; misses at {stable:?}");
    }
    // Descriptor/ring recycling: live counts and ring pages stay flat in the steady state.
    // Serial cold recording switches to up to eight recording pages after
    // cache warmup; the last capture branch is exercised at frame 9.
    let steady = &rows[10..];
    let live_min = steady.iter().map(|r| r.shader_heap.live).min().unwrap();
    let live_max = steady.iter().map(|r| r.shader_heap.live).max().unwrap();
    // Overlap/page distribution can change peaks without retaining old frames.
    // Physical page capacity must still stabilize; the drained exact ownership
    // assertion above checks retention independently of completion timing.
    eprintln!("MULTIFRAME in-flight shader descriptors: {live_min}..{live_max}");
    assert!(steady.iter().all(|r| r.shader_heap.capacity == steady[0].shader_heap.capacity),
        "shader descriptor pages keep growing after warmup");
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
    // The fixture must consume the same owned publisher as the window path.
    // A submission counter alone cannot prove that a GPU copy was harvested.
    let readbacks = driver.app_mut().world().resource::<country_core::render::gpu_readback::GpuReadbacks>();
    assert_eq!(readbacks.submitted_frame(), frames as u64);
    let depth = readbacks.get::<country_core::resources::compute_mouse_depth_value::MouseDepthValueSsbo>()
        .expect("production GPU readback publisher never delivered mouse depth");
    assert!(!depth.data.is_empty());
    assert!(depth.frame > 0 && depth.frame <= readbacks.submitted_frame());
    assert!(depth.revision > 1, "the owned readback stayed at its initial publication");
    eprintln!("MULTIFRAME production GPU readbacks: submitted={} mouse_depth_frame={} revision={}",
        readbacks.submitted_frame(), depth.frame, depth.revision);
    let late_hits = rows.iter().skip(2).filter(|r| r.stats.plan_cache_hit).count();
    eprintln!("MULTIFRAME plan_cache_hit frames>=2: {late_hits}/{}", frames - 2);
    let mut times = rows.iter().skip(2).map(|r| (r.update + r.draw_lists + r.declare + r.compile + r.execute).as_secs_f64() * 1e3).collect::<Vec<_>>();
    times.sort_by(f64::total_cmp);
    eprintln!("MULTIFRAME frame ms (frames>=2): median {:.1} p95 {:.1} p99 {:.1}", percentile(&times, 0.5), percentile(&times, 0.95), percentile(&times, 0.99));

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

const MIN_GROUND_DETAIL: f32 = 2.0;

/// Mean absolute horizontal luma difference over the lower 40% of the picture.
fn lower_gradient_energy(rgba: &[u8], w: u32, h: u32) -> f32 {
    let l = luma(rgba);
    let (w, h) = (w as usize, h as usize);
    let (mut sum, mut count) = (0.0f64, 0usize);
    for y in h * 6 / 10..h {
        for x in 0..w.saturating_sub(1) {
            sum += f64::from((l[y * w + x] - l[y * w + x + 1]).abs());
            count += 1;
        }
    }
    (sum / count.max(1) as f64) as f32
}


/// Photomode autofocus on the production App and device. A pin request (camera focus
/// tag 0) makes the production `photomode_focus` system set `PhotomodeState.index` to the
/// camera index; the autofocus pass receives it as `focus_lock_request`, the packaged
/// `/dof/autofocus.cs` shader unprojects the depth under the focus point into
/// `DofFocusReadbackSsbo`, the production readback publisher returns it to the game
/// World, and `photomode_focus` then pins the camera (tag 1) at exactly the world point of
/// the readback it consumed. Outside photomode the shader receives its "no request" value
/// `0xffff_ffff`.
#[test]
fn photomode_autofocus_request_reaches_gpu_and_returns_through_the_production_readback() -> Result<()> {
    use country_core::{
        render::gpu_readback::GpuReadbacks,
        resources::gui::PhotomodeScreen,
        systems::{
            camera_rig::MainCamera, mode_manager::AppMode,
            render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo,
        },
    };
    use tiny_glade::game_loop::GameFrameDriver;
    const NO_REQUEST: u32 = 0xffff_ffff;
    const FOCUS_OFFSETS: [usize; 6] = [0x170, 0x228, 0x2a8, 0x348, 0x400, 0x488];
    let BootedSession { mut driver, context, mut bridge, shaders: _shaders, device, _boot } =
        boot_shipped_session()?;
    let world = driver.app_mut().world_mut();
    tiny_glade::production_renderer::warm_up_gameplay(bridge.warmup_owners()?, world)
        .context("worker warm-up frame")?;
    {
        let completed = world.resource::<tiny_glade::render_bridge::CompletedGameplayWarmup>();
        bridge.accept_gameplay_warmup(completed)?;
    }
    let mut in_flight = std::collections::VecDeque::<u64>::new();
    // Runs `frames` full frames. `before` runs ahead of the App update, `after` right behind it
    // (its `true` stops early); the render of that frame still happens.
    let mut run_frames = |driver: &mut GameFrameDriver,
                          photomode: bool,
                          frames: usize,
                          before: &mut dyn FnMut(&mut GameFrameDriver),
                          after: &mut dyn FnMut(&mut GameFrameDriver) -> bool|
     -> Result<()> {
        for _ in 0..frames {
            if photomode {
                let world = driver.app_mut().world_mut();
                *world.resource_mut::<AppMode>() = AppMode::Photomode;
                *world.resource_mut::<PhotomodeScreen>() = PhotomodeScreen { was_enabled: true, enabled: true };
            }
            before(driver);
            context.begin_frame(egui::RawInput::default());
            driver.app_mut().update();
            let _ = context.end_frame();
            let stop = after(driver);
            while bridge.with_session(|session| session.pending_frame_count())
                >= device.frames_in_flight() as usize
            {
                wait_oldest_pending(&mut bridge, &mut in_flight)?;
            }
            let world = driver.app_mut().world_mut();
            if photomode {
                *world.resource_mut::<AppMode>() = AppMode::Photomode;
            }
            let frame = tiny_glade::production_renderer::prepare_headless_gameplay_frame(&mut bridge, world)
                .context("autofocus frame")?;
            bridge.prepare_test_execution()?;
            let frame = frame.execute(&mut bridge, world)?;
            world.resource_mut::<country_core::render::RenderTemporalState>().frame_render_returned();
            in_flight.push_back(frame.receipt.frame);
            drop(frame);
            bridge.collect_completed_frames()?;
            if stop {
                break;
            }
        }
        // Wait until the session has retired every frame, then the non-blocking publisher
        // harvests the final copies.
        while bridge.with_session(|session| session.pending_frame_count()) > 0 {
            wait_oldest_pending(&mut bridge, &mut in_flight)?;
            bridge.collect_completed_frames()?;
        }
        in_flight.clear();
        bridge.collect_completed_frames()?;
        Ok(())
    };
    let sample = |driver: &mut GameFrameDriver| -> Result<(DofFocusReadbackSsbo, u64)> {
        let readbacks = driver.app_mut().world().resource::<GpuReadbacks>();
        let view = readbacks
            .get::<DofFocusReadbackSsbo>()
            .context("production readback publisher never delivered DofFocusReadbackSsbo")?;
        Ok((*view.data.first().context("empty autofocus readback")?, view.revision))
    };
    let focus_state = |driver: &mut GameFrameDriver| {
        let camera = driver.app_mut().world().resource::<MainCamera>();
        let focus = FOCUS_OFFSETS[usize::from(camera.mode_byte())];
        (
            u32::from_le_bytes(camera.bytes()[focus..focus + 4].try_into().unwrap()),
            [camera.read_f32(focus + 4), camera.read_f32(focus + 8), camera.read_f32(focus + 12)],
        )
    };
    let (mut nothing_before, mut nothing_after) = (|_: &mut GameFrameDriver| {}, |_: &mut GameFrameDriver| false);

    // Phase A: ordinary gameplay. The pass still runs every frame and answers "no request".
    run_frames(&mut driver, false, 10, &mut nothing_before, &mut nothing_after)?;
    let (idle, idle_revision) = sample(&mut driver)?;
    eprintln!("AUTOFOCUS gameplay: {idle:?} revision {idle_revision}");
    assert_eq!(idle.lock_reponse, NO_REQUEST);
    assert!(idle_revision >= 5, "readback was not republished by the running pass");

    // Phase B: photomode. The production PhotoModeCmd selects a photo camera (camera 0 is the
    // ordinary game camera, which the original cannot tell apart from the zeroed readback);
    // give the camera transition time to settle so the world point under the focus point is
    // stable from frame to frame.
    driver.app_mut().world_mut().send_event(country_core::resources::photo_mode::PhotoModeCmd(0));
    run_frames(&mut driver, true, 30, &mut nothing_before, &mut nothing_after)?;
    let camera_mode = {
        let world = driver.app_mut().world_mut();
        let mut camera = world.resource_mut::<MainCamera>();
        let mode = usize::from(camera.mode_byte());
        let focus = FOCUS_OFFSETS[mode];
        eprintln!(
            "AUTOFOCUS camera mode {mode}: focus tag {}, point ({}, {}), box height {}",
            u32::from_le_bytes(camera.bytes()[focus..focus + 4].try_into().unwrap()),
            camera.read_f32(focus + 0x10), camera.read_f32(focus + 0x14), camera.read_f32(focus + 0x18),
        );
        // A pin request at the screen centre.
        camera.write_f32(focus, f32::from_bits(0));
        camera.write_f32(focus + 0x10, 0.0);
        camera.write_f32(focus + 0x14, 0.0);
        mode as u32
    };
    assert_ne!(camera_mode, 0, "PhotoModeCmd did not leave the ordinary game camera");
    // Step frame by frame: the response `photomode_focus` consumes is the readback published
    // before its update, so that is the sample the pinned point has to equal.
    let mut consumed: Option<DofFocusReadbackSsbo> = None;
    let mut pinned = None;
    {
        let consumed = std::cell::RefCell::new(&mut consumed);
        let pinned = std::cell::RefCell::new(&mut pinned);
        let latest = std::cell::Cell::new(None);
        run_frames(
            &mut driver,
            true,
            40,
            &mut |driver| latest.set(sample(driver).ok().map(|(s, _)| s)),
            &mut |driver| {
                let (tag, position) = focus_state(driver);
                if tag == 1 {
                    **consumed.borrow_mut() = latest.get();
                    **pinned.borrow_mut() = Some(position);
                    true
                } else {
                    false
                }
            },
        )?;
    }
    let pinned = pinned.context("photomode_focus never pinned the camera: the GPU lock request/response did not complete")?;
    let consumed = consumed.context("no readback preceded the pin")?;
    eprintln!("AUTOFOCUS photomode: pinned {pinned:?} from consumed response {consumed:?}");
    // `lock_reponse` is the camera index the GPU answered; photomode_focus only pins on it.
    assert_eq!(consumed.lock_reponse, camera_mode, "the GPU answered a different camera");
    assert!(pinned.iter().all(|v| v.is_finite() && v.abs() < 10_000.0), "lock point is not a world position");
    assert_ne!(pinned, idle.lock_pos_ws, "the lock never replaced the initial shader state");
    assert_eq!(
        pinned.map(f32::to_bits), consumed.lock_pos_ws.map(f32::to_bits),
        "the pinned focus must be exactly the GPU-unprojected lock point of the consumed readback",
    );
    // Independent CPU check of the GPU unprojection: the lock point was requested at the
    // screen centre, so projecting it back with the camera matrices must land there.
    let ndc = {
        use glam::Vec4;
        let camera = driver.app_mut().world().resource::<MainCamera>();
        let mul = |m: glam::Mat4, p: Vec4| m.y_axis * p.y + m.x_axis * p.x + m.z_axis * p.z + m.w_axis;
        let view = mul(camera.read_matrix(0x80), Vec4::new(pinned[0], pinned[1], pinned[2], 1.0));
        let clip = mul(camera.read_matrix(0), view);
        (clip / clip.w).truncate().truncate()
    };
    eprintln!("AUTOFOCUS lock point reprojects to NDC {ndc:?}");
    assert!(ndc.abs().max_element() < 5e-3, "GPU depth unprojection disagrees with the camera: {ndc:?}");
    Ok(())
}


/// T6, menu-save leg: the escape menu's `SaveGameCmd` event (its producer is covered by
/// `original_escape_menu`) goes through the production `save_game_system` on the real shipped
/// session — snapshot capture, section assembly, the save worker's synchronous flush — and
/// ends as `SavingGameComplete` plus a readable `history.json` in the session folder. The
/// temporary account of `boot_shipped_session` keeps the player's saves out of it.
#[test]
fn menu_save_request_reaches_disk_through_the_production_save_system() -> Result<()> {
    use country_core::systems::save::save_game::{SaveGameCmd, SavingGameComplete};
    let BootedSession { mut driver, context, bridge: _bridge, shaders: _shaders, device: _device, _boot } =
        boot_shipped_session()?;
    let mut update = |driver: &mut tiny_glade::game_loop::GameFrameDriver| {
        context.begin_frame(egui::RawInput::default());
        driver.app_mut().update();
        let _ = context.end_frame();
    };
    // Let the shipped session finish loading and the replay buffer drain: the save system only
    // captures a snapshot while the world is stable.
    for _ in 0..30 {
        update(&mut driver);
    }
    let session = driver.app_mut().world().resource::<ActiveSession>().id;
    let folder = session.save_folder_path();
    let before = std::fs::metadata(folder.join("history.json")).ok().map(|m| m.modified().unwrap());
    // The menu entry: the Escape menu's Save/Exit writes this event.
    driver.app_mut().world_mut().send_event(SaveGameCmd);
    let mut completed = None;
    for frame in 0..600 {
        update(&mut driver);
        let done = driver
            .app_mut()
            .world_mut()
            .resource_mut::<bevy_ecs::event::Events<SavingGameComplete>>()
            .drain()
            .last();
        if let Some(done) = done {
            completed = Some((frame, done.path));
            break;
        }
    }
    let (frame, path) = completed.context("SaveGameCmd never produced SavingGameComplete")?;
    eprintln!("SAVE completed after {frame} frames: {path:?}");
    assert_eq!(path, folder, "completion must name the active session folder");
    let history = folder.join("history.json");
    let text = std::fs::read_to_string(&history).context("history.json missing after the menu save")?;
    let value: serde_json::Value = serde_json::from_str(&text).context("history.json is not valid JSON")?;
    assert!(value.is_array() || value.is_object(), "unexpected history.json shape");
    let modified = std::fs::metadata(&history)?.modified()?;
    assert!(before.is_none_or(|b| modified >= b), "history.json was not written by this save");
    let mut names = std::fs::read_dir(&folder)?
        .map(|e| e.map(|e| e.file_name().to_string_lossy().into_owned()))
        .collect::<std::io::Result<Vec<_>>>()?;
    names.sort();
    eprintln!("SAVE folder contents: {names:?}");
    assert!(names.len() > 1, "session meta/snapshot files were not written next to history.json: {names:?}");
    Ok(())
}

/// 仅诊断运行使用：保留实际 BufferHost 的所有能力，并额外允许传输读取。
/// 描述符与原生产运行不同，不能把它当成原生产路径的原样证据。
#[derive(Clone)]
struct M6NaniDiagnosticDevice(pub std::sync::Arc<tiny_glade::render_host::BufferHost>);
impl country_core::render::ssbo::rhapsody_boundary::GraphicsDevice for M6NaniDiagnosticDevice {
    fn create_buffer(&self, desc: &country_core::render::ssbo::rhapsody_boundary::BufferDesc, name: &str, data: Option<&[u8]>)
        -> std::result::Result<country_core::render::ssbo::rhapsody_boundary::Buffer, country_core::render::ssbo::rhapsody_boundary::BackendError> {
        use country_core::render::ssbo::rhapsody_boundary::GraphicsDevice;
        let mut desc = *desc;
        desc.usage |= ash::vk::BufferUsageFlags::TRANSFER_SRC.as_raw();
        self.0.create_buffer(&desc, name, data)
    }
    fn release_buffer(&self, buffer: &country_core::render::ssbo::rhapsody_boundary::Buffer) {
        use country_core::render::ssbo::rhapsody_boundary::GraphicsDevice;
        self.0.release_buffer(buffer)
    }
    fn update_buffer_bytes(&self, buffer: &country_core::render::ssbo::rhapsody_boundary::Buffer, data: &[u8],
        ranges: &[std::ops::Range<usize>], offset: usize) -> std::result::Result<(), country_core::render::ssbo::rhapsody_boundary::BackendError> {
        use country_core::render::ssbo::rhapsody_boundary::GraphicsDevice;
        self.0.update_buffer_bytes(buffer, data, ranges, offset)
    }
    unsafe fn register_cpu_authoritative_buffer(&self, buffer: &country_core::render::ssbo::rhapsody_boundary::Buffer) {
        use country_core::render::ssbo::rhapsody_boundary::GraphicsDevice;
        unsafe { self.0.register_cpu_authoritative_buffer(buffer) }
    }
    fn update_cpu_buffer_bytes(&self, buffer: &country_core::render::ssbo::rhapsody_boundary::Buffer, data: &[u8],
        ranges: &[std::ops::Range<usize>]) -> std::result::Result<(), country_core::render::ssbo::rhapsody_boundary::BackendError> {
        use country_core::render::ssbo::rhapsody_boundary::GraphicsDevice;
        self.0.update_cpu_buffer_bytes(buffer, data, ranges)
    }
    fn mapped_slice_mut(&self, buffer: &country_core::render::ssbo::rhapsody_boundary::Buffer) -> Option<&mut [u8]> {
        use country_core::render::ssbo::rhapsody_boundary::GraphicsDevice;
        self.0.mapped_slice_mut(buffer)
    }
    fn mapped_upload_writer(&self, buffer: &country_core::render::ssbo::rhapsody_boundary::Buffer)
        -> Option<std::sync::Arc<dyn country_core::render::ssbo::rhapsody_boundary::MappedUploadWriter>> {
        use country_core::render::ssbo::rhapsody_boundary::GraphicsDevice;
        self.0.mapped_upload_writer(buffer)
    }
    fn device_address(&self, buffer: &country_core::render::ssbo::rhapsody_boundary::Buffer) -> u64 {
        use country_core::render::ssbo::rhapsody_boundary::GraphicsDevice;
        self.0.device_address(buffer)
    }
    fn ivy_leaf_scale(&self) -> f32 {
        use country_core::render::ssbo::rhapsody_boundary::GraphicsDevice;
        self.0.ivy_leaf_scale()
    }
}

/// 必须在诊断帧已完成且当前owner未轮换时调用；只读取当前真实物理缓冲。
fn m6_read_diagnostic_range(device: &std::sync::Arc<rhapsody::device::Device>,
    host: &tiny_glade::render_host::BufferHost,
    buffer: &country_core::render::ssbo::rhapsody_boundary::Buffer,
    offset: usize, size: usize) -> Result<Vec<u8>> {
    use ash::vk;
    use rhapsody::{command::buffer_access, native::AccessScope,
        resource::{Buffer as NativeBuffer, BufferDesc as NativeBufferDesc}, transfer::with_setup_command};
    let source = host.managed_buffer(buffer).context("诊断源buffer句柄已失效")?;
    anyhow::ensure!(source.usage.contains(vk::BufferUsageFlags::TRANSFER_SRC), "此owner不是可读的诊断描述符");
    anyhow::ensure!(size > 0 && offset.checked_add(size).is_some_and(|end| end as u64 <= source.size), "诊断读取超出真实buffer");
    let output = std::sync::Arc::new(NativeBuffer::new(device.clone(),
        &NativeBufferDesc::new_gpu_to_cpu(size as u64, vk::BufferUsageFlags::TRANSFER_DST))?);
    let queue = device.queue(rhapsody::device::QueueClass::Graphics).context("缺少graphics队列")?;
    with_setup_command(device, &[], |command| unsafe {
        buffer_access(command, &source.shared(), queue, offset as u64, size as u64,
            AccessScope { stages:vk::PipelineStageFlags::TRANSFER, access:vk::AccessFlags::TRANSFER_READ, write:false })?;
        buffer_access(command, &output, queue, 0, size as u64,
            AccessScope { stages:vk::PipelineStageFlags::TRANSFER, access:vk::AccessFlags::TRANSFER_WRITE, write:true })?;
        device.raw_device().cmd_copy_buffer(command.raw_handle(), source.raw_handle(), output.raw_handle(),
            &[vk::BufferCopy::default().src_offset(offset as u64).size(size as u64)]);
        buffer_access(command, &output, queue, 0, size as u64,
            AccessScope { stages:vk::PipelineStageFlags::HOST, access:vk::AccessFlags::HOST_READ, write:false })
    })?.wait(rhapsody::transfer::SETUP_WAIT_TIMEOUT_NS)?;
    let mut result = vec![0; size];
    output.read_bytes(0, &mut result)?;
    Ok(result)
}

/// 只说明诊断描述符下最终可读的间接命令；不声称已观测 raster packet 或像素因果。
#[derive(Debug)]
enum M6PlasterPhase { FinalPhaseEmpty, FinalPhaseNoTarget, TargetVisible }

#[derive(Debug)]
struct M6PlasterIndirectObservation {
    phase: M6PlasterPhase,
    target_entities: usize,
    active_plaster_instances: usize,
    target_dense: Vec<u32>,
    commands: Vec<[u32; 5]>,
    matched_dense: Vec<u32>,
}

/// 将真实实体归属与dense数量分开，重复创建的原行为不能由总数臆断为候选泄漏。
fn m6_observe_plaster_owners(world: &mut bevy_ecs::world::World, stage: &str) {
    use country_core::{resources::walls::{plaster::Plaster, public_walls::WallPlasters},
        render::nani::gpu_world::InstanceKey};
    let registered: std::collections::HashSet<_> = world.resource::<WallPlasters>().0.values()
        .flat_map(|entities| entities.iter().flatten().copied()).collect();
    let mut by_wall = std::collections::BTreeMap::<u64, [usize; 3]>::new();
    let mut query = world.query::<(bevy_ecs::entity::Entity, &Plaster, Option<&InstanceKey>)>();
    for (entity, plaster, key) in query.iter(world) {
        let count = by_wall.entry(plaster.wall.0).or_default();
        count[usize::from(!registered.contains(&entity))] += 1;
        count[2] += usize::from(key.is_some());
    }
    let mut owners = world.query_filtered::<bevy_ecs::entity::Entity,
        bevy_ecs::query::With<system_wall_constructor::plaster_systems::PlasterBufferOwner>>();
    let owner_ids: Vec<_> = owners.iter(world).map(|id| (id, registered.contains(&id))).collect();
    eprintln!("M06 plaster entity ownership {stage}: per_wall[registered,unregistered,with_key]={by_wall:?} buffer_owners[entity,registered]={owner_ids:?}");
}

fn observe_plaster_indirect(device: &std::sync::Arc<rhapsody::device::Device>,
    host: &tiny_glade::render_host::BufferHost, world: &mut bevy_ecs::world::World,
    wall: country_core::resources::walls::WallId) -> Result<M6PlasterIndirectObservation> {
    use country_core::{asset_libraries::ssbo_library::AssetSsboLibrary,
        render::nani::{buffers::{NaniDrawListSsbo, NaniInstanceIndirectionSsbo},
            draw_ext::nani_list_index,
            gpu_world::{GpuWorld, GpuWorldSubsetKind, InstanceDataBuffer},
            instance_data::PlasterInstanceData,
            instance_updates::{build_plaster, PlasterWallInputs}},
        resources::walls::{plaster::{AnimatedPlasterPeel, Plaster}, public_walls::{PublicWalls, WallPlasters}}};
    m6_observe_plaster_owners(world, "after-production-frame");
    let gpu = world.resource::<GpuWorld>();
    anyhow::ensure!(!gpu.instance_uploads_pending(), "Nani实例上传尚未完成");
    let public = world.resource::<PublicWalls>().get(wall).context("抹灰所属墙已不存在")?;
    let curve = public.simplified_curve_2d();
    let inputs = PlasterWallInputs { simplified_point_count: curve.points.len() as u64,
        simplified_length: curve.length, shape_tag: public.shape().tag(),
        default_peel: public.authority.plaster_peel_amount() };
    let peel = world.resource::<AnimatedPlasterPeel>().get(wall);
    let entities = world.resource::<WallPlasters>().0.get(&wall).context("抹灰owner缺失")?;
    let expected: Vec<_> = entities.iter().flatten().filter_map(|id| world.get::<Plaster>(*id))
        .map(|plaster| build_plaster(plaster.nani_view(), inputs, peel).words).collect();
    anyhow::ensure!(!expected.is_empty(), "抹灰owner没有实际条带");
    let kind = GpuWorldSubsetKind::Plaster;
    let subset = gpu.owned_subset_mut(kind);
    let count = subset.instance_count();
    let typed = subset.data().as_any().downcast_ref::<InstanceDataBuffer<PlasterInstanceData>>()
        .context("Plaster subset 实例类型不匹配")?;
    anyhow::ensure!(count <= typed.shadow.data.len(), "Plaster active prefix越界");
    let mut used = vec![false; count];
    let mut target_dense = Vec::with_capacity(expected.len());
    for record in &expected {
        let index = typed.shadow.data[..count].iter().enumerate()
            .find(|(i, value)| !used[*i] && value.raw == *record)
            .map(|(i, _)| i).context("真实Plaster载荷没有唯一active dense槽")?;
        used[index] = true;
        target_dense.push(u32::try_from(index)?);
    }
    let parts = subset.mesh_part_count();
    anyhow::ensure!(parts > 0 && parts <= 4096, "Plaster图集分片数无效");
    drop(subset);
    let list = nani_list_index(kind, 0) as usize;
    anyhow::ensure!(list == 76, "Plaster主绘制列表偏移异常");
    let command_slot = gpu.list_index_to_cmd_offset(list) as usize;
    let parameters = gpu.draw_list_params();
    let start = parameters.cmd_offsets[list] as u64;
    let end = parameters.cmd_offsets.get(list + 1).copied().unwrap_or(parameters.total_cmd_offset) as u64;
    let library = world.resource::<AssetSsboLibrary>();
    let commands_buffer = library.get_buffer::<NaniDrawListSsbo>().context("缺少真实Nani draw list")?;
    let indirection_buffer = library.get_buffer::<NaniInstanceIndirectionSsbo>().context("缺少真实Nani indirection")?;
    let raw = m6_read_diagnostic_range(device, host, commands_buffer,
        command_slot.checked_mul(20).context("间接命令偏移溢出")?, parts.checked_mul(20).context("间接命令长度溢出")?)?;
    let mut commands = Vec::new();
    let mut matched_dense = Vec::new();
    for chunk in raw.chunks_exact(20) {
        let words: [u32; 5] = std::array::from_fn(|i| u32::from_le_bytes(chunk[i*4..i*4+4].try_into().unwrap()));
        commands.push(words);
        if words[0] == 0 || words[1] == 0 { continue; }
        let first = u64::from(words[4]);
        let last = first.checked_add(u64::from(words[1])).context("实例区间溢出")?;
        anyhow::ensure!(first >= start && last <= end, "间接实例区间超出该draw list");
        let offset = usize::try_from(first.checked_mul(4).context("间接索引偏移溢出")?)?;
        let size = usize::try_from(u64::from(words[1]).checked_mul(4).context("间接索引长度溢出")?)?;
        let raw = m6_read_diagnostic_range(device, host, indirection_buffer, offset, size)?;
        for chunk in raw.chunks_exact(4) {
            let dense = u32::from_le_bytes(chunk.try_into().unwrap());
            if target_dense.contains(&dense) && !matched_dense.contains(&dense) { matched_dense.push(dense); }
        }
    }
    eprintln!("M06 diagnostic descriptor variant: wall={} target_entities={} active_plaster_instances={} target_dense={target_dense:?} commands={commands:?} matched_dense={matched_dense:?}",
        wall.0, expected.len(), count);
    let phase = if !matched_dense.is_empty() { M6PlasterPhase::TargetVisible }
        else if commands.iter().any(|words| words[0] > 0 && words[1] > 0) { M6PlasterPhase::FinalPhaseNoTarget }
        else { M6PlasterPhase::FinalPhaseEmpty };
    Ok(M6PlasterIndirectObservation { phase, target_entities:expected.len(), active_plaster_instances:count,
        target_dense, commands, matched_dense })
}

/// 独立诊断首轮剔除；不执行 raster，且额外使用 TRANSFER_SRC 描述符。
fn probe_plaster_initial_cull(bridge: &mut CoreRenderBridge,
    device: &std::sync::Arc<rhapsody::device::Device>, world: &mut bevy_ecs::world::World,
    wall: country_core::resources::walls::WallId) -> Result<M6PlasterIndirectObservation> {
    use test_support::ShaderTestDriver as _;
    use country_core::{asset_libraries::ssbo_library::AssetSsboLibrary,
        render::nani::buffers::{NaniDrawListSsbo, NaniInstanceIndirectionSsbo}};
    let (_, built) = bridge.build_frame(|frame| {
        frame.import_gpu_world(world)?;
        let result = tiny_glade::frame_setup::with_frame_resources(frame, world, |frame, world| {
            tiny_glade::scene_compute::generate_nani_draw_lists(frame, world, None)?;
            Ok(())
        });
        let export = frame.export_gpu_world(world);
        result.and(export)
    })?;
    bridge.shader_backend().prepare_test_pipelines(std::time::Duration::from_secs(60))?;
    let compiled = bridge.compile_built(built)?;
    anyhow::ensure!(compiled.snapshot().pass_order.iter().any(|s| s == "Nani cull & bucket"),
        "独立诊断剔除pass未保留");
    anyhow::ensure!(compiled.snapshot().pass_order.iter().any(|s| s == "Nani generate draws"),
        "独立诊断命令生成pass未保留");
    let receipt = bridge.execute_after_uploads(compiled)?;
    // 在等待/退役前关联receipt：完成回收后token可能已经注销。
    let ssbos = world.resource::<AssetSsboLibrary>();
    for (name, handle) in [("commands", ssbos.get_buffer::<NaniDrawListSsbo>().context("缺少命令owner")?),
        ("indirection", ssbos.get_buffer::<NaniInstanceIndirectionSsbo>().context("缺少实例间接owner")?)] {
        let owner = bridge.hosts().buffers.managed_buffer(handle).context("诊断owner失效")?.shared();
        let present = bridge.with_session(|session| receipt.snapshot.physical_mapping.iter().any(|mapping|
            matches!(mapping.token.and_then(|token| session.backend.resource_clone(token)),
                Some(rhapsody::rg::GpuResource::Buffer(actual)) if std::sync::Arc::ptr_eq(&actual, &owner))));
        anyhow::ensure!(present, "诊断帧receipt未保留当前 {name} 真实buffer owner");
    }
    bridge.wait_submits_done(10_000_000_000)?;
    let observed = observe_plaster_indirect(device, bridge.hosts().buffers.as_ref(), world, wall)?;
    anyhow::ensure!(matches!(observed.phase, M6PlasterPhase::TargetVisible),
        "独立首轮剔除无目标抹灰实例: {:?}", observed);
    Ok(observed)
}

/// 有界诊断：仅用真实 nani_plaster draw entry 与生产 main_pass，读回深度。
/// 诊断目标额外启用 TRANSFER_SRC；它不是完整生产帧的最终颜色对照。
fn probe_plaster_raster_depth(
    bridge: &mut CoreRenderBridge,
    device: &std::sync::Arc<rhapsody::device::Device>,
    world: &mut bevy_ecs::world::World,
    shaders: &country_core::asset_libraries::shader_library::AssetShaderLibrary,
) -> Result<()> {
    use ash::vk::{self, Handle as _};
    use test_support::ShaderTestDriver as _;
    use country_core::{render::pass_builder::framebuffer_binding::AttachmentTexture,
        systems::render_loop::{draw_passes::{draw_lists::{self, DrawLists}, main_pass::MainPassTargets},
            render_world::formats}};
    let extent = world.resource::<country_core::render::render_config::FrameRenderConfig>().extent_0;
    let mut original = DrawLists::new();
    draw_lists::build_draw_lists(world, shaders, &mut original);
    let mut only_plaster = DrawLists::new();
    for index in 0..draw_lists::DRAW_LIST_COUNT {
        only_plaster.list_mut(index).extend(original.list(index).iter()
            .filter(|entry| entry.0.params.shader_name == "nani_plaster").cloned());
    }
    anyhow::ensure!(!only_plaster.list(draw_lists::list::GBUFFER).is_empty(),
        "真实主G-buffer列表没有nani_plaster");
    let empty = DrawLists::new();
    let mut observations = Vec::new();
    for lists in [&empty, &only_plaster] {
        let make_color = |format| std::sync::Arc::new(rhapsody::resource::Texture::new(
            device.clone(), &rhapsody::resource::TextureDesc::new_2d(extent[0], extent[1], format)
                .usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::SAMPLED)).unwrap());
        let gbuffer = make_color(vk::Format::from_raw(formats::GBUFFER as i32));
        let velocity = make_color(vk::Format::from_raw(formats::VELOCITY as i32));
        let emissive = make_color(vk::Format::from_raw(formats::COLOR_HDR as i32));
        let depth = std::sync::Arc::new(rhapsody::resource::Texture::new(device.clone(),
            &rhapsody::resource::TextureDesc::new_2d(extent[0], extent[1], vk::Format::D32_SFLOAT_S8_UINT)
                .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT | vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_SRC))?);
        let output = std::sync::Arc::new(rhapsody::resource::Buffer::new(device.clone(),
            &rhapsody::resource::BufferDesc::new_gpu_to_cpu(u64::from(extent[0] * extent[1] * 4),
                vk::BufferUsageFlags::TRANSFER_DST))?);
        let (_, built) = bridge.build_frame(|frame| frame.with_main_prelude(world, |frame, world| {
            tiny_glade::scene_compute::generate_nani_draw_lists(frame, world, None)?;
            let g = frame.import_native_texture(gbuffer.clone(), 0)?;
            let v = frame.import_native_texture(velocity.clone(), 0)?;
            let e = frame.import_native_texture(emissive.clone(), 0)?;
            let d = frame.import_native_texture(depth.clone(), 0)?;
            let out = frame.import_native_buffer(output.clone(), 0)?;
            let attachment = |id: u32, format: u32| AttachmentTexture {
                id, extent: (extent[0], extent[1]), format, sample_count: 1,
            };
            let targets = MainPassTargets { gbuffer: attachment(g.raw(), formats::GBUFFER),
                velocity: attachment(v.raw(), formats::VELOCITY), depth: attachment(d.raw(), 130) };
            let _vao = frame.raster_resources().begin_vao_scope(world, lists)?;
            tiny_glade::scene_raster::SceneRasterHost::new(frame, world, lists)?
                .main_pass(&targets, &attachment(e.raw(), formats::COLOR_HDR))?;
            frame.with_graph(|graph| {
                let mut copy = graph.add_pass("M06 diagnostic plaster depth readback");
                copy.add_resource_access(d, 0x25, 0)?;
                copy.add_resource_access(out, 0x34, 0)?;
                let image = depth.clone(); let buffer = output.clone(); let device = device.clone();
                copy.render_with_result_context(move |ctx| {
                    unsafe { device.raw_device().cmd_copy_image_to_buffer(
                        vk::CommandBuffer::from_raw(ctx.command.handle()), image.raw_handle(),
                        vk::ImageLayout::TRANSFER_SRC_OPTIMAL, buffer.raw_handle(),
                        &[vk::BufferImageCopy::default().image_subresource(
                            vk::ImageSubresourceLayers::default().aspect_mask(vk::ImageAspectFlags::DEPTH).layer_count(1))
                            .image_extent(vk::Extent3D { width: extent[0], height: extent[1], depth: 1 })]); }
                    Ok(())
                });
                graph.export(out, 0x26)?;
                Ok(())
            })?;
            Ok(())
        }))?;
        bridge.shader_backend().prepare_test_pipelines(Duration::from_secs(60))?;
        let compiled = bridge.compile_built(built)?;
        bridge.execute_after_uploads(compiled)?;
        bridge.wait_submits_done(10_000_000_000)?;
        let mut bytes = vec![0u8; (extent[0] * extent[1] * 4) as usize];
        output.read_bytes(0, &mut bytes)?;
        let depths: Vec<f32> = bytes.chunks_exact(4)
            .map(|p| f32::from_le_bytes(p.try_into().unwrap())).collect();
        anyhow::ensure!(depths.iter().all(|value| value.is_finite() && (0.0..=1.0).contains(value)),
            "抹灰诊断深度包含非有限值或超出[0,1]的样本");
        observations.push(depths.iter().filter(|value| **value > 0.0).count());
    }
    anyhow::ensure!(observations[0] == 0, "空抹灰列表深度未清零: {}", observations[0]);
    anyhow::ensure!(observations[1] > 0, "真实抹灰列表未产生深度覆盖");
    eprintln!("M06 bounded production plaster raster depth control={} covered={}", observations[0], observations[1]);
    Ok(())
}

/// 验收注入的是合成输入和地形命中；拾取、UI相位、历史与墙系统仍使用生产调度。
#[derive(bevy_ecs::prelude::Resource)]
struct M6SyntheticEraserInput {
    position: glam::Vec3,
    click: country_core::input::action_set::DigitalActionPhase,
    undo: country_core::input::action_set::DigitalActionPhase,
    redo: country_core::input::action_set::DigitalActionPhase,
}

fn m6_inject_before_picker(
    input: bevy_ecs::prelude::Res<M6SyntheticEraserInput>,
    mut actions: bevy_ecs::prelude::ResMut<country_core::input::action_set::ActionSetStates>,
    mut terrain: bevy_ecs::prelude::ResMut<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,
) {
    use country_core::input::action_set::DigitalActionState;
    for (slot, phase) in [(0, input.click), (16, input.undo), (17, input.redo)] {
        actions.digital[slot] = DigitalActionState { phase, timer_seconds: 0.0 };
    }
    terrain.update(Some(input.position));
}

fn m6_inject_before_eraser(
    input: bevy_ecs::prelude::Res<M6SyntheticEraserInput>,
    mut terrain: bevy_ecs::prelude::ResMut<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,
) {
    terrain.update(Some(input.position));
}

#[test]
fn m06_shipped_wall_synthetic_input_uses_real_ui_and_history() -> Result<()> {
    use bevy_ecs::schedule::IntoScheduleConfigs;
    use country_core::{
        input::action_set::DigitalActionPhase as Phase,
        resources::{history::History, walls::EraserSizeMultiplier},
        systems::{mode_manager::{AppMode, EraserLayer}, ui::hover_intersection::hover_intersection,
            wall::eraser::wall_eraser::wall_eraser},
    };
    if std::env::var_os("TG_M6_ERASER_CHILD").is_some() { return m6_eraser_child_recovery(); }
    let BootedSession { mut driver, context, bridge: _bridge, shaders: _shaders, device: _device, _boot } = boot_shipped_session()?;
    let world = driver.app().world();
    let inner = world.resource::<InnerWalls>();
    // 在真实长墙段中点使用小刷，保留至少一面墙供跨进程核验。
    let (wall, point) = inner.wall_ids().filter_map(|id| {
        let state = inner.get_unwrapped(id);
        state.snapshot().source.curve.points.windows(2).filter_map(|points| {
            let a = glam::Vec2::from_array(points[0]);
            let b = glam::Vec2::from_array(points[1]);
            let length = a.distance(b);
            (length > 1.5).then_some((id, (a + b) * 0.5, length))
        }).max_by(|a, b| a.2.total_cmp(&b.2))
    }).max_by(|a, b| a.2.total_cmp(&b.2)).map(|(id, p, _)| (id, p.to_array()))
        .context("初始会话没有可局部擦除的长墙段")?;
    let original = m6_authority_map(world);
    let before = world.resource::<History>().ledger().entries.len();
    driver.app_mut().insert_resource(AppMode::Eraser(EraserLayer::Wall));
    driver.app_mut().insert_resource(EraserSizeMultiplier(0.4));
    driver.app_mut().insert_resource(M6SyntheticEraserInput {
        position: glam::Vec3::new(point[0], 0.0, point[1]),
        click: Phase::Idle, undo: Phase::Idle, redo: Phase::Idle,
    });
    driver.app_mut().add_systems(bevy_app::Update, m6_probe_height_requests.before(
        country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::adjust_wall_height_impl));
    driver.app_mut().add_systems(bevy_app::Update, m6_inject_before_picker.before(hover_intersection));
    driver.app_mut().add_systems(bevy_app::Update, m6_inject_before_eraser.before(wall_eraser::<country_core::resources::history::MaybePendingSnapshotId>));
    let step = |driver: &mut tiny_glade::game_loop::GameFrameDriver| {
        context.begin_frame(egui::RawInput::default());
        driver.app_mut().update();
        let _ = context.end_frame();
    };
    // 第一帧注册真实画布，第二帧由生产拾取和ui_update产生按下相位。
    eprintln!("M6_STAGE register target={wall:?} point={point:?}");
    step(&mut driver);
    driver.app_mut().world_mut().resource_mut::<M6SyntheticEraserInput>().click = Phase::JustPressed;
    step(&mut driver);
    eprintln!("M6_STAGE after_press walls={:?}", driver.app().world().resource::<InnerWalls>().wall_ids().collect::<Vec<_>>());
    anyhow::ensure!(!driver.app().world().resource::<InnerWalls>().contains(wall), "真实UI未拆分所选墙");
    anyhow::ensure!(driver.app().world().resource::<InnerWalls>().wall_ids().count() > 0, "擦除后没有保留墙");
    driver.app_mut().world_mut().resource_mut::<M6SyntheticEraserInput>().click = Phase::JustReleased;
    step(&mut driver);
    driver.app_mut().world_mut().resource_mut::<M6SyntheticEraserInput>().click = Phase::Idle;
    for _ in 0..5 { step(&mut driver); }
    anyhow::ensure!(driver.app().world().resource::<History>().ledger().entries.len() > before, "擦除未提交历史");
    let committed_cursor = driver.app().world().resource::<History>().ledger().cursor;
    eprintln!("M6_STAGE before_undo history_entries={} cursor={committed_cursor}",
        driver.app().world().resource::<History>().ledger().entries.len());
    driver.app_mut().world_mut().resource_mut::<M6SyntheticEraserInput>().undo = Phase::JustPressed;
    step(&mut driver);
    driver.app_mut().world_mut().resource_mut::<M6SyntheticEraserInput>().undo = Phase::Idle;
    m6_wait_replay_completion(&mut driver, &context, committed_cursor - 1, "撤销")?;
    anyhow::ensure!(driver.app().world().resource::<InnerWalls>().contains(wall), "真实撤销未恢复所选墙");
    for _ in 0..10 { step(&mut driver); }
    anyhow::ensure!(m6_authority_map(driver.app().world()) == original, "撤销后的墙曲线、字段、颜色、屋顶关联或装饰ID不同");
    let restored_entity = driver.app().world().resource::<country_core::resources::walls::public_walls::WallEntities>()
        .wall_entity(wall).context("撤销后的墙缺少实体/GPU owner")?;
    m6_log_constructor_ranges(driver.app().world(), wall, "重做前")?;
    eprintln!("M6_STAGE before_redo");
    driver.app_mut().world_mut().resource_mut::<M6SyntheticEraserInput>().redo = Phase::JustPressed;
    step(&mut driver);
    driver.app_mut().world_mut().resource_mut::<M6SyntheticEraserInput>().redo = Phase::Idle;
    m6_wait_replay_completion(&mut driver, &context, committed_cursor, "重做")?;
    anyhow::ensure!(!driver.app().world().resource::<InnerWalls>().contains(wall), "真实重做未删除所选墙");
    use country_core::systems::save::save_game::{SaveGameCmd, SavingGameComplete};
    for _ in 0..30 { step(&mut driver); }
    let world = driver.app().world();
    let remaining = m6_authority_map(world);
    anyhow::ensure!(remaining.as_array().is_some_and(|walls| !walls.is_empty()), "重做后必须保留真实墙");
    m6_assert_removed_owners(world, wall)?;
    m6_log_constructor_ranges(world, wall, "重做后")?;
    anyhow::ensure!(world.get::<country_core::geometry::instanced_wall::InstancedWall<
        country_core::systems::wall::construction::bricks::WallBrickInstance>>(restored_entity).is_none(),
        "原墙实体仍持有GPU实例分配");
    let history_entry = serde_json::to_string(&country_core::resources::history::serialized::SavedMultiEdit(
        world.resource::<History>().ledger().entries.last().context("擦除历史为空")?.clone()))?;
    let session = world.resource::<ActiveSession>().id;
    driver.app_mut().world_mut().send_event(SaveGameCmd);
    let mut saved = false;
    for _ in 0..600 {
        step(&mut driver);
        if driver.app_mut().world_mut().resource_mut::<bevy_ecs::event::Events<SavingGameComplete>>()
            .drain().any(|event| event.path == session.save_folder_path()) { saved = true; break; }
    }
    anyhow::ensure!(saved, "生产存档 worker 未完成");
    // 子进程启动前释放父进程的GPU所有者，隔离账户仍由静态TempDir保留。
    drop(driver);
    drop(_bridge);
    drop(_shaders);
    drop(_device);
    let status = std::process::Command::new(std::env::current_exe()?)
        .args(["--exact", "m06_shipped_wall_synthetic_input_uses_real_ui_and_history", "--nocapture", "--test-threads=1"])
        .env("TG_M6_ERASER_CHILD", "1")
        .env("TG_M6_ERASER_FOLDER", session.save_folder_path())
        .env("TG_M6_ERASER_SESSION", serde_json::to_string(&session)?)
        .env("TG_M6_ERASER_WALL", wall.0.to_string())
        .env("TG_M6_ERASER_REMAINING", serde_json::to_string(&remaining)?)
        .env("TG_M6_ERASER_HISTORY", history_entry)
        .status()?;
    anyhow::ensure!(status.success(), "新进程恢复验收失败: {status}");
    Ok(())
}

/// 子进程先使用自己的隔离账户启动，再复制父进程实际存档并走生产会话命令。
fn m6_eraser_child_recovery() -> Result<()> {
    use country_core::{meta_commander::{MetaCommand, MetaCommander},
        resources::{session::SessionId, walls::WallId}};
    fn copy_tree(from: &std::path::Path, to: &std::path::Path) -> Result<()> {
        std::fs::create_dir_all(to)?;
        for entry in std::fs::read_dir(from)? {
            let entry = entry?;
            let target = to.join(entry.file_name());
            if entry.file_type()?.is_dir() { copy_tree(&entry.path(), &target)?; }
            else { std::fs::copy(entry.path(), target)?; }
        }
        Ok(())
    }
    let session: SessionId = serde_json::from_str(&std::env::var("TG_M6_ERASER_SESSION")?)?;
    let wall = WallId(std::env::var("TG_M6_ERASER_WALL")?.parse()?);
    let source = PathBuf::from(std::env::var_os("TG_M6_ERASER_FOLDER").context("缺少父存档")?);
    let BootedSession { mut driver, context, bridge: _bridge, shaders: _shaders, device: _device, _boot } = boot_shipped_session()?;
    copy_tree(&source, &session.save_folder_path())?;
    driver.app_mut().world_mut().resource_mut::<MetaCommander>()
        .extend_script([MetaCommand::load_session(session, false)]);
    let mut loaded = false;
    let mut saw_reload = false;
    for _ in 0..1200 {
        context.begin_frame(egui::RawInput::default());
        driver.app_mut().update();
        let _ = context.end_frame();
        let world = driver.app().world();
        saw_reload |= world.get_resource::<bevy_state::prelude::State<country_core::app_loading_state::AppLoadingState>>()
            .is_some_and(|state| *state.get() == country_core::app_loading_state::AppLoadingState::Reload);
        if saw_reload && world.get_resource::<ActiveSession>().is_some_and(|s| s.id == session)
            && world.resource::<LoadGameManager>().options.is_none()
            && world.get_resource::<country_core::resources::history::ReplayBuffer>().is_some_and(|r| r.is_empty())
            && world.get_resource::<country_core::systems::history::PendingHistoryCommands>().is_some_and(|p| p.0.is_empty())
            && world.get_resource::<country_core::systems::mode_manager::AppMode>().is_some_and(|m| m.native_tag() != 13)
            && world.get_resource::<bevy_state::prelude::State<country_core::app_loading_state::AppLoadingState>>()
                .is_some_and(|state| *state.get() == country_core::app_loading_state::AppLoadingState::Game) {
            loaded = true;
            break;
        }
    }
    anyhow::ensure!(loaded, "新进程未通过生产会话命令加载存档");
    // 让回放最后一帧发送的事件由生产消费者完整处理。
    for _ in 0..2 {
        context.begin_frame(egui::RawInput::default());
        driver.app_mut().update();
        let _ = context.end_frame();
    }
    let world = driver.app().world();
    anyhow::ensure!(world.resource::<country_core::resources::history::ReplayBuffer>().is_empty()
        && world.resource::<country_core::systems::history::PendingHistoryCommands>().0.is_empty()
        && world.resource::<country_core::systems::mode_manager::AppMode>().native_tag() != 13,
        "新进程恢复后回放再次启动");
    anyhow::ensure!(!world.resource::<InnerWalls>().contains(wall), "存档恢复后被擦除的墙重新出现");
    let expected: serde_json::Value = serde_json::from_str(&std::env::var("TG_M6_ERASER_REMAINING")?)?;
    anyhow::ensure!(m6_authority_map(world) == expected, "新进程的剩余墙权威状态不一致");
    let mut expected_history: serde_json::Value = serde_json::from_str(&std::env::var("TG_M6_ERASER_HISTORY")?)?;
    let entry = world.resource::<country_core::resources::history::History>().ledger().entries.last().context("恢复历史为空")?;
    // 与父端和实际存档同走 f32 文本写出，再解析成 JSON 值逐字段严格比较。
    let actual_wire = serde_json::to_string(&country_core::resources::history::serialized::SavedMultiEdit(entry.clone()))?;
    let mut actual_history: serde_json::Value = serde_json::from_str(&actual_wire)?;
    canonicalize_decorator_diff_map(&mut expected_history);
    canonicalize_decorator_diff_map(&mut actual_history);
    if let Some((path, expected, actual)) = m6_first_json_difference(&expected_history, &actual_history) {
        anyhow::bail!("恢复历史末项不同于实际擦除记录：路径 {path}；expected={expected}；actual={actual}");
    }
    m6_assert_removed_owners(world, wall)?;
    Ok(())
}

/// 以位模式比较完整墙源和元数据；实体代次不跨重建比较，屋顶检查墙ID与序列化权威字段。
fn m6_authority_map(world: &bevy_ecs::world::World) -> serde_json::Value {
    use country_core::resources::{color_id::{WallColorIds, HalftimberColorIds}, walls::{
        decorator_storage::DecoratorStorage, public_walls::PublicWalls,
    }};
    let inner = world.resource::<InnerWalls>();
    let colors = world.resource::<WallColorIds>();
    let timber = world.resource::<HalftimberColorIds>();
    let public = world.resource::<PublicWalls>();
    let decorators = world.resource::<DecoratorStorage>();
    let mut ids: Vec<_> = inner.wall_ids().collect();
    ids.sort_by_key(|id| id.0);
    let rows: Vec<_> = ids.into_iter().map(|id| {
        let state = inner.get_unwrapped(id);
        let snapshot = state.snapshot();
        let source = &snapshot.source;
        let mut decoration_ids: Vec<_> = decorators.wall_decorator_ids(id).collect();
        decoration_ids.sort_unstable();
        serde_json::json!({
            "id": id.0,
            "points": source.curve.points.iter().map(|p| p.map(f32::to_bits)).collect::<Vec<_>>(),
            "u": source.curve.points_u.iter().map(|p| p.to_bits()).collect::<Vec<_>>(),
            "length": source.curve.length.to_bits(), "kind": source.curve_kind_tag,
            "floor": source.terrain_floor.to_bits(), "shape": source.shape.bytes(),
            "style": source.style, "extra": source.extra.to_bits(), "special": source.special,
            "path": source.observed_path_scalar.to_bits(), "clutter": source.clutter_version,
            "source64": source.observed_source_flag_64, "source65": state.source_flag_65(),
            "height": state.height().to_bits(), "elevation": state.elevation().to_bits(),
            "metadata60": snapshot.metadata.source_60_bits, "metadata66": snapshot.metadata.source_66,
            "metadata67": snapshot.metadata.source_67, "metadata69": snapshot.metadata.source_69,
            "wall_color": colors.0.get(id.0 as usize), "timber_color": timber.0.get(id.0 as usize),
            "roof": public.roof_entity(id).map(|entity| {
                let roof = world.get::<country_core::resources::roofs::Roof>(entity)
                    .expect("公开墙屋顶关联缺少真实屋顶组件");
                assert_eq!(roof.wall_id(), id, "屋顶关联指向其他墙");
                serde_json::to_value(roof).expect("屋顶权威字段序列化失败")
            }), "decorator_ids": decoration_ids,
        })
    }).collect();
    serde_json::Value::Array(rows)
}

/// 检查被拆掉的原墙已从实际墙实体、抹灰和支撑权威表退役。
fn m6_assert_removed_owners(world: &bevy_ecs::world::World, wall: country_core::resources::walls::WallId) -> Result<()> {
    use country_core::resources::walls::{wall_stone_columns::StoneColumnCollisionStorage,
        public_walls::{PublicWalls, WallEntities, WallPlasters}};
    use system_wall_constructor::{construct_arches::WallConstructorOutput, gpu_output::{ConstructorKey, ConstructorPart}};
    anyhow::ensure!(!world.resource::<WallEntities>().0.contains_key(&wall), "已删墙仍保留实体/GPU owner");
    anyhow::ensure!(!world.resource::<WallPlasters>().0.contains_key(&wall), "已删墙仍保留抹灰 owner");
    let gpu = &world.resource::<WallConstructorOutput>().gpu;
    let key = ConstructorKey { wall: wall.0, part: ConstructorPart::ElevationSupports };
    anyhow::ensure!(gpu.brick_allocation(key).is_none() && gpu.plank_allocation(key).is_none()
        && gpu.half_timber_allocation(wall.0).is_none(), "已删墙仍保留实际构造器GPU分配owner");
    let columns = world.resource::<StoneColumnCollisionStorage>();
    anyhow::ensure!(columns.columns.get(&wall).is_none_or(Vec::is_empty), "已删墙仍保留石柱几何");
    anyhow::ensure!(!columns.colliders.contains_key(&wall), "已删墙仍保留石柱碰撞句柄");
    anyhow::ensure!(world.resource::<PublicWalls>().get_unchecked(wall).is_none(), "已删墙仍保留公开墙");
    Ok(())
}

/// 独立事件读取器仅诊断消费前的真实队列，不消费生产 reader 的游标。
fn m6_probe_height_requests(
    mut events: bevy_ecs::prelude::EventReader<country_core::systems::wall::adjust_wall_height::AdjustWallHeightCmd>,
    inner: bevy_ecs::prelude::Res<InnerWalls>,
) {
    for event in events.read() {
        eprintln!("M6_HEIGHT request={:?} exists={} ids={:?}", event.0,
            inner.contains(country_core::resources::walls::WallId(event.0.wall.0)),
            inner.wall_ids().collect::<Vec<_>>());
    }
}

/// 必须先确认请求改变了历史游标，再等待原回放完成门；额外两帧让回放末帧
/// 产生的高度等事件经生产消费者处理。事件队列自身保留双缓冲，不能用长度当消费游标。
fn m6_wait_replay_completion(
    driver: &mut tiny_glade::game_loop::GameFrameDriver,
    context: &egui::Context,
    expected_cursor: i32,
    stage: &str,
) -> Result<()> {
    use country_core::{resources::history::{History, ReplayBuffer}, systems::{history::PendingHistoryCommands, mode_manager::AppMode}};
    let complete = |world: &bevy_ecs::world::World| {
        world.resource::<History>().ledger().cursor == expected_cursor
            && world.resource::<ReplayBuffer>().is_empty()
            && world.resource::<PendingHistoryCommands>().0.is_empty()
            && world.resource::<AppMode>().native_tag() != 13
    };
    let mut reached = false;
    for _ in 0..600 {
        context.begin_frame(egui::RawInput::default());
        driver.app_mut().update();
        let _ = context.end_frame();
        if complete(driver.app().world()) { reached = true; break; }
    }
    anyhow::ensure!(reached, "{stage}历史游标/队列/回放模式未完成");
    for _ in 0..2 {
        context.begin_frame(egui::RawInput::default());
        driver.app_mut().update();
        let _ = context.end_frame();
        anyhow::ensure!(complete(driver.app().world()), "{stage}完成后回放再次启动");
    }
    Ok(())
}

/// 显示该墙各真实分配的绘制范围及全局活跃范围；释放后全局范围可能被其他墙复用。
fn m6_log_constructor_ranges(world: &bevy_ecs::world::World, wall: country_core::resources::walls::WallId, stage: &str) -> Result<()> {
    use country_core::{asset_libraries::ssbo_library::AssetSsboLibrary,
        geometry::instanced_wall::BrickTransformSsbo, resources::roofs::roof_buffers::WoodenPlankSsbo};
    use system_wall_constructor::{construct_arches::WallConstructorOutput, gpu_output::{ConstructorKey, ConstructorPart}};
    let gpu = &world.resource::<WallConstructorOutput>().gpu;
    let library = world.resource::<AssetSsboLibrary>();
    let bricks = library.get::<BrickTransformSsbo>().context("缺少实际墙砖SSBO")?;
    let planks = library.get::<WoodenPlankSsbo>().context("缺少实际木条SSBO")?;
    let key = ConstructorKey { wall: wall.0, part: ConstructorPart::ElevationSupports };
    eprintln!("M6_CONSTRUCTOR_RANGES stage={stage} wall={wall:?} brick={:?} plank={:?} half={:?} active_bricks={:?} active_planks={:?}",
        gpu.brick_allocation(key).map(|a| bricks.allocation_to_draw_multi_instanced(a)),
        gpu.plank_allocation(key).map(|a| planks.allocation_to_draw_multi_instanced(a)),
        gpu.half_timber_allocation(wall.0).map(|a| planks.allocation_to_draw_multi_instanced(a)),
        bricks.to_draw_multi_instanced(), planks.to_draw_multi_instanced());
    Ok(())
}

/// 报告首个具体JSON差异，不输出整份历史；数组保持顺序，映射按键递归比较。
fn m6_first_json_difference(expected: &serde_json::Value, actual: &serde_json::Value)
    -> Option<(String, String, String)> {
    fn short(value: Option<&serde_json::Value>) -> String {
        let text = value.map_or_else(|| "<missing>".to_owned(), |v| v.to_string());
        if text.len() <= 320 { text } else { format!("{}… ({} bytes)", text.chars().take(160).collect::<String>(), text.len()) }
    }
    fn walk(path: String, a: &serde_json::Value, b: &serde_json::Value) -> Option<(String, String, String)> {
        match (a, b) {
            (serde_json::Value::Object(a), serde_json::Value::Object(b)) => {
                let keys: std::collections::BTreeSet<_> = a.keys().chain(b.keys()).collect();
                for key in keys {
                    let next = format!("{path}/{}", key.replace('~', "~0").replace('/', "~1"));
                    match (a.get(key), b.get(key)) {
                        (Some(x), Some(y)) => if let Some(diff) = walk(next, x, y) { return Some(diff); },
                        (x, y) => return Some((next, short(x), short(y))),
                    }
                }
                None
            }
            (serde_json::Value::Array(a), serde_json::Value::Array(b)) => {
                for i in 0..a.len().max(b.len()) {
                    let next = format!("{path}/{i}");
                    match (a.get(i), b.get(i)) {
                        (Some(x), Some(y)) => if let Some(diff) = walk(next, x, y) { return Some(diff); },
                        (x, y) => return Some((next, short(x), short(y))),
                    }
                }
                None
            }
            _ if a != b => Some((path, short(Some(a)), short(Some(b)))),
            _ => None,
        }
    }
    walk(String::new(), expected, actual)
}

/// 合成鼠标动作经过生产拾取/画布/绘墙/吸附，真实OS输入另验。
#[test]
fn m06_real_ui_draw_closed_wall_undo_redo_and_save() -> Result<()> {
    use bevy_ecs::schedule::IntoScheduleConfigs;
    use country_core::{input::action_set::DigitalActionPhase as Phase,
        resources::{history::History, in_progress_curve::InProgressCurve},
        systems::{mode_manager::{AppMode, WallShape}, ui::hover_intersection::hover_intersection,
            wall::draw_wall::draw_wall}};
    if std::env::var_os("TG_M6_CLOSED_CHILD").is_some() { return m6_closed_child_recovery(); }
    let BootedSession { mut driver, context, bridge: _bridge, shaders: _shaders, device: _device, _boot } = boot_shipped_session()?;
    let original = m6_authority_map(driver.app().world());
    let before = driver.app().world().resource::<History>().ledger().entries.len();
    driver.app_mut().insert_resource(AppMode::Wall(WallShape::Freeform));
    driver.app_mut().insert_resource(M6SyntheticEraserInput { position: glam::Vec3::new(-18., 0., -18.),
        click: Phase::Idle, undo: Phase::Idle, redo: Phase::Idle });
    driver.app_mut().add_systems(bevy_app::Update, m6_inject_before_picker.before(hover_intersection));
    // set10 的真实射线写入后注入合成命中，生产画布及吸附/绘墙都读取同一位置。
    driver.app_mut().add_systems(bevy_app::Update, m6_inject_before_eraser
        .after(country_core::resources::mouse_terrain_raycast::mouse_terrain_raycast)
        .before(country_core::systems::wall::ui_draw_wall::ui_draw_wall)
        .before(draw_wall));
    let step = |driver: &mut tiny_glade::game_loop::GameFrameDriver| {
        context.begin_frame(egui::RawInput::default()); driver.app_mut().update(); let _ = context.end_frame();
    };
    step(&mut driver);
    driver.app_mut().world_mut().resource_mut::<M6SyntheticEraserInput>().click = Phase::JustPressed;
    step(&mut driver);
    driver.app_mut().world_mut().resource_mut::<M6SyntheticEraserInput>().click = Phase::Held;
    let corners = [glam::Vec2::new(-18.,-18.), glam::Vec2::new(-12.,-18.),
        glam::Vec2::new(-12.,-12.), glam::Vec2::new(-18.,-12.), glam::Vec2::new(-18.,-18.)];
    for segment in corners.windows(2) {
        for i in 1..=30 {
            let p = segment[0].lerp(segment[1], i as f32 / 30.0);
            driver.app_mut().world_mut().resource_mut::<M6SyntheticEraserInput>().position = glam::Vec3::new(p.x, 0., p.y);
            step(&mut driver);
        }
    }
    let wall = driver.app().world().resource::<InProgressCurve>().wall.context("真实画布未生成进行中的墙")?;
    driver.app_mut().world_mut().resource_mut::<M6SyntheticEraserInput>().click = Phase::JustReleased;
    step(&mut driver);
    driver.app_mut().world_mut().resource_mut::<M6SyntheticEraserInput>().click = Phase::Idle;
    for _ in 0..10 { step(&mut driver); }
    let world = driver.app().world();
    anyhow::ensure!(world.resource::<InnerWalls>().get_unwrapped(wall).source().curve_kind_tag == 3,
        "真实绘墙未触发自吸附闭合");
    anyhow::ensure!(world.resource::<History>().ledger().entries.len() > before, "闭合未提交历史");
    let cursor = world.resource::<History>().ledger().cursor;
    let closed = m6_authority_map(world);
    driver.app_mut().world_mut().resource_mut::<M6SyntheticEraserInput>().undo = Phase::JustPressed;
    step(&mut driver);
    driver.app_mut().world_mut().resource_mut::<M6SyntheticEraserInput>().undo = Phase::Idle;
    m6_wait_replay_completion(&mut driver, &context, cursor - 1, "闭合撤销")?;
    for _ in 0..10 { step(&mut driver); }
    anyhow::ensure!(m6_authority_map(driver.app().world()) == original, "闭合撤销后权威墙不一致");
    driver.app_mut().world_mut().resource_mut::<M6SyntheticEraserInput>().redo = Phase::JustPressed;
    step(&mut driver);
    driver.app_mut().world_mut().resource_mut::<M6SyntheticEraserInput>().redo = Phase::Idle;
    m6_wait_replay_completion(&mut driver, &context, cursor, "闭合重做")?;
    for _ in 0..30 { step(&mut driver); }
    anyhow::ensure!(m6_authority_map(driver.app().world()) == closed, "闭合重做后权威墙不一致");
    use country_core::systems::save::save_game::{SaveGameCmd, SavingGameComplete};
    let session = driver.app().world().resource::<ActiveSession>().id;
    let folder = session.save_folder_path();
    let saved_walls = m6_authority_map(driver.app().world());
    let saved_history = serde_json::to_string(&country_core::resources::history::serialized::SavedMultiEdit(
        driver.app().world().resource::<History>().ledger().entries.last().context("闭合历史为空")?.clone()))?;
    driver.app_mut().world_mut().send_event(SaveGameCmd);
    let mut saved = false;
    for _ in 0..600 {
        step(&mut driver);
        if driver.app_mut().world_mut().resource_mut::<bevy_ecs::event::Events<SavingGameComplete>>()
            .drain().any(|event| event.path == folder) { saved = true; break; }
    }
    anyhow::ensure!(saved && folder.join("history.json").is_file(), "闭合历史未经生产存档worker写入");
    drop(driver); drop(_bridge); drop(_shaders); drop(_device);
    let status = std::process::Command::new(std::env::current_exe()?)
        .args(["--exact", "m06_real_ui_draw_closed_wall_undo_redo_and_save", "--nocapture", "--test-threads=1"])
        .env("TG_M6_CLOSED_CHILD", "1").env("TG_M6_CLOSED_FOLDER", folder)
        .env("TG_M6_CLOSED_SESSION", serde_json::to_string(&session)?)
        .env("TG_M6_CLOSED_WALLS", serde_json::to_string(&saved_walls)?)
        .env("TG_M6_CLOSED_HISTORY", saved_history).status()?;
    anyhow::ensure!(status.success(), "闭合墙新进程恢复失败: {status}");
    Ok(())
}

fn m6_closed_child_recovery() -> Result<()> {
    use country_core::{meta_commander::{MetaCommand, MetaCommander}, resources::session::SessionId};
    fn copy_tree(from: &std::path::Path, to: &std::path::Path) -> Result<()> {
        std::fs::create_dir_all(to)?;
        for entry in std::fs::read_dir(from)? {
            let entry = entry?; let target = to.join(entry.file_name());
            if entry.file_type()?.is_dir() { copy_tree(&entry.path(), &target)?; }
            else { std::fs::copy(entry.path(), target)?; }
        }
        Ok(())
    }
    let session: SessionId = serde_json::from_str(&std::env::var("TG_M6_CLOSED_SESSION")?)?;
    let source = PathBuf::from(std::env::var_os("TG_M6_CLOSED_FOLDER").context("缺少闭合墙存档")?);
    let BootedSession { mut driver, context, bridge: _bridge, shaders: _shaders, device: _device, _boot } = boot_shipped_session()?;
    copy_tree(&source, &session.save_folder_path())?;
    driver.app_mut().world_mut().resource_mut::<MetaCommander>().extend_script([MetaCommand::load_session(session, false)]);
    let mut saw_reload = false; let mut loaded = false;
    for _ in 0..1200 {
        context.begin_frame(egui::RawInput::default()); driver.app_mut().update(); let _ = context.end_frame();
        let world = driver.app().world();
        let loading = world.resource::<bevy_state::prelude::State<country_core::app_loading_state::AppLoadingState>>().get();
        saw_reload |= *loading == country_core::app_loading_state::AppLoadingState::Reload;
        if saw_reload && *loading == country_core::app_loading_state::AppLoadingState::Game
            && world.get_resource::<ActiveSession>().is_some_and(|s| s.id == session)
            && world.resource::<LoadGameManager>().options.is_none()
            && world.resource::<country_core::resources::history::ReplayBuffer>().is_empty()
            && world.resource::<country_core::systems::history::PendingHistoryCommands>().0.is_empty()
            && world.resource::<country_core::systems::mode_manager::AppMode>().native_tag() != 13 { loaded = true; break; }
    }
    anyhow::ensure!(loaded, "闭合墙新进程未完成生产Reload");
    for _ in 0..2 { context.begin_frame(egui::RawInput::default()); driver.app_mut().update(); let _ = context.end_frame(); }
    let world = driver.app().world();
    let expected: serde_json::Value = serde_json::from_str(&std::env::var("TG_M6_CLOSED_WALLS")?)?;
    anyhow::ensure!(m6_authority_map(world) == expected, "闭合墙新进程权威状态不一致");
    let mut expected: serde_json::Value = serde_json::from_str(&std::env::var("TG_M6_CLOSED_HISTORY")?)?;
    let entry = world.resource::<country_core::resources::history::History>().ledger().entries.last().context("恢复闭合历史为空")?;
    let wire = serde_json::to_string(&country_core::resources::history::serialized::SavedMultiEdit(entry.clone()))?;
    let mut actual: serde_json::Value = serde_json::from_str(&wire)?;
    canonicalize_decorator_diff_map(&mut expected); canonicalize_decorator_diff_map(&mut actual);
    if let Some((path, expected, actual)) = m6_first_json_difference(&expected, &actual) {
        anyhow::bail!("闭合历史差异 {path}: expected={expected}, actual={actual}");
    }
    Ok(())
}
