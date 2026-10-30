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

#[derive(bevy_ecs::prelude::Resource, Clone, Copy, Debug, Default)]
struct M2TerrainSyntheticInput {
    active: bool,
    phase: u8,
    position: glam::Vec3,
}

#[derive(bevy_ecs::prelude::Resource, Clone, Copy, Debug, Default)]
struct M2HistorySyntheticInput {
    undo: country_core::input::action_set::DigitalActionPhase,
    redo: country_core::input::action_set::DigitalActionPhase,
}

fn m2_inject_history_input(
    input: bevy_ecs::prelude::Res<M2HistorySyntheticInput>,
    mut actions: bevy_ecs::prelude::ResMut<country_core::input::action_set::ActionSetStates>,
) {
    use country_core::input::action_set::DigitalActionState;
    for (slot, phase) in [(16, input.undo), (17, input.redo)] {
        actions.digital[slot] = DigitalActionState { phase, timer_seconds: 0.0 };
    }
}

fn m2_trace_history_commands(
    mut commands: bevy_ecs::prelude::EventReader<
        country_core::systems::mode_manager::HistoryCommand,
    >,
) {
    for command in commands.read() {
        eprintln!("M2_CHILD_HISTORY_EVENT {:?}", command.bytes);
    }
}

/// The test supplies only the already-recovered UI and canonical cursor
/// inputs. Terrain drawing, W3 submission, W5 readback and rendering remain
/// in their production schedules.
fn m2_inject_terrain_input(
    input: bevy_ecs::prelude::Res<M2TerrainSyntheticInput>,
    mut mode: bevy_ecs::prelude::ResMut<country_core::systems::mode_manager::AppMode>,
    mut terrain: bevy_ecs::prelude::ResMut<
        country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast,
    >,
    mut ui: bevy_ecs::prelude::ResMut<country_core::systems::ui::resources::ui_state::UiState>,
) {
    if !input.active {
        return;
    }
    *mode = country_core::systems::mode_manager::AppMode::Terrain;
    terrain.update(Some(input.position));
    use country_core::systems::{
        terrain::ui_draw_terrain::{
            DRAW_CANVAS_ARGUMENT, DRAW_CANVAS_LAYER, DRAW_CANVAS_NAME,
        },
        ui::resources::ui_state::{UiInteraction, WidgetId},
    };
    let widget = WidgetId::clearing_canvas(DRAW_CANVAS_NAME);
    // A real picker publishes this canvas before the next frame's history
    // start. Use the same constructor and retain its Arc as active_params.
    ui.clearing_canvas_w_layer_and_sound(
        DRAW_CANVAS_NAME,
        DRAW_CANVAS_ARGUMENT,
        DRAW_CANVAS_LAYER,
    );
    let params = ui.widget(widget);
    let payload = ui.widget_context(widget).intersecting();
    ui.replace_interaction_full(
        UiInteraction {
            native_tag: 2,
            native_phase: input.phase,
            widget,
        },
        params,
        payload,
    );
}

#[derive(bevy_ecs::prelude::Resource)]
struct M1DrawnBrickBuffer(std::sync::Arc<rhapsody::resource::Buffer>);

#[derive(bevy_ecs::prelude::Resource, Default)]
struct M6DrawnWallColorBuffer(Option<std::sync::Arc<rhapsody::resource::Buffer>>);

fn m6_assert_wall_color_gpu(
    world: &bevy_ecs::world::World,
    bridge: &CoreRenderBridge,
    device: &std::sync::Arc<rhapsody::device::Device>,
) -> Result<()> {
    use country_core::{asset_libraries::ssbo_library::AssetSsboLibrary,
        resources::color_id::{WallColorIdSsbo, WallColorIds}};
    let storage = world.resource::<AssetSsboLibrary>().get::<WallColorIdSsbo>()
        .context("wall color SSBO absent")?;
    let current = bridge.hosts().buffers.managed_buffer(&storage.buffer)
        .context("physical wall color SSBO absent")?.shared();
    let drawn = world.resource::<M6DrawnWallColorBuffer>().0.as_ref()
        .context("no production raster wall color observation")?;
    anyhow::ensure!(std::sync::Arc::ptr_eq(&current, drawn), "wall color producer/draw owners differ");
    let expected: &[u8] = bytemuck::cast_slice(&world.resource::<WallColorIds>().0);
    let actual = m6_read_diagnostic_native_range(device, drawn, 0, expected.len())?;
    anyhow::ensure!(actual == expected, "drawn wall color SSBO differs from complete authoritative table");
    Ok(())
}

fn run_m2_production_frame(
    driver: &mut tiny_glade::game_loop::GameFrameDriver,
    context: &egui::Context,
    bridge: &mut CoreRenderBridge,
    capture: Option<(
        &str,
        std::sync::Arc<parking_lot::Mutex<Vec<(String, u32, u32, Vec<u8>)>>>,
    )>,
) -> Result<Vec<String>> {
    let observe_bricks = capture.as_ref().is_some_and(|(label, _)|
        *label == "m2-stable" || *label == "m2-edited");
    let mean_label = capture.as_ref().filter(|(label, _)|
        std::env::var_os("TG_M2_DUMP_DIR").is_some()
            && matches!(*label, "m2-saved" | "m2-reloaded-matched"
                | "m2-parent-canonical-inputs" | "m2-child-canonical-inputs"))
        .map(|(label, _)| (*label).to_owned());
    context.begin_frame(egui::RawInput::default());
    driver.app_mut().update();
    let _ = context.end_frame();
    let world = driver.app_mut().world_mut();
    if let Some((label, sink)) = capture {
        let label = label.to_owned();
        let lighting_label = format!("{label}-terrain-lighting");
        let lighting_sink = sink.clone();
        tiny_glade::screenshot::gather_screenshot_requests(world,
            [tiny_glade::screenshot::ScreenshotRequest {
                purpose: tiny_glade::screenshot::ScreenshotPurpose::TerrainLightingDiagnostic,
                dimensions: None,
                sink: tiny_glade::screenshot::ScreenshotSink::Rgba(std::sync::Arc::new(
                    move |w, h, rgba| lighting_sink.lock().push((lighting_label.clone(), w, h, rgba)),
                )),
                completion: None,
            }])?;
        tiny_glade::screenshot::gather_screenshot_requests(
            world,
            [tiny_glade::screenshot::ScreenshotRequest {
                purpose: tiny_glade::screenshot::ScreenshotPurpose::MenuBackground,
                dimensions: None,
                sink: tiny_glade::screenshot::ScreenshotSink::Rgba(std::sync::Arc::new(
                    move |w, h, rgba| sink.lock().push((label.clone(), w, h, rgba)),
                )),
                completion: None,
            }],
        )?;
    }
    #[cfg(feature = "surface-compare-test")]
    let raw_stages_armed = mean_label.is_some() && std::env::var_os("TG_M2_RAW_INPUTS").is_some();
    #[cfg(feature = "surface-compare-test")]
    if raw_stages_armed { tiny_glade::screenshot::shared_texture_capture::arm_raw_stages(world)?; }
    let ready = tiny_glade::production_renderer::prepare_headless_gameplay_frame(bridge, world)
        .context("prepare M2 production world frame")?;
    bridge.prepare_test_execution()?;
    let frame = ready.execute(bridge, world)?;
    // The lighting shader must read the same actual graph resources whose
    // contents are copied into the two terrain-height readback buffers.
    let graph = &frame.receipt.snapshot;
    if world.contains_resource::<M6DrawnWallColorBuffer>() {
        use country_core::{asset_libraries::ssbo_library::AssetSsboLibrary,
            resources::color_id::WallColorIdSsbo};
        let storage = world.resource::<AssetSsboLibrary>().get::<WallColorIdSsbo>()
            .context("wall color producer absent during raster observation")?;
        let owner = bridge.hosts().buffers.managed_buffer(&storage.buffer)
            .context("wall color physical producer absent")?.shared();
        let logical = bridge.with_session(|session| graph.physical_mapping.iter().filter_map(|mapping| {
            match mapping.token.and_then(|token| session.backend.resource_clone(token)) {
                Some(rhapsody::rg::GpuResource::Buffer(actual)) if std::sync::Arc::ptr_eq(&actual, &owner)
                    => Some(mapping.logical),
                _ => None,
            }
        }).collect::<std::collections::HashSet<_>>());
        anyhow::ensure!(graph.pass_order.iter().enumerate().any(|(i, name)|
            name.starts_with("G-buffer") && graph.accesses[graph.pass_ranges[i].clone()].iter()
                .any(|access| logical.contains(&access.resource_index)
                    && (access.access_code == 0x37 || !rhapsody::native::is_write(access.access_code)))),
            "production G-buffer does not read the actual wall color producer");
        world.resource_mut::<M6DrawnWallColorBuffer>().0 = Some(owner);
    }
    let input_owners = if mean_label.is_some() {
        m2_render_input_owners(world, bridge, graph)?
    } else { Vec::new() };
    let texture_owners = if mean_label.is_some() && std::env::var_os("TG_M2_RAW_INPUTS").is_some() {
        m2_render_texture_owners(world, bridge, graph)?
    } else { Vec::new() };
    let mean_owner = if mean_label.is_some() {
        use country_core::{asset_libraries::ssbo_library::AssetSsboLibrary,
            systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo};
        let library = world.resource::<AssetSsboLibrary>();
        let handle = library.get_buffer::<ShadowMaskMeanSsbo>().context("shadow mean owner missing")?;
        let owner = bridge.hosts().buffers.managed_buffer(handle)
            .context("shadow mean physical buffer missing")?.shared();
        let logical = bridge.with_session(|session| graph.physical_mapping.iter().filter_map(|mapping| {
            match mapping.token.and_then(|token| session.backend.resource_clone(token)) {
                Some(rhapsody::rg::GpuResource::Buffer(actual)) if std::sync::Arc::ptr_eq(&actual, &owner)
                    => Some(mapping.logical),
                _ => None,
            }
        }).collect::<std::collections::HashSet<_>>());
        for expected_pass in ["Shadow mask mean", "Final blit"] {
            anyhow::ensure!(graph.pass_order.iter().enumerate().any(|(i, name)|
                name == expected_pass && graph.accesses[graph.pass_ranges[i].clone()].iter()
                    .any(|access| logical.contains(&access.resource_index))),
                "shadow mean actual owner absent from {expected_pass}");
        }
        Some(owner)
    } else { None };
    if observe_bricks {
        use country_core::{asset_libraries::ssbo_library::AssetSsboLibrary,
            geometry::instanced_wall::BrickTransformSsbo};
        let storage = world.resource::<AssetSsboLibrary>().get::<BrickTransformSsbo>()
            .context("brick storage absent in observed frame")?;
        let producer = bridge.hosts().buffers.managed_buffer(&storage.buffer.buffer)
            .context("brick producer physical owner absent")?.shared();
        let matches = bridge.with_session(|session| graph.physical_mapping.iter().filter_map(|mapping| {
            let token = mapping.token?;
            match session.backend.resources.get(&token)? {
                rhapsody::rg::GpuResource::Buffer(owner) if std::sync::Arc::ptr_eq(owner, &producer) => Some(mapping.logical),
                _ => None,
            }
        }).collect::<std::collections::HashSet<_>>());
        let owner_accesses: Vec<_> = graph.pass_order.iter().enumerate().flat_map(|(index, name)| {
            graph.accesses[graph.pass_ranges[index].clone()].iter()
                .filter(|access| matches.contains(&access.resource_index))
                .map(move |access| (name.clone(), access.resource_index, access.access_code))
        }).collect();
        let raster_reads: Vec<_> = graph.pass_order.iter().enumerate().filter_map(|(index, name)| {
            let raster = name == "Shadows" || name.starts_with("G-buffer") || name.starts_with("Forward")
                || name == "Velocity" || name == "Velocity OC/2" || name.starts_with("Late ");
            (raster && graph.accesses[graph.pass_ranges[index].clone()].iter()
                .any(|access| matches.contains(&access.resource_index)
                    // General (0x37) is conservative MEMORY_READ|MEMORY_WRITE;
                    // external draw SSBOs intentionally use it.
                    && (access.access_code == 0x37 || !rhapsody::native::is_write(access.access_code)))).then_some(name.clone())
        }).collect();
        anyhow::ensure!(!raster_reads.is_empty(),
            "brick physical owner is absent from actual raster pass accesses (matching logical resources: {matches:?}; accesses: {owner_accesses:?})");
        eprintln!("M1 brick production raster owner reads: {raster_reads:?}");
        world.insert_resource(M1DrawnBrickBuffer(producer));
    }
    if let (Some(read), Some(light)) = (
        graph.pass_order.iter().position(|name| name == "Read terrain height"),
        graph.pass_order.iter().position(|name| name == "Calculate Terrain Lighting Texture"),
    ) {
        let read_inputs = &graph.accesses[graph.pass_ranges[read].clone()];
        let light_inputs = &graph.accesses[graph.pass_ranges[light].clone()];
        let shared: std::collections::HashSet<_> = read_inputs.iter()
            .filter(|item| item.access_code == 0x19 && light_inputs.iter()
                .any(|other| other.access_code == 0x19 && other.resource_index == item.resource_index))
            .map(|item| item.resource_index).collect();
        anyhow::ensure!(shared.len() >= 2,
            "terrain lighting does not read both height resources used by the production readback: {shared:?}");
    }
    let passes = frame
        .receipt
        .snapshot
        .pass_order
        .iter()
        .map(ToString::to_string)
        .collect();
    tiny_glade::screenshot::finish_screenshot_capture(world)?;
    world
        .resource_mut::<country_core::render::RenderTemporalState>()
        .frame_render_returned();
    #[cfg(not(feature = "surface-compare-test"))]
    bridge.with_session(|session| session.wait_frame(frame.receipt.frame, 120_000_000_000))?;
    #[cfg(feature = "surface-compare-test")]
    let raw_stages = if raw_stages_armed {
        tiny_glade::screenshot::shared_texture_capture::take_raw_stages(world, bridge)?
    } else {
        bridge.with_session(|session| session.wait_frame(frame.receipt.frame, 120_000_000_000))?;
        Vec::new()
    };
    if let (Some(label), Some(owner)) = (mean_label, mean_owner) {
        let device = bridge.hosts().buffers.graphics_device().clone();
        let bytes = m6_read_diagnostic_native_range(&device, &owner, 0, 4)?;
        let bits = u32::from_le_bytes(bytes.as_slice().try_into()?);
        let dir = PathBuf::from(std::env::var_os("TG_M2_DUMP_DIR").expect("mean diagnostic directory"));
        std::fs::create_dir_all(&dir)?;
        std::fs::write(dir.join(format!("{label}-shadow-mean.json")),
            serde_json::to_vec_pretty(&serde_json::json!({"bits":bits,"value":f32::from_bits(bits)}))?)?;
        m2_dump_render_input_bytes(bridge, &dir, &label, input_owners)?;
        m2_dump_render_texture_bytes(bridge, &dir, &label, texture_owners)?;
        #[cfg(feature = "surface-compare-test")]
        {
            let mut manifest = Vec::with_capacity(raw_stages.len());
            for stage in raw_stages {
                let file = format!("{label}-stage-{}.bin", stage.role);
                std::fs::write(dir.join(&file), &stage.bytes)?;
                manifest.push(serde_json::json!({"role":stage.role,"extent":stage.extent,
                    "format":stage.format,"aspect":stage.aspect,"bytes":stage.bytes.len(),"file":file,
                    "actual_receipt_owner_confirmed":true,"receipt_frame":frame.receipt.frame}));
            }
            std::fs::write(dir.join(format!("{label}-raw-stages.json")), serde_json::to_vec_pretty(&manifest)?)?;
        }
    }
    drop(frame);
    bridge.collect_completed_frames()?;
    Ok(passes)
}

/// Only existing library registrations are selected. Full allocation bytes are
/// diagnostic evidence, not proof that inactive capacity affects a pixel.
/// Large allocations are recorded as metadata, never truncated as though the
/// prefix were the complete input; targeted brick ranges have a separate probe.
struct M2RenderInputOwner {
    name: String,
    owner: std::sync::Arc<rhapsody::resource::Buffer>,
    count: Option<usize>,
    accesses: Vec<(String, u32, u8)>,
}

fn m2_render_input_owners(
    world: &bevy_ecs::world::World,
    bridge: &mut CoreRenderBridge,
    graph: &rhapsody::rg::Snapshot,
) -> Result<Vec<M2RenderInputOwner>> {
    use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
    let library = world.resource::<AssetSsboLibrary>();
    let mut result = Vec::new();
    for lease in library.snapshot_graph_leases() {
        let name = lease.canonical_type_name.rsplit("::").next().context("M2 registered input type")?;
        if !matches!(name, "ColorLibrarySsbo" | "LeafParticleSsbo" | "ParticleEmitterSsbo" | "LanternSsbo"
            | "SheepAnimationSsbo" | "BrickTransformSsbo") { continue; }
        let owner = bridge.hosts().buffers.managed_buffer(lease.buffer())
            .with_context(|| format!("M2 registered {name} physical owner missing"))?.shared();
        let logical = bridge.with_session(|session| graph.physical_mapping.iter().filter_map(|mapping| {
            match mapping.token.and_then(|token| session.backend.resource_clone(token)) {
                Some(rhapsody::rg::GpuResource::Buffer(actual)) if std::sync::Arc::ptr_eq(&actual, &owner)
                    => Some(mapping.logical),
                _ => None,
            }
        }).collect::<std::collections::HashSet<_>>());
        let accesses = graph.pass_order.iter().enumerate().flat_map(|(index, pass)| {
            graph.accesses[graph.pass_ranges[index].clone()].iter()
                .filter(|access| logical.contains(&access.resource_index))
                .map(move |access| (pass.clone(), access.resource_index, access.access_code))
        }).collect::<Vec<_>>();
        if accesses.is_empty() { continue; }
        // The retained Arc is resolved before wait_frame/retirement, matching
        // the SSBO consumed by this exact submitted production frame.
        result.push(M2RenderInputOwner { name: name.to_owned(), owner,
            count: lease.instance_count(), accesses });
    }
    result.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(result)
}

fn m2_dump_render_input_bytes(
    bridge: &mut CoreRenderBridge,
    dir: &std::path::Path,
    label: &str,
    inputs: Vec<M2RenderInputOwner>,
) -> Result<()> {
    let device = bridge.hosts().buffers.graphics_device().clone();
    let mut manifest = Vec::new();
    let mut total = 0usize;
    for input in inputs {
        let size = usize::try_from(input.owner.size)?;
        let within_bound = total.checked_add(size).is_some_and(|sum| sum <= 32 * 1024 * 1024);
        let file = format!("{label}-input-{}.bin", input.name);
        let capable = input.owner.usage.contains(ash::vk::BufferUsageFlags::TRANSFER_SRC);
        let readable = capable && within_bound;
        if readable {
            total += size;
            let bytes = m6_read_diagnostic_native_range(&device, &input.owner, 0, size)?;
            std::fs::write(dir.join(&file), bytes)?;
        }
        manifest.push(serde_json::json!({"type":input.name,"allocation_bytes":size,
            "instance_count":input.count,"actual_receipt_owner_confirmed":true,
            "accesses":input.accesses,"file":readable.then_some(file),
            "capture_status":if !within_bound { "allocation exceeds bounded capture; no bytes captured" }
                else if readable { "complete allocation raw bytes" }
                else { "TRANSFER_SRC capability absent; bytes not captured" }}));
    }
    std::fs::write(dir.join(format!("{label}-render-inputs.json")),
        serde_json::to_vec_pretty(&manifest)?)?;
    Ok(())
}

/// These two outputs are persistent preset allocations. Retain their actual
/// receipt owners before retirement; unlike transient graph textures they are
/// not eligible for aliasing between their producer and this completed frame.
struct M2RenderTextureOwner {
    name: &'static str,
    owner: std::sync::Arc<rhapsody::resource::Texture>,
    aspect: ash::vk::ImageAspectFlags,
    accesses: Vec<(String, u32, u8)>,
}

fn m2_render_texture_owners(
    world: &bevy_ecs::world::World,
    bridge: &mut CoreRenderBridge,
    graph: &rhapsody::rg::Snapshot,
) -> Result<Vec<M2RenderTextureOwner>> {
    use country_core::{asset_libraries::texture_library::AssetTextureLibrary,
        resources::texture_presets::TexturePreset};
    let library = world.resource::<AssetTextureLibrary>();
    let mut result = Vec::new();
    for (preset, name, aspect) in [
        (0x11, "shadow-atlas", ash::vk::ImageAspectFlags::DEPTH),
        (8, "terrain-lighting", ash::vk::ImageAspectFlags::COLOR),
    ] {
        let state = library.preset(TexturePreset(preset)).context("M2 persistent preset missing")?;
        let owner = bridge.hosts().textures.get(&state.texture)
            .context("M2 persistent preset physical owner missing")?.shared();
        let logical = bridge.with_session(|session| graph.physical_mapping.iter().filter_map(|mapping| {
            match mapping.token.and_then(|token| session.backend.resource_clone(token)) {
                Some(rhapsody::rg::GpuResource::Image(actual)) if std::sync::Arc::ptr_eq(&actual, &owner)
                    => Some(mapping.logical),
                _ => None,
            }
        }).collect::<std::collections::HashSet<_>>());
        let accesses = graph.pass_order.iter().enumerate().flat_map(|(index, pass)| {
            graph.accesses[graph.pass_ranges[index].clone()].iter()
                .filter(|access| logical.contains(&access.resource_index))
                .map(move |access| (pass.clone(), access.resource_index, access.access_code))
        }).collect::<Vec<_>>();
        anyhow::ensure!(accesses.iter().any(|(pass, _, _)| pass == "Calculate Terrain Lighting Texture"),
            "M2 {name} owner is absent from the observed terrain lighting pass");
        if preset == 0x11 {
            anyhow::ensure!(accesses.iter().any(|(pass, _, _)| pass == "Shadows"),
                "M2 atlas owner is absent from the observed shadow pass");
            anyhow::ensure!(owner.desc.array_elements == 3, "M2 shadow atlas must include all three cascades");
        }
        result.push(M2RenderTextureOwner { name, owner, aspect, accesses });
    }
    Ok(result)
}

fn m2_dump_render_texture_bytes(
    bridge: &CoreRenderBridge,
    dir: &std::path::Path,
    label: &str,
    textures: Vec<M2RenderTextureOwner>,
) -> Result<()> {
    use ash::vk;
    use rhapsody::{command::{buffer_access, image_access}, native::AccessScope,
        resource::{Buffer, BufferDesc}, tracking::OwnershipRange,
        transfer::with_setup_command};
    let device = bridge.hosts().buffers.graphics_device();
    let queue = device.queue(rhapsody::device::QueueClass::Graphics).context("M2 readback graphics queue missing")?;
    let mut manifest = Vec::new();
    for texture in textures {
        let desc = &texture.owner.desc;
        anyhow::ensure!(matches!((desc.format, texture.aspect),
            (vk::Format::D32_SFLOAT, vk::ImageAspectFlags::DEPTH)
            | (vk::Format::B10G11R11_UFLOAT_PACK32, vk::ImageAspectFlags::COLOR)),
            "M2 raw preset capture has an unsupported format/aspect");
        anyhow::ensure!(desc.depth == 1 && desc.sample_count == 1
            && desc.usage.contains(vk::ImageUsageFlags::TRANSFER_SRC),
            "M2 raw preset capture requires a single-sample persistent transfer source");
        let mut size = 0u64;
        let mut regions = Vec::with_capacity(usize::from(desc.mip_levels));
        let mut mips = Vec::with_capacity(usize::from(desc.mip_levels));
        for mip in 0..u32::from(desc.mip_levels) {
            let width = (desc.width >> mip).max(1);
            let height = (desc.height >> mip).max(1);
            let bytes = u64::from(width).checked_mul(u64::from(height))
                .and_then(|bytes| bytes.checked_mul(u64::from(desc.array_elements)))
                .and_then(|bytes| bytes.checked_mul(4))
                .context("M2 raw preset mip size overflow")?;
            regions.push(vk::BufferImageCopy::default().buffer_offset(size)
                .image_subresource(vk::ImageSubresourceLayers::default()
                    .aspect_mask(texture.aspect).mip_level(mip).layer_count(desc.array_elements))
                .image_extent(vk::Extent3D { width, height, depth: 1 }));
            mips.push(serde_json::json!({"mip":mip,"width":width,"height":height,
                "layers":desc.array_elements,"offset":size,"bytes":bytes}));
            size = size.checked_add(bytes).filter(|bytes| *bytes <= 64 * 1024 * 1024)
                .context("M2 raw preset capture exceeds its 64 MiB bound")?;
        }
        anyhow::ensure!(size > 0, "M2 raw preset capture is empty");
        let output = std::sync::Arc::new(Buffer::new(device.clone(),
            &BufferDesc::new_gpu_to_cpu(size, vk::BufferUsageFlags::TRANSFER_DST))?);
        // SAFETY: the exact receipt owner is persistent and the frame has
        // completed. Both accesses are registered with the same device's
        // state tracker; the regions cover every mip and its actual layers.
        // No preset contents, render counter or gameplay state is changed.
        with_setup_command(device, &[], |command| unsafe {
            image_access(command, &texture.owner, queue, OwnershipRange::Image {
                aspects: texture.aspect.as_raw(), base_mip: 0, mip_count: u32::from(desc.mip_levels),
                base_layer: 0, layer_count: desc.array_elements,
            }, AccessScope { stages: vk::PipelineStageFlags::TRANSFER,
                access: vk::AccessFlags::TRANSFER_READ, write: false }, vk::ImageLayout::TRANSFER_SRC_OPTIMAL)?;
            buffer_access(command, &output, queue, 0, size,
                AccessScope { stages: vk::PipelineStageFlags::TRANSFER,
                    access: vk::AccessFlags::TRANSFER_WRITE, write: true })?;
            device.raw_device().cmd_copy_image_to_buffer(command.raw_handle(), texture.owner.raw_handle(),
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL, output.raw_handle(),
                &regions);
            buffer_access(command, &output, queue, 0, size,
                AccessScope { stages: vk::PipelineStageFlags::HOST,
                    access: vk::AccessFlags::HOST_READ, write: false })
        })?.wait(rhapsody::transfer::SETUP_WAIT_TIMEOUT_NS)?;
        let mut bytes = vec![0; usize::try_from(size)?];
        output.read_bytes(0, &mut bytes)?;
        let file = format!("{label}-raw-{}.bin", texture.name);
        std::fs::write(dir.join(&file), bytes)?;
        manifest.push(serde_json::json!({"role":texture.name,"file":file,
            "format":desc.format.as_raw(),"aspect":texture.aspect.as_raw(),
            "width":desc.width,"height":desc.height,"layers":desc.array_elements,
            "mips":mips,"bytes":size,"actual_receipt_owner_confirmed":true,
            "persistent_preset":true,"accesses":texture.accesses}));
    }
    std::fs::write(dir.join(format!("{label}-raw-textures.json")), serde_json::to_vec_pretty(&manifest)?)?;
    Ok(())
}

