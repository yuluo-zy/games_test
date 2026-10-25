//! `normal_body.rs` — `NormalCamera` body（0xb8 B，位于 `MainCamera + 0x100 / 0x1b8 / 0x390`）的已核对运算。
//!
//! 证据（B RNE 汇编，逐条核对）：
//! - `zoom` 0x140911280（反编译失败，按汇编手解）；
//! - `rotate_yaw_pitch` 0x140949cc0 + 求和辅助 0x141625db0；
//! - `constrain_to_be_above_terrain` 0x140949dc0。
//!
//! body 布局（O）：`rotations` Vec{cap@0,ptr@8,len@0x10}（元素 0x1c B）、`active: Option<ActiveRotation>`
//! （tag u32 @0x18，载荷 @0x1c）、`position` @0x38、`pending_translation` @0x44、`pitch` @0x50、`yaw` @0x54、
//! `pitch_min` @0x58、`pitch_max` @0x5c、`radial_min` @0x60、`radial_max` @0x64、`terrain_clearance` @0x68、
//! `loose_ground_projection` @0x6c。
//!
//! 纠错（相对此前文档）：
//! - 缩放系数是 `exp2(clamp(amount * 0.05, -1, 1))`（0x142930d70 = 0.05），不是 `-0.08`；
//! - `zoom` 的模式跳表（0x142aabc58）只有 6 项，Free/Walk/Butterfly 是空操作，不存在"默认路径"；
//! - 非 active 分支不是 `pending *= factor`，而是把 `(position + pending - anchor)` 缩放后再钳到最小距离，
//!   差值加回 `pending`；
//! - 最小距离表 0x142aab4a0 = `[3.0, 40.0]`（Normal/TopDown 与 TiltShift），平方表 0x142aab4a8 = `[9.0, 1600.0]`；
//! - 锚点水平距离必须 `< 130.0`（0x142a7b658），否则整次缩放被忽略。
//!
//! `update` 0x140949fe0（2339B）与 `translate_screen_space` 0x140949e70（365B）按完整 B RNE 恢复；每个 body 的持久状态仍由 MainCamera 拥有。
use glam::{Quat, Vec2, Vec3};

use ::utils::scalar::{clamp, max_f32};
use super::main_camera::CameraMode;

/// 0x1c B 的活动旋转记录（O：`load_parameters` 创建、`zoom`/`rotate_yaw_pitch` 读写）。
///
/// `unknown_*` 是创建时写入的 `+0x00`（0）、`+0x10`/`+0x14`（`-0.0`/`-0.0`）、`+0x18`（`-1.0`），语义未恢复。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActiveRotation {
    pub unknown_00: f32,
    /// `+0x04`：`zoom` 在 active 时写这里（目标臂长）。
    pub arm_length: f32,
    /// `+0x08`：俯仰增量，`rotate_yaw_pitch` 与求和辅助读写。
    pub pitch_delta: f32,
    /// `+0x0c`：偏航增量。
    pub yaw_delta: f32,
    pub unknown_10: f32,
    pub unknown_14: f32,
    pub unknown_18: f32,
}

/// `zoom` 的最小距离表（O：0x142aab4a0 / 0x142aab4a8）。
const ZOOM_MIN_DISTANCE: [f32; 2] = [3.0, 40.0];
const ZOOM_MIN_DISTANCE_SQ: [f32; 2] = [9.0, 1600.0];
/// 锚点水平距离上限（O：0x142a7b658）。
const ZOOM_ANCHOR_RADIUS: f32 = 130.0;
/// 缩放输入系数与指数范围（O：0x142930d70、0x14295a0e4、0x142925904）。
const ZOOM_AMOUNT_SCALE: f32 = 0.05;

/// body 类别：决定 `zoom` 用哪一档最小距离。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZoomBody {
    Normal,
    TiltShift,
    TopDown,
}

