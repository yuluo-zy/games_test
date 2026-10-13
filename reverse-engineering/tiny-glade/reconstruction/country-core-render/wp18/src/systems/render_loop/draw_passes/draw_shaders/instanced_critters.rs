//! instanced_critters.rs — 蝴蝶/萤火虫/鸭/鹪鹩（`_critters_*.raster`）。
//!
//! PDB 骨架 `systems/render_loop/draw_passes/draw_shaders/instanced_critters.rs`
//! （编译单元 1615）。机器码（O）：
//! - `draw_instanced_butterfly_shader` 0x1415547e0（size 310，line 19）。
//! - `draw_instanced_firefly_shader` 0x1415549b0（size 569，line 34）。
//! - `draw_instanced_firefly_glow_shader` 0x141554cf0（size 642，line 55）。
//! - `draw_duck_shader` 0x141555080（size 1719，line 80）。
//! - `draw_wren_shader` 0x141555ec0（size 289，line 124）。
//!
//! 绑定序：
//! - butterfly（O）：`TexturePreset(9)` → 10 → `read_storage_buffer<ButterflySsbo>`
//!   → `TextureFromDisk(0x56463770c9cae8e1)` → `draw_instanced_w_gl_draw_flag`。
//! - firefly（O）：资源 `0xa2127765959c8b85`（U，萤火虫计数/密度，f32）非 0 →
//!   `TexturePreset(9)` → 10 → `write_uniform<f32>` →
//!   `read_storage_buffer<FireflySsbo>` → `draw_instanced_w_gl_draw_flag`；0 → `discard`。
//! - firefly_glow（O）：同 firefly + 置 `+0xa8 &= 0x7f`（blend，I）+ 读
//!   `FireflySsbo` + `FireflyVisibilitySsbo`（=`_critters_firefly_vis_query.cs.hlsl`
//!   生产者，O）+ `DofAutofocusSsbo` → `draw_instanced_w_gl_draw_flag`。
//! - duck（O）：读间接参数（u64：低 32 索引/高 32 键）+ `get_component`×3（U：
//!   transform 组件族）→ `TexturePreset(9)` → 10 → `write_uniform<Mat4>`
//!   （四元数→矩阵，O：glam Mat4）→ `write_uniform<f32>`×2 → `write_uniform<bool>`
//!   → `write_uniform<f32>` → `write_uniform<i32>`×2 → `draw()`。
//! - wren（O）：`TexturePreset(9)` → 10 → `TextureFromDisk(0xe69294dae8320741)` →
//!   `nani_draw`（鹪鹩走 nani GPU-driven）。
//!
//! shader URI（O）：`/critters/{butterfly,firefly,firefly_glow,duck_animated,
//! nani_wren}.raster.hlsl`（compiled `_critters_*`，b903…/f0ad…）。
//! typed gap：`0xa2127765959c8b85` 资源语义 U；duck 三组件/Mat4 来源 U；
//! butterfly/wren 磁盘纹理名 U。

#![allow(dead_code, unused_imports)]

use crate::resources::critters::{ButterflySsbo, FireflySsbo, FireflyVisibilitySsbo};
use crate::resources::texture_presets::{TextureFromDisk, TexturePreset};
use country_core::consume::core::DofAutofocusSsbo;
use country_core::consume::pass_builder::DrawBuilder;
use country_core::render::nani::draw_ext::NaniDrawExt;
use country_core::render::nani::gpu_world::GpuWorldSubsetKind;
use country_core::typed_gap::Gap;

/// `draw_instanced_butterfly_shader`（O：0x1415547e0）。
pub fn draw_instanced_butterfly_shader(
    _builder: DrawBuilder,
    _butterflies: &ButterflySsbo,
) -> DrawBuilder {
    let _ = (TextureFromDisk(0), TexturePreset(0));
    unimplemented!()
}

/// `draw_instanced_firefly_shader`（O：0x1415549b0）。密度非 0 才绘制，否则 discard。
pub fn draw_instanced_firefly_shader(
    _builder: DrawBuilder,
    _fireflies: &FireflySsbo,
    _density: &Gap,
) -> DrawBuilder {
    let _ = (TextureFromDisk(0), TexturePreset(0));
    unimplemented!()
}

/// `draw_instanced_firefly_glow_shader`（O：0x141554cf0）。+可见性/DoF SSBO。
pub fn draw_instanced_firefly_glow_shader(
    _builder: DrawBuilder,
    _fireflies: &FireflySsbo,
    _visibility: &FireflyVisibilitySsbo,
    _dof: &DofAutofocusSsbo,
    _density: &Gap,
) -> DrawBuilder {
    let _ = (TextureFromDisk(0), TexturePreset(0));
    unimplemented!()
}

/// `draw_duck_shader`（O：0x141555080）。间接参数 → 鸭子组件 → Mat4 动画 uniform →
/// `draw`。
pub fn draw_duck_shader(_builder: DrawBuilder, _world: &Gap) -> DrawBuilder {
    let _ = TexturePreset(0);
    unimplemented!()
}

/// `draw_wren_shader`（O：0x141555ec0）。`nani_draw`（鹪鹩走 nani 实例化）。
pub fn draw_wren_shader(_builder: DrawBuilder, _kind: GpuWorldSubsetKind) -> DrawBuilder {
    let _ = TextureFromDisk(0xe69294dae8320741);
    unimplemented!()
}
