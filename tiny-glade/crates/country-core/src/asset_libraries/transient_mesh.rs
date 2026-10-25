//! 瞬态网格生命周期。组件析构只入队，游戏线程释放 CPU 网格和 GPU 缓冲。
//! 原作来源见项目 evidence；当前接口、行为变化与验证范围见 README.md。

use std::sync::{Arc, Mutex};

use bevy_ecs::component::Component;

use super::Handle;
use super::mesh_library::AssetMeshLibrary;
use super::vao_library::AssetVAOLibrary;
use crate::render::mesh::Mesh;

/// 共享的待删除队列。
pub type TransientMeshRegistry = Mutex<Vec<Handle<Mesh>>>;

/// 随实体存活的回收凭据，析构仅登记删除请求。
#[derive(Component)]
pub struct TransientMesh {
    registry: Arc<TransientMeshRegistry>,
    mesh_handle: Handle<Mesh>,
}

impl TransientMesh {
    pub fn new_with_registry(
        mesh_handle: Handle<Mesh>,
        registry: &Arc<TransientMeshRegistry>,
    ) -> Self {
        Self {
            registry: Arc::clone(registry),
            mesh_handle,
        }
    }

    pub fn mesh_handle(&self) -> Handle<Mesh> {
        self.mesh_handle
    }
}

impl Drop for TransientMesh {
    fn drop(&mut self) {
        self.registry
            .lock()
            .unwrap_or_else(|poison| poison.into_inner())
            .push(self.mesh_handle);
    }
}

pub fn drain_meshes_to_delete(
    registry: &TransientMeshRegistry,
    mut delete: impl FnMut(Handle<Mesh>),
) {
    // 批次出队后释放锁，删除与析构过程中允许新请求入队。
    // 若回调中途展开，守卫会把未完成项归还队列。
    struct DeletionBatch<'a> {
        registry: &'a TransientMeshRegistry,
        handles: Vec<Handle<Mesh>>,
        completed: usize,
    }
    impl Drop for DeletionBatch<'_> {
        fn drop(&mut self) {
            self.handles.drain(..self.completed);
            let mut arrivals = self
                .registry
                .lock()
                .unwrap_or_else(|poison| poison.into_inner());
            self.handles.append(&mut arrivals);
            std::mem::swap(&mut *arrivals, &mut self.handles);
        }
    }
    let handles =
        std::mem::take(&mut *registry.lock().unwrap_or_else(|poison| poison.into_inner()));
    let mut batch = DeletionBatch {
        registry,
        handles,
        completed: 0,
    };
    while let Some(handle) = batch.handles.get(batch.completed).copied() {
        delete(handle);
        batch.completed += 1;
    }
}

/// `delete_dropped_transient_meshes`：先克隆注册表的 `Arc`，再逐个释放网格与 VAO（顺序固定：网格库、VAO 库）。
pub fn delete_dropped_transient_meshes(
    device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
    meshes: &mut AssetMeshLibrary,
    vaos: &mut AssetVAOLibrary,
) {
    let registry = Arc::clone(meshes.transient_meshes());
    drain_meshes_to_delete(&registry, |handle| {
        meshes.remove(handle);
        vaos.remove(device, handle);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn registry() -> Arc<TransientMeshRegistry> {
        Arc::new(Mutex::new(Vec::new()))
    }

    #[test]
    fn creating_a_component_does_not_touch_the_queue() {
        let reg = registry();
        let h = Handle::<Mesh>::new();
        let t = TransientMesh::new_with_registry(h, &reg);
        assert_eq!(t.mesh_handle(), h);
        assert_eq!(Arc::strong_count(&reg), 2);
        assert!(reg.lock().unwrap().is_empty());
    }

    #[test]
    fn dropping_the_component_queues_its_handle_in_order() {
        let reg = registry();
        let (a, b) = (Handle::<Mesh>::new(), Handle::<Mesh>::new());
        let ta = TransientMesh::new_with_registry(a, &reg);
        let tb = TransientMesh::new_with_registry(b, &reg);
        drop(tb);
        drop(ta);
        assert_eq!(*reg.lock().unwrap(), vec![b, a]);
        assert_eq!(Arc::strong_count(&reg), 1);
    }

    #[test]
    fn drain_releases_lock_and_keeps_capacity() {
        let reg = registry();
        let hs: Vec<_> = (0..5).map(|_| Handle::<Mesh>::new()).collect();
        reg.lock().unwrap().extend(hs.iter().copied());
        let cap_before = reg.lock().unwrap().capacity();
        let mut seen = Vec::new();
        drain_meshes_to_delete(&reg, |h| {
            assert!(reg.try_lock().is_ok(), "回调必须在队列锁之外执行");
            seen.push(h);
        });
        assert_eq!(seen, hs);
        let queue = reg.lock().unwrap();
        assert!(queue.is_empty());
        assert_eq!(queue.capacity(), cap_before);
    }

    #[test]
    fn poisoned_registry_still_allows_reclamation() {
        let reg = registry();
        let r2 = Arc::clone(&reg);
        let _ = std::thread::spawn(move || {
            let _guard = r2.lock().unwrap();
            panic!("poison");
        })
        .join();
        let result = std::panic::catch_unwind(|| drain_meshes_to_delete(&reg, |_| {}));
        assert!(result.is_ok());
    }

    #[test]
    fn delete_dropped_removes_the_meshes_from_the_library() {
        let mut meshes = AssetMeshLibrary::new();
        let mut vaos = AssetVAOLibrary::new();
        let handle = meshes.add(crate::asset_libraries::mesh_library::MeshAsset::new(
            Mesh::default(),
            None,
        ));
        let component = TransientMesh::new_with_registry(handle, meshes.transient_meshes());
        assert!(meshes.get(handle).is_some());
        delete_dropped_transient_meshes(
            &crate::render::test_device::TestDevice(0),
            &mut meshes,
            &mut vaos,
        );
        assert!(
            meshes.get(handle).is_some(),
            "still alive while the entity exists"
        );
        drop(component);
        delete_dropped_transient_meshes(
            &crate::render::test_device::TestDevice(0),
            &mut meshes,
            &mut vaos,
        );
        assert!(meshes.get(handle).is_none());
        assert!(meshes.transient_meshes().lock().unwrap().is_empty());
    }
}
