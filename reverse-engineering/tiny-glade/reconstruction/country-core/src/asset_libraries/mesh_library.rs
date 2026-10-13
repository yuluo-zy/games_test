//! `AssetMeshLibrary` — 网格注册表（layer A3，逐函数对照原版机器码重写）。
//!
//! 证据全文：`evidence/country-core-audit/A3_mesh.md`。标记：O＝机器码直接观察，I＝推断，U＝未知。
//!
//! # 布局（O：`new` 0x14092c920 的各字段写入，总大小 0xd0）
//! | 偏移 | 字段 | 说明 |
//! |---|---|---|
//! | +0x00 | `modified_meshes: Vec<Handle<Mesh>>` | `get_mut` 每次调用先 `push(handle)`；`vao_update::rebuild_vaos` 遍历后整体替换为“重建失败”的子集（O）。名字 I |
//! | +0x18 | `transient_meshes: Arc<Mutex<Vec<_>>>` | ArcInner 0x30 B：强/弱计数 1、std `Mutex`（futex 字节@+0x10、poison 字节@+0x11）、`Vec`@+0x18（O）。访问器 `transient_meshes()` = `lea rax,[rcx+0x18]`（0x140002450，与别的 ICF 合并）。元素类型 U |
//! | +0x20 | `mesh_entries: FrozenMap<Handle<Mesh>, Box<Mesh>>` | 0x40 B：`[+0x28]` “借用中”字节、hashbrown 表@+0x30（桶 16 B = 键 + `Box` 指针；Box 大小 0x78 = `Mesh`）、`RandomState`@+0x50（O）。panic 文案里的字段名 `self.mesh_entries`（O） |
//! | +0x60 | `names: RwLock<HashMap<String, Handle<Mesh>>>` | parking_lot 锁字@+0x60、表@+0x68、items@+0x80、`RandomState`@+0x88；桶 0x20 B = `String`(0x18)+handle（O） |
//! | +0x98 | `paths: RwLock<HashMap<PathBuf, Handle<Mesh>>>` | 同上，桶 0x28 B = Windows `PathBuf`(0x20，含 `is_known_utf8` 字节)+handle（O） |
//!
//! `Handle<Mesh>` 的 id 来自全局原子计数器 `HANDLE_COUNT`（0x143634cd0，`lock xadd`，与 `AssetVAOLibrary::add`
//! 共用同一个计数器，O），即 `Handle::new()`。
//!
//! # 旧版错误（已修）
//! * `meshes: HashMap` 不是 `FrozenMap`；`remove`/`reload_mesh_src` 里的 `disposed` 标志不存在——那个字节是 FrozenMap 的
//!   “借用中”标志（命中则 `Result::unwrap` panic，来自 `frozen/src/lib.rs`）。
//! * `add(name, mesh)` 的签名错：原函数只取一个 0x90 B 的 `{ asset: Mesh, name: Option<String> }`；名字是**可选**的，
//!   插入失败走 `assert!(self.mesh_entries.insert(..).is_ok())`；源文件路径取自 `Mesh.source_file`（旧版 `mesh_source_file`
//!   恒返回 `None`，路径表永远是空的）。
//! * `get_mut` 会把 handle 压入 `modified_meshes`（旧版没有，VAO 重建列表因此永远为空）。
//! * `get_by_name` 返回 `Option<&Mesh>`（不 unwrap），缺名字时 panic 文案是 `Mesh {:?} does not exist in the library`，
//!   旧版是 `mesh {:?} not found`。
//! * `remove` 只清名字表（`retain(|_, h| *h != handle)`），**不**清路径表；旧版同样只清名字表但多了假的 disposed 断言。
//! * `reload_mesh_src` 的失败路径：路径表里查不到 → panic（文案见下）；扩展名断言是 `assert_eq!(ext, Some("json"))`；
//!   `get_mut`（压入 modified）发生在加载**之前**；加载失败只 `log::error!`，保留旧网格。旧版把失败静默吞掉并先检查扩展名。

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use bevy_ecs::resource::Resource;
use parking_lot::RwLock;

use crate::asset_libraries::Handle;
use crate::frozen_map::FrozenMap;
use crate::render::mesh::Mesh;

/// `add` 的唯一入参（O：0x90 B，`Mesh`@0、`Option<String>`@0x78；panic 文案里的 `asset.asset`）。
pub struct MeshAsset {
    pub asset: Mesh,
    /// `None` 时不登记名字（O：`cap == i64::MIN` 的 niche 判断，跳过 `names.insert`）。
    pub name: Option<String>,
}

