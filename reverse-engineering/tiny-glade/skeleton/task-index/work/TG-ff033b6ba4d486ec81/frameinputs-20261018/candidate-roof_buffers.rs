//! B WoodenPlankSsbo: the CPU writer, sparse constructor and original SPIR-V
//! agree on 64B records. Evidence: WOODEN-PLANK-CONTRACT.md in TG-ff033b6ba4d486ec81.
use bevy_ecs::resource::Resource;
use glam::{Affine3A, Vec3};

use crate::asset_libraries::ssbo_library::SsboType;
use crate::render::sparse_instance_buffer::{SparseInstanceBuffer, SparseSsbo, SLOTS_PER_CHUNK};
use crate::render::ssbo::{rhapsody_boundary::GraphicsDevice, StorageElement};
use crate::resources::render::Affine3Packed;

/// RoofBuffers' other fields still require their original producer/consumer.
#[derive(Resource, Clone, Debug, Default)]
pub struct RoofBuffers {
    pub(crate) _opaque: (),
}

/// Exact original wooden-plank GPU record. Seeds are signed SPIR-V i32s;
/// wall_id and flags are u32s. Affine3Packed stores four compact Vec3 columns.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WoodenPlankSsbo {
    pub transform: Affine3Packed,
    pub gate_seed: i32,
    pub piece_seed: i32,
    pub wall_id: u32,
    pub flags: u32,
}
// SAFETY: four compact f32x3 columns followed by four 32-bit scalar words,
// with no padding. All constituent scalar bit patterns are valid.
unsafe impl bytemuck::Zeroable for WoodenPlankSsbo {}
unsafe impl bytemuck::Pod for WoodenPlankSsbo {}
const _: [(); 64] = [(); core::mem::size_of::<WoodenPlankSsbo>()];
const _: [(); 4] = [(); core::mem::align_of::<WoodenPlankSsbo>()];

impl Default for WoodenPlankSsbo {
    /// Sparse CPU-shadow default @1412e520a..52dd, not a zero record.
    fn default() -> Self {
        Self {
            transform: Affine3Packed::from_cols(Vec3::X, Vec3::Y, Vec3::Z, Vec3::ZERO),
            gate_seed: 0,
            piece_seed: 0,
            wall_id: 0,
            flags: 1,
        }
    }
}
impl WoodenPlankSsbo {
    /// Both native timberframe constructors compact their complete Affine3A
    /// via 14089d560, then copy {0,0,0,2} from 142b14ea0 into the tail.
    pub fn timberframe(transform: Affine3A) -> Self {
        Self {
            transform: transform.into(),
            gate_seed: 0,
            piece_seed: 0,
            wall_id: 0,
            flags: 2,
        }
    }
}
impl StorageElement for WoodenPlankSsbo {
    const NAME: &'static str = "WoodenPlank";
    const BUFFER_LEN: usize = 0x1000 * SLOTS_PER_CHUNK;
}
// SAFETY: original constructor 1412e5170: 4096 chunks × 32 records, 8MiB
// shadow, 4097 byte-map entries with terminal32, two 64-word bitsets.
unsafe impl SparseSsbo for WoodenPlankSsbo {
    const CHUNK_COUNT: usize = 0x1000;
}
impl SsboType for WoodenPlankSsbo {
    type Storage = SparseInstanceBuffer<Self>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        SparseInstanceBuffer::<Self>::new_storage_buffer(device, initial)
    }
}
impl crate::render::pass_builder::pass_resource_access::SsboType for WoodenPlankSsbo {
    fn rg_buffer_hash() -> u64 {
        crate::asset_libraries::ssbo_library::SsboId::of::<Self>()
            .0
             .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{asset_libraries::ssbo_library::AssetSsboLibrary, render::test_device::TestDevice};

    #[test]
    fn native_wooden_sparse_default_and_timberframe_records_are_distinct() {
        assert_eq!(
            crate::asset_libraries::ssbo_library::SsboId::of::<WoodenPlankSsbo>().0.0,
            0x153e2eb3e8651534,
        );
        let library = AssetSsboLibrary::new();
        library.add_new::<WoodenPlankSsbo>(&TestDevice(1));
        let buffer = library.get::<WoodenPlankSsbo>().unwrap();
        assert_eq!(buffer.buffer.buffer.byte_size, 0x800000);
        assert_eq!(buffer.byte_map.len(), 0x1001);
        assert_eq!(buffer.byte_map[0x1000], 32);
        assert_eq!(buffer.data.len(), 0x20000);
        assert_eq!(buffer.free_chunks.read().first(), Some(&4095));
        assert_eq!(buffer.free_chunks.read().last(), Some(&0));
        let default_words: &[u32] = bytemuck::cast_slice(&buffer.data[..1]);
        assert_eq!(
            default_words,
            &[
                1f32.to_bits(),
                0,
                0,
                0,
                1f32.to_bits(),
                0,
                0,
                0,
                1f32.to_bits(),
                0,
                0,
                0,
                0,
                0,
                0,
                1
            ]
        );
        let beam = WoodenPlankSsbo::timberframe(Affine3A::from_scale_rotation_translation(
            Vec3::new(0.2, 4.0, 0.2),
            glam::Quat::IDENTITY,
            Vec3::new(3.0, 1.0, 5.0),
        ));
        assert_eq!(
            beam.transform.cols,
            [
                Vec3::new(0.2, 0.0, 0.0),
                Vec3::new(0.0, 4.0, 0.0),
                Vec3::new(0.0, 0.0, 0.2),
                Vec3::new(3.0, 1.0, 5.0)
            ]
        );
        assert_eq!(
            (beam.gate_seed, beam.piece_seed, beam.wall_id, beam.flags),
            (0, 0, 0, 2)
        );
        assert_eq!(core::mem::offset_of!(WoodenPlankSsbo, gate_seed), 48);
        assert_eq!(core::mem::offset_of!(WoodenPlankSsbo, piece_seed), 52);
        assert_eq!(core::mem::offset_of!(WoodenPlankSsbo, wall_id), 56);
        assert_eq!(core::mem::offset_of!(WoodenPlankSsbo, flags), 60);
    }
}
