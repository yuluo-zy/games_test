//! `transient_mesh.rs` — `TransientMesh` 组件与"延迟删除网格"链（B `components::transient_mesh` +
//! `backend::delete_dropped_transient_meshes`）。
//!
//! 证据（`components-geometry/ALGORITHMS.md` §4，机器码为准）：
//! - 注册表就是 `AssetMeshLibrary.transient_meshes`（`Arc<Mutex<Vec<u64>>>`，std futex Mutex），不是独立 `Res`；
//! - `TransientMesh = { registry: Arc<..>@+0, mesh_handle: u64@+8 }`（16 B）；
//!   `new_with_registry`（0x140aa8200）只做 `Arc::clone`，实参序是 `(mesh_handle, &Arc)`；
//! - `Drop`（0x140899f90）：加锁（中毒 → `unwrap` panic）后 `push(mesh_handle)`，组件本身不碰 GPU；
//! - `drain_meshes_to_delete`（0x1412e9e70）：加锁（中毒 → panic），在**锁内**对每个句柄调用闭包，随后只把
//!   `len` 清零（保留容量，不是 `mem::take`）；闭包依次 `AssetMeshLibrary::remove`（0x14092d750）、
//!   `AssetVAOLibrary::remove`（0x1409247a0）；
//! - 系统参数面只有 `(ResMut<AssetMeshLibrary>, ResMut<AssetVAOLibrary>)`，持有注册表的 `Arc` 克隆。
use std::sync::{Arc, Mutex};

use bevy_ecs::component::Component;

use super::Handle;
use super::mesh_library::AssetMeshLibrary;
use super::vao_library::AssetVAOLibrary;
use crate::render::mesh::Mesh;

/// 共享的待删除队列。
pub type TransientMeshRegistry = Mutex<Vec<Handle<Mesh>>>;

/// 随实体存活的瞬态网格；实体移除时把句柄交还给注册表。
#[derive(Component)]
pub struct TransientMesh {
    registry: Arc<TransientMeshRegistry>,
    mesh_handle: Handle<Mesh>,
}

impl TransientMesh {
    /// `new_with_registry`（0x140aa8200）：克隆 `Arc`，不碰队列。
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
    /// 0x140899f90：`registry.lock().unwrap().push(mesh_handle)`。
    fn drop(&mut self) {
        self.registry.lock().unwrap().push(self.mesh_handle);
    }
}

/// `drain_meshes_to_delete`（0x1412e9e70）：锁内逐个调用 `delete`，之后 `len = 0`、容量保留。
pub fn drain_meshes_to_delete(
    registry: &TransientMeshRegistry,
    mut delete: impl FnMut(Handle<Mesh>),
) {
    let mut queue = registry.lock().unwrap();
    for handle in queue.iter().copied() {
        delete(handle);
    }
    queue.clear();
}

/// `delete_dropped_transient_meshes`：先克隆注册表的 `Arc`，再逐个释放网格与 VAO（顺序固定：网格库、VAO 库）。
pub fn delete_dropped_transient_meshes(meshes: &mut AssetMeshLibrary, vaos: &mut AssetVAOLibrary) {
    let registry = Arc::clone(meshes.transient_meshes());
    drain_meshes_to_delete(&registry, |handle| {
        meshes.remove(handle);
        vaos.remove(handle);
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
    fn drain_visits_every_handle_under_the_lock_and_keeps_capacity() {
        let reg = registry();
        let hs: Vec<_> = (0..5).map(|_| Handle::<Mesh>::new()).collect();
        reg.lock().unwrap().extend(hs.iter().copied());
        let cap_before = reg.lock().unwrap().capacity();
        let mut seen = Vec::new();
        drain_meshes_to_delete(&reg, |h| {
            assert!(
                reg.try_lock().is_err(),
                "closure must run with the lock held"
            );
            seen.push(h);
        });
        assert_eq!(seen, hs);
        let queue = reg.lock().unwrap();
        assert!(queue.is_empty());
        assert_eq!(queue.capacity(), cap_before);
    }

    #[test]
    fn poisoned_registry_panics_like_the_original_unwrap() {
        let reg = registry();
        let r2 = Arc::clone(&reg);
        let _ = std::thread::spawn(move || {
            let _guard = r2.lock().unwrap();
            panic!("poison");
        })
        .join();
        let result = std::panic::catch_unwind(|| drain_meshes_to_delete(&reg, |_| {}));
        assert!(result.is_err());
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
        delete_dropped_transient_meshes(&mut meshes, &mut vaos);
        assert!(
            meshes.get(handle).is_some(),
            "still alive while the entity exists"
        );
        drop(component);
        delete_dropped_transient_meshes(&mut meshes, &mut vaos);
        assert!(meshes.get(handle).is_none());
        assert!(meshes.transient_meshes().lock().unwrap().is_empty());
    }
}
