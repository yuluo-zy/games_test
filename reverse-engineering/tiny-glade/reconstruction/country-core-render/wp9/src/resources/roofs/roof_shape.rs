//! roof_shape.rs — Roof / RoofShapeParams 及形状/剖面函数族（WP9）。
//!
//! PDB 骨架 `resources/roofs/roof_shape.rs`：命名实体 `Roof` / `RoofShapeParams` /
//! `RoofType` / `RoofRidgeDirection` / `GableRoofStyle` / `FlatRoofStyle` /
//! `FlatRoofRimStyle` / `FlatRoofMaterial`。函数清单（compiled_instances rva → VA）见
//! `evidence/roof-domain-dump.txt` 与 `evidence/rva-map.txt`。
//!
//! 数值核心 = 已机器码验证的 `reconstruction/roof/{surface,tiles,roof_state,
//! parameters}.rs`（本文件内联等价；保留每处证据引用）。数学复用 glam 0.29.3，
//! 曲线复用 `utils::curve`（游戏自有；此处以 `Curve2` 等价表示）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;

use crate::resources::roof_shape_curve::Curve2;
use crate::typed_gap::Gap;

// ---------------------------------------------------------------------------
// 布局常量（O：`Roof::new` 0x1408e31c0 / roof_state.rs `new_observed`）
// ---------------------------------------------------------------------------

/// Roof 88 字节组件布局中的字段偏移（O）。字段名多数未从 PDB 恢复 → 结构体以
/// `raw: [u8; 88]` 保存 + 具名访问器（与 prior work 一致，不臆造字段名）。
pub mod layout {
    pub const ID: usize = 0x00; // u64（低 32 位 = roof_id）
    pub const TYPE_BIT: usize = 0x08; // bit0：0=圆，1=矩形
    pub const CENTER_XZ: usize = 0x0c; // [f32;2]
    pub const SIZE_OR_ROT: usize = 0x14; // 圆半径 / 矩形长边；圆旋转字在 0x18
    pub const RECT_DIMS: usize = 0x1c; // 矩形 [f32;2]
    pub const PARAMS: usize = 0x24; // RoofShapeParams（28B 参数块）
    pub const BASE_Y: usize = 0x4c; // f32
    pub const BASE_H: usize = 0x50; // f32
    pub const SPECIAL_MODE: usize = 0x54; // byte（半木/特殊标记）
    pub const SIZE: usize = 0x58; // 88
}

// ---------------------------------------------------------------------------
// 枚举（O 判别，U 值编号）
// ---------------------------------------------------------------------------

/// RoofType：`Roof::ty`（0x1408e3250）按 `height_01 > 0.1 (NaN→true)` 判别。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoofType {
    Flat,
    Gable,
}

