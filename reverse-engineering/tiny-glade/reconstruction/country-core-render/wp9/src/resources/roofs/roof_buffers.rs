//! roof_buffers.rs — 屋顶 SSBO 元素（WP9）。
//!
//! PDB 骨架 `resources/roofs/roof_buffers.rs`（源 0 行代码）。命名实体 `RoofSupportBeamSsbo`
//! / `WoodenPlankSsbo`（布局 U）；`impl$15::fmt` 常量（shader 绑定开关）。

#![allow(dead_code, unused_imports)]

use crate::typed_gap::Gap;

/// 屋顶支撑梁实例（draw_instanced_roof_support_beam_shader 读 RoofSupportBeamSsbo）。
#[derive(Clone, Debug, Default)]
pub struct RoofSupportBeamSsbo(pub Gap);

/// 木条（支撑梁/木地板实例元素）。
#[derive(Clone, Debug, Default)]
pub struct WoodenPlankSsbo(pub Gap);

/// 格式化常量位（O：`impl$15::fmt` 命名；语义=shader 绑定开关，U）。
pub mod fmt_flags {
    pub const LESS_SNOWY: u32 = 1;
    pub const NO_ERASER_HIGHLIGHT: u32 = 2;
    pub const TIMBEFRAME_PREVIEW: u32 = 4;
    pub const WALL_ID_IS_COLOR_ID: u32 = 8;
}
