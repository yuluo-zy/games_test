//! stairs_draw.rs — 楼梯间接绘制（`_rocky_terrain_stairs_stairs_indirect.raster`）。
//!
//! PDB 骨架 `systems/render_loop/draw_passes/draw_shaders/stairs_draw.rs`
//! （编译单元 482）。机器码（O）：
//! - `stairs_draw` 0x14086a780（size 1140，line 15）。
//! - `closure$0`（line 23，inlinee）：哈希表迭代闭包（哨兵 0x1d）。
//!
//! 逻辑（O）：`fetch_ref`（实体间接参数）→ 两张 RwLock 保护 hash map（+0x50/
//! +0xb8，读者计数 +0x90，`0x775c6361f3425a3a`/`0x3bad471c3be19d00` 键，U）——
//! 楼梯实例查表 → `Table::get_component`（`StairsTransformSsbo`，U：每楼梯
//! transform）→ 绑定：`read_storage_buffer<StairsTransformSsbo>` →
//! `TextureFromDisk(0x84a037e697de0324)` → `write_uniform<i32>` →
//! `write_uniform<bool>` → `TexturePreset(0xc)` → `multi_draw_indexed_indirect`
//! （间接参数源 = W20 `_stairs_stairs`/`_stairs_counter` compute，I）。
//!
//! shader URI（O）：`/rocky_terrain/stairs/stairs_indirect.raster.hlsl`（compiled
//! `_rocky_terrain_stairs_stairs_indirect`，b903…/f0ad…）。
//! typed gap：两 map 键/哨兵语义 U；`StairsTransformSsbo` 记录布局 U。

#![allow(dead_code, unused_imports)]

use crate::resources::texture_presets::{TextureFromDisk, TexturePreset};
use country_core::consume::pass_builder::DrawBuilder;
use country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo;
use country_core::typed_gap::Gap;

/// `stairs_draw`（O：0x14086a780）。楼梯 transform 查表 → 绑定 →
/// `multi_draw_indexed_indirect`。
pub fn stairs_draw(
    _builder: DrawBuilder,
    _transforms: &StairsTransformSsbo,
    _world: &Gap,
) -> DrawBuilder {
    let _ = (TextureFromDisk(0), TexturePreset(0xc));
    unimplemented!()
}

/// `stairs_draw::closure$0`（O：line 23）。哈希表迭代闭包（0x1d 哨兵位掩码）。
pub fn closure_0(_window: (u64, u64)) -> bool {
    unimplemented!()
}
