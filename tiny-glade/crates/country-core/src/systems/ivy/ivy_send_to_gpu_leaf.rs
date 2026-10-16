//! B `ivy_send_to_gpu_leaf` 0x1408c1580: collect all ivy leaf instances and
//! upload 48-byte GPU rows to the IvyLeafSsbo / IvyFlowerSsbo buffers.
//!
//! O-level facts (2026-10-15 re-analyst, chapter ivy_send_to_gpu_leaf.rs.md):
//! - `IvyLeafStorage::all_leaves` collect (fat-ptr iterator);
//! - SSBO keys (rne-read qword at 0x142c2e248/0x142c2e280): IvyLeafSsbo =
//!   0x9e3cfa7f2993f406, IvyFlowerSsbo = 0x1d6de5e69ac50d2a, registered under
//!   the full type-path strings in the ssbo name table;
//! - GPU element = 48 B (`count * 0x30`); capacity check; the buffer write
//!   primitive is the shared ssbo-library path.
//!
//! SsboId reconciliation (2026-10-15 restorer-eco): `NameHash::compute` =
//! default `xxh3_64` (decompiled 0x140c9cd80), and the mechanism is confirmed
//! for Cobblestone3RasterStagingSsbo etc. The ivy SsboIds above are the actual
//! qword immediates, but `xxh3_64` of the known type names (short or full
//! path) does not reproduce them — the producing NAME string is 待取证. The
//! element types here carry the full-path `NAME` (matching the name-table
//! string) for the name-match assertion; their `SsboId::of` will not equal the
//! verified keys until the producing name is resolved.
//!
//! The GPU element is 48 B (`count * 0x30`), matching the shader struct in
//! `reconstruction/human-readable-shaders/shaders/ivy/instanced_ivy_leaf.raster.hlsl`:
//! `{ position: float3, size: float, forward_floattor: float3, seed: int,
//!   branch_dir: float3, arch_height: float }`. The CPU (104 B `IvyLeaf`) →
//! GPU mapping is verified for position/size/vector_2c→forward/
//! branch_direction→branch_dir; `seed` and `arch_height` are 待取证 (the leaf
//! spawner's `frame` is the seed candidate). The actual GPU commit needs a
//! `GraphicsDevice`, which the render world supplies; `upload_ivy_leaves` is
//! the library-side business mirroring `rasterize_buildings::upload_buildings`.

use crate::asset_libraries::ssbo_library::{AssetSsboLibrary, SsboType};
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::render::ssbo::{InstanceBuffer, StorageElement};
use crate::resources::ivy::{IvyLeaf, IvyLeafStorage};
use glam::Vec3;

/// Verified IvyLeafSsbo key (rne-read qword).
pub const IVY_LEAF_SSBO_ID: u64 = 0x9e3cfa_7f29_93f406;
/// Verified IvyFlowerSsbo key (rne-read qword).
pub const IVY_FLOWER_SSBO_ID: u64 = 0x1d6de5_e69a_c50d2a;
/// The full type-path strings the ssbo name table carries (O: name-table
/// strings at 0x142a9eb87 / 0x142a9da84).
pub const IVY_LEAF_SSBO_NAME: &str =
    "country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo";
pub const IVY_FLOWER_SSBO_NAME: &str =
    "country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo";

/// GPU leaf element, 48 B (O: `count * 0x30`; shader `instanced_ivy_leaf`).
/// The field order matches the shader struct; the CPU mapping for `seed` /
/// `arch_height` is 待取证.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IvyLeafGpu {
    pub position: Vec3,
    pub size: f32,
    /// The leaf's forward/facing direction (shader `forward_floattor`).
    pub forward: Vec3,
    /// Shader `seed` (int); the leaf spawner's `frame` is the candidate
    /// (待取证).
    pub seed: i32,
    /// The leaf's branch direction (shader `branch_dir`).
    pub branch_dir: Vec3,
    /// Shader `arch_height`; source 待取证 (needs the owning wall's arch).
    pub arch_height: f32,
}

