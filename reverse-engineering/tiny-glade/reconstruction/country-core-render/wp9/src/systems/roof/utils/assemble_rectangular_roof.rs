//! systems/roof/utils/assemble_*.rs — 铺瓦组装（WP9，prior work 骨架）。
//!
//! PDB 骨架 `systems/roof/src/utils/assemble_{circular,rectangular}_roof.rs`。
//! 机器码：`assemble_circular_roof` 0x1421b3120、`assemble_rectangular_roof` 0x1421aeef0。
//! 数值核心已机器码验证：`reconstruction/roof/tiles.rs`（圆形 11467 记录零差异；
//! 矩形前置上下文/角点验证）。此处为骨架对齐入口。

#![allow(dead_code, unused_imports)]

use crate::resources::roofs::roof_shape::Roof;
use crate::resources::roof_tile_ssbo::RoofShingle;
use crate::typed_gap::Gap;

/// assemble_circular_roof（O：0x1421b3120）。
/// 圆形屋顶 64B RoofShingle 记录生成（prior `assemble_circular_from_observed`）。
pub fn assemble_circular_roof(_roof: &Roof, _writer: Gap) -> Vec<RoofShingle> {
    Vec::new()
}

/// assemble_rectangular_roof（O：0x1421aeef0）。
/// 矩形屋顶记录生成（prior `rectangular_context_from_observed` + 面/檐/脊）。
pub fn assemble_rectangular_roof(_roof: &Roof, _rect: Gap, _writer: Gap) -> Vec<RoofShingle> {
    Vec::new()
}
