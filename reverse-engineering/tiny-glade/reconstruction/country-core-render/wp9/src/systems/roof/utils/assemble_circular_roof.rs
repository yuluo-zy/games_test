//! systems/roof/utils/assemble_circular_roof.rs — 圆形铺瓦（WP9，prior work 骨架）。
//!
//! 见 assemble_rectangular_roof.rs 头注；数值核心 = `reconstruction/roof/tiles.rs`
//! `assemble_circular_tiles` / `assemble_circular_from_observed`（机器码验证）。

#![allow(dead_code, unused_imports)]

use crate::resources::roofs::roof_shape::Roof;
use crate::resources::roof_tile_ssbo::RoofShingle;
use crate::typed_gap::Gap;

/// assemble_circular_roof（O：0x1421b3120）。
pub fn assemble_circular_roof(_roof: &Roof, _writer: Gap) -> Vec<RoofShingle> {
    Vec::new()
}
