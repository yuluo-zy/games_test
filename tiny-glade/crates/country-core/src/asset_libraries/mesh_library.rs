//! CPU 网格注册表。名字和源路径定位资产，可变访问登记后续 VAO 更新。
//! 原作来源见项目 evidence；当前接口、行为变化与验证范围见 README.md。

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use bevy_ecs::resource::Resource;
use parking_lot::RwLock;

use crate::asset_libraries::Handle;
use crate::frozen_map::FrozenMap;
use crate::render::mesh::Mesh;

/// 待注册网格及其可选名字。
pub struct MeshAsset {
    pub asset: Mesh,
    pub name: Option<String>,
}

impl MeshAsset {
    pub fn new(asset: Mesh, name: Option<String>) -> Self {
        MeshAsset { asset, name }
    }
}

/// 拥有 CPU 网格及其名字、源路径索引。
#[derive(Resource)]
pub struct AssetMeshLibrary {
    pub modified_meshes: Vec<Handle<Mesh>>,
    /// CPU 删除先登记身份，维护阶段再通过创建设备释放派生 GPU 资源。
    removed_meshes: Vec<Handle<Mesh>>,
    transient_meshes: Arc<Mutex<Vec<Handle<Mesh>>>>,
    pub mesh_entries: FrozenMap<Handle<Mesh>, Mesh>,
    names: RwLock<HashMap<String, Handle<Mesh>>>,
    paths: RwLock<HashMap<PathBuf, Handle<Mesh>>>,
}

impl AssetMeshLibrary {
    /// 创建初始状态。
    pub fn new() -> Self {
        AssetMeshLibrary {
            modified_meshes: Vec::new(),
            removed_meshes: Vec::new(),
            transient_meshes: Arc::new(Mutex::new(Vec::new())),
            mesh_entries: FrozenMap::new(),
            names: RwLock::new(HashMap::new()),
            paths: RwLock::new(HashMap::new()),
        }
    }

    pub fn transient_meshes(&self) -> &Arc<Mutex<Vec<Handle<Mesh>>>> {
        &self.transient_meshes
    }

    /// 注册资产并返回稳定身份。
    pub fn add(&self, asset: MeshAsset) -> Handle<Mesh> {
        let mesh_handle = Handle::<Mesh>::new();
        let source_path = asset
            .asset
            .source_file
            .as_ref()
            .map(|file| PathBuf::from(&file.path));
        assert!(
            self.mesh_entries
                .insert(mesh_handle, Box::new(asset.asset))
                .is_ok(),
            "assertion failed: self.mesh_entries.insert(mesh_handle, Box::new(asset.asset)).is_ok()"
        );
        if let Some(name) = asset.name {
            self.names.write().insert(name, mesh_handle);
        }
        if let Some(path) = source_path {
            self.paths.write().insert(path, mesh_handle);
        }
        mesh_handle
    }

    pub fn get(&self, handle: Handle<Mesh>) -> Option<&Mesh> {
        self.mesh_entries.get(&handle)
    }

    /// 取得可变资产并登记派生资源更新。
    pub fn get_mut(&mut self, handle: Handle<Mesh>) -> Option<&mut Mesh> {
        let mesh = self.mesh_entries.get_mut(&handle)?;
        self.modified_meshes.push(handle);
        Some(mesh)
    }

    /// 按业务名字读取已注册资产。
    pub fn get_by_name(&self, name: &str) -> Option<&Mesh> {
        match self.try_get_handle_by_name(name) {
            Some(handle) => self.get(handle),
            None => Self::missing_name(name),
        }
    }

    /// 按业务名字取得资产句柄。
    pub fn get_handle_by_name(&self, name: &str) -> Handle<Mesh> {
        match self.try_get_handle_by_name(name) {
            Some(handle) => handle,
            None => Self::missing_name(name),
        }
    }

