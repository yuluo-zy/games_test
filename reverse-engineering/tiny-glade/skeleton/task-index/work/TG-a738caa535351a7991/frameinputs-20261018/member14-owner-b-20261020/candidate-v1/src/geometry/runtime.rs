//! 同库墙砖在PostUpdate统一flush：base容量失败保旧，装饰host保原partial语义；
//! 设备失败保dirty。AwaitingProduction与设备上传状态分别表达。
use super::instanced_wall::{BrickTransformSsbo, InstancedWall, WallUploadError};
use crate::{
    asset_libraries::{runtime::AssetLibraryRuntime, ssbo_library::AssetSsboLibrary},
    render::sparse_instance_buffer::SparseInstanceBuffer,
};
use bevy_ecs::prelude::*;
use crate::resources::walls::public_walls::{PublicWalls, WallEntities};
use crate::resources::walls::WallId;
use crate::render::sparse_instance_buffer::SparseInstanceBufferWriter;

/// Same real19 publication output and same WallEntities authority. Runs after
/// publication/deletion; ordinary set edges make Commands visible before25.
pub fn prepare_wall_brick_assembly_owners(
    public: Res<PublicWalls>,
    library: Res<AssetSsboLibrary>,
    mut entities: ResMut<WallEntities>,
    mut owners: Query<&mut InstancedWall<BrickTransformSsbo>>,
    mut commands: Commands,
) {
    if public.last_delta.changed.is_empty() {
        return;
    }
    let storage = library.get::<BrickTransformSsbo>()
        .expect("published wall requires the actual BrickTransformSsbo storage");
    for &wall in &public.last_delta.changed {
        if public.get(wall).is_none() {
            continue;
        }
        if let Some(entity) = entities.wall_entity(wall) {
            let mut owner = owners.get_mut(entity)
                .expect("WallEntities contains a stale wall brick owner");
            assert_eq!(owner.id, wall.0, "WallEntities points to a different wall id");
            // Read-only validation leaves a complete unchanged owner tick alone.
            if !owner.validate_assembly_ownership(storage)
                .expect("wall brick owner belongs to another storage")
            {
                owner.initialize_assembly_allocations(storage)
                    .expect("legacy wall brick owner belongs to another storage");
            }
        } else {
            let entity = commands.spawn(
                InstancedWall::new_awaiting_production(wall.0, storage)
            ).id();
            entities.0.insert(wall, entity);
        }
    }
}

/// Native hosts retain their actual Library input; this scoped adapter adds
/// the current Entity mapping, because the two lanes no longer live in PublicWallState.
#[derive(bevy_ecs::system::SystemParam)]
pub struct WallBrickAssemblyResources<'w, 's> {
    library: ResMut<'w, AssetSsboLibrary>,
    entities: Res<'w, WallEntities>,
    owners: Query<'w, 's, &'static InstancedWall<BrickTransformSsbo>>,
}

impl<'w, 's> WallBrickAssemblyResources<'w, 's> {
    /// Exactly one typed entry lookup per dirty wall, before the PublicWalls gate.
    /// With no dirty wall the host never calls this method.
    pub fn for_batch<'a>(&'a mut self) -> WallBrickAssemblyBatch<'a, 'w, 's> {
        let storage = self.library.get_mut::<BrickTransformSsbo>()
            .expect("wall assembly requires the actual BrickTransformSsbo storage");
        WallBrickAssemblyBatch { storage, entities: &self.entities, owners: &self.owners }
    }
}

/// Only references into the same SystemParam borrow. No ownership or registry
/// is copied; this view cannot outlive or cross its wall callback.
pub struct WallBrickAssemblyBatch<'a, 'w, 's> {
    storage: &'a mut SparseInstanceBuffer<BrickTransformSsbo>,
    entities: &'a WallEntities,
    owners: &'a Query<'w, 's, &'static InstancedWall<BrickTransformSsbo>>,
}

