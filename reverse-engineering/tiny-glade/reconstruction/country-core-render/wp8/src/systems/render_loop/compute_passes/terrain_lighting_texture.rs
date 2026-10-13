//! `terrain_lighting_texture.rs` — W24 地形光照纹理计算 + mips（WP8 恢复升级）。
//!
//! 反编译（本轮 Ghidra）：`calc_terrain_lighting_texture`@0x140a51440（行 19）。
//!
//! 业务（O）：
//! 1. `AssetTextureLibrary::get_rg_texture()`（输出光栅）。
//! 2. `ComputePass::new`（`/terrain_lighting_texture.cs.hlsl`）：
//!    `CameraMatrices::eye_position()` → `write_uniform<Vec3>`；
//!    `bind_texture_to_compute_pass<TexturePreset>` preset 8/9/10/0x11；
//!    `write_uniform<bool>`；dispatch（1,1）。
//! 3. `generate_mips<TexturePreset>()`（生成光照纹理 mip）。
//!
//! 以 std `RwLock` 读计数保护纹理库查找（O：`in_RDX+0x90` 读写计数、`+0xf0` 数据指针）。
//! 帧调用者：frame 外层 @0x1404ec350（W24）。

#![allow(dead_code, unused_imports)]

use crate::consume::pass_builder::ComputePass;
use crate::typed_gap::Gap;

/// W24 地形光照纹理系统（O：0x140a51440，行 19；流程见文件头）。
pub fn calc_terrain_lighting_texture(world: &mut Gap, _camera: &Gap, _lighting_tex: &Gap) {
    let _ = world;
    unimplemented!("recover calc_terrain_lighting_texture: eye_position uniform + preset 8/9/10/0x11 + dispatch + generate_mips")
}
