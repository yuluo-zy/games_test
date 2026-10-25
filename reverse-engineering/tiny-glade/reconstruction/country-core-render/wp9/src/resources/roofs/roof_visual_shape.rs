//! roof_visual_shape.rs — RoofBoundsDims / 扩展上下界（WP9）。
//!
//! PDB 骨架 `resources/roofs/roof_visual_shape.rs`。命名实体 `RoofBoundsDims`。
//! 函数：`get_roof_expanded_top_and_bottom`（0x140a880c0）、`RoofBoundsDims::as_circle`
//! （0x140a88200）、`as_rectangle`（0x140a88240）。
//! = `reconstruction/roof/tiles.rs::expanded_bounds_observed`（机器码验证）。

#![allow(dead_code, unused_imports)]

use crate::resources::roofs::roof_shape::Roof;

/// 屋顶扩展上下界（tagged 20B）。tag 位0：0=圆，1=矩形。
#[derive(Clone, Copy, Debug)]
pub enum RoofBoundsDims {
    /// {bottom, top=0.2}。
    Circle { bottom: f32, top: f32 },
    /// {bottom, top}（[f32;2] 各）。
    Rectangle { bottom: [f32; 2], top: [f32; 2] },
}

impl RoofBoundsDims {
    /// as_circle（O：0x140a88200）：断言圆，返回底半径。
    pub fn as_circle(&self) -> f32 {
        match self {
            RoofBoundsDims::Circle { bottom, .. } => *bottom,
            _ => panic!("RoofBoundsDims is not circle"),
        }
    }
    /// as_rectangle（O：0x140a88240）：断言矩形，返回 (bottom, top)。
    pub fn as_rectangle(&self) -> ([f32; 2], [f32; 2]) {
        match self {
            RoofBoundsDims::Rectangle { bottom, top } => (*bottom, *top),
            _ => panic!("RoofBoundsDims is not rectangle"),
        }
    }
}

/// get_roof_expanded_top_and_bottom（O：0x140a880c0 / tiles.rs `expanded_bounds_observed`）。
pub fn get_roof_expanded_top_and_bottom(roof: &Roof) -> RoofBoundsDims {
    let f = |o: usize| f32::from_le_bytes(roof.0[o..o + 4].try_into().unwrap());
    let p = roof.profile_01();
    let e = f(0x38);
    let root = p.sqrt();
    let expanded = (root + (1.0 - root) * 0.5) * (1.5 * e + (1.0 - e) * 0.0);
    if !roof.is_rectangle() {
        RoofBoundsDims::Circle {
            bottom: (expanded * 1.2 + 0.28) * 0.5 + roof.radius(),
            top: 0.2,
        }
    } else {
        let ridge = f(0x34);
        let inset = (1.0 - ridge) * 0.1;
        let (top_x, top_y) = if roof.0[0x3c] == 0 {
            (f(0x1c) * ridge + inset, 0.1)
        } else {
            (0.1, f(0x20) * ridge + inset)
        };
        RoofBoundsDims::Rectangle {
            bottom: [(0.28 + f(0x1c)) + expanded, (0.28 + f(0x20)) + expanded],
            top: [top_x, top_y],
        }
    }
}
