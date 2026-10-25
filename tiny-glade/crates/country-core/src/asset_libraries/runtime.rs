//! 每帧维护资产：回收已删除对象、重载变更、构建并刷新 VAO。
use super::{
    Handle,
    color_library::{ColorLibrarySsbo, GladeTheme, try_update_color_library_in},
    configwatch::ConfigWatch,
    mesh_library::AssetMeshLibrary,
    meshwatch::MeshWatch,
    ssbo_library::AssetSsboLibrary,
    texture_library::{AssetTextureLibrary, TextureLibraryDevice},
    texture_watch::TextureWatch,
    transient_mesh::delete_dropped_transient_meshes,
    vao_library::AssetVAOLibrary,
};
use crate::{
    name_hash::NameHash,
    render::{mesh::Mesh, ssbo::rhapsody_boundary::GraphicsDevice, vao::Vao},
};
use bevy_ecs::prelude::*;
use std::{path::PathBuf, sync::Arc};

/// 宿主注入真实资源表。缺少设备时保留队列，不制造更新成功。
#[derive(Clone, Resource)]
pub struct AssetLibraryRuntime {
    pub(crate) buffers: Arc<dyn GraphicsDevice + Send + Sync>,
    pub(crate) textures: Arc<dyn TextureLibraryDevice + Send + Sync>,
    pub assets_root: PathBuf,
    pub compiled_texture_root: PathBuf,
    pub theme: GladeTheme,
}
impl AssetLibraryRuntime {
    /// Borrow the same physical buffer device used by library maintenance.
    /// External game systems may flush their typed sparse owners without
    /// constructing a second device or taking ownership of its tables.
    pub fn buffer_device(&self) -> &(dyn GraphicsDevice + Send + Sync) {
        self.buffers.as_ref()
    }
    pub fn new(
        buffers: Arc<dyn GraphicsDevice + Send + Sync>,
        textures: Arc<dyn TextureLibraryDevice + Send + Sync>,
    ) -> Self {
        Self {
            buffers,
            textures,
            assets_root: "assets".into(),
            compiled_texture_root: crate::render::load_texture::COMPILED_TEXTURE_DIR.into(),
            theme: GladeTheme::Summer,
        }
    }
}

/// 单调度兼容入口的维护顺序。生产 VAO 前段属于 Update44；其余维护仍在 PostUpdate。
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AssetMaintenance {
    Reclaim,
    Reload,
    Rebuild,
    Geometry,
}

/// Existing single-schedule fixture/adapter entry. Production uses the split
/// PostUpdate entry plus the required VAO systems in Update44.
pub fn add_asset_library_systems(schedule: &mut Schedule) {
    use bevy_ecs::schedule::common_conditions::resource_exists;
    add_asset_library_post_update_systems(schedule);
    schedule.configure_sets((
        AssetMaintenance::Reclaim, AssetMaintenance::Reload,
        AssetMaintenance::Rebuild, AssetMaintenance::Geometry,
    ).chain());
    // Preserve this old adapter's optional resource-availability behavior.
    // Production registers the required systems directly without these guards.
    schedule.add_systems((build_missing_vaos, rebuild_vaos)
        .chain_ignore_deferred()
        .in_set(AssetMaintenance::Rebuild)
        .run_if(resource_exists::<AssetLibraryRuntime>)
        .run_if(resource_exists::<AssetMeshLibrary>)
        .run_if(resource_exists::<AssetVAOLibrary>));
}

/// Keep the unrelated reclamation, hot reload and geometry owners in their
/// existing phase. PostUpdate reloads enter the next frame's Update44 queue.
pub fn add_asset_library_post_update_systems(schedule: &mut Schedule) {
    schedule.configure_sets((
        AssetMaintenance::Reclaim, AssetMaintenance::Reload, AssetMaintenance::Geometry,
    ).chain());
    schedule.add_systems((
        reclaim_transient_meshes.in_set(AssetMaintenance::Reclaim),
        (reload_meshes, reload_textures, reload_colors).in_set(AssetMaintenance::Reload),
        crate::geometry::runtime::sync_wall_bricks.in_set(AssetMaintenance::Geometry),
    ));
}

