//! `AssetVAOLibrary` — 网格 → VAO 注册表（layer A3，逐函数对照原版机器码重写）。
//!
//! 证据全文：`evidence/country-core-audit/A3_mesh.md`。标记：O＝机器码直接观察，I＝推断，U＝未知。
//!
//! # 布局（O：`new` 0x140924030，总大小 0x60）
//! 两张 hashbrown 表，各自带一个 `RandomState`（密钥取自线程局部 `KEYS`）：
//! * +0x00 `vaos: HashMap<Handle<Vao>, VaoEntry>` —— 桶 0x50 B：键@0、`Vao`(0x40，`bindings`@+8、`index_buffer`@+0x20)、
//!   `index_count: usize`@+0x48（`add`/`rebuild_vao` 写入 `mesh.indices.len()`）。
//! * +0x30 `mesh_to_vao: HashMap<Handle<Mesh>, Handle<Vao>>` —— 桶 16 B。items@+0x48、`RandomState`@+0x50。
//!
//! # 旧版错误（已修）
//! * 旧版是单张 `HashMap<u64, Vao>`，用 `Vao { bindings, _gap }` 占位；原版是**两张表**（网格 handle → VAO handle →
//!   `(Vao, 索引数)`），VAO 有自己的 handle（与网格共用全局 `HANDLE_COUNT`，0x143634cd0）。
//! * 旧 `add(mesh_handle)` 不取设备/网格库，没有校验；原版 `add(device, mesh_lib, mesh_handle)` 返回
//!   `Result<Handle<Vao>, anyhow::Error>`，网格缺失 / 无索引都是 `Err`。
//! * 旧 `rebuild_vao` 是空函数；原版会重建缓冲并替换（先 drop 旧 `bindings` 与索引缓冲）。
//! * 旧 `remove` 只删一张表；原版同时从两张表删除并 drop `Vao`。

use std::collections::HashMap;

use anyhow::anyhow;
use bevy_ecs::resource::Resource;

use crate::asset_libraries::Handle;
use crate::asset_libraries::mesh_library::AssetMeshLibrary;
use crate::render::mesh::Mesh;
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::render::vao::{Vao, build_vao};

/// `vaos` 表的值（O：0x48 B = `Vao` + 索引数）。字段名 I，偏移与取值 O。
pub struct VaoEntry {
    pub vao: Vao,
    /// 构建时的 `mesh.indices.len()`（O：`add` 写 `[rcx+0x10]`，`rebuild_vao` 写 `[rsi+0x10]`）。
    pub index_count: usize,
}

/// VAO 注册表（O：`Res<AssetVAOLibrary>`）。
#[derive(Resource)]
pub struct AssetVAOLibrary {
    /// +0x00（O）。
    vaos: HashMap<Handle<Vao>, VaoEntry>,
    /// +0x30（O）。
    mesh_to_vao: HashMap<Handle<Mesh>, Handle<Vao>>,
}

impl AssetVAOLibrary {
    /// `new`（O 0x140924030）：两张空表。
    pub fn new() -> Self {
        AssetVAOLibrary {
            vaos: HashMap::new(),
            mesh_to_vao: HashMap::new(),
        }
    }

    /// `has_vao`：`vao_update::rebuild_vaos` / `build_missing_vaos` 内联的 `mesh_to_vao` 查询（O：`items != 0` 短路后哈希查找）。
    pub fn has_vao(&self, mesh: Handle<Mesh>) -> bool {
        self.mesh_to_vao.contains_key(&mesh)
    }

    /// 查询某网格当前的 VAO 条目（I：两张表的组合读取；原版调用点内联）。
    pub fn get(&self, mesh: Handle<Mesh>) -> Option<&VaoEntry> {
        self.vaos.get(self.mesh_to_vao.get(&mesh)?)
    }

