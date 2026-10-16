//! timberframe_rectangle.rs — 矩形木框架梁构建（B 0x141a41ba0，O；re-analyst-A
//! 契约 §2）。
//!
//! 算法骨架（O）：`Rectangle2d::as_curve2`（0x140c98cf0）×2 / `as_curve3`
//! （0x140c991f0）×3 / `get_point`（0x140c99600）×2 / `orientation_mat3`
//! （0x140c99570）×2 / `perimeter`（0x140c99530）；`RoofRidgeDims::calc`
//! （0x1408e2700）、`ridge_length_01_from_rectangle_ratio`（0x1408e3b60）、
//! `roof_height_ws_from_wall_shape`（0x1408e3c70）；常量 0.2 间距（0x142a8c6f0）、
//! 0.5 归一化（0x142925870）、0.63 缩放（0x1429258e0 区）、1.0/-0.3
//! （0x142925900）；输出四边梁实例 → `TimberframeInstanceBuffer`。

use bevy_ecs::prelude::*;
use glam::Vec3;
use utils::geometry::Rectangle2d;

use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::events::ConstructTimberframeCmd;
use country_core::resources::roofs::roof_shape::roof_height_ws_from_wall_shape;
use country_core::systems::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache;
use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};

use crate::timberframe_beam_count_change_audio::TimberframeBeamCount;
use crate::timberframe_circle::{TimberframePlank, BEAM_SPACING, PLANK_LENGTH_SCALE};
use crate::TimberframeInstanceBuffer;

/// 归一化系数（O：0x142925870 = 0.5，`0.5 / sqrt(…)` 归一化面）。
pub const NORMALIZE_HALF: f32 = 0.5;
/// 矩形缩放（O：0x1429258e0 区 = 0.63，`(a+b) * 0.63` 面；字节直读 I）。
pub const RECT_SCALE: f32 = 0.63;

/// `construct_timberframe_rectangle` @0x141a41ba0（O 面）。
pub fn construct_timberframe_rectangle(
    mut constructs: EventReader<ConstructTimberframeCmd>,
    mut buffer: ResMut<TimberframeInstanceBuffer>,
    _library: Res<AssetSsboLibrary>,
    mut beams: ResMut<TimberframeBeamCount>,
    _params: Res<TimberframeConstructionParamsCache>,
) {
    for cmd in constructs.read() {
        if cmd.shape.len() < 28 {
            continue;
        }
        let params = ShapeParameters::from_observed(cmd.shape[..28].try_into().unwrap());
        match params.get_shape() {
            Some(Shape::Rectangle(rect)) => {
                build_rectangle_beams(rect, &cmd.shape, &mut buffer.0);
            }
            _ => {}
        }
        beams.current = buffer.0.len() as i32;
        beams.non_empty = !buffer.0.is_empty();
    }
}

/// 矩形四边梁（O 骨架：as_curve2/3 + get_point + orientation + perimeter +
/// roof/ridge 域高度；采样相位 I）。
fn build_rectangle_beams(rect: Rectangle2d, wall_shape: &[u8], out: &mut Vec<TimberframePlank>) {
    let height = roof_height_ws_from_wall_shape(wall_shape);
    let corners = rect.as_points2();
    let perimeter = rect.perimeter();
    let count = (perimeter / BEAM_SPACING).ceil().max(1.0) as usize;
    for index in 0..count {
        // I：矩形边 u 采样（机器码 get_point ×2 的边采样族；四边成对输出——
        // re-analyst §2：实例范围标记 {start=0, count=2}）。
        let t = (index as f32 + 0.5) / count as f32;
        let edge = index % 4;
        let a = corners[edge];
        let b = corners[(edge + 1) % 4];
        let pos = a + (b - a) * t;
        let dir = (b - a).normalize();
        out.push(TimberframePlank {
            position: Vec3::new(pos.x, height * NORMALIZE_HALF, pos.y),
            orientation: Vec3::new(dir.y, 0.0, -dir.x),
            length: PLANK_LENGTH_SCALE * (b - a).length() * RECT_SCALE,
            _pad: [0; 4],
        });
    }
}
