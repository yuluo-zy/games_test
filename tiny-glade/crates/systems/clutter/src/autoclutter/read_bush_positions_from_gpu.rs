//! B mapped BushCpuSsbo -> per-cell raw-position lists -> normalized dirty hashes.
//! Original reader140eebaa0 is750B (PDB1081:260); old745B evidence ended inside
//! the final panic CALL. Original raw lexicographic u32 order and low32 hash XOR
//! empty hash are retained. The current shared geometry/raster owners are reused.
use bevy_ecs::prelude::*;
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
pub use country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo;
use country_core::utils::dirty_tiles::{DirtyTiles, world_rect_cells};
use glam::Vec2;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// 回读 SSBO 类型名（O，0x142f3b328，0x5c B）。
pub const BUSH_CPU_SSBO_NAME: &str =
    "country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo";
/// 回读 SSBO 哈希键（O，0x142f3b268：0x0deff5ab6a35972a）。
pub const BUSH_CPU_SSBO_ID: u64 = 0x0deff5ab_6a35972a;
/// `Aabb2::from_center_radius` 半径（O，0x1429cd030 = f32 0.7）。
pub const BUSH_CELL_RADIUS: f32 = 0.7;
/// 懒网格格数（64×64；位图 0x200 B / 0x1000 位）。
pub const BUSH_GRID_CELLS: usize = 64 * 64;

/// Compatibility import now names the canonical mapped element, not an
/// unfilled parallel Resource containing count/Vec data.
pub type BushPositions = BushCpuSsbo;

/// 懒网格单格：本帧 bush 位置表 + 上一帧哈希（O 记录 stride 0x20）。
#[derive(Clone, Debug, Default)]
pub struct BushGridCell {
    pub positions: Vec<Vec2>,
    pub hash: u64,
}

/// 懒网格（64×64；记录下标 = `x*64 + y`，O 0x140aac630）+ 脏格发布者。
///
/// 机器码证据：`read_bush_positions_from_gpu`（0x140eebaa0）最后对 `*BushGrid + 0x18` 调
/// `Publisher<DirtyTiles>::publish`；`autoclutter_subscribe_to_tabloids`（0x1422d6930）对其第 3 个
/// 参数 `ResMut<BushGrid>` 同样取 `*grid + 0x18` 调 `Publisher<DirtyTiles>::subscribe`。即发布者是
/// `BushGrid` 的内联字段（Vec 头 0x18 B 之后），不是单独的资源。
#[derive(Resource)]
pub struct BushGrid {
    pub cells: Vec<BushGridCell>,
    pub publisher: tabloid::Publisher<DirtyTiles>,
}

impl std::fmt::Debug for BushGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BushGrid").field("cells", &self.cells.len()).finish_non_exhaustive()
    }
}

impl Default for BushGrid {
    fn default() -> Self {
        Self {
            cells: vec![BushGridCell::default(); BUSH_GRID_CELLS],
            publisher: tabloid::Publisher::default(),
        }
    }
}

/// Original array<u32,2> ordering, preserving signed-zero and NaN payload bits.
fn sort_bush_positions(positions: &mut [Vec2]) {
    positions.sort_unstable_by_key(|point| [point.x.to_bits(), point.y.to_bits()]);
}

/// Native calculate_hash141cf7270 hashes a slice of [u32;2]. The caller stores
/// only low32(hash ^ empty_hash), making every empty cell's initial hash zero.
/// Keep the current u64 API, widening that observed u32 rather than extra bits.
pub fn hash_bush_positions(positions: &[Vec2]) -> u64 {
    let mut hash = DefaultHasher::new();
    positions.len().hash(&mut hash);
    for point in positions {
        [point.x.to_bits(), point.y.to_bits()].hash(&mut hash);
    }
    let mut empty = DefaultHasher::new();
    0usize.hash(&mut empty);
    u64::from((hash.finish() ^ empty.finish()) as u32)
}