fn reclaim_transient_meshes(
    runtime: Option<Res<AssetLibraryRuntime>>,
    meshes: Option<ResMut<AssetMeshLibrary>>,
    vaos: Option<ResMut<AssetVAOLibrary>>,
) {
    let (Some(runtime), Some(mut meshes), Some(mut vaos)) = (runtime, meshes, vaos) else {
        return;
    };
    delete_dropped_transient_meshes(runtime.buffers.as_ref(), &mut meshes, &mut vaos);
    // 普通资产也能由编辑器或业务代码直接删除，不依赖实体上存在 TransientMesh。
    for mesh in meshes.drain_removed_meshes() {
        vaos.remove(runtime.buffers.as_ref(), mesh);
    }
}
fn reload_meshes(watch: Option<Res<MeshWatch>>, meshes: Option<ResMut<AssetMeshLibrary>>) {
    let (Some(watch), Some(mut meshes)) = (watch, meshes) else {
        return;
    };
    let paths: Vec<_> = watch.changed_paths().lock().unwrap().drain().collect();
    for path in paths {
        if let Err(error) = meshes.try_reload_mesh_src(&path) {
            log::error!("网格热重载失败，保留旧资产 {}：{error:#}", path.display());
        }
    }
}
fn reload_colors(
    runtime: Option<Res<AssetLibraryRuntime>>,
    watch: Option<Res<ConfigWatch>>,
    ssbos: Option<Res<AssetSsboLibrary>>,
    palette: Option<ResMut<ColorLibrarySsbo>>,
) {
    let (Some(runtime), Some(watch), Some(ssbos), Some(mut palette)) =
        (runtime, watch, ssbos, palette)
    else {
        return;
    };
    if watch.take_pending().is_none() {
        return;
    }
    if let Err(error) = try_update_color_library_in(
        &runtime.assets_root,
        runtime.theme,
        &ssbos,
        runtime.buffers.as_ref(),
        &mut palette,
    ) {
        log::error!("颜色热重载失败，保留旧调色板：{error:#}");
    }
}
fn reload_textures(
    runtime: Option<Res<AssetLibraryRuntime>>,
    watch: Option<Res<TextureWatch>>,
    textures: Option<ResMut<AssetTextureLibrary>>,
) {
    let (Some(runtime), Some(watch), Some(mut textures)) = (runtime, watch, textures) else {
        return;
    };
    let names: Vec<_> = watch.changed().lock().unwrap().drain().collect();
    for name in names {
        let result = (|| -> anyhow::Result<()> {
            let path = runtime
                .compiled_texture_root
                .join(name.as_str())
                .with_extension(crate::render::load_texture::COMPILED_TEXTURE_EXT);
            let file = crate::render::load_texture::AlignedFile::read(&path)?;
            let image = crate::render::load_texture::try_parse_compiled_image(file.bytes(), &path)?;
            let texture =
                runtime
                    .textures
                    .create_texture(name.as_str(), &image.desc, image.data)?;
            match textures.replace(NameHash::compute(name.as_str()), texture.clone()) {
                Ok(previous) => runtime.textures.release_texture(&previous),
                Err(error) => {
                    runtime.textures.release_texture(&texture);
                    return Err(error.into());
                }
            }
            Ok(())
        })();
        if let Err(error) = result {
            log::error!("纹理热重载失败，保留旧资产 {name}：{error:#}");
        }
    }
}
/// Original B build_missing_vaos@1408646b0 (1331 bytes). The original Query
/// excludes entities already carrying a VAO, including stale components.
type MissingMeshVaoComponents = (Entity, &'static Handle<Mesh>);
const VAO_LOG_TARGET: &str = "country_core::systems::backend::vao_update";

pub fn build_missing_vaos(
    mesh_entities: Query<MissingMeshVaoComponents, Without<Handle<Vao>>>,
    meshes: Res<AssetMeshLibrary>,
    runtime: Res<AssetLibraryRuntime>,
    mut vaos: ResMut<AssetVAOLibrary>,
    mut commands: Commands,
) {
    for (entity, mesh) in &mesh_entities {
        if meshes.get(*mesh).is_some_and(|asset| asset.indices.is_empty()) {
            log::trace!(target: VAO_LOG_TARGET, "Mesh is empty. Skip building VAO");
            continue;
        }
        // Missing CPU meshes reach the canonical add error path too.
        match vaos.add(runtime.buffers.as_ref(), &meshes, *mesh) {
            Ok(vao) => { commands.entity(entity).insert(vao); }
            Err(error) => log::warn!(target: VAO_LOG_TARGET, "Couldn't build VAO: {error:?}"),
        }
    }
}

/// Original B rebuild_vaos@140864bf0 (1115 bytes). Preserve modified queue order
/// and duplicates; only actual rebuild failures enter the replacement queue.
pub fn rebuild_vaos(
    mut meshes: ResMut<AssetMeshLibrary>,
    mut vaos: ResMut<AssetVAOLibrary>,
    runtime: Res<AssetLibraryRuntime>,
) {
    if meshes.modified_meshes.is_empty() {
        return;
    }
    let mut failed = Vec::new();
    for mesh in &meshes.modified_meshes {
        if !vaos.has_vao(*mesh) {
            log::warn!(target: VAO_LOG_TARGET, "WARNING: Tried rebuilding a mesh that has no VAOs!");
            continue;
        }
        if let Err(error) = vaos.rebuild_vao(runtime.buffers.as_ref(), &meshes, *mesh) {
            failed.push(*mesh);
            log::warn!(target: VAO_LOG_TARGET, "Couldn't rebuild VAO: {error:?}");
        }
    }
    meshes.modified_meshes = failed;
}

#[cfg(test)]
mod vao_prefix_tests {
    use super::*;
    use crate::{
        asset_libraries::mesh_library::MeshAsset,
        render::{mesh::{ATTR_POSITION, VertexAttributeValues},
            ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc, usage},
            test_device::TestDevice},
    };
    use std::{ops::Range, sync::{Mutex, atomic::{AtomicBool, AtomicU64, Ordering}}};

    // CPU publication/failure recorder, with distinct allocation identities.
    // These tests make no Vulkan submission or draw-output assertion.
    #[derive(Default)]
    struct RecordingBuffers {
        next_id: AtomicU64,
        fail: AtomicBool,
        index_attempts: Mutex<Vec<u32>>,
        released: Mutex<Vec<u64>>,
    }
    impl GraphicsDevice for RecordingBuffers {
        fn create_buffer(&self, desc: &BufferDesc, _: &str, data: Option<&[u8]>) -> Result<Buffer, BackendError> {
            if desc.usage == usage::INDEX_BUFFER {
                let first = data.unwrap()[..4].try_into().unwrap();
                self.index_attempts.lock().unwrap().push(u32::from_ne_bytes(first));
            }
            if self.fail.load(Ordering::Relaxed) {
                return Err(BackendError::OutOfDeviceMemory);
            }
            Ok(Buffer {
                id: self.next_id.fetch_add(1, Ordering::Relaxed) + 1,
                byte_size: desc.size as u64,
                ..Buffer::default()
            })
        }
        fn release_buffer(&self, buffer: &Buffer) {
            self.released.lock().unwrap().push(buffer.id);
        }
        fn update_buffer_bytes(&self, _: &Buffer, _: &[u8], _: &[Range<usize>], _: usize) -> Result<(), BackendError> {
            Ok(())
        }
    }
    fn triangle(first: u32) -> Mesh {
        let mut mesh = Mesh::new("vao-prefix-test");
        mesh.indices = vec![first, 0, 2];
        mesh.attributes.insert(ATTR_POSITION.into(), VertexAttributeValues::Float3(vec![
            glam::Vec3::ZERO, glam::Vec3::X, glam::Vec3::Y,
        ]));
        mesh
    }
    fn world_with_device(device: Arc<RecordingBuffers>) -> World {
        let mut world = World::new();
        world.insert_resource(AssetLibraryRuntime::new(device, Arc::new(TestDevice(1))));
        world.insert_resource(AssetMeshLibrary::new());
        world.insert_resource(AssetVAOLibrary::new());
        world
    }

    #[derive(Resource)]
    struct PublicationProbe { entity: Entity, mesh: Handle<Mesh>, before: bool, after: bool }
    fn before_barrier(query: Query<&Handle<Vao>>, vaos: Res<AssetVAOLibrary>, mut probe: ResMut<PublicationProbe>) {
        assert!(vaos.has_vao(probe.mesh));
        assert!(query.get(probe.entity).is_err());
        probe.before = true;
    }
    fn after_barrier(query: Query<&Handle<Vao>>, vaos: Res<AssetVAOLibrary>, mut probe: ResMut<PublicationProbe>) {
        let handle = *query.get(probe.entity).unwrap();
        assert!(vaos.get_by_handle(handle).is_some());
        probe.after = true;
    }

    #[test]
    fn missing_components_publish_at_explicit_barrier_and_existing_components_are_excluded() {
        let device = Arc::new(RecordingBuffers::default());
        let mut world = world_with_device(device.clone());
        let meshes = world.resource::<AssetMeshLibrary>();
        let mesh = meshes.add(MeshAsset::new(triangle(1), None));
        let excluded_mesh = meshes.add(MeshAsset::new(triangle(2), None));
        let empty_mesh = meshes.add(MeshAsset::new(Mesh::new("empty"), None));
        let entity = world.spawn(mesh).id();
        let stale = Handle::<Vao>::new();
        let excluded = world.spawn((excluded_mesh, stale)).id();
        let empty = world.spawn(empty_mesh).id();
        let missing = world.spawn(Handle::<Mesh>::new()).id();
        world.insert_resource(PublicationProbe { entity, mesh, before: false, after: false });
        let mut schedule = Schedule::default();
        schedule.add_systems((build_missing_vaos, rebuild_vaos, before_barrier,
            bevy_ecs::schedule::ApplyDeferred, after_barrier).chain_ignore_deferred());
        schedule.run(&mut world);
        let probe = world.resource::<PublicationProbe>();
        assert!(probe.before && probe.after);
        assert_eq!(world.get::<Handle<Vao>>(excluded), Some(&stale));
        assert!(!world.resource::<AssetVAOLibrary>().has_vao(excluded_mesh));
        assert!(world.get::<Handle<Vao>>(empty).is_none());
        assert!(world.get::<Handle<Vao>>(missing).is_none());
        assert_eq!(*device.index_attempts.lock().unwrap(), vec![1]);
    }

    #[test]
    fn modified_queue_preserves_duplicate_failures_and_actual_retry_order() {
        let device = Arc::new(RecordingBuffers::default());
        let mut world = world_with_device(device.clone());
        let meshes = world.resource::<AssetMeshLibrary>();
        let a = meshes.add(MeshAsset::new(triangle(1), None));
        let b = meshes.add(MeshAsset::new(triangle(2), None));
        let no_vao = meshes.add(MeshAsset::new(triangle(0), None));
        let mut vaos = world.remove_resource::<AssetVAOLibrary>().unwrap();
        let a_vao = vaos.add(device.as_ref(), world.resource(), a).unwrap();
        let b_vao = vaos.add(device.as_ref(), world.resource(), b).unwrap();
        let old_a_buffer = vaos.get(a).unwrap().vao.index_buffer.id;
        world.insert_resource(vaos);
        device.index_attempts.lock().unwrap().clear();
        world.resource_mut::<AssetMeshLibrary>().modified_meshes = vec![b, a, b, no_vao];
        device.fail.store(true, Ordering::Relaxed);
        let mut schedule = Schedule::default();
        schedule.add_systems(rebuild_vaos);
        schedule.run(&mut world);
        assert_eq!(world.resource::<AssetMeshLibrary>().modified_meshes, vec![b, a, b]);
        assert_eq!(*device.index_attempts.lock().unwrap(), vec![2, 1, 2]);
        assert_eq!(world.resource::<AssetVAOLibrary>().get(a).unwrap().vao.index_buffer.id, old_a_buffer);
        assert!(device.released.lock().unwrap().is_empty());
        device.fail.store(false, Ordering::Relaxed);
        device.index_attempts.lock().unwrap().clear();
        schedule.run(&mut world);
        assert!(world.resource::<AssetMeshLibrary>().modified_meshes.is_empty());
        assert_eq!(*device.index_attempts.lock().unwrap(), vec![2, 1, 2]);
        assert!(world.resource::<AssetVAOLibrary>().get_by_handle(a_vao).is_some());
        assert!(world.resource::<AssetVAOLibrary>().get_by_handle(b_vao).is_some());
        assert_eq!(device.released.lock().unwrap().len(), 6);
    }

    #[test]
    fn empty_modified_queue_does_not_mark_the_mesh_library_changed() {
        let mut world = world_with_device(Arc::new(RecordingBuffers::default()));
        let before = world.resource_ref::<AssetMeshLibrary>().last_changed();
        let mut schedule = Schedule::default();
        schedule.add_systems(rebuild_vaos);
        schedule.run(&mut world);
        assert_eq!(world.resource_ref::<AssetMeshLibrary>().last_changed(), before);
    }
}
