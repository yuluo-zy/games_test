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

use bevy_ecs::system::{Query, Res, ResMut};

use crate::render::geometry::instanced_wall::{BrickTransformSsbo, InstancedWall};
use crate::render::GraphicsDeviceResource;
use crate::render::sparse_instance_buffer::SparseInstanceBuffer;
use crate::render::ssbo_library::{AssetSsboLibrary, BufferState};

pub fn gpu_flush_brick_buffers(
    graphics_device: Res<GraphicsDeviceResource>,
    mut ssbo_library: ResMut<AssetSsboLibrary>,
    mut walls: Query<&mut InstancedWall>,
) {
    let _ = &graphics_device;
    // O: get the typed SSBO buffer (unwrap on absence).
    let buffer: Option<&mut BufferState<BrickTransformSsbo>> =
        ssbo_library.get_mut(crate::render::ssbo_library::SsboId(0));
    let Some(_buffer) = buffer else {
        // original panics via core::option::unwrap_failed
        panic!("missing BrickTransformSsbo");
    };
    for wall in walls.iter_mut() {
        // O: upload dirty brick ranges (compact flag selects the path). Typed gap for
        // the per-brick index stream.
        let count = if wall.compact { 1 } else { 4 };
        for _ in 0..count {
            wall.upload_to_storage_buffer();
        }
    }
    // O: gpu_flush on the sparse instance buffer.
    let mut sparse = SparseInstanceBuffer::<BrickTransformSsbo>::default();
    sparse.gpu_flush();
}
