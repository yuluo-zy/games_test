//! `object_grass_push.rs` — W9 物体推草 pass（`object_grass_push_pass` 0x1409daaa0，3955 B）。着色器 `grass_object_push`。
//!
//! O 顺序：`Res<AssetShaderLibrary>` → `get_raster_handle_by_name("grass_object_push"(0x11))`（未命中 `expect`）→
//! 若干资源（TypeId -0x63e22d59…、`AssetTextureLibrary`、-0x3d4716be…、-0x445bae11…、0x062dfe8f…=`CursorTerrainRaycast`）→
//! `get_rg_texture(预设 0xf)` 作推草目标 → 半径常量（f32：`6.0`、`7.5`、`1.5 * 0.75 + 6.0`、`9.0`；由相机/模式字节
//! `[res+4] ∈ {0, 3}`、`[res+0x4d0] == 3` 选择，选择条件 U）→ `Query<(&Transform, …)>` 映射为 `Vec4(位置, 半径)`，
//! 与 `CameraMatrices::eye_position` 及光标 `Vec4(position or 0, 半径)` `chain` 后 `collect::<Vec<Vec4>>()`（上限 13：`cmp rcx, 0xd`）→
//! `extent_2d(目标)` → `RasterPass::new_impl("Grass object push pass"(0x16), [tag1（视图字 0）= 目标], 无深度, 视口 = 目标)` →
//! **每个 Vec4 一次**：`DrawPacket::new(head = [0, …])` / `DrawBuilder::new(variant 0)` →
//! `write_uniform<Vec4>(0x337ad802c8b34f67, v)` → 状态 `byte0 &= 0x7f`、`byte3 = (byte3 & 0x9f) | 0x40` → `draw_no_vao(6)` → `render`。
//! `ObjectGrassPush` 与 `compute_passes::grass` 共用（见该模块）。
//!
//! # 半径选择（B_d_gap 解码，O：0x1409daf0d..0x1409db013）
//! `GladeTheme`（TypeId 0xc2b8e941c91a6428/0x59f3c6d1d3da43a0，与 `compute_passes::grass::GladeThemeId` 同一资源）首个 `u32 t`、
//! 字节 `[+4] = sub`；`t - 6`（无符号 < 14）查 14 项跳转表（`0x142ac2dd8`）：
//!
//! | `t` | 半径 |
//! |---|---|
//! | 6 | `sub == 0` ? `1.5` : `0` |
//! | 7、8 | `P.f0 + 6.0`（`P` = TypeId -0x63e22d59476fc50 资源，`+0` f32）|
//! | 9 | `sub != 3` ? `7.5` : `0` |
//! | 10 | `7.5` |
//! | 11..=16 及范围外 | `0` |
//! | 17 | `P.f8 * 0.75 + 6.0`（`+8` f32）|
//! | 18 | `1.5` |
//! | 19 | `CameraMatrices` 字节 `[+0x4d0] == 3` ? `9.0` : `0` |
//!
//! 半径 `> 0`（`ucomiss; jbe`）才产生额外项：`CameraMatrices[+0x4d0] == 3` → `Vec4(eye_position, radius)`，
//! 否则 `CursorTerrainRaycast`（TypeId 0x062dfe8f…）`[+0] & 1` 命中 → `Vec4(position@+4, radius)`、未命中 → `Vec4(0, radius)`。
//! 查询项（`Query<(&Transform, &ObjectGrassPush)>`，`chain::next` 内联 map 闭包 0x1412f0f23）= `Vec4(translation, push)`，
//! `ObjectGrassPush` 是单个 f32（`movss xmm1,[r8]`）。额外项排在查询项**之后**（`Chain<Map<QueryIter>, Option>`）。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::collapsible_match)]
use glam::{Vec3, Vec4};

use crate::render::pass_builder::draw_builder::{DrawBuilder, UniformValue};
use crate::render::pass_builder::draw_stream::DrawPacket;
use crate::systems::render_loop::draw_passes::custom_cursor_pass::overlay_state;
use crate::systems::render_loop::draw_passes::d_main_support::{ColorSpec, DepthSpec, DrawPassCtx};

