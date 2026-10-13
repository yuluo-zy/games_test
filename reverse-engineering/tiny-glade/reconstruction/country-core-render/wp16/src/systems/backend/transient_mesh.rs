//! transient_mesh.rs — 瞬时网格后端系统（O）。
//!
//! 源 `systems/backend/transient_mesh.rs`。
//! `delete_dropped_transient_meshes`（内联）：排空 `TransientMeshRegistry`
//! 的待删网格句柄并释放 GPU 网格（组件在 backend 每帧跑）。

#![allow(dead_code)]

use crate::components::transient_mesh::TransientMeshRegistry;

/// `delete_dropped_transient_meshes`（O：`TransientMeshRegistry::drain_meshes_to_delete`
/// 0x1412e9e70 的消费闭包）。
pub fn delete_dropped_transient_meshes(_world: &mut bevy_ecs::world::World) {
    // U：GPU 网格释放路径（typed gap）。
}

/// 删除处理器（O：`drain_meshes_to_delete` 闭包负载）。
pub struct DeleteDroppedTransientMeshes {
    pub registry: std::sync::Arc<TransientMeshRegistry>,
}
