//! instanced_ivy.rs — 常春藤枝/叶/花（`_ivy_*.raster`）。
//!
//! PDB 骨架 `systems/render_loop/draw_passes/draw_shaders/instanced_ivy.rs`
//! （编译单元 497）。机器码（O）：
//! - `draw_instanced_ivy_branch_shader` 0x1408dbc50（size 855，line 19）。
//! - `draw_instanced_ivy_leaf_shader` 0x1408dc180（size 21，line 50）→ 调 `ivy_leaf_shader`。
//! - `draw_instanced_ivy_flower_shader` 0x1408dc1a0（size 21，line 60）→ 调 `ivy_leaf_shader`。
//! - `ivy_leaf_shader` 0x1408dc1c0（size 1555，line 70）：叶/花共用绑定。
//!
//! **branch**（O）：资源 `0xdf356893cc4e98a8`（U，枝段 map；位掩码迭代，哨兵
//! `'4'`）非空 → `read_storage_buffer<IvySegmentSsbo>` → `TexturePreset(9)` → 10 → 8
//! → `TextureFromDisk(0x579f39ab91a01865)`（=water_noise_texture，O）→
//! `TexturePreset(5)` → `TextureFromDisk(0xedf425b62b653c03)` →
//! `draw_instanced_w_gl_draw_flag`。
//!
//! **ivy_leaf_shader**（O）：`CameraMatrices::eye_position` → `write_uniform<Vec3>`
//! → `TexturePreset(9)` → 10 → 5 → `TextureFromDisk(0x579f39ab91a01865)` →
//! `write_uniform<bool>` → 叶/花标志：叶 → `read_storage_buffer<IvyLeafSsbo>` +
//! `FrozenMap::get<SsboId,…>` assert（0x3d=61 字节名字）；花 →
//! `read_storage_buffer<IvyFlowerSsbo>` + assert（0x3f=63 字节名字）→
//! `draw_instanced_w_gl_draw_flag`。
//!
//! shader URI（O）：`/ivy/instanced_ivy_branch.raster.hlsl`、
//! `/ivy/instanced_ivy_leaf.raster.hlsl`（compiled `_ivy_instanced_ivy_*`）。
//! compiled 另含 `_ivy_ivy.raster.hlsl`（无对应静态 URI，U：花/叶变体或运行时
//! 拼接）。
//! typed gap：`0xdf356893cc4e98a8` 资源语义 U；FrozenMap 断言名 U；
//! `_ivy_ivy.raster` 接线 U。

#![allow(dead_code, unused_imports)]

use crate::resources::texture_presets::{TextureFromDisk, TexturePreset};
use crate::consume::pass_builder::DrawBuilder;
use crate::systems::ivy::ivy_send_to_gpu_leaf::{IvyFlowerSsbo, IvyLeafSsbo};
use crate::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo;
use crate::typed_gap::Gap;

/// `draw_instanced_ivy_branch_shader`（O：0x1408dbc50）。枝段 SSBO + 水噪/叶纹理 →
/// `draw_instanced_w_gl_draw_flag`。
pub fn draw_instanced_ivy_branch_shader(
    _builder: DrawBuilder,
    _segments: &IvySegmentSsbo,
) -> DrawBuilder {
    let _ = (TextureFromDisk(0), TexturePreset(0));
    unimplemented!()
}

/// `draw_instanced_ivy_leaf_shader`（O：0x1408dc180）。叶绘制 wrapper → `ivy_leaf_shader`。
pub fn draw_instanced_ivy_leaf_shader(_builder: DrawBuilder, _leaves: &IvyLeafSsbo) -> DrawBuilder {
    unimplemented!()
}

/// `draw_instanced_ivy_flower_shader`（O：0x1408dc1a0）。花绘制 wrapper → `ivy_leaf_shader`。
pub fn draw_instanced_ivy_flower_shader(
    _builder: DrawBuilder,
    _flowers: &IvyFlowerSsbo,
) -> DrawBuilder {
    unimplemented!()
}

/// `ivy_leaf_shader`（O：0x1408dc1c0）。叶/花共用绑定：相机眼位 uniform + 纹理 +
/// 叶/花 SSBO 二选一 → `draw_instanced_w_gl_draw_flag`。
pub fn ivy_leaf_shader(
    _builder: DrawBuilder,
    _leaf: bool,
    _leaves: &IvyLeafSsbo,
    _flowers: &IvyFlowerSsbo,
) -> DrawBuilder {
    let _ = (TextureFromDisk(0), TexturePreset(0));
    unimplemented!()
}
