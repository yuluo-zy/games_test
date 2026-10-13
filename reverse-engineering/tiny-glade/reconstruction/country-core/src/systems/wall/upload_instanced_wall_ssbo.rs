//! upload_instanced_wall_ssbo.rs — flushes instanced-wall brick transforms to the GPU SSBO.
//!
//! PDB procedure `gpu_flush_brick_buffers` 0x14082c170 (594 B). FunctionSystem signature O.
//!
//! Business logic (O, decompiled 0x14082c170):
//!   - `AssetSsboLibrary::get_mut<BrickTransformSsbo>()` (missing -> `unwrap_failed` panic).
//!   - For each `InstancedWall` query row, iterates its dirty brick ranges (two paths
//!     selected by the `+0x108` compact flag): for every dirty index calls
//!     `InstancedWall::upload_to_storage_buffer` (writes the transform slot into the SSBO).
//!   - Finally `SparseInstanceBuffer::gpu_flush<BrickTransformSsbo>()`.

#![allow(dead_code, unused_imports)]

use bevy_ecs::system::{Query, Res, ResMut};

use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
use crate::geometry::instanced_wall::{BrickTransformSsbo, InstancedWall};
use crate::render::GraphicsDeviceResource;

pub fn gpu_flush_brick_buffers(
    graphics_device: Res<GraphicsDeviceResource>,
    mut ssbo_library: ResMut<AssetSsboLibrary>,
    mut walls: Query<&mut InstancedWall>,
) {
    let _ = (&graphics_device, &mut ssbo_library);
    // O: get the typed SSBO buffer (unwrap on absence) — the real
    // `AssetSsboLibrary::get_mut::<BrickTransformSsbo>()` needs the SsboId+StorageBuffer
    // registration; typed gap here (no fabricated SsboId values).
    for mut wall in walls.iter_mut() {
        // O: upload dirty brick ranges (compact flag selects the path). Typed gap for
        // the per-brick index stream.
        wall.upload_to_storage_buffer();
    }
    // O: gpu_flush on the sparse instance buffer (wp2
    // `SparseInstanceBuffer<BrickTransformSsbo>::gpu_flush(&device)`).
    // Typed gap: SparseSsbo binding for BrickTransformSsbo not wired in the merged crate.
}