impl ZoomBody {
    /// `zoom` 的模式跳表（O，0x142aabc58，6 项）：0→Normal(+0x100)、1→TiltShift(+0x1b8)、4→TopDown(+0x390)，
    /// 2/3/5 → 无操作。其余模式字节在原版里读到的是相邻表（不会发生），这里同样返回 `None`。
    pub fn for_mode(mode: CameraMode) -> Option<Self> {
        match mode.0 {
            0 => Some(Self::Normal),
            1 => Some(Self::TiltShift),
            4 => Some(Self::TopDown),
            _ => None,
        }
    }

    fn table_index(self) -> usize {
        usize::from(self == Self::TiltShift)
    }
}

/// 原版 `f32::clamp` 断言（0x140d8b480）：`radial_max < min_distance`（含 NaN）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZoomClampPanic {
    pub min: f32,
    pub max: f32,
}

impl core::fmt::Display for ZoomClampPanic {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "min > max, or either was NaN. min = {:?}, max = {:?}",
            self.min, self.max
        )
    }
}

impl std::error::Error for ZoomClampPanic {}

/// body 的已核对字段。
#[derive(Clone, Debug, PartialEq)]
pub struct NormalCameraBody {
    pub rotations: Vec<ActiveRotation>,
    pub active: Option<ActiveRotation>,
    pub position: Vec3,
    pub pending_translation: Vec3,
    pub pitch: f32,
    pub yaw: f32,
    pub pitch_min: f32,
    pub pitch_max: f32,
    pub radial_min: f32,
    pub radial_max: f32,
    pub terrain_clearance: f32,
    pub loose_ground_projection: f32,
}

fn non_finite(v: f32) -> bool {
    v.to_bits() & 0x7fff_ffff > 0x7f7f_ffff
}

impl NormalCameraBody {
    /// `zoom`（0x140911280）。`amount` 或 `anchor` 含非有限值、或锚点水平距离 `>= 130` 时无操作。
    pub fn zoom(
        &mut self,
        which: ZoomBody,
        amount: f32,
        anchor: Vec3,
    ) -> Result<(), ZoomClampPanic> {
        if non_finite(amount)
            || non_finite(anchor.x)
            || non_finite(anchor.y)
            || non_finite(anchor.z)
        {
            return Ok(());
        }
        let horizontal = (anchor.z * anchor.z + anchor.x * anchor.x).sqrt();
        if horizontal.is_nan() || horizontal >= ZOOM_ANCHOR_RADIUS {
            return Ok(());
        }
        let t = which.table_index();
        let lower = ZOOM_MIN_DISTANCE[t];
        let exponent = clamp(amount * ZOOM_AMOUNT_SCALE, -1.0, 1.0);

        if let Some(active) = self.active.as_mut() {
            let radial_max = self.radial_max;
            if radial_max.is_nan() || radial_max < lower {
                return Err(ZoomClampPanic {
                    min: lower,
                    max: radial_max,
                });
            }
            let factor = exponent.exp2();
            let target = clamp(factor * active.arm_length, lower, radial_max);
            active.arm_length = target;
            return Ok(());
        }

        let p = self.pending_translation;
        let offset_x = (self.position.x + p.x) - anchor.x;
        let offset_y = (self.position.y + p.y) - anchor.y;
        let offset_z = (self.position.z + p.z) - anchor.z;
        let factor = exponent.exp2();
        let (mut sx, mut sy, mut sz) = (factor * offset_x, factor * offset_y, factor * offset_z);
        let len_sq = sz * sz + (sy * sy + sx * sx);
        if ZOOM_MIN_DISTANCE_SQ[t] > len_sq {
            let len = len_sq.sqrt();
            sx = (sx / len) * lower;
            sy = (sy / len) * lower;
            sz = (sz / len) * lower;
        }
        self.pending_translation = Vec3::new(
            p.x + (sx - offset_x),
            p.y + (sy - offset_y),
            p.z + (sz - offset_z),
        );
        Ok(())
    }