impl MeshAsset {
    pub fn new(asset: Mesh, name: Option<String>) -> Self {
        MeshAsset { asset, name }
    }
}

/// 网格注册表。
#[derive(Resource)]
pub struct AssetMeshLibrary {
    /// +0x00（O）。`get_mut` 压入，`rebuild_vaos` 消费。
    pub modified_meshes: Vec<Handle<Mesh>>,
    /// +0x18（O 布局 / U 元素类型）。
    transient_meshes: Arc<Mutex<Vec<Handle<Mesh>>>>,
    /// +0x20（O）。
    pub mesh_entries: FrozenMap<Handle<Mesh>, Mesh>,
    /// +0x60（O）。
    names: RwLock<HashMap<String, Handle<Mesh>>>,
    /// +0x98（O）。
    paths: RwLock<HashMap<PathBuf, Handle<Mesh>>>,
}

impl AssetMeshLibrary {
    /// `new`（O 0x14092c920）：五个字段全为空表，各 `RandomState` 取线程局部随机密钥。
    pub fn new() -> Self {
        AssetMeshLibrary {
            modified_meshes: Vec::new(),
            transient_meshes: Arc::new(Mutex::new(Vec::new())),
            mesh_entries: FrozenMap::new(),
            names: RwLock::new(HashMap::new()),
            paths: RwLock::new(HashMap::new()),
        }
    }

    /// `transient_meshes`（O 0x140002450：`lea rax,[rcx+0x18]`）。
    pub fn transient_meshes(&self) -> &Arc<Mutex<Vec<Handle<Mesh>>>> {
        &self.transient_meshes
    }

    /// `add`（O 0x14092cc60）。顺序：
    /// 1. `HANDLE_COUNT.fetch_add(1)` 取新 handle；
    /// 2. 若 `source_file` 为 `Some`，克隆其路径（在装箱之前）；
    /// 3. `Box::new(asset.asset)`（0x78 B）→ `assert!(self.mesh_entries.insert(mesh_handle, Box::new(asset.asset)).is_ok())`；
    /// 4. `name` 为 `Some` → 写锁 `names.insert(name, handle)`（同名覆盖，旧值丢弃）；
    /// 5. 路径克隆为 `Some` → 写锁 `paths.insert(path, handle)`。
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

    /// `get`（O 0x14092d2e0）：就是 `FrozenMap::get`。
    pub fn get(&self, handle: Handle<Mesh>) -> Option<&Mesh> {
        self.mesh_entries.get(&handle)
    }

    /// `get_mut`（O 0x14092d300）：**先**把 handle 压入 `modified_meshes`（无条件，哪怕 handle 不存在），
    /// 再查表。原版对 FrozenMap 的“借用中”字节做 `unwrap`（`&mut self` 下不会触发，占位不建模）。
    pub fn get_mut(&mut self, handle: Handle<Mesh>) -> Option<&mut Mesh> {
        self.modified_meshes.push(handle);
        self.mesh_entries.get_mut(&handle)
    }

    /// `get_by_name`（O 0x14092d450）：`get_handle_by_name` 失败 panic；之后 `FrozenMap::get` 的结果**原样返回**
    /// （`Option<&Mesh>`，没有 unwrap）。panic 文案 `Mesh {:?} does not exist in the library`（`{:?}` 因为
    /// 实参格式化函数是 `str` 的 Debug：0x1405338a0 → `core::fmt::impl$19`，900 B 的转义实现）。
    pub fn get_by_name(&self, name: &str) -> Option<&Mesh> {
        match self.try_get_handle_by_name(name) {
            Some(handle) => self.get(handle),
            None => Self::missing_name(name),
        }
    }

    /// `get_handle_by_name`（O 0x14092d4f0）。
    pub fn get_handle_by_name(&self, name: &str) -> Handle<Mesh> {
        match self.try_get_handle_by_name(name) {
            Some(handle) => handle,
            None => Self::missing_name(name),
        }
    }

    fn missing_name(name: &str) -> ! {
        panic!("Mesh {:?} does not exist in the library", name)
    }

    /// `try_get_handle_by_name`（O 0x14092d570）：读锁；表空时直接 `None`（不算哈希）。
    pub fn try_get_handle_by_name(&self, name: &str) -> Option<Handle<Mesh>> {
        let names = self.names.read();
        if names.is_empty() {
            return None;
        }
        names.get(name).copied()
    }

