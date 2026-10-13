//! color_id_roof.rs — RoofColorIdSsbo（WP9）。
//!
//! PDB 骨架 `resources/color_id_roof.rs`。命名实体 `RoofColorIdSsbo`（
//! draw_instanced_roof_shader_nani 读）；`impl$0::default_value`（0x14099ca50）。

#![allow(dead_code, unused_imports)]

use crate::typed_gap::Gap;

/// 屋顶颜色 ID（SSBO 元素；draw_instanced_roof_shader_nani 读 RoofColorIdSsbo）。
#[derive(Clone, Debug, Default)]
pub struct RoofColorIdSsbo(pub Gap);

impl RoofColorIdSsbo {
    /// impl$0::default_value（O：0x14099ca50；默认色值）。
    pub fn default_value(&self) -> u32 {
        // 默认颜色 ID；正文简（decomp return），值 U。
        0
    }
}