fn copy_test_tree(from: &std::path::Path, to: &std::path::Path) -> Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_test_tree(&entry.path(), &target)?;
        } else {
            std::fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

fn m2_saved_history_values(
    entries: &[country_core::resources::history::serialized::SavedMultiEdit],
) -> Result<Vec<serde_json::Value>> {
    entries
        .iter()
        .map(|entry| {
            let mut value = serde_json::to_value(entry)?;
            canonicalize_decorator_diff_map(&mut value);
            Ok(value)
        })
        .collect()
}

fn m2_camera_parameters(
    world: &bevy_ecs::world::World,
) -> Result<serde_json::Value> {
    Ok(serde_json::to_value(
        world
            .resource::<country_core::systems::camera_rig::main_camera::MainCamera>()
            .store_parameters(),
    )?)
}

fn m2_assert_sheep_snapshot_bits(
    expected: &country_core::resources::critters::sheep::control::SheepControlSnapshot,
    actual: &country_core::resources::critters::sheep::control::SheepControlSnapshot,
) -> Result<()> {
    use country_core::resources::critters::sheep::SheepMoveState;
    anyhow::ensure!(expected.sheep.len() == actual.sheep.len(), "M2 saved sheep count differs");
    for (index, (expected, actual)) in expected.sheep.iter().zip(&actual.sheep).enumerate() {
        anyhow::ensure!(expected.position.to_array().map(f32::to_bits)
            == actual.position.to_array().map(f32::to_bits),
            "M2 saved sheep {index} position bits differ");
        anyhow::ensure!(expected.goal.map(|value| value.to_array().map(f32::to_bits))
            == actual.goal.map(|value| value.to_array().map(f32::to_bits)),
            "M2 saved sheep {index} goal bits differ");
        let same_movement = match (expected.move_state, actual.move_state) {
            (SheepMoveState::Idle { timeout_s: expected }, SheepMoveState::Idle { timeout_s: actual })
                => expected.to_bits() == actual.to_bits(),
            (SheepMoveState::Walking { timeout_s: et, goal_change_timeout: eg, force_reach_goal: ef },
                SheepMoveState::Walking { timeout_s: at, goal_change_timeout: ag, force_reach_goal: af })
                => et.to_bits() == at.to_bits() && eg.to_bits() == ag.to_bits() && ef == af,
            _ => false,
        };
        anyhow::ensure!(same_movement, "M2 saved sheep {index} movement bits differ");
    }
    Ok(())
}

/// Optional diagnostics are taken at the game-world frame that requests the
/// capture. FrameConstants are retained in full Debug form, including views;
/// exposure is also exposed separately for easy parent/child comparison.
fn m2_dump_frame_state(world: &bevy_ecs::world::World, label: &str) -> Result<()> {
    let Some(dir) = std::env::var_os("TG_M2_DUMP_DIR") else { return Ok(()); };
    let dir = PathBuf::from(dir);
    std::fs::create_dir_all(&dir)?;
    let frame = world.resource::<country_core::render::frame_constants::FrameConstants>();
    let render = world.resource::<country_core::render::render_config::FrameRenderConfig>();
    let camera = world.resource::<country_core::systems::camera_rig::main_camera::MainCamera>();
    let tod = world.resource::<country_core::resources::time_of_day::TimeOfDay>();
    let config = world.resource::<country_core::game_config::GameConfig>();
    let formation = world.resource::<country_core::resources::image_formation::ImageFormationConfigs>();
    // These are observations of live render inputs, not persisted fields or
    // replacement state. Entity ids are deliberately excluded across processes.
    let mut sheep = world.iter_entities().filter_map(|entity| {
        use country_core::{resources::critters::{sheep::{Sheep, SheepWalkRecord}, CritterGoal},
            utils::components::transform::Transform};
        let value = entity.get::<Sheep>()?;
        Some(serde_json::json!({
            "sheep": value,
            "transform": format!("{:?}", entity.get::<Transform>()),
            "previous_transform": format!("{:?}", entity.get::<country_core::components::drawable::PrevTransform>()),
            "walk": format!("{:?}", entity.get::<SheepWalkRecord>()),
            "goal": format!("{:?}", entity.get::<CritterGoal>()),
        }))
    }).collect::<Vec<_>>();
    sheep.sort_by_cached_key(serde_json::Value::to_string);
    let data = serde_json::json!({
        "camera_parameters": m2_camera_parameters(world)?,
        "camera_matrices": ([0, 0x40, 0x80, 0xc0].map(|offset| camera.read_matrix(offset).to_cols_array())),
        "frame_constants": format!("{frame:#?}"),
        "exposure_ev_mult": frame.ev_mult,
        "time_of_day": format!("{tod:#?}"),
        "game_clock_runtime": format!("{:#?}", world.resource::<country_core::resources::game_clock::GameClock>()),
        "app_mode": format!("{:?}", world.resource::<country_core::systems::mode_manager::AppMode>()),
        "terrain_cursor": format!("{:?}", world.resource::<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>()),
        "frame_render_config": format!("{render:#?}"),
        "video_settings": &config.video,
        "image_formation": formation,
        "weather": format!("{:?}", world.resource::<country_core::resources::weather::Weather>()),
        "shader_refactoring": format!("{:?}", world.resource::<country_core::resources::shader_refactoring_state::ShaderRefactoringState>()),
        "bevy_time": format!("{:?}", world.resource::<bevy_time::Time>()),
        "color_library_cpu_bytes": bytemuck::bytes_of(world.resource::<country_core::asset_libraries::color_library::ColorLibrarySsbo>()),
        "leaf_particle_count": world.resource::<country_core::resources::glade_settings::GladeSettings>().leaf_particle_count,
        "loaded_particle_emitter_count": world.get_resource::<country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCount>().map(|count|count.0),
        "sheep_runtime": sheep,
    });
    std::fs::write(dir.join(format!("{label}-state.json")), serde_json::to_vec_pretty(&data)?)?;
    Ok(())
}

fn m2_dump_image(label: &str, width: u32, height: u32, rgba: &[u8]) -> Result<()> {
    let Some(dir) = std::env::var_os("TG_M2_DUMP_DIR") else { return Ok(()); };
    let dir = PathBuf::from(dir);
    std::fs::create_dir_all(&dir)?;
    std::fs::write(dir.join(format!("{label}.rgba")), rgba)?;
    std::fs::write(dir.join(format!("{label}.json")), serde_json::to_vec_pretty(&serde_json::json!({"width":width,"height":height}))?)?;
    Ok(())
}

fn m2_first_json_difference(
    expected: &[serde_json::Value],
    actual: &[serde_json::Value],
) -> Option<(String, String, String)> {
    if expected.len() != actual.len() {
        return Some((
            "/length".to_owned(),
            expected.len().to_string(),
            actual.len().to_string(),
        ));
    }
    expected
        .iter()
        .zip(actual)
        .enumerate()
        .find_map(|(index, (expected, actual))| {
            m6_first_json_difference(expected, actual)
                .map(|(path, expected, actual)| (format!("/{index}{path}"), expected, actual))
        })
}

fn m2_latest_heights(
    world: &bevy_ecs::world::World,
) -> Option<(
    Vec<u16>,
    Vec<u16>,
    u64,
    country_core::render::gpu_readback::TerrainReadbackEpoch,
)> {
    use country_core::{
        render::gpu_readback::GpuReadbacks,
        resources::terrain_height::{TerrainHeightSsbo, VisualTerrainHeightSsbo},
    };
    let readbacks = world.get_resource::<GpuReadbacks>()?;
    let height = readbacks.get::<TerrainHeightSsbo>()?;
    let visual = readbacks.get::<VisualTerrainHeightSsbo>()?;
    let epoch = height.epoch?;
    if visual.epoch != Some(epoch) {
        return None;
    }
    Some((
        bytemuck::cast_slice(height.data).to_vec(),
        bytemuck::cast_slice(visual.data).to_vec(),
        height.frame.min(visual.frame),
        epoch,
    ))
}

fn contains_ordered_passes(passes: &[String], required: &[&str]) -> bool {
    passes.iter().try_fold(0, |next, pass| {
        if required.get(next).is_some_and(|required| pass == required) {
            Some(next + 1)
        } else {
            Some(next)
        }
    }) == Some(required.len())
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
            validation: std::env::var_os("TG_M6_VK_VALIDATION").is_some_and(|value| value == "1"),
            ..rhapsody::device::DeviceConfig::headless()
        })
        .context("strict gameplay physical device")?;
    let shaders = app.world().resource::<ApplicationShaderLibrary>().0.clone();
    let bridge = if std::env::var_os("TG_M2_RAW_INPUTS").is_some() {
        CoreRenderBridge::new_with_extra_usage(device.clone(), installed.clone(), shaders.clone(),
            ash::vk::BufferUsageFlags::TRANSFER_SRC, ash::vk::ImageUsageFlags::TRANSFER_SRC)?
    } else { CoreRenderBridge::new(device.clone(), installed.clone(), shaders.clone())? };
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
    let mean_diagnostic = std::env::var_os("TG_M2_DUMP_DIR").is_some();
    let diagnostic = (diagnostic_nani || mean_diagnostic).then(|| std::sync::Arc::new(
        M6NaniDiagnosticDevice(bridge.hosts().buffers.clone(),
            (!diagnostic_nani).then_some("ShadowMaskMeanSsbo"))));
    if let Some(device) = diagnostic.as_ref().filter(|_| diagnostic_nani) {
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
        bridge.prepare_test_execution()?;
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
        bridge.prepare_test_execution()?;
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

/// Read only the target wall's currently owned physical brick ranges after
/// production frame completion; compare every valid 96-byte record to the
/// canonical producer's source, not to an unrelated drawable or empty owner.
fn m1_wall_gpu_bricks(
    world: &bevy_ecs::world::World,
    bridge: &CoreRenderBridge,
    device: &std::sync::Arc<rhapsody::device::Device>,
    wall: country_core::resources::walls::WallId,
) -> Result<Vec<u8>> {
    use country_core::{geometry::instanced_wall::{BrickTransformSsbo, InstancedWall},
        resources::walls::public_walls::WallEntities,
        asset_libraries::ssbo_library::AssetSsboLibrary};
    let entity = world.resource::<WallEntities>().wall_entity(wall).context("wall entity missing")?;
    let instance = world.get::<InstancedWall<BrickTransformSsbo>>(entity).context("wall brick owner missing")?;
    anyhow::ensure!(!instance.is_upload_pending(), "wall brick producer still has pending records");
    let allocation = instance.allocation().context("wall brick allocation missing")?;
    let storage = world.resource::<AssetSsboLibrary>().get::<BrickTransformSsbo>()
        .context("brick SSBO missing")?;
    anyhow::ensure!(storage.dirty_bitset.iter().all(|word| word.load(std::sync::atomic::Ordering::Acquire) == 0),
        "brick SSBO retains unsubmitted dirty chunks");
    let ranges = storage.allocation_to_draw_multi_instanced(allocation);
    anyhow::ensure!(!ranges.is_empty(), "target wall has no drawable brick records");
    let drawn = &world.get_resource::<M1DrawnBrickBuffer>()
        .context("no production raster brick owner observation")?.0;
    let current = bridge.hosts().buffers.managed_buffer(&storage.buffer.buffer)
        .context("brick producer physical owner absent")?.shared();
    anyhow::ensure!(std::sync::Arc::ptr_eq(drawn, &current),
        "brick owner changed after the observed production raster frame");
    let mut result = Vec::new();
    for (first, count) in ranges {
        let first = first as usize;
        let count = count as usize;
        let expected: &[u8] = bytemuck::cast_slice(&storage.data[first..first + count]);
        let actual = m6_read_diagnostic_native_range(device, drawn, first * 96, count * 96)?;
        anyhow::ensure!(actual == expected, "wall GPU brick records differ from producer data at instance {first}");
        result.extend_from_slice(&actual);
    }
    Ok(result)
}

#[test]
fn m02_actual_app_terrain_edit_changes_floor_rocky_heights_lighting_and_image() -> Result<()> {
    use bevy_ecs::schedule::IntoScheduleConfigs;
    use country_core::{
        render::gpu_readback::GpuReadbacks,
        resources::terrain_height::{TerrainHeightSsbo, VisualTerrainHeightSsbo},
    };

    let BootedSession {
        mut driver,
        context,
        mut bridge,
        shaders: _shaders,
        device,
        _boot,
    } = boot_shipped_session_with_nani_readback(true)?;
    let world = driver.app_mut().world_mut();
    anyhow::ensure!(
        tiny_glade::screenshot::shared_texture_capture::installed(world),
        "M2 production frame has no screenshot owner"
    );
    tiny_glade::production_renderer::warm_up_gameplay(bridge.warmup_owners()?, world)
        .context("M2 production warmup")?;
    bridge.accept_gameplay_warmup(
        world.resource::<tiny_glade::render_bridge::CompletedGameplayWarmup>(),
    )?;
    // Use the real wall authority constructor; publication and collision are
    // produced by the normal set 19/28 systems in later frames.
    let wall = {
        use country_core::resources::walls::{InnerWalls, WallState};
        let curve = country_core::resources::roofs::geometry::surface::Curve2::try_new(
            vec![[-4.0, 0.0], [4.0, 0.0]], false).expect("valid fixture curve");
        let mut walls = world.resource_mut::<InnerWalls>();
        let wall = walls.new_wall_from_curve(WallState::new(curve, 4.0, 0, 0, false));
        walls.mark_as_non_enclosed(wall);
        wall
    };
    // Independent observers of the same production height publishers. They
    // do not drain the wall system's own subscriber.
    let mut floor_dirty = world.resource_mut::<country_core::resources::terrain_height::TerrainHeightsData<TerrainHeightSsbo>>().subscribe();
    let mut visual_dirty = world.resource_mut::<country_core::resources::terrain_height::TerrainHeightsData<VisualTerrainHeightSsbo>>().subscribe();
    world.insert_resource(M2TerrainSyntheticInput::default());
    world.resource_mut::<country_core::systems::terrain::ui_draw_terrain::TerrainDrawState>().state =
        0;
    driver.app_mut().add_systems(
        bevy_app::Update,
        m2_inject_terrain_input
            .before(
                country_core::systems::history::recording::history_start_recording::<
                    country_core::resources::history::MaybePendingSnapshotId,
                >,
            )
            .before(country_core::systems::terrain::ui_draw_terrain::ui_draw_terrain),
    );

    let captured = std::sync::Arc::new(parking_lot::Mutex::new(Vec::<
        (String, u32, u32, Vec<u8>),
    >::new()));
    let _ = run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;

    let mut baseline = None;
    for _ in 0..10 {
        let _ = run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
        let world = driver.app().world();
        if let Some(readbacks) = world.get_resource::<GpuReadbacks>()
            && let Some(height) = readbacks.get::<TerrainHeightSsbo>()
            && let Some(visual) = readbacks.get::<VisualTerrainHeightSsbo>()
        {
            // Native stores two u16 height cells in every u32 readback word.
            assert_eq!(height.data.len(), 512 * 512 / 2);
            assert_eq!(visual.data.len(), 512 * 512 / 2);
            baseline = Some((
                bytemuck::cast_slice::<_, u16>(height.data).to_vec(),
                bytemuck::cast_slice::<_, u16>(visual.data).to_vec(),
                height.frame.min(visual.frame),
            ));
            break;
        }
    }
    let (baseline_height, baseline_visual, baseline_frame) =
        baseline.context("M2 baseline W5 height readbacks did not arrive")?;
    let _ = run_m2_production_frame(&mut driver, &context, &mut bridge,
        Some(("m2-baseline", captured.clone())))?;
    let _ = run_m2_production_frame(&mut driver, &context, &mut bridge,
        Some(("m2-stable", captured.clone())))?;
    let baseline_wall = driver.app().world().resource::<country_core::resources::walls::public_walls::PublicWalls>()
        .get(wall).context("fixture wall was not published by the real wall system")?.height_outline().to_vec();
    let wall_entity = driver.app().world().resource::<country_core::resources::walls::public_walls::WallEntities>()
        .wall_entity(wall).context("fixture wall entity was not built")?;
    let baseline_collider = driver.app().world().resource::<country_core::systems::collision::world::RaycastWorld>()
        .wall_collider_instance(wall_entity).context("fixture wall collider was not built")?.handle();
    bridge.wait_submits_done(120_000_000_000)?;
    let baseline_bricks = m1_wall_gpu_bricks(driver.app().world(), &bridge, &device, wall)?;
    let mut collider_changes = driver.app_mut().world_mut()
        .resource_mut::<country_core::systems::collision::world::RaycastWorld>()
        .mesh_intersect_collider_diff_publisher().subscribe();
    let mut wall_events = bevy_ecs::event::EventCursor::<country_core::resources::walls::public_walls::OnWallChanged>::default();
    wall_events.read(driver.app().world().resource::<bevy_ecs::event::Events<country_core::resources::walls::public_walls::OnWallChanged>>()).for_each(drop);
    let mut wall_republished = false;
    let _ = floor_dirty.take();
    let _ = visual_dirty.take();
    let baseline_cpu = driver.app().world().resource::<country_core::resources::terrain_height::TerrainHeightsData<TerrainHeightSsbo>>().heights().to_vec();

    let next_index_before = driver
        .app()
        .world()
        .resource::<country_core::resources::terrain_editing::TerrainEditingData>()
        .next_index;
    let mut production_passes = Vec::<Vec<String>>::new();
    for (phase, position) in [
        (3, glam::Vec3::new(-3.0, 0.0, -3.0)),
        (3, glam::Vec3::new(0.0, 0.0, 0.0)),
        (3, glam::Vec3::new(3.0, 0.0, 3.0)),
        (1, glam::Vec3::new(4.0, 0.0, 4.0)),
    ] {
        *driver
            .app_mut()
            .world_mut()
            .resource_mut::<M2TerrainSyntheticInput>() = M2TerrainSyntheticInput {
            active: true,
            phase,
            position,
        };
        production_passes.push(run_m2_production_frame(
            &mut driver,
            &context,
            &mut bridge,
            None,
        )?);
        wall_republished |= wall_events.read(driver.app().world().resource::<bevy_ecs::event::Events<country_core::resources::walls::public_walls::OnWallChanged>>()).any(|event| event.0 == wall);
    }
    *driver
        .app_mut()
        .world_mut()
        .resource_mut::<M2TerrainSyntheticInput>() = M2TerrainSyntheticInput::default();

    {
        let world = driver.app().world();
        let editing =
            world.resource::<country_core::resources::terrain_editing::TerrainEditingData>();
        anyhow::ensure!(
            editing.next_index == next_index_before + 1,
            "production ui_draw_terrain did not create exactly one stroke: {} -> {}",
            next_index_before,
            editing.next_index
        );
        let id = editing.last_stroke_id();
        let stroke = editing
            .strokes
            .get(&id)
            .context("created terrain stroke disappeared")?;
        anyhow::ensure!(
            stroke.points.len() >= 3,
            "terrain UI did not append the dragged points: {:?}",
            stroke.points
        );
        eprintln!(
            "M02 actual ui_draw_terrain stroke={} points={} height={}",
            id.0,
            stroke.points.len(),
            stroke.height_ws
        );
    }

    let mut edited = None;
    for _frame_index in 0..24 {
        production_passes.push(run_m2_production_frame(
            &mut driver,
            &context,
            &mut bridge,
            None,
        )?);
        wall_republished |= wall_events.read(driver.app().world().resource::<bevy_ecs::event::Events<country_core::resources::walls::public_walls::OnWallChanged>>()).any(|event| event.0 == wall);
        let world = driver.app().world();
        if let Some(readbacks) = world.get_resource::<GpuReadbacks>()
            && let Some(height) = readbacks.get::<TerrainHeightSsbo>()
            && let Some(visual) = readbacks.get::<VisualTerrainHeightSsbo>()
            && height.frame.min(visual.frame) > baseline_frame
            && height.epoch.is_some() && height.epoch == visual.epoch
            && height.epoch == Some(country_core::render::gpu_readback::TerrainReadbackEpoch::capture(world))
            && bytemuck::cast_slice::<_, u16>(height.data) != baseline_height.as_slice()
            && bytemuck::cast_slice::<_, u16>(visual.data) != baseline_visual.as_slice()
            && world.resource::<country_core::resources::terrain_height::TerrainHeightsData<TerrainHeightSsbo>>().heights() == bytemuck::cast_slice::<_, u16>(height.data)
            && world.resource::<country_core::resources::terrain_height::TerrainHeightsData<VisualTerrainHeightSsbo>>().heights() == bytemuck::cast_slice::<_, u16>(visual.data)
        {
            edited = Some((
                bytemuck::cast_slice::<_, u16>(height.data).to_vec(),
                bytemuck::cast_slice::<_, u16>(visual.data).to_vec(),
                height.frame.min(visual.frame),
            ));
            break;
        }
    }
    let (edited_height, edited_visual, edited_frame) =
        edited.context("M2 edited W5 height readbacks did not arrive")?;
    anyhow::ensure!(edited_frame > baseline_frame);
    let height_changes = baseline_height
        .iter()
        .zip(&edited_height)
        .filter(|(before, after)| before != after)
        .count();
    let visual_changes = baseline_visual
        .iter()
        .zip(&edited_visual)
        .filter(|(before, after)| before != after)
        .count();
    anyhow::ensure!(
        height_changes > 0,
        "actual terrain stroke did not change the 512x512 floor height authority"
    );
    anyhow::ensure!(
        visual_changes > 0,
        "actual terrain stroke did not change the 512x512 visual/rocky height authority"
    );
    anyhow::ensure!(baseline_cpu != edited_height, "completed floor readback never reached CPU authority");
    let floor_tiles = floor_dirty.take().context("floor publisher did not notify its subscriber")?;
    let visual_tiles = visual_dirty.take().context("visual publisher did not notify its subscriber")?;
    anyhow::ensure!(floor_tiles.tile_indices().next().is_some() && visual_tiles.tile_indices().next().is_some(),
        "terrain height change published empty dirty tiles");
    {
        use country_core::resources::terrain_height::TerrainHeightsData;
        let world = driver.app().world();
        let floor = world.resource::<TerrainHeightsData<TerrainHeightSsbo>>();
        let mut before = TerrainHeightsData::<TerrainHeightSsbo>::new();
        before.copy_from_gpu(&baseline_cpu);
        let mut best = None::<(f32, f32, f32, f32)>;
        for z in -6..=6 { for x in -6..=6 {
            let (x, z) = (x as f32, z as f32);
            let old = before.sample_at_world_xz(x, z);
            let new = floor.sample_at_world_xz(x, z);
            if best.is_none_or(|(_, _, a, b)| (new - old).abs() > (b - a).abs()) {
                best = Some((x, z, old, new));
            }
        }}
        let (x, z, old, new) = best.context("terrain collision sampling grid empty")?;
        anyhow::ensure!((new - old).abs() > 0.02, "edited terrain did not change collision sampling in stroke area");
        let collision = world.resource::<country_core::systems::collision::world::RaycastWorld>();
        let origin = glam::Vec3::new(x, 30.0, z);
        let max_toi = 30.0 - (old + new) * 0.5;
        let old_hit = collision.intersects_any_w_terrain(&before, origin, -glam::Vec3::Y, max_toi, 0x20);
        let new_hit = collision.intersects_any_w_terrain(&floor, origin, -glam::Vec3::Y, max_toi, 0x20);
        anyhow::ensure!(old_hit != new_hit, "terrain-aware collision did not observe changed floor height");
    }
    // Capture only after the same-session GPU results reached both real CPU
    // owners. This also gives the wall publisher another production frame.
    production_passes.push(run_m2_production_frame(&mut driver, &context, &mut bridge,
        Some(("m2-edited", captured.clone())))?);
    wall_republished |= wall_events.read(driver.app().world().resource::<bevy_ecs::event::Events<country_core::resources::walls::public_walls::OnWallChanged>>()).any(|event| event.0 == wall);
    let public = driver.app().world().resource::<country_core::resources::walls::public_walls::PublicWalls>();
    let updated_wall = public.get(wall).context("terrain-edited wall was not republished")?;
    anyhow::ensure!(wall_republished && updated_wall.height_outline() != baseline_wall.as_slice(),
        "terrain change did not republish the affected wall and its height outline");
    let updated_entity = driver.app().world().resource::<country_core::resources::walls::public_walls::WallEntities>()
        .wall_entity(wall).context("terrain-edited wall lost its real render entity")?;
    let updated_collider = driver.app().world().resource::<country_core::systems::collision::world::RaycastWorld>()
        .wall_collider_instance(updated_entity).context("terrain-edited wall lost its real collider")?.handle();
    let collider_diff = collider_changes.take().context("terrain-edited wall did not publish collider changes")?;
    anyhow::ensure!(collider_diff.added.contains(&updated_collider) || collider_diff.removed.contains(&baseline_collider),
        "collider changes did not include the affected wall");
    bridge.wait_submits_done(120_000_000_000)?;
    let edited_bricks = m1_wall_gpu_bricks(driver.app().world(), &bridge, &device, wall)?;
    anyhow::ensure!(edited_bricks != baseline_bricks, "terrain edit did not update target wall GPU brick records");
    let wall_terrain = &driver.app().world().resource::<country_core::systems::wall::wall_state::publish_public_wall_state::WallPublicationTerrain>().0;
    anyhow::ensure!(wall_terrain.encoded.as_ref() == edited_height.as_slice(),
        "wall publication terrain view did not synchronize the completed floor readback");
    let required_passes = [
        "Rasterize Terrain Stroke",
        "Displace Rocky Terrain",
        "Visual terrain height",
        "Read terrain height",
        "Calculate Terrain Lighting Texture",
    ];
    anyhow::ensure!(
        production_passes
            .iter()
            .any(|passes| contains_ordered_passes(passes, &required_passes)),
        "no production frame contained the ordered terrain chain {required_passes:?}; observed={production_passes:?}"
    );
    eprintln!(
        "M02 W5 frames {baseline_frame}->{edited_frame}, changed floor={height_changes}, visual={visual_changes}"
    );

    {
        let world = driver.app_mut().world_mut();
        for _ in 0..8 {
            tiny_glade::screenshot::shared_texture_capture::finish(world)?;
        }
        tiny_glade::screenshot::shared_texture_capture::shutdown(world)?;
    }
    bridge.wait_submits_done(10_000_000_000)?;
    let images = captured.lock();
    let lighting = |label: &str| images.iter().find(|(name, _, _, _)| name == label)
        .with_context(|| format!("terrain lighting capture {label} did not arrive"));
    let initial_light = lighting("m2-baseline-terrain-lighting")?;
    let stable_light = lighting("m2-stable-terrain-lighting")?;
    let edited_light = lighting("m2-edited-terrain-lighting")?;
    anyhow::ensure!((initial_light.1, initial_light.2, initial_light.3.len()) ==
        (stable_light.1, stable_light.2, stable_light.3.len()) &&
        (stable_light.1, stable_light.2, stable_light.3.len()) ==
        (edited_light.1, edited_light.2, edited_light.3.len()), "lighting capture extent changed");
    let delta = |a: &[u8], b: &[u8]| -> u64 {
        a.iter().zip(b).map(|(&a, &b)| u64::from(a.abs_diff(b))).sum()
    };
    let idle_delta = delta(&initial_light.3, &stable_light.3);
    let edit_delta = delta(&stable_light.3, &edited_light.3);
    anyhow::ensure!(edit_delta > idle_delta.saturating_mul(2),
        "terrain lighting output did not change beyond unedited-frame variation: idle={idle_delta}, edited={edit_delta}");
    let baseline_image = images
        .iter()
        .find(|(label, _, _, _)| label == "m2-baseline")
        .context("M2 baseline production image did not arrive")?;
    let edited_image = images
        .iter()
        .find(|(label, _, _, _)| label == "m2-edited")
        .context("M2 edited production image did not arrive")?;
    anyhow::ensure!(
        (baseline_image.1, baseline_image.2) == (edited_image.1, edited_image.2)
            && baseline_image.3.len() == edited_image.3.len()
    );
    let changed_pixels = baseline_image
        .3
        .chunks_exact(4)
        .zip(edited_image.3.chunks_exact(4))
        .filter(|(before, after)| before != after)
        .count();
    anyhow::ensure!(
        changed_pixels > 0,
        "terrain edit reached W5 but the visible production image did not change"
    );
    let mean = edited_image
        .3
        .chunks_exact(4)
        .map(|px| (u32::from(px[0]) + u32::from(px[1]) + u32::from(px[2])) as f64 / 3.0)
        .sum::<f64>()
        / f64::from(edited_image.1 * edited_image.2);
    let distinct = edited_image
        .3
        .chunks_exact(4)
        .map(|px| u32::from_le_bytes([px[0], px[1], px[2], 0]))
        .collect::<std::collections::HashSet<_>>()
        .len();
    anyhow::ensure!(mean > 40.0 && distinct > 2000);
    eprintln!(
        "M02 actual image {}x{} mean_luma={mean:.1} distinct={distinct} changed_pixels={changed_pixels}",
        edited_image.1, edited_image.2
    );
    Ok(())
}

// GameClock is not persisted. Give both processes the same deterministic time
// input and let the real time_update system advance it to the observation
// instant. Zero-duration ticks align counters first; both renderers then start
// at the same absolute frame and receive identical positive time steps. In
// particular, shadow-mask-mean uses delta time to converge and cannot settle
// while the observation input is zero.
/// Simulation-only waits still drive the real host upload admission boundary.
/// No render counter, feedback payload or readiness state is manufactured.
fn m2_tick_production_app(
    driver: &mut tiny_glade::game_loop::GameFrameDriver,
    context: &egui::Context,
    bridge: &CoreRenderBridge,
) -> Result<()> {
    context.begin_frame(egui::RawInput::default());
    driver.app_mut().update();
    let _ = context.end_frame();
    bridge.prepare_test_execution()?;
    Ok(())
}

fn m2_wait_replay_completion(
    driver: &mut tiny_glade::game_loop::GameFrameDriver,
    context: &egui::Context,
    bridge: &CoreRenderBridge,
    expected_cursor: i32,
    stage: &str,
) -> Result<()> {
    use country_core::{resources::history::{History, ReplayBuffer},
        systems::{history::PendingHistoryCommands, mode_manager::AppMode}};
    let complete = |world: &bevy_ecs::world::World| {
        world.resource::<History>().ledger().cursor == expected_cursor
            && world.resource::<ReplayBuffer>().is_empty()
            && world.resource::<PendingHistoryCommands>().0.is_empty()
            && world.resource::<AppMode>().native_tag() != 13
    };
    let mut reached = false;
    for _ in 0..600 {
        m2_tick_production_app(driver, context, bridge)?;
        if complete(driver.app().world()) { reached = true; break; }
    }
    anyhow::ensure!(reached, "{stage}历史游标/队列/回放模式未完成");
    for _ in 0..2 {
        m2_tick_production_app(driver, context, bridge)?;
        anyhow::ensure!(complete(driver.app().world()), "{stage}完成后回放再次启动");
    }
    Ok(())
}

fn m2_settle_observation_clock(
    driver: &mut tiny_glade::game_loop::GameFrameDriver,
    context: &egui::Context,
    bridge: &mut CoreRenderBridge,
) -> Result<u64> {
    use country_core::systems::mode_manager::{AppMode, EditSubject, RequestAppModeChange};
    // Tool selection is unsaved runtime UI state. Use the canonical toolbar
    // mode request and its real consumer to select ordinary editing in both
    // processes. Action 26 is world clearing, so it is not an exit-tool input.
    driver.app_mut().world_mut().send_event(RequestAppModeChange(AppMode::Editing(EditSubject::None)));
    run_m2_production_frame(driver, context, bridge, None)?;
    run_m2_production_frame(driver, context, bridge, None)?;
    anyhow::ensure!(*driver.app().world().resource::<AppMode>() == AppMode::Editing(EditSubject::None),
        "M2 toolbar mode request did not finish the runtime editing mode");
    const TARGET_SECONDS: f64 = 20.0;
    for _ in 0..3600 {
        let total = driver.app().world().resource::<country_core::resources::game_clock::GameClock>().total_secs;
        if total >= TARGET_SECONDS {
            driver.app_mut().insert_resource(bevy_time::TimeUpdateStrategy::ManualDuration(std::time::Duration::ZERO));
            // Some real producers also use GameClock.frame_count as an RNG
            // input. Zero-duration simulation ticks align its offset from the
            // render counter, without assigning either counter or save field.
            for _ in 0..4096 {
                let world = driver.app().world();
                let clock = world.resource::<country_core::resources::game_clock::GameClock>();
                let rendered = world.resource::<country_core::render::frame_constants::FrameConstants>().frame_counter;
                let offset = clock.frame_count.wrapping_sub(rendered);
                anyhow::ensure!(offset <= 2048, "M2 observation frame offset already exceeds target: {offset}");
                if offset == 2048 {
                    let observation_clock = clock.total_secs.to_bits();
                    // Align the absolute renderer counter through real frames,
                    // so subsequent positive-time rendering advances both
                    // clocks and all temporal GPU producers identically.
                    for _ in 0..=128 {
                        let rendered = driver.app().world()
                            .resource::<country_core::render::frame_constants::FrameConstants>().frame_counter;
                        anyhow::ensure!(rendered <= 128,
                            "M2 pre-observation renderer already exceeds common frame: {rendered}");
                        if rendered == 128 {
                            driver.app_mut().insert_resource(bevy_time::TimeUpdateStrategy::ManualDuration(
                                std::time::Duration::from_secs_f64(1.0 / 60.0)));
                            return Ok(observation_clock);
                        }
                        let capture = (rendered == 127 && std::env::var_os("TG_M2_RAW_INPUTS").is_some())
                            .then(|| {
                                let label = if std::env::var_os("TG_M2_SAVE_CHILD").is_some() {
                                    "m2-child-canonical-inputs"
                                } else { "m2-parent-canonical-inputs" };
                                (label, std::sync::Arc::new(parking_lot::Mutex::new(Vec::new())))
                            });
                        run_m2_production_frame(driver, context, bridge, capture)?;
                    }
                    anyhow::bail!("M2 common renderer observation frame was not reached");
                }
                m2_tick_production_app(driver, context, bridge)?;
            }
            anyhow::bail!("M2 observation frame offset did not converge");
        }
        // The normal host admits uploads between simulation ticks. Advancing
        // only the App starves the bounded queue and is not a valid host loop.
        m2_tick_production_app(driver, context, bridge)?;
    }
    anyhow::bail!("M2 observation clock did not advance to {TARGET_SECONDS}s")
}

#[test]
fn m02_actual_terrain_history_save_new_process_reload_and_image() -> Result<()> {
    if std::env::var_os("TG_M2_SAVE_CHILD").is_some() {
        return m02_actual_terrain_save_child();
    }

    use bevy_ecs::schedule::IntoScheduleConfigs;
    use country_core::{
        input::action_set::DigitalActionPhase as Phase,
        resources::{
            history::{History, MaybePendingSnapshotId, ReplayBuffer},
            terrain_editing::TerrainEditingData,
        },
        systems::{
            history::PendingHistoryCommands,
            mode_manager,
        },
    };

    let BootedSession {
        mut driver,
        context,
        mut bridge,
        shaders: _shaders,
        device: _device,
        _boot,
    } = boot_shipped_session_with_nani_readback(std::env::var_os("TG_M2_RAW_INPUTS").is_some())?;
    // Playback progress and frame counter are runtime state, not save fields.
    // Rewind via the real configuration before comparing rendered frames.
    driver.app_mut().world_mut().resource_mut::<country_core::game_config::GameConfig>().dev.day_night_cycle = false;
    let world = driver.app_mut().world_mut();
    anyhow::ensure!(
        tiny_glade::screenshot::shared_texture_capture::installed(world),
        "M2 persistence production frame has no screenshot owner"
    );
    tiny_glade::production_renderer::warm_up_gameplay(bridge.warmup_owners()?, world)
        .context("M2 persistence warmup")?;
    bridge.accept_gameplay_warmup(
        world.resource::<tiny_glade::render_bridge::CompletedGameplayWarmup>(),
    )?;
    world.insert_resource(M2TerrainSyntheticInput::default());
    world.insert_resource(M2HistorySyntheticInput::default());
    world
        .resource_mut::<country_core::systems::terrain::ui_draw_terrain::TerrainDrawState>()
        .state = 0;
    driver.app_mut().add_systems(
        bevy_app::Update,
        m2_inject_terrain_input
            .before(
                country_core::systems::history::recording::history_start_recording::<
                    MaybePendingSnapshotId,
                >,
            )
            .before(country_core::systems::terrain::ui_draw_terrain::ui_draw_terrain),
    );
    driver.app_mut().add_systems(
        bevy_app::Update,
        m2_inject_history_input
            .after(country_core::systems::backend::input::calculate_input_delta::calculate_input_delta)
            .before(mode_manager::mode_manager::<
                country_core::resources::history::EditType,
                MaybePendingSnapshotId,
            >),
    );

    let _ = run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    for (phase, position) in [
        (3, glam::Vec3::new(-3.0, 0.0, -3.0)),
        (3, glam::Vec3::new(0.0, 0.0, 0.0)),
        (3, glam::Vec3::new(3.0, 0.0, 3.0)),
        (1, glam::Vec3::new(4.0, 0.0, 4.0)),
    ] {
        *driver
            .app_mut()
            .world_mut()
            .resource_mut::<M2TerrainSyntheticInput>() =
            M2TerrainSyntheticInput { active: true, phase, position };
        let _ = run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    }
    *driver
        .app_mut()
        .world_mut()
        .resource_mut::<M2TerrainSyntheticInput>() = M2TerrainSyntheticInput::default();

    let (stroke_id, stroke_before_undo) = {
        let world = driver.app().world();
        let editing = world.resource::<TerrainEditingData>();
        let id = editing.last_stroke_id();
        let stroke = editing
            .strokes
            .get(&id)
            .context("M2 persistence stroke did not commit")?
            .clone();
        (id, stroke)
    };

    let mut edited_heights = None;
    for _ in 0..24 {
        let _ = run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
        if let Some((floor, visual, frame, epoch)) =
            m2_latest_heights(driver.app().world())
            && epoch.session
                == Some(driver.app().world().resource::<ActiveSession>().id)
        {
            edited_heights = Some((floor, visual, frame, epoch));
            break;
        }
    }
    let (edited_floor, edited_visual, edited_frame, edited_epoch) =
        edited_heights.context("M2 persistence edited W5 heights did not arrive")?;
    let committed_cursor = {
        use country_core::resources::history::EditType;
        let history = driver.app().world().resource::<History>();
        anyhow::ensure!(
            !history.is_recording()
                && history.ledger().entries.last().is_some_and(|entry| {
                    entry.edits.iter().any(|edit| {
                        matches!(
                            edit,
                            EditType::Terrain(edit)
                                if matches!(
                                    edit.as_ref(),
                                    country_core::systems::terrain::deferred_terrain_history::TerrainEdit::NewTerrainStroke {
                                        index,
                                        ..
                                    } if *index == stroke_id
                                )
                        )
                    })
                }),
            "M2 terrain edit did not finish as the latest canonical history entry: recording={}, cursor={}, entries={}",
            history.is_recording(),
            history.ledger().cursor,
            history.ledger().entries.len(),
        );
        history.ledger().cursor
    };

    *driver
        .app_mut()
        .world_mut()
        .resource_mut::<M2HistorySyntheticInput>() =
        M2HistorySyntheticInput { undo: Phase::JustPressed, redo: Phase::Idle };
    let _ = run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    driver
        .app_mut()
        .world_mut()
        .resource_mut::<M2HistorySyntheticInput>()
        .undo = Phase::Idle;
    m2_wait_replay_completion(&mut driver, &context, &bridge, committed_cursor - 1, "地形撤销")?;
    anyhow::ensure!(
        driver
            .app()
            .world()
            .resource::<TerrainEditingData>()
            .strokes
            .get(&stroke_id)
            .is_some_and(|stroke| stroke.height_ws.to_bits() == 0),
        "地形 undo 未按原版保留 stroke 并把 height_ws 归零"
    );

    *driver
        .app_mut()
        .world_mut()
        .resource_mut::<M2HistorySyntheticInput>() =
        M2HistorySyntheticInput { undo: Phase::Idle, redo: Phase::JustPressed };
    let _ = run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    driver
        .app_mut()
        .world_mut()
        .resource_mut::<M2HistorySyntheticInput>()
        .redo = Phase::Idle;
    m2_wait_replay_completion(&mut driver, &context, &bridge, committed_cursor, "地形重做")?;
    anyhow::ensure!(
        driver
            .app()
            .world()
            .resource::<TerrainEditingData>()
            .strokes
            .get(&stroke_id)
            == Some(&stroke_before_undo),
        "地形 redo 未逐字段恢复原 stroke"
    );
    anyhow::ensure!(
        driver
            .app()
            .world()
            .resource::<History>()
            .ledger()
            .cursor
            == committed_cursor
    );

    for _ in 0..600 {
        if driver.app().world().resource::<country_core::resources::time_of_day::TimeOfDay>().progress == 0.0 { break; }
        m2_tick_production_app(&mut driver, &context, &bridge)?;
    }
    anyhow::ensure!(driver.app().world().resource::<country_core::resources::time_of_day::TimeOfDay>().progress == 0.0,
        "M2 observation: time-of-day playback did not settle");
    let observation_clock = m2_settle_observation_clock(&mut driver, &context, &mut bridge)?;
    let captured = std::sync::Arc::new(parking_lot::Mutex::new(Vec::<
        (String, u32, u32, Vec<u8>),
    >::new()));
    let mut final_heights = None;
    let mut saved_phase = None;
    use country_core::systems::save::save_game::{SaveGameCmd, SavingGameComplete};
    let session = driver.app().world().resource::<ActiveSession>().id;
    let folder = session.save_folder_path();
    let mut save_completions = bevy_ecs::event::EventCursor::<SavingGameComplete>::default();
    save_completions.read(driver.app().world().resource::<bevy_ecs::event::Events<SavingGameComplete>>())
        .for_each(drop);
    let mut saved_with_capture = false;
    for frame_index in 0..384 {
        let previous_counter = driver.app().world().resource::<country_core::render::frame_constants::FrameConstants>().frame_counter;
        let request = frame_index >= 128 && final_heights.is_some()
            && previous_counter.wrapping_add(1) % 128 == 0;
        if request {
            // The real save system synchronously captures and flushes set43
            // before this same App frame is rendered. Saving in a later tick
            // changes the persisted sheep positions and movement timers.
            driver.app_mut().world_mut().send_event(SaveGameCmd);
        }
        let capture = request.then_some(("m2-saved", captured.clone()));
        let _ = run_m2_production_frame(&mut driver, &context, &mut bridge, capture)?;
        if request {
            saved_with_capture = save_completions.read(driver.app().world()
                .resource::<bevy_ecs::event::Events<SavingGameComplete>>())
                .any(|event| event.path == folder);
            anyhow::ensure!(saved_with_capture, "M2 production save did not complete in the captured frame");
            let counter = driver.app().world().resource::<country_core::render::frame_constants::FrameConstants>().frame_counter;
            anyhow::ensure!(counter % 128 == 0, "M2 parent capture missed requested render phase: {counter}");
            saved_phase = Some(counter % 128);
            m2_dump_frame_state(driver.app().world(), "m2-saved")?;
        }
        if let Some((floor, visual, frame, epoch)) =
            m2_latest_heights(driver.app().world())
            && frame > edited_frame
            && epoch == edited_epoch
        {
            final_heights = Some((floor, visual, frame, epoch));
        }
        if saved_phase.is_some() && final_heights.is_some() { break; }
    }
    let saved_phase = saved_phase.context("M2 parent did not reach matching observation phase")?;
    let (saved_floor, saved_visual, _saved_frame, _saved_epoch) =
        final_heights.context("M2 redo 后 W5 heights 未发布")?;
    anyhow::ensure!(
        saved_floor == edited_floor && saved_visual == edited_visual,
        "M2 undo/redo 后 GPU 高度与编辑结果未逐 word 恢复"
    );

    let recovery = folder.join("m2-save-recovery");
    std::fs::create_dir_all(&recovery)?;
    let captured_sheep = {
        use country_core::{resources::critters::{sheep::Sheep, CritterGoal},
            utils::components::transform::Transform};
        let world = driver.app_mut().world_mut();
        let mut rows = world.query::<(&Transform, &Sheep, &CritterGoal)>();
        country_core::resources::critters::sheep::control::capture_sheep_snapshot(rows.iter(world))
    };
    std::fs::write(recovery.join("sheep.json"), serde_json::to_vec(&captured_sheep)?)?;
    {
        let world = driver.app().world();
        let editing = world.resource::<TerrainEditingData>();
        let terrain = serde_json::json!({
            "next_index": editing.next_index,
            "strokes": editing.strokes,
        });
        std::fs::write(
            recovery.join("terrain.json"),
            serde_json::to_vec_pretty(&terrain)?,
        )?;
        let tod = world.resource::<country_core::resources::time_of_day::TimeOfDay>();
        std::fs::write(recovery.join("tod.json"), serde_json::to_vec(tod)?)?;

        let history = world.resource::<History>();
        let entries = history
            .ledger()
            .entries
            .iter()
            .cloned()
            .map(country_core::resources::history::serialized::SavedMultiEdit)
            .collect::<Vec<_>>();
        let history_json = serde_json::json!({
            "cursor": history.ledger().cursor,
            "undo_anchor": history.ledger().undo_anchor,
            "observed_count_58": history.ledger().observed_count_58,
            "entries": entries,
        });
        std::fs::write(
            recovery.join("history.json"),
            serde_json::to_vec_pretty(&history_json)?,
        )?;
    }
    std::fs::write(
        recovery.join("floor.u16"),
        bytemuck::cast_slice::<u16, u8>(&saved_floor),
    )?;
    std::fs::write(
        recovery.join("visual.u16"),
        bytemuck::cast_slice::<u16, u8>(&saved_visual),
    )?;

    {
        let world = driver.app_mut().world_mut();
        for _ in 0..8 {
            tiny_glade::screenshot::shared_texture_capture::finish(world)?;
        }
        tiny_glade::screenshot::shared_texture_capture::shutdown(world)?;
    }
    bridge.wait_submits_done(10_000_000_000)?;
    let saved_image = {
        let images = captured.lock();
        images
            .iter()
            .find(|(label, _, _, _)| label == "m2-saved")
            .context("M2 persistence image did not arrive")?
            .clone()
    };
    m2_dump_image("m2-saved", saved_image.1, saved_image.2, &saved_image.3)?;
    if let Some((label, width, height, rgba)) = captured.lock().iter()
        .find(|(label, _, _, _)| label == "m2-saved-terrain-lighting")
    {
        m2_dump_image(label, *width, *height, rgba)?;
    }
    std::fs::write(recovery.join("image.rgba"), &saved_image.3)?;
    std::fs::write(
        recovery.join("authority.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "session": session,
            "width": saved_image.1,
            "height": saved_image.2,
            "stroke_id": stroke_id,
            "render_phase_mod_128": saved_phase,
            "observation_clock_bits": observation_clock,
        }))?,
    )?;

    anyhow::ensure!(
        saved_with_capture
            && folder.join("history.json").is_file()
            && folder.join("meta.json").is_file(),
        "M2 terrain production save worker did not finish"
    );
    anyhow::ensure!(
        driver
            .app()
            .world()
            .resource::<ReplayBuffer>()
            .is_empty()
            && driver
                .app()
                .world()
                .resource::<PendingHistoryCommands>()
                .0
                .is_empty(),
        "M2 save completed while history replay was still active"
    );
    {
        let saved: serde_json::Value =
            serde_json::from_slice(&std::fs::read(folder.join("history.json"))?)?;
        let disk_sheep: Option<country_core::resources::critters::sheep::control::SheepControlSnapshot> =
            serde_json::from_value(saved["Sheep"].clone())?;
        m2_assert_sheep_snapshot_bits(&captured_sheep,
            disk_sheep.as_ref().context("M2 saved sheep snapshot is absent")?)?;
        let expected_tod: country_core::resources::time_of_day::TimeOfDay = serde_json::from_slice(&std::fs::read(recovery.join("tod.json"))?)?;
        let disk_tod: country_core::resources::time_of_day::TimeOfDay = serde_json::from_value(saved["Time of day"].clone())?;
        anyhow::ensure!([disk_tod.time_of_day.to_bits(), disk_tod.sun_xz_rotation.to_bits(), disk_tod.latitude_degrees.to_bits()]
            == [expected_tod.time_of_day.to_bits(), expected_tod.sun_xz_rotation.to_bits(), expected_tod.latitude_degrees.to_bits()],
            "M2 saved TimeOfDay three-f32 contract differs from observed state");
        eprintln!(
            "M2_SAVE_DISK version={:?} history.chain_index={:?} history.history_start={:?} history.snapshots={:?} expected_cursor={committed_cursor}",
            saved.get("Version"),
            saved.pointer("/History/chain_index"),
            saved.pointer("/History/history_start"),
            saved.pointer("/History/snapshots"),
        );
    }

    if let Some(dir) = std::env::var_os("TG_M2_DUMP_DIR") {
        std::fs::write(PathBuf::from(dir).join("m2-recovery.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "session":session,"save_folder":folder,"recovery_folder":recovery,
                "save_and_capture_same_app_frame":true,
            }))?)?;
    }

    drop(driver);
    drop(bridge);
    drop(_shaders);
    drop(_device);
    let status = std::process::Command::new(std::env::current_exe()?)
        .args([
            "--exact",
            "m02_actual_terrain_history_save_new_process_reload_and_image",
            "--nocapture",
            "--test-threads=1",
        ])
        .env("TG_M2_SAVE_CHILD", "1")
        .env("TG_M2_SAVE_FOLDER", &folder)
        .env("TG_M2_SAVE_RECOVERY", &recovery)
        .env("TG_M2_SAVE_SESSION", serde_json::to_string(&session)?)
        .env("TG_M2_TRACE_HISTORY", "1")
        .status()?;
    anyhow::ensure!(status.success(), "M2 terrain new-process reload failed: {status}");
    Ok(())
}

