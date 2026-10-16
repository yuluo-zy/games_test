//! roof_shape.rs — 屋顶形状资源（WP6 恢复 + reviewer-C §8 机器码契约补入）。
//!
//! 源 SHA256（B skeleton）`resources/roofs/roof_shape.rs`；PDB 命名实体 8 个：
//! `Roof` / `RoofShapeParams` / `RoofType` / `RoofRidgeDirection` /
//! `GableRoofStyle` / `FlatRoofStyle` / `FlatRoofRimStyle` / `FlatRoofMaterial`。
//! `Roof` 在 ECS 查询中作**组件**（`Query<&Roof>` / `Option<&Roof>`，O）。
//!
//! 本轮（2026-10-16，reviewer-C 修复项 5）：14 个未入重建业务按 re-analyst
//! RE-ANALYSIS §8 机器码契约补入（is_flat/is_flat_w_no_rim/gable_sides/
//! shape_dims/rotation_yaw/ridge_length_01_from_rectangle_ratio/
//! roof_height_ws_from_wall_shape 等）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;
use glam::Vec3;

use crate::resources::walls::WallShapeSnapshot;

fn at(b: &[u8], offset: usize) -> f32 {
    f32::from_le_bytes(b[offset..offset + 4].try_into().unwrap())
}
fn clamp01(v: f32) -> f32 {
    if v < 0.0 { 0.0 } else if v > 1.0 { 1.0 } else { v }
}
fn sse_max(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}
fn sse_min(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}
fn smoothstep(t: f32) -> f32 {
    (3.0 - 2.0 * t) * t * t
}

/// 屋顶形状（O：ECS `&Roof`；观测 0x58 字节投影）。
#[derive(Component, Clone, Copy, Debug)]
pub struct Roof(pub [u8; 88]);

impl Default for Roof {
    fn default() -> Self {
        Self([0; 88])
    }
}

impl Roof {
    pub fn from_observed(bytes: [u8; 88]) -> Self {
        Self(bytes)
    }
    pub fn observed(&self) -> &[u8; 88] {
        &self.0
    }
    /// B `Roof::ty`（0x1408e3250）= c < height_01（含无序）；c = 0.1。
    pub fn ty(&self) -> u8 {
        let height = at(&self.0, 0x30);
        let c = f32::from_bits(0x3dcccccd);
        (c < height || c.is_nan() || height.is_nan()) as u8
    }
    /// B `Roof::is_flat`（0x1408e3380）= c >= height_01（有序）。
    pub fn is_flat(&self) -> bool {
        at(&self.0, 0x30) <= f32::from_bits(0x3dcccccd)
    }
    /// B `Roof::is_flat_w_no_rim`（0x1408e3390）= is_flat && rim@+0x3d != 0。
    pub fn is_flat_w_no_rim(&self) -> bool {
        self.is_flat() && self.0[0x3d] != 0
    }
    /// B `Roof::height`（0x1408e3260）：尺寸相关 smoothstep 上限
    /// （(8|9)→12 与 0.4→12 的 height_01 插值；0.1 平顶门控）。
    pub fn height(&self) -> f32 {
        let h = at(&self.0, 0x30);
        if !h.is_nan() && h <= 0.1 {
            return 0.0;
        }
        let (size, base) = if self.0[8] & 1 == 0 {
            ((at(&self.0, 0x14) + (-0.90000004)) / 7.1, 8.0)
        } else {
            let x = at(&self.0, 0x1c);
            let z = at(&self.0, 0x20);
            let greatest = if x.is_nan() { z } else { sse_max(z, x) };
            ((greatest + (-2.25)) / 12.75, 9.0)
        };
        let u = sse_min(1.0, sse_max(0.0, size / 0.4));
        let smooth = (3.0 - (u + u)) * (u * u);
        let cap = smooth * 12.0 + (1.0 - smooth) * base;
        assert!(cap >= 0.0, "original height cap assertion");
        let wanted = h * 12.0 + (1.0 - h) * 0.4;
        sse_min(cap, sse_max(0.0, wanted))
    }
    pub fn pivot_3d(&self) -> Vec3 {
        let tag = u32::from_le_bytes(self.0[8..12].try_into().unwrap()) as usize;
        assert!(tag < 2, "unsupported Roof enclosure");
        Vec3::new(
            at(&self.0, 12 + tag * 8),
            at(&self.0, 0x4c),
            at(&self.0, 16 + tag * 8),
        )
    }
    /// B `is_gable`（0x1408e33b0）= height_01 > 0.1 && ridge_length_01 == 1.0。
    pub fn is_gable(&self) -> bool {
        at(&self.0, 0x30) > f32::from_bits(0x3dcccccd) && at(&self.0, 0x34) == 1.0
    }
    /// B `gable_sides`（0x1408e33e0）：ridge==1.0 且 height>0.1 →
    /// {1, rim@+0x3c, rim|2}，否则 {0}。
    pub fn gable_sides(&self) -> Option<[u8; 2]> {
        self.is_gable().then_some([self.0[0x3c], self.0[0x3c] | 2])
    }
    /// B `shape_dims`（0x1408e3800/0x1408e3820 链）：圆 {2r, 2r}；矩 {d1, d2}。
    pub fn shape_dims(&self) -> [f32; 2] {
        if self.0[8] & 1 == 0 {
            let diameter = at(&self.0, 0x14) + at(&self.0, 0x14);
            [diameter; 2]
        } else {
            [at(&self.0, 0x1c), at(&self.0, 0x20)]
        }
    }
    /// B `rotation_yaw_radians`（0x1408e3b10）：circle atan2(−sin,cos)；rect −π/2−angle。
    pub fn rotation_yaw_radians(&self) -> f32 {
        if self.0[8] & 1 == 0 {
            let (sin, cos) = (at(&self.0, 0x10), at(&self.0, 0x0c));
            f32::atan2(-sin, cos)
        } else {
            let dims = self.shape_dims();
            -std::f32::consts::FRAC_PI_2 - f32::atan2(dims[1], dims[0])
        }
    }
    /// B `profile_curve_normalized`（0x1408e3420）：20 精确原采样坐标。
    pub fn profile_curve_normalized(&self) -> [[f32; 2]; 20] {
        const U: [u32; 20] = [
            0, 0x3d579436, 0x3dd79436, 0x3e21af28, 0x3e579436, 0x3e86bca2, 0x3ea1af28,
            0x3ebca1af, 0x3ed79436, 0x3ef286bd, 0x3f06bca2, 0x3f1435e5, 0x3f21af28,
            0x3f2f286c, 0x3f3ca1af, 0x3f4a1af3, 0x3f579436, 0x3f650d79, 0x3f7286bd,
            0x3f800000,
        ];
        let profile = at(&self.0, 0x2c);
        let exp = 1.5 * profile + (1.0 - profile) * 0.5;
        std::array::from_fn(|i| {
            let u = f32::from_bits(U[i]);
            [u, if i == 19 { 1.0 } else { u.powf(exp) }]
        })
    }
    /// B `roof_tip_offset_xz`（0x1408e3780）：tip·dim·0.5（矩形旋转后）。
    pub fn roof_tip_offset_xz(&self) -> [f32; 2] {
        if self.0[8] & 1 == 0 {
            [0.0, 0.0]
        } else {
            let p = [
                at(&self.0, 0x24) * at(&self.0, 0x1c) * 0.5,
                at(&self.0, 0x28) * at(&self.0, 0x20) * 0.5,
            ];
            let yaw = self.rotation_yaw_radians();
            [
                p[0] * yaw.cos() - p[1] * yaw.sin(),
                p[0] * yaw.sin() + p[1] * yaw.cos(),
            ]
        }
    }
}

