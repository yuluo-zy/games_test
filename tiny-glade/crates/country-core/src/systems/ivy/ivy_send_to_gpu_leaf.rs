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
//! The exact CPU (104 B) → GPU (48 B) field mapping is a recorded blocker
//! (re-analyst 卡点); the packed rows below carry the documented leaf
//! attributes (position, facing, scatter direction, size, thickness, growth).
//! The actual GPU commit needs a `GraphicsDevice`, which the render world
//! supplies; `upload_ivy_leaves` is the library-side business mirroring
//! `rasterize_buildings::upload_buildings`.

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

/// GPU leaf element, 48 B (O: `count * 0x30`), and its SSBO type. Field subset
/// documented; exact mapping to the 104 B CPU `IvyLeaf` is 待取证.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IvyLeafGpu {
    pub position: Vec3,
    pub vector_2c: Vec3,
    pub vector_58: Vec3,
    pub size: f32,
    pub branch_thickness: f32,
    pub growth: f32,
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
    pub vector_2c: Vec3,
    pub vector_58: Vec3,
    pub size: f32,
    pub branch_thickness: f32,
    pub growth: f32,
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

/// Pack one CPU leaf into a 48 B GPU row (subset; exact mapping 待取证).
pub fn pack_leaf(leaf: &IvyLeaf) -> IvyLeafGpu {
    IvyLeafGpu {
        position: leaf.position,
        vector_2c: leaf.vector_2c,
        vector_58: leaf.vector_58,
        size: leaf.size,
        branch_thickness: leaf.branch_thickness,
        growth: leaf.growth,
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
