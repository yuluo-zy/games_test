//! 资产发布与生命周期回归：使用生产库及真实 ECS 调度，设备只记录资源调用。
use bevy_ecs::{schedule::Schedule, world::World};
use country_core::{
    asset_libraries::{
        Handle,
        color_library::ColorLibrarySsbo,
        mesh_library::{AssetMeshLibrary, MeshAsset},
        meshwatch::MeshWatch,
        runtime::{AssetLibraryRuntime, add_asset_library_systems},
        ssbo_library::Ssbo,
        texture_library::{
            AssetTextureLibrary, DescriptorImageUpdate, ImageView, TextureLibraryDevice,
            TextureUpdate, format_bytes_per_texel,
        },
        transient_mesh::TransientMesh,
        vao_library::AssetVAOLibrary,
    },
    name_hash::NameHash,
    render::{
        load_texture::boundary::TextureDevice,
        mesh::{Mesh, SourceFile},
        shadowed_buffer::ShadowedBuffer,
        ssbo::{
            StorageElement,
            rhapsody_boundary::{Buffer, BufferDesc, GraphicsDevice},
        },
        vao::Vao,
    },
    resources::texture_presets::TexturePreset,
    rhapsody_contract::DescriptorSet,
    rhapsody_types::{
        BackendError, BufferGraphBackend, ImportedBuffer, RenderGraph, Texture, TextureDesc,
    },
};
use std::{
    ops::Range,
    sync::{
        Arc, Barrier, Mutex,
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
    },
};

#[derive(Default)]
struct RecordingDevice {
    next_id: AtomicU64,
    buffers_created: AtomicUsize,
    buffers_released: AtomicUsize,
    fail_descriptors: AtomicBool,
    fail_buffers: AtomicBool,
    descriptor_calls: AtomicUsize,
    descriptor_updates: Mutex<Vec<Vec<DescriptorImageUpdate>>>,
    pixel_lengths: Mutex<Vec<usize>>,
}
impl GraphicsDevice for RecordingDevice {
    fn create_buffer(
        &self,
        desc: &BufferDesc,
        _: &str,
        _: Option<&[u8]>,
    ) -> Result<Buffer, BackendError> {
        if self.fail_buffers.load(Ordering::Relaxed) {
            return Err(BackendError::OutOfDeviceMemory);
        }
        self.buffers_created.fetch_add(1, Ordering::Relaxed);
        Ok(Buffer {
            id: self.next_id.fetch_add(1, Ordering::Relaxed) + 1,
            byte_size: desc.size as u64,
            ..Default::default()
        })
    }
    fn release_buffer(&self, _: &Buffer) {
        self.buffers_released.fetch_add(1, Ordering::Relaxed);
    }
    fn update_buffer_bytes(
        &self,
        _: &Buffer,
        _: &[u8],
        _: &[Range<usize>],
        _: usize,
    ) -> Result<(), BackendError> {
        Ok(())
    }
}
impl TextureDevice for RecordingDevice {
    fn create_texture(
        &self,
        _: &str,
        desc: &TextureDesc,
        _: &[u8],
    ) -> Result<Texture, BackendError> {
        Ok(Texture {
            id: self.next_id.fetch_add(1, Ordering::Relaxed) + 1,
            width: desc.extent[0],
            height: desc.extent[1],
            format: desc.format,
        })
    }
}
impl TextureLibraryDevice for RecordingDevice {
    fn create_persistent_descriptor_set(&self, _: &str) -> Result<DescriptorSet, BackendError> {
        Ok(DescriptorSet::default())
    }
    fn update_persistent_descriptor_set(
        &self,
        _: &DescriptorSet,
        updates: &[DescriptorImageUpdate],
    ) -> Result<(), BackendError> {
        self.descriptor_calls.fetch_add(1, Ordering::Relaxed);
        if self.fail_descriptors.load(Ordering::Relaxed) {
            Err(BackendError::DeviceLost)
        } else {
            self.descriptor_updates
                .lock()
                .unwrap()
                .push(updates.to_vec());
            Ok(())
        }
    }
    fn get_vk_image_view(&self, texture: &Texture, _: u32) -> Result<ImageView, BackendError> {
        Ok(ImageView(texture.id + 100))
    }
    fn update_texture(&self, _: &Texture, update: &TextureUpdate) -> Result<(), BackendError> {
        self.pixel_lengths.lock().unwrap().push(update.data.len());
        Ok(())
    }
}
struct RecordingGraph;
impl BufferGraphBackend for RecordingGraph {
    fn import_unowned(&mut self, resource: u64, access: u8) -> ImportedBuffer {
        ImportedBuffer {
            resource,
            generation: 1,
            access: access as u32,
        }
    }
    fn export(&mut self, _: ImportedBuffer, _: u8) {}
}