/// read_bush_positions_from_gpu（0x140eebaa0）：GPU 回读灌木位置 → 懒网格分桶 →
/// 哈希去重 → 发布 DirtyTiles。
pub fn read_bush_positions_from_gpu(
    ssbos: Res<AssetSsboLibrary>,
    mut grid: ResMut<BushGrid>,
    readbacks: Option<Res<country_core::render::gpu_readback::GpuReadbacks>>,
    mut processed: Local<u64>,
) {
    // The canonical library performs the native full-type check. Missing
    // registration returns before touching BushGrid or its change tick.
    if ssbos.get::<BushCpuSsbo>().is_none() {
        return;
    }
    // Rows come from the render thread's latest complete publication, never
    // the mapping the in-flight GPU frame is rewriting. Before the first one
    // (zero-initialized buffer: count 0) or for an already processed copy the
    // grid would be unchanged, so neither touches BushGrid or its change tick.
    let Some(view) = readbacks.as_deref().and_then(|readbacks| readbacks.get::<BushCpuSsbo>()) else {
        return;
    };
    if view.revision == *processed {
        return;
    }
    *processed = view.revision;
    let rows = view.data;
    let len = rows.len();
    if len == 0 {
        panic!("read_bush_positions_from_gpu: panic_bounds_check (empty buffer)");
    }
    let count = rows[0].0[0] as usize;
    if len <= count {
        panic!("read_bush_positions_from_gpu: slice_end_index_len_fail (count={count}, mapped_len={len})");
    }

    // 1. 每格 positions.len 清零（O：`*(u64*)(data + i*0x20 + 0x10)=0`）。
    for cell in grid.cells.iter_mut() {
        cell.positions.clear();
    }

    // 3. 逐 bush：Aabb2::from_center_radius(center, 0.7) → 每覆盖格追加 bush 位置。
    for row in &rows[1..count + 1] {
        let pos = Vec2::new(f32::from_bits(row.0[0]), f32::from_bits(row.0[1]));
        let rect = [
            pos.x - BUSH_CELL_RADIUS,
            pos.y - BUSH_CELL_RADIUS,
            pos.x + BUSH_CELL_RADIUS,
            pos.y + BUSH_CELL_RADIUS,
        ];
        let cells = world_rect_cells(rect);
        if cells.is_empty() {
            continue;
        }
        for y in cells.minimum[1]..cells.end[1] {
            for x in cells.minimum[0]..cells.end[0] {
                let idx = x as usize * 64 + y as usize;
                grid.cells[idx].positions.push(pos);
            }
        }
    }

    // 4. 逐格排序 + 哈希比较 + 置脏位（位号 = 格下标，O）。
    let mut dirty = DirtyTiles::new_clear();
    let mut changed = false;
    for (idx, cell) in grid.cells.iter_mut().enumerate() {
        if cell.positions.len() > 1 {
            sort_bush_positions(&mut cell.positions);
        }
        let hash = hash_bush_positions(&cell.positions);
        if cell.hash != hash {
            cell.hash = hash;
            // Native stores the new hash before its dirty-bit bounds check.
            assert!(
                idx < BUSH_GRID_CELLS,
                "read_bush_positions_from_gpu: dirty tile index"
            );
            // 位号 = 记录下标：DirtyTiles 位图行 = idx/64、列 = idx%64。
            dirty.set_cell((idx % 64) as i32, (idx / 64) as i32);
            changed = true;
        }
    }
    // 5. changed → publish（O：memcpy 0x200 + Publisher::publish<DirtyTiles>）。
    if changed {
        let _ = grid.publisher.publish(dirty);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::render::{gpu_readback::GpuReadbacks, ssbo::{MappedStorageBuffer, rhapsody_boundary::Buffer}};

    /// Actual canonical typed storage/library, with a test-owned mapped page.
    /// This represents completed host readback; it does not emulate GPU work.
    fn fixture(rows: Option<Vec<[u32; 2]>>) -> (World, Schedule, tabloid::Subscriber<DirtyTiles>) {
        let mut world = World::new();
        let library = AssetSsboLibrary::new();
        let mut readbacks = GpuReadbacks::default();
        if let Some(rows) = rows {
            let size = rows.len() * 8;
            readbacks.publish_slice(&rows.iter().copied().map(BushCpuSsbo).collect::<Vec<_>>(), 1);
            let mapped = Box::leak(
                rows.into_iter()
                    .map(BushCpuSsbo)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            );
            library.register::<BushCpuSsbo>(MappedStorageBuffer::from_mapping(
                mapped,
                Buffer {
                    id: 71,
                    generation: 1,
                    _gap10: 0,
                    byte_size: size as u64,
                    _gap20: 0,
                },
            ));
        }
        let mut grid = BushGrid::default();
        let subscriber = grid.publisher.subscribe();
        world.insert_resource(library);
        world.insert_resource(readbacks);
        world.insert_resource(grid);
        let mut schedule = Schedule::default();
        schedule.add_systems(read_bush_positions_from_gpu);
        (world, schedule, subscriber)
    }
    fn bits(point: Vec2) -> [u32; 2] {
        [point.x.to_bits(), point.y.to_bits()]
    }
    /// A later completed GPU copy, published by the render thread.
    fn mapped(world: &mut World, change: impl FnOnce(&mut [BushCpuSsbo])) {
        let mut readbacks = world.resource_mut::<GpuReadbacks>();
        let view = readbacks.get::<BushCpuSsbo>().unwrap();
        let (mut rows, frame) = (view.data.to_vec(), view.frame);
        change(&mut rows);
        readbacks.publish_slice(&rows, frame + 1);
    }

    #[test]
    fn missing_canonical_buffer_keeps_existing_grid_and_publisher_untouched() {
        let (mut world, mut schedule, mut subscriber) = fixture(None);
        world.resource_mut::<BushGrid>().cells[0]
            .positions
            .push(Vec2::ONE);
        world.resource_mut::<BushGrid>().cells[0].hash = 17;
        schedule.run(&mut world);
        assert_eq!(world.resource::<BushGrid>().cells[0].positions, [Vec2::ONE]);
        assert_eq!(world.resource::<BushGrid>().cells[0].hash, 17);
        assert!(subscriber.take().is_none());
        assert_eq!(world.resource::<BushGrid>().publisher.sequence(), 0);
    }
    #[test]
    fn empty_mapped_header_is_valid_and_does_not_dirty_all_initial_cells() {
        let (mut world, mut schedule, mut subscriber) = fixture(Some(vec![[0, 0x7fc12345]]));
        schedule.run(&mut world);
        assert!(
            world
                .resource::<BushGrid>()
                .cells
                .iter()
                .all(|cell| cell.hash == 0 && cell.positions.is_empty())
        );
        assert!(subscriber.take().is_none());
    }
    #[test]
    fn actual_mapped_reader_preserves_raw_sort_reordering_and_dirty_clear_lifecycle() {
        let negative = Vec2::splat(-0.25);
        let positive = Vec2::splat(0.25);
        let rows = vec![
            [3, 0x7fc12345],
            bits(negative),
            bits(positive),
            bits(negative),
            [0xffc12345, 0x7fc45678],
        ];
        let (mut world, mut schedule, mut subscriber) = fixture(Some(rows));
        schedule.run(&mut world);
        let index = 31 * 64 + 31;
        let expected = [positive, negative, negative];
        let cell = &world.resource::<BushGrid>().cells[index];
        assert_eq!(cell.positions, expected);
        assert_eq!(cell.hash, hash_bush_positions(&expected));
        assert_eq!(cell.hash >> 32, 0);
        let dirty = subscriber.take().unwrap();
        assert_eq!(
            dirty.rows().iter().map(|row| row.count_ones()).sum::<u32>(),
            4
        );
        for x in [31, 32] {
            for y in [31, 32] {
                assert!(dirty.contains(x, y));
            }
        }
        drop(dirty);
        mapped(&mut world, |rows| rows.swap(1, 2));
        schedule.run(&mut world);
        assert!(subscriber.take().is_none());
        assert_eq!(world.resource::<BushGrid>().publisher.sequence(), 1);
        mapped(&mut world, |rows| rows[1].0 = bits(Vec2::splat(-0.5)));
        schedule.run(&mut world);
        assert!(subscriber.take().is_some());
        assert_eq!(world.resource::<BushGrid>().publisher.sequence(), 2);
        mapped(&mut world, |rows| rows[0].0[0] = 0);
        schedule.run(&mut world);
        assert!(subscriber.take().is_some());
        assert!(
            world
                .resource::<BushGrid>()
                .cells
                .iter()
                .all(|cell| cell.hash == 0 && cell.positions.is_empty())
        );
        schedule.run(&mut world);
        assert!(subscriber.take().is_none());
        assert_eq!(world.resource::<BushGrid>().publisher.sequence(), 3);
    }
    #[test]
    fn raw_array_order_and_normalized_hash_preserve_nan_and_negative_zero_bits() {
        let raw = [
            [0xffc00002, 1],
            [0x80000000, 0],
            [0x7fc00002, 0],
            [0x40000000, 1],
            [0, 0],
            [0xc0000000, 0],
            [0x7fc00001, 0],
            [0x40000000, 0],
        ];
        let mut positions: Vec<_> = raw
            .into_iter()
            .map(|p| Vec2::new(f32::from_bits(p[0]), f32::from_bits(p[1])))
            .collect();
        sort_bush_positions(&mut positions);
        let actual: Vec<_> = positions.iter().copied().map(bits).collect();
        assert_eq!(
            actual,
            [
                [0, 0],
                [0x40000000, 0],
                [0x40000000, 1],
                [0x7fc00001, 0],
                [0x7fc00002, 0],
                [0x80000000, 0],
                [0xc0000000, 0],
                [0xffc00002, 1]
            ]
        );
        let mut reference = DefaultHasher::new();
        actual.as_slice().hash(&mut reference);
        let mut empty = DefaultHasher::new();
        (&[] as &[[u32; 2]]).hash(&mut empty);
        assert_eq!(
            hash_bush_positions(&positions),
            u64::from((reference.finish() ^ empty.finish()) as u32)
        );
        assert_eq!(hash_bush_positions(&[]), 0);
    }
    #[test]
    #[should_panic(expected = "empty buffer")]
    fn empty_actual_mapped_span_panics() {
        let (mut world, mut schedule, _) = fixture(Some(vec![]));
        schedule.run(&mut world);
    }
    #[test]
    #[should_panic(expected = "slice_end_index_len_fail")]
    fn count_cannot_include_the_mapped_header() {
        let (mut world, mut schedule, _) = fixture(Some(vec![[1, 0]]));
        schedule.run(&mut world);
    }
    #[test]
    #[should_panic(expected = "dirty tile index")]
    fn changed_grid_cell_beyond_original_bit_capacity_panics() {
        let (mut world, mut schedule, _) = fixture(Some(vec![[0, 0]]));
        world.resource_mut::<BushGrid>().cells.push(BushGridCell {
            positions: Vec::new(),
            hash: 17,
        });
        schedule.run(&mut world);
    }
}
