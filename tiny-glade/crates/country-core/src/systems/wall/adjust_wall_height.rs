//! B AdjustWallHeightCmd transport into the actual InnerWalls authority.
//! Original submodules (`adjust_wall_height::{adjust_wall_height_impl, signifier_animation, ui_adjust_wall_height_mode}`).
pub mod adjust_wall_height_impl;
pub mod signifier_animation;
pub mod ui_adjust_wall_height_mode;
pub mod ui_adjust_wall_height;
use crate::systems::wall::height_command::AdjustWallHeightRequest;
use crate::systems::wall::height_command::WallHeightCommandError;
use crate::systems::wall::height_command::WallHeightFeedback;
use crate::resources::walls::WallId;
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use crate::systems::camera_rig::main_camera::MainCamera;

#[derive(Event, Clone, Copy, Debug)]
pub struct AdjustWallHeightCmd(pub AdjustWallHeightRequest);
/// 旧合并反馈事件。原版 `adjust_wall_height_impl` 不产生它（见 `adjust_wall_height_impl`：写
/// `RequestWallMaterialAudio`/`PlayBuildingAudio`）；仅为宿主既有 `add_event` 引用保留。
#[derive(Event, Clone, Copy, Debug)]
pub struct WallHeightAudio(pub WallHeightFeedback);
/// 旧失败事件：原版没有该事件（`mode`/曲线非法走 panic）；不再被写出，仅为宿主引用保留。
#[derive(Event, Clone, Copy, Debug)]
pub struct WallHeightCommandFailed {
    pub wall: WallId,
    pub error: WallHeightCommandError,
}

pub fn register_wall_height_events(world: &mut World) {
    world.init_resource::<Events<AdjustWallHeightCmd>>();
    world.init_resource::<Events<WallHeightAudio>>();
    world.init_resource::<Events<WallHeightCommandFailed>>();
    // 原版 `adjust_wall_height_impl` 实际写出的两个事件队列（0x140ac1f20 param_3/param_4）。
    world.init_resource::<Events<crate::systems::audio::RequestWallMaterialAudio>>();
    world.init_resource::<Events<crate::systems::audio::PlayBuildingAudio>>();
    // B `adjust_wall_ui_mode::UiWallHeight` host resource (+0 wall, +8 flag,
    // +9 mode; +0x90/+0x98 points, +0xe0/+0xe8 heights).
    world.init_resource::<UiWallHeight>();
}

/// 原版系统 `adjust_wall_height::adjust_wall_height_impl::adjust_wall_height_impl`
/// （0x140ac1f20）。本仓库旧名 `adjust_wall_height` 保留为同一函数项别名，
/// 以免宿主 `.before(adjust_wall_height)` 排序引用失效；原版没有第二个系统。
pub use self::adjust_wall_height_impl::adjust_wall_height_impl as adjust_wall_height;

// ---------------------------------------------------------------------------
// `ui_adjust_wall_height_mode::get_closest_point_u_to_cursor` (B 0x140ac7400)
//
// Fresh evidence (restorer-walls5, 2026-10-17): decompiled body + 301-instruction
// disassembly (`work/restorer-walls5/decomp/140ac7400.{c,txt,asm}`), helper
// disassembly (`utils::camera::CameraMatrices::aspect_ratio` 0x140c96e60 =
// `[rcx+0x14] / [rcx]`, `utils::lerp` 0x140c904d0 = `a + (b - a) * t`,
// `utils::mouse_ss_to_cs` 0x140c906d0), caller `ui_adjust_wall_height_mode`
// 0x140ac6d60 (call site 0x140ac705f), constants via `read_memory`
// (1e-20 @0x142a7baa0, 1e-10 @0x142abaa00, FLT_MAX @0x142925980, 1.0
// @0x142925904) and panic strings in the shipped EXE.
//
// Behaviour: for every segment `(points[i], points[i+1])` project the two
// (height-lifted) points through `view` then `projection`, divide by `w`,
// scale x by `aspect_ratio`, find the clamped closest point to the canvas-space
// cursor, keep the best segment, and return the perspective-correct
// interpolation fraction applied to `u_values` and the 3D point.
// ---------------------------------------------------------------------------

/// Native `CameraMatrices` matrix block inside `MainCamera.raw` (fresh
/// 0x140ac7400: the world→view matrix is read at `+0x80`, then the projection
/// at `+0x00`; `aspect_ratio` 0x140c96e60 reads `+0x14/+0x00`; `eye_position`
/// 0x140c96d40 reads `inverse_view` at `+0xc0`).
pub const CAMERA_PROJECTION_OFFSET: usize = 0x00;
/// World→view matrix (`MainCamera.raw + 0x80`), see above.
pub const CAMERA_VIEW_OFFSET: usize = 0x80;

