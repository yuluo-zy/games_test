//! clearing_shaders.rs — 林间空地树 draw（`_clearing_tree_{trunk,canopy}.raster`）。
//!
//! PDB 骨架 `systems/render_loop/draw_passes/draw_shaders/clearing_shaders.rs`
//! （编译单元 530）。机器码（O）：
//! - `tree_trunk` 0x1409c9830（size 1643，line 26）。
//! - `tree_canopy` 0x1409ca350（size 1373，line 80）。
//!
//! 通用：`DrawBuilder`（in_RCX，0xf8B 值传递）。`+0xf0` 状态字节非 0 时置
//! `+0xa8 |= 4`（cull mode，I）。结尾 `nani_draw()`（0x140904ae0，
//! `render::nani::draw_ext::DrawExt`，读 `NaniInstanceIndirectionSsbo` +
//! NaniDrawList* + draw_indexed_indirect_raw/count，O）。
//!
//! **tree_trunk 绑定序（O）**：6×`TextureFromDisk`（名字哈希
//! `0xce1615b9bd3e034`/`0x32dce6939b569f34`/`0x8215a488d99562a6`/
//! `0x93201d69003f98d4`/`0xaa12b01311b77c4d`/`0x4dbb39acf1c7b7`）→
//! `write_uniform<f32>` → `CameraMatrices::eye_position` →
//! `write_uniform<Vec3>` → `write_uniform<bool>` → 资源
//! `0x1b4e385d376c1cc0`（U，TreeCanopyMaxCullDistance 邻类）→
//! `write_uniform<f32>` + `TexturePreset(0xe)` → `nani_draw()`。
//! 资源 type-id `0xd2f519263066413b`（U）/`0xbba451ee6a08a3ff`（=camera，O）。
//!
//! **tree_canopy 绑定序（O）**：5×`TextureFromDisk`
//! (`0x579f39ab91a01865`=water_noise_texture、`0x521d5b30024d9662`、
//! `0x93201d69003f98d4`、`0xaa12b01311b77c4d`、`0x4dbb39acf1c7b7`) →
//! `write_uniform<f32>`×2 → `write_uniform<bool>` → 资源 `0x1b4e385d376c1cc0`
//! → `write_uniform<f32>` → `nani_draw()`。骨架 deps：`TreeCanopyMaxCullDistance::
//! get_dist`（0x…75048）、`MainCamera::lens`、`CameraModeStates::current_dyn`、
//! `lerp`、`RasterPipelineOpts::set_cull_mode`（cull back-face，O）。
//!
//! shader URI（O）：`/clearing/tree_trunk.raster.hlsl`、`/clearing/tree_canopy.raster.hlsl`
//! （compiled `_clearing_tree_*`，b903…/f0ad…）。
//! typed gap：各 TextureFromDisk 名字哈希对应的纹理资源名（除 water_noise_texture
//! O 外 U）；`0x1b4e385d376c1cc0` 资源 type-id 语义 U。

#![allow(dead_code, unused_imports)]

use crate::resources::texture_presets::{TextureFromDisk, TexturePreset};
use country_core::consume::pass_builder::DrawBuilder;
use country_core::render::nani::draw_ext::NaniDrawExt;
use country_core::render::nani::gpu_world::GpuWorldSubsetKind;
use country_core::typed_gap::Gap;

/// `tree_trunk`（O：0x1409c9830）。树干 clearing draw。
/// 6×磁盘纹理 + 相机眼位/距离 uniform + `nani_draw`。
pub fn tree_trunk(_builder: DrawBuilder, _world: &Gap, _kind: GpuWorldSubsetKind) -> DrawBuilder {
    let _ = (TextureFromDisk(0), TexturePreset(0xe));
    unimplemented!()
}

/// `tree_canopy`（O：0x1409ca350）。树冠 clearing draw。
/// 5×磁盘纹理（含 water_noise_texture）+ 距离/高度 uniform + `nani_draw`。
pub fn tree_canopy(_builder: DrawBuilder, _world: &Gap, _kind: GpuWorldSubsetKind) -> DrawBuilder {
    let _ = (TextureFromDisk(0), TexturePreset(0));
    unimplemented!()
}
