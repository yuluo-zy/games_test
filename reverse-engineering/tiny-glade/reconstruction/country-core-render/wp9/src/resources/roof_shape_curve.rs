//! roof_shape_curve.rs — Roof 形状/剖面数值核心（WP9）。
//!
//! 等价于已机器码验证的 `reconstruction/roof/surface.rs`（本文件为内联移植，保留证据
//! 引用；`evidence/roof-tiles/findings.md`：surface/Curve2 通过机器码验证）。反编译地址
//! 见 `evidence/roof-domain-dump.txt`（roof_shape 模块）。

#![allow(dead_code, unused_imports)]

use crate::resources::roofs::roof_shape::{Roof, RoofSection, profile_exponent};
use crate::typed_gap::Gap;

// ---------------------------------------------------------------------------
// 曲线（Curve2 = 游戏自有 utils::curve，等价表示）
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum CurveError {
    TooFewPoints(u8),
    NonFiniteLength,
    DuplicateSegment(usize),
}

#[derive(Clone, Debug)]
pub struct Curve2 {
    pub points: Vec<[f32; 2]>,
    pub points_u: Vec<f32>,
    pub length: f32,
}

impl Curve2 {
    pub fn try_new(points: Vec<[f32; 2]>, reject_duplicates: bool) -> Result<Self, CurveError> {
        if points.len() < 2 {
            return Err(CurveError::TooFewPoints(points.len() as u8));
        }
        let mut distance = 0.0;
        let mut previous = points[0];
        let mut points_u = Vec::with_capacity(points.len());
        for point in &points {
            let dx = previous[0] - point[0];
            let dy = previous[1] - point[1];
            distance += (dy * dy + dx * dx).sqrt();
            points_u.push(distance);
            previous = *point;
        }
        if distance == 0.0 {
            return Err(CurveError::NonFiniteLength);
        }
        for u in &mut points_u {
            *u /= distance;
            if u.to_bits() & 0x7fffffff > 0x7f7fffff {
                return Err(CurveError::NonFiniteLength);
            }
        }
        if reject_duplicates {
            for i in 0..points.len() - 1 {
                if points[i][0] == points[i + 1][0] && points[i][1] == points[i + 1][1] {
                    return Err(CurveError::DuplicateSegment(i));
                }
            }
        }
        Ok(Self {
            points,
            points_u,
            length: distance,
        })
    }
}

// ---------------------------------------------------------------------------
// height（O：0x1408e3260 反编译 + surface.rs 全函数机器码验证）
// ---------------------------------------------------------------------------

