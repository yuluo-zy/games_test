from pathlib import Path
import json,hashlib
R=Path('/Users/yuluo/Desktop/game');W=Path(__file__).resolve().parent;rel='games_test/tiny-glade/crates/systems/clutter/src/autoclutter/read_bush_positions_from_gpu.rs';old=(R/rel).read_text()
header='''//! B mapped BushCpuSsbo -> per-cell raw-position lists -> normalized dirty hashes.
//! Original reader140eebaa0 is750B (PDB1081:260); old745B evidence ended inside
//! the final panic CALL. Original raw lexicographic u32 order and low32 hash XOR
//! empty hash are retained. The current shared geometry/raster owners are reused.
'''
s=header+old[old.index('use bevy_ecs::prelude::*;'):]
s=s.replace('use bevy_ecs::prelude::*;','''use bevy_ecs::prelude::*;
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
pub use country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo;''',1)
a=s.index('/// 灌木 CPU 回读缓冲');b=s.index('/// 懒网格单格',a)
s=s[:a]+'''/// Compatibility import now names the canonical mapped element, not an
/// unfilled parallel Resource containing count/Vec data.
pub type BushPositions = BushCpuSsbo;

'''+s[b:]
a=s.index('/// Vec2 序列哈希');b=s.index('/// read_bush_positions_from_gpu',a)
s=s[:a]+'''/// Original array<u32,2> ordering, preserving signed-zero and NaN payload bits.
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

'''+s[b:]
s=s.replace('''    bush: Res<BushCpuSsbo>,''','''    ssbos: Res<AssetSsboLibrary>,''',1)
s=s.replace('''    // 2. 切片边界（O：puVar3[1]==0 → panic_bounds_check；len<=count → slice_end_index_len_fail）。
    let len = bush.positions.len();''','''    // The canonical library performs the native full-type check. Missing
    // registration returns before touching BushGrid or its change tick.
    let Some(bush) = ssbos.get::<BushCpuSsbo>() else {
        return;
    };
    let len = bush.mapped.len();''',1)
s=s.replace('''    let count = bush.count as usize;''','''    let count = bush.mapped[0].0[0] as usize;''',1)
s=s.replace('''    for pos in bush.positions.iter().take(count) {
        let rect''','''    for row in &bush.mapped[1..count + 1] {
        let pos = Vec2::new(f32::from_bits(row.0[0]), f32::from_bits(row.0[1]));
        let rect''',1).replace('''grid.cells[idx].positions.push(*pos);''','''grid.cells[idx].positions.push(pos);''',1)