/// Result of [`get_closest_point_u_to_cursor`]: the interpolated curve
/// parameter and the interpolated (height-lifted) 3D point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallCursorHit {
    pub u: f32,
    pub point: Vec3,
}

/// 原 `UiWallHeight` 的局部字段为WallId/+8 flag/+9 mode（原140b303c6..ce）。
/// 原widget的+0x90/+0x98和+0xe0/+0xe8属于独立传入的PublicWallState，
/// 分别是prepared Curve3和arch_segment_heights，不能把它们当成self字段。
/// 本类型保留早期公共数学辅助面上的owned points/heights以兼容既有调用；
/// 真实生产者借用PublicWallState，不填充或克隆这两个兼容数组。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct UiWallHeight {
    /// +0x00.
    pub wall: WallId,
    /// +0x08.
    pub flag: u8,
    /// +0x09.
    pub mode: u8,
    /// 兼容数学辅助面；真实生产读取PublicWallState::height_outline。
    pub points: Vec<Vec3>,
    /// 兼容数学辅助面；真实生产读取PublicWallState::arch_segment_heights。
    pub heights: Vec<f32>,
}

impl UiWallHeight {
    /// Widget `0.6` lift when the mode byte is 0 (0x1409566d0).
    pub const FLAT_LIFT: f32 = 0.6;
    /// Widget `-0.29` lift when mode != 0 and the stack flag is set.
    pub const STACKED_LIFT: f32 = f32::from_bits(0xbe94_7ae2);
    /// Signifier env scalar when `+8 == 0` (0x3e4ccccd = 0.2).
    pub const SIGNIFIER_ENV_SCALAR: f32 = 0.2;

    /// Widget point transform (0x1409566d0): `min(points.len, heights.len)`
    /// entries, y replaced by `height + lift` where
    /// `lift = mode == 0 ? 0.6 : (stacked ? -0.29 : 0.0)`.
    pub fn lifted_points(&self, stacked: bool) -> Vec<Vec3> {
        let count = self.points.len().min(self.heights.len());
        let lift = if self.mode == 0 {
            Self::FLAT_LIFT
        } else if stacked {
            Self::STACKED_LIFT
        } else {
            0.0
        };
        self.points[..count]
            .iter()
            .zip(&self.heights[..count])
            .map(|(point, height)| Vec3::new(point.x, *height + lift, point.z))
            .collect()
    }

    /// Widget name: `format!("edit_height_{}", WallId Display)`.
    pub fn widget_name(&self) -> String {
        format!("edit_height_{}", self.wall)
    }

    /// Signifier guard for `mode == 1 && flag == 0` (0x140956cd0): the native
    /// returns early unless `byte[state+0x18] != 0`, `qword[state+0x38] ==
    /// context widget id` and `dword[state+0x28] != 2`.
    pub fn signifier_guard_passes(&self, state: &crate::systems::ui::resources::ui_state::UiState) -> bool {
        if self.mode != 1 || self.flag != 0 {
            return true;
        }
        state.hover_byte() != 0
            && state.active_widget() == crate::systems::ui::resources::ui_state::WidgetId::from_name(&self.widget_name())
            && state.intersection_tag() != 2
    }

    /// Signifier names (0x140956cd0):
    /// - `mode == 1`: `wall_height_edit_{wall}`;
    /// - else: `wall_height_edit_{wall}{count}` where `count` is GameTime.frame_count,
    ///   saved from R9D at 0x140956cfa.
    pub fn signifier_name(&self, frame: u32) -> String {
        if self.mode == 1 {
            format!("wall_height_edit_{}", self.wall)
        } else {
            format!("wall_height_edit_{}{}", self.wall, frame)
        }
    }

    /// Signifier env scalar for the `mode == 1` branch: `0.2` when `+8 == 0`,
    /// otherwise `0.0` (0x140956cd0 `uVar10 = cVar2 == 0 ? 0x3e4ccccd : 0`).
    pub fn signifier_env_scalar(&self) -> f32 {
        if self.flag == 0 {
            Self::SIGNIFIER_ENV_SCALAR
        } else {
            0.0
        }
    }
}