fn m02_actual_terrain_save_child() -> Result<()> {
    use country_core::{
        app_loading_state::AppLoadingState,
        meta_commander::{MetaCommand, MetaCommander},
        render::gpu_readback::TerrainCopyEpoch,
        resources::{
            history::History,
            session::SessionId,
            terrain_editing::TerrainEditingData,
        },
        systems::history::PendingHistoryCommands,
    };
    let session: SessionId =
        serde_json::from_str(&std::env::var("TG_M2_SAVE_SESSION")?)?;
    let source =
        PathBuf::from(std::env::var_os("TG_M2_SAVE_FOLDER").context("缺少 M2 存档目录")?);
    let recovery =
        PathBuf::from(std::env::var_os("TG_M2_SAVE_RECOVERY").context("缺少 M2 恢复基准")?);
    let mut expected_terrain: serde_json::Value =
        serde_json::from_slice(&std::fs::read(recovery.join("terrain.json"))?)?;
    // Parse through the actual stroke schema before comparing the child. A
    // JSON Value uses f64 for decimals, but every stroke coordinate and
    // parameter is f32; this preserves all fields without decimal-rounding
    // differences introduced by the fixture's intermediate JSON parse.
    let strokes: std::collections::HashMap<tg_terrain::schema::StrokeId, tg_terrain::stroke::TerrainStroke> =
        serde_json::from_value(expected_terrain["strokes"].clone())?;
    expected_terrain["strokes"] = serde_json::to_value(strokes)?;
    let expected_runtime_history: serde_json::Value =
        serde_json::from_slice(&std::fs::read(recovery.join("history.json"))?)?;
    let expected_authority: serde_json::Value =
        serde_json::from_slice(&std::fs::read(recovery.join("authority.json"))?)?;
    let expected_floor = std::fs::read(recovery.join("floor.u16"))?;
    let expected_visual = std::fs::read(recovery.join("visual.u16"))?;
    let expected_image = std::fs::read(recovery.join("image.rgba"))?;
    let saved_disk_history: country_core::systems::save::load_game::HistorySectionJson = {
        let saved: serde_json::Value =
            serde_json::from_slice(&std::fs::read(source.join("history.json"))?)?;
        eprintln!(
            "M2_CHILD_SOURCE version={:?} history.chain_index={:?} history.history_start={:?} history.snapshots={:?}",
            saved.get("Version"),
            saved.pointer("/History/chain_index"),
            saved.pointer("/History/history_start"),
            saved.pointer("/History/snapshots"),
        );
        serde_json::from_value(
            saved
                .get("History")
                .cloned()
                .context("M2 source save has no History section")?,
        )
        .context("M2 source History must decode through the production save schema")?
    };
    let expected_saved_entries = m2_saved_history_values(&saved_disk_history.edit_chain)?;
    let expected_saved_snapshots = saved_disk_history
        .snapshots
        .iter()
        .cloned()
        .map(serde_json::from_value::<
            country_core::resources::history::ResolvedHistoryChainSnapshot,
        >)
        .collect::<Result<Vec<_>, _>>()
        .context("M2 source snapshot references")?;

    let BootedSession {
        mut driver,
        context,
        mut bridge,
        shaders: _shaders,
        device: _device,
        _boot,
    } = boot_shipped_session_with_nani_readback(std::env::var_os("TG_M2_RAW_INPUTS").is_some())?;
    driver.app_mut().world_mut().resource_mut::<country_core::game_config::GameConfig>().dev.day_night_cycle = false;
    {
        let world = driver.app_mut().world_mut();
        anyhow::ensure!(
            tiny_glade::screenshot::shared_texture_capture::installed(world),
            "M2 child production frame has no screenshot owner"
        );
        tiny_glade::production_renderer::warm_up_gameplay(bridge.warmup_owners()?, world)
            .context("M2 child production warmup")?;
        bridge.accept_gameplay_warmup(
            world.resource::<tiny_glade::render_bridge::CompletedGameplayWarmup>(),
        )?;
    }
    let mut previous_boot_history = None;
    let mut stable_boot_frames = 0;
    for _ in 0..64 {
        let _ = run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
        let history = driver.app().world().resource::<History>();
        let trace = (
            history.ledger().cursor,
            history.ledger().entries.len(),
            history.is_recording(),
        );
        if previous_boot_history == Some(trace) {
            stable_boot_frames += 1;
        } else {
            previous_boot_history = Some(trace);
            stable_boot_frames = 0;
        }
        if stable_boot_frames >= 8 {
            break;
        }
    }
    anyhow::ensure!(
        stable_boot_frames >= 8,
        "M2 child boot session did not settle before Reload"
    );
    // The production loader preserves the old runtime playback progress while
    // replacing only the three saved fields. Let that progress reach zero
    // before loading, so the newly loaded hour is not immediately advanced.
    for _ in 0..600 {
        if driver.app().world().resource::<country_core::resources::time_of_day::TimeOfDay>().progress == 0.0 { break; }
        m2_tick_production_app(&mut driver, &context, &bridge)?;
    }
    let boot_tod = driver.app().world().resource::<country_core::resources::time_of_day::TimeOfDay>();
    anyhow::ensure!(boot_tod.progress == 0.0, "M2 child old-session TOD playback did not settle: {boot_tod:?}");
    eprintln!("M2_CHILD_PRELOAD_TOD {boot_tod:?}");
    copy_test_tree(&source, &session.save_folder_path())?;
    driver
        .app_mut()
        .add_systems(bevy_app::Update, m2_trace_history_commands);
    driver
        .app_mut()
        .world_mut()
        .resource_mut::<MetaCommander>()
        .extend_script([MetaCommand::load_session(session, false)]);
    let mut saw_reload = false;
    let mut loaded = false;
    let mut previous_load_trace = None;
    for _ in 0..1200 {
        m2_tick_production_app(&mut driver, &context, &bridge)?;
        let world = driver.app().world();
        let loading = world
            .resource::<bevy_state::prelude::State<AppLoadingState>>()
            .get();
        saw_reload |= *loading == AppLoadingState::Reload;
        let manager = world.resource::<LoadGameManager>();
        let cursor = world.resource::<History>().ledger().cursor;
        let entries = world.resource::<History>().ledger().entries.len();
        let trace = (
            format!("{loading:?}"),
            format!("{:?}", manager.state),
            manager.progress_counter,
            manager.options.is_some(),
            cursor,
            entries,
            world
                .resource::<country_core::resources::history::ReplayBuffer>()
                .is_empty(),
            world
                .resource::<PendingHistoryCommands>()
                .0
                .len(),
        );
        if previous_load_trace.as_ref() != Some(&trace) {
            eprintln!("M2_CHILD_LOAD_TRACE {trace:?}");
            let tod = world.resource::<country_core::resources::time_of_day::TimeOfDay>();
            eprintln!("M2_CHILD_LOAD_TOD bits=[{:08x},{:08x},{:08x}] state={:?} progress={:?}",
                tod.time_of_day.to_bits(), tod.sun_xz_rotation.to_bits(), tod.latitude_degrees.to_bits(), tod.state, tod.progress);
            previous_load_trace = Some(trace);
        }
        if saw_reload
            && *loading == AppLoadingState::Game
            && world
                .get_resource::<ActiveSession>()
                .is_some_and(|active| active.id == session)
            && world.resource::<LoadGameManager>().options.is_none()
            && world.resource::<country_core::resources::history::ReplayBuffer>().is_empty()
            && world.resource::<PendingHistoryCommands>().0.is_empty()
        {
            loaded = true;
            break;
        }
    }
    anyhow::ensure!(loaded, "M2 child did not complete production Reload");
    let expected_tod: country_core::resources::time_of_day::TimeOfDay = serde_json::from_slice(&std::fs::read(recovery.join("tod.json"))?)?;
    let actual_tod = driver.app().world().resource::<country_core::resources::time_of_day::TimeOfDay>();
    let expected_tod_bits = [expected_tod.time_of_day.to_bits(), expected_tod.sun_xz_rotation.to_bits(), expected_tod.latitude_degrees.to_bits()];
    let actual_tod_bits = [actual_tod.time_of_day.to_bits(), actual_tod.sun_xz_rotation.to_bits(), actual_tod.latitude_degrees.to_bits()];
    anyhow::ensure!(actual_tod_bits == expected_tod_bits,
        "M2 loaded TimeOfDay [hour, rotation, latitude] bits differ: expected={expected_tod_bits:08x?}, actual={actual_tod_bits:08x?}; actual={actual_tod:?}");
    anyhow::ensure!(actual_tod.progress == 0.0, "M2 child playback must be settled before observation");
    let observation_clock = m2_settle_observation_clock(&mut driver, &context, &mut bridge)?;
    anyhow::ensure!(Some(observation_clock) == expected_authority["observation_clock_bits"].as_u64(),
        "M2 child observation clock differs from parent");
    let expected_phase = expected_authority["render_phase_mod_128"].as_u64().context("M2 capture phase")? as u32;

    let captured = std::sync::Arc::new(parking_lot::Mutex::new(Vec::<
        (String, u32, u32, Vec<u8>),
    >::new()));
    let mut heights = None;
    let mut previous_history_trace = None;
    let mut matched_capture = false;
    for frame_index in 0..384 {
        let previous_counter = driver.app().world().resource::<country_core::render::frame_constants::FrameConstants>().frame_counter;
        let request = frame_index >= 128 && previous_counter.wrapping_add(1) % 128 == expected_phase;
        let capture_label = if request { Some("m2-reloaded-matched") } else { match frame_index {
            8 => Some("m2-reloaded-8"),
            16 => Some("m2-reloaded-16"),
            32 => Some("m2-reloaded-32"),
            48 => Some("m2-reloaded-48"),
            64 => Some("m2-reloaded-64"),
            96 => Some("m2-reloaded-96"),
            128 => Some("m2-reloaded-128"),
            192 => Some("m2-reloaded-192"),
            _ => None,
        }};
        let capture = capture_label.map(|label| (label, captured.clone()));
        let _ = run_m2_production_frame(&mut driver, &context, &mut bridge, capture)?;
        if let Some(label) = capture_label { m2_dump_frame_state(driver.app().world(), label)?; }
        if request {
            let counter = driver.app().world().resource::<country_core::render::frame_constants::FrameConstants>().frame_counter;
            anyhow::ensure!(counter % 128 == expected_phase, "M2 child capture missed requested render phase: {counter}");
            matched_capture = true;
        }
        let history = driver.app().world().resource::<History>();
        let draw = driver
            .app()
            .world()
            .resource::<country_core::systems::terrain::ui_draw_terrain::TerrainDrawState>();
        let ui = driver
            .app()
            .world()
            .resource::<country_core::systems::ui::resources::ui_state::UiState>();
        let mode = driver
            .app()
            .world()
            .resource::<country_core::systems::mode_manager::AppMode>();
        let cursor = driver
            .app()
            .world()
            .resource::<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>();
        eprintln!(
            "M2_CHILD_INPUT frame={frame_index} mode={} draw=({},{},{:?},{}) ui=({},{},{:?}) cursor=({},{:?})",
            mode.native_tag(),
            draw.state,
            u8::from(draw.active),
            draw.drag_distance,
            draw.records.len(),
            ui.hover_byte(),
            ui.click_byte(),
            ui.active_widget(),
            cursor.tag,
            cursor.pos,
        );
        let trace = (
            history.ledger().cursor,
            history.ledger().entries.len(),
            history.is_recording(),
            driver
                .app()
                .world()
                .resource::<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>()
                .edit
                .is_some(),
        );
        if previous_history_trace != Some(trace) {
            let tail = history
                .ledger()
                .entries
                .last()
                .map(|entry| {
                    serde_json::to_string(
                        &country_core::resources::history::serialized::SavedMultiEdit(
                            entry.clone(),
                        ),
                    )
                })
                .transpose()?;
            eprintln!(
                "M2_CHILD_FRAME_HISTORY frame={frame_index} trace={trace:?} tail={tail:?}"
            );
            previous_history_trace = Some(trace);
        }
        if let Some((floor, visual, frame, epoch)) =
            m2_latest_heights(driver.app().world())
            && epoch.session == Some(session)
            && frame > 0
        {
            heights = Some((floor, visual, frame, epoch));
            if matched_capture {
                break;
            }
        }
    }
    anyhow::ensure!(matched_capture, "M2 child did not reach the saved observation phase");
    let (floor, visual, frame, epoch) =
        heights.context("M2 child W5 heights did not arrive")?;
    anyhow::ensure!(
        bytemuck::cast_slice::<u16, u8>(&floor) == expected_floor
            && bytemuck::cast_slice::<u16, u8>(&visual) == expected_visual,
        "M2 child floor/visual heights differ from saved bytes"
    );
    anyhow::ensure!(epoch.session == Some(session));
    anyhow::ensure!(
        driver
            .app()
            .world()
            .get_resource::<TerrainCopyEpoch>()
            .is_none_or(|copy| copy.0.session == Some(session)),
        "M2 child accepted a terrain copy from a previous session"
    );

    let actual_terrain = {
        let editing = driver.app().world().resource::<TerrainEditingData>();
        serde_json::json!({
            "next_index": editing.next_index,
            "strokes": editing.strokes,
        })
    };
    if let Some((path, expected, actual)) = m6_first_json_difference(&expected_terrain, &actual_terrain) {
        anyhow::bail!("M2 child terrain strokes/counter differ at {path}: expected={expected}, actual={actual}");
    }
    let (actual_cursor, actual_undo_anchor, actual_observed_count, actual_entries, actual_snapshots) = {
        let history = driver.app().world().resource::<History>();
        let ledger = history.ledger();
        (
            ledger.cursor,
            ledger.undo_anchor,
            ledger.observed_count_58,
            ledger
                .entries
                .iter()
                .cloned()
                .map(country_core::resources::history::serialized::SavedMultiEdit)
                .collect::<Vec<_>>(),
            country_core::resources::history::snapshots::resolved_snapshot_references(
                &ledger.resolved_snapshots,
            ),
        )
    };
    anyhow::ensure!(
        actual_cursor == saved_disk_history.chain_index,
        "M2 child history cursor differs from saved disk: expected={}, actual={actual_cursor}",
        saved_disk_history.chain_index,
    );
    anyhow::ensure!(
        actual_undo_anchor == saved_disk_history.history_start,
        "M2 child history start differs from saved disk: expected={}, actual={actual_undo_anchor}",
        saved_disk_history.history_start,
    );
    let expected_observed_count = expected_runtime_history["observed_count_58"]
        .as_u64()
        .context("M2 recovery history observed_count_58")?;
    anyhow::ensure!(
        actual_observed_count == expected_observed_count as usize,
        "M2 child observed history counter differs: expected={expected_observed_count}, actual={actual_observed_count}",
    );
    anyhow::ensure!(
        actual_snapshots == expected_saved_snapshots,
        "M2 child snapshot references differ from saved disk: expected={expected_saved_snapshots:?}, actual={actual_snapshots:?}",
    );
    let actual_saved_entries = m2_saved_history_values(&actual_entries)?;
    if let Some((path, expected, actual)) =
        m2_first_json_difference(&expected_saved_entries, &actual_saved_entries)
    {
        anyhow::bail!(
            "M2 child typed saved-history entries differ from disk at {path}: expected={expected}, actual={actual}"
        );
    }
    anyhow::ensure!(
        expected_authority["session"] == serde_json::to_value(session)?
            && expected_authority["width"].as_u64().is_some()
            && expected_authority["height"].as_u64().is_some(),
        "M2 child recovery authority is incomplete"
    );

    {
        let world = driver.app_mut().world_mut();
        for _ in 0..8 {
            tiny_glade::screenshot::shared_texture_capture::finish(world)?;
        }
        tiny_glade::screenshot::shared_texture_capture::shutdown(world)?;
    }
    bridge.wait_submits_done(10_000_000_000)?;
    let image = {
        let images = captured.lock();
        for (label, width, height, rgba) in images.iter() {
            m2_dump_image(label, *width, *height, rgba)?;
        }
        images
            .iter()
            .rev()
            .find(|(label, _, _, _)| label == "m2-reloaded-matched")
            .cloned()
            .context("M2 child production image did not arrive")?
    };
    let expected_width = expected_authority["width"]
        .as_u64()
        .context("M2 saved width")? as u32;
    let expected_height = expected_authority["height"]
        .as_u64()
        .context("M2 saved height")? as u32;
    anyhow::ensure!(
        (image.1, image.2) == (expected_width, expected_height)
            && image.3.len() == expected_image.len(),
        "M2 child production image dimensions differ from saved frame"
    );
    let changed = image
        .3
        .chunks_exact(4)
        .zip(expected_image.chunks_exact(4))
        .filter(|(actual, expected)| actual != expected)
        .count();
    if changed != 0 {
        let first = image.3.chunks_exact(4).zip(expected_image.chunks_exact(4))
            .enumerate().find(|(_, (actual, expected))| actual != expected).unwrap();
        let pixel = first.0;
        anyhow::bail!("M2 child visible image differs from saved production frame: {changed} pixels; first=({}, {}) expected={:?} actual={:?}",
            pixel % image.1 as usize, pixel / image.1 as usize, first.1.1, first.1.0);
    }
    eprintln!(
        "M02 save/reload exact: frame={frame}, floor={} bytes, visual={} bytes, image={}x{}, changed_pixels=0",
        expected_floor.len(),
        expected_visual.len(),
        image.1,
        image.2,
    );
    Ok(())
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

#[test]
#[ignore = "Daily excluded by user; candidate retained without acceptance"]
fn daily_menu_picker_reloads_real_session_and_applies_original_history() -> Result<()> {
    new_session_menu_chain(true)
}

#[test]
fn ordinary_new_session_picker_save_and_reenter() -> Result<()> {
    new_session_menu_chain(false)
}

/// Synthetic input passes through the real MEI picker and new-session menu.
fn new_session_menu_chain(daily: bool) -> Result<()> {
    use country_core::{
        app_loading_state::AppLoadingState,
        game_profile::GameProfile,
        resources::{daily::{CurrentDaily, DailyThemeInfo, DailyThemeState}, gui::EscMenuState,
            history::{History, ReplayBuffer}, session::{SessionMeta, GladeType}, window::WindowSize},
        systems::{backend::input::{button_input::MouseButtonInput, translate_input_to_cursor_moved::InputCursorMoved},
            history::PendingHistoryCommands, mode_manager::AppMode,
            ui::resources::ui_state::{UiState, WidgetId}},
    };
    use toolbar::{MeiContext, mei::{core::{GlobalId, calculate_hash}, widgets::MenuButtonContent}};
    use bevy_state::state::State;
    let installed = PathBuf::from(std::env::var_os("TG_INSTALL_ROOT").context("TG_INSTALL_ROOT required")?);
    let BootedSession { mut driver, context, bridge: _bridge, shaders: _shaders, device: _device, _boot } = boot_shipped_session()?;
    let old_session = driver.app().world().resource::<ActiveSession>().id;
    let meta = if daily { SessionMeta::default() } else { SessionMeta { glade: GladeType::Summer, ..Default::default() } };
    let info = daily.then(|| DailyThemeInfo { languages: vec!["Injected Daily".into()],
        history_path: installed.join("assets/starting-builds/full/intro/history.json"),
        meta: meta.clone(), credits: None, start_time: std::time::UNIX_EPOCH,
        end_time: std::time::UNIX_EPOCH + std::time::Duration::from_secs(86_400) });
    *driver.app_mut().world_mut().resource_mut::<EscMenuState>() = EscMenuState::NewSession;
    let frame = |driver: &mut tiny_glade::game_loop::GameFrameDriver, held: bool, position: glam::Vec2| {
        let world = driver.app_mut().world_mut();
        if let Some(info) = &info { world.resource_mut::<CurrentDaily>().0 = DailyThemeState::Ready(info.clone()); }
        let mut mouse = world.resource_mut::<MouseButtonInput>();
        mouse.clear();
        if held { mouse.press(0); } else { mouse.release(0); }
        world.send_event(InputCursorMoved::Absolute(position));
        context.begin_frame(egui::RawInput::default());
        driver.app_mut().update();
        let _ = context.end_frame();
    };
    for _ in 0..12 { frame(&mut driver, false, glam::Vec2::ZERO); }
    let key = lang_lib::lookup(if daily { "New.Glade.Daily" } else { "New.Glade.Summer" });
    let (id, position) = {
        let world = driver.app().world();
        let scale = world.resource::<WindowSize>().height as f32 / 1000.;
        // Production layout consumes pending_text during prepare; identify the
        // button by the same parent/name/type hash used by menu_button.
        let mei = world.resource::<MeiContext>();
        let label: &str = key.as_ref();
        let local = calculate_hash(&(calculate_hash(&(label, ())), 0x615bfcfe4597ab8fu64));
        let (id, node) = mei.nodes.keys().find_map(|parent| {
            let id = GlobalId(calculate_hash(&(parent.0, local)));
            mei.nodes.get_key_value(&id).filter(|(_, node)| {
                let content: &dyn std::any::Any = node.content.as_ref();
                content.is::<MenuButtonContent>()
            })
        }).with_context(|| format!("new-session button absent: menu={:?}, current nodes={}, previous nodes={}",
            world.resource::<EscMenuState>(), mei.nodes.len(), mei.previous_nodes.len()))?;
        anyhow::ensure!(node.size.min_element() > 0., "new-session button lacks laid-out hit bounds");
        (*id, (node.pos + node.size * 0.5) * scale)
    };
    for _ in 0..4 { frame(&mut driver, false, position); }
    anyhow::ensure!(driver.app().world().resource::<UiState>().active_widget() == WidgetId::from_name(&id.0.to_string()), "new-session button was not picked");
    let before_count = driver.app().world().resource::<GameProfile>().new_glade_count;
    frame(&mut driver, true, position);
    frame(&mut driver, false, position);
    let mut saw_reload = false;
    let mut loaded = false;
    for _ in 0..1200 {
        frame(&mut driver, false, position);
        let world = driver.app().world();
        let state = *world.resource::<State<AppLoadingState>>().get();
        saw_reload |= state == AppLoadingState::Reload;
        if saw_reload && state == AppLoadingState::Game && world.resource::<ActiveSession>().id != old_session
            && world.resource::<LoadGameManager>().options.is_none()
            && world.resource::<ReplayBuffer>().is_empty()
            && world.resource::<PendingHistoryCommands>().0.is_empty()
            && !matches!(*world.resource::<AppMode>(), AppMode::LoadGame | AppMode::Replay) {
            loaded = true; break;
        }
    }
    anyhow::ensure!(saw_reload && loaded, "new-session menu did not complete Reload and history replay");
    let world = driver.app().world();
    assert_eq!(world.resource::<ActiveSession>().meta, meta);
    assert_eq!(world.resource::<GameProfile>().new_glade_count, before_count + 1);
    if daily {
        let saved: serde_json::Value = serde_json::from_slice(&std::fs::read(&info.as_ref().unwrap().history_path)?)?;
        let expected_entries = saved["History"]["edit_chain"].as_array().context("fixture history chain")?.len();
        let expected_cursor = saved["History"]["chain_index"].as_i64().context("fixture history cursor")?;
        assert_eq!(world.resource::<History>().ledger().entries.len(), expected_entries);
        assert_eq!(i64::from(world.resource::<History>().ledger().cursor), expected_cursor);
    } else {
        anyhow::ensure!(!world.resource::<History>().ledger().entries.is_empty(), "ordinary starting history is empty");
        let session = world.resource::<ActiveSession>().id;
        let entries = world.resource::<History>().ledger().entries.iter().cloned()
            .map(country_core::resources::history::serialized::SavedMultiEdit)
            .map(|entry| serde_json::to_string(&entry)).collect::<std::result::Result<Vec<_>, _>>()?;
        let cursor = world.resource::<History>().ledger().cursor;
        let walls = m6_authority_map(world);
        use country_core::systems::save::save_game::{SaveGameCmd, SavingGameComplete};
        driver.app_mut().world_mut().send_event(SaveGameCmd);
        let mut saved = false;
        for _ in 0..600 {
            frame(&mut driver, false, position);
            if driver.app_mut().world_mut().resource_mut::<bevy_ecs::event::Events<SavingGameComplete>>()
                .drain().any(|event| event.path == session.save_folder_path()) { saved = true; break; }
        }
        anyhow::ensure!(saved && session.save_folder_path().join("history.json").is_file(), "new session save did not complete");
        use country_core::meta_commander::{MetaCommand, MetaCommander};
        driver.app_mut().world_mut().resource_mut::<MetaCommander>().extend_script([MetaCommand::load_session(session, false)]);
        let mut reloading = false;
        let mut reentered = false;
        for _ in 0..1200 {
            frame(&mut driver, false, position);
            let world = driver.app().world();
            let state = *world.resource::<State<AppLoadingState>>().get();
            reloading |= state == AppLoadingState::Reload;
            if reloading && state == AppLoadingState::Game && world.resource::<ActiveSession>().id == session
                && world.resource::<LoadGameManager>().options.is_none()
                && world.resource::<ReplayBuffer>().is_empty()
                && world.resource::<PendingHistoryCommands>().0.is_empty()
                && !matches!(*world.resource::<AppMode>(), AppMode::LoadGame | AppMode::Replay) {
                reentered = true; break;
            }
        }
        anyhow::ensure!(reloading && reentered, "saved session did not complete reentry");
        let world = driver.app().world();
        assert_eq!(world.resource::<ActiveSession>().meta, meta);
        assert_eq!(world.resource::<History>().ledger().cursor, cursor);
        let restored = world.resource::<History>().ledger().entries.iter().cloned()
            .map(country_core::resources::history::serialized::SavedMultiEdit)
            .map(|entry| serde_json::to_string(&entry)).collect::<std::result::Result<Vec<_>, _>>()?;
        assert_eq!(restored, entries, "complete committed history must survive save and reentry");
        if let Some((path, expected, actual)) = m6_first_json_difference(&walls, &m6_authority_map(world)) {
            anyhow::bail!("reentered wall authority differs at {path}: expected={expected} actual={actual}");
        }
    }
    Ok(())
}

/// 仅诊断运行使用：保留实际 BufferHost 的所有能力，并额外允许传输读取。
/// 描述符与原生产运行不同，不能把它当成原生产路径的原样证据。
#[derive(Clone)]
struct M6NaniDiagnosticDevice(
    pub std::sync::Arc<tiny_glade::render_host::BufferHost>,
    /// None preserves the existing Nani diagnostic. M1 reads only this named
    /// SSBO and leaves every other descriptor unchanged.
    pub Option<&'static str>,
);
impl country_core::render::ssbo::rhapsody_boundary::GraphicsDevice for M6NaniDiagnosticDevice {
    fn create_buffer(&self, desc: &country_core::render::ssbo::rhapsody_boundary::BufferDesc, name: &str, data: Option<&[u8]>)
        -> std::result::Result<country_core::render::ssbo::rhapsody_boundary::Buffer, country_core::render::ssbo::rhapsody_boundary::BackendError> {
        use country_core::render::ssbo::rhapsody_boundary::GraphicsDevice;
        let mut desc = *desc;
        if self.1.is_none_or(|target| name == target) {
            desc.usage |= ash::vk::BufferUsageFlags::TRANSFER_SRC.as_raw();
        }
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
    let source = host.managed_buffer(buffer).context("诊断源buffer句柄已失效")?;
    m6_read_diagnostic_native_range(device, &source.shared(), offset, size)
}

fn m6_read_diagnostic_native_range(device: &std::sync::Arc<rhapsody::device::Device>,
    source: &std::sync::Arc<rhapsody::resource::Buffer>, offset: usize, size: usize) -> Result<Vec<u8>> {
    use ash::vk;
    use rhapsody::{command::buffer_access, native::AccessScope,
        resource::{Buffer as NativeBuffer, BufferDesc as NativeBufferDesc}, transfer::with_setup_command};
    anyhow::ensure!(source.usage.contains(vk::BufferUsageFlags::TRANSFER_SRC), "此owner不是可读的诊断描述符");
    anyhow::ensure!(size > 0 && offset.checked_add(size).is_some_and(|end| end as u64 <= source.size), "诊断读取超出真实buffer");
    let output = std::sync::Arc::new(NativeBuffer::new(device.clone(),
        &NativeBufferDesc::new_gpu_to_cpu(size as u64, vk::BufferUsageFlags::TRANSFER_DST))?);
    let queue = device.queue(rhapsody::device::QueueClass::Graphics).context("缺少graphics队列")?;
    with_setup_command(device, &[], |command| unsafe {
        buffer_access(command, source, queue, offset as u64, size as u64,
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
    let entry = &world.resource::<History>().ledger().entries[cursor as usize];
    for (index, edit) in entry.edits.iter().enumerate() {
        if let country_core::resources::history::EditType::Wall(compound) = edit {
            let groups: Vec<Vec<_>> = compound.groups.iter().map(|group| group.iter()
                .map(|operation| (operation.wall_id(), operation.operation_tag())).collect()).collect();
            eprintln!("M6_CLOSED_HISTORY edit={index} groups={groups:?}");
        } else {
            eprintln!("M6_CLOSED_HISTORY edit={index} owner=non-wall");
        }
    }
    let closed = m6_authority_map(world);
    driver.app_mut().world_mut().resource_mut::<M6SyntheticEraserInput>().undo = Phase::JustPressed;
    step(&mut driver);
    driver.app_mut().world_mut().resource_mut::<M6SyntheticEraserInput>().undo = Phase::Idle;
    m6_wait_replay_completion(&mut driver, &context, cursor - 1, "闭合撤销")?;
    for _ in 0..10 { step(&mut driver); }
    let undone = m6_authority_map(driver.app().world());
    if let Some((path, expected, actual)) = m6_first_json_difference(&original, &undone) {
        anyhow::bail!("闭合撤销后权威墙不一致: {path}: expected={expected} actual={actual}");
    }
    driver.app_mut().world_mut().resource_mut::<M6SyntheticEraserInput>().redo = Phase::JustPressed;
    step(&mut driver);
    driver.app_mut().world_mut().resource_mut::<M6SyntheticEraserInput>().redo = Phase::Idle;
    m6_wait_replay_completion(&mut driver, &context, cursor, "闭合重做")?;
    for _ in 0..30 { step(&mut driver); }
    let redone = m6_authority_map(driver.app().world());
    if let Some((path, expected, actual)) = m6_first_json_difference(&closed, &redone) {
        anyhow::bail!("闭合重做后权威墙不一致: {path}: expected={expected} actual={actual}");
    }
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

/// Only pointer position and digital phases are injected; the production UI
/// picker, collider raycast, menu producer, writers and history remain live.
#[derive(bevy_ecs::prelude::Resource)]
struct M6ColorInput {
    cursor: glam::Vec2,
    click: country_core::input::action_set::DigitalActionPhase,
    undo: country_core::input::action_set::DigitalActionPhase,
    redo: country_core::input::action_set::DigitalActionPhase,
}

/// Select tools through the same request consumed by the toolbar mode manager.
/// Directly replacing AppMode would bypass its cancellation/history ownership.
fn m6_request_mode(
    driver: &mut tiny_glade::game_loop::GameFrameDriver,
    context: &egui::Context,
    bridge: &mut CoreRenderBridge,
    mode: country_core::systems::mode_manager::AppMode,
) -> Result<()> {
    use country_core::systems::mode_manager::{AppMode, RequestAppModeChange};
    m6_dismiss_welcome(driver, context, bridge)?;
    driver.app_mut().world_mut().send_event(RequestAppModeChange(mode));
    for _ in 0..3 {
        run_m2_production_frame(driver, context, bridge, None)?;
        if *driver.app().world().resource::<AppMode>() == mode { return Ok(()); }
    }
    anyhow::bail!("production mode request did not select {mode:?}: {:?}",
        driver.app().world().resource::<AppMode>())
}

/// Fresh process profiles show the real Welcome modal over the world. Confirm
/// it through its configured keyboard binding before attempting world clicks.
fn m6_dismiss_welcome(
    driver: &mut tiny_glade::game_loop::GameFrameDriver,
    context: &egui::Context,
    bridge: &mut CoreRenderBridge,
) -> Result<()> {
    use country_core::{game_profile::{GameProfile, desktop_keycode_index},
        systems::backend::input::button_input::KeyButtonInput,
        resources::onboarding::WelcomeScreenState};
    if driver.app().world().resource::<WelcomeScreenState>().is_closed() { return Ok(()); }
    let binding = driver.app().world().resource::<GameProfile>().key_bindings.bindings[29]
        .context("Welcome confirm binding missing")?;
    let code = desktop_keycode_index(binding).context("Welcome confirm has no desktop key")?;
    for _ in 0..3 {
        for _ in 0..24 { run_m2_production_frame(driver, context, bridge, None)?; }
        driver.app_mut().world_mut().resource_mut::<KeyButtonInput>().press(code);
        run_m2_production_frame(driver, context, bridge, None)?;
        {
            let mut keys = driver.app_mut().world_mut().resource_mut::<KeyButtonInput>();
            keys.release_all();
            keys.clear();
        }
        run_m2_production_frame(driver, context, bridge, None)?;
        if driver.app().world().resource::<WelcomeScreenState>().is_closed() {
            anyhow::ensure!(driver.app().world().resource::<GameProfile>().welcome_was_dismissed(),
                "Welcome closed without its canonical profile confirmation");
            for _ in 0..3 { run_m2_production_frame(driver, context, bridge, None)?; }
            return Ok(());
        }
    }
    anyhow::bail!("Welcome modal did not close through its real confirm input")
}

/// Keep the real renderer and bounded upload consumer alive during replay.
/// App-only ticks are useful for CPU fixtures, but cannot establish GPU cleanup.
fn m6_wait_rendered_replay_completion(
    driver: &mut tiny_glade::game_loop::GameFrameDriver,
    context: &egui::Context,
    bridge: &mut CoreRenderBridge,
    expected_cursor: i32,
    stage: &str,
) -> Result<()> {
    use country_core::{resources::history::{History, ReplayBuffer},
        systems::{history::PendingHistoryCommands, mode_manager::AppMode}};
    let complete = |world: &bevy_ecs::world::World| {
        world.resource::<History>().ledger().cursor == expected_cursor
            && world.resource::<ReplayBuffer>().is_empty()
            && world.resource::<PendingHistoryCommands>().0.is_empty()
            && world.resource::<AppMode>().native_tag() != 13
            && m6_history_is_quiet(world)
    };
    for _ in 0..600 {
        run_m2_production_frame(driver, context, bridge, None)?;
        if !complete(driver.app().world()) { continue; }
        for _ in 0..2 {
            run_m2_production_frame(driver, context, bridge, None)?;
            anyhow::ensure!(complete(driver.app().world()), "{stage} replay restarted after completion");
        }
        return Ok(());
    }
    anyhow::bail!("{stage} did not finish through production renderer/replay")
}

fn m6_history_is_quiet(world: &bevy_ecs::world::World) -> bool {
    let history = world.resource::<country_core::resources::history::History>();
    let deferred = world.resource::<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>();
    !history.is_recording() && !deferred.pending && deferred.records.is_empty()
        && deferred.potentially_surviving_affected_decorators.is_empty()
        && world.resource::<country_core::resources::history::ReplayBuffer>().is_empty()
        && world.resource::<country_core::systems::history::PendingHistoryCommands>().0.is_empty()
        && world.resource::<LoadGameManager>().options.is_none()
}

/// SavingGameComplete acknowledges disk flushing even when snapshot capture is
/// deferred. Observe a finished transaction and unchanged full ledger first.
fn m6_settle_history(
    driver: &mut tiny_glade::game_loop::GameFrameDriver,
    context: &egui::Context,
    bridge: &mut CoreRenderBridge,
) -> Result<()> {
    use country_core::resources::history::{History, serialized::SavedMultiEdit};
    let mut previous = None;
    let mut stable = 0;
    for _ in 0..120 {
        run_m2_production_frame(driver, context, bridge, None)?;
        let world = driver.app().world();
        let history = world.resource::<History>();
        let mut entries = serde_json::to_value(history.ledger().entries.iter().cloned()
            .map(SavedMultiEdit).collect::<Vec<_>>())?;
        for entry in entries.as_array_mut().context("ledger is not an array")? {
            canonicalize_decorator_diff_map(entry);
        }
        let current = (history.ledger().cursor, entries);
        if m6_history_is_quiet(world) && previous.as_ref() == Some(&current) { stable += 1; }
        else { stable = 0; }
        previous = Some(current);
        if stable >= 3 { return Ok(()); }
    }
    anyhow::bail!("production transaction/deferred history did not settle before save")
}

/// Optional retained test artifacts; the real session and its save path are untouched.
fn m6_retain_color_diagnostic(scenario: &str, folder: &std::path::Path,
    expected: &std::path::Path, child_env: serde_json::Value) -> Result<()> {
    let Some(root) = std::env::var_os("TG_M6_DUMP_DIR") else { return Ok(()); };
    let root = PathBuf::from(root).join(scenario);
    std::fs::create_dir_all(&root)?;
    let pending = tempfile::Builder::new().prefix("run-").tempdir_in(&root)?;
    copy_test_tree(folder, &pending.path().join("save"))?;
    std::fs::copy(expected, pending.path().join("expected.json"))?;
    std::fs::write(pending.path().join("meta.json"), serde_json::to_vec_pretty(&child_env)?)?;
    let retained = pending.keep();
    eprintln!("M6 diagnostic retained: {}", retained.display());
    Ok(())
}

fn m6_history_wire(entries: &[country_core::resources::history::serialized::SavedMultiEdit]) -> Result<Vec<String>> {
    entries.iter().map(|entry| {
        // Serialize the typed f32 directly: Ryu's finite f32 representation is
        // round-trip exact, including signed zero. Parse/canonicalize only the
        // unordered decorator map, then compare its full canonical wire text.
        let wire = serde_json::to_vec(entry)?;
        // Reject non-round-trippable values (including non-finite required
        // floats serialized as null); do not cast integers or f64 to f32.
        let _: country_core::resources::history::serialized::SavedMultiEdit = serde_json::from_slice(&wire)
            .context("typed history contains a non-round-trippable value")?;
        let mut value: serde_json::Value = serde_json::from_slice(&wire)?;
        canonicalize_decorator_diff_map(&mut value);
        Ok(serde_json::to_string(&value)?)
    }).collect()
}

fn m6_color_authority_map(world: &bevy_ecs::world::World) -> Result<serde_json::Value> {
    let mut result = m6_authority_map(world);
    for row in result.as_array_mut().context("wall authority rows")? {
        let roof = row.get_mut("roof").context("wall roof field")?;
        if !roof.is_null() {
            let typed: country_core::resources::roofs::roof_shape::Roof = serde_json::from_value(roof.clone())?;
            *roof = serde_json::Value::String(serde_json::to_string(&typed)?);
        }
    }
    Ok(result)
}

fn m6_check_disk_history(world: &bevy_ecs::world::World, folder: &std::path::Path, expected_wire: &[String], stage: &str) -> Result<()> {
    use country_core::{resources::history::{History, serialized::SavedMultiEdit}, systems::save::load_game::HistorySectionJson};
    let document: serde_json::Value = serde_json::from_slice(&std::fs::read(folder.join("history.json"))?)?;
    let disk: HistorySectionJson = serde_json::from_value(document.get("History").context("missing disk History")?.clone())?;
    let history = world.resource::<History>();
    let ledger = history.ledger();
    let runtime: Vec<_> = ledger.entries.iter().cloned().map(SavedMultiEdit).collect();
    let runtime_wire = m6_history_wire(&runtime)?;
    let disk_wire = m6_history_wire(&disk.edit_chain)?;
    for (source, actual) in [("runtime", &runtime_wire), ("disk", &disk_wire)] {
        if actual.as_slice() == expected_wire { continue; }
        if let Some(root) = std::env::var_os("TG_M6_DUMP_DIR") {
            let root = PathBuf::from(root).join("history-mismatches");
            std::fs::create_dir_all(&root)?;
            let pending = tempfile::Builder::new().prefix("run-").tempdir_in(&root)?;
            std::fs::write(pending.path().join("expected.json"), serde_json::to_vec_pretty(expected_wire)?)?;
            std::fs::write(pending.path().join("actual.json"), serde_json::to_vec_pretty(actual)?)?;
            std::fs::write(pending.path().join("stage.txt"), format!("{stage} {source}"))?;
            eprintln!("M6 history mismatch retained: {}", pending.keep().display());
        }
        for index in 0..actual.len().max(expected_wire.len()) {
            match (expected_wire.get(index), actual.get(index)) {
                (Some(expected), Some(actual)) if expected != actual => {
                    let expected: serde_json::Value = serde_json::from_str(expected)?;
                    let actual: serde_json::Value = serde_json::from_str(actual)?;
                    if let Some((path, expected, actual)) = m6_first_json_difference(&expected, &actual) {
                        anyhow::bail!("{stage} {source} typed history entry {index} differs at {path}: parent={expected}, actual={actual}");
                    }
                    anyhow::bail!("{stage} {source} typed history entry {index} has unequal canonical wire text");
                }
                (Some(_), None) | (None, Some(_)) => anyhow::bail!(
                    "{stage} {source} typed history length differs: parent={}, actual={}, first absent entry={index}", expected_wire.len(), actual.len()),
                _ => {}
            }
        }
        anyhow::bail!("{stage} {source} differs from settled parent typed history");
    }
    anyhow::ensure!(ledger.cursor == disk.chain_index, "{stage} cursor differs: runtime={} disk={}", ledger.cursor, disk.chain_index);
    anyhow::ensure!(ledger.undo_anchor == disk.history_start, "{stage} history_start differs: runtime={} disk={}", ledger.undo_anchor, disk.history_start);
    Ok(())
}

fn m6_check_parent_history(expected: &serde_json::Value, actual: &serde_json::Value, stage: &str) -> Result<()> {
    use country_core::resources::history::serialized::SavedMultiEdit;
    if let Some((path, before, after)) = m6_first_json_difference(expected, actual) {
        eprintln!("{stage} raw JSON first difference at {path}: parent={before}, child={after}; checking typed history");
    }
    let parent: Vec<SavedMultiEdit> = serde_json::from_value(expected.clone()).context("parent typed history")?;
    let child: Vec<SavedMultiEdit> = serde_json::from_value(actual.clone()).context("child typed history")?;
    let expected = m2_saved_history_values(&parent)?;
    let actual = m2_saved_history_values(&child)?;
    if let Some((path, before, after)) = m2_first_json_difference(&expected, &actual) {
        anyhow::bail!("{stage} typed parent/child history differs at {path}: parent={before}, child={after}");
    }
    Ok(())
}

fn m6_color_pointer_input(input: bevy_ecs::prelude::Res<M6ColorInput>,
    mut cursor: bevy_ecs::prelude::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,
    mut actions: bevy_ecs::prelude::ResMut<country_core::input::action_set::ActionSetStates>) {
    cursor.0 = input.cursor;
    for (slot, phase) in [(0, input.click), (16, input.undo), (17, input.redo)] {
        actions.digital[slot] = country_core::input::action_set::DigitalActionState { phase, timer_seconds: 0.0 };
    }
}

fn m6_trace_color_frame(world: &mut bevy_ecs::world::World, stage: &str) {
    use country_core::{resources::color_icon::CustomizationIcon,
        systems::{mode_manager::AppMode, ui::resources::ui_state::UiState}};
    let ui = world.resource::<UiState>();
    eprintln!("M6_COLOR {stage} mode={:?} active={:?} interaction={:?} hover={} show_events={} hide_events={} welcome={:?}",
        world.resource::<AppMode>(), ui.active_widget(), ui.interaction(), ui.hover_byte(),
        world.resource::<bevy_ecs::event::Events<system_color::ShowColorMenuCmd>>().len(),
        world.resource::<bevy_ecs::event::Events<system_color::HideColorMenuCmd>>().len(),
        world.resource::<country_core::resources::onboarding::WelcomeScreenState>());
    let mut menus = world.query::<&system_color::radial_menu::ColorRadialMenu>();
    let snapshot: Vec<_> = menus.iter(world).map(|menu| (menu.target, menu.icon_entities.clone())).collect();
    for (target, entities) in snapshot {
        let icons: Vec<_> = entities.into_iter().take(5).map(|id| (id,
            world.get::<CustomizationIcon>(id).map(|icon| icon.alpha_18),
            world.get::<country_core::resources::color_icon::ColorIcon>(id).map(|color| color.0),
            world.get::<country_core::components::Ui2dExtent>(id).map(|extent| (extent.pos, extent.size)))).collect();
        eprintln!("M6_COLOR {stage} target={target:?} icons={icons:?}");
    }
}

fn m6_trace_decorator_id(world: &bevy_ecs::world::World, stage: &str) {
    use country_core::resources::walls::{decorator::{DecoratorId, DecoratorIdGen}, decorator_storage::DecoratorStorage};
    let id = world.resource::<DecoratorIdGen>().peek();
    let storage = world.resource::<DecoratorStorage>();
    eprintln!("M6_ID {stage} next={id:?} address={:?} info={:?} zero_addr={:?} zero_info={:?}",
        storage.lookup_storage(id), storage.lookup_decorator_info(id),
        storage.lookup_storage(DecoratorId(0)), storage.lookup_decorator_info(DecoratorId(0)));
}

fn m6_visible_icon_point(world: &bevy_ecs::world::World,
    extent: &country_core::components::Ui2dExtent) -> Option<glam::Vec2> {
    let size = world.resource::<country_core::resources::window::WindowSize>().as_vec2();
    let minimum = (extent.pos - extent.size * 0.5).max(glam::Vec2::ZERO);
    let maximum = (extent.pos + extent.size * 0.5).min(size - glam::Vec2::ONE);
    (maximum.x > minimum.x && maximum.y > minimum.y).then(|| (minimum + maximum) * 0.5)
}

#[test]
fn m06_real_wall_color_picker_history_save_new_process() -> Result<()> {
    m6_wall_color_scenario(false)
}

#[test]
fn m06_real_wall_color_preview_cancel_restores_authority_and_gpu() -> Result<()> {
    m6_wall_color_scenario(true)
}

fn m6_wall_color_scenario(cancel_preview: bool) -> Result<()> {
    use bevy_ecs::schedule::IntoScheduleConfigs;
    use country_core::{input::action_set::DigitalActionPhase as Phase,
        resources::{color_icon::{ColorIcon, CustomizationIcon}, color_id::WallColorIds,
            history::{History, MaybePendingSnapshotId}, mouse_terrain_raycast::{camera_mouse_ray, CursorPositionSS},
            walls::WallId, window::WindowSize},
        systems::{camera_rig::MainCamera, collision::world::RaycastWorld,
            mode_manager::{self, AppMode}, ui::hover_intersection::hover_intersection}};
    use system_color::radial_menu::ColorRadialMenu;
    if std::env::var_os("TG_M6_COLOR_CHILD").is_some() { return m6_color_reload_child(); }
    let BootedSession { mut driver, context, mut bridge, shaders, device, _boot } = boot_shipped_session_with_nani_readback(true)?;
    driver.app_mut().init_resource::<M6DrawnWallColorBuffer>();
    let world = driver.app_mut().world_mut();
    tiny_glade::production_renderer::warm_up_gameplay(bridge.warmup_owners()?, world)?;
    bridge.accept_gameplay_warmup(world.resource::<tiny_glade::render_bridge::CompletedGameplayWarmup>())?;
    for _ in 0..8 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    // Search screen pixels using the same camera unprojection and real collider
    // owner as camera_mouse_ray; no hit or menu command is supplied by the test.
    m6_dismiss_welcome(&mut driver, &context, &mut bridge)?;
    m6_settle_history(&mut driver, &context, &mut bridge)?;
    let (wall, cursor) = {
        let world = driver.app().world();
        let camera = world.resource::<MainCamera>();
        let window = world.resource::<WindowSize>();
        let collision = world.resource::<RaycastWorld>();
        let mut result = None;
        for y in (12..window.height.saturating_sub(12)).step_by(6) {
            for x in (12..window.width.saturating_sub(12)).step_by(6) {
                let point = glam::Vec2::new(x as f32, y as f32);
                let ray = utils::geometry::Ray::from_mouse_ss(point, window.as_vec2(), camera.read_matrix(0x40), camera.read_matrix(0xc0));
                let hit = collision.raycast(ray.origin, ray.direction, 0xcb);
                if hit.is_hit() && matches!(hit.handle[0] as u8, 0 | 6 | 7)
                    && world.resource::<country_core::resources::walls::public_walls::PublicWalls>().get(WallId(hit.handle[1])).is_some() {
                    // The original radial menu opens at the clicked pixel and
                    // has no viewport fitting. Choose a real wall ray with
                    // room to its right/above, keeping every click on screen.
                    if result.as_ref().is_none_or(|(_, old): &(WallId, glam::Vec2)|
                        point.x < old.x || (point.x == old.x && point.y > old.y)) {
                        result = Some((WallId(hit.handle[1]), point));
                    }
                }
            }
        }
        result.context("no visible published wall collider under a real camera ray")?
    };
    let before = driver.app().world().resource::<WallColorIds>().0[wall.0 as usize];
    let before_authority = m6_color_authority_map(driver.app().world())?;
    let history_before = driver.app().world().resource::<History>().ledger().entries.len();
    let cursor_before = driver.app().world().resource::<History>().ledger().cursor;
    driver.app_mut().insert_resource(M6ColorInput { cursor, click: Phase::Idle, undo: Phase::Idle, redo: Phase::Idle });
    driver.app_mut().add_systems(bevy_app::Update, m6_color_pointer_input
        .after(country_core::systems::backend::input::update_cursor_position::update_cursor_position)
        .before(hover_intersection).before(camera_mouse_ray)
        .before(mode_manager::mode_manager::<country_core::resources::history::EditType, MaybePendingSnapshotId>));
    m6_request_mode(&mut driver, &context, &mut bridge, AppMode::Color)?;
    eprintln!("M6_COLOR target_wall={wall:?} cursor={cursor:?}");
    for phase in [Phase::Idle, Phase::JustPressed, Phase::JustReleased, Phase::Idle] {
        driver.app_mut().world_mut().resource_mut::<M6ColorInput>().click = phase;
        run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
        m6_trace_color_frame(driver.app_mut().world_mut(), &format!("picker {phase:?}"));
    }
    let (chosen, position) = {
        let mut found = None;
        for _ in 0..90 {
            run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
            let world = driver.app_mut().world_mut();
            let mut menus = world.query::<&ColorRadialMenu>();
            let entities = menus.iter(world).find(|menu| menu.target.kind == 0 && menu.target.id == wall.0)
                .map(|menu| menu.icon_entities.clone());
            if let Some(entities) = entities {
                found = entities.into_iter().filter_map(|id| {
                    let color = world.get::<ColorIcon>(id)?.0;
                    let icon = world.get::<CustomizationIcon>(id)?;
                    let extent = world.get::<country_core::components::Ui2dExtent>(id)?;
                    let position = m6_visible_icon_point(world, extent)?;
                    (color != before && color != 777 && icon.alpha_18 >= 0.7 && extent.size.x > 0.0
                        ).then_some((color, position))
                }).next();
                if found.is_some() { break; }
            }
        }
        if found.is_none() { m6_trace_color_frame(driver.app_mut().world_mut(), "picker exhausted"); }
        found.context("real wall menu did not produce an actionable different color icon")?
    };
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().cursor = position;
    if cancel_preview {
        for _ in 0..4 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
        anyhow::ensure!(driver.app().world().resource::<WallColorIds>().0[wall.0 as usize] == chosen,
            "real icon hover did not preview a different wall color");
        m6_assert_wall_color_gpu(driver.app().world(), &bridge, &device)?;
        use country_core::{game_profile::{GameProfile, desktop_keycode_index},
            systems::backend::input::button_input::KeyButtonInput};
        let binding = driver.app().world().resource::<GameProfile>().key_bindings.bindings[28]
            .context("color cancel binding missing")?;
        let code = desktop_keycode_index(binding).context("color cancel has no desktop key")?;
        driver.app_mut().world_mut().resource_mut::<KeyButtonInput>().press(code);
        run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
        {
            let mut keys = driver.app_mut().world_mut().resource_mut::<KeyButtonInput>();
            keys.release_all();
            keys.clear();
        }
        for _ in 0..60 {
            run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
            let world = driver.app_mut().world_mut();
            if world.query::<&ColorRadialMenu>().iter(world).next().is_none()
                && world.query::<&CustomizationIcon>().iter(world).next().is_none() { break; }
        }
        m6_settle_history(&mut driver, &context, &mut bridge)?;
        let world = driver.app_mut().world_mut();
        anyhow::ensure!(world.resource::<WallColorIds>().0[wall.0 as usize] == before,
            "cancel did not restore original wall color");
        anyhow::ensure!(m6_color_authority_map(world)? == before_authority,
            "cancel did not restore complete wall/roof authority");
        let history = world.resource::<History>();
        anyhow::ensure!(history.ledger().entries.len() == history_before && history.ledger().cursor == cursor_before,
            "cancelled color preview changed history");
        anyhow::ensure!(world.query::<&ColorRadialMenu>().iter(world).next().is_none()
            && world.query::<&CustomizationIcon>().iter(world).next().is_none(),
            "cancel left menu/icon entities alive");
        m6_assert_wall_color_gpu(world, &bridge, &device)?;
        return Ok(());
    }
    for phase in [Phase::Idle, Phase::JustPressed, Phase::JustReleased, Phase::Idle, Phase::Idle] {
        driver.app_mut().world_mut().resource_mut::<M6ColorInput>().click = phase;
        run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    }
    for _ in 0..12 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    let world = driver.app().world();
    anyhow::ensure!(world.resource::<WallColorIds>().0[wall.0 as usize] == chosen, "real color icon click did not commit the wall color");
    m6_assert_wall_color_gpu(world, &bridge, &device)?;
    let history = world.resource::<History>();
    anyhow::ensure!(history.ledger().entries.len() > history_before, "wall color click did not commit history");
    let committed_cursor = history.ledger().cursor;
    anyhow::ensure!(history.ledger().entries.last().is_some_and(|entry| entry.edits.iter().any(|edit| {
        matches!(edit, country_core::resources::history::EditType::Color(color)
            if matches!(color.as_ref(), country_core::resources::history::color_edit::ColorEdit::Wall {
                wall_id, from, to
            } if *wall_id == wall && from.0 == before && to.0 == chosen))
    })), "history does not contain the clicked wall and its exact before/after colors");
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().undo = Phase::JustPressed;
    run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().undo = Phase::Idle;
    m6_wait_rendered_replay_completion(&mut driver, &context, &mut bridge, committed_cursor - 1, "color undo")?;
    anyhow::ensure!(driver.app().world().resource::<WallColorIds>().0[wall.0 as usize] == before, "color undo failed");
    m6_assert_wall_color_gpu(driver.app().world(), &bridge, &device)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().redo = Phase::JustPressed;
    run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().redo = Phase::Idle;
    m6_wait_rendered_replay_completion(&mut driver, &context, &mut bridge, committed_cursor, "color redo")?;
    anyhow::ensure!(driver.app().world().resource::<WallColorIds>().0[wall.0 as usize] == chosen, "color redo failed");
    m6_assert_wall_color_gpu(driver.app().world(), &bridge, &device)?;
    m6_settle_history(&mut driver, &context, &mut bridge)?;
    let world = driver.app().world();
    let after_authority = m6_color_authority_map(world)?;
    let history = world.resource::<History>();
    let settled_entries: Vec<_> = history.ledger().entries.iter().cloned().map(country_core::resources::history::serialized::SavedMultiEdit).collect();
    let settled_wire = m6_history_wire(&settled_entries)?;
    let mut history_json = serde_json::to_value(history.ledger().entries.iter().cloned()
        .map(country_core::resources::history::serialized::SavedMultiEdit).collect::<Vec<_>>())?;
    for entry in history_json.as_array_mut().context("history entries are not an array")? { canonicalize_decorator_diff_map(entry); }
    let expected_json = serde_json::json!({"history": history_json, "typed_history": settled_wire, "cursor": committed_cursor,
        "before_authority": before_authority, "after_authority": after_authority, "before_color": before});
    let session = world.resource::<ActiveSession>().id;
    driver.app_mut().world_mut().send_event(country_core::systems::save::save_game::SaveGameCmd);
    let mut saved = false;
    for _ in 0..600 {
        run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
        if driver.app_mut().world_mut().resource_mut::<bevy_ecs::event::Events<country_core::systems::save::save_game::SavingGameComplete>>()
            .drain().any(|event| event.path == session.save_folder_path()) { saved = true; break; }
    }
    anyhow::ensure!(saved, "color save worker did not finish");
    let expected_dir = tempfile::tempdir()?;
    let expected_path = expected_dir.path().join("wall-color-history.json");
    std::fs::write(&expected_path, serde_json::to_vec(&expected_json)?)?;
    m6_retain_color_diagnostic("wall-color", &session.save_folder_path(), &expected_path, serde_json::json!({
        "test": "m06_real_wall_color_picker_history_save_new_process",
        "child_env": {"TG_M6_COLOR_CHILD":"1", "TG_M6_COLOR_FOLDER":"save", "TG_M6_COLOR_HISTORY_FILE":"expected.json",
            "TG_M6_COLOR_SESSION":serde_json::to_string(&session)?, "TG_M6_COLOR_WALL":wall.0.to_string(), "TG_M6_COLOR_VALUE":chosen.to_string()},
        "cursor": committed_cursor
    }))?;
    m6_check_disk_history(driver.app().world(), &session.save_folder_path(), &settled_wire, "wall color parent")?;
    drop(driver); drop(bridge); drop(shaders); drop(device);
    let status = std::process::Command::new(std::env::current_exe()?)
        .args(["--exact", "m06_real_wall_color_picker_history_save_new_process", "--nocapture", "--test-threads=1"])
        .env("TG_M6_COLOR_CHILD", "1").env("TG_M6_COLOR_FOLDER", session.save_folder_path())
        .env("TG_M6_COLOR_SESSION", serde_json::to_string(&session)?)
        .env("TG_M6_COLOR_WALL", wall.0.to_string()).env("TG_M6_COLOR_VALUE", chosen.to_string())
        .env("TG_M6_TRACE_PREPLACED", "1")
        .env("TG_M6_COLOR_HISTORY_FILE", &expected_path).status()?;
    anyhow::ensure!(status.success(), "new process failed color recovery: {status}");
    Ok(())
}

#[test]
fn m06_real_rim_picker_commits_same_history_transaction_and_replays() -> Result<()> {
    use bevy_ecs::schedule::IntoScheduleConfigs;
    use country_core::{input::action_set::DigitalActionPhase as Phase,
        resources::{color_icon::{CustomizationIcon, FlatRoofRimTypeToggle}, history::{EditType, History, MaybePendingSnapshotId},
            mouse_terrain_raycast::camera_mouse_ray, roofs::roof_shape::Roof,
            walls::{WallId, public_walls::PublicWalls}, window::WindowSize},
        systems::{camera_rig::MainCamera, collision::world::RaycastWorld, mode_manager::{self, AppMode},
            ui::hover_intersection::hover_intersection}};
    use system_color::radial_menu::ColorRadialMenu;
    if std::env::var_os("TG_M6_RIM_CHILD").is_some() { return m6_rim_reload_child(); }
    let BootedSession { mut driver, context, mut bridge, shaders, device, _boot } = boot_shipped_session()?;
    eprintln!("M6_RIM session booted");
    let world = driver.app_mut().world_mut();
    tiny_glade::production_renderer::warm_up_gameplay(bridge.warmup_owners()?, world)?;
    bridge.accept_gameplay_warmup(world.resource::<tiny_glade::render_bridge::CompletedGameplayWarmup>())?;
    for _ in 0..8 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    // Construct through the production wall and roof edit consumers; do not
    // seed a Roof component or fabricate the later collider/widget hit.
    m6_dismiss_welcome(&mut driver, &context, &mut bridge)?;
    m6_settle_history(&mut driver, &context, &mut bridge)?;
    eprintln!("M6_RIM rendered warmup and welcome complete");
    let before_walls: std::collections::HashSet<_> = driver.app().world().resource::<InnerWalls>().wall_ids().collect();
    driver.app_mut().world_mut().resource_mut::<History>().start_input_recording(0)
        .map_err(|error| anyhow::anyhow!("rim setup recording failed: {error:?}"))?;
    driver.app_mut().world_mut().send_event(country_core::systems::wall::shapes::CreateRectangleCmd {
        center: glam::Vec2::ZERO, dimensions: glam::Vec2::new(6.0, 4.0), orientation: glam::Vec2::X,
        previous_wall: None, height: 4.0, elevation: 1.5,
    });
    for _ in 0..20 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    let created_wall = driver.app().world().resource::<InnerWalls>().wall_ids()
        .find(|id| !before_walls.contains(id)).context("rectangle setup did not create a wall")?;
    anyhow::ensure!(driver.app().world().resource::<PublicWalls>().roof_entity(created_wall).is_some(), "created wall has no roof");
    eprintln!("M6_RIM rectangle published: {created_wall:?}");
    use country_core::resources::roofs::{commands::EditRoofCmd, geometry::edit_rules::{RoofEditEvent, ScalarEdit, TipEdit}};
    let keep = ScalarEdit { mode: 0, value: 0.0 };
    driver.app_mut().world_mut().send_event(EditRoofCmd(RoofEditEvent {
        wall_id: created_wall.0, profile: keep, height: ScalarEdit { mode: 1, value: 0.0 },
        ridge: keep, eave: keep, tip: TipEdit { mode: 0, value: glam::Vec2::ZERO }, source_flag: 0,
    }));
    for _ in 0..20 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    if driver.app().world().resource::<History>().is_recording() {
        driver.app_mut().world_mut().send_event(country_core::resources::history::FinishInputRecordingCmd);
    }
    for _ in 0..3 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    let (wall, roof_entity, before, cursor) = {
        let world = driver.app().world();
        let camera = world.resource::<MainCamera>();
        let window = world.resource::<WindowSize>();
        let collision = world.resource::<RaycastWorld>();
        let walls = world.resource::<PublicWalls>();
        let mut found = None;
        for y in (4..window.height.saturating_sub(4)).step_by(3) {
            for x in (4..window.width.saturating_sub(4)).step_by(3) {
                let cursor = glam::Vec2::new(x as f32, y as f32);
                let ray = utils::geometry::Ray::from_mouse_ss(cursor, window.as_vec2(), camera.read_matrix(0x40), camera.read_matrix(0xc0));
                let hit = collision.raycast(ray.origin, ray.direction, 0xcb);
                if !hit.is_hit() || !matches!(hit.handle[0] as u8, 0 | 6 | 7) { continue; }
                let wall = WallId(hit.handle[1]);
                if wall != created_wall { continue; }
                if !walls.get(wall).is_some_and(|state| state.shape().tag() < 2) { continue; }
                let Some(entity) = walls.roof_entity(wall) else { continue; };
                let Some(roof) = world.get::<Roof>(entity) else { continue; };
                if !roof.is_flat() { continue; }
                if found.as_ref().is_none_or(|(_, _, _, old): &(_, _, _, glam::Vec2)|
                    cursor.x < old.x || (cursor.x == old.x && cursor.y > old.y)) {
                    found = Some((wall, entity, roof.observed()[0x3d], cursor));
                }
            }
        }
        found.context("no visible wall with a flat roof for the rim picker")?
    };
    eprintln!("M6_RIM flat roof picker: {wall:?}, cursor={cursor:?}");
    let history_before = driver.app().world().resource::<History>().ledger().entries.len();
    driver.app_mut().insert_resource(M6ColorInput { cursor, click: Phase::Idle, undo: Phase::Idle, redo: Phase::Idle });
    driver.app_mut().add_systems(bevy_app::Update, m6_color_pointer_input
        .after(country_core::systems::backend::input::update_cursor_position::update_cursor_position)
        .before(hover_intersection).before(camera_mouse_ray)
        .before(mode_manager::mode_manager::<EditType, MaybePendingSnapshotId>));
    m6_request_mode(&mut driver, &context, &mut bridge, AppMode::Color)?;
    for phase in [Phase::Idle, Phase::JustPressed, Phase::JustReleased, Phase::Idle] {
        driver.app_mut().world_mut().resource_mut::<M6ColorInput>().click = phase;
        run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    }
    let mut rim_position = None;
    for _ in 0..90 {
        run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
        let world = driver.app_mut().world_mut();
        let mut menus = world.query::<&ColorRadialMenu>();
        let icons = menus.iter(world).find(|menu| menu.target.kind == 0 && menu.target.id == wall.0)
            .map(|menu| menu.icon_entities.clone());
        if let Some(icons) = icons {
            rim_position = icons.into_iter().find_map(|entity| {
                world.get::<FlatRoofRimTypeToggle>(entity)?;
                let icon = world.get::<CustomizationIcon>(entity)?;
                let extent = world.get::<country_core::components::Ui2dExtent>(entity)?;
                let position = m6_visible_icon_point(world, extent)?;
                (icon.alpha_18 >= 0.7 && extent.size.x > 0.0).then_some(position)
            });
        }
        if rim_position.is_some() { break; }
    }
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().cursor = rim_position.context("rim toggle did not appear")?;
    for phase in [Phase::Idle, Phase::JustPressed, Phase::JustReleased, Phase::Idle, Phase::Idle] {
        driver.app_mut().world_mut().resource_mut::<M6ColorInput>().click = phase;
        run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    }
    for _ in 0..4 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    let world = driver.app().world();
    let after = world.get::<Roof>(roof_entity).context("roof disappeared")?.observed()[0x3d];
    anyhow::ensure!(after == (before ^ 1), "rim input did not toggle roof byte");
    let history = world.resource::<History>();
    anyhow::ensure!(history.ledger().entries.len() == history_before + 1, "rim input did not commit one transaction");
    let entry = history.ledger().entries.last().context("rim history entry missing")?;
    anyhow::ensure!(entry.edits.iter().filter(|edit| matches!(edit, EditType::Roof(roof) if roof.tag() == 4 && roof.wall_id() == wall.0 && roof.payload()[1] == before && roof.payload()[2] == after)).count() == 1,
        "same transaction does not contain exactly one target rim edit");
    let committed_cursor = history.ledger().cursor;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().undo = Phase::JustPressed;
    run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().undo = Phase::Idle;
    m6_wait_rendered_replay_completion(&mut driver, &context, &mut bridge, committed_cursor - 1, "rim undo")?;
    for _ in 0..3 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    anyhow::ensure!(driver.app().world().get::<Roof>(roof_entity).context("roof missing after undo")?.observed()[0x3d] == before, "rim undo did not restore state");
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().redo = Phase::JustPressed;
    run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().redo = Phase::Idle;
    m6_wait_rendered_replay_completion(&mut driver, &context, &mut bridge, committed_cursor, "rim redo")?;
    for _ in 0..3 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    anyhow::ensure!(driver.app().world().get::<Roof>(roof_entity).context("roof missing after redo")?.observed()[0x3d] == after, "rim redo did not restore state");
    m6_settle_history(&mut driver, &context, &mut bridge)?;
    let world = driver.app().world();
    let roof_json = serde_json::to_string(world.get::<Roof>(roof_entity).context("roof missing before save")?)?;
    let settled_entries: Vec<_> = world.resource::<History>().ledger().entries.iter().cloned().map(country_core::resources::history::serialized::SavedMultiEdit).collect();
    let settled_wire = m6_history_wire(&settled_entries)?;
    let mut history_json = serde_json::to_value(world.resource::<History>().ledger().entries.iter().cloned()
        .map(country_core::resources::history::serialized::SavedMultiEdit).collect::<Vec<_>>())?;
    for entry in history_json.as_array_mut().context("history entries are not an array")? { canonicalize_decorator_diff_map(entry); }
    let session = world.resource::<ActiveSession>().id;
    let folder = session.save_folder_path();
    driver.app_mut().world_mut().send_event(country_core::systems::save::save_game::SaveGameCmd);
    let mut saved = false;
    for _ in 0..600 {
        run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
        if driver.app_mut().world_mut().resource_mut::<bevy_ecs::event::Events<country_core::systems::save::save_game::SavingGameComplete>>()
            .drain().any(|event| event.path == folder) { saved = true; break; }
    }
    anyhow::ensure!(saved && folder.join("history.json").is_file(), "rim save worker did not finish");
    let expected_dir = tempfile::tempdir()?;
    let expected_path = expected_dir.path().join("rim-expected.json");
    std::fs::write(&expected_path, serde_json::to_vec(&serde_json::json!({
        "roof_wire": roof_json,
        "history": history_json, "typed_history": settled_wire,
    }))?)?;
    m6_retain_color_diagnostic("roof-rim", &folder, &expected_path, serde_json::json!({
        "test":"m06_real_rim_picker_commits_same_history_transaction_and_replays",
        "child_env":{"TG_M6_RIM_CHILD":"1", "TG_M6_RIM_FOLDER":"save", "TG_M6_RIM_EXPECTED_FILE":"expected.json",
            "TG_M6_RIM_SESSION":serde_json::to_string(&session)?, "TG_M6_RIM_WALL":wall.0.to_string(),
            "TG_M6_RIM_BEFORE":before.to_string(), "TG_M6_RIM_AFTER":after.to_string(), "TG_M6_RIM_CURSOR":committed_cursor.to_string()}
    }))?;
    m6_check_disk_history(driver.app().world(), &folder, &settled_wire, "rim parent")?;
    drop(driver); drop(bridge); drop(shaders); drop(device);
    let status = std::process::Command::new(std::env::current_exe()?)
        .args(["--exact", "m06_real_rim_picker_commits_same_history_transaction_and_replays", "--nocapture", "--test-threads=1"])
        .env("TG_M6_RIM_CHILD", "1").env("TG_M6_RIM_FOLDER", folder)
        .env("TG_M6_RIM_SESSION", serde_json::to_string(&session)?)
        .env("TG_M6_RIM_WALL", wall.0.to_string()).env("TG_M6_RIM_BEFORE", before.to_string())
        .env("TG_M6_RIM_AFTER", after.to_string()).env("TG_M6_RIM_CURSOR", committed_cursor.to_string())
        .env("TG_M6_RIM_EXPECTED_FILE", &expected_path).status()?;
    anyhow::ensure!(status.success(), "new process failed rim recovery: {status}");
    Ok(())
}

fn m6_rim_reload_child() -> Result<()> {
    use bevy_ecs::schedule::IntoScheduleConfigs;
    use country_core::{input::action_set::DigitalActionPhase as Phase,
        meta_commander::{MetaCommand, MetaCommander},
        resources::{history::{EditType, History, MaybePendingSnapshotId, ReplayBuffer},
            mouse_terrain_raycast::camera_mouse_ray, roofs::roof_shape::Roof, session::SessionId,
            walls::{WallId, public_walls::PublicWalls}},
        systems::{history::PendingHistoryCommands, mode_manager::{self, AppMode}, ui::hover_intersection::hover_intersection}};
    let session: SessionId = serde_json::from_str(&std::env::var("TG_M6_RIM_SESSION")?)?;
    let wall = WallId(std::env::var("TG_M6_RIM_WALL")?.parse()?);
    let before: u8 = std::env::var("TG_M6_RIM_BEFORE")?.parse()?;
    let after: u8 = std::env::var("TG_M6_RIM_AFTER")?.parse()?;
    let cursor: i32 = std::env::var("TG_M6_RIM_CURSOR")?.parse()?;
    let source = PathBuf::from(std::env::var_os("TG_M6_RIM_FOLDER").context("missing rim save")?);
    let BootedSession { mut driver, context, mut bridge, .. } = boot_shipped_session()?;
    {
        let world = driver.app_mut().world_mut();
        tiny_glade::production_renderer::warm_up_gameplay(bridge.warmup_owners()?, world)?;
        bridge.accept_gameplay_warmup(world.resource::<tiny_glade::render_bridge::CompletedGameplayWarmup>())?;
    }
    copy_test_tree(&source, &session.save_folder_path())?;
    driver.app_mut().world_mut().resource_mut::<MetaCommander>().extend_script([MetaCommand::load_session(session, false)]);
    let mut saw_reload = false; let mut loaded = false;
    for _ in 0..1200 {
        context.begin_frame(egui::RawInput::default()); driver.app_mut().update(); let _ = context.end_frame();
        bridge.prepare_test_execution()?;
        let world = driver.app().world();
        let state = world.resource::<bevy_state::prelude::State<country_core::app_loading_state::AppLoadingState>>().get();
        saw_reload |= *state == country_core::app_loading_state::AppLoadingState::Reload;
        if saw_reload && *state == country_core::app_loading_state::AppLoadingState::Game
            && world.get_resource::<ActiveSession>().is_some_and(|s| s.id == session)
            && world.resource::<LoadGameManager>().options.is_none() && world.resource::<ReplayBuffer>().is_empty()
            && world.resource::<PendingHistoryCommands>().0.is_empty() && world.resource::<AppMode>().native_tag() != 13 { loaded = true; break; }
    }
    anyhow::ensure!(loaded, "rim session reload incomplete");
    for _ in 0..8 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    let world = driver.app().world();
    let entity = world.resource::<PublicWalls>().roof_entity(wall).context("reloaded roof association missing")?;

    let roof = world.get::<Roof>(entity).context("reloaded roof missing")?;
    let expected: serde_json::Value = serde_json::from_slice(&std::fs::read(std::env::var_os("TG_M6_RIM_EXPECTED_FILE").context("missing rim expectation file")?)?)?;
    let expected_wire: Vec<String> = serde_json::from_value(expected["typed_history"].clone())?;
    m6_check_disk_history(world, &source, &expected_wire, "rim child")?;
    let expected_roof = expected["roof_wire"].as_str().context("missing typed roof wire")?;
    anyhow::ensure!(serde_json::to_string(roof)? == expected_roof, "reloaded full saved roof differs");
    anyhow::ensure!(roof.observed()[0x3d] == after, "reloaded rim differs");
    let history = world.resource::<History>();
    let mut actual_history = serde_json::to_value(history.ledger().entries.iter().cloned()
        .map(country_core::resources::history::serialized::SavedMultiEdit).collect::<Vec<_>>())?;
    for entry in actual_history.as_array_mut().context("history entries are not an array")? { canonicalize_decorator_diff_map(entry); }
    m6_check_parent_history(&expected["history"], &actual_history, "rim")?;
    anyhow::ensure!(history.ledger().cursor == cursor, "reloaded rim cursor differs");
    driver.app_mut().insert_resource(M6ColorInput { cursor: glam::Vec2::ZERO, click: Phase::Idle, undo: Phase::Idle, redo: Phase::Idle });
    driver.app_mut().add_systems(bevy_app::Update, m6_color_pointer_input
        .after(country_core::systems::backend::input::update_cursor_position::update_cursor_position)
        .before(hover_intersection).before(camera_mouse_ray)
        .before(mode_manager::mode_manager::<EditType, MaybePendingSnapshotId>));
    m6_request_mode(&mut driver, &context, &mut bridge, AppMode::Color)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().undo = Phase::JustPressed;
    run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().undo = Phase::Idle;
    m6_wait_rendered_replay_completion(&mut driver, &context, &mut bridge, cursor - 1, "reloaded rim undo")?;
    for _ in 0..3 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    anyhow::ensure!(driver.app().world().get::<Roof>(entity).context("roof missing after reloaded undo")?.observed()[0x3d] == before, "reloaded rim undo failed");
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().redo = Phase::JustPressed;
    run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().redo = Phase::Idle;
    m6_wait_rendered_replay_completion(&mut driver, &context, &mut bridge, cursor, "reloaded rim redo")?;
    for _ in 0..3 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    let roof = driver.app().world().get::<Roof>(entity).context("roof missing after reloaded redo")?;
    anyhow::ensure!(roof.observed()[0x3d] == after && serde_json::to_string(roof)? == expected_roof, "reloaded rim redo failed to restore full roof");
    Ok(())
}

#[test]
fn m06_gate_color_setter_uploads_complete_filtered_table() -> Result<()> {
    use country_core::{asset_libraries::ssbo_library::AssetSsboLibrary,
        resources::{color_id::{GateColorIdSsbo, GateColorIds, SetGateColorIdCmd},
            walls::{WallId, public_walls::PublicWalls}}};
    let BootedSession { mut driver, context, mut bridge, device, .. } = boot_shipped_session_with_nani_readback(true)?;
    let world = driver.app_mut().world_mut();
    tiny_glade::production_renderer::warm_up_gameplay(bridge.warmup_owners()?, world)?;
    bridge.accept_gameplay_warmup(world.resource::<tiny_glade::render_bridge::CompletedGameplayWarmup>())?;
    for _ in 0..8 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    let wall = driver.app().world().resource::<PublicWalls>().wall_ids().next().context("no published wall")?;
    let missing = WallId(wall.0 | (1u64 << 32));
    anyhow::ensure!(driver.app().world().resource::<PublicWalls>().get(missing).is_none(), "missing-wall test identity exists");
    driver.app_mut().world_mut().send_event(SetGateColorIdCmd::new(wall, 4, 0));
    driver.app_mut().world_mut().send_event(SetGateColorIdCmd::new(missing, 9, 0));
    run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    let world = driver.app().world();
    let colors = &world.resource::<GateColorIds>().0;
    anyhow::ensure!(colors[wall.0 as usize] == 4, "missing wall command changed the valid wall slot");
    let storage = world.resource::<AssetSsboLibrary>().get::<GateColorIdSsbo>().context("gate SSBO missing")?;
    let actual = m6_read_diagnostic_range(&device, bridge.hosts().buffers.as_ref(), &storage.buffer, 0, colors.len() * 4)?;
    let expected: Vec<u8> = colors.iter().flat_map(|color| color.to_le_bytes()).collect();
    anyhow::ensure!(actual == expected, "gate GPU table differs from filtered CPU table");
    Ok(())
}

/// 在装饰生产者后、原归档恢复前观察预览，避免把帧末权威桶当预览存续时间。
#[derive(bevy_ecs::prelude::Resource, Default)]
struct M6PlacementObservation {
    wall: u64,
    preview: Vec<(u32, country_core::resources::walls::decorator::DecoratorInfo)>,
}

fn m6_observe_placement(
    storage: bevy_ecs::prelude::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,
    mut observation: bevy_ecs::prelude::ResMut<M6PlacementObservation>,
) {
    use country_core::resources::walls::{WallId, decorator_storage::DecoratorStorageAddr};
    observation.preview = storage.get(DecoratorStorageAddr::Wall(WallId(observation.wall)))
        .into_iter().flat_map(|bucket| bucket.decorators.iter())
        .filter(|(_, info)| info.ty == b'8' && info.state() == 1)
        .map(|(&id, info)| (id, *info)).collect();
}

#[test]
fn m06_real_decorator_collision_picker_opens_matching_menu() -> Result<()> {
    use bevy_ecs::schedule::IntoScheduleConfigs;
    use country_core::{input::action_set::DigitalActionPhase as Phase,
        resources::{history::MaybePendingSnapshotId, mouse_terrain_raycast::camera_mouse_ray,
            walls::{decorator::DecoratorId, decorator_storage::{DecoratorStorage, DecoratorStorageAddr}, WallId}, window::WindowSize},
        systems::{camera_rig::MainCamera, collision::world::RaycastWorld, mode_manager::{self, AppMode},
            ui::hover_intersection::hover_intersection}};
    use system_color::radial_menu::ColorRadialMenu;
    if std::env::var_os("TG_M6_DECO_COLOR_CHILD").is_some() { return m6_decorator_color_reload_child(); }
    let BootedSession { mut driver, context, mut bridge, shaders, device, _boot } = boot_shipped_session()?;
    let world = driver.app_mut().world_mut();
    tiny_glade::production_renderer::warm_up_gameplay(bridge.warmup_owners()?, world)?;
    bridge.accept_gameplay_warmup(world.resource::<tiny_glade::render_bridge::CompletedGameplayWarmup>())?;
    for _ in 0..8 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    m6_dismiss_welcome(&mut driver, &context, &mut bridge)?;
    m6_settle_history(&mut driver, &context, &mut bridge)?;
    let before_walls: std::collections::HashSet<_> = driver.app().world().resource::<InnerWalls>().wall_ids().collect();
    let center = {
        let walls = driver.app().world().resource::<InnerWalls>();
        let mut found = None;
        for z in [0.0f32, -6.0, 6.0, -12.0, 12.0] {
            for x in [0.0f32, -6.0, 6.0, -12.0, 12.0] {
                let p = glam::Vec2::new(x, z);
                if walls.wall_ids().all(|id| walls.get_unwrapped(id).source().curve.points.windows(2).all(|pair| {
                    let a = glam::Vec2::from_array(pair[0]);
                    let b = glam::Vec2::from_array(pair[1]);
                    let ab = b - a;
                    let t = if ab.length_squared() > 0.0 { ((p - a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0) } else { 0.0 };
                    p.distance(a + ab * t) > 7.0
                })) { found = Some(p); break; }
            }
            if found.is_some() { break; }
        }
        found.context("no non-overlapping window fixture site")?
    };
    driver.app_mut().world_mut().resource_mut::<country_core::resources::history::History>().start_input_recording(0)
        .map_err(|error| anyhow::anyhow!("decorator setup recording failed: {error:?}"))?;
    driver.app_mut().world_mut().send_event(country_core::systems::wall::shapes::CreateRectangleCmd {
        center, dimensions: glam::Vec2::new(6.0, 4.0), orientation: glam::Vec2::X,
        previous_wall: None, height: 4.0, elevation: 1.5,
    });
    for _ in 0..20 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    let created_wall = driver.app().world().resource::<InnerWalls>().wall_ids()
        .find(|id| !before_walls.contains(id)).context("decorator setup did not create a wall")?;
    if driver.app().world().resource::<country_core::resources::history::History>().is_recording() {
        driver.app_mut().world_mut().send_event(country_core::resources::history::FinishInputRecordingCmd);
    }
    for _ in 0..3 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    let placement_cursor = {
        let world = driver.app().world(); let camera = world.resource::<MainCamera>();
        let window = world.resource::<WindowSize>(); let collision = world.resource::<RaycastWorld>();
        let mut found: Option<(f32, glam::Vec2)> = None;
        for y in (8..window.height.saturating_sub(8)).step_by(3) {
            for x in (8..window.width.saturating_sub(8)).step_by(3) {
                let point = glam::Vec2::new(x as f32, y as f32);
                let ray = utils::geometry::Ray::from_mouse_ss(point, window.as_vec2(), camera.read_matrix(0x40), camera.read_matrix(0xc0));
                let hit = collision.raycast(ray.origin, ray.direction, 0x30ab | 0x100);
                if hit.is_hit() && hit.handle[0] as u8 == 0 && hit.handle[1] == created_wall.0 {
                    let relative = hit.position - glam::Vec3::new(center.x, 0.0, center.y);
                    // Select the middle of a straight wall face, away from corners and roof.
                    let along = if hit.normal.x.abs() > hit.normal.z.abs() { relative.z } else { relative.x };
                    if along.abs() < 0.8 && relative.y > 2.0 && relative.y < 4.0 {
                        let score = along.abs() + (relative.y - 3.0).abs();
                        if found.as_ref().is_none_or(|(best, _)| score < *best) { found = Some((score, point)); }
                    }
                }
            }
        }
        found.map(|(_, point)| point).context("created wall has no visible central window placement ray")?
    };
    driver.app_mut().insert_resource(M6ColorInput { cursor: placement_cursor, click: Phase::Idle, undo: Phase::Idle, redo: Phase::Idle });
    driver.app_mut().add_systems(bevy_app::Update, m6_color_pointer_input
        .after(country_core::systems::backend::input::update_cursor_position::update_cursor_position)
        .before(hover_intersection).before(camera_mouse_ray)
        .before(mode_manager::mode_manager::<country_core::resources::history::EditType, MaybePendingSnapshotId>));
    m6_trace_decorator_id(driver.app().world(), "before decorator mode");
    m6_request_mode(&mut driver, &context, &mut bridge,
        AppMode::Decorator(country_core::systems::mode_manager::DecoratorType::CottageWindow))?;
    let widget = country_core::systems::ui::resources::ui_state::WidgetId::from_name("place_decorator");
    let placement_id = driver.app().world().resource::<country_core::resources::walls::decorator::DecoratorIdGen>().peek();
    let placement_history_before = driver.app().world().resource::<country_core::resources::history::History>().ledger().entries.len();
    driver.app_mut().insert_resource(M6PlacementObservation { wall: created_wall.0, ..Default::default() });
    driver.app_mut().add_systems(bevy_app::Update, m6_observe_placement
        .after(country_core::systems::GameSystemSet(25)).before(country_core::systems::GameSystemSet(26)));
    let mut active = false;
    for attempt in 0..60 {
        m6_trace_decorator_id(driver.app().world(), &format!("before wait {attempt}"));
        run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
        let world = driver.app().world();
        let ui = world.resource::<country_core::systems::ui::resources::ui_state::UiState>();
        let dispatcher = world.resource::<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>();
        if attempt % 10 == 0 {
            let backup = world.resource::<system_decorator::backup::DecoratorBackup>();
            eprintln!("M6_WINDOW wait={attempt} cursor={placement_cursor:?} active={:?} interaction={:?} place={:?} backup=({}, {})",
                ui.active_widget(), ui.interaction(), dispatcher.place_decorator(), backup.created, backup.state);
        }
        if ui.widget_context(widget).is_active() && dispatcher.place_decorator().is_some_and(|p|
            p.widget == widget && p.hit.handle[0] as u8 == 0 && p.hit.handle[1] == created_wall.0) {
            active = true; break;
        }
    }
    anyhow::ensure!(active, "central wall placement widget did not become active on the created wall");
    let mut released_info = None;
    for phase in [Phase::Idle, Phase::JustPressed, Phase::JustReleased, Phase::Idle] {
        driver.app_mut().world_mut().resource_mut::<M6ColorInput>().click = phase;
        m6_trace_decorator_id(driver.app().world(), &format!("before placement {phase:?}"));
        run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
        let world = driver.app().world();
        let ui = world.resource::<country_core::systems::ui::resources::ui_state::UiState>();
        let dispatcher = world.resource::<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>();
        let backup = world.resource::<system_decorator::backup::DecoratorBackup>();
        let bucket = world.resource::<DecoratorStorage>().get(DecoratorStorageAddr::Wall(created_wall));
        let entries: Vec<_> = bucket.into_iter().flat_map(|b| b.decorators.iter())
            .map(|(&id, info)| (id, info.ty, info.state())).collect();
        eprintln!("M6_WINDOW phase={phase:?} cursor={placement_cursor:?} active={:?} interaction={:?} place={:?} backup=({}, {}) bucket={entries:?}",
            ui.active_widget(), ui.interaction(), dispatcher.place_decorator(), backup.created, backup.state);
        if phase == Phase::JustPressed {
            anyhow::ensure!(ui.widget_context(widget).is_active() && ui.widget_context(widget).dragged() == 3,
                "window placement did not capture its real widget");
            anyhow::ensure!(world.resource::<M6PlacementObservation>().preview.iter().any(|(id, info)|
                *id == placement_id.0 && info.ty == b'8' && info.state() == 1),
                "central wall placement did not produce an accepted cottage-window preview");
        }
        if phase == Phase::JustReleased {
            released_info = world.resource::<DecoratorStorage>().lookup_decorator_info(placement_id).copied();
            anyhow::ensure!(released_info.is_some(), "window release did not commit complete info");
        }
        if phase == Phase::Idle && released_info.is_some() {
            anyhow::ensure!(world.resource::<DecoratorStorage>().lookup_decorator_info(placement_id) == released_info.as_ref(),
                "committed window changed on next frame");
        }
    }
    for _ in 0..20 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    let created_id = {
        let storage = driver.app().world().resource::<DecoratorStorage>();
        let bucket = storage.get(DecoratorStorageAddr::Wall(created_wall)).context("window placement created no wall decorator bucket")?;
        let (&id, _) = bucket.decorators.iter().find(|(_, info)| info.ty == b'8' && info.state() == 0)
            .context("real placement did not commit a cottage window")?;
        DecoratorId(id)
    };
    {
        use country_core::resources::history::{EditType, History};
        use country_core::resources::walls::decorator::DecoratorIdGen;
        let world = driver.app().world();
        let storage = world.resource::<DecoratorStorage>();
        anyhow::ensure!(created_id == placement_id, "committed window did not retain preview ID");
        anyhow::ensure!(world.resource::<DecoratorIdGen>().peek().0 == placement_id.0.wrapping_add(1), "placement did not advance ID once");
        anyhow::ensure!(storage.lookup_storage(created_id) == Some(DecoratorStorageAddr::Wall(created_wall)), "committed window d2s address differs");
        let info = storage.lookup_decorator_info(created_id).context("committed window info missing")?;
        anyhow::ensure!(info.ty == b'8' && info.state() == 0, "committed window info is not final");
        let history = world.resource::<History>();
        anyhow::ensure!(history.ledger().entries.len() > placement_history_before && history.ledger().entries[placement_history_before..].iter().any(|entry|
            entry.edits.iter().any(|edit| matches!(edit, EditType::Decorator(_) | EditType::DecoratorV5(_)))),
            "placement did not commit decorator history");
    }
    let placement_cursor_history = driver.app().world().resource::<country_core::resources::history::History>().ledger().cursor;
    let committed_info = released_info.context("released window info missing")?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().undo = Phase::JustPressed;
    run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().undo = Phase::Idle;
    m6_wait_rendered_replay_completion(&mut driver, &context, &mut bridge, placement_cursor_history - 1, "window placement undo")?;
    {
        let storage = driver.app().world().resource::<DecoratorStorage>();
        anyhow::ensure!(storage.lookup_storage(created_id).is_none() && storage.lookup_decorator_info(created_id).is_none(),
            "window placement undo left its ID or info");
    }
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().redo = Phase::JustPressed;
    run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().redo = Phase::Idle;
    m6_wait_rendered_replay_completion(&mut driver, &context, &mut bridge, placement_cursor_history, "window placement redo")?;
    {
        let storage = driver.app().world().resource::<DecoratorStorage>();
        anyhow::ensure!(storage.lookup_storage(created_id) == Some(DecoratorStorageAddr::Wall(created_wall)) &&
            storage.lookup_decorator_info(created_id) == Some(&committed_info), "window placement redo did not restore complete info and d2s");
    }
    let (id, kind, subtype, cursor) = {
        let world = driver.app().world();
        let camera = world.resource::<MainCamera>();
        let window = world.resource::<WindowSize>();
        let collision = world.resource::<RaycastWorld>();
        let storage = world.resource::<DecoratorStorage>();
        let mut found = None;
        'pixels: for y in (4..window.height.saturating_sub(4)).step_by(2) {
            for x in (4..window.width.saturating_sub(4)).step_by(2) {
                let cursor = glam::Vec2::new(x as f32, y as f32);
                let ray = utils::geometry::Ray::from_mouse_ss(cursor, window.as_vec2(), camera.read_matrix(0x40), camera.read_matrix(0xc0));
                let hit = collision.legacy_raycast_w_params(ray.origin, ray.direction, 0x3108);
                if !hit.is_hit() { continue; }
                let id = DecoratorId((hit.handle[0] >> 32) as u32);
                if id != created_id { continue; }
                let info = match hit.handle[0] as u8 {
                    13 | 14 => storage.lookup_decorator_info(id),
                    4 => { let addr = match hit.handle[1] {
                        0 => DecoratorStorageAddr::Wall(WallId(hit.handle[2])),
                        1 => DecoratorStorageAddr::Terrain, 2 => DecoratorStorageAddr::Stair, _ => continue,
                    }; storage.get(addr).and_then(|bucket| bucket.get(id)) },
                    _ => continue,
                };
                let Some(info) = info else { continue; };
                let (kind, subtype) = match info.ty {
                    b'8' => (3, 0), b'9' | b':' => (3, 1), b';' => (10, info.ty),
                    b'<' => (9, info.ty), b'=' => (6, info.ty), _ => continue,
                };
                found = Some((id, kind, subtype, cursor)); break 'pixels;
            }
        }
        found.context("no visible eligible decorator collider in shipped session")?
    };
    let before = *driver.app().world().resource::<DecoratorStorage>().lookup_decorator_info(id).context("window info missing before color")?;
    let history_before = driver.app().world().resource::<country_core::resources::history::History>().ledger().entries.len();
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().cursor = cursor;
    m6_request_mode(&mut driver, &context, &mut bridge, AppMode::Color)?;
    for phase in [Phase::Idle, Phase::JustPressed, Phase::JustReleased, Phase::Idle] {
        driver.app_mut().world_mut().resource_mut::<M6ColorInput>().click = phase;
        run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    }
    for _ in 0..12 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    let (chosen, icon_position) = {
        use country_core::resources::color_icon::{ColorIcon, CustomizationIcon};
        let mut found = None;
        for _ in 0..90 {
            run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
            let world = driver.app_mut().world_mut();
            let mut menus = world.query::<&ColorRadialMenu>();
            let icons = menus.iter(world).find(|menu| menu.target.kind == kind && menu.target.subtype == subtype && menu.target.payload == id.0)
                .map(|menu| menu.icon_entities.clone());
            if let Some(icons) = icons {
                found = icons.into_iter().find_map(|entity| {
                    let color = world.get::<ColorIcon>(entity)?.0;
                    let icon = world.get::<CustomizationIcon>(entity)?;
                    let extent = world.get::<country_core::components::Ui2dExtent>(entity)?;
                    let position = m6_visible_icon_point(world, extent)?;
                    (color != before.u28 && color != 777 && icon.alpha_18 >= 0.7 && extent.size.x > 0.0
                        ).then_some((color, position))
                });
            }
            if found.is_some() { break; }
        }
        found.context("real window menu did not produce a different actionable color")?
    };
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().cursor = icon_position;
    for phase in [Phase::Idle, Phase::JustPressed, Phase::JustReleased, Phase::Idle, Phase::Idle] {
        driver.app_mut().world_mut().resource_mut::<M6ColorInput>().click = phase;
        run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    }
    for _ in 0..12 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    use country_core::resources::history::{History, EditType, color_edit::ColorEdit, decorator_edit::DecoratorHistoryState, serialized::SavedMultiEdit};
    let world = driver.app().world();
    let after = *world.resource::<DecoratorStorage>().lookup_decorator_info(id).context("window info missing after color")?;
    anyhow::ensure!(after.u28 == chosen, "window color click did not commit chosen color");
    let history = world.resource::<History>();
    anyhow::ensure!(history.ledger().entries.len() == history_before + 1, "window color did not commit one history entry");
    anyhow::ensure!(history.ledger().entries.last().is_some_and(|entry| entry.edits.iter().any(|edit| matches!(edit,
        EditType::Color(color) if matches!(color.as_ref(), ColorEdit::Decorator { decorator_id, from, to }
            if *decorator_id == id && from.0 == before.u28 && to.0 == chosen)))), "window color history payload differs");
    let committed = history.ledger().cursor;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().undo = Phase::JustPressed;
    run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().undo = Phase::Idle;
    m6_wait_rendered_replay_completion(&mut driver, &context, &mut bridge, committed - 1, "window color undo")?;
    anyhow::ensure!(driver.app().world().resource::<DecoratorStorage>().lookup_decorator_info(id) == Some(&before), "window color undo did not restore complete info");
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().redo = Phase::JustPressed;
    run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().redo = Phase::Idle;
    m6_wait_rendered_replay_completion(&mut driver, &context, &mut bridge, committed, "window color redo")?;
    anyhow::ensure!(driver.app().world().resource::<DecoratorStorage>().lookup_decorator_info(id) == Some(&after), "window color redo did not restore complete info");
    m6_settle_history(&mut driver, &context, &mut bridge)?;
    let world = driver.app().world();
    let settled_entries: Vec<_> = world.resource::<History>().ledger().entries.iter().cloned().map(country_core::resources::history::serialized::SavedMultiEdit).collect();
    let settled_wire = m6_history_wire(&settled_entries)?;
    let mut history_json = serde_json::to_value(world.resource::<History>().ledger().entries.iter().cloned().map(SavedMultiEdit).collect::<Vec<_>>())?;
    for entry in history_json.as_array_mut().context("history entries are not an array")? { canonicalize_decorator_diff_map(entry); }
    let session = world.resource::<ActiveSession>().id;
    let folder = session.save_folder_path();
    let before_state = DecoratorHistoryState::from_info(&before);
    let after_state = DecoratorHistoryState::from_info(&after);
    anyhow::ensure!(before_state.to_info() == before && after_state.to_info() == after,
        "window history state does not preserve complete fixture info");
    let before_json = serde_json::to_string(&before_state)?;
    let after_json = serde_json::to_string(&after_state)?;
    driver.app_mut().world_mut().send_event(country_core::systems::save::save_game::SaveGameCmd);
    let mut saved = false;
    for _ in 0..600 {
        run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
        if driver.app_mut().world_mut().resource_mut::<bevy_ecs::event::Events<country_core::systems::save::save_game::SavingGameComplete>>()
            .drain().any(|event| event.path == folder) { saved = true; break; }
    }
    anyhow::ensure!(saved && folder.join("history.json").is_file(), "window color save worker did not finish");
    let expected_dir = tempfile::tempdir()?;
    let expected_path = expected_dir.path().join("window-color-expected.json");
    std::fs::write(&expected_path, serde_json::to_vec(&serde_json::json!({
        "before": serde_json::from_str::<serde_json::Value>(&before_json)?,
        "after": serde_json::from_str::<serde_json::Value>(&after_json)?,
        "history": history_json, "typed_history": settled_wire,
    }))?)?;
    m6_retain_color_diagnostic("decorator-color", &folder, &expected_path, serde_json::json!({
        "test":"m06_real_decorator_collision_picker_opens_matching_menu",
        "child_env":{"TG_M6_DECO_COLOR_CHILD":"1", "TG_M6_DECO_COLOR_FOLDER":"save", "TG_M6_DECO_COLOR_EXPECTED_FILE":"expected.json",
            "TG_M6_DECO_COLOR_SESSION":serde_json::to_string(&session)?, "TG_M6_DECO_COLOR_ID":id.0.to_string(),
            "TG_M6_DECO_COLOR_CURSOR":committed.to_string()}
    }))?;
    m6_check_disk_history(driver.app().world(), &folder, &settled_wire, "window color parent")?;
    drop(driver); drop(bridge); drop(shaders); drop(device);
    let status = std::process::Command::new(std::env::current_exe()?)
        .args(["--exact", "m06_real_decorator_collision_picker_opens_matching_menu", "--nocapture", "--test-threads=1"])
        .env("TG_M6_DECO_COLOR_CHILD", "1").env("TG_M6_DECO_COLOR_FOLDER", folder)
        .env("TG_M6_DECO_COLOR_SESSION", serde_json::to_string(&session)?)
        .env("TG_M6_DECO_COLOR_ID", id.0.to_string()).env("TG_M6_DECO_COLOR_CURSOR", committed.to_string())
        .env("TG_M6_DECO_COLOR_EXPECTED_FILE", &expected_path).status()?;
    anyhow::ensure!(status.success(), "new process failed window color recovery: {status}");
    Ok(())
}

fn m6_decorator_color_reload_child() -> Result<()> {
    use bevy_ecs::schedule::IntoScheduleConfigs;
    use country_core::{input::action_set::DigitalActionPhase as Phase,
        meta_commander::{MetaCommand, MetaCommander},
        resources::{history::{EditType, History, MaybePendingSnapshotId, ReplayBuffer, decorator_edit::DecoratorHistoryState, serialized::SavedMultiEdit},
            mouse_terrain_raycast::camera_mouse_ray, session::SessionId,
            walls::{decorator::DecoratorId, decorator_storage::DecoratorStorage}},
        systems::{history::PendingHistoryCommands, mode_manager::{self, AppMode}, ui::hover_intersection::hover_intersection}};
    let session: SessionId = serde_json::from_str(&std::env::var("TG_M6_DECO_COLOR_SESSION")?)?;
    let id = DecoratorId(std::env::var("TG_M6_DECO_COLOR_ID")?.parse()?);
    let cursor: i32 = std::env::var("TG_M6_DECO_COLOR_CURSOR")?.parse()?;
    let expected: serde_json::Value = serde_json::from_slice(&std::fs::read(std::env::var_os("TG_M6_DECO_COLOR_EXPECTED_FILE").context("missing window color expectation file")?)?)?;
    let before: DecoratorHistoryState = serde_json::from_value(expected["before"].clone())?;
    let after: DecoratorHistoryState = serde_json::from_value(expected["after"].clone())?;
    let source = PathBuf::from(std::env::var_os("TG_M6_DECO_COLOR_FOLDER").context("missing window color save")?);
    let BootedSession { mut driver, context, mut bridge, .. } = boot_shipped_session()?;
    {
        let world = driver.app_mut().world_mut();
        tiny_glade::production_renderer::warm_up_gameplay(bridge.warmup_owners()?, world)?;
        bridge.accept_gameplay_warmup(world.resource::<tiny_glade::render_bridge::CompletedGameplayWarmup>())?;
    }
    copy_test_tree(&source, &session.save_folder_path())?;
    driver.app_mut().world_mut().resource_mut::<MetaCommander>().extend_script([MetaCommand::load_session(session, false)]);
    let mut saw_reload = false; let mut loaded = false;
    for _ in 0..1200 {
        context.begin_frame(egui::RawInput::default()); driver.app_mut().update(); let _ = context.end_frame();
        bridge.prepare_test_execution()?;
        let world = driver.app().world();
        let state = world.resource::<bevy_state::prelude::State<country_core::app_loading_state::AppLoadingState>>().get();
        saw_reload |= *state == country_core::app_loading_state::AppLoadingState::Reload;
        if saw_reload && *state == country_core::app_loading_state::AppLoadingState::Game
            && world.get_resource::<ActiveSession>().is_some_and(|s| s.id == session)
            && world.resource::<LoadGameManager>().options.is_none() && world.resource::<ReplayBuffer>().is_empty()
            && world.resource::<PendingHistoryCommands>().0.is_empty() && world.resource::<AppMode>().native_tag() != 13 { loaded = true; break; }
    }
    anyhow::ensure!(loaded, "window color session reload incomplete");
    for _ in 0..8 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    let world = driver.app().world();
    anyhow::ensure!(world.resource::<DecoratorStorage>().lookup_decorator_info(id) == Some(&after.to_info()), "reloaded window complete info differs");

    let history = world.resource::<History>();
    let mut actual = serde_json::to_value(history.ledger().entries.iter().cloned().map(SavedMultiEdit).collect::<Vec<_>>())?;
    for entry in actual.as_array_mut().context("history entries are not an array")? { canonicalize_decorator_diff_map(entry); }
    let expected_wire: Vec<String> = serde_json::from_value(expected["typed_history"].clone())?;
    m6_check_disk_history(world, &source, &expected_wire, "window color child")?;
    m6_check_parent_history(&expected["history"], &actual, "window color")?;
    anyhow::ensure!(history.ledger().cursor == cursor, "reloaded window color cursor differs");
    driver.app_mut().insert_resource(M6ColorInput { cursor: glam::Vec2::ZERO, click: Phase::Idle, undo: Phase::Idle, redo: Phase::Idle });
    driver.app_mut().add_systems(bevy_app::Update, m6_color_pointer_input
        .after(country_core::systems::backend::input::update_cursor_position::update_cursor_position)
        .before(hover_intersection).before(camera_mouse_ray)
        .before(mode_manager::mode_manager::<EditType, MaybePendingSnapshotId>));
    m6_request_mode(&mut driver, &context, &mut bridge, AppMode::Color)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().undo = Phase::JustPressed;
    run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().undo = Phase::Idle;
    m6_wait_rendered_replay_completion(&mut driver, &context, &mut bridge, cursor - 1, "reloaded window color undo")?;
    anyhow::ensure!(driver.app().world().resource::<DecoratorStorage>().lookup_decorator_info(id) == Some(&before.to_info()), "reloaded window undo did not restore complete info");
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().redo = Phase::JustPressed;
    run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().redo = Phase::Idle;
    m6_wait_rendered_replay_completion(&mut driver, &context, &mut bridge, cursor, "reloaded window color redo")?;
    anyhow::ensure!(driver.app().world().resource::<DecoratorStorage>().lookup_decorator_info(id) == Some(&after.to_info()), "reloaded window redo did not restore complete info");
    Ok(())
}

fn m6_color_reload_child() -> Result<()> {
    use bevy_ecs::schedule::IntoScheduleConfigs;
    use country_core::{input::action_set::DigitalActionPhase as Phase,
        meta_commander::{MetaCommand, MetaCommander}, resources::{color_id::WallColorIds,
        history::{EditType, History, MaybePendingSnapshotId, ReplayBuffer, serialized::SavedMultiEdit},
        mouse_terrain_raycast::camera_mouse_ray, session::SessionId},
        systems::{history::PendingHistoryCommands, mode_manager::{self, AppMode}, ui::hover_intersection::hover_intersection}};
    let session: SessionId = serde_json::from_str(&std::env::var("TG_M6_COLOR_SESSION")?)?;
    let wall: usize = std::env::var("TG_M6_COLOR_WALL")?.parse()?;
    let expected: u32 = std::env::var("TG_M6_COLOR_VALUE")?.parse()?;
    let source = PathBuf::from(std::env::var_os("TG_M6_COLOR_FOLDER").context("missing parent save")?);
    let BootedSession { mut driver, context, mut bridge, shaders: _shaders, device, _boot } = boot_shipped_session_with_nani_readback(true)?;
    driver.app_mut().init_resource::<M6DrawnWallColorBuffer>();
    {
        let world = driver.app_mut().world_mut();
        tiny_glade::production_renderer::warm_up_gameplay(bridge.warmup_owners()?, world)?;
        bridge.accept_gameplay_warmup(world.resource::<tiny_glade::render_bridge::CompletedGameplayWarmup>())?;
    }
    copy_test_tree(&source, &session.save_folder_path())?;
    // 记录复制后的实际目的文件与加载过程的首条历史，区分读错路径、
    // 反序列化/迁移和加载完成后另一次录制。无需修改历史或生产调度。
    let destination = session.save_folder_path();
    let copied: serde_json::Value = serde_json::from_slice(&std::fs::read(destination.join("history.json"))?)?;
    let copied_history: country_core::systems::save::load_game::HistorySectionJson =
        serde_json::from_value(copied["History"].clone())?;
    let expected_document: serde_json::Value = serde_json::from_slice(&std::fs::read(
        std::env::var_os("TG_M6_COLOR_HISTORY_FILE").context("missing wall color history file")?)?)?;
    let expected_copied_wire: Vec<String> = serde_json::from_value(expected_document["typed_history"].clone())?;
    anyhow::ensure!(m6_history_wire(&copied_history.edit_chain)? == expected_copied_wire,
        "wall color child destination differs from settled parent before Reload");
    eprintln!("M6_COLOR_LOAD source={} destination={} version={} cursor={} snapshots={} first_edit_count={}",
        source.display(), destination.display(), copied["Version"], copied["History"]["chain_index"],
        copied["History"]["snapshots"].as_array().map_or(0, Vec::len),
        copied["History"]["edit_chain"][0]["edits"].as_array().map_or(0, Vec::len));
    driver.app_mut().world_mut().resource_mut::<MetaCommander>().extend_script([MetaCommand::load_session(session, false)]);
    let mut saw_reload = false;
    let mut loaded = false;
    let mut previous_history = None;
    for _ in 0..1200 {
        context.begin_frame(egui::RawInput::default());
        driver.app_mut().update();
        let _ = context.end_frame();
        bridge.prepare_test_execution()?;
        let world = driver.app().world();
        let state = world.resource::<bevy_state::prelude::State<country_core::app_loading_state::AppLoadingState>>().get();
        let history = world.resource::<History>();
        let load = world.resource::<LoadGameManager>();
        let source_label = match &load.source {
            country_core::systems::save::load_game::LoadGameSource::None => "none".to_owned(),
            country_core::systems::save::load_game::LoadGameSource::Path(path) => path.display().to_string(),
            country_core::systems::save::load_game::LoadGameSource::Parsed { .. } => "parsed".to_owned(),
        };
        let spawn_events = world.resource::<bevy_ecs::event::Events<
            country_core::resources::history::tree_edit::SpawnPreplacedTrees>>().len();
        let trace = (format!("{state:?}"), history.ledger().cursor, history.ledger().entries.len(),
            history.ledger().entries.first().map(|entry| entry.edits.len()), history.is_recording(),
            spawn_events, format!("{:?}", load.state), source_label);
        if previous_history.as_ref() != Some(&trace) {
            eprintln!("M6_COLOR_LOAD history={trace:?}");
            previous_history = Some(trace);
        }
        saw_reload |= *state == country_core::app_loading_state::AppLoadingState::Reload;
        if saw_reload && *state == country_core::app_loading_state::AppLoadingState::Game
            && world.get_resource::<ActiveSession>().is_some_and(|s| s.id == session)
            && world.resource::<LoadGameManager>().options.is_none()
            && world.resource::<ReplayBuffer>().is_empty() && world.resource::<PendingHistoryCommands>().0.is_empty()
            && world.resource::<AppMode>().native_tag() != 13 { loaded = true; break; }
    }
    anyhow::ensure!(loaded, "new process did not complete color save Reload");
    for _ in 0..3 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    m6_settle_history(&mut driver, &context, &mut bridge)?;
    m6_assert_wall_color_gpu(driver.app().world(), &bridge, &device)?;
    let world = driver.app().world();
    anyhow::ensure!(world.resource::<WallColorIds>().0[wall] == expected, "reloaded wall color differs");

    let expected_json: serde_json::Value = serde_json::from_slice(&std::fs::read(
        std::env::var_os("TG_M6_COLOR_HISTORY_FILE").context("missing wall color history file")?)?)?;
    let history = world.resource::<History>();
    let mut actual = serde_json::to_value(history.ledger().entries.iter().cloned().map(SavedMultiEdit).collect::<Vec<_>>())?;
    for entry in actual.as_array_mut().context("history entries are not an array")? { canonicalize_decorator_diff_map(entry); }
    let cursor = i32::try_from(expected_json["cursor"].as_i64().context("missing saved color cursor")?)?;
    let expected_wire: Vec<String> = serde_json::from_value(expected_json["typed_history"].clone())?;
    m6_check_disk_history(world, &destination, &expected_wire, "wall color child destination")?;
    m6_check_disk_history(world, &source, &expected_wire, "wall color child")?;
    m6_check_parent_history(&expected_json["history"], &actual, "wall color")?;
    if let Some((path, expected, actual)) = m6_first_json_difference(&expected_json["history"], &actual) {
        eprintln!("reloaded raw color history differs at {path}: expected={expected}, actual={actual}; entries {} vs {}; cursor {} vs {cursor}",
            expected_json["history"].as_array().map_or(0, Vec::len), history.ledger().entries.len(), history.ledger().cursor);
    }
    anyhow::ensure!(history.ledger().cursor == cursor,
        "reloaded color history cursor differs: actual={}, expected={cursor}", history.ledger().cursor);
    anyhow::ensure!(m6_color_authority_map(world)? == expected_json["after_authority"], "reloaded wall authority differs");
    let before: u32 = serde_json::from_value(expected_json["before_color"].clone())?;
    driver.app_mut().insert_resource(M6ColorInput { cursor: glam::Vec2::ZERO, click: Phase::Idle, undo: Phase::Idle, redo: Phase::Idle });
    driver.app_mut().add_systems(bevy_app::Update, m6_color_pointer_input
        .after(country_core::systems::backend::input::update_cursor_position::update_cursor_position)
        .before(hover_intersection).before(camera_mouse_ray)
        .before(mode_manager::mode_manager::<EditType, MaybePendingSnapshotId>));
    m6_request_mode(&mut driver, &context, &mut bridge, AppMode::Color)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().undo = Phase::JustPressed;
    run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().undo = Phase::Idle;
    m6_wait_rendered_replay_completion(&mut driver, &context, &mut bridge, cursor - 1, "reloaded wall color undo")?;
    anyhow::ensure!(driver.app().world().resource::<WallColorIds>().0[wall] == before
        && m6_color_authority_map(driver.app().world())? == expected_json["before_authority"], "reloaded wall color undo failed to restore authority");
    m6_assert_wall_color_gpu(driver.app().world(), &bridge, &device)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().redo = Phase::JustPressed;
    run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().redo = Phase::Idle;
    m6_wait_rendered_replay_completion(&mut driver, &context, &mut bridge, cursor, "reloaded wall color redo")?;
    anyhow::ensure!(driver.app().world().resource::<WallColorIds>().0[wall] == expected
        && m6_color_authority_map(driver.app().world())? == expected_json["after_authority"], "reloaded wall color redo failed to restore authority");
    m6_assert_wall_color_gpu(driver.app().world(), &bridge, &device)?;
    Ok(())
}

/// Find the center of a straight face using the actual wall collider, away
/// from corners and the roof. The rectangle itself is a command fixture.
fn m06_delete_central_window_cursor(world: &bevy_ecs::world::World,
    wall: country_core::resources::walls::WallId, center: glam::Vec2) -> Result<glam::Vec2> {
    use country_core::{resources::window::WindowSize, systems::{camera_rig::MainCamera, collision::world::RaycastWorld}};
    let camera = world.resource::<MainCamera>();
    let window = world.resource::<WindowSize>();
    let collision = world.resource::<RaycastWorld>();
    let mut found: Option<(f32, glam::Vec2)> = None;
    for y in (8..window.height.saturating_sub(8)).step_by(3) {
        for x in (8..window.width.saturating_sub(8)).step_by(3) {
            let point = glam::Vec2::new(x as f32, y as f32);
            let ray = utils::geometry::Ray::from_mouse_ss(point, window.as_vec2(), camera.read_matrix(0x40), camera.read_matrix(0xc0));
            let hit = collision.raycast(ray.origin, ray.direction, 0x30ab | 0x100);
            if hit.is_hit() && hit.handle[0] as u8 == 0 && hit.handle[1] == wall.0 {
                let relative = hit.position - glam::Vec3::new(center.x, 0.0, center.y);
                let along = if hit.normal.x.abs() > hit.normal.z.abs() { relative.z } else { relative.x };
                if along.abs() < 0.8 && relative.y > 2.0 && relative.y < 4.0 {
                    let score = along.abs() + (relative.y - 3.0).abs();
                    if found.as_ref().is_none_or(|(best, _)| score < *best) { found = Some((score, point)); }
                }
            }
        }
    }
    found.map(|(_, point)| point).context("delete fixture has no visible central window placement ray")
}

// Real wall picker, decorator lifecycle, history and saved-session reload.
#[test]
fn m06_wall_bulldozer_decorator_lifecycle_save_reload() -> Result<()> {
    use bevy_ecs::schedule::IntoScheduleConfigs;
    use country_core::{input::action_set::DigitalActionPhase as Phase,
        resources::{history::{History, MaybePendingSnapshotId}, mouse_terrain_raycast::camera_mouse_ray,
            walls::{WallId, decorator::DecoratorId, decorator_storage::{DecoratorStorage, DecoratorStorageAddr}}, window::WindowSize},
        systems::{camera_rig::MainCamera, collision::world::RaycastWorld,
            mode_manager::{self, AppMode, EraserLayer}, ui::hover_intersection::hover_intersection}};
    if std::env::var_os("TG_M6_DELETE_CHILD").is_some() { return m06_delete_reload_child(); }
    let BootedSession { mut driver, context, mut bridge, shaders, device, _boot } = boot_shipped_session()?;
    tiny_glade::production_renderer::warm_up_gameplay(bridge.warmup_owners()?, driver.app_mut().world_mut())?;
    bridge.accept_gameplay_warmup(driver.app().world().resource::<tiny_glade::render_bridge::CompletedGameplayWarmup>())?;
    for _ in 0..8 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    m6_dismiss_welcome(&mut driver, &context, &mut bridge)?;
    m6_settle_history(&mut driver, &context, &mut bridge)?;
    let old: std::collections::HashSet<_> = driver.app().world().resource::<InnerWalls>().wall_ids().collect();
    // Rectangle creation is an explicit production-command fixture, not a drawn-input claim.
    // Keep its entire bounding circle away from every existing wall segment.
    let center = {
        let walls = driver.app().world().resource::<InnerWalls>();
        let mut found = None;
        for z in [0.0f32, -6.0, 6.0, -12.0, 12.0] {
            for x in [0.0f32, -6.0, 6.0, -12.0, 12.0] {
                let p = glam::Vec2::new(x, z);
                let clear = walls.wall_ids().all(|id| {
                    let points = &walls.get_unwrapped(id).source().curve.points;
                    points.windows(2).all(|pair| {
                        let a = glam::Vec2::from_array(pair[0]); let b = glam::Vec2::from_array(pair[1]);
                        let ab = b - a;
                        let t = if ab.length_squared() > 0.0 { ((p - a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0) } else { 0.0 };
                        p.distance(a + ab * t) > 7.0
                    })
                });
                if clear { found = Some(p); break; }
            }
            if found.is_some() { break; }
        }
        found.context("no non-overlapping rectangle fixture site")?
    };
    driver.app_mut().world_mut().resource_mut::<History>().start_input_recording(0)
        .map_err(|e| anyhow::anyhow!("setup recording: {e:?}"))?;
    driver.app_mut().world_mut().send_event(country_core::systems::wall::shapes::CreateRectangleCmd {
        center, dimensions: glam::Vec2::new(6., 4.), orientation: glam::Vec2::X,
        previous_wall: None, height: 4., elevation: 1.5,
    });
    for _ in 0..20 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    let wall = driver.app().world().resource::<InnerWalls>().wall_ids().find(|id| !old.contains(id)).context("rectangle missing")?;
    if driver.app().world().resource::<History>().is_recording() {
        driver.app_mut().world_mut().send_event(country_core::resources::history::FinishInputRecordingCmd);
    }
    for _ in 0..3 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    let find_wall = |world: &bevy_ecs::world::World, mask: u32| -> Option<glam::Vec2> {
        let camera = world.resource::<MainCamera>(); let window = world.resource::<WindowSize>();
        let physics = world.resource::<RaycastWorld>();
        for y in (8..window.height.saturating_sub(8)).step_by(3) {
            for x in (8..window.width.saturating_sub(8)).step_by(3) {
                let point = glam::Vec2::new(x as f32, y as f32);
                let ray = utils::geometry::Ray::from_mouse_ss(point, window.as_vec2(), camera.read_matrix(0x40), camera.read_matrix(0xc0));
                let hit = physics.raycast(ray.origin, ray.direction, mask);
                if hit.is_hit() && hit.handle[0] as u8 == 0 && hit.handle[1] == wall.0 { return Some(point); }
            }
        }
        None
    };
    let cursor = m06_delete_central_window_cursor(driver.app().world(), wall, center)?;
    m6_request_mode(&mut driver, &context, &mut bridge, AppMode::Decorator(country_core::systems::mode_manager::DecoratorType::CottageWindow))?;
    driver.app_mut().insert_resource(M6ColorInput { cursor, click: Phase::Idle, undo: Phase::Idle, redo: Phase::Idle });
    driver.app_mut().add_systems(bevy_app::Update, m6_color_pointer_input
        .after(country_core::systems::backend::input::update_cursor_position::update_cursor_position)
        .before(hover_intersection).before(camera_mouse_ray)
        .before(mode_manager::mode_manager::<country_core::resources::history::EditType, MaybePendingSnapshotId>));
    let mut placement_active = false;
    let placement_widget = country_core::systems::ui::resources::ui_state::WidgetId::from_name("place_decorator");
    let placement_id = driver.app().world().resource::<country_core::resources::walls::decorator::DecoratorIdGen>().peek();
    let placement_history_before = driver.app().world().resource::<History>().ledger().entries.len();
    driver.app_mut().insert_resource(M6PlacementObservation { wall: wall.0, ..Default::default() });
    driver.app_mut().add_systems(bevy_app::Update, m6_observe_placement
        .after(country_core::systems::GameSystemSet(25)).before(country_core::systems::GameSystemSet(26)));
    for _ in 0..60 {
        run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
        let world = driver.app().world();
        let ui = world.resource::<country_core::systems::ui::resources::ui_state::UiState>();
        let dispatcher = world.resource::<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>();
        if ui.widget_context(placement_widget).is_active() && dispatcher.place_decorator().is_some_and(|p|
            p.widget == placement_widget && p.hit.handle[0] as u8 == 0 && p.hit.handle[1] == wall.0) {
            placement_active = true; break;
        }
    }
    anyhow::ensure!(placement_active, "window placement widget did not become active");
    let mut placement_info = None;
    for phase in [Phase::Idle, Phase::JustPressed, Phase::JustReleased, Phase::Idle] {
        driver.app_mut().world_mut().resource_mut::<M6ColorInput>().click = phase;
        run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
        if phase == Phase::JustPressed {
            let world = driver.app().world();
            let ui = world.resource::<country_core::systems::ui::resources::ui_state::UiState>();
            anyhow::ensure!(ui.widget_context(placement_widget).is_active() && ui.widget_context(placement_widget).dragged() == 3,
                "window placement did not capture the real widget");
            anyhow::ensure!(world.resource::<M6PlacementObservation>().preview.iter().any(|(id, info)|
                *id == placement_id.0 && info.ty == b'8' && info.state() == 1),
                "central wall placement did not produce an accepted cottage-window preview");
        }
        if phase == Phase::JustReleased {
            let world = driver.app().world();
            placement_info = world.resource::<DecoratorStorage>().lookup_decorator_info(placement_id).copied();
            anyhow::ensure!(placement_info.is_some(), "window release did not commit complete info");
        }
        if phase == Phase::Idle && placement_info.is_some() {
            anyhow::ensure!(driver.app().world().resource::<DecoratorStorage>().lookup_decorator_info(placement_id) == placement_info.as_ref(),
                "committed window changed on next frame");
        }
    }
    for _ in 0..20 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    let addr = DecoratorStorageAddr::Wall(wall);
    let (id, info) = {
        let bucket = driver.app().world().resource::<DecoratorStorage>().get(addr).context("window bucket missing")?;
        let (&id, &info) = bucket.decorators.iter().find(|(_, info)| info.ty == b'8' && info.state() == 0).context("window missing")?;
        (DecoratorId(id), info)
    };
    {
        use country_core::resources::history::EditType;
        use country_core::resources::walls::decorator::DecoratorIdGen;
        let world = driver.app().world();
        let storage = world.resource::<DecoratorStorage>();
        anyhow::ensure!(id == placement_id && Some(info) == placement_info, "committed window ID or full info differs from release");
        anyhow::ensure!(world.resource::<DecoratorIdGen>().peek().0 == id.0.wrapping_add(1), "placement did not advance ID once");
        anyhow::ensure!(storage.lookup_storage(id) == Some(addr), "committed window d2s address differs");
        anyhow::ensure!(world.resource::<History>().ledger().entries[placement_history_before..].iter().any(|entry|
            entry.edits.iter().any(|edit| matches!(edit, EditType::Decorator(_) | EditType::DecoratorV5(_)))),
            "placement did not commit decorator history");
    }
    m06_assert_deleted_decorator(driver.app().world(), id, false)?;
    let original_colliders = m06_decorator_collider_instances(driver.app().world(), id)?;
    let original_walls = m6_authority_map(driver.app().world());
    let original_wall_entity = driver.app().world().resource::<country_core::resources::walls::public_walls::WallEntities>()
        .wall_entity(wall).context("original wall entity missing")?;
    let cursor = find_wall(driver.app().world(), 0x77fb).context("no real bulldozer wall ray")?;
    m6_request_mode(&mut driver, &context, &mut bridge, AppMode::Eraser(EraserLayer::Bulldoze))?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().cursor = cursor;
    let widget = country_core::systems::ui::resources::ui_state::WidgetId::from_name(&format!("bulldoze_wall_{wall}"));
    let mut wall_active = false;
    for _ in 0..60 {
        run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
        let ui = driver.app().world().resource::<country_core::systems::ui::resources::ui_state::UiState>();
        if ui.widget(widget).is_some() && ui.widget_context(widget).is_active() { wall_active = true; break; }
    }
    anyhow::ensure!(wall_active, "target wall bulldozer widget did not become active");
    let before_cursor = driver.app().world().resource::<History>().ledger().cursor;
    for phase in [Phase::Idle, Phase::JustPressed, Phase::JustReleased, Phase::Idle] {
        driver.app_mut().world_mut().resource_mut::<M6ColorInput>().click = phase;
        run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    }
    m06_wait_wall_deleted(&mut driver, &context, &mut bridge, wall)?;
    m06_assert_deleted_decorator(driver.app().world(), id, true)?;
    m06_assert_colliders_retired(driver.app().world(), &original_colliders)?;
    anyhow::ensure!(!driver.app().world().resource::<InnerWalls>().contains(wall), "deleted wall remains in authority");
    m6_assert_removed_owners(driver.app().world(), wall)?;
    m06_assert_common_constructor_owners_retired(driver.app().world(), wall)?;
    anyhow::ensure!(!driver.app().world().entities().contains(original_wall_entity), "original wall entity survives deletion");
    let committed = driver.app().world().resource::<History>().ledger().cursor;
    anyhow::ensure!(committed > before_cursor && driver.app().world().resource::<History>().ledger().entries[committed as usize]
        .edits.iter().any(|edit| matches!(edit, country_core::resources::history::EditType::Wall(compound)
            if compound.groups.iter().flatten().any(|op| matches!(op, country_core::resources::history::wall_edit::WallHistoryEdit::Delete { wall: id, .. } if id.0 == wall.0)))),
        "bulldozer did not commit target wall deletion history");
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().undo = Phase::JustPressed;
    run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().undo = Phase::Idle;
    m6_wait_rendered_replay_completion(&mut driver, &context, &mut bridge, committed - 1, "delete undo")?;
    for _ in 0..20 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    anyhow::ensure!(driver.app().world().resource::<DecoratorStorage>().lookup_decorator_info(id) == Some(&info), "undo did not restore full decorator info");
    m06_assert_deleted_decorator(driver.app().world(), id, false)?;
    anyhow::ensure!(m6_authority_map(driver.app().world()) == original_walls, "undo did not restore wall authority and roof");
    let restored_wall_entity = driver.app().world().resource::<country_core::resources::walls::public_walls::WallEntities>()
        .wall_entity(wall).context("undo wall publication missing")?;
    let restored_decorator_entities = driver.app().world().resource::<system_decorator::decorator_visual_state::DecoratorVisualState>()
        .persistent_entities_of(id.0).context("undo decorator entities missing")?.to_vec();
    let restored_colliders = m06_decorator_collider_instances(driver.app().world(), id)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().redo = Phase::JustPressed;
    run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().redo = Phase::Idle;
    m6_wait_rendered_replay_completion(&mut driver, &context, &mut bridge, committed, "delete redo")?;
    m06_wait_wall_deleted(&mut driver, &context, &mut bridge, wall)?;
    m06_assert_deleted_decorator(driver.app().world(), id, true)?;
    m06_assert_colliders_retired(driver.app().world(), &restored_colliders)?;
    anyhow::ensure!(!driver.app().world().resource::<InnerWalls>().contains(wall), "redo wall remains in authority");
    m6_assert_removed_owners(driver.app().world(), wall)?;
    m06_assert_common_constructor_owners_retired(driver.app().world(), wall)?;
    anyhow::ensure!(!driver.app().world().entities().contains(restored_wall_entity), "redo left restored wall entity");
    m6_settle_history(&mut driver, &context, &mut bridge)?;
    let session = driver.app().world().resource::<ActiveSession>().id;
    let folder = session.save_folder_path();
    let expected_walls = serde_json::to_string(&m6_authority_map(driver.app().world()))?;
    let expected_info = serde_json::to_string(&country_core::resources::history::decorator_edit::DecoratorHistoryState::from_info(&info))?;
    let ledger = driver.app().world().resource::<History>().ledger();
    let settled_entries: Vec<_> = ledger.entries.iter().cloned()
        .map(country_core::resources::history::serialized::SavedMultiEdit).collect();
    let settled_wire = m6_history_wire(&settled_entries)?;
    let settled_cursor = ledger.cursor;
    let settled_history_start = ledger.undo_anchor;
    driver.app_mut().world_mut().send_event(country_core::systems::save::save_game::SaveGameCmd);
    let mut saved = false;
    for _ in 0..600 {
        run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
        if driver.app_mut().world_mut().resource_mut::<bevy_ecs::event::Events<country_core::systems::save::save_game::SavingGameComplete>>().drain().any(|e| e.path == folder) { saved = true; break; }
    }
    anyhow::ensure!(saved, "delete save did not complete");
    m6_check_disk_history(driver.app().world(), &folder, &settled_wire, "delete parent")?;
    anyhow::ensure!(driver.app().world().resource::<History>().ledger().cursor == settled_cursor
        && driver.app().world().resource::<History>().ledger().undo_anchor == settled_history_start,
        "delete parent cursor or history_start changed during save");
    driver.app_mut().init_resource::<M06DeleteReloadTransitions>();
    driver.app_mut().add_systems(bevy_state::state::OnExit(country_core::app_loading_state::AppLoadingState::Game),
        |mut counts: bevy_ecs::prelude::ResMut<M06DeleteReloadTransitions>| counts.exits += 1);
    driver.app_mut().add_systems(bevy_state::state::OnEnter(country_core::app_loading_state::AppLoadingState::Game),
        |mut counts: bevy_ecs::prelude::ResMut<M06DeleteReloadTransitions>| counts.enters += 1);
    m06_reload_saved_session(&mut driver, &context, &mut bridge, session)?;
    let transitions = driver.app().world().resource::<M06DeleteReloadTransitions>();
    anyhow::ensure!(transitions.exits == 1 && transitions.enters == 1, "same-App reload did not execute Game OnExit/OnEnter");
    for _ in 0..20 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    m06_assert_deleted_decorator(driver.app().world(), id, true)?;
    m6_check_disk_history(driver.app().world(), &folder, &settled_wire, "delete same-App reload")?;
    anyhow::ensure!(driver.app().world().resource::<History>().ledger().cursor == settled_cursor
        && driver.app().world().resource::<History>().ledger().undo_anchor == settled_history_start,
        "delete same-App reload cursor or history_start differs");
    anyhow::ensure!(!driver.app().world().resource::<InnerWalls>().contains(wall), "same-App reload restored deleted wall");
    m6_assert_removed_owners(driver.app().world(), wall)?;
    m06_assert_common_constructor_owners_retired(driver.app().world(), wall)?;
    anyhow::ensure!(!driver.app().world().entities().contains(restored_wall_entity)
        && restored_decorator_entities.iter().all(|entity| !driver.app().world().entities().contains(*entity)),
        "same-App reload retained old entity generations");
    let exchange = tempfile::tempdir()?;
    std::fs::write(exchange.path().join("original-walls.json"), serde_json::to_vec(&original_walls)?)?;
    std::fs::write(exchange.path().join("history.json"), serde_json::to_vec(&(settled_wire, settled_cursor, settled_history_start))?)?;
    std::fs::write(exchange.path().join("walls.json"), expected_walls)?;
    std::fs::write(exchange.path().join("info.json"), expected_info)?;
    drop(driver); drop(bridge); drop(shaders); drop(device);
    let status = std::process::Command::new(std::env::current_exe()?)
        .args(["--exact", "m06_wall_bulldozer_decorator_lifecycle_save_reload", "--nocapture", "--test-threads=1"])
        .env("TG_M6_DELETE_CHILD", "1").env("TG_M6_DELETE_FOLDER", folder)
        .env("TG_M6_DELETE_SESSION", serde_json::to_string(&session)?).env("TG_M6_DELETE_ID", id.0.to_string())
        .env("TG_M6_DELETE_WALL", wall.0.to_string())
        .env("TG_M6_DELETE_EXCHANGE", exchange.path()).status()?;
    anyhow::ensure!(status.success(), "delete reload child failed: {status}");
    Ok(())
}

#[derive(bevy_ecs::prelude::Resource, Default)]
struct M06DeleteReloadTransitions { exits: usize, enters: usize }

fn m06_wait_wall_deleted(driver: &mut tiny_glade::game_loop::GameFrameDriver, context: &egui::Context,
    bridge: &mut CoreRenderBridge, wall: country_core::resources::walls::WallId) -> Result<()> {
    use country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer;
    for _ in 0..600 {
        run_m2_production_frame(driver, context, bridge, None)?;
        let world = driver.app().world();
        if !world.resource::<InnerWalls>().contains(wall)
            && !world.resource::<AnimateWallBulldozeBuffer>().contains(wall)
            && m6_assert_removed_owners(world, wall).is_ok() {
            m6_settle_history(driver, context, bridge)?;
            anyhow::ensure!(!driver.app().world().resource::<InnerWalls>().contains(wall)
                && !driver.app().world().resource::<AnimateWallBulldozeBuffer>().contains(wall), "wall deletion restarted");
            return Ok(());
        }
    }
    anyhow::bail!("wall deletion animation/authority/owners did not finish")
}

fn m06_decorator_collider_instances(world: &bevy_ecs::world::World, id: country_core::resources::walls::decorator::DecoratorId)
    -> Result<[country_core::systems::collision::world::ColliderMeshInstance; 2]> {
    let owners = world.resource::<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
    Ok([*owners.first.get(&id).context("physics collider missing")?,
        *owners.second.get(&id).context("interaction collider missing")?])
}

fn m06_assert_colliders_retired(world: &bevy_ecs::world::World,
    old: &[country_core::systems::collision::world::ColliderMeshInstance; 2]) -> Result<()> {
    let physics = world.resource::<country_core::systems::collision::world::RaycastWorld>();
    for instance in old {
        anyhow::ensure!(physics.get_rapier_collider_ref(instance.slot, instance.collider).is_none(),
            "old Rapier collider handle still resolves: {:?}", instance.handle());
    }
    Ok(())
}

/// Check the current common constructor sidecar independently of the support
/// owner; this does not assert its unresolved native +0x120 ownership mapping.
fn m06_assert_common_constructor_owners_retired(world: &bevy_ecs::world::World,
    wall: country_core::resources::walls::WallId) -> Result<()> {
    use system_wall_constructor::{construct_arches::WallConstructorOutput, gpu_output::{ConstructorKey, ConstructorPart}};
    let gpu = &world.resource::<WallConstructorOutput>().gpu;
    let key = ConstructorKey { wall: wall.0, part: ConstructorPart::Arches };
    anyhow::ensure!(gpu.brick_allocation(key).is_none() && gpu.plank_allocation(key).is_none(),
        "deleted wall retains common constructor brick/plank owners");
    Ok(())
}

fn m06_assert_deleted_decorator(world: &bevy_ecs::world::World, id: country_core::resources::walls::decorator::DecoratorId, deleted: bool) -> Result<()> {
    use country_core::resources::walls::{decorator_collision::DecoratorPhysicsColliders, decorator_storage::DecoratorStorage};
    let storage = world.resource::<DecoratorStorage>();
    let blueprints = world.resource::<system_decorator::blueprint::DecoratorBlueprints>();
    let physics = world.resource::<DecoratorPhysicsColliders>();
    let visual = world.resource::<system_decorator::decorator_visual_state::DecoratorVisualState>();
    anyhow::ensure!(storage.lookup_decorator_info(id).is_none() == deleted, "decorator authority mismatch");
    anyhow::ensure!(blueprints.get(id.0).is_none() == deleted, "decorator blueprint mismatch");
    // generate_cottage_wall_windows supplies both mesh slots via set_positioned_colliders.
    anyhow::ensure!(physics.first.contains_key(&id) != deleted && physics.second.contains_key(&id) != deleted, "decorator collider mismatch");
    anyhow::ensure!(visual.persistent_entities_of(id.0).is_none() == deleted, "decorator visual mismatch");
    if let Some(entities) = visual.persistent_entities_of(id.0) {
        anyhow::ensure!(!entities.is_empty() && entities.iter().all(|entity| world.entities().contains(*entity)), "decorator persistent entities missing");
    }
    Ok(())
}

fn m06_reload_saved_session(driver: &mut tiny_glade::game_loop::GameFrameDriver, context: &egui::Context,
    bridge: &mut CoreRenderBridge, session: country_core::resources::session::SessionId) -> Result<()> {
    use country_core::{meta_commander::{MetaCommand, MetaCommander}, resources::history::ReplayBuffer,
        systems::{history::PendingHistoryCommands, mode_manager::AppMode}};
    driver.app_mut().world_mut().resource_mut::<MetaCommander>().extend_script([MetaCommand::load_session(session, false)]);
    let mut saw_reload = false;
    for _ in 0..1200 {
        context.begin_frame(egui::RawInput::default()); driver.app_mut().update(); let _ = context.end_frame();
        bridge.prepare_test_execution()?;
        let world = driver.app().world();
        let state = world.resource::<bevy_state::prelude::State<country_core::app_loading_state::AppLoadingState>>().get();
        saw_reload |= *state == country_core::app_loading_state::AppLoadingState::Reload;
        if saw_reload && *state == country_core::app_loading_state::AppLoadingState::Game
            && world.get_resource::<ActiveSession>().is_some_and(|s| s.id == session)
            && world.resource::<LoadGameManager>().options.is_none() && world.resource::<ReplayBuffer>().is_empty()
            && world.resource::<PendingHistoryCommands>().0.is_empty() && world.resource::<AppMode>().native_tag() != 13 { return Ok(()); }
    }
    anyhow::bail!("delete session reload incomplete")
}

fn m06_delete_reload_child() -> Result<()> {
    use country_core::{meta_commander::{MetaCommand, MetaCommander}, resources::{history::{History, ReplayBuffer}, session::SessionId,
        walls::decorator::DecoratorId}, systems::{history::PendingHistoryCommands, mode_manager::AppMode}};
    let session: SessionId = serde_json::from_str(&std::env::var("TG_M6_DELETE_SESSION")?)?;
    let id = DecoratorId(std::env::var("TG_M6_DELETE_ID")?.parse()?);
    let wall = country_core::resources::walls::WallId(std::env::var("TG_M6_DELETE_WALL")?.parse()?);
    let source = PathBuf::from(std::env::var_os("TG_M6_DELETE_FOLDER").context("missing delete save")?);
    let exchange = PathBuf::from(std::env::var_os("TG_M6_DELETE_EXCHANGE").context("missing delete exchange directory")?);
    let BootedSession { mut driver, context, mut bridge, .. } = boot_shipped_session()?;
    tiny_glade::production_renderer::warm_up_gameplay(bridge.warmup_owners()?, driver.app_mut().world_mut())?;
    bridge.accept_gameplay_warmup(driver.app().world().resource::<tiny_glade::render_bridge::CompletedGameplayWarmup>())?;
    copy_test_tree(&source, &session.save_folder_path())?;
    driver.app_mut().world_mut().resource_mut::<MetaCommander>().extend_script([MetaCommand::load_session(session, false)]);
    let mut saw_reload = false; let mut loaded = false;
    for _ in 0..1200 {
        context.begin_frame(egui::RawInput::default()); driver.app_mut().update(); let _ = context.end_frame();
        bridge.prepare_test_execution()?;
        let world = driver.app().world();
        let state = world.resource::<bevy_state::prelude::State<country_core::app_loading_state::AppLoadingState>>().get();
        saw_reload |= *state == country_core::app_loading_state::AppLoadingState::Reload;
        if saw_reload && *state == country_core::app_loading_state::AppLoadingState::Game
            && world.get_resource::<ActiveSession>().is_some_and(|s| s.id == session)
            && world.resource::<LoadGameManager>().options.is_none() && world.resource::<ReplayBuffer>().is_empty()
            && world.resource::<PendingHistoryCommands>().0.is_empty() && world.resource::<AppMode>().native_tag() != 13 { loaded = true; break; }
    }
    anyhow::ensure!(loaded, "delete session reload incomplete");
    for _ in 0..20 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    m6_settle_history(&mut driver, &context, &mut bridge)?;
    m06_assert_deleted_decorator(driver.app().world(), id, true)?;
    anyhow::ensure!(!driver.app().world().resource::<InnerWalls>().contains(wall), "reloaded deleted wall remains in authority");
    m6_assert_removed_owners(driver.app().world(), wall)?;
    m06_assert_common_constructor_owners_retired(driver.app().world(), wall)?;
    let (settled_wire, settled_cursor, settled_history_start): (Vec<String>, i32, i32) =
        serde_json::from_slice(&std::fs::read(exchange.join("history.json"))?)?;
    m6_check_disk_history(driver.app().world(), &source, &settled_wire, "delete child")?;
    anyhow::ensure!(driver.app().world().resource::<History>().ledger().cursor == settled_cursor
        && driver.app().world().resource::<History>().ledger().undo_anchor == settled_history_start,
        "delete child cursor or history_start differs from parent");
    let walls: serde_json::Value = serde_json::from_slice(&std::fs::read(exchange.join("walls.json"))?)?;
    anyhow::ensure!(m6_authority_map(driver.app().world()) == walls, "reloaded wall authority differs");
    use bevy_ecs::schedule::IntoScheduleConfigs;
    use country_core::{input::action_set::DigitalActionPhase as Phase,
        resources::{history::MaybePendingSnapshotId, mouse_terrain_raycast::camera_mouse_ray},
        systems::{mode_manager, ui::hover_intersection::hover_intersection}};
    driver.app_mut().insert_resource(M6ColorInput { cursor: glam::Vec2::ZERO, click: Phase::Idle, undo: Phase::Idle, redo: Phase::Idle });
    driver.app_mut().add_systems(bevy_app::Update, m6_color_pointer_input
        .after(country_core::systems::backend::input::update_cursor_position::update_cursor_position)
        .before(hover_intersection).before(camera_mouse_ray)
        .before(mode_manager::mode_manager::<country_core::resources::history::EditType, MaybePendingSnapshotId>));
    let cursor = driver.app().world().resource::<History>().ledger().cursor;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().undo = Phase::JustPressed;
    run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().undo = Phase::Idle;
    m6_wait_rendered_replay_completion(&mut driver, &context, &mut bridge, cursor - 1, "reload delete undo")?;
    for _ in 0..20 { run_m2_production_frame(&mut driver, &context, &mut bridge, None)?; }
    let expected: country_core::resources::history::decorator_edit::DecoratorHistoryState = serde_json::from_slice(&std::fs::read(exchange.join("info.json"))?)?;
    let restored = driver.app().world().resource::<country_core::resources::walls::decorator_storage::DecoratorStorage>()
        .lookup_decorator_info(id).copied().context("reloaded undo decorator missing")?;
    anyhow::ensure!(restored == expected.to_info(), "reloaded undo full info differs");
    m06_assert_deleted_decorator(driver.app().world(), id, false)?;
    anyhow::ensure!(driver.app().world().resource::<InnerWalls>().contains(wall), "reloaded undo wall missing");
    let original_walls: serde_json::Value = serde_json::from_slice(&std::fs::read(exchange.join("original-walls.json"))?)?;
    anyhow::ensure!(m6_authority_map(driver.app().world()) == original_walls, "reloaded undo wall authority or roof differs");
    let restored_colliders = m06_decorator_collider_instances(driver.app().world(), id)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().redo = Phase::JustPressed;
    run_m2_production_frame(&mut driver, &context, &mut bridge, None)?;
    driver.app_mut().world_mut().resource_mut::<M6ColorInput>().redo = Phase::Idle;
    m6_wait_rendered_replay_completion(&mut driver, &context, &mut bridge, cursor, "reload delete redo")?;
    m06_wait_wall_deleted(&mut driver, &context, &mut bridge, wall)?;
    m06_assert_deleted_decorator(driver.app().world(), id, true)?;
    m06_assert_colliders_retired(driver.app().world(), &restored_colliders)?;
    anyhow::ensure!(!driver.app().world().resource::<InnerWalls>().contains(wall), "reloaded redo wall remains in authority");
    m6_assert_removed_owners(driver.app().world(), wall)?;
    m06_assert_common_constructor_owners_retired(driver.app().world(), wall)?;
    anyhow::ensure!(m6_authority_map(driver.app().world()) == walls, "reloaded redo wall authority differs");
    Ok(())
}
