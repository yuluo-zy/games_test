//! B `ivy_send_to_gpu_segments` 0x140a9b4e0: collect every ivy growth segment
//! and upload 48-byte GPU rows to the IvySegmentSsbo buffer.
//!
//! O-level facts (2026-10-15 re-analyst, chapter ivy_send_to_gpu_segments.rs.md):
//! - `IvyStorage::total_segment_count` collect;
//! - SSBO key (rne-read qword): IvySegmentSsbo = 0x2002d2fbdff3d80f, registered
//!   under the full type path string in the ssbo name table;
//! - GPU element = 48 B (`count * 0x30`); capacity check + buffer write.
//!
//! The GPU element is 48 B (`count * 0x30`), matching the shader struct in
//! `reconstruction/human-readable-shaders/shaders/ivy/instanced_ivy_branch.raster.hlsl`:
//! `{ start_pos: float3, start_thickness: float, end_pos: float3,
//!   end_thickness: float, wall_normal: float3, arch_height: float }`. The CPU
//! mapping is verified for start/end position and the point thicknesses;
//! `wall_normal` and `arch_height` are 待取证 (the point's animated thickness
//! fills `start_thickness`/`end_thickness`). The GPU commit needs a
//! `GraphicsDevice`, supplied by the render world; `upload_ivy_segments` is
//! the library-side business mirroring `rasterize_buildings::upload_buildings`.

use crate::asset_libraries::ssbo_library::{AssetSsboLibrary, SsboType};
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::render::ssbo::{InstanceBuffer, StorageElement};
use crate::resources::ivy::{Ivy, IvyStorage};
use glam::Vec3;

/// Verified IvySegmentSsbo key (rne-read qword).
pub const IVY_SEGMENT_SSBO_ID: u64 = 0x2002d2_fbdf_f3d80f;
/// The full type-path string the ssbo name table carries (O).
pub const IVY_SEGMENT_SSBO_NAME: &str =
    "country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo";

/// GPU segment element, 48 B (O: `count * 0x30`; shader `instanced_ivy_branch`).
/// Field order matches the shader struct; `wall_normal`/`arch_height` mapping
/// is 待取证.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IvySegmentGpu {
    pub start_pos: Vec3,
    pub start_thickness: f32,
    pub end_pos: Vec3,
    pub end_thickness: f32,
    /// Shader `wall_normal`; the unit tangent perpendicular is the candidate
    /// (待取证).
    pub wall_normal: Vec3,
    /// Shader `arch_height`; source 待取证 (needs the owning wall's arch).
    pub arch_height: f32,
}

impl StorageElement for IvySegmentGpu {
    const NAME: &'static str = IVY_SEGMENT_SSBO_NAME;
    const BUFFER_LEN: usize = 0;
}
impl SsboType for IvySegmentGpu {
    type Storage = InstanceBuffer<IvySegmentGpu>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        InstanceBuffer::<IvySegmentGpu>::new_storage_buffer(device, initial)
    }
}

/// Pack one ivy's consecutive points into 48 B GPU segment rows. The segment
/// start/end are the deque-ordered point positions; the animated point
/// thickness fills the start/end thickness (the branch shader lerps the
/// thickness across the segment). `wall_normal`/`arch_height` are 待取证
/// (0.0 / tangent-perpendicular stand in).
pub fn pack_ivy_segments(ivy: &Ivy) -> Vec<IvySegmentGpu> {
    ivy.points
        .iter()
        .zip(ivy.points.iter().skip(1))
        .map(|(a, b)| {
            let d = b.position - a.position;
            let len = d.length().max(1e-20);
            let normal = Vec3::new(-d.z / len, 0.0, d.x / len);
            IvySegmentGpu {
                start_pos: a.position,
                start_thickness: a.thickness,
                end_pos: b.position,
                end_thickness: b.thickness,
                wall_normal: normal,
                arch_height: 0.0,
            }
        })
        .collect()
}

/// `ivy_send_to_gpu_segments` body: collect all segments, pack 48 B rows and
/// upload to the IvySegmentSsbo buffer, updating the instance count. Returns
/// the packed count. An unregistered buffer skips the upload silently (O).
pub fn upload_ivy_segments(
    storage: &IvyStorage,
    ssbos: &mut AssetSsboLibrary,
    device: &dyn GraphicsDevice,
) -> usize {
    let elements: Vec<IvySegmentGpu> = storage
        .iter()
        .flat_map(|(_, ivy)| pack_ivy_segments(ivy))
        .collect();
    let count = elements.len();
    let Some(buffer) = ssbos.get_mut::<IvySegmentGpu>() else {
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
    fn segment_packs_to_48_bytes() {
        assert_eq!(core::mem::size_of::<IvySegmentGpu>(), 48);
        let mut storage = IvyStorage::new();
        let id = storage.spawn_ivy(Vec3::ZERO, Vec3::X, 1.0..2.0, 3..4, 0);
        let ivy = storage.get_mut(id).unwrap();
        ivy.add_point(Vec3::X, Vec3::X);
        let rows = pack_ivy_segments(ivy);
        assert_eq!(rows.len(), 1);
        // Verified start/end mapping (shader `instanced_ivy_branch.raster.hlsl`).
        assert_eq!(rows[0].start_pos, Vec3::ZERO);
        assert_eq!(rows[0].end_pos, Vec3::X);
    }

    #[test]
    fn verified_key_is_the_documented_constant() {
        // Original buffer key (rne-read qword); see ivy_send_to_gpu_leaf for
        // the crate's `SsboId::of` modeling gap.
        assert_eq!(IVY_SEGMENT_SSBO_ID, 0x2002d2fbdff3d80f);
    }
}