#[test]
fn all_presets_keep_their_complete_description_and_d16_clear_size() {
    let device = RecordingDevice::default();
    let mut library = AssetTextureLibrary::new(&device);
    library.create_preset_textures(&device);
    let mut graph = RenderGraph::with_backend(RecordingGraph);
    library.import_texture_presets_to_rg(&mut graph);
    for preset in 0..TexturePreset::COUNT {
        let expected = TexturePreset(preset).texture().desc;
        assert_eq!(library.get_rg_texture(TexturePreset(preset)).desc, expected);
        assert_eq!(
            format_bytes_per_texel(expected.format),
            rhapsody::format::observed::try_vk_format_info(ash::vk::Format::from_raw(
                expected.format as i32
            ))
            .unwrap()
            .word_at_04()
        );
        assert!(expected.kind == 2 || expected.kind == 3);
    }
    library.clear_preset_texture(&device, TexturePreset(10));
    assert_eq!(*device.pixel_lengths.lock().unwrap(), [512 * 512 * 2]);
    library.export_preset_textures_from_rg(&mut graph);
}

#[test]
fn descriptor_failure_keeps_pending_work_and_success_commits_it_once() {
    let device = RecordingDevice::default();
    let mut library = AssetTextureLibrary::new(&device);
    let texture = device
        .create_texture("wall", &TextureDesc::new_2d([1, 1], 37), &[])
        .unwrap();
    let asset = library.add(Some("wall".into()), texture);
    library.handle_to_bindless_index(asset);
    device.fail_descriptors.store(true, Ordering::Relaxed);
    assert!(library.try_bindless_textures_dset(&device).is_err());
    assert_eq!(library.pending_bindless(), [asset]);
    device.fail_descriptors.store(false, Ordering::Relaxed);
    library.try_bindless_textures_dset(&device).unwrap();
    library.try_bindless_textures_dset(&device).unwrap();
    assert!(library.pending_bindless().is_empty());
    assert_eq!(device.descriptor_calls.load(Ordering::Relaxed), 2);
}

#[test]
fn concurrent_queries_allocate_one_descriptor_for_the_same_asset() {
    let device = RecordingDevice::default();
    let mut library = AssetTextureLibrary::new(&device);
    let asset = library.add(None, Texture::default());
    let library = Arc::new(library);
    let start = Arc::new(Barrier::new(12));
    let threads: Vec<_> = (0..12)
        .map(|_| {
            let library = library.clone();
            let start = start.clone();
            std::thread::spawn(move || {
                start.wait();
                library.handle_to_bindless_index(asset)
            })
        })
        .collect();
    for thread in threads {
        assert_eq!(thread.join().unwrap().0, 0);
    }
    assert_eq!(library.pending_bindless(), [asset]);
}

#[test]
fn deferred_texture_reloads_reuse_the_slot_and_publish_the_latest_image() {
    let device = RecordingDevice::default();
    let mut library = AssetTextureLibrary::new(&device);
    let desc = TextureDesc::new_2d([1, 1], 37);
    let first = device.create_texture("wall", &desc, &[]).unwrap();
    let asset = library.add(Some("wall".into()), first);
    let index = library.handle_to_bindless_index(asset);
    // 尚未提交时多次替换，应只发布最后一张图片。
    for _ in 0..2 {
        let texture = device.create_texture("wall", &desc, &[]).unwrap();
        library.replace(NameHash::compute("wall"), texture).unwrap();
    }
    assert_eq!(library.pending_bindless(), [asset]);
    library.try_bindless_textures_dset(&device).unwrap();
    // 超过表容量的连续重载不能消耗新槽位。
    for _ in 0..country_core::asset_libraries::texture_library::REQUIRED_BINDLESS_DESCRIPTOR_COUNT {
        let texture = device.create_texture("wall", &desc, &[]).unwrap();
        let latest_id = texture.id;
        library.replace(NameHash::compute("wall"), texture).unwrap();
        assert_eq!(library.handle_to_bindless_index(asset), index);
        assert_eq!(library.pending_bindless(), [asset]);
        library.try_bindless_textures_dset(&device).unwrap();
        let updates = device.descriptor_updates.lock().unwrap();
        let last = updates.last().unwrap();
        assert_eq!(last.len(), 1);
        assert_eq!((last[0].index, last[0].texture_id), (index, latest_id));
    }
}