/// B 0x140ac7400 `get_closest_point_u_to_cursor`.
///
/// `cursor_canvas` uses the native pair order `(screen_y, screen_x * aspect)`
/// (`UNPCKLPS XMM2(second float arg), XMM1(first float arg)` at 0x140ac7567;
/// the caller passes `(mouse.y, mouse.x * aspect)`).
///
/// Panics exactly where the original does:
/// - empty `points` → `assertion failed: !curve.points.is_empty()` (0x2a B
///   string, 0x142adf9e0);
/// - final `u` outside `[0, 1]` (NaN included, because `UCOMISS; JC` sets CF
///   on unordered) → `Tried to init CurveU from an invalid u: {}`
///   (0x142aa5398, 40 B) with the value formatted;
/// - then `assert!(!u.is_nan())` → `assertion failed: !u.is_nan()` (29 B,
///   0x142aa53d0; unreachable for NaN because the range guard fires first,
///   kept because the machine code contains it).
pub fn get_closest_point_u_to_cursor(
    camera: &MainCamera,
    points: &[Vec3],
    u_values: &[f32],
    cursor_canvas: Vec2,
    height_offset: f32,
) -> WallCursorHit {
    assert!(
        !points.is_empty(),
        "assertion failed: !curve.points.is_empty()"
    );
    let projection = camera.read_matrix(CAMERA_PROJECTION_OFFSET);
    let view = camera.read_matrix(CAMERA_VIEW_OFFSET);
    let aspect = projection.y_axis.y / projection.x_axis.x;
    let point_count = points.len();
    // 0x140ac74bf: `len - 1 == 0` (single point) skips the loop and returns
    // the zero-initialised result.
    if point_count < 2 {
        return WallCursorHit {
            u: 0.0,
            point: Vec3::ZERO,
        };
    }
    // 0x140ac749c..0x140ac74bb: `min(len_points, len_u)` and
    // `min(len_u - 1, len_points - 1)` bound the loop; the effective segment
    // count is `min(len_u - 1, len_points - 1)`.
    let segment_count = u_values
        .len()
        .saturating_sub(1)
        .min(point_count - 1);

    let project = |point: Vec3| -> (Vec2, f32) {
        // Column-major `view * p`, then `projection * (view * p)`; both are
        // the exact splat-multiply-add chains at 0x140ac75f3..0x140ac76d8.
        let view_point = view.transform_point3(point);
        let clip = projection * view_point.extend(1.0);
        let inv_w = 1.0 / clip.w;
        // 0x140ac76f3: x /= w then *= aspect; y /= w. Pair order is (y, x).
        (
            Vec2::new(clip.y * inv_w, clip.x * inv_w * aspect),
            clip.w,
        )
    };

    let mut best = f32::MAX; // 0x142925980
    let mut u_out = 0.0f32;
    let mut point_out = Vec3::ZERO;
    for index in 0..segment_count {
        // 0x140ac75c8..0x140ac75e6: only the y (second) component is lifted.
        let p0 = Vec3::new(
            points[index].x,
            points[index].y + height_offset,
            points[index].z,
        );
        let p1 = Vec3::new(
            points[index + 1].x,
            points[index + 1].y + height_offset,
            points[index + 1].z,
        );
        let (s0, w0) = project(p0);
        let (s1, w1) = project(p1);

        // 0x140ac7725..0x140ac7783: closest point on the 2D segment, with
        // `MAXSS 0` / `MINSS 1` ordered-semantics clamping.
        let direction = s1 - s0;
        let length_sq = direction.x * direction.x + direction.y * direction.y;
        // `MAXSS length_sq, 1e-20`: NaN in the length selects 1e-20.
        let denominator = if length_sq > 1e-20 { length_sq } else { 1e-20 };
        let raw_t = ((cursor_canvas.x - s0.x) * direction.x
            + (cursor_canvas.y - s0.y) * direction.y)
            / denominator;
        // 0x140ac7764..0x140ac777a: max(0, raw) then min(1, .) keeping NaN.
        let t_above_zero = if raw_t < 0.0 { 0.0 } else { raw_t };
        let t = if t_above_zero > 1.0 {
            1.0
        } else {
            t_above_zero
        };

        let closest = Vec2::new(
            s0.x * (1.0 - t) + s1.x * t,
            s0.y * (1.0 - t) + s1.y * t,
        );
        let delta = closest - cursor_canvas;
        let distance = (delta.x * delta.x + delta.y * delta.y).sqrt();

        // 0x140ac77b5 `UCOMISS best, distance; JBE skip` is an ordered `<`
        // (unordered/NaN skips the update).
        if distance < best {
            // 0x140ac77be..0x140ac781c: perspective-correct fraction
            //   lerp(0 / w0, 1 / w1, t) / max(lerp(1 / w0, 1 / w1, t), 1e-10)
            // clamped by `MAXSS 0` / `MINSS 1` (`utils::lerp` = a + (b-a)t).
            let inverse_w0 = 1.0 / w0;
            let inverse_w1 = 1.0 / w1;
            let zero_over_w0 = 0.0 / w0;
            let numerator = zero_over_w0 + (inverse_w1 - zero_over_w0) * t;
            let raw_denominator =
                inverse_w0 + (inverse_w1 - inverse_w0) * t;
            // `MAXSS denominator, 1e-10`: NaN selects 1e-10.
            let denominator = if raw_denominator > 1e-10 {
                raw_denominator
            } else {
                1e-10
            };
            let ratio = numerator / denominator;
            let above_zero = if ratio < 0.0 { 0.0 } else { ratio };
            let fraction = if above_zero > 1.0 {
                1.0
            } else {
                above_zero
            };

            // 0x140ac7821..0x140ac7835: lerp the per-point u values.
            let from_u = u_values[index];
            let to_u = u_values[index + 1];
            u_out = from_u + (to_u - from_u) * fraction;
            // 0x140ac784b..0x140ac7876: `p0 * (1 - fraction) + p1 * fraction`.
            let keep = 1.0 - fraction;
            point_out = Vec3::new(
                p0.x * keep + p1.x * fraction,
                p0.y * keep + p1.y * fraction,
                p0.z * keep + p1.z * fraction,
            );
            best = distance;
        }
    }

    // 0x140ac78b0..0x140ac78cd: CurveU constructor assertions.
    if !(u_out >= 0.0 && u_out <= 1.0) {
        panic!("Tried to init CurveU from an invalid u: {}", u_out);
    }
    assert!(!u_out.is_nan(), "assertion failed: !u.is_nan()");
    WallCursorHit {
        u: u_out,
        point: point_out,
    }
}

