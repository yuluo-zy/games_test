//! `displace_rocky_terrain.rs` — W4 岩质地层位移（WP8 恢复升级）。
//!
//! 反编译（本轮 Ghidra）：`displace_rocky_terrain`@0x140a3ee80（行 14）。
//!
//! 业务（O）：
//! 1. 取 `RockyTerrainMesh` 资源（查找失败 → unwrap_failed）。
//! 2. `uVar2 / 3` = 三角形数 → 顶点组数（dispatch 规模，写入 PushConstants）。
//! 3. `ComputePass::new`（`/rocky_terrain/displace_rocky_terrain.cs.hlsl`）：
//!    `write_storage_buffer<RockyTerrainVertexSSBO>` +
//!    `write_storage_buffer<RockyTerrainTriangleSSBO>`；
//!    `bind_texture_to_compute_pass<TexturePreset>` ×5（preset 9、2×、0xe、5、0x13）。
//! 4. `dispatch`（规模 = 三角形数）。
//!
//! 帧调用者：frame 外层 @0x1404ec350（W4）。

#![allow(dead_code, unused_imports)]

use crate::consume::pass_builder::ComputePass;
use crate::resources::rocky_terrain::{RockyTerrainTriangleSSBO, RockyTerrainVertexSSBO};
use crate::typed_gap::Gap;

/// W4 岩质地层位移系统（O：0x140a3ee80，行 14；流程见文件头）。
pub fn displace_rocky_terrain(world: &mut Gap, _mesh: &Gap, _heightmap: &Gap) {
    let _ = world;
    let _ = (RockyTerrainVertexSSBO::default(), RockyTerrainTriangleSSBO::default());
    unimplemented!(
        "recover displace_rocky_terrain: ComputePass::new + 2x write_storage_buffer + 5x bind_texture + dispatch(tri)"
    )
}
