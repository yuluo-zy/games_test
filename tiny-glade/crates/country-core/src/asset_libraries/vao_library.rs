//! 网格 GPU 缓冲注册表。创建和重建成功后发布，删除通过宿主释放物理资源。
//! 原作来源见项目 evidence；当前接口、行为变化与验证范围见 README.md。

use std::{collections::HashMap, sync::{Arc, atomic::{AtomicU64, Ordering}}};

// A clone shares published entries, but each later mutation gets a globally
// unique revision. Two branches must never publish different tables with the
// same revision.
fn next_revision() -> u64 {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |revision| revision.checked_add(1))
        .expect("VAO publication revision exhausted")
}

use anyhow::anyhow;
use bevy_ecs::resource::Resource;

use crate::asset_libraries::Handle;
use crate::asset_libraries::mesh_library::AssetMeshLibrary;
use crate::render::mesh::Mesh;
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::render::vao::{Vao, release_vao, try_build_vao};
use crate::render::extract::{ChangeCursor, ChangeJournal};

/// 已发布 VAO 与绘制所需索引数。
#[derive(Clone)]
pub struct VaoEntry {
    pub vao: Vao,
    pub index_count: usize,
}

/// 维护 mesh → VAO 身份 → GPU 数据的对应关系。
#[derive(Resource)]
pub struct AssetVAOLibrary {
    vaos: HashMap<Handle<Vao>, Arc<VaoEntry>>,
    mesh_to_vao: HashMap<Handle<Mesh>, Handle<Vao>>,
    /// 任何实际改变 VAO 表的 `add/rebuild_vao/remove` 都会自增。宿主据此判断
    /// 同一帧里缓存的 VAO 快照是否仍然有效。克隆后分支修改也产生唯一版本。
    revision: u64,
    changes: ChangeJournal<Handle<Mesh>>,
    source_cursor: Option<ChangeCursor>,
}

impl Clone for AssetVAOLibrary {
    fn clone(&self) -> Self {
        let source_cursor = Some(self.changes.cursor());
        Self {
            vaos: self.vaos.clone(), mesh_to_vao: self.mesh_to_vao.clone(), revision: self.revision,
            changes: ChangeJournal::default(), source_cursor,
        }
    }
}

impl AssetVAOLibrary {
    /// 创建初始状态。
    pub fn new() -> Self {
        AssetVAOLibrary {
            vaos: HashMap::new(),
            mesh_to_vao: HashMap::new(),
            revision: next_revision(),
            changes: ChangeJournal::default(),
            source_cursor: None,
        }
    }

    /// VAO 表的修订号;只有表真的发生变化时才改变。
    pub fn revision(&self) -> u64 {
        self.revision
    }
    pub fn snapshot_revision(&self) -> u64 { self.changes.revision() }

    /// Share only newly published VAO metadata and retire deleted mappings.
    /// The destination retains its tables and all unaffected Arc owners.
    pub fn update_render_snapshot(&self, destination: &mut Self) {
        let Some((cursor, changed)) = self.changes.since(destination.source_cursor) else {
            *destination = self.clone();
            return;
        };
        let mut seen = std::collections::HashSet::with_capacity(changed.len());
        for mesh in changed {
            if !seen.insert(mesh) { continue; }
            if let Some(&handle) = self.mesh_to_vao.get(&mesh) {
                if let Some(old) = destination.mesh_to_vao.insert(mesh, handle) {
                    if old != handle { destination.vaos.remove(&old); }
                }
                destination.vaos.insert(handle, self.vaos[&handle].clone());
            } else if let Some(handle) = destination.mesh_to_vao.remove(&mesh) {
                destination.vaos.remove(&handle);
            }
            destination.changes.record(mesh);
        }
        destination.revision = self.revision;
        destination.source_cursor = Some(cursor);
    }

    pub fn has_vao(&self, mesh: Handle<Mesh>) -> bool {
        self.mesh_to_vao.contains_key(&mesh)
    }

    pub fn get(&self, mesh: Handle<Mesh>) -> Option<&VaoEntry> {
        self.vaos.get(self.mesh_to_vao.get(&mesh)?).map(Arc::as_ref)
    }

    /// 绘制包携带 VAO 身份；宿主按该身份解析已发布的数据。
    /// 网格句柄与 VAO 句柄分别索引，不能相互替代。
    pub fn get_by_handle(&self, vao: Handle<Vao>) -> Option<&VaoEntry> {
        self.vaos.get(&vao).map(Arc::as_ref)
    }