    /// `add`（O 0x140924180）。
    /// 1. `mesh_to_vao` 非空且已含该网格 → 直接 `Ok(已有 vao handle)`（不做任何校验、不分配 handle）；
    /// 2. `HANDLE_COUNT.fetch_add(1)` 分配 VAO handle——**早于**后面的校验，所以失败也消耗一个 id（O）；
    /// 3. `mesh_lib.get(mesh)` 缺失 → `Err("Mesh is not in the  mesh library")`（O：**两个空格**）；
    /// 4. `indices` 为空 → `Err("Mesh is invalid")`；
    /// 5. `build_vao(device, mesh)`；（原版另有 `bindings.cap == i64::MIN` 的 `Err` 透传分支：build_vao 内部全是 `unwrap`，
    ///    该分支不可达，I）
    /// 6. `vaos.insert(vao_handle, (vao, indices.len()))`（旧值 drop），`mesh_to_vao.insert(mesh, vao_handle)`，`Ok(vao_handle)`。
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
        let vao = build_vao(device, mesh_ref);
        let index_count = mesh_ref.indices.len();
        drop(self.vaos.insert(vao_handle, VaoEntry { vao, index_count }));
        self.mesh_to_vao.insert(mesh, vao_handle);
        Ok(vao_handle)
    }

    /// `rebuild_vao`（O 0x1409243f0；原版返回 `Option<anyhow::Error>`，这里用 `Result<(), _>`）。
    /// 1. `mesh_lib.get(mesh).expect("Mesh is not in the mesh library")`（vao_library.rs 第 84 行，**单**空格，panic）；
    /// 2. `indices` 为空 → `Ok(())`（静默跳过，旧 VAO 保留）；
    /// 3. `mesh_to_vao` 为空或查不到 → `Err("The requested mesh doesn't exist")`；
    /// 4. `vaos` 为空或查不到 → `Err("VAO handle is invalid")`；
    /// 5. `build_vao` 后替换：先 drop 旧 `bindings`，再 drop 旧索引缓冲并写入新值，`index_count = indices.len()`。
    pub fn rebuild_vao(
        &mut self,
        device: &dyn GraphicsDevice,
        mesh_lib: &AssetMeshLibrary,
        mesh: Handle<Mesh>,
    ) -> Result<(), anyhow::Error> {
        let mesh_ref = mesh_lib.get(mesh).expect("Mesh is not in the mesh library");
        if mesh_ref.indices.is_empty() {
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
        let new_vao = build_vao(device, mesh_ref);
        entry.vao = new_vao;
        entry.index_count = mesh_ref.indices.len();
        Ok(())
    }

    /// `remove`（O 0x1409247a0）：从 `mesh_to_vao` 取出该网格的条目；若有，再按 vao handle 从 `vaos` 删除并 drop。
    pub fn remove(&mut self, mesh: Handle<Mesh>) {
        if let Some(vao_handle) = self.mesh_to_vao.remove(&mesh) {
            drop(self.vaos.remove(&vao_handle));
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
    fn rebuild_with_empty_indices_is_silent_ok_and_keeps_old_vao() {
        let (mut meshes, h) = setup();
        let dev = MockDevice::default();
        let mut vaos = AssetVAOLibrary::new();
        vaos.add(&dev, &meshes, h).unwrap();
        meshes.get_mut(h).unwrap().indices.clear();
        let n = dev.created.borrow().len();
        assert!(vaos.rebuild_vao(&dev, &meshes, h).is_ok());
        assert_eq!(dev.created.borrow().len(), n);
        assert_eq!(vaos.get(h).unwrap().index_count, 6);
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
    #[should_panic(expected = "Mesh is not in the mesh library")]
    fn rebuild_missing_mesh_panics_via_expect() {
        let meshes = AssetMeshLibrary::new();
        let dev = MockDevice::default();
        let mut vaos = AssetVAOLibrary::new();
        let _ = vaos.rebuild_vao(&dev, &meshes, Handle::<Mesh>::new());
    }

    #[test]
    fn remove_clears_both_tables() {
        let (meshes, h) = setup();
        let dev = MockDevice::default();
        let mut vaos = AssetVAOLibrary::new();
        vaos.add(&dev, &meshes, h).unwrap();
        vaos.remove(h);
        assert!(!vaos.has_vao(h));
        assert!(vaos.vaos.is_empty());
        vaos.remove(h); // 重复删除无副作用
    }

    #[test]
    fn layout_constants_match_original() {
        // 桶：键 8 + Vao 0x40 + usize 8 = 0x50（O：stride 0x50）。
        assert_eq!(core::mem::size_of::<(Handle<Vao>, VaoEntry)>(), 0x50);
        assert_eq!(core::mem::size_of::<(Handle<Mesh>, Handle<Vao>)>(), 16);
    }
}
