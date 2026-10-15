//! `draw_callbacks_a` 的单元测试（d_cb_a）。见 `draw_callbacks_a.rs` 模块文档里的「测试结构」。

use super::*;
use crate::asset_libraries::mesh_library::MeshAsset;
use crate::components::{BackgroundTerrain, DontReceiveContactShadows, OverwriteRoughness};
use crate::render::pass_builder::draw_builder::DrawTransform;
use crate::render::pass_builder::draw_list::DrawListFn;
use crate::render::pass_builder::draw_stream::DrawKind;
use crate::systems::render_loop::draw_passes::draw_shaders::test_support::{
    Call, Rig, as_f32, as_i32, rg, uniform_at,
};
use glam::Quat;
use xxhash_rust::xxh3::xxh3_64;

const NONE_MSG: &str = "called `Option::unwrap()` on a `None` value";

fn params(e: Entity) -> EntityDrawParams {
    EntityDrawParams {
        topology: None,
        entity: e,
        shader_name: "t",
        transform: DrawTransform {
            rotation: Quat::IDENTITY,
            translation: Vec3::ZERO,
            scale: Vec3::ONE,
        },
    }
}

/// 跑回调并取走已提交的 draw。
fn run(
    cb: DrawListFn,
    rig: &mut Rig,
    w: &mut World,
    p: &EntityDrawParams,
) -> Vec<crate::render::pass_builder::draw_stream::DrawPacket> {
    cb(rig.builder(), p, w);
    rig.finish()
}

/// 跑回调并期望 panic：返回 panic 文案；随后让 pass 通过 `render()` 检查（否则 `RasterPass` 的 Drop 会再 panic）。
fn run_panics(cb: DrawListFn, w: &mut World, p: &EntityDrawParams) -> String {
    let mut rig = Rig::new(&[], &[]);
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| cb(rig.builder(), p, w)));
    rig.finish();
    let err = r.expect_err("应该 panic");
    err.downcast_ref::<String>()
        .cloned()
        .or_else(|| err.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_default()
}

fn disk(k: u64) -> Call {
    Call::Disk(k)
}
fn preset(n: u8) -> Call {
    Call::Preset(n)
}

// ---------------------------------------------------------------- 常量 / 标签

#[test]
fn ssbo_ids_are_xxh3_of_the_slot_names() {
    fn chk<T: DrawSsbo>() {
        assert_eq!(
            T::rg_buffer_hash(),
            xxh3_64(T::SLOT_NAME.as_bytes()),
            "{}",
            T::TYPE_NAME
        );
    }
    chk::<GateColorIdTag>();
    chk::<HalftimberColorIdTag>();
    chk::<WoodenPlankTag>();
    chk::<BushTag>();
    chk::<FlowerTag>();
    chk::<SheepAnimationTag>();
}

#[test]
fn negative_movabs_constants_match_the_machine_code() {
    // `movabs` 的有符号立即数（O）。
    assert_eq!(E11_KEY_V4, -0x7acae4ed9d1550fc_i64 as u64);
    assert_eq!(E11_KEY_OTHER, -0x24dec67bf60cbc89_i64 as u64);
    assert_eq!(CLOTH_KEY_C_DEFAULT, -0x1adc8a43cd569b5c_i64 as u64);
    assert_eq!(E6_NORMAL_DEFAULT, -0x1bef39396fe244af_i64 as u64);
    assert_eq!(E27_MARKER_KEY, -0x42c6aaecc8d7a04a_i64 as u64);
    assert_eq!(NAME_ALBEDO, -0x4235cf06a19fb2f_i64 as u64);
    assert_eq!(COLOR_ID_UNIFORM, -0x40673b8019e26560_i64 as u64);
    assert_eq!(CLOTH_NAME_A, -0x5abac170cb1077b0_i64 as u64);
    assert_eq!(CLOTH_KEY_B, -0x48d0458e667a5a74_i64 as u64);
    assert_eq!(CLOTH_NAME_B, -0x7c49cd202b5caa74_i64 as u64);
    assert_eq!(CLOTH_NAME_C, -0x317b9b95d74077ed_i64 as u64);
    assert_eq!(E6_DISK4.1, -0x104665158732bf7_i64 as u64);
    assert_eq!(E6_NORMAL_NAME, -0x25cd68f8afdc0929_i64 as u64);
    assert_eq!(
        E45_DISKS[4].0, E27_MARKER_KEY,
        "Env45 的最后一张磁盘与 Env27 的标记键相同（O）"
    );
}

// ---------------------------------------------------------------- 取值助手

#[test]
fn helpers_follow_the_machine_code_panics() {
    let mut w = World::new();
    let e = w.spawn(SplashParticleView { f8: 1.0 }).id();
    assert_eq!(comp::<SplashParticleView>(&w, e).f8, 1.0);
    assert!(comp_opt::<AlbedoMapView>(&w, e).is_none());
    assert!(!has::<SemiTransparent>(&w, e));
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        comp::<AlbedoMapView>(&w, e);
    }));
    assert!(r.is_err(), "必需组件缺失 → Option::unwrap panic");
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        res::<WeatherView>(&w);
    }));
    assert!(r.is_err(), "资源缺失 → Option::unwrap panic");
}

#[test]
fn cull_state_matches_and_0xf3_or_v() {
    assert_eq!(cull_state(0xffff_ffff, 4), 0xffff_fff7);
    assert_eq!(cull_state(0x0000_0000, 8), 8);
    assert_eq!(cull_state(0xc, 0), 0);
}