/// 脊线方向：参数块 byte@24（0=Width，1=Length；`RoofRidgeDirection`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoofRidgeDirection {
    Width = 0,
    Length = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GableRoofStyle {
    /// 判别/值编号 U（命名实体）。`is_stepped` 0x140833230 / `is_overhang` 0x1408c5680。
    Unknown(Gap),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlatRoofStyle {
    /// 命名实体。`is_wooden` 0x1408e31a0 / `pattern_id`。
    Unknown(Gap),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlatRoofRimStyle {
    /// 命名实体；rim 字节在参数块 0x25，判别 U。
    Unknown(Gap),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlatRoofMaterial {
    /// 命名实体；判别 U。
    Unknown(Gap),
}

// ---------------------------------------------------------------------------
// RoofShapeParams（O：PDB observed 26B + parameters.rs 28B 参数块）
// ---------------------------------------------------------------------------

/// 屋顶形状参数（Roof[0x24..0x40] 参数块）。
#[derive(Clone, Copy, Debug)]
pub struct RoofShapeParams {
    pub tip_offset_01: [f32; 2],
    pub profile_01: f32,
    pub height_01: f32,
    pub ridge_length_01: f32,
    pub eave_length_01: f32,
    pub ridge_dir: u8,
    pub rim: u8,
}

impl RoofShapeParams {
    pub fn from_roof(roof: &[u8; 88]) -> Self {
        let f = |o: usize| f32::from_le_bytes(roof[o..o + 4].try_into().unwrap());
        Self {
            tip_offset_01: [f(0x24), f(0x28)],
            profile_01: f(0x2c),
            height_01: f(0x30),
            ridge_length_01: f(0x34),
            eave_length_01: f(0x38),
            ridge_dir: roof[0x3c],
            rim: roof[0x3d],
        }
    }
    pub fn write_to_roof(&self, roof: &mut [u8; 88]) {
        let mut put = |o: usize, v: f32| roof[o..o + 4].copy_from_slice(&v.to_le_bytes());
        put(0x24, self.tip_offset_01[0]);
        put(0x28, self.tip_offset_01[1]);
        put(0x2c, self.profile_01);
        put(0x30, self.height_01);
        put(0x34, self.ridge_length_01);
        put(0x38, self.eave_length_01);
        roof[0x3c] = self.ridge_dir;
        roof[0x3d] = self.rim;
    }

    /// RoofShapeParams::eave_length_ws（O：0x1408e3c30 = parameters.rs 62B 原函数机器码验证）。
    pub fn eave_length_ws(&self) -> f32 {
        let eave = self.eave_length_01;
        let p = self.profile_01.sqrt();
        (p + (1.0 - p) * 0.5) * (1.5 * eave + (1.0 - eave) * 0.0)
    }

    /// RoofShapeParams::height_ws（I：与 roof_height_ws_from_wall_shape 共用 easing）。
    pub fn height_ws(&self) -> f32 {
        crate::resources::roof_shape_curve::roof_height_ws_from_wall_shape(self)
    }
}

// ---------------------------------------------------------------------------
// Roof（O：88B ECS 组件）
// ---------------------------------------------------------------------------

/// 屋顶组件（`Query<&Roof>` / `Option<&Roof>`）。88B 原始布局；访问器来自机器码验证。
#[derive(Component, Clone, Copy, Debug)]
#[repr(C)]
pub struct Roof(pub [u8; layout::SIZE]);

impl Default for Roof {
    fn default() -> Self {
        Self([0u8; layout::SIZE])
    }
}

impl Roof {
    pub fn new(
        id: u64,
        geometry: [u8; 28],
        params: RoofShapeParams,
        base_y: f32,
        base_h: f32,
        special: u8,
        tail: [u8; 12],
    ) -> Self {
        let mut out = [0u8; layout::SIZE];
        out[0..8].copy_from_slice(&id.to_le_bytes());
        out[8..0x24].copy_from_slice(&geometry);
        params.write_to_roof(&mut out);
        out[0x4c..0x50].copy_from_slice(&base_y.to_le_bytes());
        out[0x50..0x54].copy_from_slice(&base_h.to_le_bytes());
        out[0x54] = special;
        out[0x40..0x4c].copy_from_slice(&tail);
        // 0x55..0x58 原函数未写。
        Self(out)
    }

    pub fn is_rectangle(&self) -> bool {
        self.0[layout::TYPE_BIT] & 1 != 0
    }

    /// Roof::ty（O：0x1408e3250 / roof_state.rs `ty_observed`）。
    pub fn ty(&self) -> RoofType {
        let h = self.height_01();
        if h > 0.1 || h.is_nan() {
            RoofType::Gable
        } else {
            RoofType::Flat
        }
    }
    /// Roof::is_flat（O：0x1408e3380）。
    pub fn is_flat(&self) -> bool {
        self.ty() == RoofType::Flat
    }
    /// Roof::is_flat_w_no_rim（O：0x1408e3390；rim 语义 U）。
    pub fn is_flat_w_no_rim(&self) -> bool {
        self.is_flat() && self.0[0x3d] == 0
    }
    /// Roof::is_gable（O：0x1408e33b0 / roof_state.rs `is_gable_observed`）。
    pub fn is_gable(&self) -> bool {
        let h = self.height_01();
        (h > 0.1 || h.is_nan()) && self.ridge_length_01() == 1.0
    }

    /// Roof::pivot_3d（O：0x1408e3220 / edit_rules.rs `pivot`）。
    pub fn pivot_3d(&self) -> [f32; 3] {
        let tag = u32::from_le_bytes(self.0[8..12].try_into().unwrap()) as usize;
        [
            self.f32_at(layout::CENTER_XZ + tag * 8),
            self.f32_at(layout::BASE_Y),
            self.f32_at(0x10 + tag * 8),
        ]
    }

    /// Roof::height（O：0x1408e3260 / surface.rs `height` 全函数机器码验证）。
    pub fn height(&self) -> f32 {
        crate::resources::roof_shape_curve::height_impl(self)
    }

    /// Roof::shape_dims（O：0x1408e3800）。矩形返回 dims，圆返回半径对。
    pub fn shape_dims(&self) -> [f32; 2] {
        if self.is_rectangle() {
            [self.f32_at(0x1c), self.f32_at(0x20)]
        } else {
            [self.f32_at(0x14), self.f32_at(0x14)]
        }
    }

    /// Roof::gable_sides（O：0x1408e33e0）。
    pub fn gable_sides(&self) -> Option<[u8; 2]> {
        if self.is_gable() {
            Some([self.0[0x3c], self.0[0x3c] | 2])
        } else {
            None
        }
    }

    /// Roof::roof_tip_offset_xz（O：0x1408e3780 / tiles.rs `tip`）。圆返回局部，
    /// 矩形经 Rectangle2d 转世界（U 内部）。
    pub fn roof_tip_offset_xz(&self) -> [f32; 2] {
        if !self.is_rectangle() {
            let r = self.f32_at(0x14);
            let tip = [self.f32_at(0x24), self.f32_at(0x28)];
            [r * 2.0 * tip[0] * 0.5, r * 2.0 * tip[1] * 0.5]
        } else {
            // 矩形：orientation_mat2 转世界（tiles.rs `rectangular_context_from_observed`
            // local_tip_center；此处留 Gap 字段位）。
            [self.f32_at(0x24), self.f32_at(0x28)]
        }
    }

    /// Roof::rotation_yaw_radians（O：0x1408e3b10）。
    pub fn rotation_yaw_radians(&self) -> f32 {
        if self.is_rectangle() {
            (-self.f32_at(0x10)).atan2(self.f32_at(0x0c))
        } else {
            // 圆：utils::circular_f32::CircularF32::as_f32()（游戏自有 util，U）。
            self.f32_at(0x18)
        }
    }

    /// Roof::profile_curve_normalized（O：0x1408e3420 / surface.rs `profile_curve_normalized`）。
    pub fn profile_curve_normalized(&self) -> Curve2 {
        crate::resources::roof_shape_curve::profile_curve_normalized(self.profile_01())
    }

    /// Roof::get_roof_shape(t)（O：0x1408e4920 / surface.rs `get_roof_shape`）。
    pub fn get_roof_shape(&self, t: f32) -> RoofSection {
        crate::resources::roof_shape_curve::get_roof_shape(self, t)
    }

    /// Roof::roof_shape_at_intersection_y（O：0x1408e3820）。
    pub fn roof_shape_at_intersection_y(&self, y: f32) -> RoofSection {
        crate::resources::roof_shape_curve::roof_shape_at_intersection_y(self, y)
    }

    /// Roof::get_height_01_unclamped_from_ws（O：0x1408e4790）。
    pub fn get_height_01_unclamped_from_ws(&self) -> f32 {
        crate::resources::roof_shape_curve::get_height_01_unclamped_from_ws(self)
    }

    /// Roof::get_roof_shape_from_pos_ws_clamped（O：0x1408e4750）。
    pub fn get_roof_shape_from_pos_ws_clamped(&self) -> RoofSection {
        self.get_roof_shape(self.get_height_01_unclamped_from_ws())
    }

    // ---- 具名浮点访问器（O 偏移） ----
    pub fn roof_id(&self) -> u32 {
        u32::from_le_bytes(self.0[0..4].try_into().unwrap())
    }
    pub fn center_xz(&self) -> [f32; 2] {
        [self.f32_at(0x0c), self.f32_at(0x10)]
    }
    pub fn radius(&self) -> f32 {
        self.f32_at(0x14)
    }
    pub fn circle_rotation(&self) -> f32 {
        self.f32_at(0x18)
    }
    pub fn profile_01(&self) -> f32 {
        self.f32_at(0x2c)
    }
    pub fn height_01(&self) -> f32 {
        self.f32_at(0x30)
    }
    pub fn ridge_length_01(&self) -> f32 {
        self.f32_at(0x34)
    }
    pub fn eave_length_01(&self) -> f32 {
        self.f32_at(0x38)
    }
    pub fn base_y(&self) -> f32 {
        self.f32_at(0x4c)
    }
    pub fn special_mode(&self) -> u8 {
        self.0[0x54]
    }
    pub fn set_base_y(&mut self, v: f32) {
        self.0[0x4c..0x50].copy_from_slice(&v.to_le_bytes());
    }
    fn f32_at(&self, o: usize) -> f32 {
        f32::from_le_bytes(self.0[o..o + 4].try_into().unwrap())
    }
}

// ---------------------------------------------------------------------------
// 截面 / 剖面（O：28B 输出；surface.rs `RoofSection`）
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub enum RoofSection {
    Circle {
        frame: [f32; 2],
        radius: f32,
        word_0x18: u32,
    },
    Rectangle {
        frame: [f32; 4],
        dimensions: [f32; 2],
    },
    /// tag=2：y 在 base..base+height 之外（无截面）。
    Outside,
}

/// profile 指数：`1.5·p + (1-p)·0.5`（surface.rs `profile_exponent`）。
pub fn profile_exponent(profile: f32) -> f32 {
    1.5 * profile + (1.0 - profile) * 0.5
}

// ---------------------------------------------------------------------------
// 自由函数（PDB：roof_shape 模块）
// ---------------------------------------------------------------------------

/// ridge_length_01_from_rectangle_ratio（O：0x1408e3b60）。
pub fn ridge_length_01_from_rectangle_ratio(_dims: [f32; 2], _ratio: f32) -> f32 {
    // 矩形长宽比 → 脊线长度 01；正文简（decomp void()），数值 U。
    crate::typed_gap::gap_f32()
}

/// RoofShapeParams::from_wall_shape（I，inline）。
pub fn roof_shape_params_from_wall_shape(_wall: crate::typed_gap::Gap) -> RoofShapeParams {
    // 墙形状 → 参数块；字段映射 U。
    RoofShapeParams {
        tip_offset_01: [0.0; 2],
        profile_01: 0.0,
        height_01: 0.0,
        ridge_length_01: 0.0,
        eave_length_01: 0.0,
        ridge_dir: 0,
        rim: 0,
    }
}

/// roof_height_01_from_wall_shape（I，inline）。
pub fn roof_height_01_from_wall_shape(_wall: crate::typed_gap::Gap) -> f32 {
    crate::typed_gap::gap_f32()
}
