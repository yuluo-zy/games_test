//! draw_bricks.rs — 墙砖 LOD0/1 间接绘制（`_wall_wall_brick_lod{0,1}.raster`）。
//!
//! PDB 骨架 `systems/render_loop/draw_passes/draw_shaders/draw_bricks.rs`
//! （编译单元 487）。机器码（O）：
//! - `indirect_draw_bricks<0>` 0x1408a53d0（size 1387，line 19，LOD0）。
//! - `indirect_draw_bricks<1>` 0x1408a4ca0（size 1417，line 19，LOD1）。
//!
//! const 泛型 `<0>/<1>` = 墙砖 LOD 变体。
//!
//! 逻辑（O，两版一致）：`+0xf0` 状态字节（<0>：>1 则 `discard`；<1>：`&7` 取
//! 变体，case 3/4 直接 `multi_draw_indexed_indirect`）→ 读间接参数（u64：低 32 位
//! 间接缓冲索引，高 32 位期望键；`unwrap_failed<Result>`）→ `UnsafeEntityCell::
//! get<WallBrickInstancedDrawProxy>`（空则 `discard`）→ 置 `+0xa8` cull（<0> |=4，
//! <1> &=0xf3）→ 资源 `0x8226cc786e679355`（U，ssbo library）→
//! `FrozenMap::get<SsboId, Box<BufferState>>` + `assert_failed<str,str>`（58 字节
//! 名字）→ vtable +0x18 取 buffer 指针（空则 `unwrap_failed`）→
//! `bind_external_storage_buffer` + `read_storage_buffer<BrickDrawListSsbo>` →
//! 资源 `0x850dbd51eafef756`（U）空且标记 → `discard`；否则
//! `bind_brick_erase_highlight` + `bind_wall_color_customization` +
//! `TextureFromDisk(0x7db9e7483fa24078)`（砖纹理，名字 U）+
//! `read_storage_buffer<RoofColorIdSsbo>` + `write_uniform<i32>` +
//! `multi_draw_indexed_indirect`。
//!
//! shader URI（O）：`/wall/wall_brick_lod0.raster.hlsl`、
//! `/wall/wall_brick_lod1.raster.hlsl`（compiled `_wall_wall_brick_lod{0,1}`，
//! b903…/f0ad…）。
//! typed gap：`0x8226cc786e679355`/`0x850dbd51eafef756` 资源 type-id 语义 U；
//! `FrozenMap` 58 字节断言名字 U；`<1>` 变体开关 case 3/4 语义 U。

#![allow(dead_code, unused_imports)]

use crate::resources::color_id_roof::RoofColorIdSsbo;
use crate::resources::texture_presets::TextureFromDisk;
use country_core::components::geometry::BrickDrawListSsbo;
use country_core::consume::pass_builder::DrawBuilder;
use country_core::geometry::instanced_wall::WallBrickInstancedDrawProxy;
use country_core::typed_gap::Gap;

/// `indirect_draw_bricks<LOD>`（O：0x1408a53d0 / 0x1408a4ca0）。
/// 按间接参数取 `WallBrickInstancedDrawProxy` → ssbo 库断言 → 砖 SSBO/纹理绑定 →
/// `multi_draw_indexed_indirect`（间接参数源 = W21/W27 写的 draw-list）。
pub fn indirect_draw_bricks<const LOD: u8>(
    _builder: DrawBuilder,
    _proxy: &WallBrickInstancedDrawProxy,
    _draw_list: &BrickDrawListSsbo,
    _roof_colors: &RoofColorIdSsbo,
    _world: &Gap,
) -> DrawBuilder {
    let _ = TextureFromDisk(0x7db9e7483fa24078);
    unimplemented!()
}