// ---------------------------------------------------------------- 1..3 楼梯 / 墙砖 / 常春藤

#[test]
fn stairs_flag_comes_from_the_first_byte_of_the_component() {
    // idx = flag + 1；静态表 `[1] = 300`、`[2] = 96`（见 stairs_draw 模块文档）。
    for (flags, want) in [(0u8, 300u32), (1, 96), (2, 300), (3, 96)] {
        let mut w = World::new();
        let e = w.spawn(StairsStepView { flags }).id();
        let mut rig = Rig::new(&[], &[]);
        let d = run(stairs_draw_cb, &mut rig, &mut w, &params(e));
        assert_eq!(d.len(), 1);
        match &d[0].kind {
            DrawKind::IndexedIndirect {
                first_index_count, ..
            } => {
                assert_eq!(*first_index_count, Some(want), "flags = {flags}")
            }
            k => panic!("{k:?}"),
        }
    }
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn stairs_without_the_component_panics() {
    let mut w = World::new();
    let e = w.spawn_empty().id();
    let mut rig = Rig::new(&[], &[]);
    run(stairs_draw_cb, &mut rig, &mut w, &params(e));
}

#[test]
fn bricks_discard_happens_before_any_resource_read() {
    // LOD0：`b1 == 1` → discard；此时世界里没有 `AssetSsboLibrary`，不能因资源缺失而 panic。
    let mut w = World::new();
    let e = w
        .spawn(WallBrickInstancedDrawProxyView {
            b0: 0,
            b1: 1,
            b2: 0,
        })
        .id();
    let mut rig = Rig::new(&[], &[]);
    let d = run(bricks_lod0_cb, &mut rig, &mut w, &params(e));
    assert!(d.is_empty(), "discard 不提交 draw");
    // LOD1：`b1 != 1` 且 mode 位为 0 → 通过，随后需要 `AssetSsboLibrary` → Option::unwrap panic。
    let e = w
        .spawn(WallBrickInstancedDrawProxyView {
            b0: 0,
            b1: 0,
            b2: 0,
        })
        .id();
    assert!(run_panics(bricks_lod1_cb, &mut w, &params(e)).contains(NONE_MSG));
}

#[test]
fn bricks_without_the_proxy_panics() {
    let mut w = World::new();
    let e = w.spawn_empty().id();
    assert!(run_panics(bricks_lod0_cb, &mut w, &params(e)).contains(NONE_MSG));
}

#[test]
fn erase_highlight_reads_the_four_resources() {
    let mut w = World::new();
    w.insert_resource(EditingHighlightView {
        enabled: true,
        ..Default::default()
    });
    w.insert_resource(CursorTerrainRaycast {
        tag: 1,
        pos: Vec3::new(1.0, 2.0, 3.0),
    });
    w.insert_resource(EraserSizeMultiplier(2.5));
    w.insert_resource(DebugGuiView::default());
    let i = read_erase_highlight(&w);
    assert_eq!(
        i,
        BrickEraseHighlightInputs {
            enabled: true,
            cursor_position: Some(Vec3::new(1.0, 2.0, 3.0)),
            radius: 2.5,
            suppress: false,
        }
    );
    w.insert_resource(CursorTerrainRaycast {
        tag: 0,
        pos: Vec3::ONE,
    });
    assert_eq!(
        read_erase_highlight(&w).cursor_position,
        None,
        "tag bit0 == 0 → None"
    );
}

#[test]
fn ivy_segment_counts_come_from_the_storage() {
    let mut w = World::new();
    let s = IvyDrawView {
        point_counts: vec![3, 1, 0],
    };
    w.insert_resource(s);
    let mut c = ivy_segment_counts(&w);
    c.sort();
    assert_eq!(c, vec![0, 1, 3]);
}

#[test]
fn ivy_branch_draws_instances_summed_with_saturating_sub() {
    let mut w = World::new();
    let s = IvyDrawView {
        point_counts: vec![3, 1, 0, 5],
    };
    w.insert_resource(s);
    let e = w.spawn_empty().id();
    let mut rig = Rig::new(&[], &[]);
    let d = run(ivy_branch_cb, &mut rig, &mut w, &params(e));
    // (3-1) + (1-1) + 0 + (5-1) = 6
    assert!(
        matches!(d[0].kind, DrawKind::InstancedGlDrawFlag { flag: 6 }),
        "{:?}",
        d[0].kind
    );
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn ivy_leaf_needs_the_camera_resource() {
    let mut w = World::new();
    let e = w.spawn_empty().id();
    let mut rig = Rig::new(&[], &[]);
    run(ivy_leaf_cb, &mut rig, &mut w, &params(e));
}

// ---------------------------------------------------------------- 7 env6

#[test]
fn env6_binds_in_order_with_defaults_for_missing_optional_maps() {
    let mut w = World::new();
    let e = w.spawn(AlbedoMapView(0xaaaa)).id();
    let i = read_env6(&w, e);
    assert_eq!(
        i,
        Env6In {
            albedo: 0xaaaa,
            alpha: None,
            normal: None
        }
    );
    let mut rig = Rig::new(&[], &[E6_NORMAL_UNIFORM]);
    let mut data = Vec::new();
    env6_body(rig.builder(), &i, |b| {
        data = b.constants_data().unwrap().to_vec();
        b.draw();
    });
    assert_eq!(
        as_i32(&uniform_at(&data, 0, 4)),
        0,
        "normal.is_none() → false"
    );
    assert_eq!(
        rig.calls(),
        vec![
            disk(0xaaaa),
            disk(E6_ALPHA_DEFAULT),
            disk(E6_NORMAL_DEFAULT),
            disk(E6_DISK4.0),
            preset(0xf),
            preset(5),
            preset(0xe),
            preset(0x13),
            preset(9),
            preset(10),
        ]
    );
    assert_eq!(rig.finish().len(), 1);
}

#[test]
fn env6_uses_alpha_and_normal_keys_when_present() {
    let mut w = World::new();
    let e = w
        .spawn((AlbedoMapView(1), AlphaMapView(2), NormalMapView(3)))
        .id();
    let i = read_env6(&w, e);
    let mut rig = Rig::new(&[], &[E6_NORMAL_UNIFORM]);
    let mut data = Vec::new();
    env6_body(rig.builder(), &i, |b| {
        data = b.constants_data().unwrap().to_vec();
        b.draw();
    });
    assert_eq!(as_i32(&uniform_at(&data, 0, 4)), 1);
    assert_eq!(
        &rig.calls()[..4],
        &[disk(1), disk(2), disk(3), disk(E6_DISK4.0)]
    );
    rig.finish();
}

#[test]
fn env6_callback_requires_the_albedo_map() {
    let mut w = World::new();
    let e = w.spawn(AlphaMapView(2)).id();
    assert!(run_panics(env6, &mut w, &params(e)).contains(NONE_MSG));
    let e = w.spawn(AlbedoMapView(9)).id();
    let mut rig = Rig::new(&[], &[]);
    let d = run(env6, &mut rig, &mut w, &params(e));
    assert_eq!(d.len(), 1);
}

// ---------------------------------------------------------------- 8 env27

fn env27_unis() -> [u64; 4] {
    [E27_BOOL_A, E27_F32, E27_BOOL_SEMI, E27_BOOL_NO_A6C0]
}

#[test]
fn env27_reads_markers_and_defaults() {
    let mut w = World::new();
    let e = w.spawn((AlbedoMapView(7), SemiTransparent)).id();
    let i = read_env27(&w, e);
    assert_eq!(
        i,
        Env27In {
            semi_transparent: true,
            albedo: Some(7),
            background_terrain: false,
            overwrite_roughness: None,
            dont_receive_contact_shadows: false,
        }
    );
    let mut rig = Rig::new(&[], &env27_unis());
    let mut data = Vec::new();
    env27_body(rig.builder(), &i, |b| {
        data = b.constants_data().unwrap().to_vec();
        b.draw();
    });
    assert_eq!(rig.calls(), vec![disk(7)]);
    assert_eq!(as_i32(&uniform_at(&data, 0, 4)), 0);
    assert!(
        (as_f32(&uniform_at(&data, 1, 4)) - 0.8).abs() < 1e-7,
        "缺省 0.8（0x3f4ccccd）"
    );
    assert_eq!(as_i32(&uniform_at(&data, 2, 4)), 1);
    assert_eq!(as_i32(&uniform_at(&data, 3, 4)), 1, "a6c0 不存在 → true");
    rig.finish();
}

#[test]
fn env27_marker_overrides_the_albedo_key() {
    let mut w = World::new();
    let e = w
        .spawn((
            AlbedoMapView(7),
            BackgroundTerrain,
            OverwriteRoughness(0.25),
            DontReceiveContactShadows,
        ))
        .id();
    let i = read_env27(&w, e);
    let mut rig = Rig::new(&[], &env27_unis());
    let mut data = Vec::new();
    env27_body(rig.builder(), &i, |b| {
        data = b.constants_data().unwrap().to_vec();
        b.draw();
    });
    assert_eq!(rig.calls(), vec![disk(E27_MARKER_KEY)]);
    assert_eq!(as_i32(&uniform_at(&data, 0, 4)), 1);
    assert_eq!(as_f32(&uniform_at(&data, 1, 4)), 0.25);
    assert_eq!(as_i32(&uniform_at(&data, 2, 4)), 0);
    assert_eq!(as_i32(&uniform_at(&data, 3, 4)), 0);
    rig.finish();
    // 没有 albedo、没有标记 → 缺省键。
    let e = w.spawn_empty().id();
    let i = read_env27(&w, e);
    let mut rig = Rig::new(&[], &env27_unis());
    env27_body(rig.builder(), &i, |b| b.draw());
    assert_eq!(rig.calls(), vec![disk(E27_ALBEDO_DEFAULT)]);
    rig.finish();
}

// ---------------------------------------------------------------- 9 env2

#[test]
fn env2_uniforms_matrix_and_ssbo() {
    let sheep = SheepView {
        q28: 0x1_0000_0007,
        f58: 1.5,
        b60: 1,
    };
    let prev = PrevTransformView {
        rotation: Quat::from_rotation_y(0.5),
        translation: Vec3::new(1.0, 2.0, 3.0),
        scale: Vec3::new(2.0, 3.0, 4.0),
    };
    let mut w = World::new();
    let e = w.spawn((sheep, prev)).id();
    let i = read_env2(&w, e);
    assert_eq!(i, Env2In { sheep, prev });
    let mut rig = Rig::new(
        &[SHEEP_ANIMATION_BINDING],
        &[
            E2_I32_UNIFORM,
            E2_F32_UNIFORM,
            E2_BOOL_UNIFORM,
            E2_MAT4_UNIFORM,
        ],
    );
    let mut data = Vec::new();
    env2_body(rig.builder(), &i, |b| {
        data = b.constants_data().unwrap().to_vec();
        b.draw();
    });
    assert_eq!(as_i32(&uniform_at(&data, 0, 4)), 7, "qword 取低 32 位");
    assert_eq!(as_f32(&uniform_at(&data, 1, 4)), 1.5);
    assert_eq!(as_i32(&uniform_at(&data, 2, 4)), 1);
    let m = uniform_at(&data, 3, 64);
    let want = Mat4::from_scale_rotation_translation(prev.scale, prev.rotation, prev.translation);
    for (k, f) in want.to_cols_array().iter().enumerate() {
        assert_eq!(as_f32(&m[k * 4..]), *f, "mat4[{k}]");
    }
    assert_eq!(rig.calls(), vec![rg::<SheepAnimationTag>()]);
    rig.finish();
}

#[test]
fn env2_callback_requires_both_components() {
    let mut w = World::new();
    let e = w.spawn(SheepView::default()).id();
    assert!(
        run_panics(env2, &mut w, &params(e)).contains(NONE_MSG),
        "缺 PrevTransform → Option::unwrap panic"
    );
}

// ---------------------------------------------------------------- 10/20/13/26/31 纯绑定

#[test]
fn env48_and_env53_share_four_disks_but_differ_in_instance_count() {
    let keys: Vec<Call> = E48_DISKS.iter().map(|d| disk(d.0)).collect();
    for (cb, n) in [(env48 as DrawListFn, 0x1f4u32), (env53, 0x3c)] {
        let mut w = World::new();
        let e = w.spawn_empty().id();
        let mut rig = Rig::new(&[], &[]);
        let d = run(cb, &mut rig, &mut w, &params(e));
        assert_eq!(rig.calls(), keys);
        assert!(
            matches!(d[0].kind, DrawKind::InstancedGlDrawFlag { flag } if flag == n),
            "{:?}",
            d[0].kind
        );
    }
}

#[test]
fn env45_binds_five_disks_and_draws() {
    let mut w = World::new();
    let e = w.spawn_empty().id();
    let mut rig = Rig::new(&[], &[]);
    let d = run(env45, &mut rig, &mut w, &params(e));
    assert_eq!(
        rig.calls(),
        E45_DISKS.iter().map(|d| disk(d.0)).collect::<Vec<_>>()
    );
    assert_eq!(d.len(), 1);
}

#[test]
fn env20_and_env40_bind_presets() {
    let mut w = World::new();
    let e = w.spawn_empty().id();
    let mut rig = Rig::new(&[], &[]);
    run(env20, &mut rig, &mut w, &params(e));
    assert_eq!(rig.calls(), vec![preset(0x13)]);
    let mut rig = Rig::new(&[], &[]);
    let d = run(env40, &mut rig, &mut w, &params(e));
    assert_eq!(
        rig.calls(),
        vec![
            preset(0x13),
            preset(5),
            disk(0x84a037e697de0324),
            preset(0x15)
        ]
    );
    assert_eq!(d.len(), 1);
}

// ---------------------------------------------------------------- 11 env52

#[test]
fn env52_discards_without_events_and_draws_with_them() {
    let mut w = World::new();
    w.init_resource::<bevy_ecs::event::Events<ShowDemoBorderOverlay>>();
    let e = w.spawn_empty().id();
    assert!(!read_env52(&w));
    let mut rig = Rig::new(&[], &[]);
    let d = run(env52, &mut rig, &mut w, &params(e));
    assert!(d.is_empty(), "无事件 → discard");
    assert!(rig.calls().is_empty(), "discard 在任何绑定之前");
    w.send_event(ShowDemoBorderOverlay);
    assert!(read_env52(&w));
    let mut rig = Rig::new(&[], &[]);
    let d = run(env52, &mut rig, &mut w, &params(e));
    assert_eq!(d.len(), 1);
    assert_eq!(rig.calls(), vec![disk(E52_DISK.0), preset(9), preset(10)]);
}

#[test]
fn env52_missing_events_resource_panics() {
    let mut w = World::new();
    let e = w.spawn_empty().id();
    assert!(run_panics(env52, &mut w, &params(e)).contains(NONE_MSG));
}

// ---------------------------------------------------------------- 12 env57

#[test]
fn env57_reads_mesh_index_count_and_dashed_length() {
    let mut w = World::new();
    let lib = AssetMeshLibrary::new();
    let mut m = Mesh::new("m");
    m.indices = vec![0; 12];
    let h = lib.add(MeshAsset::new(m, None));
    w.insert_resource(lib);
    let e = w.spawn((h, DashedLineLengthView(0.75))).id();
    let i = read_env57(&w, e);
    assert_eq!(
        i,
        Env57In {
            index_count: 12,
            dashed_len: 0.75
        }
    );
    let mut rig = Rig::new(&[], &[E57_I32_UNIFORM, E57_F32_UNIFORM]);
    let mut data = Vec::new();
    env57_body(rig.builder(), &i, |b| {
        data = b.constants_data().unwrap().to_vec();
        b.draw();
    });
    assert_eq!(as_i32(&uniform_at(&data, 0, 4)), 12);
    assert_eq!(as_f32(&uniform_at(&data, 1, 4)), 0.75);
    rig.finish();
}

#[test]
fn env57_applies_the_alt_topology_before_reading_the_world() {
    let mut w = World::new();
    let lib = AssetMeshLibrary::new();
    let h = lib.add(MeshAsset::new(Mesh::new("m"), None));
    w.insert_resource(lib);
    let e = w.spawn((h, DashedLineLengthView(1.0))).id();
    let mut p = params(e);
    p.topology = Some(DrawTopology::Alt(0.5));
    let mut rig = Rig::new(&[], &[]);
    let d = run(env57, &mut rig, &mut w, &p);
    assert_eq!(d[0].blend, 0.5);
    assert_eq!(d[0].state.0 & 3, 2);
}

// ---------------------------------------------------------------- 14 env0 / 16 / 17

#[test]
fn env0_reads_resources_and_sets_cull_mode_4() {
    let mut w = World::new();
    w.insert_resource(EditingHighlightView {
        enabled: true,
        ..Default::default()
    });
    w.insert_resource(CursorTerrainRaycast {
        tag: 1,
        pos: Vec3::new(1.0, 2.0, 3.0),
    });
    w.insert_resource(EraserSizeMultiplier(0.5));
    let i = read_env0(&w);
    assert_eq!(
        i,
        Env0In {
            enabled: true,
            cursor: Some(Vec3::new(1.0, 2.0, 3.0)),
            radius: 0.5
        }
    );
    let mut rig = Rig::new(
        &[INSTANCE_SSBO_BINDING, HALFTIMBER_COLOR_BINDING],
        &[
            BRICK_ERASE_ENABLED_UNIFORM,
            BRICK_ERASE_CURSOR_UNIFORM,
            BRICK_ERASE_RADIUS_UNIFORM,
        ],
    );
    let mut data = Vec::new();
    let mut state = 0;
    env0_body(rig.builder(), &i, |b| {
        data = b.constants_data().unwrap().to_vec();
        state = b.packet().state.0;
        b.draw();
    });
    assert_eq!(state & 0x0c, 4, "byte0 = (b & 0xf3) | 4");
    assert_eq!(as_i32(&uniform_at(&data, 0, 4)), 1);
    let v = uniform_at(&data, 1, 12);
    assert_eq!(
        [as_f32(&v[0..]), as_f32(&v[4..]), as_f32(&v[8..])],
        [1.0, 2.0, 3.0]
    );
    assert_eq!(as_f32(&uniform_at(&data, 2, 4)), 0.5);
    assert_eq!(
        rig.calls(),
        vec![
            preset(9),
            preset(10),
            rg::<WoodenPlankTag>(),
            disk(E0_DISK_A.0),
            rg::<HalftimberColorIdTag>(),
            disk(DISK_DORMER.0),
            disk(E0_DISK_C.0),
        ]
    );
    rig.finish();
}

#[test]
fn env0_without_cursor_hit_writes_zero_vec3() {
    let i = Env0In {
        enabled: false,
        cursor: None,
        radius: 1.0,
    };
    let mut rig = Rig::new(
        &[],
        &[
            BRICK_ERASE_ENABLED_UNIFORM,
            BRICK_ERASE_CURSOR_UNIFORM,
            BRICK_ERASE_RADIUS_UNIFORM,
        ],
    );
    let mut data = Vec::new();
    env0_body(rig.builder(), &i, |b| {
        data = b.constants_data().unwrap().to_vec();
        b.draw();
    });
    assert_eq!(as_i32(&uniform_at(&data, 0, 4)), 0);
    assert_eq!(uniform_at(&data, 1, 12), vec![0u8; 12]);
    rig.finish();
}

#[test]
fn env16_roof_highlight_uniforms() {
    assert_eq!(
        roof_highlight_uniforms(&RoofHighlight { value: 9, tag: 1 }),
        (9, 1)
    );
    assert_eq!(
        roof_highlight_uniforms(&RoofHighlight { value: 9, tag: 0 }),
        (0, 0),
        "tag == 0 → 值为 0"
    );
}

#[test]
fn env16_and_env17_share_the_floor_binding_sequence() {
    let head = |first_uniform: bool| {
        let mut v = vec![
            disk(DISK_DORMER.0),
            rg::<DormerHoleTag>(),
            rg::<DormerHoleCountTag>(),
        ];
        // `bind_wall_color_customization`：四张磁盘 + 墙色 SSBO（未在此展开），见下面的前缀长度检查。
        let _ = first_uniform;
        v.truncate(3);
        v
    };
    let mut rig = Rig::new(&[], &[E16_I32_VALUE, E16_I32_TAG]);
    let mut state = 0;
    let mut data = Vec::new();
    env16_body(rig.builder(), &RoofHighlight { value: 5, tag: 1 }, |b| {
        state = b.packet().state.0;
        data = b.constants_data().unwrap().to_vec();
        b.draw();
    });
    let c16 = rig.calls();
    rig.finish();
    assert_eq!(state & 0x0c, 8, "byte0 = (b & 0xf3) | 8");
    assert_eq!(as_i32(&uniform_at(&data, 0, 4)), 5);
    assert_eq!(as_i32(&uniform_at(&data, 1, 4)), 1);
    assert_eq!(&c16[..3], &head(true)[..]);
    assert_eq!(c16.last(), Some(&disk(DISK_COMMON_B.0)));
    assert!(c16.contains(&rg::<RoofColorIdTag>()));
    assert!(c16.contains(&rg::<HalftimberColorIdTag>()));

    let mut rig = Rig::new(&[], &[]);
    let b = env17_binds(rig.builder());
    assert_eq!(b.packet().state.0 & 0x0c, 8);
    b.draw();
    let c17 = rig.calls();
    rig.finish();
    assert_eq!(c17, c16, "两者的绑定调用序列相同（env16 只多两个 uniform）");
}

// ---------------------------------------------------------------- 15 env37 / 17 env19 / 19 env41 / 28 env55

#[test]
fn env37_writes_logo_floats() {
    let mut rig = Rig::new(&[], &[E37_F32_A, E37_F32_B]);
    let mut data = Vec::new();
    env37_body(
        rig.builder(),
        &LogoAnimationView {
            f0: 0.25,
            f10: 0.75,
        },
        |b| {
            data = b.constants_data().unwrap().to_vec();
            b.draw();
        },
    );
    assert_eq!(rig.calls(), vec![disk(E37_DISK_A.0), disk(E37_DISK_B)]);
    assert_eq!(as_f32(&uniform_at(&data, 0, 4)), 0.25);
    assert_eq!(as_f32(&uniform_at(&data, 1, 4)), 0.75);
    rig.finish();
}

#[test]
fn env19_uniform_from_component() {
    let mut w = World::new();
    let e = w.spawn(SplashParticleView { f8: 3.5 }).id();
    let s = *comp::<SplashParticleView>(&w, e);
    let mut rig = Rig::new(&[], &[F32_3CC1_UNIFORM]);
    let mut data = Vec::new();
    env19_body(rig.builder(), &s, |b| {
        data = b.constants_data().unwrap().to_vec();
        b.draw();
    });
    assert_eq!(as_f32(&uniform_at(&data, 0, 4)), 3.5);
    rig.finish();
    // 回调：必需组件缺失 → panic。
    let e2 = w.spawn_empty().id();
    assert!(run_panics(env19, &mut w, &params(e2)).contains(NONE_MSG));
}

#[test]
fn env41_sets_byte3_field_and_writes_progress() {
    let mut rig = Rig::new(&[], &[E41_F32_A, E41_F32_B]);
    let mut data = Vec::new();
    let mut state = 0;
    env41_body(
        rig.builder(),
        &LoadingScreenProgressView { f0: 0.1, f4: 0.9 },
        |b| {
            data = b.constants_data().unwrap().to_vec();
            state = b.packet().state.0;
            b.draw();
        },
    );
    assert_eq!(
        state & 0x6000_0000,
        0x4000_0000,
        "byte3 = (b & 0x9f) | 0x40"
    );
    assert_eq!(rig.calls(), vec![disk(E41_DISK)]);
    assert_eq!(as_f32(&uniform_at(&data, 0, 4)), 0.1);
    assert_eq!(as_f32(&uniform_at(&data, 1, 4)), 0.9);
    rig.finish();
}

#[test]
fn env55_reads_the_arrow_gizmo_with_swapped_offsets() {
    let mut rig = Rig::new(&[], &[E55_F32_A, E55_F32_B]);
    let mut data = Vec::new();
    let mut state = 0;
    env55_body(rig.builder(), &ArrowGizmoView { f0: 1.0, f4: 2.0 }, |b| {
        data = b.constants_data().unwrap().to_vec();
        state = b.packet().state.0;
        b.draw();
    });
    assert_eq!(state & 0x6000_0000, 0x4000_0000);
    assert_eq!(
        as_f32(&uniform_at(&data, 0, 4)),
        2.0,
        "第一个 uniform 读 +4"
    );
    assert_eq!(
        as_f32(&uniform_at(&data, 1, 4)),
        1.0,
        "第二个 uniform 读 +0"
    );
    rig.finish();
}

// ---------------------------------------------------------------- 21/32 cloth

#[test]
fn cloth_third_key_scans_session_bytes_for_values_up_to_one() {
    assert_eq!(
        cloth_key_c(&[]),
        CLOTH_KEY_C_DEFAULT,
        "空序列：循环不进入，保持缺省"
    );
    assert_eq!(cloth_key_c(&[2, 3, 4]), CLOTH_KEY_C_DEFAULT);
    assert_eq!(cloth_key_c(&[2, 1, 4]), CLOTH_KEY_C_FOUND);
    assert_eq!(cloth_key_c(&[0]), CLOTH_KEY_C_FOUND);
}

#[test]
fn env23_roof_cloth_sequence() {
    let mut w = World::new();
    w.insert_resource(ActiveSessionView {
        variant: 1,
        glade_bytes: vec![5, 0, 9],
    });
    let e = w.spawn((RoofFlagClothView(0xdead), ColorId(42))).id();
    let i = read_env23(&w, e);
    let mut rig = Rig::new(&[], &[COLOR_ID_UNIFORM, CLOTH_ID_UNIFORM]);
    let mut data = Vec::new();
    env23_body(rig.builder(), &i, |b| {
        data = b.constants_data().unwrap().to_vec();
        b.draw();
    });
    assert_eq!(
        rig.calls(),
        vec![disk(E23_KEY_A), disk(CLOTH_KEY_B), disk(CLOTH_KEY_C_FOUND)]
    );
    assert_eq!(as_i32(&uniform_at(&data, 0, 4)), 42);
    assert_eq!(as_i32(&uniform_at(&data, 1, 4)), 0xdead);
    rig.finish();
}

#[test]
fn env22_wall_cloth_sequence() {
    let mut w = World::new();
    w.insert_resource(ActiveSessionView {
        variant: 1,
        glade_bytes: vec![5, 6],
    });
    let cloth = WallFlagClothView {
        id: -3,
        f4: 0.5,
        f8: 1.5,
        fc: 2.5,
        flag: true,
    };
    let e = w.spawn((cloth, ColorId(7))).id();
    let i = read_env22(&w, e);
    assert_eq!(i.cloth, cloth);
    let unis = [
        E22_F32_4,
        E22_F32_8,
        E22_F32_C,
        E22_BOOL_10,
        CLOTH_ID_UNIFORM,
        COLOR_ID_UNIFORM,
    ];
    let mut rig = Rig::new(&[], &unis);
    let mut data = Vec::new();
    env22_body(rig.builder(), &i, |b| {
        data = b.constants_data().unwrap().to_vec();
        b.draw();
    });
    assert_eq!(
        rig.calls(),
        vec![
            disk(E22_KEY_A),
            disk(CLOTH_KEY_B),
            disk(CLOTH_KEY_C_DEFAULT)
        ]
    );
    assert_eq!(as_f32(&uniform_at(&data, 0, 4)), 0.5);
    assert_eq!(as_f32(&uniform_at(&data, 1, 4)), 1.5);
    assert_eq!(as_f32(&uniform_at(&data, 2, 4)), 2.5);
    assert_eq!(as_i32(&uniform_at(&data, 3, 4)), 1);
    assert_eq!(as_i32(&uniform_at(&data, 4, 4)), -3);
    assert_eq!(as_i32(&uniform_at(&data, 5, 4)), 7);
    rig.finish();
}

#[test]
fn cloth_callbacks_require_color_id() {
    let mut w = World::new();
    w.insert_resource(ActiveSessionView::default());
    let e = w.spawn(RoofFlagClothView(1)).id();
    assert!(run_panics(env23, &mut w, &params(e)).contains(NONE_MSG));
}

// ---------------------------------------------------------------- nani 小闭包

#[test]
fn nani_kinds_match_the_immediates() {
    assert_eq!(E39_KIND as u8, 0x12);
    assert_eq!(E28_KIND as u8, 0x17);
    assert_eq!(E32_KIND as u8, 0x16);
    assert_eq!(E8_KIND as u8, 0xe);
    assert_eq!(E10_KIND as u8, 0xb);
    assert_eq!(E11_KIND as u8, 0xc);
    assert_eq!(E30_KIND as u8, 0x10);
    assert_eq!(E33_KIND as u8, 0x14);
    assert_eq!(E0_KIND as u8, 0xd);
    assert_eq!(E16_KIND as u8, 9);
    assert_eq!(E17_KIND as u8, 0x1a);
}

#[test]
fn nani_binds_precede_nani_draw() {
    type B = for<'p> fn(DrawBuilder<'p>) -> DrawBuilder<'p>;
    let cases: [(B, Vec<Call>); 5] = [
        (
            env32_binds,
            vec![disk(0x5b9a9defbd41da38), disk(E32_DISK_B.0)],
        ),
        (
            env8_binds,
            vec![
                preset(9),
                preset(10),
                preset(0xf),
                disk(E8_DISK.0),
                preset(0x16),
            ],
        ),
        (
            env10_binds,
            vec![preset(9), preset(10), preset(0xf), rg::<FlowerTag>()],
        ),
        (env30_binds, vec![preset(9), preset(10), disk(E30_DISK.0)]),
        (env33_binds, vec![disk(E33_DISK.0)]),
    ];
    for (f, want) in cases {
        let mut rig = Rig::new(&[], &[]);
        f(rig.builder()).draw();
        assert_eq!(rig.calls(), want);
        rig.finish();
    }
}

#[test]
fn env11_second_texture_depends_on_the_session_variant() {
    assert_eq!(env11_key(3), E11_KEY_V3);
    assert_eq!(env11_key(4), E11_KEY_V4);
    for v in [0u8, 1, 2, 5, 255] {
        assert_eq!(env11_key(v), E11_KEY_OTHER);
    }
    let mut rig = Rig::new(&[], &[]);
    env11_binds(rig.builder(), 4).draw();
    assert_eq!(
        rig.calls(),
        vec![
            disk(E11_DISK_A.0),
            disk(E11_KEY_V4),
            preset(9),
            preset(10),
            preset(0xf),
            rg::<BushTag>(),
        ]
    );
    rig.finish();
}

#[test]
fn nani_callbacks_panic_when_gpu_world_is_missing() {
    let cbs: [DrawListFn; 11] = [
        env39, env28, env32, env8, env10, env11, env30, env33, env17, env0, env16,
    ];
    for (n, cb) in cbs.into_iter().enumerate() {
        let mut w = World::new();
        // env11/env0/env16 先读各自的资源，所以补上；GpuWorld 始终缺失。
        w.insert_resource(ActiveSessionView::default());
        w.insert_resource(EditingHighlightView::default());
        w.insert_resource(CursorTerrainRaycast::default());
        w.insert_resource(EraserSizeMultiplier(0.0));
        w.insert_resource(RoofHighlight::default());
        let e = w.spawn_empty().id();
        let msg = run_panics(cb, &mut w, &params(e));
        assert!(msg.contains(NONE_MSG), "cb #{n}: {msg}");
    }
}

// ---------------------------------------------------------------- 33 env3

#[test]
fn env3_discards_when_weather_is_not_positive_without_reading_other_resources() {
    for v in [0.0f32, -1.0, f32::NAN] {
        let mut w = World::new();
        w.insert_resource(WeatherView { intensity: v });
        assert_eq!(read_env3(&w), Env3In::Discard);
        let e = w.spawn_empty().id();
        let mut rig = Rig::new(&[], &[]);
        let d = run(env3, &mut rig, &mut w, &params(e));
        assert!(d.is_empty());
        assert!(rig.calls().is_empty());
    }
}

#[test]
fn env3_draws_with_mode_and_season_flags() {
    let mut w = World::new();
    w.insert_resource(WeatherView { intensity: 0.5 });
    w.insert_resource(AppModeView { tag: 0x13 });
    w.insert_resource(MainCameraView {
        eye: Vec3::ZERO,
        season: 4,
    });
    let i = read_env3(&w);
    assert_eq!(
        i,
        Env3In::Draw {
            intensity: 0.5,
            mode_is_x13: true,
            season_is_4: true
        }
    );
    let mut rig = Rig::new(&[E3_DOF_BINDING], &[E3_F32, E3_BOOL_MODE, E3_BOOL_SEASON]);
    let mut data = Vec::new();
    env3_body(rig.builder(), 0.5, true, false, |b| {
        data = b.constants_data().unwrap().to_vec();
        b.draw();
    });
    assert_eq!(as_f32(&uniform_at(&data, 0, 4)), 0.5);
    assert_eq!(as_i32(&uniform_at(&data, 1, 4)), 1);
    assert_eq!(as_i32(&uniform_at(&data, 2, 4)), 0);
    assert_eq!(rig.calls(), vec![rg::<DofAutofocusTag>()]);
    rig.finish();
    // 回调：终结操作是 draw_instanced(0x2ee0)。
    let e = w.spawn_empty().id();
    let mut rig = Rig::new(&[], &[]);
    let d = run(env3, &mut rig, &mut w, &params(e));
    assert!(
        matches!(d[0].kind, DrawKind::InstancedGlDrawFlag { flag: 0x2ee0 }),
        "{:?}",
        d[0].kind
    );
}

#[test]
fn env3_flags_are_false_for_other_values() {
    let mut w = World::new();
    w.insert_resource(WeatherView { intensity: 1.0 });
    w.insert_resource(AppModeView { tag: 0x12 });
    w.insert_resource(MainCameraView {
        eye: Vec3::ZERO,
        season: 3,
    });
    assert_eq!(
        read_env3(&w),
        Env3In::Draw {
            intensity: 1.0,
            mode_is_x13: false,
            season_is_4: false
        }
    );
}

// ---------------------------------------------------------------- 35 env14

#[test]
fn env14_wooden_gate_sequence() {
    let g = WoodenGateView {
        q18: 0x1_0000_0005,
        d30: -9,
    };
    let mut w = World::new();
    let e = w.spawn(g).id();
    assert_eq!(*comp::<WoodenGateView>(&w, e), g);
    let mut rig = Rig::new(&[E14_GATE_COLOR_BINDING], &[E14_I32_A, CLOTH_ID_UNIFORM]);
    let mut data = Vec::new();
    env14_body(rig.builder(), &g, |b| {
        data = b.constants_data().unwrap().to_vec();
        b.draw();
    });
    assert_eq!(
        rig.calls(),
        vec![
            disk(E14_DISK_A),
            rg::<GateColorIdTag>(),
            disk(E14_DISK_B.0),
            disk(E14_DISK_C.0)
        ]
    );
    assert_eq!(as_i32(&uniform_at(&data, 0, 4)), 5, "qword 取低 32 位");
    assert_eq!(as_i32(&uniform_at(&data, 1, 4)), -9);
    rig.finish();
}

// ---------------------------------------------------------------- draw_lists 接线

#[test]
fn draw_lists_points_my_ids_at_this_file() {
    use crate::systems::render_loop::draw_passes::draw_lists::DrawFnId;
    let mine = [
        (DrawFnId::StairsDraw, stairs_draw_cb as DrawListFn),
        (DrawFnId::IndirectDrawBricksLod1, bricks_lod1_cb),
        (DrawFnId::IndirectDrawBricksLod0, bricks_lod0_cb),
        (DrawFnId::DrawInstancedIvyBranchShader, ivy_branch_cb),
        (DrawFnId::DrawInstancedIvyLeafShader, ivy_leaf_cb),
        (DrawFnId::DrawInstancedIvyFlowerShader, ivy_flower_cb),
        (DrawFnId::Env6, env6),
        (DrawFnId::Env27, env27),
        (DrawFnId::Env2, env2),
        (DrawFnId::Env48, env48),
        (DrawFnId::Env52, env52),
        (DrawFnId::Env57, env57),
        (DrawFnId::Env45, env45),
        (DrawFnId::Env0, env0),
        (DrawFnId::Env37, env37),
        (DrawFnId::Env39, env39),
        (DrawFnId::Env19, env19),
        (DrawFnId::Env32, env32),
        (DrawFnId::Env41, env41),
        (DrawFnId::Env53, env53),
        (DrawFnId::Env23, env23),
        (DrawFnId::Env8, env8),
        (DrawFnId::Env28, env28),
        (DrawFnId::Env16, env16),
        (DrawFnId::Env11, env11),
        (DrawFnId::Env20, env20),
        (DrawFnId::Env10, env10),
        (DrawFnId::Env55, env55),
        (DrawFnId::Env33, env33),
        (DrawFnId::Env17, env17),
        (DrawFnId::Env40, env40),
        (DrawFnId::Env22, env22),
        (DrawFnId::Env3, env3),
        (DrawFnId::Env30, env30),
        (DrawFnId::Env14, env14),
    ];
    assert_eq!(mine.len(), 35);
    for (id, f) in mine {
        assert!(id.is_recovered(), "{id:?}");
        assert_eq!(id.func() as usize, f as usize, "{id:?}");
    }
}