#[cfg(test)]
mod cursor_tests {
    use super::*;
    use crate::systems::camera_rig::main_camera::MainCamera;

    fn identity_camera() -> MainCamera {
        let mut camera = MainCamera::default();
        camera.write_matrix(CAMERA_PROJECTION_OFFSET, &glam::Mat4::IDENTITY);
        camera.write_matrix(CAMERA_VIEW_OFFSET, &glam::Mat4::IDENTITY);
        camera
    }

    #[test]
    fn single_point_returns_the_zero_initialised_result() {
        let camera = identity_camera();
        let hit = get_closest_point_u_to_cursor(
            &camera,
            &[Vec3::new(1.0, 2.0, 3.0)],
            &[0.5],
            Vec2::ZERO,
            0.0,
        );
        assert_eq!(hit.u, 0.0);
        assert_eq!(hit.point, Vec3::ZERO);
    }

    #[test]
    fn closest_segment_interpolates_u_and_point() {
        // Identity projection: aspect = 1, screen pair = (y, x).
        let camera = identity_camera();
        let points = [Vec3::new(0.0, 0.0, 0.0), Vec3::new(10.0, 0.0, 0.0)];
        let hit = get_closest_point_u_to_cursor(
            &camera,
            &points,
            &[0.0, 1.0],
            Vec2::new(0.0, 4.0), // (y, x) = (0, 4)
            0.0,
        );
        assert!((hit.u - 0.4).abs() < 1e-6);
        assert!((hit.point.x - 4.0).abs() < 1e-6);
        assert_eq!(hit.point.y, 0.0);
        assert_eq!(hit.point.z, 0.0);
    }

    #[test]
    fn height_offset_lifts_the_second_component() {
        let camera = identity_camera();
        let points = [Vec3::new(0.0, 0.0, 0.0), Vec3::new(10.0, 0.0, 0.0)];
        let hit = get_closest_point_u_to_cursor(
            &camera,
            &points,
            &[0.0, 1.0],
            Vec2::new(2.0, 5.0), // (y, x) = (2, 5)
            2.0,
        );
        assert!((hit.u - 0.5).abs() < 1e-6);
        assert!((hit.point.x - 5.0).abs() < 1e-6);
        assert!((hit.point.y - 2.0).abs() < 1e-6);
    }