fn world_with_device(device: Arc<RecordingDevice>) -> (World, Schedule) {
    let mut world = World::new();
    world.insert_resource(AssetLibraryRuntime::new(device.clone(), device));
    world.init_resource::<AssetMeshLibrary>();
    world.init_resource::<AssetVAOLibrary>();
    let mut schedule = Schedule::default();
    add_asset_library_systems(&mut schedule);
    (world, schedule)
}
fn triangle() -> Mesh {
    let mut mesh = Mesh::new("triangle");
    mesh.indices = vec![0, 1, 2];
    mesh
}

#[test]
fn removing_a_plain_mesh_reclaims_an_orphan_vao_after_the_host_returns() {
    let device = Arc::new(RecordingDevice::default());
    let (mut world, mut schedule) = world_with_device(device.clone());
    let mut meshes = AssetMeshLibrary::new();
    let mesh = meshes.add(MeshAsset::new(triangle(), None));
    let mut vaos = AssetVAOLibrary::new();
    vaos.add(device.as_ref(), &meshes, mesh).unwrap();
    // 没有实体或 TransientMesh，仅普通 CPU 资产删除。
    meshes.remove(mesh);
    meshes.remove(mesh);
    world.insert_resource(meshes);
    world.insert_resource(vaos);
    let runtime = world.remove_resource::<AssetLibraryRuntime>().unwrap();
    schedule.run(&mut world);
    assert_eq!(device.buffers_released.load(Ordering::Relaxed), 0);
    world.insert_resource(runtime);
    schedule.run(&mut world);
    assert!(world.resource::<AssetVAOLibrary>().is_empty());
    assert_eq!(device.buffers_released.load(Ordering::Relaxed), 1);
    schedule.run(&mut world);
    assert_eq!(device.buffers_released.load(Ordering::Relaxed), 1);
}

#[test]
fn failed_vao_rebuild_keeps_the_old_buffers_and_retries_next_frame() {
    let device = Arc::new(RecordingDevice::default());
    let (mut world, mut schedule) = world_with_device(device.clone());
    let mesh = world
        .resource::<AssetMeshLibrary>()
        .add(MeshAsset::new(triangle(), None));
    let entity = world.spawn(mesh).id();
    schedule.run(&mut world);
    let published_vao = *world.entity(entity).get::<Handle<Vao>>().unwrap();
    world
        .resource_mut::<AssetMeshLibrary>()
        .get_mut(mesh)
        .unwrap()
        .indices
        .extend([0, 1, 2]);
    device.fail_buffers.store(true, Ordering::Relaxed);
    schedule.run(&mut world);
    assert_eq!(
        world
            .resource::<AssetVAOLibrary>()
            .get(mesh)
            .unwrap()
            .index_count,
        3
    );
    assert_eq!(world.resource::<AssetMeshLibrary>().modified_meshes, [mesh]);
    assert_eq!(device.buffers_released.load(Ordering::Relaxed), 0);
    device.fail_buffers.store(false, Ordering::Relaxed);
    schedule.run(&mut world);
    assert_eq!(
        *world.entity(entity).get::<Handle<Vao>>().unwrap(),
        published_vao
    );
    assert_eq!(
        world
            .resource::<AssetVAOLibrary>()
            .get(mesh)
            .unwrap()
            .index_count,
        6
    );
    assert!(
        world
            .resource::<AssetMeshLibrary>()
            .modified_meshes
            .is_empty()
    );
    assert_eq!(device.buffers_released.load(Ordering::Relaxed), 1);
}

