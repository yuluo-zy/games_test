//! `country_core::systems::wall::upload_instanced_wall_ssbo`（原版源文件）。
//!
//! 原版 `gpu_flush_brick_buffers`（0x14082c170，594B；SystemParam 面：
//! `(Res<GraphicsDeviceResource>, ResMut<AssetSsboLibrary>,
//! Query<ComponentWrite<InstancedWall<BrickTransformSsbo>>>)`，按 0x14082c170 的参数
//! 指针 param_1 → 设备、param_2 → ResMut 库、param_3 → 查询状态还原）：
//!
//! 1. `AssetSsboLibrary::get_mut::<BrickTransformSsbo>()`；`None` → `Option::unwrap` panic
//!    （ResMut 的变更 tick 在 `deref_mut` 时写入，`*(param_2[2]) = this_run`）。
//! 2. 遍历 Query（dense 与 sparse 两条迭代路径均无条件逐个调用
//!    `InstancedWall::upload_to_storage_buffer(wall, storage)`；组件 tick 同样被写
//!    为 this_run，即 `&mut` 访问）。待提交判断在 `upload_to_storage_buffer` 内部。
//! 3. `SparseInstanceBuffer::gpu_flush`（内部先 `take_out_waste`，再把脏 chunk 位图合并成区间后
//!    `TypedStorageBuffer::gpu_write_ranges_nonoverlapping`，并清脏位）。
//!
//! 未闭合：反编译无法证实 Query 是否带 With/Without 过滤（运行体没有过滤分支）；
//! 本实现用无过滤 `Query<ComponentWrite<InstancedWall<BrickTransformSsbo>>>`。
use bevy_ecs::prelude::*;

use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
use crate::geometry::instanced_wall::{BrickTransformSsbo, InstancedWall};
use crate::render::device::GraphicsDeviceResource;

/// 原版 `gpu_flush_brick_buffers`（0x14082c170）。
pub fn gpu_flush_brick_buffers(
    device: Res<GraphicsDeviceResource>,
    mut library: ResMut<AssetSsboLibrary>,
    mut walls: Query<ComponentWrite<InstancedWall<BrickTransformSsbo>>>,
) {
    let storage = library.get_mut::<BrickTransformSsbo>().unwrap();
    for mut wall in &mut walls {
        // 容量不足/跨存储：保留 CPU 数据与旧分配，待下一帧重试（同 geometry::runtime::sync_wall_bricks）。
        if let Err(error) = wall.upload_to_storage_buffer(storage) {
            log::error!("gpu_flush_brick_buffers: 墙砖上传失败：{error:?}");
        }
    }
    if let Err(error) = storage.gpu_flush(device.device()) {
        log::error!("gpu_flush_brick_buffers: 设备上传失败，脏标记保留待重试：{error}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_param_surface_is_registrable() {
        let mut world = World::new();
        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(gpu_flush_brick_buffers);
        // 初始化参数面即可验证 Res/ResMut/Query 访问冲突检查；不运行（需要设备资源）。
        schedule.initialize(&mut world).unwrap();
    }
}

use crate::render::extract::ComponentWrite;