impl StorageElement for IvyLeafGpu {
    const NAME: &'static str = IVY_LEAF_SSBO_NAME;
    const BUFFER_LEN: usize = 10_000;
}
impl SsboType for IvyLeafGpu {
    type Storage = InstanceBuffer<IvyLeafGpu>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        InstanceBuffer::<IvyLeafGpu>::new_storage_buffer(device, initial)
    }
}

/// GPU flower element (same 48 B layout, separate buffer). The flower
/// collection is not yet represented in `IvyLeafStorage`; this type lets the
/// buffer be registered and uploaded once flowers exist (待取证).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IvyFlowerGpu {
    pub position: Vec3,
    pub size: f32,
    pub forward: Vec3,
    pub seed: i32,
    pub branch_dir: Vec3,
    pub arch_height: f32,
}
impl StorageElement for IvyFlowerGpu {
    const NAME: &'static str = IVY_FLOWER_SSBO_NAME;
    const BUFFER_LEN: usize = 10_000;
}
impl SsboType for IvyFlowerGpu {
    type Storage = InstanceBuffer<IvyFlowerGpu>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        InstanceBuffer::<IvyFlowerGpu>::new_storage_buffer(device, initial)
    }
}

/// Pack one CPU leaf into a 48 B GPU row. `forward` = `vector_2c`,
/// `branch_dir` = `branch_direction`, `seed` = `frame` (candidate);
/// `arch_height` is 待取证 (0.0 stands in).
pub fn pack_leaf(leaf: &IvyLeaf) -> IvyLeafGpu {
    IvyLeafGpu {
        position: leaf.position,
        size: leaf.size,
        forward: leaf.vector_2c,
        seed: leaf.frame as i32,
        branch_dir: leaf.branch_direction,
        arch_height: 0.0,
    }
}

/// `ivy_send_to_gpu_leaf` body: collect all leaves, pack 48 B rows and upload
/// to the IvyLeafSsbo buffer, updating the instance count. Returns the packed
/// count. An unregistered buffer skips the upload silently (O pattern).
pub fn upload_ivy_leaves(
    leaves: &IvyLeafStorage,
    ssbos: &mut AssetSsboLibrary,
    device: &dyn GraphicsDevice,
) -> usize {
    let elements: Vec<IvyLeafGpu> = leaves.all_leaves().map(pack_leaf).collect();
    let count = elements.len();
    let Some(buffer) = ssbos.get_mut::<IvyLeafGpu>() else {
        return count;
    };
    if count == 0 {
        buffer.instance_count = 0;
    } else {
        buffer
            .buffer
            .gpu_write(device, &elements)
            .unwrap_or_else(|e| panic!("called `Result::unwrap()` on an `Err` value: {e:?}"));
        buffer.instance_count = count;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaf_packs_to_48_bytes() {
        assert_eq!(core::mem::size_of::<IvyLeafGpu>(), 48);
        let leaf = IvyLeaf {
            point_a: 0,
            point_b: 1,
            ivy: 7,
            source_18: 3,
            position: Vec3::new(1.0, 2.0, 3.0),
            vector_2c: Vec3::Y,
            branch_direction: Vec3::X,
            size: 1.0,
            branch_thickness: 0.5,
            growth: 0.0,
            growth_delay: 0.1,
            frame: 5,
            vector_58: Vec3::Z,
        };
        let gpu = pack_leaf(&leaf);
        assert_eq!(gpu.position, leaf.position);
        assert_eq!(gpu.size, leaf.size);
        // Verified GPU mapping (shader `instanced_ivy_leaf.raster.hlsl`).
        assert_eq!(gpu.forward, leaf.vector_2c);
        assert_eq!(gpu.branch_dir, leaf.branch_direction);
        assert_eq!(gpu.seed, 5);
    }

    #[test]
    fn verified_keys_are_the_documented_constants() {
        // The original's buffer keys (rne-read qword). The crate's `SsboId::of`
        // hashes `T::NAME` instead, a known modeling gap (same as
        // GroundedBuildingsSsbo); these constants document the original ids.
        assert_eq!(IVY_LEAF_SSBO_ID, 0x9e3cfa7f2993f406);
        assert_eq!(IVY_FLOWER_SSBO_ID, 0x1d6de5e69ac50d2a);
    }
}
