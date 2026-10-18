use super::*;
use crate::asset_libraries::runtime::add_asset_library_systems;
use crate::systems::wall::construction::bricks::WallBrickInstance;
use crate::render::sparse_instance_buffer::SparseSsbo;
use crate::render::ssbo::rhapsody_boundary::BackendError;
use crate::render::ssbo::rhapsody_boundary::Buffer;
use crate::render::ssbo::rhapsody_boundary::BufferDesc;
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::render::test_device::TestDevice;
use crate::systems::render_loop::draw_passes::draw_callbacks_a::LibView;
use crate::systems::render_loop::draw_passes::draw_shaders::ssbo_tags::BrickTransformTag;
use crate::systems::render_loop::draw_passes::draw_shaders::ssbo_tags::storage_buffer;
use std::{
    ops::Range,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
};

#[derive(Default)]
struct RecordingDevice {
    fail_create: AtomicBool,
    fail_write: AtomicBool,
    creations: AtomicUsize,
    releases: Mutex<Vec<Buffer>>,
    writes: Mutex<Vec<Vec<Range<usize>>>>,
}
impl GraphicsDevice for RecordingDevice {
    fn create_buffer(
        &self,
        description: &BufferDesc,
        _: &str,
        initial: Option<&[u8]>,
    ) -> Result<Buffer, BackendError> {
        if self.fail_create.load(Ordering::Relaxed) {
            return Err(BackendError::DeviceLost);
        }
        assert!(initial.is_none(), "稀疏缓冲不得创建整块临时清零数据");
        let generation = self.creations.fetch_add(1, Ordering::Relaxed) as u64 + 1;
        Ok(Buffer {
            id: 17,
            generation,
            byte_size: description.size as u64,
            ..Buffer::default()
        })
    }
    fn release_buffer(&self, buffer: &Buffer) {
        self.releases.lock().unwrap().push(*buffer);
    }
    fn update_buffer_bytes(
        &self,
        _: &Buffer,
        data: &[u8],
        ranges: &[Range<usize>],
        _: usize,
    ) -> Result<(), BackendError> {
        self.writes.lock().unwrap().push(ranges.to_vec());
        assert!(ranges.iter().all(|range| range.end <= data.len()));
        if self.fail_write.load(Ordering::Relaxed) {
            return Err(BackendError::DeviceLost);
        }
        Ok(())
    }
}

fn test_world(device: Arc<RecordingDevice>) -> (World, Schedule) {
    let mut world = World::new();
    world.insert_resource(AssetLibraryRuntime::new(device, Arc::new(TestDevice(1))));
    world.init_resource::<AssetSsboLibrary>();
    world.init_resource::<WallBrickUploadStatus>();
    let mut schedule = Schedule::default();
    add_asset_library_systems(&mut schedule);
    (world, schedule)
}

#[test]
fn production_component_uploads_once_replaces_clears_and_reclaims_on_delete() {
    let device = Arc::new(RecordingDevice::default());
    let (mut world, mut schedule) = test_world(device.clone());
    // CPU 铺砖类型就是实际 GPU 元素，不需要中转 Vec 或逐字段复制。
    let entity = world
        .spawn(InstancedWall::<WallBrickInstance>::new(
            7,
            vec![
                WallBrickInstance {
                    seed: 41,
                    ..Default::default()
                };
                33
            ],
        ))
        .id();
    schedule.run(&mut world);
    let library = world.resource::<AssetSsboLibrary>();
    let storage = library.get::<BrickTransformSsbo>().unwrap();
    let allocation = world
        .get::<InstancedWall<BrickTransformSsbo>>(entity)
        .unwrap()
        .allocation()
        .unwrap();
    assert_eq!(
        storage.allocation_to_draw_multi_instanced(allocation),
        [(0, 33)]
    );
    assert_eq!(storage.data[0].seed, 41);
    assert_eq!(
        storage_buffer::<BrickTransformTag>(&LibView(library))
            .unwrap()
            .0,
        17
    );
    assert_eq!(
        device.writes.lock().unwrap().as_slice(),
        &[vec![0..64 * 96]]
    );
    schedule.run(&mut world);
    assert_eq!(
        device.writes.lock().unwrap().len(),
        1,
        "静止帧不重复上传或清空"
    );
    world
        .get_mut::<InstancedWall<BrickTransformSsbo>>(entity)
        .unwrap()
        .update(vec![BrickTransformSsbo {
            seed: 99,
            ..Default::default()
        }]);
    schedule.run(&mut world);
    assert_eq!(
        world
            .resource::<AssetSsboLibrary>()
            .get::<BrickTransformSsbo>()
            .unwrap()
            .data[0]
            .seed,
        99
    );
    world
        .get_mut::<InstancedWall<BrickTransformSsbo>>(entity)
        .unwrap()
        .update(vec![]);
    schedule.run(&mut world);
    assert!(
        world
            .resource::<AssetSsboLibrary>()
            .get::<BrickTransformSsbo>()
            .unwrap()
            .to_draw_multi_instanced()
            .is_empty()
    );
    world
        .get_mut::<InstancedWall<BrickTransformSsbo>>(entity)
        .unwrap()
        .update(vec![BrickTransformSsbo::default()]);
    schedule.run(&mut world);
    world.despawn(entity);
    schedule.run(&mut world);
    let library = world.resource::<AssetSsboLibrary>();
    let storage = library.get::<BrickTransformSsbo>().unwrap();
    assert!(storage.to_draw_multi_instanced().is_empty());
    assert_eq!(
        storage.free_chunks.read().len(),
        BrickTransformSsbo::CHUNK_COUNT
    );
    assert_eq!(device.creations.load(Ordering::Relaxed), 1);
    drop(world);
    assert_eq!(device.releases.lock().unwrap().len(), 1);
}

