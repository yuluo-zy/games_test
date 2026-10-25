//! `roof_visual_shape.rs` — 屋顶可视形状查询族（B `roofs/roof_visual_shape.rs`，6 业务）。
//!
//! 前三个函数 PDB 归属 `roof_shape::Roof`，本文件按原文件布局放置：
//! - `Roof::get_height_01_unclamped_from_ws`（0x1408e4790，O 公式级，roofs-rt/ALGORITHMS §5.3）；
//! - `Roof::get_roof_shape_from_pos_ws_clamped`（0x1408e4750，O）；
//! - `Roof::get_roof_shape`（0x1408e4920，O = `crate::resources::roofs::geometry::surface::get_roof_shape`，3,004 案例 0 差异）；
//! - `get_roof_expanded_top_and_bottom`（0x140a880c0，O 反编译 `roof-tiles/0x140a880c0.c`）；
//! - `RoofBoundsDims::as_circle`（0x140a88200）/`as_rectangle`（0x140a88240，O 手解）。
//!
//! **RoofBoundsDims 布局修正**：`get_roof_expanded_top_and_bottom` 输出 = `{tag@0,
//! bottom@+4, top@+8}`——矩形 bottom/top 各是 Vec2，圆 bottom/top 各是半径（圆 top 恒 0.2）。
//! 旧章节把圆 +4/+8 记为"圆心/半径"是误读；`as_circle` 只是拷这两个 f32。
use crate::resources::roofs::{
    roof_noise::noise,
    roof_shape::Roof,
};
use glam::Vec3;
use ::utils::bytes::read_f32_le as at;
use ::utils::scalar::{clamp, clamp01, max_f32};

/// `RoofBoundsDims` 判别 tag（O：bit0 = circle）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RoofBoundsDims {
    /// 圆（+0 tag=0）：`bottom_radius` @+4 / `top_radius` @+8。
    Circle {
        bottom_radius: f32,
        top_radius: f32,
    },
    /// 矩形（+0 tag=1）：`bottom` @+4/+8 / `top` @+0xc/+0x10。
    Rectangle {
        bottom: [f32; 2],
        top: [f32; 2],
    },
}

impl RoofBoundsDims {
    /// `as_circle`（0x140a88200，O）：tag bit0==1 → panic；否则返回 {f32@+4, f32@+8}。
    pub fn as_circle(self) -> (f32, f32) {
        match self {
            Self::Circle {
                bottom_radius,
                top_radius,
            } => (bottom_radius, top_radius),
            Self::Rectangle { .. } => panic!("RoofBoundsDims::as_circle called on a rectangle"),
        }
    }

    /// `as_rectangle`（0x140a88240，O）：tag bit0==0 → panic；否则拷 16 字节矩形参数。
    pub fn as_rectangle(self) -> ([f32; 2], [f32; 2]) {
        match self {
            Self::Rectangle { bottom, top } => (bottom, top),
            Self::Circle { .. } => panic!("RoofBoundsDims::as_rectangle called on a circle"),
        }
    }
}

impl Roof {
    /// `get_roof_shape_from_pos_ws_clamped`（0x1408e4750，O）：
    /// `h01 = get_height_01_unclamped_from_ws(pos)`，clamp[0,1] 后调 `get_roof_shape(h01)`。
    pub fn get_roof_shape_from_pos_ws_clamped(&self, pos: Vec3) -> crate::resources::roofs::geometry::surface::RoofSection {
        let h01 = self.get_height_01_unclamped_from_ws(pos).clamp(0.0, 1.0);
        self.get_roof_shape(h01)
    }

    /// `get_height_01_unclamped_from_ws`（0x1408e4790，O 公式级，roofs-rt/ALGORITHMS §5.3）：
    /// `(pos.y + noise(pos.x·0.5, pos.z·0.5)·(−0.3) − floor(+0x4c)) / (cap − floor)`；
    /// `cap` 与 `Roof::height` 的 smoothstep 上限族同源。
    pub fn get_height_01_unclamped_from_ws(&self, pos: Vec3) -> f32 {
        let b = self.observed();
        let n = noise(pos.x * 0.5, pos.z * 0.5);
        let y = pos.y + n * -0.3;
        let height_01 = at(b, 0x30);
        let floor = at(b, 0x4c);
        let mut cap = 0.0;
        if f32::from_bits(0x3dcccccd) < height_01 {
            // reviewer-C 机器码裁决：circle 高度基 = **8.0**
            // （0x142934604 = 0x41000000；16.0 系旧读错值，roofs-rt
            // ALGORITHMS §8 已纠）。
            let (dim_n, base) = if b[8] & 1 == 0 {
                ((at(b, 0x14) + -0.9) / 7.1, 8.0)
            } else {
                let x = at(b, 0x1c);
                let z = at(b, 0x20);
                ((max_f32(x, z) + -2.25) / 12.75, 9.0)
            };
            let t = clamp01(dim_n / 0.4);
            let smooth = (3.0 - (t + t)) * t * t;
            let cap_base = smooth * 12.0 + (1.0 - smooth) * base;
            assert!(cap_base >= 0.0, "original height cap assertion");
            let cap_h = height_01 * 12.0 + (1.0 - height_01) * 0.4;
            cap = clamp(cap_h, 0.0, cap_base);
        }
        // 分子 (pos.y + noise·(−0.3)) − floor；分母 (cap + floor) − floor（保留原求值序）。
        (y - floor) / ((cap + floor) - floor)
    }

    /// `get_roof_shape`（0x1408e4920，O）= `crate::resources::roofs::geometry::surface::get_roof_shape`
    /// （q=1.5p+(1−p)0.5，t^(1/q) 混合屋檐/屋脊；矩→28B，圆→20B+保尾；3,004 案例 0 差异）。
    pub fn get_roof_shape(&self, t: f32) -> crate::resources::roofs::geometry::surface::RoofSection {
        crate::resources::roofs::geometry::surface::section(self.observed(), t)
    }
}

