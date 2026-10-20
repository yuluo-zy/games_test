//! B upload_dormer_holes_to_gpu@1407b1d40/14341B. The actual current
//! blueprint's canonical48B list feeds a fixed15x15 spatial table, then the
//! same App's registered count/hole buffers. This is middle tuple member3;
//! independent dormer-roof geometry and real record producers remain required.

use bevy_ecs::system::{Res, ResMut};
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::render::device::GraphicsDeviceResource;
use country_core::render::ssbo::rhapsody_boundary::GraphicsDevice;
use country_core::resources::walls::dormers::{DormerHoleCountPerCellSsbo, DormerHoleSsbo};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use glam::Vec2;
use utils::geometry::rectangle::Rectangle2d;

use crate::blueprint::DecoratorBlueprints;

pub const GRID_COLS: usize = 15;
pub const GRID_ROWS: usize = 15;
pub const CELL_COUNT: usize = GRID_COLS * GRID_ROWS;
pub const COUNT_BUFFER_LEN: usize = CELL_COUNT;
pub const CELL_CAP: usize = 50;
pub const GRID_ORIGIN: f32 = 65.0;
pub const GRID_CELL_SIZE: f32 = f32::from_bits(0x410a_aaab);

/// Native floor followed by saturating f32 -> i32, including NaN ->0.
pub fn world_space_to_grid_coord_unclamped(world: f32) -> i32 {
    ((world + GRID_ORIGIN) / GRID_CELL_SIZE).floor() as i32
}

/// Only minima are lower-clamped and maxima upper-clamped. A box wholly
/// outside an edge has an empty inclusive range; it is not forced into a cell.
pub fn get_cells_from_bbx(min: Vec2, max: Vec2) -> Vec<usize> {
    let col_min = world_space_to_grid_coord_unclamped(min.x).max(0);
    let row_min = world_space_to_grid_coord_unclamped(min.y).max(0);
    let col_max = world_space_to_grid_coord_unclamped(max.x).min(GRID_COLS as i32 - 1);
    let row_max = world_space_to_grid_coord_unclamped(max.y).min(GRID_ROWS as i32 - 1);
    let mut result = Vec::new();
    for row in row_min..=row_max {
        for col in col_min..=col_max {
            result.push((row * GRID_COLS as i32 + col) as usize);
        }
    }
    result
}

/// Per-invocation CPU state, not a second authoritative resource. Every cell
/// has all50 canonical rows initialized, even though native accepts only49.
#[derive(Debug)]
pub struct DormerHoleGrid {
    pub counts: Vec<DormerHoleCountPerCellSsbo>,
    pub cells: Vec<Vec<DormerHoleSsbo>>,
}

fn record_rectangle(record: &DormerHoleSsbo) -> Rectangle2d {
    let words = record.0;
    // The original creates this value directly, without constructor guards or
    // normalizing raw floats. Its geometry operation is the existing aabb().
    Rectangle2d {
        axis: Vec2::new(f32::from_bits(words[8]), f32::from_bits(words[9])),
        center: Vec2::new(f32::from_bits(words[0]), f32::from_bits(words[2])),
        size: Vec2::new(f32::from_bits(words[4]), f32::from_bits(words[6])),
    }
}

pub fn build_dormer_hole_grid(blueprints: &DecoratorBlueprints) -> DormerHoleGrid {
    // Preserve the original full initialized row and225 cloned cell Vecs.
    let row = vec![DormerHoleSsbo::dummy(); CELL_CAP];
    let mut cells = vec![row; CELL_COUNT];
    let mut counts = vec![DormerHoleCountPerCellSsbo(0); COUNT_BUFFER_LEN];
    for blueprint in blueprints.map.values() {
        for record in &blueprint.dormer_holes {
            let aabb = record_rectangle(record).aabb();
            for cell in get_cells_from_bbx(aabb.min, aabb.max) {
                let next = counts[cell].0 + 1;
                if next < CELL_CAP as u32 {
                    counts[cell].0 = next;
                    cells[cell][next as usize - 1] = *record;
                } else {
                    // Native level2 warning; count remains49 and no row is
                    // overwritten. The original's bounds checks still panic.
                    log::warn!(
                        "Exceeded maximum count {} of dormer holes per spatial accel grid cell at cell id {}",
                        CELL_CAP,
                        cell
                    );
                }
            }
        }
    }
    DormerHoleGrid { counts, cells }
}