#[test]
fn device_failures_preserve_cpu_updates_and_dirty_ranges_for_retry() {
    let device = Arc::new(RecordingDevice::default());
    let (mut world, mut schedule) = test_world(device.clone());
    let entity = world
        .spawn(InstancedWall::new(7, vec![BrickTransformSsbo::default()]))
        .id();
    device.fail_create.store(true, Ordering::Relaxed);
    schedule.run(&mut world);
    assert!(
        world
            .get::<InstancedWall<BrickTransformSsbo>>(entity)
            .unwrap()
            .is_upload_pending()
    );
    assert!(!world.resource::<WallBrickUploadStatus>().gpu_ready);
    device.fail_create.store(false, Ordering::Relaxed);
    device.fail_write.store(true, Ordering::Relaxed);
    schedule.run(&mut world);
    assert!(!world.resource::<WallBrickUploadStatus>().gpu_ready);
    device.fail_write.store(false, Ordering::Relaxed);
    schedule.run(&mut world);
    assert!(world.resource::<WallBrickUploadStatus>().gpu_ready);
    assert_eq!(
        device.writes.lock().unwrap().as_slice(),
        &[vec![0..32 * 96], vec![0..32 * 96]]
    );
    schedule.run(&mut world);
    assert_eq!(device.writes.lock().unwrap().len(), 2);
    drop(world);
    assert_eq!(device.releases.lock().unwrap().len(), 1);
}

#[test]
fn library_teardown_releases_each_generation_while_runtime_survives() {
    let device = Arc::new(RecordingDevice::default());
    let (mut world, mut schedule) = test_world(device.clone());
    for generation in 1..=2 {
        world.init_resource::<AssetSsboLibrary>();
        let entity = world
            .spawn(InstancedWall::new(7, vec![BrickTransformSsbo::default()]))
            .id();
        schedule.run(&mut world);
        let buffer = world
            .resource::<AssetSsboLibrary>()
            .get::<BrickTransformSsbo>()
            .unwrap()
            .buffer
            .buffer;
        assert_eq!(buffer.generation, generation);
        assert_eq!(
            device.releases.lock().unwrap().len(),
            generation as usize - 1
        );
        drop(world.remove_resource::<AssetSsboLibrary>().unwrap());
        assert_eq!(device.releases.lock().unwrap().last(), Some(&buffer));
        assert_eq!(device.releases.lock().unwrap().len(), generation as usize);
        // 分配晚于库销毁，弱引用失效后不能重复归还设备缓冲。
        world.despawn(entity);
    }
    drop(world);
    assert_eq!(device.releases.lock().unwrap().len(), 2);
}

#[test]
fn storage_keeps_its_creating_device_alive_after_runtime_removal() {
    let device = Arc::new(RecordingDevice::default());
    let weak_device = Arc::downgrade(&device);
    let (mut world, mut schedule) = test_world(device.clone());
    world.spawn(InstancedWall::new(7, vec![BrickTransformSsbo::default()]));
    schedule.run(&mut world);
    world.remove_resource::<AssetLibraryRuntime>();
    drop(device);
    assert!(weak_device.upgrade().is_some());
    drop(world.remove_resource::<AssetSsboLibrary>().unwrap());
    assert!(weak_device.upgrade().is_none());
    drop(world);
}

#[test]
fn empty_world_and_new_empty_wall_do_not_allocate_the_shared_brick_buffer() {
    let device = Arc::new(RecordingDevice::default());
    let (mut world, mut schedule) = test_world(device.clone());
    schedule.run(&mut world);
    let entity = world
        .spawn(InstancedWall::<BrickTransformSsbo>::new(7, vec![]))
        .id();
    schedule.run(&mut world);
    assert_eq!(device.creations.load(Ordering::Relaxed), 0);
    drop(world.remove_resource::<AssetSsboLibrary>().unwrap());
    assert!(device.releases.lock().unwrap().is_empty());
    assert!(
        !world
            .get::<InstancedWall<BrickTransformSsbo>>(entity)
            .unwrap()
            .is_upload_pending()
    );
}
