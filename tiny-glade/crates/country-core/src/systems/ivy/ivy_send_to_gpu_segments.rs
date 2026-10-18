//! Current B IvySegmentSsbo identity and fixed48B ×40000 constructor.
//! Original TypedStorageBuffer@14161a3c0 is451B; NAME is the19B
//! `Ivy Segments Buffer`, usage0x22/memory1/alignment4, None no upload.
//! Existing CPU segment packing below still requires independent original
//! validation; fixing this type/factory does not certify its guessed normal.
use crate::asset_libraries::ssbo_library::{AssetSsboLibrary, SsboType};
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::render::ssbo::{InstanceBuffer, StorageElement};
use crate::resources::ivy::{Ivy, IvyStorage};
use crate::resources::walls::public_walls::PublicWalls;
use crate::resources::walls::WallId;
use crate::systems::ivy::ivy_grower::wall_arch_height;
use glam::Vec3;

/// Verified IvySegmentSsbo key (rne-read qword).
pub const IVY_SEGMENT_SSBO_ID: u64 = 0x2002d2_fbdf_f3d80f;
/// Original buffer NAME, separate from the canonical Rust type identity.
pub const IVY_SEGMENT_SSBO_NAME: &str = "Ivy Segments Buffer";

/// GPU segment element, 48 B (O: `count * 0x30`; shader `instanced_ivy_branch`).
/// Field order matches the shader struct; `wall_normal`/`arch_height` mapping
/// is 待取证.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct IvySegmentSsbo {
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

pub use IvySegmentSsbo as IvySegmentGpu;

impl StorageElement for IvySegmentSsbo {
    const NAME: &'static str = IVY_SEGMENT_SSBO_NAME;
    const BUFFER_LEN: usize = 40_000;
}
impl SsboType for IvySegmentSsbo {
    type Storage = InstanceBuffer<IvySegmentSsbo>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        super::ivy_send_to_gpu_leaf::create_ivy_instance_storage::<IvySegmentSsbo>(device, initial)
    }
}

/// Pack one ivy's consecutive points into 48 B GPU segment rows. The segment
/// start/end are the deque-ordered point positions; the animated point
/// thickness fills the start/end thickness (the branch shader lerps the
/// thickness across the segment). `wall_normal` is the segment's horizontal
/// unit tangent perpendicular — confirmed against the shader
/// `instanced_ivy_branch.raster.hlsl` (`side = normalize(cross(dir,
/// wall_normal))`, so `wall_normal` must be the horizontal wall-face normal,
/// read at buffer offset 32). `arch_height` is the owning wall's arch height
/// (`ivy.source_60` = wall id, shader offset 44, `base = arch_height -
/// min(hstart, 0.0)`), falling back to 0.0 when the wall is gone.
pub fn pack_ivy_segments(ivy: &Ivy, walls: &PublicWalls) -> Vec<IvySegmentSsbo> {
    let arch_height = walls
        .get(WallId(ivy.source_60))
        .map_or(0.0, wall_arch_height);
    ivy.points
        .iter()
        .zip(ivy.points.iter().skip(1))
        .map(|(a, b)| {
            let d = b.position - a.position;
            let len = d.length().max(1e-20);
            let normal = Vec3::new(-d.z / len, 0.0, d.x / len);
            IvySegmentSsbo {
                start_pos: a.position,
                start_thickness: a.thickness,
                end_pos: b.position,
                end_thickness: b.thickness,
                wall_normal: normal,
                arch_height,
            }
        })
        .collect()
}

/// `ivy_send_to_gpu_segments` body: collect all segments, pack 48 B rows and
/// upload to the IvySegmentSsbo buffer, updating the instance count. Returns
/// the packed count. An unregistered buffer skips the upload silently (O).
pub fn upload_ivy_segments(
    storage: &IvyStorage,
    walls: &PublicWalls,
    ssbos: &mut AssetSsboLibrary,
    device: &dyn GraphicsDevice,
) -> usize {
    let elements: Vec<IvySegmentSsbo> = storage
        .iter()
        .flat_map(|(_, ivy)| pack_ivy_segments(ivy, walls))
        .collect();
    let count = elements.len();
    let Some(buffer) = ssbos.get_mut::<IvySegmentSsbo>() else {
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
        assert_eq!(core::mem::size_of::<IvySegmentSsbo>(), 48);
        let mut storage = IvyStorage::new();
        let id = storage.spawn_ivy(Vec3::ZERO, Vec3::X, 1.0..2.0, 3..4, 0);
        let ivy = storage.get_mut(id).unwrap();
        ivy.add_point(Vec3::X, Vec3::X);
        let rows = pack_ivy_segments(ivy, &PublicWalls::default());
        assert_eq!(rows.len(), 1);
        // Verified start/end mapping (shader `instanced_ivy_branch.raster.hlsl`).
        assert_eq!(rows[0].start_pos, Vec3::ZERO);
        assert_eq!(rows[0].end_pos, Vec3::X);
        // A missing owning wall leaves arch_height at zero.
        assert_eq!(rows[0].arch_height, 0.0);
    }

    #[test]
    fn verified_key_is_the_documented_constant() {
        assert_eq!(
            crate::asset_libraries::ssbo_library::SsboId::of::<IvySegmentSsbo>()
                .0
                 .0,
            IVY_SEGMENT_SSBO_ID
        );
    }
}

impl crate::render::pass_builder::pass_resource_access::SsboType for IvySegmentSsbo {
    fn rg_buffer_hash() -> u64 {
        crate::asset_libraries::ssbo_library::SsboId::of::<Self>()
            .0
             .0
    }
}