a=s.index('''            // O：`sort_unstable`''');b=s.index('\n        }',a)
s=s[:a]+'''            sort_bush_positions(&mut cell.positions);'''+s[b:]
s+='''
#[cfg(test)]
mod tests {
    use super::*;
    use country_core::render::ssbo::{MappedStorageBuffer, rhapsody_boundary::Buffer};

    /// Actual canonical typed storage/library, with a test-owned mapped page.
    /// This represents completed host readback; it does not emulate GPU work.
    fn fixture(rows: Option<Vec<[u32; 2]>>) -> (World, Schedule, tabloid::Subscriber<DirtyTiles>) {
        let mut world = World::new();
        let library = AssetSsboLibrary::new();
        if let Some(rows) = rows {
            let size = rows.len() * 8;
            let mapped = Box::leak(rows.into_iter().map(BushCpuSsbo).collect::<Vec<_>>().into_boxed_slice());
            library.register::<BushCpuSsbo>(MappedStorageBuffer {
                mapped,
                buffer: Buffer { id: 71, generation: 1, _gap10: 0, byte_size: size as u64, _gap20: 0 },
            });
        }
        let mut publisher = DirtyTilesPublisher::default();
        let subscriber = publisher.0.subscribe();
        world.insert_resource(library);
        world.init_resource::<BushGrid>();
        world.insert_resource(publisher);
        let mut schedule = Schedule::default();
        schedule.add_systems(read_bush_positions_from_gpu);
        (world, schedule, subscriber)
    }
    fn bits(point: Vec2) -> [u32; 2] { [point.x.to_bits(), point.y.to_bits()] }
    fn mapped(world: &mut World, change: impl FnOnce(&mut [BushCpuSsbo])) {
        change(&mut world.resource_mut::<AssetSsboLibrary>().get_mut::<BushCpuSsbo>().unwrap().mapped);
    }

    #[test]
    fn missing_canonical_buffer_keeps_existing_grid_and_publisher_untouched() {
        let (mut world, mut schedule, mut subscriber) = fixture(None);
        world.resource_mut::<BushGrid>().cells[0].positions.push(Vec2::ONE);
        world.resource_mut::<BushGrid>().cells[0].hash = 17;
        schedule.run(&mut world);
        assert_eq!(world.resource::<BushGrid>().cells[0].positions, [Vec2::ONE]);
        assert_eq!(world.resource::<BushGrid>().cells[0].hash, 17);
        assert!(subscriber.take().is_none());
        assert_eq!(world.resource::<DirtyTilesPublisher>().0.sequence(), 0);
    }
    #[test]
    fn empty_mapped_header_is_valid_and_does_not_dirty_all_initial_cells() {
        let (mut world, mut schedule, mut subscriber) = fixture(Some(vec![[0, 0x7fc12345]]));
        schedule.run(&mut world);
        assert!(world.resource::<BushGrid>().cells.iter().all(|cell| cell.hash == 0 && cell.positions.is_empty()));
        assert!(subscriber.take().is_none());
    }
    #[test]
    fn actual_mapped_reader_preserves_raw_sort_reordering_and_dirty_clear_lifecycle() {
        let negative = Vec2::splat(-0.25);
        let positive = Vec2::splat(0.25);
        let rows = vec![[3, 0x7fc12345], bits(negative), bits(positive), bits(negative), [0xffc12345, 0x7fc45678]];
        let (mut world, mut schedule, mut subscriber) = fixture(Some(rows));
        schedule.run(&mut world);
        let index = 31 * 64 + 31;
        let expected = [positive, negative, negative];
        let cell = &world.resource::<BushGrid>().cells[index];
        assert_eq!(cell.positions, expected);
        assert_eq!(cell.hash, hash_bush_positions(&expected));
        assert_eq!(cell.hash >> 32, 0);
        let dirty = subscriber.take().unwrap();
        assert_eq!(dirty.rows().iter().map(|row| row.count_ones()).sum::<u32>(), 4);
        for x in [31, 32] { for y in [31, 32] { assert!(dirty.contains(x, y)); } }
        drop(dirty);
        mapped(&mut world, |rows| rows.swap(1, 2));
        schedule.run(&mut world);
        assert!(subscriber.take().is_none());
        assert_eq!(world.resource::<DirtyTilesPublisher>().0.sequence(), 1);
        mapped(&mut world, |rows| rows[1].0 = bits(Vec2::splat(-0.5)));
        schedule.run(&mut world);
        assert!(subscriber.take().is_some());
        assert_eq!(world.resource::<DirtyTilesPublisher>().0.sequence(), 2);
        mapped(&mut world, |rows| rows[0].0[0] = 0);
        schedule.run(&mut world);
        assert!(subscriber.take().is_some());
        assert!(world.resource::<BushGrid>().cells.iter().all(|cell| cell.hash == 0 && cell.positions.is_empty()));
        schedule.run(&mut world);
        assert!(subscriber.take().is_none());
        assert_eq!(world.resource::<DirtyTilesPublisher>().0.sequence(), 3);
    }
    #[test]
    fn raw_array_order_and_normalized_hash_preserve_nan_and_negative_zero_bits() {
        let raw = [[0xffc00002, 1], [0x80000000, 0], [0x7fc00002, 0], [0x40000000, 1], [0, 0], [0xc0000000, 0], [0x7fc00001, 0], [0x40000000, 0]];
        let mut positions: Vec<_> = raw.into_iter().map(|p| Vec2::new(f32::from_bits(p[0]), f32::from_bits(p[1]))).collect();
        sort_bush_positions(&mut positions);
        let actual: Vec<_> = positions.iter().copied().map(bits).collect();
        assert_eq!(actual, [[0, 0], [0x40000000, 0], [0x40000000, 1], [0x7fc00001, 0], [0x7fc00002, 0], [0x80000000, 0], [0xc0000000, 0], [0xffc00002, 1]]);
        let mut reference = DefaultHasher::new();
        actual.as_slice().hash(&mut reference);
        let mut empty = DefaultHasher::new();
        (&[] as &[[u32; 2]]).hash(&mut empty);
        assert_eq!(hash_bush_positions(&positions), u64::from((reference.finish() ^ empty.finish()) as u32));
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
}
'''
rows=[]
for rel,base,candidate in [(rel,old,s)]:
 for folder,value in [('base',base),('candidate',candidate)]:
  p=W/folder/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(value)
 rows.append({'path':rel,'base_sha256':hashlib.sha256(base.encode()).hexdigest(),'candidate':str(W/'candidate'/rel),'role':'owned upper reader'})
rel='games_test/tiny-glade/crates/systems/clutter/src/lib.rs';old=(R/rel).read_text();s=old.replace('''    world.init_resource::<autoclutter::read_bush_positions_from_gpu::BushCpuSsbo>();''','''    // The SSBO library is the actual readback owner; init preserves host data.
    world.init_resource::<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>();''',1)
s=s.replace('''            autoclutter_subscribe_to_tabloids,
''','''            autoclutter_subscribe_to_tabloids,
            // Original build65226c/ee2723 registers this canonical reader.
            // Current integration reads before populate; no numeric native set
            // is reinterpreted as a different current architecture stage.
            autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu,
''',1)
s=s.replace('''            // Original build65226c/ee2723 registers this canonical reader.
            // Current integration reads before populate; no numeric native set
            // is reinterpreted as a different current architecture stage.
            autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu,
''','',1)
s=s.replace('''    schedule.add_systems(
''','''    // Original build65226c/ee2723 registers this canonical reader. Keep the
    // current chain within Bevy's tuple limit; only add its data dependency.
    schedule.add_systems(
        autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu
            .after(autoclutter_subscribe_to_tabloids)
            .before(populate_autoclutter_regions),
    );
    schedule.add_systems(
''',1)
for folder,value in [('base',old),('candidate',s)]:
 p=W/folder/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(value)
rows.append({'path':rel,'base_sha256':hashlib.sha256(old.encode()).hexdigest(),'candidate':str(W/'candidate'/rel),'role':'necessary shared registration adapter'})
(W/'source-map.json').write_text(json.dumps(rows,indent=2)+'\n')
