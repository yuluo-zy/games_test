//! Source-owned constructor output -> current sparse SSBO library. These owners
//! complement the main wall InstancedWall; they never replace its allocation.
//! Rebuild replaces a complete part, deletion releases it, failed allocation or
//! device upload retains the pending revision for the next flush.
use bevy_ecs::prelude::*;
use country_core::{
    asset_libraries::{runtime::AssetLibraryRuntime, ssbo_library::AssetSsboLibrary},
    geometry::instanced_wall::BrickTransformSsbo,
    render::sparse_instance_buffer::{
        SparseInstanceBuffer, SparseInstanceBufferAllocation, SparseSsbo,
    },
    resources::{roofs::roof_buffers::WoodenPlankSsbo, walls::public_walls::OnWallDeleted},
};
use hashbrown::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ConstructorPart {
    Arches,
    Gates,
    Lintels,
    Fences,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ConstructorKey {
    pub wall: u64,
    pub part: ConstructorPart,
}
struct Part<T> {
    allocation: Option<SparseInstanceBufferAllocation<T>>,
    pending: Option<Vec<T>>,
}
impl<T> Default for Part<T> {
    fn default() -> Self {
        Self {
            allocation: None,
            pending: None,
        }
    }
}
#[derive(Resource, Default)]
pub struct ConstructorGpuOutput {
    bricks: HashMap<ConstructorKey, Part<BrickTransformSsbo>>,
    planks: HashMap<ConstructorKey, Part<WoodenPlankSsbo>>,
    /// Capacity and physical-storage errors are observable; never discarded.
    pub failures: HashMap<ConstructorKey, String>,
    pub last_device_error: Option<String>,
}
impl ConstructorGpuOutput {
    pub fn replace_bricks(&mut self, key: ConstructorKey, records: Vec<BrickTransformSsbo>) {
        self.bricks.entry(key).or_default().pending = Some(records);
    }
    pub fn replace_planks(&mut self, key: ConstructorKey, records: Vec<WoodenPlankSsbo>) {
        self.planks.entry(key).or_default().pending = Some(records);
    }
    pub fn pending_bricks(&self, key: ConstructorKey) -> Option<&[BrickTransformSsbo]> {
        self.bricks.get(&key)?.pending.as_deref()
    }
    pub fn brick_allocation(
        &self,
        key: ConstructorKey,
    ) -> Option<&SparseInstanceBufferAllocation<BrickTransformSsbo>> {
        self.bricks.get(&key)?.allocation.as_ref()
    }
    pub fn plank_allocation(
        &self,
        key: ConstructorKey,
    ) -> Option<&SparseInstanceBufferAllocation<WoodenPlankSsbo>> {
        self.planks.get(&key)?.allocation.as_ref()
    }
    pub fn remove_wall(&mut self, wall: u64) {
        self.bricks.retain(|key, _| key.wall != wall);
        self.planks.retain(|key, _| key.wall != wall);
        self.failures.retain(|key, _| key.wall != wall);
    }
}
fn write_parts<T: SparseSsbo>(
    storage: &mut SparseInstanceBuffer<T>,
    parts: &mut HashMap<ConstructorKey, Part<T>>,
    failures: &mut HashMap<ConstructorKey, String>,
) {
    storage.take_out_waste();
    for (&key, part) in parts {
        let Some(pending) = &part.pending else {
            continue;
        };
        let allocation = part
            .allocation
            .get_or_insert_with(|| storage.new_allocation());
        if !storage.owns_allocation(allocation) {
            failures.insert(
                key,
                "constructor allocation belongs to another physical storage".into(),
            );
            continue;
        }
        match storage.writer(allocation).replace_all(pending) {
            Ok(()) => {
                part.pending = None;
                failures.remove(&key);
            }
            Err(_) => {
                failures.insert(
                    key,
                    "constructor sparse instance capacity exhausted; revision remains pending"
                        .into(),
                );
            }
        }
    }
}
/// Run after source constructors and before the canonical sparse upload. There
/// is one physical buffer: its active chunks are consumed by W21/W27 and Nani.
pub fn flush_constructor_output(
    runtime: Res<AssetLibraryRuntime>,
    mut library: ResMut<AssetSsboLibrary>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    let ConstructorGpuOutput {
        bricks,
        planks,
        failures,
        last_device_error,
    } = &mut output.gpu;
    *last_device_error = None;
    if let Some(storage) = library.get_mut::<BrickTransformSsbo>() {
        write_parts(storage, bricks, failures);
        if let Err(error) = storage.try_gpu_flush(runtime.buffer_device()) {
            *last_device_error = Some(error.to_string());
        }
    } else if !bricks.is_empty() {
        *last_device_error = Some("BrickTransformSsbo is not registered".into());
    }
    if let Some(storage) = library.get_mut::<WoodenPlankSsbo>() {
        write_parts(storage, planks, failures);
        if let Err(error) = storage.try_gpu_flush(runtime.buffer_device()) {
            *last_device_error = Some(error.to_string());
        }
    } else if !planks.is_empty() {
        *last_device_error = Some("WoodenPlankSsbo is not registered".into());
    }
}
pub fn delete_constructor_output(
    mut deleted: EventReader<OnWallDeleted>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    for event in deleted.read() {
        output.gpu.remove_wall(event.0.0);
    }
}

/// The frozen geometry owner already emits all64 meaningful bytes. Preserve
/// their bit patterns instead of treating the affine as a row-major matrix or
/// applying default flags (original support/fence flags are zero).
pub fn wooden_plank(
    record: &country_core::systems::wall::construction::structures::WallWoodenPlankInstance64B,
) -> WoodenPlankSsbo {
    let a = &record.affine;
    WoodenPlankSsbo {
        transform: country_core::resources::render::Affine3Packed {
            cols: std::array::from_fn(|i| glam::Vec3::new(a[i * 3], a[i * 3 + 1], a[i * 3 + 2])),
        },
        gate_seed: record.segment_ordinal as i32,
        piece_seed: record.source_index as i32,
        wall_id: record.wall_id,
        flags: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::render::ssbo::rhapsody_boundary::{
        BackendError, Buffer, BufferDesc, GraphicsDevice,
    };
    /// CPU-shadow ownership fixture only. An attempted GPU upload fails
    /// explicitly; this test cannot accidentally count a fake upload as passed.
    struct TestDevice(u64);
    impl GraphicsDevice for TestDevice {
        fn create_buffer(
            &self,
            desc: &BufferDesc,
            _name: &str,
            _data: Option<&[u8]>,
        ) -> Result<Buffer, BackendError> {
            Ok(Buffer {
                id: self.0,
                byte_size: desc.size as u64,
                ..Default::default()
            })
        }
        fn update_buffer_bytes(
            &self,
            _buffer: &Buffer,
            _data: &[u8],
            _ranges: &[std::ops::Range<usize>],
            _offset: usize,
        ) -> Result<(), BackendError> {
            Err(BackendError::invalid_usage(
                "CPU-shadow fixture has no GPU upload device",
            ))
        }
    }
    #[test]
    fn source_replacement_deletion_and_foreign_storage_keep_real_sparse_ownership() {
        let key = ConstructorKey {
            wall: 44,
            part: ConstructorPart::Arches,
        };
        let mut owners = ConstructorGpuOutput::default();
        owners.replace_bricks(
            key,
            vec![
                BrickTransformSsbo {
                    seed: 7,
                    ..Default::default()
                };
                35
            ],
        );
        let mut storage =
            SparseInstanceBuffer::<BrickTransformSsbo>::new_storage_buffer(&TestDevice(1), None);
        write_parts(&mut storage, &mut owners.bricks, &mut owners.failures);
        assert_eq!(
            storage
                .to_draw_multi_instanced()
                .iter()
                .map(|(_, n)| *n)
                .sum::<u32>(),
            35
        );
        let allocation = owners.brick_allocation(key).unwrap();
        assert_eq!(
            storage
                .allocation_to_draw_multi_instanced(allocation)
                .iter()
                .map(|(_, n)| *n)
                .sum::<u32>(),
            35
        );
        assert_eq!(storage.data[0].seed, 7);
        owners.replace_bricks(
            key,
            vec![
                BrickTransformSsbo {
                    seed: 9,
                    ..Default::default()
                };
                2
            ],
        );
        write_parts(&mut storage, &mut owners.bricks, &mut owners.failures);
        assert_eq!(storage.to_draw_multi_instanced(), vec![(0, 2)]);
        assert_eq!(storage.data[0].seed, 9);
        let mut foreign =
            SparseInstanceBuffer::<BrickTransformSsbo>::new_storage_buffer(&TestDevice(2), None);
        owners.replace_bricks(key, vec![BrickTransformSsbo::default()]);
        write_parts(&mut foreign, &mut owners.bricks, &mut owners.failures);
        assert!(owners.failures.contains_key(&key));
        assert!(owners.pending_bricks(key).is_some());
        assert!(foreign.to_draw_multi_instanced().is_empty());
        owners.remove_wall(44);
        storage.take_out_waste();
        assert!(storage.to_draw_multi_instanced().is_empty());
    }
    #[test]
    fn plank_projection_preserves_all_sixty_four_output_bytes() {
        let source =
            country_core::systems::wall::construction::structures::WallWoodenPlankInstance64B {
                affine: std::array::from_fn(|i| f32::from_bits(0x3f000000 + i as u32)),
                segment_ordinal: u32::MAX,
                source_index: 0xdeadbeef,
                wall_id: 43,
            };
        let plank = wooden_plank(&source);
        let mut bytes = [0; 64];
        for (i, v) in plank
            .transform
            .cols
            .iter()
            .flat_map(|v| v.to_array())
            .enumerate()
        {
            bytes[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
        }
        bytes[48..52].copy_from_slice(&plank.gate_seed.to_le_bytes());
        bytes[52..56].copy_from_slice(&plank.piece_seed.to_le_bytes());
        bytes[56..60].copy_from_slice(&plank.wall_id.to_le_bytes());
        bytes[60..64].copy_from_slice(&plank.flags.to_le_bytes());
        assert_eq!(bytes, source.observed_bytes());
    }
}