    /// Immutable publication owner; snapshots share the binding metadata until
    /// this particular VAO is rebuilt or removed.
    pub fn shared_by_handle(&self, vao: Handle<Vao>) -> Option<&Arc<VaoEntry>> {
        self.vaos.get(&vao)
    }

    /// 注册资产并返回稳定身份。
    pub fn add(
        &mut self,
        device: &dyn GraphicsDevice,
        mesh_lib: &AssetMeshLibrary,
        mesh: Handle<Mesh>,
    ) -> Result<Handle<Vao>, anyhow::Error> {
        if let Some(existing) = self.mesh_to_vao.get(&mesh) {
            return Ok(*existing);
        }
        let vao_handle = Handle::<Vao>::new();
        let mesh_ref = mesh_lib
            .get(mesh)
            .ok_or_else(|| anyhow!("Mesh is not in the  mesh library"))?;
        if mesh_ref.indices.is_empty() {
            return Err(anyhow!("Mesh is invalid"));
        }
        let vao = try_build_vao(device, mesh_ref)?;
        let index_count = mesh_ref.indices.len();
        drop(self.vaos.insert(vao_handle, Arc::new(VaoEntry { vao, index_count })));
        self.mesh_to_vao.insert(mesh, vao_handle);
        self.revision = next_revision();
        self.changes.record(mesh);
        Ok(vao_handle)
    }

    /// 构建成功后替换 GPU 缓冲，保留 VAO 身份。
    pub fn rebuild_vao(
        &mut self,
        device: &dyn GraphicsDevice,
        mesh_lib: &AssetMeshLibrary,
        mesh: Handle<Mesh>,
    ) -> Result<(), anyhow::Error> {
        let mesh_ref = mesh_lib
            .get(mesh)
            .ok_or_else(|| anyhow!("Mesh is not in the mesh library"))?;
        if mesh_ref.indices.is_empty() {
            self.remove(device, mesh);
            return Ok(());
        }
        let vao_handle = *self
            .mesh_to_vao
            .get(&mesh)
            .ok_or_else(|| anyhow!("The requested mesh doesn't exist"))?;
        let entry = self
            .vaos
            .get_mut(&vao_handle)
            .ok_or_else(|| anyhow!("VAO handle is invalid"))?;
        let new_vao = try_build_vao(device, mesh_ref)?;
        release_vao(device, &entry.vao);
        *entry = Arc::new(VaoEntry { vao: new_vao, index_count: mesh_ref.indices.len() });
        self.revision = next_revision();
        self.changes.record(mesh);
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.vaos.is_empty()
    }

    /// 删除 VAO 并释放宿主拥有的物理缓冲。
    pub fn remove(&mut self, device: &dyn GraphicsDevice, mesh: Handle<Mesh>) {
        if let Some(vao_handle) = self.mesh_to_vao.remove(&mesh) {
            self.revision = next_revision();
            self.changes.record(mesh);
            if let Some(entry) = self.vaos.remove(&vao_handle) {
                release_vao(device, &entry.vao);
            }
        }
    }
}