impl WallBrickAssemblyBatch<'_, '_, '_> {
    pub fn chimney_writer(&mut self, wall: WallId) -> SparseInstanceBufferWriter<'_, BrickTransformSsbo> {
        self.writer(wall, true)
    }

    pub fn door_stairs_writer(&mut self, wall: WallId) -> SparseInstanceBufferWriter<'_, BrickTransformSsbo> {
        self.writer(wall, false)
    }

    fn writer(&mut self, wall: WallId, chimney: bool) -> SparseInstanceBufferWriter<'_, BrickTransformSsbo> {
        let entity = self.entities.wall_entity(wall)
            .expect("present Public wall has no wall brick owner");
        let owner = self.owners.get(entity)
            .expect("present Public wall has a stale wall brick owner");
        assert_eq!(owner.id, wall.0, "WallEntities points to a different wall id");
        let allocation = if chimney { owner.chimney_allocation() } else { owner.door_stairs_allocation() }
            .expect("present Public wall has no preinitialized assembly lane");
        // The canonical writer validates same-storage ownership and owns the
        // allocation's real mutex guard until the host finishes removals.
        self.storage.writer(allocation)
    }
}

/// 现raster绘制入口据此拦截设备上传失败；compute未用同gate。
/// 此状态不是base真实几何/Trigger已经构造的证据。
#[derive(Resource, Debug)]
pub struct WallBrickUploadStatus {
    pub gpu_ready: bool,
    pub capacity_failures: usize,
    pub last_error: Option<String>,
}

impl Default for WallBrickUploadStatus {
    fn default() -> Self {
        Self {
            gpu_ready: true,
            capacity_failures: 0,
            last_error: None,
        }
    }
}

impl WallBrickUploadStatus {
    fn report_device_error(&mut self, error: impl std::fmt::Display) {
        let message = error.to_string();
        if self.last_error.as_ref() != Some(&message) {
            log::error!("墙砖设备发布失败，保留脏数据等待重试：{message}");
        }
        self.gpu_ready = false;
        self.last_error = Some(message);
    }
}

/// 每堵墙只在待提交时改写 CPU 影子；共享缓冲每帧最多提交一次。
/// 实体销毁时其分配自动入回收队列，flush 前归还 chunk。
pub fn sync_wall_bricks(
    runtime: Option<Res<AssetLibraryRuntime>>,
    library: Option<ResMut<AssetSsboLibrary>>,
    status: Option<ResMut<WallBrickUploadStatus>>,
    mut walls: Query<&mut InstancedWall<BrickTransformSsbo>>,
) {
    let (Some(runtime), Some(mut library), Some(mut status)) = (runtime, library, status) else {
        return;
    };
    if !library.contains::<BrickTransformSsbo>() {
        let mut needs_storage = false;
        for mut wall in &mut walls {
            if wall.is_upload_pending() {
                wall.finish_unallocated_empty_update();
                needs_storage |= wall.is_upload_pending();
            }
        }
        if !needs_storage {
            status.gpu_ready = true;
            status.last_error = None;
            status.capacity_failures = 0;
            return;
        }
        match SparseInstanceBuffer::<BrickTransformSsbo>::try_new_owned(runtime.buffers.clone()) {
            Ok(storage) => library.register::<BrickTransformSsbo>(storage),
            Err(error) => {
                status.report_device_error(error);
                return;
            }
        }
    }
    let storage = library
        .get_mut::<BrickTransformSsbo>()
        .expect("墙砖存储已经注册");
    storage.take_out_waste();
    let mut capacity_failures = 0;
    for mut wall in &mut walls {
        if wall.is_upload_pending() {
            match wall.upload_to_storage_buffer(storage) {
                Err(WallUploadError::CapacityExceeded) => capacity_failures += 1,
                Err(WallUploadError::ForeignStorage) => {
                    status
                        .report_device_error("墙的稀疏分配不属于当前存储，禁止跨设备或跨存储复用");
                    return;
                }
                Ok(_) => {}
            }
        }
    }
    if capacity_failures != status.capacity_failures && capacity_failures > 0 {
        log::warn!("{capacity_failures} 堵墙的砖实例空间不足，保留旧几何并等待空闲 chunk");
    }
    status.capacity_failures = capacity_failures;
    match storage.try_gpu_flush(runtime.buffers.as_ref()) {
        Ok(()) => {
            status.gpu_ready = true;
            status.last_error = None;
        }
        Err(error) => status.report_device_error(error),
    }
}

#[cfg(test)]
#[path = "tests/runtime.rs"]
mod tests;
