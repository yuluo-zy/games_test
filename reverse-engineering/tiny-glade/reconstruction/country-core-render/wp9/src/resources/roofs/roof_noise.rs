//! roof_noise.rs — 屋顶噪声（WP9）。
//!
//! PDB 骨架 `resources/roofs/roof_noise.rs`。函数：`apply_roof_noise`（0x1412348f0）、
//! `revert_roof_noise`（inline）、`gpunoise::{hash1,hash2,hash_combine2,uint_to_u01_float,
//! lerp}`（inline）、`gpunoise::noise`（0x140a152c0）。

#![allow(dead_code, unused_imports)]

use crate::typed_gap::Gap;

/// gpunoise::noise(x,y)（O：0x140a152c0；floorf 起点，哈希正文 U）。
pub fn noise(_x: f32, _y: f32) -> f32 {
    // 原函数调用 floorf(x), floorf(y) 后进入哈希；正文未展开 → typed gap。
    crate::typed_gap::gap_f32()
}

/// apply_roof_noise（O：0x1412348f0）：`dst[+4] += noise()·0.3·smoothstep(clamp01(x))`。
/// 作用对象：RoofTileSimulation 或 Shingle 记录（dst+4 = field_4 候选）。U 目标类型。
pub fn apply_roof_noise(dst: &mut [f32], x: f32) {
    let t = x.clamp(0.0, 1.0);
    let smooth = (3.0 - (t + t)) * (t * t);
    if let Some(slot) = dst.get_mut(1) {
        *slot += noise(0.0, 0.0) * 0.3 * smooth;
    }
}

/// revert_roof_noise（inline，I：apply 的逆/回滚）。
pub fn revert_roof_noise(_dst: &mut [f32], _x: f32) {
    let _ = Gap;
}
