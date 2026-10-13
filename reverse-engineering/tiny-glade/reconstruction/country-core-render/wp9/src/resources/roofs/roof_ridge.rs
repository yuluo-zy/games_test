//! roof_ridge.rs — RoofRidgeDims / 脊线维度（WP9）。
//!
//! PDB 骨架 `resources/roofs/roof_ridge.rs`。命名实体 `RoofRidgeDims`。
//! 函数：`from`（0x1408e2690）、`calc`（0x1408e2700）、`get_inner`（0x1400885d0，
//! callers=283）、`to_world_space`（0x1408e2750）+ closure（0x1408e2eb0）、
//! `calc_roof_tip_center_ls`（0x1408e30b0）、`inverse_ridge_01_param`（0x1408e30f0）。

#![allow(dead_code, unused_imports)]

use crate::resources::roofs::roof_shape::{Roof, RoofRidgeDirection};

/// 脊线维度（矩形、Width 向）。内部字段名 U。
#[derive(Clone, Copy, Debug, Default)]
pub struct RoofRidgeDims {
    pub raw: crate::typed_gap::Gap,
}

impl RoofRidgeDims {
    /// RoofRidgeDims::from（O：0x1408e2690）。仅矩形 + ridge_dir==Width 才有 dims。
    pub fn from(roof: &Roof) -> Option<Self> {
        if !roof.is_rectangle() {
            return None;
        }
        if roof.0[0x3c] != RoofRidgeDirection::Width as u8 {
            return None;
        }
        Some(Self { raw: crate::typed_gap::Gap })
    }

    /// RoofRidgeDims::get_inner（O：0x1400885d0；callers=283）。
    pub fn get_inner(&self) -> [f32; 2] {
        // 脊线内尺寸 [f32;2]；正文简，数值 U。
        crate::typed_gap::gap_f32_pair()
    }

    /// RoofRidgeDims::calc（O：0x1408e2700；in_DL 门）。
    pub fn calc(&mut self, _flag: bool) {
        let _ = _flag;
    }

    /// RoofRidgeDims::to_world_space（O：0x1408e2750）。
    /// 输出 28B `{tag, start, end}`：tag=0 无脊（max_dim<=0.1），tag=1 脊线世界两点；
    /// 圆 panic。内部闭包/矩阵 U。
    pub fn to_world_space(&self, _roof: &Roof) -> RidgeWorld {
        RidgeWorld::Unknown(crate::typed_gap::Gap)
    }
}

/// 脊线世界空间输出（tagged；U 内部）。
#[derive(Clone, Copy, Debug)]
pub enum RidgeWorld {
    None,
    /// {tag=1, start, end}（U 精确字段）。
    Some {
        start: crate::typed_gap::Gap,
        end: crate::typed_gap::Gap,
    },
    Unknown(crate::typed_gap::Gap),
}

/// calc_roof_tip_center_ls（O：0x1408e30b0；局部尖点中心）。
pub fn calc_roof_tip_center_ls(_roof: &Roof) -> [f32; 2] {
    crate::typed_gap::gap_f32_pair()
}

/// inverse_ridge_01_param（O：0x1408e30f0）。
/// `(value-0.1)/(bottom-0.1)` clamp01；`flag==0 → bottom=top`；bottom==0.1 panic、
/// 越界/NaN panic。
pub fn inverse_ridge_01_param(value: f32, top: f32, bottom: f32, flag: bool) -> f32 {
    let bottom = if flag { bottom } else { top };
    assert!(!(bottom == 0.1 && !bottom.is_nan()), "ridge bottom == 0.1");
    let r = (value - 0.1) / (bottom - 0.1);
    let r = r.max(0.0);
    assert!(!r.is_nan() && (0.0..=1.0).contains(&r), "ridge param out of [0,1]");
    r.min(1.0)
}

/// RoofRidgeDims::max_dim（inline，I）。
pub fn max_dim(_dims: [f32; 2]) -> f32 {
    crate::typed_gap::gap_f32()
}