impl Default for AssetVAOLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asset_libraries::mesh_library::MeshAsset;
    use crate::render::mesh::VertexAttributeValues;
    use crate::render::ssbo::tests::MockDevice;
    use glam::Vec3;

    fn quad() -> Mesh {
        let mut m = Mesh::new("quad");
        m.indices = vec![0, 1, 2, 2, 1, 3];
        m.attributes.insert(
            "Vertex_Position".into(),
            VertexAttributeValues::Float3(vec![Vec3::X, Vec3::Y, Vec3::Z, Vec3::ONE]),
        );
        m
    }

    fn setup() -> (AssetMeshLibrary, Handle<Mesh>) {
        let lib = AssetMeshLibrary::new();
        let h = lib.add(MeshAsset::new(quad(), None));
        (lib, h)
    }

    #[test]
    fn revision_changes_exactly_when_the_table_changes() {
        let (meshes, h) = setup();
        let dev = MockDevice::default();
        let mut vaos = AssetVAOLibrary::new();
        let r0 = vaos.revision();
        vaos.add(&dev, &meshes, h).unwrap();
        let r1 = vaos.revision();
        assert_ne!(r0, r1);
        // 已存在的 mesh 不改表。
        vaos.add(&dev, &meshes, h).unwrap();
        assert_eq!(vaos.revision(), r1);
        // 失败的 add 不改表。
        assert!(vaos.add(&dev, &meshes, Handle::<Mesh>::new()).is_err());
        assert_eq!(vaos.revision(), r1);
        // 重建保留身份但换了物理缓冲,必须使快照失效。
        vaos.rebuild_vao(&dev, &meshes, h).unwrap();
        let r2 = vaos.revision();
        assert_ne!(r1, r2);
        // 删除不存在的 mesh 不改表;删除存在的才改。
        vaos.remove(&dev, Handle::<Mesh>::new());
        assert_eq!(vaos.revision(), r2);
        vaos.remove(&dev, h);
        assert_ne!(vaos.revision(), r2);
        // 另一个实例的修订号不会与本实例撞上。
        assert_ne!(AssetVAOLibrary::new().revision(), vaos.revision());
    }

    #[test]
    fn cloned_publications_share_metadata_and_diverging_rebuilds_get_unique_revisions() {
        let (meshes, mesh) = setup();
        let device = MockDevice::default();
        let mut original = AssetVAOLibrary::new();
        let handle = original.add(&device, &meshes, mesh).unwrap();
        let old = original.shared_by_handle(handle).unwrap().clone();
        let mut branch = original.clone();
        assert!(Arc::ptr_eq(&old, branch.shared_by_handle(handle).unwrap()));
        original.rebuild_vao(&device, &meshes, mesh).unwrap();
        branch.rebuild_vao(&device, &meshes, mesh).unwrap();
        assert_ne!(original.revision(), branch.revision());
        assert!(!Arc::ptr_eq(&old, original.shared_by_handle(handle).unwrap()));
        assert!(!Arc::ptr_eq(original.shared_by_handle(handle).unwrap(), branch.shared_by_handle(handle).unwrap()));
        assert_eq!(old.index_count, 6, "old metadata remains an immutable publication");
    }

    #[test]
    fn incremental_snapshot_rebuilds_and_removes_exact_publications() {
        let (mut meshes, a) = setup();
        let b = meshes.add(MeshAsset::new(quad(), None));
        let device = MockDevice::default();
        let mut source = AssetVAOLibrary::new();
        let ah = source.add(&device, &meshes, a).unwrap();
        let bh = source.add(&device, &meshes, b).unwrap();
        let mut render = source.clone();
        let old_a = render.shared_by_handle(ah).unwrap().clone();
        let old_b = render.shared_by_handle(bh).unwrap().clone();
        meshes.get_mut(a).unwrap().indices.push(3);
        source.rebuild_vao(&device, &meshes, a).unwrap();
        source.update_render_snapshot(&mut render);
        assert_eq!(render.get(a).unwrap().index_count, 7);
        assert_eq!(old_a.index_count, 6);
        assert!(Arc::ptr_eq(&old_b, render.shared_by_handle(bh).unwrap()));
        source.remove(&device, a);
        source.update_render_snapshot(&mut render);
        assert!(!render.has_vao(a));
        assert!(render.get_by_handle(ah).is_none());
        let new_ah = source.add(&device, &meshes, a).unwrap();
        source.update_render_snapshot(&mut render);
        assert_ne!(new_ah, ah);
        assert!(render.get_by_handle(new_ah).is_some());
        assert!(render.get_by_handle(ah).is_none());
        AssetVAOLibrary::new().update_render_snapshot(&mut render);
        assert!(render.is_empty());
    }

    #[test]
    fn new_is_empty() {
        let l = AssetVAOLibrary::new();
        assert!(l.vaos.is_empty() && l.mesh_to_vao.is_empty());
        assert!(!l.has_vao(Handle::<Mesh>::new()));
    }

    #[test]
    fn add_builds_vao_and_records_index_count() {
        let (meshes, h) = setup();
        let dev = MockDevice::default();
        let mut vaos = AssetVAOLibrary::new();
        let vh = vaos.add(&dev, &meshes, h).unwrap();
        assert!(vaos.has_vao(h));
        let e = vaos.get(h).unwrap();
        assert_eq!(e.index_count, 6);
        assert_eq!(e.vao.bindings.len(), 1);
        assert_eq!(vaos.mesh_to_vao[&h], vh);
        // 索引缓冲 + 1 个顶点缓冲
        assert_eq!(dev.created.borrow().len(), 2);
    }

    #[test]
    fn add_existing_mesh_returns_same_handle_without_building() {
        let (meshes, h) = setup();
        let dev = MockDevice::default();
        let mut vaos = AssetVAOLibrary::new();
        let a = vaos.add(&dev, &meshes, h).unwrap();
        let n = dev.created.borrow().len();
        let b = vaos.add(&dev, &meshes, h).unwrap();
        assert_eq!(a, b);
        assert_eq!(dev.created.borrow().len(), n);
    }

    #[test]
    fn add_missing_mesh_errors_with_double_space_text_and_burns_a_handle() {
        let meshes = AssetMeshLibrary::new();
        let dev = MockDevice::default();
        let mut vaos = AssetVAOLibrary::new();
        let before = Handle::<u8>::new().id().raw();
        let err = vaos.add(&dev, &meshes, Handle::<Mesh>::new()).unwrap_err();
        assert_eq!(err.to_string(), "Mesh is not in the  mesh library");
        let after = Handle::<u8>::new().id().raw();
        // 中间依次消耗了：Handle::<Mesh>::new()、VAO handle，再加 `after` 自己 → 至少隔 3
        assert!(after - before >= 3, "{before} {after}");
        assert!(vaos.vaos.is_empty());
    }

    #[test]
    fn add_mesh_without_indices_is_invalid() {
        let meshes = AssetMeshLibrary::new();
        let h = meshes.add(MeshAsset::new(Mesh::new("empty"), None));
        let dev = MockDevice::default();
        let mut vaos = AssetVAOLibrary::new();
        let err = vaos.add(&dev, &meshes, h).unwrap_err();
        assert_eq!(err.to_string(), "Mesh is invalid");
        assert!(!vaos.has_vao(h));
        assert!(dev.created.borrow().is_empty());
    }

    #[test]
    fn rebuild_replaces_vao_and_updates_index_count() {
        let (mut meshes, h) = setup();
        let dev = MockDevice::default();
        let mut vaos = AssetVAOLibrary::new();
        vaos.add(&dev, &meshes, h).unwrap();
        meshes.get_mut(h).unwrap().indices.push(3);
        let n = dev.created.borrow().len();
        vaos.rebuild_vao(&dev, &meshes, h).unwrap();
        assert_eq!(vaos.get(h).unwrap().index_count, 7);
        assert_eq!(dev.created.borrow().len(), n + 2);
        assert_eq!(dev.created.borrow()[n].0.size, 28, "新索引缓冲 = 7*4");
    }

    #[test]
    fn rebuild_with_empty_indices_removes_old_vao() {
        let (mut meshes, h) = setup();
        let dev = MockDevice::default();
        let mut vaos = AssetVAOLibrary::new();
        vaos.add(&dev, &meshes, h).unwrap();
        meshes.get_mut(h).unwrap().indices.clear();
        let n = dev.created.borrow().len();
        assert!(vaos.rebuild_vao(&dev, &meshes, h).is_ok());
        assert_eq!(dev.created.borrow().len(), n);
        assert!(vaos.get(h).is_none());
    }

    #[test]
    fn rebuild_without_registered_vao_errors() {
        let (meshes, h) = setup();
        let dev = MockDevice::default();
        let mut vaos = AssetVAOLibrary::new();
        let err = vaos.rebuild_vao(&dev, &meshes, h).unwrap_err();
        assert_eq!(err.to_string(), "The requested mesh doesn't exist");
    }

    #[test]
    fn rebuild_with_dangling_vao_handle_errors() {
        let (meshes, h) = setup();
        let dev = MockDevice::default();
        let mut vaos = AssetVAOLibrary::new();
        let vh = vaos.add(&dev, &meshes, h).unwrap();
        vaos.vaos.remove(&vh);
        let err = vaos.rebuild_vao(&dev, &meshes, h).unwrap_err();
        assert_eq!(err.to_string(), "VAO handle is invalid");
    }

    #[test]
    fn rebuild_missing_mesh_returns_error_without_allocating() {
        let meshes = AssetMeshLibrary::new();
        let dev = MockDevice::default();
        let mut vaos = AssetVAOLibrary::new();
        let error = vaos
            .rebuild_vao(&dev, &meshes, Handle::<Mesh>::new())
            .unwrap_err();
        assert_eq!(error.to_string(), "Mesh is not in the mesh library");
        assert!(dev.created.borrow().is_empty());
    }

    #[test]
    fn remove_clears_both_tables() {
        let (meshes, h) = setup();
        let dev = MockDevice::default();
        let mut vaos = AssetVAOLibrary::new();
        vaos.add(&dev, &meshes, h).unwrap();
        vaos.remove(&dev, h);
        assert!(!vaos.has_vao(h));
        assert!(vaos.vaos.is_empty());
        vaos.remove(&dev, h); // 重复删除无副作用
    }

    #[test]
    fn layout_constants_match_original() {
        // 桶：键 8 + Vao 0x40 + usize 8 = 0x50（O：stride 0x50）。
        assert_eq!(core::mem::size_of::<(Handle<Vao>, VaoEntry)>(), 0x50);
        assert_eq!(core::mem::size_of::<(Handle<Mesh>, Handle<Vao>)>(), 16);
    }
}
