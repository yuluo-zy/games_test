//! floor_shader.rs — 地形地面（`_terrain_editing_floor.raster`）。
//!
//! PDB 骨架 `systems/render_loop/draw_passes/draw_shaders/floor_shader.rs`
//! （编译单元 530）。机器码（O）：
//! - `draw_floor_shader` 0x1409cacb0（size 1363，line 14）。
//!
//! 逻辑（O）：置 `+0xa8 |= 8`（blend/渲染标志，I）→ 绑定序：
//! `TexturePreset(9)` → 10 → 6 → 0x13 → `TextureFromDisk(0x4a2b687d0c1ecf58)` →
//! `bind_stone_floor_textures`（0x14090c300，`DrawBuilder::bind_*` 内联族，O）→
//! `bind_wall_color_customization`（0x14090bb70）→
//! `TextureFromDisk(0x84b51135b59095ba)` → `TextureFromDisk(0xb484366d00600ae5)` →
//! `TextureFromDisk(0x59b98d76c4e95bd1)` → `TexturePreset(0x15)` → 7 → 0xc → 0xe →
//! 0x14 → `TextureFromDisk(0x954b7e4974bbcd6b)` → `write_uniform<bool>` →
//! `draw()`（0x140906f40，直 draw）。
//!
//! shader URI（O）：`/terrain_editing/floor.raster.hlsl`（compiled
//! `_terrain_editing_floor`，b903…/f0ad…）。
//! typed gap：preset 槽/磁盘纹理名语义 U（9/10/6/0x13/0x15/7/0xc/0xe/0x14 为
//! terrain 纹理族，I）；`bind_stone_floor_textures` 在本束以文档记录（共享 crate
//! 建模在 `CommonDrawParams`，O 实为 `DrawBuilder::bind_*`）。

#![allow(dead_code, unused_imports)]

use crate::resources::texture_presets::{TextureFromDisk, TexturePreset};
use crate::consume::pass_builder::DrawBuilder;

/// `draw_floor_shader`（O：0x1409cacb0）。地面材质：多 preset/磁盘纹理 +
/// 石材/墙色绑定 + `draw`。
pub fn draw_floor_shader(_builder: DrawBuilder) -> DrawBuilder {
    let _ = (TextureFromDisk(0), TexturePreset(0));
    unimplemented!()
}
