//! 墙砖在 PostUpdate 统一发布：容量失败保留旧版本，设备失败保留脏数据。
use super::instanced_wall::{BrickTransformSsbo, InstancedWall, WallUploadError};
use crate::{
    asset_libraries::{runtime::AssetLibraryRuntime, ssbo_library::AssetSsboLibrary},
    render::sparse_instance_buffer::SparseInstanceBuffer,
};
use bevy_ecs::prelude::*;

/// 绘制入口用此状态阻止读取上传失败后的不完整设备数据。
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