#[test]
fn ecs_builds_reuses_rebuilds_and_reclaims_mesh_buffers() {
    let device = Arc::new(RecordingDevice::default());
    let (mut world, mut schedule) = world_with_device(device.clone());
    let meshes = world.resource::<AssetMeshLibrary>();
    let handle = meshes.add(MeshAsset::new(triangle(), Some("triangle".into())));
    let transient = TransientMesh::new_with_registry(handle, meshes.transient_meshes());
    let entity = world.spawn((handle, transient)).id();
    schedule.run(&mut world);
    let initial_vao = *world.entity(entity).get::<Handle<Vao>>().unwrap();
    assert_eq!(device.buffers_created.load(Ordering::Relaxed), 1);
    schedule.run(&mut world);
    assert_eq!(device.buffers_created.load(Ordering::Relaxed), 1);
    world
        .resource_mut::<AssetMeshLibrary>()
        .get_mut(handle)
        .unwrap()
        .indices
        .extend([0, 1, 2]);
    schedule.run(&mut world);
    assert_eq!(
        *world.entity(entity).get::<Handle<Vao>>().unwrap(),
        initial_vao
    );
    assert_eq!(
        world
            .resource::<AssetVAOLibrary>()
            .get(handle)
            .unwrap()
            .index_count,
        6
    );
    assert_eq!(device.buffers_released.load(Ordering::Relaxed), 1);
    world.despawn(entity);
    schedule.run(&mut world);
    assert!(world.resource::<AssetMeshLibrary>().get(handle).is_none());
    assert!(!world.resource::<AssetVAOLibrary>().has_vao(handle));
    assert_eq!(
        device.buffers_created.load(Ordering::Relaxed),
        device.buffers_released.load(Ordering::Relaxed)
    );
}

#[test]
fn incomplete_mesh_file_preserves_old_asset_until_a_later_valid_write() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("mesh.json");
    std::fs::write(&path, "{").unwrap();
    let (mut world, mut schedule) = world_with_device(Arc::new(RecordingDevice::default()));
    let mut mesh = triangle();
    mesh.source_file = Some(SourceFile {
        path: path.to_str().unwrap().into(),
        stamp: 0,
    });
    let handle = world
        .resource::<AssetMeshLibrary>()
        .add(MeshAsset::new(mesh, None));
    let watch = MeshWatch::new(false);
    MeshWatch::on_debounced_event(watch.changed_paths(), &path, true);
    world.insert_resource(watch);
    schedule.run(&mut world);
    assert_eq!(
        world
            .resource::<AssetMeshLibrary>()
            .get(handle)
            .unwrap()
            .indices,
        [0, 1, 2]
    );
    assert!(
        world
            .resource::<AssetMeshLibrary>()
            .modified_meshes
            .is_empty()
    );
    std::fs::write(
        &path,
        r#"{"indices":{"buffer":[0,1,2,0,1,2]},"attributes":["Vertex_Position"],"Vertex_Position":{"type":["float",3],"buffer":[[0,0,0],[1,0,0],[0,1,0]]}}"#,
    )
    .unwrap();
    MeshWatch::on_debounced_event(world.resource::<MeshWatch>().changed_paths(), &path, true);
    schedule.run(&mut world);
    assert_eq!(
        world
            .resource::<AssetMeshLibrary>()
            .get(handle)
            .unwrap()
            .indices
            .len(),
        6
    );
}

#[repr(transparent)]
#[derive(Clone, Copy)]
struct Word(u32);
// SAFETY：透明包装只有一个 u32，全部字节均已初始化，没有填充。
unsafe impl bytemuck::NoUninit for Word {}
impl StorageElement for Word {
    const NAME: &'static str = "ReviewWord";
}
#[test]
fn cpu_shadow_uses_a_checked_cast_and_preserves_buffer_identity() {
    let device = RecordingDevice::default();
    let mut shadow = ShadowedBuffer::new(&device, vec![Word(7), Word(11)]);
    assert_eq!(shadow.cpu_shadow_words(), Some([7, 11].as_slice()));
    let snapshot = shadow.snapshot();
    assert!(snapshot.cpu_shadow_words().is_none(), "render snapshot must not own a CPU shadow");
    assert!(snapshot.as_any().downcast_ref::<ShadowedBuffer<Word>>().is_none());
    shadow.write(0, &[Word(99)]);
    assert_eq!(shadow.cpu_shadow_words(), Some([99, 11].as_slice()));
    assert_eq!(snapshot.handle(), shadow.handle());
    let handle = shadow.handle();
    assert_eq!(handle.0, shadow.buffer.buffer.id);
    assert_eq!(handle.1, shadow.buffer.buffer.generation);
    let _ = (NameHash::compute("wall"), ColorLibrarySsbo::default());
}
