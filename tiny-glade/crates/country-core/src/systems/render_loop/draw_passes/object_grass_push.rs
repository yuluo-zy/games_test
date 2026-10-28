//! Original B W9 (`object_grass_push_pass` @1409daaa0, 3955 bytes).
//! Radius comes from canonical AppMode, BrushSizes and MainCamera mode. The
//! original dynamic Vec chains query items before the optional camera/cursor
//! item; @1409daf27's comparison with 13 is the mode jump table, not a Vec cap.
//! Raster state, bindings and one six-vertex draw per Vec4 retain the current
//! pass architecture. See this family's retained full original RNE evidence.

use crate::resources::brush_sizes::BrushSizes;
use crate::systems::mode_manager::{AppMode, EraserLayer, WallShape};
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
pub const QUAD_VERTICES: u32 = 6;

/// 半径常量（O：`.rdata` 中的 f32）。
pub const RADIUS_BASE: f32 = 6.0;
pub const RADIUS_7_5: f32 = 7.5;
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

/// Native @1409daf0d..1409db013. `camera_mode` is read only for
/// Photomode here; the caller independently reads it again for positive radii.
/// Brush path is +0 and garden +8. Preserve multiply then add for Garden.
pub fn push_radius(mode: AppMode, brush: &BrushSizes, camera_mode: u8) -> f32 {
    match mode {
        AppMode::Wall(WallShape::Freeform) | AppMode::Fences => RADIUS_1_5,
        AppMode::Path | AppMode::Cobblestone(_) => brush.path + RADIUS_BASE,
        AppMode::Eraser(layer) if layer != EraserLayer::Bulldoze => RADIUS_7_5,
        AppMode::Terrain => RADIUS_7_5,
        AppMode::Garden => brush.garden * 0.75 + RADIUS_BASE,
        AppMode::Photomode if camera_mode == 3 => RADIUS_9,
        _ => 0.0,
    }
}

/// `Chain` 拼接：查询项 → 光标项（`from_iter<Vec4, Chain<…>>`）。
pub fn collect_push_objects(
    query: impl IntoIterator<Item = Vec4>,
    extra: impl IntoIterator<Item = Vec4>,
) -> Vec<Vec4> {
    query.into_iter().chain(extra).collect()
}

/// Draw every item in the original dynamic Vec. There is no thirteen-item cap;
/// the native end pointer is Vec length * 16 (@1409db369..1409db3c0).
pub fn object_grass_push_pass(ctx: &mut DrawPassCtx<'_>, objects: &[Vec4]) {
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
    super::d_main_support::collect_draw_pages(&mut pass, objects,
        |host, _| super::d_main_support::draw_constant_bytes(host, shader.0.0, 0),
        |local, objects| { for v in objects {
        let state = local.host.default_raster_state();
        let packet = DrawPacket::new(shader.clone(), [0, 0], state);
        let mut b = DrawBuilder::new(local, packet, 0).uniform(PUSH_UNIFORM, Vec4Uniform(*v));
        let p = b.packet_mut();
        p.state = overlay_state(p.state);
        b.draw_no_vao(QUAD_VERTICES);
    } }, |_| {});
    pass.render();
}

#[cfg(test)]
mod radius_tests {
    use super::*;
    use crate::systems::mode_manager::EditSubject;

    fn brush() -> BrushSizes {
        BrushSizes {
            path: 2.0,
            water: 123.0,
            garden: 4.0,
        }
    }

    #[test]
    fn native_mode_radius_branches_use_canonical_fields() {
        let b = brush();
        assert_eq!(push_radius(AppMode::Wall(WallShape::Freeform), &b, 0), 1.5);
        assert_eq!(push_radius(AppMode::Wall(WallShape::Circle), &b, 0), 0.0);
        assert_eq!(push_radius(AppMode::Wall(WallShape::Rectangle), &b, 0), 0.0);
        assert_eq!(push_radius(AppMode::Path, &b, 0), 8.0);
        assert_eq!(push_radius(AppMode::Cobblestone(7), &b, 0), 8.0);
        assert_eq!(push_radius(AppMode::Eraser(EraserLayer::All), &b, 0), 7.5);
        assert_eq!(
            push_radius(AppMode::Eraser(EraserLayer::Bulldoze), &b, 0),
            0.0
        );
        assert_eq!(push_radius(AppMode::Terrain, &b, 0), 7.5);
        assert_eq!(push_radius(AppMode::Garden, &b, 0), 9.0);
        assert_eq!(push_radius(AppMode::Fences, &b, 0), 1.5);
        assert_eq!(push_radius(AppMode::Photomode, &b, 0), 0.0);
        assert_eq!(push_radius(AppMode::Photomode, &b, 3), 9.0);
        for mode in [
            AppMode::Editing(EditSubject::Terrain(1)),
            AppMode::Color,
            AppMode::Replay,
            AppMode::LoadGame,
            AppMode::Water,
            AppMode::Tree,
            AppMode::CloneClutter,
        ] {
            assert_eq!(push_radius(mode, &b, 3), 0.0);
        }
    }

    #[test]
    fn native_ordered_positive_gate_excludes_nan() {
        let mut b = brush();
        b.path = f32::NAN;
        let radius = push_radius(AppMode::Path, &b, 0);
        assert!(radius.is_nan());
        assert!(!(radius > 0.0));
    }

    #[test]
    fn dynamic_vec_keeps_query_order_and_appends_extra() {
        let query = (0..20).map(|i| Vec4::new(i as f32, 2.0, 3.0, 0.5));
        let extra = cursor_push(false, Vec3::ONE, 1.5);
        let items = collect_push_objects(query, [extra]);
        assert_eq!(items.len(), 21);
        assert_eq!(items[19], Vec4::new(19.0, 2.0, 3.0, 0.5));
        assert_eq!(items[20], Vec4::new(0.0, 0.0, 0.0, 1.5));
    }
}