pub const SHADER_NAME: &str = "grass_object_push";
pub const PASS_NAME: &str = "Grass object push pass";
/// 推草目标的 RG 纹理预设（O：`mov r8b, 0xf`）。
pub const TARGET_PRESET: u8 = 0xf;
pub const PUSH_UNIFORM: u64 = 0x337a_d802_c8b3_4f67;
/// Vec4 个数上限（O：`cmp rcx, 0xd`）。
pub const MAX_PUSH_OBJECTS: usize = 13;
pub const QUAD_VERTICES: u32 = 6;

/// 半径常量（O：`.rdata` 中的 f32）。
pub const RADIUS_BASE: f32 = 6.0;
pub const RADIUS_7_5: f32 = 7.5;
pub const RADIUS_NEAR: f32 = 1.5 * 0.75 + 6.0;
pub const RADIUS_9: f32 = 9.0;
pub const RADIUS_1_5: f32 = 1.5;

/// Vec4 uniform（16 字节）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec4Uniform(pub Vec4);
impl UniformValue for Vec4Uniform {
    fn write_bytes(&self, out: &mut Vec<u8>) {
        for c in self.0.to_array() {
            out.extend_from_slice(&c.to_le_bytes());
        }
    }
}

/// 光标项：`CursorTerrainRaycast::position`（未命中 = 0）扩成 `Vec4(position, radius)`（O：`movsd/movss` + `shufps`）。
pub fn cursor_push(cursor_hit: bool, position: Vec3, radius: f32) -> Vec4 {
    let p = if cursor_hit { position } else { Vec3::ZERO };
    p.extend(radius)
}

/// 半径选择所需的资源字段（O：见模块头）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PushRadiusInputs {
    /// `GladeTheme` 首个 u32。
    pub theme: u32,
    /// `GladeTheme` `+4` 字节（语义 U）。
    pub theme_sub: u8,
    /// TypeId -0x63e22d59476fc50 资源 `+0` f32（语义 U）。
    pub param_f0: f32,
    /// 同资源 `+8` f32。
    pub param_f8: f32,
    /// `CameraMatrices` `+0x4d0` 字节（== 3 选择相机位置作为额外项）。
    pub camera_mode: u8,
}

/// 额外项的半径（0.0 = 不产生额外项）。O：`GladeTheme` 跳转表。
pub fn push_radius(i: &PushRadiusInputs) -> f32 {
    let idx = i.theme.wrapping_sub(6);
    if idx >= 14 {
        return 0.0;
    }
    match i.theme {
        6 => {
            if i.theme_sub == 0 {
                RADIUS_1_5
            } else {
                0.0
            }
        }
        7 | 8 => i.param_f0 + RADIUS_BASE,
        9 => {
            if i.theme_sub != 3 {
                RADIUS_7_5
            } else {
                0.0
            }
        }
        10 => RADIUS_7_5,
        17 => i.param_f8 * 0.75 + RADIUS_BASE,
        18 => RADIUS_1_5,
        19 => {
            if i.camera_mode == 3 {
                RADIUS_9
            } else {
                0.0
            }
        }
        _ => 0.0,
    }
}

/// 组装 Vec4 列表（O：`Chain<Map<QueryIter>, Option<Vec4>>`）。`objects` = `(translation, ObjectGrassPush)`；
/// `eye` = `CameraMatrices::eye_position`；`cursor` = `Some(position)` 表示 `CursorTerrainRaycast` 命中。
pub fn assemble_push_objects(
    inputs: &PushRadiusInputs,
    objects: &[(Vec3, f32)],
    eye: Vec3,
    cursor: Option<Vec3>,
) -> Vec<Vec4> {
    let radius = push_radius(inputs);
    let extra = (radius > 0.0).then(|| {
        if inputs.camera_mode == 3 {
            eye.extend(radius)
        } else {
            cursor_push(cursor.is_some(), cursor.unwrap_or(Vec3::ZERO), radius)
        }
    });
    collect_push_objects(objects.iter().map(|(p, r)| p.extend(*r)), extra)
}

/// `Chain` 拼接：查询项 → 光标项（`from_iter<Vec4, Chain<…>>`）。
pub fn collect_push_objects(
    query: impl IntoIterator<Item = Vec4>,
    extra: impl IntoIterator<Item = Vec4>,
) -> Vec<Vec4> {
    query.into_iter().chain(extra).collect()
}