/// Actual original four dependencies. ResMut on the same existing SSBO owner
/// is the safe host adaptation for publishing InstanceBuffer counts; native
/// used Res with internal mutable storage. No parallel BufferHost is created.
pub fn upload_dormer_holes_to_gpu(
    device: Res<GraphicsDeviceResource>,
    mut ssbos: ResMut<AssetSsboLibrary>,
    blueprints: Res<DecoratorBlueprints>,
    _signifiers: Res<UiSignifierStream>,
) {
    upload_dormer_holes(&blueprints, &mut ssbos, device.device());
}

/// Same production body, with this App's existing device boundary forwarded
/// directly by the ECS entry. CPU tests record its actual library writes;
/// they do not manufacture a Vulkan Device or claim an initialized App/GPU.
fn upload_dormer_holes(
    blueprints: &DecoratorBlueprints,
    ssbos: &mut AssetSsboLibrary,
    device: &dyn GraphicsDevice,
) {
    let grid = build_dormer_hole_grid(blueprints);
    if let Some(counts) = ssbos.get_mut::<DormerHoleCountPerCellSsbo>() {
        counts.buffer.gpu_write(device, &grid.counts).unwrap();
        counts.instance_count = grid.counts.len();
    }
    if let Some(holes) = ssbos.get_mut::<DormerHoleSsbo>() {
        // All50 rows per cell are retained, including unoccupied dummy rows.
        let flat: Vec<DormerHoleSsbo> = grid.cells.into_iter().flatten().collect();
        holes.buffer.gpu_write(device, &flat).unwrap();
        holes.instance_count = flat.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blueprint::DecoratorBlueprint;
    use core::ops::Range;
    use country_core::asset_libraries::ssbo_library::SsboType;
    use country_core::render::ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc};
    use country_core::render::ssbo::{InstanceBuffer, StorageElement};
    use country_core::resources::walls::decorator_subtype::DecoratorCollision;
    use glam::Vec3;
    use std::sync::Mutex;

    fn record(position: Vec3, size: Vec3, axis: Vec2, payload: u32) -> DormerHoleSsbo {
        DormerHoleSsbo::gothic(payload, position, size, [axis.x, axis.y, 2.5, 0.05])
    }

    fn blueprints(records: Vec<DormerHoleSsbo>) -> DecoratorBlueprints {
        let mut blueprint = DecoratorBlueprint::new();
        blueprint.dormer_holes = records;
        let mut result = DecoratorBlueprints::default();
        result.insert(17, blueprint);
        result
    }

    #[test]
    fn canonical_lists_preserve_full_payload_and_fixed_fifty_row_stride() {
        let a = record(
            Vec3::new(-60.0, 123.0, -60.0),
            Vec3::ONE,
            Vec2::X,
            0xdead_beef,
        );
        let b = record(
            Vec3::new(60.0, -17.0, 60.0),
            Vec3::ONE,
            Vec2::X,
            0x7654_3210,
        );
        let grid = build_dormer_hole_grid(&blueprints(vec![a, b]));
        assert_eq!(grid.counts.len(), 225);
        assert_eq!(grid.cells.len(), 225);
        assert!(grid.cells.iter().all(|cell| cell.len() == 50));
        assert_eq!(grid.counts[0].0, 1);
        assert_eq!(grid.counts[224].0, 1);
        assert_eq!(grid.counts.iter().map(|count| count.0).sum::<u32>(), 2);
        let flat: Vec<_> = grid.cells.into_iter().flatten().collect();
        assert_eq!(flat.len(), 11250);
        assert_eq!(flat[0], a);
        assert_eq!(flat[224 * 50], b);
        assert_eq!(flat[49], DormerHoleSsbo::dummy());
        assert_eq!(flat[224 * 50 + 49], DormerHoleSsbo::dummy());
        assert!(
            flat[1..224 * 50]
                .iter()
                .all(|row| *row == DormerHoleSsbo::dummy())
        );
    }

    #[test]
    fn collider_footprints_are_not_dormer_hole_inputs() {
        let mut blueprint = DecoratorBlueprint::new();
        blueprint.set_colliders(vec![DecoratorCollision::empty()]);
        let mut current = DecoratorBlueprints::default();
        current.insert(3, blueprint);
        let grid = build_dormer_hole_grid(&current);
        assert!(grid.counts.iter().all(|count| count.0 == 0));
        assert!(
            grid.cells
                .iter()
                .flatten()
                .all(|row| *row == DormerHoleSsbo::dummy())
        );
    }

    #[test]
    fn raw_record_axis_projects_xz_and_saturated_coordinates_keep_empty_outside_ranges() {
        let rotated = record(
            Vec3::new(0.0, 999.0, 0.0),
            Vec3::new(30.0, 7.0, 1.0),
            Vec2::Y,
            9,
        );
        let grid = build_dormer_hole_grid(&blueprints(vec![rotated]));
        let occupied: Vec<_> = grid
            .counts
            .iter()
            .enumerate()
            .filter_map(|(cell, count)| (count.0 != 0).then_some(cell))
            .collect();
        assert_eq!(occupied, [82, 97, 112, 127, 142]);
        assert_eq!(world_space_to_grid_coord_unclamped(f32::INFINITY), i32::MAX);
        assert_eq!(
            world_space_to_grid_coord_unclamped(f32::NEG_INFINITY),
            i32::MIN
        );
        assert_eq!(world_space_to_grid_coord_unclamped(f32::NAN), 0);
        assert_eq!(world_space_to_grid_coord_unclamped(0.0), 7);
        for (min, max) in [
            (Vec2::new(-100.0, -1.0), Vec2::new(-90.0, 1.0)),
            (Vec2::new(90.0, -1.0), Vec2::new(100.0, 1.0)),
            (Vec2::new(-1.0, -100.0), Vec2::new(1.0, -90.0)),
            (Vec2::new(-1.0, 90.0), Vec2::new(1.0, 100.0)),
        ] {
            assert!(get_cells_from_bbx(min, max).is_empty());
        }
        assert_eq!(
            get_cells_from_bbx(Vec2::splat(-65.0), Vec2::splat(-65.0)),
            [0]
        );
        assert_eq!(
            get_cells_from_bbx(Vec2::splat(-100.0), Vec2::splat(100.0)),
            (0..225).collect::<Vec<_>>()
        );
        let nan_axis = record(Vec3::ZERO, Vec3::ONE, Vec2::splat(f32::NAN), 77);
        let grid = build_dormer_hole_grid(&blueprints(vec![nan_axis]));
        assert_eq!(grid.counts[0].0, 1);
        assert_eq!(grid.cells[0][0], nan_axis);
    }

    #[test]
    fn native_fiftieth_and_later_inputs_leave_count_and_rows_at_forty_nine() {
        let rows: Vec<_> = (0..51)
            .map(|payload| record(Vec3::ZERO, Vec3::ONE, Vec2::X, payload))
            .collect();
        for input_count in [49, 50, 51] {
            let grid = build_dormer_hole_grid(&blueprints(rows[..input_count].to_vec()));
            let cell = 7 * 15 + 7;
            assert_eq!(grid.counts[cell].0, 49);
            assert_eq!(&grid.cells[cell][..49], &rows[..49]);
            assert_eq!(grid.cells[cell][49], DormerHoleSsbo::dummy());
        }
    }

    #[derive(Debug)]
    struct Write {
        buffer: u64,
        bytes: Vec<u8>,
        ranges: Vec<Range<usize>>,
        offset: usize,
    }

    #[derive(Default)]
    struct Recorded {
        next_id: u64,
        attempts: usize,
        fail_on: Option<usize>,
        writes: Vec<Write>,
    }

    #[derive(Default)]
    struct Device(Mutex<Recorded>);

    impl GraphicsDevice for Device {
        fn create_buffer(
            &self,
            desc: &BufferDesc,
            _: &str,
            initial: Option<&[u8]>,
        ) -> Result<Buffer, BackendError> {
            assert!(initial.is_none());
            let mut state = self.0.lock().unwrap();
            state.next_id += 1;
            Ok(Buffer {
                id: state.next_id,
                byte_size: desc.size as u64,
                ..Buffer::default()
            })
        }

        fn update_buffer_bytes(
            &self,
            buffer: &Buffer,
            data: &[u8],
            ranges: &[Range<usize>],
            offset: usize,
        ) -> Result<(), BackendError> {
            let mut state = self.0.lock().unwrap();
            state.attempts += 1;
            if state.fail_on == Some(state.attempts) {
                return Err(BackendError::invalid_usage(
                    "injected dormer-hole write failure",
                ));
            }
            state.writes.push(Write {
                buffer: buffer.id,
                bytes: data.to_vec(),
                ranges: ranges.to_vec(),
                offset,
            });
            Ok(())
        }
    }

    fn library(device: &Device) -> AssetSsboLibrary {
        let library = AssetSsboLibrary::new();
        library.add_new::<DormerHoleCountPerCellSsbo>(device);
        library.add_new::<DormerHoleSsbo>(device);
        library
    }

    fn uploaded_word(bytes: &[u8], index: usize) -> u32 {
        u32::from_le_bytes(bytes[index * 4..index * 4 + 4].try_into().unwrap())
    }

    fn uploaded_row(bytes: &[u8], index: usize) -> DormerHoleSsbo {
        DormerHoleSsbo(std::array::from_fn(|word| {
            uploaded_word(bytes, index * 12 + word)
        }))
    }

    #[test]
    fn actual_canonical_library_writes_counts_then_all_fixed_rows_and_publishes_metadata() {
        let device = Device::default();
        let mut library = library(&device);
        let hole = record(Vec3::ZERO, Vec3::ONE, Vec2::X, 0x0123_4567);
        upload_dormer_holes(&blueprints(vec![hole]), &mut library, &device);
        assert_eq!(
            library
                .get::<DormerHoleCountPerCellSsbo>()
                .unwrap()
                .instance_count,
            225
        );
        assert_eq!(
            library.get::<DormerHoleSsbo>().unwrap().instance_count,
            11250
        );
        let state = device.0.lock().unwrap();
        assert_eq!(state.attempts, 2);
        assert_eq!(state.writes.len(), 2);
        assert_eq!(
            (state.writes[0].buffer, state.writes[0].bytes.len()),
            (1, 900)
        );
        assert_eq!(
            (state.writes[1].buffer, state.writes[1].bytes.len()),
            (2, 540000)
        );
        assert_eq!(state.writes[0].ranges, [0..900]);
        assert_eq!(state.writes[1].ranges, [0..540000]);
        assert!(state.writes.iter().all(|write| write.offset == 0));
        let count_words: Vec<_> = (0..225)
            .map(|index| uploaded_word(&state.writes[0].bytes, index))
            .collect();
        assert_eq!(count_words[112], 1);
        assert_eq!(count_words.iter().sum::<u32>(), 1);
        assert_eq!(uploaded_row(&state.writes[1].bytes, 112 * 50), hole);
        assert_eq!(
            uploaded_row(&state.writes[1].bytes, 112 * 50 + 49),
            DormerHoleSsbo::dummy()
        );
    }

    #[test]
    fn failed_write_stops_later_work_and_does_not_publish_failed_metadata() {
        for failed_attempt in [1, 2] {
            let device = Device::default();
            let mut library = library(&device);
            device.0.lock().unwrap().fail_on = Some(failed_attempt);
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                upload_dormer_holes(&DecoratorBlueprints::default(), &mut library, &device)
            }));
            assert!(result.is_err());
            let state = device.0.lock().unwrap();
            assert_eq!(state.attempts, failed_attempt);
            assert_eq!(state.writes.len(), failed_attempt - 1);
            assert_eq!(
                library
                    .get::<DormerHoleCountPerCellSsbo>()
                    .unwrap()
                    .instance_count,
                if failed_attempt == 1 { 0 } else { 225 }
            );
            assert_eq!(library.get::<DormerHoleSsbo>().unwrap().instance_count, 0);
        }
    }

    #[test]
    fn record_move_remove_and_empty_frames_replace_real_uploaded_membership() {
        let device = Device::default();
        let mut library = library(&device);
        let first = record(Vec3::new(-60.0, 0.0, -60.0), Vec3::ONE, Vec2::X, 17);
        let second = record(Vec3::new(60.0, 0.0, 60.0), Vec3::ONE, Vec2::X, 17);
        let current = blueprints(vec![first]);
        // Repeated identical input still uploads; the actual consumer has no
        // previous-map/Arc identity early return.
        upload_dormer_holes(&current, &mut library, &device);
        upload_dormer_holes(&current, &mut library, &device);
        upload_dormer_holes(&blueprints(vec![second]), &mut library, &device);
        upload_dormer_holes(&DecoratorBlueprints::default(), &mut library, &device);
        let state = device.0.lock().unwrap();
        assert_eq!(state.writes.len(), 8);
        assert_eq!(state.writes[0].bytes, state.writes[2].bytes);
        assert_eq!(state.writes[1].bytes, state.writes[3].bytes);
        assert_eq!(
            (
                uploaded_word(&state.writes[4].bytes, 0),
                uploaded_word(&state.writes[4].bytes, 224)
            ),
            (0, 1)
        );
        assert_eq!(
            uploaded_row(&state.writes[5].bytes, 0),
            DormerHoleSsbo::dummy()
        );
        assert_eq!(uploaded_row(&state.writes[5].bytes, 224 * 50), second);
        assert!(state.writes[6].bytes.iter().all(|&byte| byte == 0));
        assert!(state.writes[7].bytes.iter().all(|&byte| byte == 0));
    }

    #[test]
    fn missing_registered_types_follow_the_independent_native_optional_paths() {
        for (counts, holes) in [(false, false), (true, false), (false, true)] {
            let device = Device::default();
            let mut library = AssetSsboLibrary::new();
            if counts {
                library.add_new::<DormerHoleCountPerCellSsbo>(&device);
            }
            if holes {
                library.add_new::<DormerHoleSsbo>(&device);
            }
            upload_dormer_holes(&DecoratorBlueprints::default(), &mut library, &device);
            let state = device.0.lock().unwrap();
            assert_eq!(state.writes.len(), usize::from(counts) + usize::from(holes));
            if counts {
                assert_eq!(state.writes[0].bytes.len(), 900);
            }
            if holes {
                assert_eq!(state.writes[0].bytes.len(), 540000);
            }
        }
    }

    #[test]
    fn undersized_count_buffer_panics_before_device_writes_or_hole_publication() {
        let device = Device::default();
        let mut library = library(&device);
        // Explicitly invalid capacity fixture exercises the same native bounds
        // failure before calling the device, not a successful App state.
        library
            .get_mut::<DormerHoleCountPerCellSsbo>()
            .unwrap()
            .buffer
            .buffer
            .byte_size = 899;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            upload_dormer_holes(&DecoratorBlueprints::default(), &mut library, &device)
        }));
        assert!(result.is_err());
        assert_eq!(device.0.lock().unwrap().attempts, 0);
        assert_eq!(
            library
                .get::<DormerHoleCountPerCellSsbo>()
                .unwrap()
                .instance_count,
            0
        );
        assert_eq!(library.get::<DormerHoleSsbo>().unwrap().instance_count, 0);
    }

    #[derive(Clone, Copy)]
    #[repr(transparent)]
    struct SameNameCount(u32);

    impl StorageElement for SameNameCount {
        const NAME: &'static str = DormerHoleCountPerCellSsbo::NAME;
        const BUFFER_LEN: usize = 225;
    }

    impl SsboType for SameNameCount {
        type Storage = InstanceBuffer<Self>;
        fn new_storage_buffer(
            device: &dyn GraphicsDevice,
            initial: Option<&[u8]>,
        ) -> Self::Storage {
            // Keep the canonical dormer storage policy; this fixture changes
            // only nominal type identity to exercise the library collision.
            InstanceBuffer {
                instance_count: initial.map_or(0, |bytes| bytes.len() / core::mem::size_of::<Self>()),
                buffer: country_core::render::ssbo::TypedStorageBuffer::new_storage_buffer(device, initial),
            }
        }
    }

    #[test]
    fn full_type_collision_is_rejected_before_upload_instead_of_treated_as_missing() {
        let device = Device::default();
        let mut library = AssetSsboLibrary::new();
        // Wrong nominal type with the real short NAME deliberately collides;
        // the actual library must enforce its full-name assertion.
        library.add_new::<SameNameCount>(&device);
        library.add_new::<DormerHoleSsbo>(&device);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            upload_dormer_holes(&DecoratorBlueprints::default(), &mut library, &device)
        }));
        assert!(result.is_err());
        assert_eq!(device.0.lock().unwrap().attempts, 0);
        assert_eq!(library.get::<SameNameCount>().unwrap().instance_count, 0);
        assert_eq!(library.get::<DormerHoleSsbo>().unwrap().instance_count, 0);
    }
}