    #[test]
    fn perspective_fraction_differs_from_geometric_t() {
        let mut camera = MainCamera::default();
        // Column-major projection: w = z (column 2's w lane).
        let projection = glam::Mat4::from_cols(
            glam::Vec4::new(1.0, 0.0, 0.0, 0.0),
            glam::Vec4::new(0.0, 1.0, 0.0, 0.0),
            glam::Vec4::new(0.0, 0.0, 0.0, 1.0),
            glam::Vec4::ZERO,
        );
        camera.write_matrix(CAMERA_PROJECTION_OFFSET, &projection);
        camera.write_matrix(CAMERA_VIEW_OFFSET, &glam::Mat4::IDENTITY);
        // w0 = 1, w1 = 2; screen y = y / w: s0 = 0, s1 = 5.
        // Cursor at the geometric midpoint (t = 0.5) must still use the
        // perspective-correct fraction lerp(0, 1/w1, t)/lerp(1/w0, 1/w1, t)
        // = 0.25 / 0.75 = 1/3.
        let points = [Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 10.0, 2.0)];
        let hit = get_closest_point_u_to_cursor(
            &camera,
            &points,
            &[0.0, 1.0],
            Vec2::new(2.5, 0.0),
            0.0,
        );
        assert!(
            (hit.u - 1.0 / 3.0).abs() < 1e-6,
            "perspective fraction u = {} (point {:?})",
            hit.u,
            hit.point
        );
        assert!((hit.point.y - 10.0 / 3.0).abs() < 1e-5);
        assert!((hit.point.z - 4.0 / 3.0).abs() < 1e-5);
    }

    #[test]
    #[should_panic(expected = "assertion failed: !curve.points.is_empty()")]
    fn empty_points_panics() {
        let camera = identity_camera();
        let _ = get_closest_point_u_to_cursor(&camera, &[], &[], Vec2::ZERO, 0.0);
    }

    #[test]
    #[should_panic(expected = "Tried to init CurveU from an invalid u")]
    fn out_of_range_u_panics_with_the_curve_u_message() {
        let camera = identity_camera();
        let points = [Vec3::new(0.0, 0.0, 0.0), Vec3::new(10.0, 0.0, 0.0)];
        let _ = get_closest_point_u_to_cursor(
            &camera,
            &points,
            &[2.0, 3.0],
            Vec2::new(0.0, 4.0),
            0.0,
        );
    }
}

#[cfg(test)]
mod ui_wall_height_tests {
    use super::*;
    use crate::systems::ui::resources::ui_state::{UiInteraction, UiState, WidgetId};
    use crate::resources::walls::WallId;

    fn host(mode: u8, flag: u8) -> UiWallHeight {
        UiWallHeight {
            wall: WallId(12),
            flag,
            mode,
            points: vec![Vec3::new(1.0, 9.0, 3.0), Vec3::new(4.0, 9.0, 6.0)],
            heights: vec![2.0, 5.0],
        }
    }

    /// 0x1409566d0: `mode == 0` lifts y by +0.6; `mode != 0` uses
    /// 0.0 / -0.29 depending on the stack flag.
    #[test]
    fn widget_lift_matches_the_fresh_branches() {
        let flat = host(0, 0).lifted_points(false);
        assert_eq!(flat[0], Vec3::new(1.0, 2.6, 3.0));
        let mode_one_flat = host(1, 0).lifted_points(false);
        assert_eq!(mode_one_flat[1], Vec3::new(4.0, 5.0, 6.0));
        let mode_one_stacked = host(1, 0).lifted_points(true);
        assert_eq!(mode_one_stacked[1], Vec3::new(4.0, 4.71, 6.0));
    }

    /// The zip stops at the shorter source (`min(points.len, heights.len)`).
    #[test]
    fn widget_lift_uses_the_shorter_source() {
        let mut value = host(0, 0);
        value.heights.pop();
        assert_eq!(value.lifted_points(false).len(), 1);
    }

    /// 0x140956cd0 names and the mode/flag env scalar.
    #[test]
    fn signifier_names_and_env_scalar_match_fresh() {
        assert_eq!(host(1, 0).signifier_name(2), "wall_height_edit_WallId(12)");
        assert_eq!(host(0, 0).signifier_name(2), "wall_height_edit_WallId(12)2");
        assert_eq!(host(1, 0).signifier_env_scalar(), 0.2);
        assert_eq!(host(1, 1).signifier_env_scalar(), 0.0);
    }

    /// mode == 1 && flag == 0 requires hover, active id and intersection != 2.
    #[test]
    fn signifier_guard_reads_the_ui_state_fields() {
        let value = host(1, 0);
        let id = WidgetId::from_name(&value.widget_name());
        let mut state = UiState::new(UiInteraction {
            native_tag: 1,
            native_phase: 0,
            widget: id,
        });
        assert!(value.signifier_guard_passes(&state));
        state.set_intersection_tag(2);
        assert!(!value.signifier_guard_passes(&state));
        state.set_intersection_tag(0);
        state.replace_interaction(UiInteraction {
            native_tag: 0,
            native_phase: 0,
            widget: id,
        });
        assert!(!value.signifier_guard_passes(&state));
        // flag != 0 bypasses the reduced guard set.
        assert!(host(1, 1).signifier_guard_passes(&state));
    }
}