    /// 俯仰增量之和（O：0x141625db0）：初值 `-0.0`，依次加各 rotation 的 `+8`，最后加 active 的 `+8`。
    fn pitch_delta_sum(&self) -> f32 {
        let mut sum = -0.0_f32;
        for rotation in &self.rotations {
            sum += rotation.pitch_delta;
        }
        if let Some(active) = &self.active {
            sum += active.pitch_delta;
        }
        sum
    }

    /// `rotate_yaw_pitch`（0x140949cc0）：只在有 active 旋转时起作用。
    ///
    /// `base = pitch + Σ pitch_delta`；`bounded = clamp(-(pitch_delta_in + base), pitch_min, pitch_max)`；
    /// `active.pitch_delta += -bounded - base`、`active.yaw_delta += yaw_delta_in`。
    /// 两次钳制按原版的 NaN 语义：先对 `pitch_max` 取小（NaN 取 `pitch_max`），再对 `pitch_min` 取大（NaN 取 `pitch_min`）。
    pub fn rotate_yaw_pitch(&mut self, yaw_delta: f32, pitch_delta: f32) {
        let sum = self.pitch_delta_sum();
        let Some(active) = self.active.as_mut() else {
            return;
        };
        let base = self.pitch + sum;
        let wanted = -(pitch_delta + base);
        let upper = if wanted.is_nan() || wanted > self.pitch_max {
            self.pitch_max
        } else {
            wanted
        };
        let bounded = if upper.is_nan() || self.pitch_min > upper {
            self.pitch_min
        } else {
            upper
        };
        active.pitch_delta += (-bounded) - base;
        active.yaw_delta += yaw_delta;
    }

    /// `constrain_to_be_above_terrain`（0x140949dc0）：`terrain(x, z)` 取 `(pos.x + pending.x, pos.z + pending.z)` 处地形高度。
    ///
    /// `pending.y = max(terrain + clearance, pos.y + pending.y) - pos.y`（`pos.y + pending.y` 为 NaN 时取地形值）；
    /// `pending.x/z` 做 `(p + pos) - pos` 的浮点往返（保持逐位一致）。
    pub fn constrain_to_be_above_terrain(&mut self, terrain: impl FnOnce(f32, f32) -> f32) {
        let pos = self.position;
        let pending = self.pending_translation;
        let z_sum = pending.z + pos.z;
        let x_sum = pending.x + pos.x;
        let y_sum = pos.y + pending.y;
        let floor = terrain(x_sum, z_sum) + self.terrain_clearance;
        let y = if y_sum.is_nan() {
            floor
        } else {
            max_f32(floor, y_sum)
        };
        self.pending_translation = Vec3::new(x_sum - pos.x, y - pos.y, z_sum - pos.z);
    }
}

/// Native YXZ Euler construction from stored degrees. Keep the exact original
/// float scale; do not reinterpret these angles as radians.
fn body_rotation(yaw: f32, pitch: f32) -> Quat {
    let scale = super::main_camera::ORBIT_ANGLE_SCALE;
    Quat::from_rotation_y(yaw * scale) * Quat::from_rotation_x(pitch * scale)
}

fn original_xz_bounds(point: Vec2, extent: f32, extent_squared: f32, demo: bool) -> Vec2 {
    if demo {
        let squared = point.x * point.x + point.y * point.y;
        if extent_squared * 4.0 < squared {
            (point / squared.sqrt()) * (extent * 2.0)
        } else {
            point
        }
    } else {
        // Original max/min vector sequence explicitly maps NaN to the lower
        // bound, then allows up to one extent beyond the clamped rectangle.
        let clamp = |value: f32| {
            if value.is_nan() {
                -extent
            } else {
                clamp(value, -extent, extent)
            }
        };
        let clamped = Vec2::new(clamp(point.x), clamp(point.y));
        let excess = point - clamped;
        let squared = excess.x * excess.x + excess.y * excess.y;
        if extent_squared < squared {
            clamped + excess * (extent / squared.sqrt())
        } else {
            point
        }
    }
}