    fn missing_name(name: &str) -> ! {
        panic!("Mesh {:?} does not exist in the library", name)
    }

    pub fn try_get_handle_by_name(&self, name: &str) -> Option<Handle<Mesh>> {
        let names = self.names.read();
        if names.is_empty() {
            return None;
        }
        names.get(name).copied()
    }

    /// 移除资产及关联索引。
    pub fn remove(&mut self, handle: Handle<Mesh>) {
        if self.mesh_entries.remove(&handle).is_some() {
            self.removed_meshes.push(handle);
        }
        self.names
            .write()
            .retain(|_, registered| *registered != handle);
        self.paths
            .write()
            .retain(|_, registered| *registered != handle);
        self.modified_meshes.retain(|modified| *modified != handle);
    }

    /// 仅由有设备的维护阶段消费；没有宿主时保留通知，避免遗留孤立 VAO。
    pub(crate) fn drain_removed_meshes(&mut self) -> impl Iterator<Item = Handle<Mesh>> + '_ {
        self.removed_meshes.drain(..)
    }

    /// 重新读取源文件；失败时保留旧网格。
    pub fn reload_mesh_src(&mut self, path: &str) {
        self.reload_mesh_src_with(path, crate::utils::load_json::try_load_json_as_mesh);
    }

    /// 同 [`Self::reload_mesh_src`]，加载器由调用方注入（测试用）。
    pub fn reload_mesh_src_with(
        &mut self,
        path: &str,
        load: impl FnOnce(&str) -> anyhow::Result<Mesh>,
    ) {
        let source_path = Path::new(path);
        let handle = {
            let paths = self.paths.read();
            let found = if paths.is_empty() {
                None
            } else {
                paths.get(source_path).copied()
            };
            match found {
                Some(handle) => handle,
                None => panic!(
                    "Asset Mesh library was requested to reload a non-existing mesh path {:?}",
                    source_path
                ),
            }
        };
        let extension = source_path.extension().and_then(|e| e.to_str());
        assert_eq!(extension, Some("json"));
        match load(path) {
            Ok(mesh) => *self.get_mut(handle).expect("源路径映射到不存在的网格") = mesh,
            Err(error) => log::error!("网格重载失败，保留旧资产：{}", error),
        }
    }
    /// 热重载使用可恢复错误；加载成功后才替换并登记修改。
    pub fn try_reload_mesh_src(&mut self, path: &Path) -> anyhow::Result<()> {
        let handle = self
            .paths
            .read()
            .get(path)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("未注册网格源文件：{}", path.display()))?;
        anyhow::ensure!(
            path.extension().and_then(|extension| extension.to_str()) == Some("json"),
            "网格源必须为 JSON"
        );
        let path_text = path
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("网格源路径不是 UTF-8"))?;
        let mesh = crate::utils::load_json::try_load_json_as_mesh(path_text)?;
        *self
            .get_mut(handle)
            .ok_or_else(|| anyhow::anyhow!("网格已经删除"))? = mesh;
        Ok(())
    }
}