fn sse_max(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}
fn sse_min(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

pub(crate) fn height_impl(roof: &Roof) -> f32 {
    let h = roof.height_01();
    if !h.is_nan() && h <= 0.1 {
        return 0.0;
    }
    let (size, base) = if !roof.is_rectangle() {
        ((roof.radius() + (-0.90000004)) / 7.1, 8.0)
    } else {
        let x = roof.shape_dims()[0];
        let z = roof.shape_dims()[1];
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

/// get_height_01_unclamped_from_ws（O：0x1408e4790）：cap 计算 + inverse_lerp 反演。
pub(crate) fn get_height_01_unclamped_from_ws(roof: &Roof) -> f32 {
    let cap = height_impl(roof);
    // 反演：由世界高度反推 height_01（utils::inverse_lerp；U 输入语义）。
    // 传回 roof[0x30] 的原值 + 高度范围：此处为 typed gap 占位，防误用。
    let _ = cap;
    crate::typed_gap::gap_f32()
}

// ---------------------------------------------------------------------------
// profile 曲线（O：0x1408e3420 / surface.rs `profile_curve_normalized`）
// ---------------------------------------------------------------------------

pub(crate) fn profile_curve_normalized(profile: f32) -> Curve2 {
    const U: [u32; 20] = [
        0, 0x3d579436, 0x3dd79436, 0x3e21af28, 0x3e579436, 0x3e86bca2, 0x3ea1af28,
        0x3ebca1af, 0x3ed79436, 0x3ef286bd, 0x3f06bca2, 0x3f1435e5, 0x3f21af28,
        0x3f2f286c, 0x3f3ca1af, 0x3f4a1af3, 0x3f579436, 0x3f650d79, 0x3f7286bd,
        0x3f800000,
    ];
    let exp = profile_exponent(profile);
    let points: Vec<[f32; 2]> = (0..20)
        .map(|i| {
            let u = f32::from_bits(U[i]);
            [u, if i == 19 { 1.0 } else { u.powf(exp) }]
        })
        .collect();
    Curve2::try_new(points, false).expect("profile_curve_normalized unwrap")
}

/// profile_curve_ws（O：0x1408e3d40）：归一曲线 → 世界曲线（bottom/top/height 变换）。
pub(crate) fn profile_curve_ws(
    normalized: &Curve2,
    bottom: f32,
    top: f32,
    height: f32,
) -> Curve2 {
    let points: Vec<[f32; 2]> = normalized
        .points
        .iter()
        .map(|p| [p[0] * top + (1.0 - p[0]) * bottom, p[1] * height])
        .collect();
    Curve2::try_new(points, false).expect("profile_curve_ws unwrap")
}

// ---------------------------------------------------------------------------
// 截面（O：0x1408e4920 / surface.rs `get_roof_shape`）
// ---------------------------------------------------------------------------

pub(crate) fn get_roof_shape(roof: &Roof, t: f32) -> RoofSection {
    let mut out = [0u8; 28];
    crate::resources::roof_shape_curve::get_roof_shape_bytes(roof, t, &mut out);
    if out[0] == 1 {
        RoofSection::Rectangle {
            frame: [f(&out, 4), f(&out, 8), f(&out, 12), f(&out, 16)],
            dimensions: [f(&out, 20), f(&out, 24)],
        }
    } else if out[0] == 2 {
        RoofSection::Outside
    } else {
        RoofSection::Circle {
            frame: [f(&out, 4), f(&out, 8)],
            radius: f(&out, 12),
            word_0x18: u32::from_le_bytes(out[16..20].try_into().unwrap()),
        }
    }
}

fn f(b: &[u8], o: usize) -> f32 {
    f32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}

/// 28B 截面原始输出（surface.rs `get_roof_shape` 逐字节；0x1408e4920 一致）。
pub(crate) fn get_roof_shape_bytes(roof: &Roof, t: f32, out: &mut [u8; 28]) {
    let profile = roof.profile_01();
    let blend = t.powf(1.0 / profile_exponent(profile));
    let eave = roof.eave_length_01();
    if u32::from_le_bytes(roof.0[8..12].try_into().unwrap()) == 1 {
        out[..4].copy_from_slice(&1u32.to_le_bytes());
        out[4..20].copy_from_slice(&roof.0[0x0c..0x1c]);
        let expanded = (1.5 * eave + (1.0 - eave) * 0.0)
            * (profile.sqrt() + (1.0 - profile.sqrt()) * 0.5);
        let ridge = roof.ridge_length_01();
        let inset = (1.0 - ridge) * 0.1;
        let dx = f(&roof.0, 0x1c);
        let dz = f(&roof.0, 0x20);
        let bottom_x = expanded + (0.28 + dx);
        let bottom_z = expanded + (0.28 + dz);
        let top_x = dx * ridge + inset;
        let top_z = dz * ridge + inset;
        let switched = roof.0[0x3c] & 1 != 0;
        put(out, 20, blend * (if switched { 0.1 } else { top_x }) + (1.0 - blend) * bottom_x);
        put(out, 24, blend * (if switched { top_z } else { 0.1 }) + (1.0 - blend) * bottom_z);
    } else {
        out[..4].copy_from_slice(&0u32.to_le_bytes());
        out[4..12].copy_from_slice(&roof.0[0x0c..0x14]);
        let expanded = ((profile.sqrt() + (1.0 - profile.sqrt()) * 0.5)
            * (1.5 * eave + (1.0 - eave) * 0.0)
            * 1.2
            + 0.28)
            * 0.5;
        put(out, 12, blend * 0.2 + (1.0 - blend) * (expanded + roof.radius()));
        out[16..20].copy_from_slice(&roof.0[0x18..0x1c]);
    }
}

fn put(b: &mut [u8], o: usize, v: f32) {
    b[o..o + 4].copy_from_slice(&v.to_le_bytes());
}

/// roof_shape_at_intersection_y（O：0x1408e3820）。
pub(crate) fn roof_shape_at_intersection_y(roof: &Roof, y: f32) -> RoofSection {
    let base = roof.base_y();
    let height = height_impl(roof);
    if y == base || (y.is_nan() && base.is_nan()) {
        // base 截面：复制 roof[0x8..0x24]（tag 0/1 + 几何）。
        let mut out = [0u8; 28];
        out[..0x1c].copy_from_slice(&roof.0[8..0x24]);
        return crate::resources::roof_shape_curve::section_from_tagged(&out);
    }
    if y < base || base + height <= y {
        return RoofSection::Outside;
    }
    let t = inverse_lerp(base, base + height, y);
    get_roof_shape(roof, t.powf(1.0 / profile_exponent(roof.profile_01())))
}

fn section_from_tagged(out: &[u8; 28]) -> RoofSection {
    if u32::from_le_bytes(out[..4].try_into().unwrap()) == 1 {
        RoofSection::Rectangle {
            frame: [f(out, 4), f(out, 8), f(out, 12), f(out, 16)],
            dimensions: [f(out, 20), f(out, 24)],
        }
    } else {
        RoofSection::Circle {
            frame: [f(out, 4), f(out, 8)],
            radius: f(out, 12),
            word_0x18: u32::from_le_bytes(out[16..20].try_into().unwrap()),
        }
    }
}

pub(crate) fn inverse_lerp(a: f32, b: f32, v: f32) -> f32 {
    (v - a) / (b - a)
}

/// roof_height_ws_from_wall_shape（O：0x1408e3c70）：EasingFunction::apply 包装。
pub(crate) fn roof_height_ws_from_wall_shape(_params: &crate::resources::roofs::roof_shape::RoofShapeParams) -> f32 {
    // 墙形状 → 世界高度（utils::easing_functions::EasingFunction::apply；U 曲线参数）。
    crate::typed_gap::gap_f32()
}

/// 兼容占位：供 roof_shape 引用但未被消费的 Gap 类型。
#[allow(dead_code)]
pub(crate) fn _gap(_: Gap) {}