impl NormalCameraBody {
    /// B normal::translate_screen_space@140949e70. The movement has already
    /// been formed by the outer input controller. This body scales it by the
    /// previous matrix eye's ground-plane intersection distance, then yaws it.
    pub fn translate_screen_space(
        &mut self,
        movement: Vec3,
        matrices: utils::camera::CameraMatrices,
    ) {
        let height = matrices.eye_position().y;
        let direction_y = matrices.eye_direction().y;
        let scale = max_f32(height / -direction_y, 1.0) / 50.0;
        let rotation = Quat::from_rotation_y(self.yaw * super::main_camera::ORBIT_ANGLE_SCALE);
        self.pending_translation += (rotation * movement) * scale;
    }

    /// B main_camera_update@14091340f..458 produces this exact active orbit.
    pub fn begin_rotation(&mut self, anchor: Vec3, direction: Vec3, eye: Vec3) {
        let offset = eye - anchor;
        let length = (offset.z * offset.z + (offset.y * offset.y + offset.x * offset.x)).sqrt();
        self.active = Some(ActiveRotation {
            unknown_00: length,
            arm_length: length,
            pitch_delta: 0.0,
            yaw_delta: 0.0,
            unknown_10: direction.x,
            unknown_14: direction.y,
            unknown_18: direction.z,
        });
    }

    /// Original main-camera release transfers the existing orbit to the
    /// backlog. It does not finish the interpolation immediately.
    pub fn finish_rotation(&mut self) {
        if let Some(active) = self.active.take() {
            self.rotations.push(active);
        }
    }

