//! transient_mesh.rs — 瞬时网格组件与注册表（O）。
//!
//! 源 `components/transient_mesh.rs`；实体 `TransientMesh` /
//! `TransientMeshRegistry`。
//! 功能（O）：`TransientMesh` 组件持有 `Arc<TransientMeshRegistry>`（
//! `new_with_registry` 0x140aa8200）；Drop（`impl$2::drop` 0x140899f90）→
//! `TransientMeshRegistry::deferred_delete_mesh`（0x140aa8070）把网格句柄入待删
//! 队列；backend `delete_dropped_transient_meshes` 经
//! `drain_meshes_to_delete`（0x1412e9e70）逐帧释放 GPU 网格。

#![allow(dead_code)]

use std::sync::Arc;

use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 瞬时网格注册表（O：`Res<TransientMeshRegistry>`；`Mutex` + 待删 Vec）。
#[derive(Resource, Debug, Default)]
pub struct TransientMeshRegistry {
    /// 待删网格句柄队列（O：`deferred_delete_mesh` push，Vec<u64>）。
    pub pending_delete: std::sync::Mutex<Vec<u64>>,
    pub _gap: Gap,
}

impl TransientMeshRegistry {
    /// `deferred_delete_mesh`（O 0x140aa8070）：锁队列，push 句柄。
    pub fn deferred_delete_mesh(&self, mesh_handle: u64) {
        if let Ok(mut q) = self.pending_delete.lock() {
            q.push(mesh_handle);
        }
    }

    /// `drain_meshes_to_delete<F>`（O 0x1412e9e70）：排空队列并对每个句柄调
    /// 闭包（backend 释放 GPU 网格）。
    pub fn drain_meshes_to_delete<F>(&self, mut f: F)
    where
        F: FnMut(u64),
    {
        let drained = if let Ok(mut q) = self.pending_delete.lock() {
            std::mem::take(&mut *q)
        } else {
            Vec::new()
        };
        for h in drained {
            f(h);
        }
    }
}

/// 瞬时网格组件（O：`TransientMesh`；持有注册表 Arc，Drop 时登记删除）。
#[derive(Component, Clone, Debug, Default)]
pub struct TransientMesh {
    pub registry: Arc<TransientMeshRegistry>,
    pub mesh_handle: u64,
    pub _gap: Gap,
}

impl TransientMesh {
    /// `new_with_registry`（O 0x140aa8200）。
    pub fn new_with_registry(registry: Arc<TransientMeshRegistry>, mesh_handle: u64) -> Self {
        TransientMesh {
            registry,
            mesh_handle,
            _gap: Gap,
        }
    }
}

impl Drop for TransientMesh {
    /// `impl$2::drop`（O 0x140899f90）—— 组件移除时延迟删除网格。
    fn drop(&mut self) {
        self.registry.deferred_delete_mesh(self.mesh_handle);
    }
}