impl Default for AssetMeshLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::mesh::SourceFile;

    fn mesh(label: &'static str) -> Mesh {
        let mut m = Mesh::new(label);
        m.indices = vec![0, 1, 2];
        m
    }

    fn mesh_with_source(label: &'static str, path: &str) -> Mesh {
        let mut m = mesh(label);
        m.set_source_file(SourceFile {
            path: path.to_string(),
            stamp: 0,
        });
        m
    }

    fn named(lib: &AssetMeshLibrary, name: &str, m: Mesh) -> Handle<Mesh> {
        lib.add(MeshAsset::new(m, Some(name.to_string())))
    }

    #[test]
    fn new_library_is_empty() {
        let lib = AssetMeshLibrary::new();
        assert!(lib.modified_meshes.is_empty());
        assert_eq!(lib.mesh_entries.len(), 0);
        assert!(lib.transient_meshes().lock().unwrap().is_empty());
        assert_eq!(lib.try_get_handle_by_name("x"), None);
    }

    #[test]
    fn add_allocates_distinct_increasing_handles_and_registers_name() {
        let lib = AssetMeshLibrary::new();
        let a = named(&lib, "a", mesh("a"));
        let b = named(&lib, "b", mesh("b"));
        assert!(b.id().raw() > a.id().raw());
        assert_eq!(lib.try_get_handle_by_name("a"), Some(a));
        assert_eq!(lib.get(b).unwrap().label, "b");
        assert_eq!(lib.get_by_name("a").unwrap().label, "a");
        assert_eq!(lib.get_handle_by_name("b"), b);
    }

    #[test]
    fn add_without_name_registers_no_name() {
        let lib = AssetMeshLibrary::new();
        let h = lib.add(MeshAsset::new(mesh("anon"), None));
        assert!(lib.get(h).is_some());
        assert!(lib.names.read().is_empty());
    }

    #[test]
    fn add_same_name_overwrites_previous_mapping() {
        let lib = AssetMeshLibrary::new();
        let a = named(&lib, "dup", mesh("first"));
        let b = named(&lib, "dup", mesh("second"));
        assert_ne!(a, b);
        assert_eq!(lib.get_handle_by_name("dup"), b);
        assert!(lib.get(a).is_some(), "旧网格仍在 mesh_entries 里");
    }

    #[test]
    fn add_registers_source_path_only_when_present() {
        let lib = AssetMeshLibrary::new();
        let with = lib.add(MeshAsset::new(mesh_with_source("m", "meshes/a.json"), None));
        let _without = lib.add(MeshAsset::new(mesh("n"), None));
        let paths = lib.paths.read();
        assert_eq!(paths.len(), 1);
        assert_eq!(paths.get(Path::new("meshes/a.json")), Some(&with));
    }

    #[test]
    fn shared_handle_counter_is_monotonic_across_types() {
        // O：mesh 与 VAO 共用 HANDLE_COUNT。
        let lib = AssetMeshLibrary::new();
        let h1 = named(&lib, "x", mesh("x"));
        let v = Handle::<u8>::new();
        let h2 = named(&lib, "y", mesh("y"));
        assert!(h1.id().raw() < v.id().raw() && v.id().raw() < h2.id().raw());
    }

    #[test]
    fn get_unknown_handle_is_none() {
        let lib = AssetMeshLibrary::new();
        assert!(lib.get(Handle::<Mesh>::new()).is_none());
    }

    #[test]
    fn get_mut_pushes_handle_before_lookup_even_when_missing() {
        let mut lib = AssetMeshLibrary::new();
        let h = named(&lib, "m", mesh("m"));
        let ghost = Handle::<Mesh>::new();
        assert!(lib.get_mut(ghost).is_none());
        lib.get_mut(h).unwrap().label = "changed";
        lib.get_mut(h).unwrap();
        assert_eq!(lib.modified_meshes, vec![h, h]);
        assert_eq!(lib.get(h).unwrap().label, "changed");
    }

    #[test]
    #[should_panic(expected = "Mesh \"nope\" does not exist in the library")]
    fn get_handle_by_name_panics_with_debug_quoted_name() {
        AssetMeshLibrary::new().get_handle_by_name("nope");
    }

    #[test]
    #[should_panic(expected = "Mesh \"nope\" does not exist in the library")]
    fn get_by_name_panics_when_name_missing() {
        let lib = AssetMeshLibrary::new();
        named(&lib, "other", mesh("o"));
        lib.get_by_name("nope");
    }

    #[test]
    fn get_by_name_returns_none_when_mesh_removed_but_name_kept() {
        // 名字被 retain 清掉，所以这里用另一条路径构造：直接把名字指向不存在的 handle。
        let lib = AssetMeshLibrary::new();
        let ghost = Handle::<Mesh>::new();
        lib.names.write().insert("ghost".into(), ghost);
        assert!(lib.get_by_name("ghost").is_none());
    }

    #[test]
    fn remove_drops_mesh_and_clears_names_and_source_path() {
        let mut lib = AssetMeshLibrary::new();
        let a = lib.add(MeshAsset::new(
            mesh_with_source("a", "a.json"),
            Some("a1".into()),
        ));
        lib.names.write().insert("a2".into(), a);
        let b = named(&lib, "b", mesh("b"));
        lib.remove(a);
        assert!(lib.get(a).is_none());
        assert_eq!(lib.try_get_handle_by_name("a1"), None);
        assert_eq!(lib.try_get_handle_by_name("a2"), None);
        assert_eq!(lib.try_get_handle_by_name("b"), Some(b));
        assert!(lib.paths.read().is_empty());
    }

    #[test]
    fn remove_unknown_handle_is_noop() {
        let mut lib = AssetMeshLibrary::new();
        let b = named(&lib, "b", mesh("b"));
        lib.remove(Handle::<Mesh>::new());
        assert!(lib.get(b).is_some());
    }

    #[test]
    fn reload_replaces_mesh_in_place_and_marks_modified() {
        let mut lib = AssetMeshLibrary::new();
        let h = lib.add(MeshAsset::new(mesh_with_source("old", "m/a.json"), None));
        lib.reload_mesh_src_with("m/a.json", |p| {
            assert_eq!(p, "m/a.json");
            Ok(mesh("new"))
        });
        assert_eq!(lib.get(h).unwrap().label, "new");
        assert_eq!(lib.modified_meshes, vec![h]);
    }

    #[test]
    fn reload_failure_keeps_old_mesh_without_marking_modified() {
        let mut lib = AssetMeshLibrary::new();
        let h = lib.add(MeshAsset::new(mesh_with_source("old", "m/a.json"), None));
        lib.reload_mesh_src_with("m/a.json", |_| Err(anyhow::anyhow!("bad json")));
        assert_eq!(lib.get(h).unwrap().label, "old");
        assert!(lib.modified_meshes.is_empty());
    }

    #[test]
    #[should_panic(
        expected = "Asset Mesh library was requested to reload a non-existing mesh path \"m/zzz.json\""
    )]
    fn reload_unknown_path_panics() {
        let mut lib = AssetMeshLibrary::new();
        lib.add(MeshAsset::new(mesh_with_source("old", "m/a.json"), None));
        lib.reload_mesh_src_with("m/zzz.json", |_| unreachable!());
    }

    #[test]
    #[should_panic(expected = "non-existing mesh path")]
    fn reload_with_empty_path_table_panics() {
        AssetMeshLibrary::new().reload_mesh_src_with("m/a.json", |_| unreachable!());
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn reload_non_json_extension_asserts_and_does_not_mark_modified() {
        let mut lib = AssetMeshLibrary::new();
        lib.add(MeshAsset::new(mesh_with_source("old", "m/a.obj"), None));
        lib.reload_mesh_src_with("m/a.obj", |_| unreachable!());
    }

    #[test]
    fn reload_non_json_does_not_touch_modified_list() {
        let mut lib = AssetMeshLibrary::new();
        lib.add(MeshAsset::new(mesh_with_source("old", "m/a.obj"), None));
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            lib.reload_mesh_src_with("m/a.obj", |_| unreachable!());
        }));
        assert!(r.is_err());
        assert!(lib.modified_meshes.is_empty(), "扩展名断言先于 get_mut");
    }

    #[test]
    fn reload_path_without_extension_asserts() {
        let mut lib = AssetMeshLibrary::new();
        lib.add(MeshAsset::new(mesh_with_source("old", "m/noext"), None));
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            lib.reload_mesh_src_with("m/noext", |_| unreachable!());
        }));
        assert!(r.is_err());
    }
}
