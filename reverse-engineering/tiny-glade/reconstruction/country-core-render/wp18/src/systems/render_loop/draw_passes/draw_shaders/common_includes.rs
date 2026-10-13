//! common_includes.rs — 跨 pass 共享绑定 helper（`bind_*` 族）。
//!
//! PDB 骨架 `systems/render_loop/draw_passes/draw_shaders/common_includes.rs`
//! （编译单元 495/497/…/1638，多 pass 共用）。机器码（O）：
//! - `impl$0::bind_terrain_heightmap`（line 13，无独立过程——内联进各 pass）；
//!   骨架 deps：`read_texture`（terrain heightmap 读）。
//! - `impl$1::bind_terrain_lighting`（line 30，内联）；deps：`read_texture`。
//! - `impl$2::bind_water_object_push<ComputePass>` 0x14160e330（size 584）；
//!   `<DrawBuilder>` 0x141631f90（size 224）：水植被 push 常量绑定。
//!   - DrawBuilder 版（O）：`TexturePreset(0xf)` → `read_storage_buffer<
//!     RippleSourceSsbo>` → `TexturePreset(0x18)`（对应 `water_object_push.inc.hlsl`
//!     嵌入 lilypad/reed raster）。
//!   - ComputePass 版（O）：同序 + `read_access_buffer<RippleSourceSsbo>` +
//!     `assert_failed<SetIndex>`（set==0 校验）。
//! - `impl$3::bind_garden_exclusion<ComputePass>` 0x14160e630（size 344）：
//!   `TexturePreset(0xe)` → 0x13 → 5 → 0x10（garden 排斥 mask 4 槽）。
//!
//! shader URI 关联（I）：`water_object_push.inc.hlsl` 被
//! `_lilypad`/`_reed` 等水植被 raster 嵌入；garden exclusion 供 garden
//! spawn compute（W14/W16）读 foliage/water/path 排斥。精确槽语义 U。

#![allow(dead_code, unused_imports)]

/// `bind_terrain_heightmap<T>`（impl$0，O：内联；deps `read_texture`）。
/// 绑定 terrain_heightmap 纹理读访问。泛型 T = ComputePass/DrawBuilder（I）。
pub fn bind_terrain_heightmap<T>(pass: T) -> T {
    let _ = pass;
    unimplemented!()
}

/// `bind_terrain_lighting<T>`（impl$1，O：内联；deps `read_texture`）。
/// 绑定 terrain lighting 纹理读访问。泛型 T = ComputePass/DrawBuilder（I）。
pub fn bind_terrain_lighting<T>(pass: T) -> T {
    let _ = pass;
    unimplemented!()
}

/// `bind_water_object_push<T>`（impl$2；O）。
/// DrawBuilder 版（0x141631f90）：`TexturePreset(0xf)` +
/// `read_storage_buffer<RippleSourceSsbo>` + `TexturePreset(0x18)`。
/// ComputePass 版（0x14160e330）：同序，`read_access_buffer<RippleSourceSsbo>` +
/// set==0 断言。水植被 push 常量（`water_object_push.inc.hlsl`）。
pub fn bind_water_object_push<T>(pass: T) -> T {
    let _ = pass;
    unimplemented!()
}

/// `bind_garden_exclusion<T>`（impl$3；O：0x14160e630）。
/// `TexturePreset(0xe)` → 0x13 → 5 → 0x10：garden 排斥 mask 四槽（I）。
pub fn bind_garden_exclusion<T>(pass: T) -> T {
    let _ = pass;
    unimplemented!()
}