/// 屋顶形状参数（O：命名实体；B 序列化 28B 布局 = workspace
/// `RoofParameterBlock` 同构）。
#[derive(Clone, Copy, Debug, Default)]
pub struct RoofShapeParams(pub [u8; 28]);

impl RoofShapeParams {
    pub fn from_observed(bytes: [u8; 28]) -> Self {
        Self(bytes)
    }
    /// O：从墙形状快照构造（height 01 由墙形状 → 屋顶高 01 链）。
    pub fn from_wall_shape(s: &WallShapeSnapshot) -> Self {
        let mut bytes = [0; 28];
        let h = roof_height_01_from_wall_shape(s);
        bytes[12..16].copy_from_slice(&h.to_le_bytes());
        Self(bytes)
    }
    /// B `RoofShapeParams::eave_length_ws`（0x1408e3c30）。
    pub fn eave_length_ws(&self) -> f32 {
        let profile = at(&self.0, 8);
        let eave = at(&self.0, 20);
        (profile.sqrt() + (1.0 - profile.sqrt()) * 0.5) * 1.5 * eave
    }
    /// B `height_ws`：参数块 → 世界高（height 链同构）。
    pub fn height_ws(&self) -> f32 {
        let h = at(&self.0, 12);
        if h <= 0.1 {
            0.0
        } else {
            h * 12.0 + (1.0 - h) * 0.4
        }
    }
}

/// O：`roof_height_01_from_wall_shape`（0x1408e3c70 的 01 变体；clamp 层）。
/// 形状快照 28B：tag@0（bit0=rect）、circle 半径@0x14、rect 尺寸@0x1c/0x20。
pub fn roof_height_01_from_wall_shape(s: &WallShapeSnapshot) -> f32 {
    let bytes = s.bytes();
    let (max, circle) = if bytes[0] & 1 == 0 {
        (at(bytes, 0x14) * 2.0, true)
    } else {
        (
            sse_max(at(bytes, 0x1c), at(bytes, 0x20)),
            false,
        )
    };
    let t = if circle {
        (max - 0.9) / 7.1
    } else {
        (max - 2.25) / 12.75
    };
    clamp01(t)
}

/// B `roof_height_ws_from_wall_shape`（0x1408e3c70）：circle lerp(4,10,t)/12；
/// rect lerp(3,6,t)/12。
pub fn roof_height_ws_from_wall_shape(s: &WallShapeSnapshot) -> f32 {
    let t = smoothstep(clamp01(roof_height_01_from_wall_shape(s)));
    if s.bytes()[0] & 1 == 0 {
        (4.0 + (10.0 - 4.0) * t) / 12.0
    } else {
        (3.0 + (6.0 - 3.0) * t) / 12.0
    }
}

/// B `ridge_length_01_from_rectangle_ratio`（0x1408e3b60）：
/// r=max(x/y,y/x)；t=clamp01((max(x,y)−2.25)/5.25)；s=smoothstep(t)；
/// 结果 = 1.5·s + r（截尾按 decomp）。
pub fn ridge_length_01_from_rectangle_ratio(dim_x: f32, dim_z: f32) -> f32 {
    let r = if dim_x > dim_z { dim_x / dim_z } else { dim_z / dim_x };
    let max = if dim_x > dim_z { dim_x } else { dim_z };
    let t = clamp01((max - 2.25) / 5.25);
    let s = smoothstep(t);
    1.5 * s + r
}
