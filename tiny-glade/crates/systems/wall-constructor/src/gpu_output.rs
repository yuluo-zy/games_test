//! Source-owned constructor output -> current sparse SSBO library. These owners
//! complement the main wall InstancedWall; they never replace its allocation.
//! Original source points clear and append shared per-wall owners directly.
//! Capacity failure keeps successful prefixes; physical upload failures keep
//! dirty chunks for the real device retry. Deletion releases the owned chunks.
use bevy_ecs::prelude::*;
use country_core::{
    asset_libraries::{runtime::AssetLibraryRuntime, ssbo_library::AssetSsboLibrary},
    geometry::instanced_wall::BrickTransformSsbo,
    render::sparse_instance_buffer::{SparseInstanceBuffer, SparseInstanceBufferAllocation},
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
#[derive(Resource, Default)]
pub struct ConstructorGpuOutput {
    bricks: HashMap<u64, SparseInstanceBufferAllocation<BrickTransformSsbo>>,
    planks: HashMap<u64, SparseInstanceBufferAllocation<WoodenPlankSsbo>>,
    /// Capacity and physical-storage errors are observable; never discarded.
    pub failures: HashMap<ConstructorKey, String>,
    pub last_device_error: Option<String>,
}
impl ConstructorGpuOutput {
    pub fn append_bricks(
        &mut self,
        storage: &mut SparseInstanceBuffer<BrickTransformSsbo>,
        key: ConstructorKey,
        records: &[BrickTransformSsbo],
    ) -> usize {
        let owner = self
            .bricks
            .entry(key.wall)
            .or_insert_with(|| storage.new_allocation());
        assert!(
            storage.owns_allocation(owner),
            "arch/gate owner belongs to another physical storage"
        );
        let mut writer = storage.append_writer(owner);
        let mut failures = 0usize;
        for record in records {
            if writer.push(record).is_err() {
                failures += 1;
            }
        }
        if failures != 0 {
            self.failures.insert(key, format!("{failures} constructor brick pushes exhausted capacity; successful prefix remains"));
        }
        failures
    }
    /// Original22 arch constructor clears the one shared arch/gate owner.
    pub fn clear_bricks(
        &mut self,
        storage: &mut SparseInstanceBuffer<BrickTransformSsbo>,
        wall: u64,
    ) {
        let owner = self
            .bricks
            .entry(wall)
            .or_insert_with(|| storage.new_allocation());
        assert!(
            storage.owns_allocation(owner),
            "arch/gate owner belongs to another physical storage"
        );
        storage.writer(owner).clear();
        self.failures.retain(|key, _| {
            key.wall != wall
                || !matches!(key.part, ConstructorPart::Arches | ConstructorPart::Gates)
        });
    }
    /// Original per-wall64B owner is shared by lintels/fences. Source systems
    /// append at their actual22/28 execution points, never deferred/reordered by
    /// HashMap iteration during44 upload. Every failed push is still attempted.
    pub fn append_planks(
        &mut self,
        storage: &mut SparseInstanceBuffer<WoodenPlankSsbo>,
        key: ConstructorKey,
        records: &[WoodenPlankSsbo],
    ) -> usize {
        let owner = self
            .planks
            .entry(key.wall)
            .or_insert_with(|| storage.new_allocation());
        assert!(
            storage.owns_allocation(owner),
            "plank owner belongs to another physical storage"
        );
        let mut writer = storage.append_writer(owner);
        let mut failures = 0usize;
        for record in records {
            if writer.push(record).is_err() {
                failures += 1;
            }
        }
        if failures != 0 {
            self.failures.insert(key, format!("{failures} constructor plank pushes exhausted capacity; successful prefix remains"));
        }
        failures
    }
    pub fn clear_planks(&mut self, storage: &mut SparseInstanceBuffer<WoodenPlankSsbo>, wall: u64) {
        let owner = self
            .planks
            .entry(wall)
            .or_insert_with(|| storage.new_allocation());
        assert!(
            storage.owns_allocation(owner),
            "plank owner belongs to another physical storage"
        );
        storage.writer(owner).clear();
        self.failures.retain(|key, _| {
            key.wall != wall
                || !matches!(key.part, ConstructorPart::Lintels | ConstructorPart::Fences)
        });
    }
    pub fn brick_allocation(
        &self,
        key: ConstructorKey,
    ) -> Option<&SparseInstanceBufferAllocation<BrickTransformSsbo>> {
        self.bricks.get(&key.wall)
    }
    pub fn plank_allocation(
        &self,
        key: ConstructorKey,
    ) -> Option<&SparseInstanceBufferAllocation<WoodenPlankSsbo>> {
        self.planks.get(&key.wall)
    }
    pub fn remove_wall(&mut self, wall: u64) {
        self.bricks.retain(|id, _| *id != wall);
        self.planks.retain(|id, _| *id != wall);
        self.failures.retain(|key, _| key.wall != wall);
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
        failures: _,
        last_device_error,
    } = &mut output.gpu;
    *last_device_error = None;
    if let Some(storage) = library.get_mut::<BrickTransformSsbo>() {
        if let Err(error) = storage.try_gpu_flush(runtime.buffer_device()) {
            *last_device_error = Some(error.to_string());
        }
    } else if !bricks.is_empty() {
        *last_device_error = Some("BrickTransformSsbo is not registered".into());
    }
    if let Some(storage) = library.get_mut::<WoodenPlankSsbo>() {
        if let Err(error) = storage.try_gpu_flush(runtime.buffer_device()) {
            *last_device_error = Some(error.to_string());
        }
    } else if !planks.is_empty() {
        *last_device_error = Some("WoodenPlankSsbo is not registered".into());
    }
}
/// Original19 clear, after publication and before22/28 source appenders.
pub fn clear_constructor_planks(
    mut changed: EventReader<country_core::resources::walls::public_walls::OnWallChanged>,
    walls: Res<country_core::resources::walls::public_walls::PublicWalls>,
    mut library: ResMut<AssetSsboLibrary>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    for event in changed.read() {
        walls
            .get(event.0)
            .expect("plank clear changed event requires published wall");
        let storage = library
            .get_mut::<WoodenPlankSsbo>()
            .expect("canonical WoodenPlankSsbo must exist before wall construction");
        output.gpu.clear_planks(storage, event.0.0);
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
    fn arch_gate_tail_owner_clear_delete_and_foreign_storage_are_real() {
        let key = ConstructorKey {
            wall: 44,
            part: ConstructorPart::Arches,
        };
        let gates = ConstructorKey {
            part: ConstructorPart::Gates,
            ..key
        };
        let mut owners = ConstructorGpuOutput::default();
        let mut storage =
            SparseInstanceBuffer::<BrickTransformSsbo>::new_storage_buffer(&TestDevice(1), None);
        owners.clear_bricks(&mut storage, key.wall);
        assert_eq!(
            owners.append_bricks(
                &mut storage,
                key,
                &vec![
                    BrickTransformSsbo {
                        seed: 7,
                        ..Default::default()
                    };
                    35
                ]
            ),
            0
        );
        assert_eq!(
            owners.append_bricks(
                &mut storage,
                gates,
                &vec![
                    BrickTransformSsbo {
                        seed: 9,
                        ..Default::default()
                    };
                    2
                ]
            ),
            0
        );
        assert!(std::ptr::eq(
            owners.brick_allocation(key).unwrap(),
            owners.brick_allocation(gates).unwrap()
        ));
        assert_eq!(storage.to_draw_multi_instanced(), vec![(0, 37)]);
        assert_eq!(storage.data[34].seed, 7);
        assert_eq!(storage.data[35].seed, 9);
        owners.clear_bricks(&mut storage, key.wall);
        assert!(storage.to_draw_multi_instanced().is_empty());
        assert_eq!(
            owners.append_bricks(
                &mut storage,
                key,
                &vec![
                    BrickTransformSsbo {
                        seed: 11,
                        ..Default::default()
                    };
                    2
                ]
            ),
            0
        );
        assert_eq!(storage.to_draw_multi_instanced(), vec![(0, 2)]);
        owners.remove_wall(44);
        storage.take_out_waste();
        assert!(storage.to_draw_multi_instanced().is_empty());
        owners.append_bricks(&mut storage, key, &[BrickTransformSsbo::default()]);
        drop(storage); // Avoid simultaneously allocating two full192MiB shadows.
        let mut foreign =
            SparseInstanceBuffer::<BrickTransformSsbo>::new_storage_buffer(&TestDevice(2), None);
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| owners.append_bricks(
                &mut foreign,
                key,
                &[BrickTransformSsbo::default()]
            )))
            .is_err()
        );
        assert!(foreign.to_draw_multi_instanced().is_empty());
        owners.remove_wall(44);
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