    /// Full B normal::update@140949fe0. Includes eventual radial cap, normal
    /// and loose ground projections, soft rectangle/circular demo bounds,
    /// per-orbit smoothing, radial arm interpolation, queued translation and
    /// completed-backlog compaction. Negative dt is a pose-only read.
    pub fn update(&mut self, dt: f32, demo: bool) -> (Vec3, Quat) {
        if dt < 0.0 {
            return (self.position, body_rotation(self.yaw, self.pitch));
        }
        let residual = (dt * -8.0).exp();
        let alpha = 1.0 - residual;
        let target = self.position + self.pending_translation;
        let squared = target.z * target.z + (target.y * target.y + target.x * target.x);
        if self.radial_max * self.radial_max < squared {
            self.pending_translation += (target / squared.sqrt()) * self.radial_max - target;
        }
        let forward = body_rotation(self.yaw, self.pitch) * Vec3::NEG_Z;
        let goal = if self.loose_ground_projection != 0.0 {
            let denominator = max_f32(-forward.y, f32::from_bits(0x3e19_999a));
            let distance = max_f32(
                (self.position.y - 10.0) / denominator,
                f32::from_bits(0x4140_6667),
            );
            let back = Vec2::new(-forward.x, -forward.z) * distance;
            let point = Vec2::new(self.position.x, self.position.z) - back;
            let bounded = original_xz_bounds(
                point,
                f32::from_bits(0x4205_e666),
                f32::from_bits(0x448c_126b),
                demo,
            );
            let point = bounded + back;
            let candidate = Vec3::new(point.x, self.position.y, point.y);
            let squared =
                candidate.z * candidate.z + (candidate.y * candidate.y + candidate.x * candidate.x);
            if self.radial_min * self.radial_min < squared {
                (candidate / squared.sqrt()) * self.radial_min
            } else {
                candidate
            }
        } else {
            let forward_offset =
                Vec2::new(forward.x, forward.z) * f32::from_bits(0x42b3_6666) * 0.2;
            let point = Vec2::new(self.position.x, self.position.z) + forward_offset;
            let bounded = original_xz_bounds(
                point,
                f32::from_bits(0x4233_6666),
                f32::from_bits(0x44fb_70b7),
                demo,
            );
            let point = bounded - forward_offset;
            Vec3::new(point.x, self.position.y, point.y)
        };
        self.position += (goal - self.position) * alpha;
        for rotation in self.rotations.iter_mut().chain(self.active.iter_mut()) {
            let pitch_delta = alpha * rotation.pitch_delta;
            let yaw_delta = alpha * rotation.yaw_delta;
            rotation.pitch_delta -= pitch_delta;
            rotation.yaw_delta -= yaw_delta;
            let direction = Vec3::new(
                rotation.unknown_10,
                rotation.unknown_14,
                rotation.unknown_18,
            );
            let old_arm = body_rotation(self.yaw, self.pitch) * (-direction * rotation.unknown_00);
            rotation.unknown_00 = rotation.arm_length * alpha + rotation.unknown_00 * (1.0 - alpha);
            self.pitch += pitch_delta;
            self.yaw += yaw_delta;
            let new_arm = body_rotation(self.yaw, self.pitch) * (-direction * rotation.unknown_00);
            self.position += new_arm - old_arm;
        }
        let translation = self.pending_translation * alpha;
        self.pending_translation -= translation;
        self.position += translation;
        self.position.y = max_f32(self.position.y, 1.0);
        self.rotations.retain(|rotation| {
            // Native compares threshold > abs three times; unordered keeps
            // the record, rather than deleting a corrupted/NaN remainder.
            !(rotation.pitch_delta.abs() < 0.001
                && rotation.yaw_delta.abs() < 0.001
                && (rotation.unknown_00 - rotation.arm_length).abs()
                    / max_f32(rotation.arm_length.abs(), 1.0e-20)
                    < 0.001)
        });
        (self.position, body_rotation(self.yaw, self.pitch))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rot(pitch_delta: f32) -> ActiveRotation {
        ActiveRotation {
            unknown_00: 0.0,
            arm_length: 0.0,
            pitch_delta,
            yaw_delta: 0.0,
            unknown_10: -0.0,
            unknown_14: -0.0,
            unknown_18: -1.0,
        }
    }

    fn body() -> NormalCameraBody {
        NormalCameraBody {
            rotations: vec![],
            active: None,
            position: Vec3::new(10.0, 20.0, 30.0),
            pending_translation: Vec3::ZERO,
            pitch: 0.0,
            yaw: 0.0,
            pitch_min: -1.2,
            pitch_max: 0.3,
            radial_min: 0.0,
            radial_max: 100.0,
            terrain_clearance: 2.0,
            loose_ground_projection: 0.0,
        }
    }

    #[test]
    fn zoom_body_follows_the_six_entry_jump_table() {
        let b = |m| ZoomBody::for_mode(CameraMode(m));
        assert_eq!(b(0), Some(ZoomBody::Normal));
        assert_eq!(b(1), Some(ZoomBody::TiltShift));
        assert_eq!(b(4), Some(ZoomBody::TopDown));
        for m in [2, 3, 5, 6, 12] {
            assert_eq!(b(m), None, "mode {m}");
        }
    }

    #[test]
    fn zoom_ignores_non_finite_input_and_far_anchors() {
        let mut b = body();
        let before = b.clone();
        b.zoom(ZoomBody::Normal, f32::NAN, Vec3::ZERO).unwrap();
        b.zoom(ZoomBody::Normal, 1.0, Vec3::new(f32::INFINITY, 0.0, 0.0))
            .unwrap();
        b.zoom(ZoomBody::Normal, 1.0, Vec3::new(130.0, 0.0, 0.0))
            .unwrap();
        b.zoom(ZoomBody::Normal, 1.0, Vec3::new(100.0, 0.0, 100.0))
            .unwrap();
        assert_eq!(b, before);
    }

    #[test]
    fn zoom_scales_the_offset_from_the_anchor_by_two_to_the_clamped_exponent() {
        let mut b = body();
        b.position = Vec3::new(8.0, 0.0, 0.0);
        b.zoom(ZoomBody::Normal, 20.0, Vec3::ZERO).unwrap();
        // amount * 0.05 = 1.0 -> factor 2: offset 8 -> 16, so pending gains 8.
        assert_eq!(b.pending_translation, Vec3::new(8.0, 0.0, 0.0));
        let mut b = body();
        b.position = Vec3::new(8.0, 0.0, 0.0);
        b.zoom(ZoomBody::Normal, -1000.0, Vec3::ZERO).unwrap();
        // exponent clamped to -1 -> factor 0.5: offset 8 -> 4.
        assert_eq!(b.pending_translation, Vec3::new(-4.0, 0.0, 0.0));
    }

    #[test]
    fn zoom_never_lets_the_offset_shrink_below_the_minimum_distance() {
        let mut b = body();
        b.position = Vec3::new(4.0, 0.0, 0.0);
        b.zoom(ZoomBody::Normal, -1000.0, Vec3::ZERO).unwrap();
        // factor 0.5 -> length 2 < 3, rescaled to 3: pending = 3 - 4.
        assert_eq!(b.pending_translation, Vec3::new(-1.0, 0.0, 0.0));
        let mut t = body();
        t.position = Vec3::new(50.0, 0.0, 0.0);
        t.zoom(ZoomBody::TiltShift, -1000.0, Vec3::ZERO).unwrap();
        // factor 0.5 -> length 25 < 40 (tilt-shift minimum), rescaled to 40: pending = 40 - 50.
        assert_eq!(t.pending_translation, Vec3::new(-10.0, 0.0, 0.0));
    }

    #[test]
    fn zoom_with_active_rotation_clamps_the_arm_length() {
        let mut b = body();
        b.active = Some(ActiveRotation {
            arm_length: 10.0,
            ..rot(0.0)
        });
        b.zoom(ZoomBody::Normal, 20.0, Vec3::ZERO).unwrap();
        assert_eq!(b.active.unwrap().arm_length, 20.0);
        b.zoom(ZoomBody::Normal, -1000.0, Vec3::ZERO).unwrap();
        b.zoom(ZoomBody::Normal, -1000.0, Vec3::ZERO).unwrap();
        b.zoom(ZoomBody::Normal, -1000.0, Vec3::ZERO).unwrap();
        assert_eq!(b.active.unwrap().arm_length, 3.0);
        b.radial_max = 25.0;
        b.active = Some(ActiveRotation {
            arm_length: 20.0,
            ..rot(0.0)
        });
        b.zoom(ZoomBody::Normal, 20.0, Vec3::ZERO).unwrap();
        assert_eq!(b.active.unwrap().arm_length, 25.0);
        assert_eq!(b.pending_translation, Vec3::ZERO);
    }

    #[test]
    fn zoom_with_radial_max_below_the_minimum_is_the_clamp_panic() {
        let mut b = body();
        b.active = Some(rot(0.0));
        b.radial_max = 2.0;
        assert_eq!(
            b.zoom(ZoomBody::Normal, 1.0, Vec3::ZERO),
            Err(ZoomClampPanic { min: 3.0, max: 2.0 })
        );
        b.radial_max = f32::NAN;
        assert!(b.zoom(ZoomBody::Normal, 1.0, Vec3::ZERO).is_err());
    }

    #[test]
    fn rotate_without_active_rotation_does_nothing() {
        let mut b = body();
        b.rotations.push(rot(0.5));
        let before = b.clone();
        b.rotate_yaw_pitch(1.0, 1.0);
        assert_eq!(b, before);
    }

    #[test]
    fn rotate_accumulates_yaw_and_clamps_the_total_pitch() {
        let mut b = body();
        b.active = Some(rot(0.0));
        b.rotate_yaw_pitch(0.25, -0.1);
        let a = b.active.unwrap();
        assert_eq!(a.yaw_delta, 0.25);
        // wanted = -(−0.1 + 0) = 0.1 within [-1.2, 0.3]; pitch_delta += -0.1 - 0 + 0.
        assert_eq!(a.pitch_delta, -0.1);
        b.rotate_yaw_pitch(0.25, 5.0);
        let a = b.active.unwrap();
        assert_eq!(a.yaw_delta, 0.5);
        // wanted = -(5 + base) far below pitch_min (-1.2): bounded = -1.2 so total pitch lands at +1.2.
        let total = b.pitch + b.pitch_delta_sum();
        assert!((total - 1.2).abs() < 1e-6);
    }

    #[test]
    fn rotate_counts_every_pending_rotation_in_the_base() {
        let mut b = body();
        b.rotations = vec![rot(0.1), rot(0.2)];
        b.active = Some(rot(0.3));
        b.pitch = 0.05;
        b.rotate_yaw_pitch(0.0, 0.0);
        // base = 0.05 + 0.1 + 0.2 + 0.3 = 0.65; wanted = -0.65 inside [-1.2, 0.3] -> no net change.
        assert!((b.active.unwrap().pitch_delta - 0.3).abs() < 1e-6);
    }

    #[test]
    fn constrain_lifts_the_camera_above_terrain_plus_clearance() {
        let mut b = body();
        b.position = Vec3::new(1.0, 5.0, 2.0);
        b.pending_translation = Vec3::new(0.5, -4.0, 0.5);
        let mut asked = None;
        b.constrain_to_be_above_terrain(|x, z| {
            asked = Some((x, z));
            10.0
        });
        assert_eq!(asked, Some((1.5, 2.5)));
        // floor = 10 + 2 = 12 > pos.y + pending.y = 1  ->  pending.y = 12 - 5.
        assert_eq!(b.pending_translation, Vec3::new(0.5, 7.0, 0.5));
    }

    #[test]
    fn constrain_leaves_a_high_camera_alone() {
        let mut b = body();
        b.pending_translation = Vec3::new(0.0, 1.5, 0.0);
        b.constrain_to_be_above_terrain(|_, _| 0.0);
        assert_eq!(b.pending_translation, Vec3::new(0.0, 1.5, 0.0));
    }
}

#[cfg(test)]
mod original_update_tests {
    use super::*;
    fn body_at(position: Vec3) -> NormalCameraBody {
        NormalCameraBody {
            rotations: Vec::new(),
            active: None,
            position,
            pending_translation: Vec3::ZERO,
            pitch: 0.0,
            yaw: 0.0,
            pitch_min: 5.0,
            pitch_max: 90.0,
            radial_min: 300.0,
            radial_max: 1000.0,
            terrain_clearance: 0.75,
            loose_ground_projection: 0.0,
        }
    }
    #[test]
    fn native_soft_bounds_preserve_the_near_edge_and_only_limit_large_excess() {
        let mut near = body_at(Vec3::new(57.0, 20.0, 0.0));
        near.update(1000.0, false);
        assert_eq!(near.position.x, 57.0);
        let mut far = body_at(Vec3::new(200.0, 20.0, 0.0));
        far.update(1000.0, false);
        assert!((far.position.x - f32::from_bits(0x42b3_6666)).abs() < 1.0e-5);
    }
    #[test]
    fn native_orbit_updates_both_arm_lengths_and_preserves_the_pivot() {
        let mut body = body_at(Vec3::new(0.0, 20.0, 10.0));
        body.begin_rotation(Vec3::new(0.0, 20.0, 0.0), Vec3::NEG_Z, body.position);
        body.active.as_mut().unwrap().yaw_delta = 90.0;
        body.active.as_mut().unwrap().arm_length = 20.0;
        body.finish_rotation();
        let (position, _) = body.update(1000.0, false);
        assert!((position - Vec3::new(20.0, 20.0, 0.0)).length() < 1.0e-4);
        assert_eq!(body.yaw, 90.0);
        assert!(body.rotations.is_empty());
    }
    #[test]
    fn negative_delta_reads_the_pose_without_consuming_pending_movement() {
        let mut body = body_at(Vec3::new(200.0, 20.0, 0.0));
        body.pending_translation = Vec3::new(1.0, 2.0, 3.0);
        let before = body.clone();
        let (position, _) = body.update(-1.0, false);
        assert_eq!(position, before.position);
        assert_eq!(body, before);
    }
}