    /// `remove`（O 0x14092d750）：FrozenMap 表上 `remove_entry`，命中则 drop `Mesh` 并释放 0x78 B 的 Box；
    /// 然后写锁 `names.retain(|_, h| *h != handle)`（O：`retain` 里 `[bucket-8] == handle` 才删）。
    /// 路径表**不**清理（O）。
    pub fn remove(&mut self, handle: Handle<Mesh>) {
        drop(self.mesh_entries.remove(&handle));
        self.names.write().retain(|_, h| *h != handle);
    }

    /// `reload_mesh_src`（O 0x14092d8d0）。入参是 `&str`（`Path::extension` 与 `load_json_as_mesh<&str>`
    /// 共用同一对 ptr/len）。加载器接 `utils_contract::load_json_as_mesh`（真实函数 `utils::load_json` 仍是 todo）。
    pub fn reload_mesh_src(&mut self, path: &str) {
        self.reload_mesh_src_with(path, |p| {
            crate::utils_contract::load_json_as_mesh(Path::new(p))
                .map_err(|()| anyhow::anyhow!("load_json_as_mesh failed for {p}"))
        });
    }

    /// `reload_mesh_src` 的真实逻辑，加载器可注入（测试用）。
    ///
    /// 1. 读锁 `paths`：表空或查不到 → `panic!("Asset Mesh library was requested to reload a non-existing mesh path {:?}", path)`
    ///    （`{:?}`：格式化函数 0x14002e8d0 尾调 `std::path::impl$69::fmt` = `Path` 的 Debug，10 B）；
    /// 2. `assert_eq!(path.extension().and_then(OsStr::to_str), Some("json"))`；
    /// 3. `self.get_mut(handle).unwrap()`（压入 `modified_meshes`，缺失则 `Option::unwrap` panic）；
    /// 4. `load_json_as_mesh(path)`：`Ok(m)` → drop 旧 `Mesh` 并就地覆盖；`Err(e)` →
    ///    `log::error!("Asset mesh library couldnt reload a mesh: {}", e)`（`{}` = anyhow `ErrorImpl::display`），旧网格保留。
    pub fn reload_mesh_src_with(
        &mut self,
        path: &str,
        load: impl FnOnce(&str) -> anyhow::Result<Mesh>,
    ) {
        let as_path = Path::new(path);
        let handle = {
            let paths = self.paths.read();
            let found = if paths.is_empty() { None } else { paths.get(as_path).copied() };
            match found {
                Some(handle) => handle,
                None => panic!(
                    "Asset Mesh library was requested to reload a non-existing mesh path {:?}",
                    as_path
                ),
            }
        };
        let extension = as_path.extension().and_then(|e| e.to_str());
        assert_eq!(extension, Some("json"));
        let slot = self.get_mut(handle).unwrap();
        match load(path) {
            Ok(mesh) => *slot = mesh,
            Err(error) => log::error!("Asset mesh library couldnt reload a mesh: {}", error),
        }
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
        m.set_source_file(SourceFile { path: path.to_string(), stamp: 0 });
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
        assert_eq!(lib.modified_meshes, vec![ghost, h, h]);
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
    fn remove_drops_mesh_and_every_name_pointing_at_it_but_keeps_path() {
        let mut lib = AssetMeshLibrary::new();
        let a = lib.add(MeshAsset::new(mesh_with_source("a", "a.json"), Some("a1".into())));
        lib.names.write().insert("a2".into(), a);
        let b = named(&lib, "b", mesh("b"));
        lib.remove(a);
        assert!(lib.get(a).is_none());
        assert_eq!(lib.try_get_handle_by_name("a1"), None);
        assert_eq!(lib.try_get_handle_by_name("a2"), None);
        assert_eq!(lib.try_get_handle_by_name("b"), Some(b));
        assert_eq!(lib.paths.read().len(), 1, "原版不清路径表");
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
    fn reload_failure_keeps_old_mesh_but_still_marks_modified() {
        let mut lib = AssetMeshLibrary::new();
        let h = lib.add(MeshAsset::new(mesh_with_source("old", "m/a.json"), None));
        lib.reload_mesh_src_with("m/a.json", |_| Err(anyhow::anyhow!("bad json")));
        assert_eq!(lib.get(h).unwrap().label, "old");
        assert_eq!(lib.modified_meshes, vec![h]);
    }

    #[test]
    #[should_panic(expected = "Asset Mesh library was requested to reload a non-existing mesh path \"m/zzz.json\"")]
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