/// `get_roof_expanded_top_and_bottom`（0x140a880c0，O 反编译 + 手解）：
/// 屋檐外扩后的底/顶矩形或圆参数，喂瓦片/脊线生成。
/// 常量：0.28 屋檐加宽、eave_ws 系数 (sqrt(p)+(1−sqrt(p))·0.5)·1.5e、圆 1.2 倍 + 0.5 缩放、
/// 顶半径 0.2、脊内缩 (1−ridge)·0.1。
pub fn get_roof_expanded_top_and_bottom(roof: &Roof) -> RoofBoundsDims {
    let b = roof.observed();
    let profile = at(b, 0x2c);
    let eave = at(b, 0x38);
    let eave_ws = (profile.sqrt() + (1.0 - profile.sqrt()) * 0.5) * 1.5 * eave;
    if b[8] & 1 == 0 {
        let radius = at(b, 0x14);
        RoofBoundsDims::Circle {
            bottom_radius: ((eave_ws * 1.2 + 0.28) * 0.5) + radius,
            top_radius: 0.2,
        }
    } else {
        let dim_x = at(b, 0x1c);
        let dim_z = at(b, 0x20);
        let ridge = at(b, 0x34);
        let inset = (1.0 - ridge) * 0.1;
        let bottom = [eave_ws + (0.28 + dim_x), eave_ws + (0.28 + dim_z)];
        let top = if b[0x3c] == 0 {
            [dim_x * ridge + inset, 0.1]
        } else {
            [0.1, dim_z * ridge + inset]
        };
        RoofBoundsDims::Rectangle { bottom, top }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::roofs::roof_shape::Roof;

    fn roof_with(bytes: &[(usize, u32)]) -> Roof {
        let mut b = [0u8; 88];
        for (offset, value) in bytes {
            b[*offset..*offset + 4].copy_from_slice(&value.to_le_bytes());
        }
        Roof::from_observed(b)
    }

    #[test]
    fn flat_roof_unclamped_is_non_finite_and_clamped_yields_section() {
        // height_01 = 0.05 (flat), circle radius 3. 平顶不进入 cap 分支 → fVar7 恒 0，
        // 分母 (0 + floor) − floor = 0 → 原版产 ±inf/NaN（"unclamped" 语义；章节 O）。
        let roof = roof_with(&[(0x30, 0x3d4ccccd), (0x14, 0x40400000)]);
        let h = roof.get_height_01_unclamped_from_ws(Vec3::new(0.0, 0.0, 0.0));
        assert!(!h.is_finite(), "flat roof unclamped height must be inf/NaN, got {h}");
        // clamped 入口把非有限值夹进 [0,1]（NaN 恒等），不 panic 仍出截面。
        let _section = roof.get_roof_shape_from_pos_ws_clamped(Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn pitched_roof_clamps_and_produces_section() {
        let roof = roof_with(&[
            (0x30, f32::to_bits(0.5)),
            (0x2c, f32::to_bits(0.5)),
            (0x38, f32::to_bits(0.5)),
            (0x34, f32::to_bits(0.5)),
            (0x1c, f32::to_bits(4.0)),
            (0x20, f32::to_bits(6.0)),
            (0x4c, 0),
            (8, 1), // rect tag
        ]);
        // A point near the ridge must map into [0,1] after clamping in the
        // clamped entry point.
        let section = roof.get_roof_shape_from_pos_ws_clamped(Vec3::new(0.0, 0.0, 0.0));
        match section {
            crate::resources::roofs::geometry::surface::RoofSection::Rectangle { .. } => {}
            _ => panic!("rect roof should produce a rectangle section"),
        }
    }

    #[test]
    fn expanded_rect_bottom_uses_eave_and_inset() {
        let roof = roof_with(&[
            (0x30, f32::to_bits(0.5)),
            (0x2c, f32::to_bits(0.5)),
            (0x38, f32::to_bits(0.5)),
            (0x34, f32::to_bits(0.5)),
            (0x1c, f32::to_bits(4.0)),
            (0x20, f32::to_bits(6.0)),
            (8, 1),
            (0x3c, 0),
        ]);
        let dims = get_roof_expanded_top_and_bottom(&roof);
        let (bottom, top) = dims.as_rectangle();
        let eave_ws = (0.5f32.sqrt() + (1.0 - 0.5f32.sqrt()) * 0.5) * 1.5 * 0.5;
        assert!((bottom[0] - (eave_ws + 0.28 + 4.0)).abs() < 1e-5);
        assert!((bottom[1] - (eave_ws + 0.28 + 6.0)).abs() < 1e-5);
        assert!((top[0] - (4.0 * 0.5 + 0.05)).abs() < 1e-5);
        assert!((top[1] - 0.1).abs() < 1e-5);
    }

    #[test]
    fn expanded_circle_bottom_and_top() {
        let roof = roof_with(&[
            (0x30, f32::to_bits(0.5)),
            (0x2c, f32::to_bits(0.5)),
            (0x38, f32::to_bits(0.5)),
            (0x14, f32::to_bits(3.0)),
            (8, 0),
        ]);
        let dims = get_roof_expanded_top_and_bottom(&roof);
        let (bottom, top) = dims.as_circle();
        let eave_ws = (0.5f32.sqrt() + (1.0 - 0.5f32.sqrt()) * 0.5) * 1.5 * 0.5;
        assert!((bottom - ((eave_ws * 1.2 + 0.28) * 0.5 + 3.0)).abs() < 1e-5);
        assert!((top - 0.2).abs() < 1e-5);
    }
}