/// `object_grass_push_pass(ctx, world)`。`objects` 已按上述规则组装；超过 13 个 → panic（ArrayVec 容量）。
pub fn object_grass_push_pass(ctx: &mut DrawPassCtx<'_>, objects: &[Vec4]) {
    assert!(
        objects.len() <= MAX_PUSH_OBJECTS,
        "ArrayVec: capacity exceeded in extend/from_iter"
    );
    let shader = ctx
        .shader(SHADER_NAME)
        .expect("called `Option::expect()` on a `None` value");
    let target = ctx.host().rg_texture(TARGET_PRESET);
    let mut pass = ctx.raster_pass(
        PASS_NAME,
        &[ColorSpec::tag1(target)],
        DepthSpec::None,
        target,
    );
    for v in objects {
        let state = pass.host.default_raster_state();
        let packet = DrawPacket::new(shader.clone(), [0, 0], state);
        let mut b = DrawBuilder::new(&mut pass, packet, 0).uniform(PUSH_UNIFORM, Vec4Uniform(*v));
        let p = b.packet_mut();
        p.state = overlay_state(p.state);
        b.draw_no_vao(QUAD_VERTICES);
    }
    pass.render();
}

#[cfg(test)]
mod radius_tests {
    use super::*;

    fn inp(theme: u32, sub: u8) -> PushRadiusInputs {
        PushRadiusInputs {
            theme,
            theme_sub: sub,
            param_f0: 2.0,
            param_f8: 4.0,
            camera_mode: 0,
        }
    }

    #[test]
    fn jump_table_radii_match_the_machine_code() {
        assert_eq!(push_radius(&inp(6, 0)), 1.5);
        assert_eq!(push_radius(&inp(6, 1)), 0.0);
        assert_eq!(push_radius(&inp(7, 9)), 8.0);
        assert_eq!(push_radius(&inp(8, 9)), 8.0);
        assert_eq!(push_radius(&inp(9, 0)), 7.5);
        assert_eq!(push_radius(&inp(9, 3)), 0.0);
        assert_eq!(push_radius(&inp(10, 3)), 7.5);
        for t in 11..=16 {
            assert_eq!(push_radius(&inp(t, 0)), 0.0, "theme {t}");
        }
        assert_eq!(push_radius(&inp(17, 0)), 4.0 * 0.75 + 6.0);
        assert_eq!(push_radius(&inp(18, 7)), 1.5);
        assert_eq!(push_radius(&inp(5, 0)), 0.0);
        assert_eq!(push_radius(&inp(20, 0)), 0.0);
        assert_eq!(push_radius(&inp(0, 0)), 0.0);
    }

    #[test]
    fn theme_19_needs_camera_mode_3() {
        let mut i = inp(19, 0);
        assert_eq!(push_radius(&i), 0.0);
        i.camera_mode = 3;
        assert_eq!(push_radius(&i), RADIUS_9);
    }

    #[test]
    fn extra_item_follows_query_items_and_picks_eye_or_cursor() {
        let objs = [(Vec3::new(1.0, 2.0, 3.0), 0.5)];
        let v = assemble_push_objects(
            &inp(18, 0),
            &objs,
            Vec3::new(9.0, 9.0, 9.0),
            Some(Vec3::new(4.0, 5.0, 6.0)),
        );
        assert_eq!(
            v,
            vec![Vec4::new(1.0, 2.0, 3.0, 0.5), Vec4::new(4.0, 5.0, 6.0, 1.5)]
        );
        // 光标未命中 → 0 位置
        let v = assemble_push_objects(&inp(18, 0), &[], Vec3::ONE, None);
        assert_eq!(v, vec![Vec4::new(0.0, 0.0, 0.0, 1.5)]);
        // camera_mode == 3 → eye_position
        let mut i = inp(19, 0);
        i.camera_mode = 3;
        let v = assemble_push_objects(&i, &[], Vec3::new(7.0, 8.0, 9.0), None);
        assert_eq!(v, vec![Vec4::new(7.0, 8.0, 9.0, 9.0)]);
        // 半径 0 → 无额外项
        assert!(assemble_push_objects(&inp(12, 0), &[], Vec3::ONE, None).is_empty());
    }
}
