//! `resources/render/mesh_atlas_library.rs` -- mesh 图集库资源（A3 res_render 按机器码重写）。
//!
//! 证据（O = 机器码直接观察，I = 推断；逐函数见 `evidence/country-core-audit/A3_res_render.md`）：
//!  * O 布局 `MeshAtlasLibrary`（0x50 B，构造点 `resources::impl$1::insert_resources` 0x140940d80）：
//!    `world_subsets: HashMap<GpuWorldSubsetKind, WorldSubsetMeshAtlas, RandomState>` @0（桶 0xc8 B，
//!    RandomState @0x20）、`prefab_parts: HashMap<AtlassedMeshName, Vec<PrefabMeshAtlasPart>, nohash>` @0x30
//!    （桶 0x20 B：键 @0、Vec{cap,ptr,len} @8）。nohash 哈希器不可观察，这里用标准 HashMap。
//!  * O `WorldSubsetMeshAtlas`（0xc0 B）= `{ registered: Option<Handle<Mesh>> @0 (tag) / @8, atlas: MeshAtlas @0x10 }`。
//!  * O `PrefabMeshAtlasPart` = 8 B：`{ part_index: u32 @0, subset: GpuWorldSubsetKind(u8) @4 }`。
//!  * O 默认构造 = 对 `enum_iterator::all::<GpuWorldSubsetKind>()`（27 个）逐个插入默认
//!    `WorldSubsetMeshAtlas`（`from_iter` 0x141449580 + fold 0x141478760），prefab 表为空。
//!  * O `AtlassedMeshName(NameHash)`：Debug 经 `debug_tuple_field1_finish("AtlassedMeshName")`，内层是
//!    `NameHash` 的 Debug（`debug_struct("NameHash").field("hash", ..)`，0x1414599b0）。
//!    转换：`From<&str>` = `NameHash::compute(s)`（无前缀）；`From<PrefabName>` / `From<&PrefabName>` /
//!    `From<&PrefabNameRef>` = `NameHash::compute(format!("prefab:{name}"))`（0x140ae6430/6570/6660）。
//!  * O `MeshAtlasesModifiedCmd`：事件；其 drop 与 `HashSet<GpuWorldSubsetKind, RandomState>` 的 drop 为同一
//!    ICF 代码（0x14087bb00），故载荷是该集合（字段名 U）。
//!
//! 旧版错误：`MeshAtlasLibrary(pub Gap)` + `todo!()`；`get_by_name` 返回的是 `&[PrefabMeshAtlasPart]`
//! 而不是单个 `MeshAtlasPart`；`AtlassedMeshName` 是 `(u64)` 且 Debug 不对；`add` 的出错 panic 并不是
//! "invalid mesh: " 上下文，而是 "Error adding prefab {:?} to atlas of shader {:?}: {:#}"。

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;

use bevy_ecs::event::Event;
use bevy_ecs::resource::Resource;

use crate::asset_libraries::mesh_library::{AssetMeshLibrary, MeshAsset};
use crate::asset_libraries::Handle;
use crate::name_hash::NameHash;
use crate::render::mesh::Mesh;
use crate::render::nani::gpu_world::GpuWorldSubsetKind;
use crate::render::nani::mesh_atlas::MeshAtlas;

/// 外部 `asset::prefab::PrefabNameRef`（O：`&PrefabNameRef` 是 (ptr,len) 胖指针 => str 包装）。
#[repr(transparent)]
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct PrefabNameRef(str);

impl PrefabNameRef {
    pub fn new(s: &str) -> &Self {
        // SAFETY：`repr(transparent)` 的 str 包装。
        unsafe { &*(s as *const str as *const PrefabNameRef) }
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PrefabNameRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

/// 外部 `asset::prefab::PrefabName`（O：按值传入时由 `Arc<str>` 的强引用 -1 / `drop_slow` 释放）。
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PrefabName(pub Arc<str>);

impl PrefabName {
    pub fn new(s: &str) -> Self {
        PrefabName(Arc::from(s))
    }
    /// O：`PrefabName::as_name_ref` 0x141556130。
    pub fn as_name_ref(&self) -> &PrefabNameRef {
        PrefabNameRef::new(&self.0)
    }
}

/// O：`NameHash::compute(&format!("prefab:{}", name))`。
fn prefab_hash(name: &PrefabNameRef) -> NameHash {
    NameHash::compute(&format!("prefab:{}", name))
}

/// 图集化网格名（O：newtype over `NameHash`）。
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AtlassedMeshName(pub NameHash);

/// 复刻原版 `NameHash` 的 Debug：`NameHash { hash: N }`。
struct NameHashDebug(u64);
impl fmt::Debug for NameHashDebug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NameHash").field("hash", &self.0).finish()
    }
}

impl fmt::Debug for AtlassedMeshName {
    /// O 0x140a3d2e0。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AtlassedMeshName").field(&NameHashDebug((self.0).0)).finish()
    }
}

impl From<NameHash> for AtlassedMeshName {
    fn from(h: NameHash) -> Self {
        AtlassedMeshName(h)
    }
}
impl From<&str> for AtlassedMeshName {
    /// O：`add<&str>` 直接调 `NameHash::compute`，无 "prefab:" 前缀。
    fn from(s: &str) -> Self {
        AtlassedMeshName(NameHash::compute(s))
    }
}
impl From<PrefabName> for AtlassedMeshName {
    /// O impl$1 0x140ae6430。
    fn from(n: PrefabName) -> Self {
        AtlassedMeshName(prefab_hash(n.as_name_ref()))
    }
}
impl From<&PrefabName> for AtlassedMeshName {
    /// O impl$2 0x140ae6570。
    fn from(n: &PrefabName) -> Self {
        AtlassedMeshName(prefab_hash(n.as_name_ref()))
    }
}
impl From<&PrefabNameRef> for AtlassedMeshName {
    /// O impl$3 0x140ae6660。
    fn from(n: &PrefabNameRef) -> Self {
        AtlassedMeshName(prefab_hash(n))
    }
}

/// 一个 prefab 在某个 subset 图集里的 part（O：8 B）。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrefabMeshAtlasPart {
    /// `MeshAtlas::add` 返回的 part 序号（O @0）。
    pub part_index: u32,
    /// 所属 subset（O @4）。
    pub subset: GpuWorldSubsetKind,
}

/// 某个 world subset 的图集及其在 `AssetMeshLibrary` 里的登记句柄。
#[derive(Debug)]
pub struct WorldSubsetMeshAtlas {
    /// `register_meshes` 首次登记后保存（O：Option 判别式 @0、句柄 @8）。
    pub registered: Option<Handle<Mesh>>,
    pub atlas: MeshAtlas,
}

impl Default for WorldSubsetMeshAtlas {
    fn default() -> Self {
        WorldSubsetMeshAtlas { registered: None, atlas: MeshAtlas::default() }
    }
}

/// 图集被修改的事件（I：载荷为受影响 subset 集合，字段名 U）。
#[derive(Event, Debug, Default, Clone)]
pub struct MeshAtlasesModifiedCmd {
    pub subsets: HashSet<GpuWorldSubsetKind>,
}

/// 网格图集库。
#[derive(Resource, Debug)]
pub struct MeshAtlasLibrary {
    world_subsets: HashMap<GpuWorldSubsetKind, WorldSubsetMeshAtlas>,
    prefab_parts: HashMap<AtlassedMeshName, Vec<PrefabMeshAtlasPart>>,
}

impl Default for MeshAtlasLibrary {
    /// O：`insert_resources` 里的 `from_iter(all kinds -> default atlas)` + 空 prefab 表。
    fn default() -> Self {
        MeshAtlasLibrary {
            world_subsets: GpuWorldSubsetKind::all().map(|k| (k, WorldSubsetMeshAtlas::default())).collect(),
            prefab_parts: HashMap::new(),
        }
    }
}

impl MeshAtlasLibrary {
    /// `add<N>` 0x140a371d0（&str）/ 0x140a37760（PrefabName）。
    ///
    /// 顺序（O）：名字哈希 -> 取（缺失则补默认）subset 图集 -> `MeshAtlas::add` ->
    /// `Err` 则 panic -> 在 prefab 表里追加 `{part_index, subset}`。
    pub fn add<N: Into<AtlassedMeshName>>(&mut self, subset: GpuWorldSubsetKind, name: N, mesh: &Mesh) {
        let name: AtlassedMeshName = name.into();
        let ws = self.world_subsets.entry(subset).or_default();
        let part_index = match ws.atlas.add(name, mesh) {
            Ok(i) => i,
            // O：closure 0x140a37cf0；`{:#}` 为 anyhow 的 alternate 展开。
            Err(e) => panic!("Error adding prefab {:?} to atlas of shader {:?}: {:#}", name, subset, e),
        };
        self.prefab_parts.entry(name).or_default().push(PrefabMeshAtlasPart { part_index, subset });
    }

    /// `get_by_name<N>` 0x140a38150 / 0x140a382d0：缺失 -> panic。
    pub fn get_by_name<N: Into<AtlassedMeshName>>(&self, name: N) -> &[PrefabMeshAtlasPart] {
        let name: AtlassedMeshName = name.into();
        match self.prefab_parts.get(&name) {
            Some(v) => v,
            None => panic!("Atlas library: unknown prefab {:?}", name),
        }
    }

    /// 兼容旧调用点：`nani_maintain_instances` 用原始哈希查（O：`get_by_name<NameHash>`）。
    pub fn get_by_name_hash(&self, hash: u64) -> &[PrefabMeshAtlasPart] {
        self.get_by_name(NameHash(hash))
    }

    /// `get_by_world_subset` 0x140ae6750。
    pub fn get_by_world_subset(&self, kind: GpuWorldSubsetKind) -> &WorldSubsetMeshAtlas {
        match self.world_subsets.get(&kind) {
            Some(w) => w,
            None => panic!(
                "Atlas library: unknown subworld {:?}. Make sure to register some meshes in nani_meshes.ron",
                kind
            ),
        }
    }

    /// `clear_atlas_by_name<&PrefabName>` 0x140a37e50：prefab 的 part 列表清空（保留键），
    /// 并从每个 subset 图集的 `parts` 表中移除该名字（不动 mesh/layouts）。
    pub fn clear_atlas_by_name(&mut self, name: &PrefabName) {
        let key: AtlassedMeshName = name.into();
        if let Some(v) = self.prefab_parts.get_mut(&key) {
            v.clear();
        }
        for ws in self.world_subsets.values_mut() {
            ws.atlas.parts.remove(&key);
        }
    }

    /// `clear_world_subset_atlas` 0x140ae6880：清空该 subset 的图集，并从所有 prefab 列表里
    /// 删掉属于该 subset 的项（保序，`Vec::retain`）。`registered` 句柄保留。
    pub fn clear_world_subset_atlas(&mut self, kind: GpuWorldSubsetKind) {
        if let Some(ws) = self.world_subsets.get_mut(&kind) {
            ws.atlas.mesh.attributes.clear();
            ws.atlas.mesh.indices.clear();
            ws.atlas.parts.clear();
            ws.atlas.layouts.clear();
        }
        for v in self.prefab_parts.values_mut() {
            v.retain(|p| p.subset != kind);
        }
    }

    /// `register_meshes` 0x140ae6b90：把每个非空图集的 mesh 推进 `AssetMeshLibrary`。
    pub fn register_meshes(&mut self, meshes: &mut AssetMeshLibrary) {
        for (kind, ws) in self.world_subsets.iter_mut() {
            if ws.atlas.mesh.indices.is_empty() {
                log::warn!("Not registering atlas for world subset {:?}: fused mesh has no verts", kind);
                continue;
            }
            match ws.registered {
                Some(handle) => {
                    log::debug!("Updating mesh atlas for world subset {:?}", kind);
                    let new_mesh = ws.atlas.mesh.clone();
                    *meshes.get_mut(handle).unwrap() = new_mesh;
                }
                None => {
                    let new_mesh = ws.atlas.mesh.clone();
                    let handle = meshes.add(MeshAsset::new(new_mesh, Some(format!("{:?}:atlas", kind))));
                    ws.registered = Some(handle);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec3;

    fn quad(label: &'static str) -> Mesh {
        let mut m = Mesh::new(label);
        m.set_attribute(
            crate::render::mesh::ATTR_POSITION,
            vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(3.0, 0.0, 0.0), Vec3::new(0.0, 4.0, 0.0)],
        );
        m.set_indices(vec![0, 1, 2]);
        m
    }

    const MAIN: GpuWorldSubsetKind = GpuWorldSubsetKind::Canopy;
    const BUSH: GpuWorldSubsetKind = GpuWorldSubsetKind::Bush;

    #[test]
    fn debug_matches_original_format() {
        let n = AtlassedMeshName(NameHash(7));
        assert_eq!(format!("{:?}", n), "AtlassedMeshName(NameHash { hash: 7 })");
    }

    #[test]
    fn conversions_prefix_only_for_prefab_names() {
        assert_eq!(AtlassedMeshName::from("a").0, NameHash::compute("a"));
        let p = PrefabName::new("a");
        assert_eq!(AtlassedMeshName::from(&p).0, NameHash::compute("prefab:a"));
        assert_eq!(AtlassedMeshName::from(p.clone()), AtlassedMeshName::from(PrefabNameRef::new("a")));
        assert_eq!(AtlassedMeshName::from(NameHash(5)).0, NameHash(5));
    }

    #[test]
    fn default_has_all_27_subsets() {
        let lib = MeshAtlasLibrary::default();
        for k in GpuWorldSubsetKind::all() {
            assert!(lib.get_by_world_subset(k).registered.is_none());
            assert_eq!(lib.get_by_world_subset(k).atlas.mesh.label, "MeshAtlas");
        }
        assert_eq!(lib.world_subsets.len(), 27);
    }

    #[test]
    fn add_then_lookup_by_each_name_form() {
        let mut lib = MeshAtlasLibrary::default();
        lib.add(MAIN, "rock", &quad("q"));
        lib.add(BUSH, "rock", &quad("q"));
        let v = lib.get_by_name("rock");
        assert_eq!(
            v,
            &[
                PrefabMeshAtlasPart { part_index: 0, subset: MAIN },
                PrefabMeshAtlasPart { part_index: 0, subset: BUSH }
            ]
        );
        assert_eq!(lib.get_by_name_hash(NameHash::compute("rock").0).len(), 2);

        let p = PrefabName::new("house");
        lib.add(MAIN, p.clone(), &quad("q"));
        assert_eq!(lib.get_by_name(&p)[0].part_index, 1);
        assert_eq!(lib.get_by_name(PrefabNameRef::new("house")).len(), 1);
    }

    #[test]
    #[should_panic(expected = "Atlas library: unknown prefab AtlassedMeshName(NameHash { hash: 9 })")]
    fn unknown_prefab_panics() {
        MeshAtlasLibrary::default().get_by_name(NameHash(9));
    }

    #[test]
    #[should_panic(expected = "Atlas library: unknown subworld Canopy. Make sure to register some meshes in nani_meshes.ron")]
    fn unknown_subworld_panics() {
        let mut lib = MeshAtlasLibrary::default();
        lib.world_subsets.clear();
        lib.get_by_world_subset(MAIN);
    }

    #[test]
    #[should_panic(expected = "Error adding prefab AtlassedMeshName(NameHash { hash: 1 }) to atlas of shader Canopy: Mesh attribute mismatch")]
    fn add_error_panics_with_alternate_error() {
        let mut lib = MeshAtlasLibrary::default();
        lib.add(MAIN, NameHash(0), &quad("a"));
        let mut other = quad("b");
        other.set_attribute("Vertex_Color", vec![Vec3::ONE; 3]);
        lib.add(MAIN, NameHash(1), &other);
    }

    #[test]
    fn clear_by_name_keeps_key_and_forgets_parts() {
        let mut lib = MeshAtlasLibrary::default();
        let p = PrefabName::new("x");
        lib.add(MAIN, &p, &quad("q"));
        let key: AtlassedMeshName = (&p).into();
        lib.clear_atlas_by_name(&p);
        assert!(lib.get_by_name(&p).is_empty());
        let a = &lib.get_by_world_subset(MAIN).atlas;
        assert_eq!(a.part_index(key), None);
        assert_eq!(a.layouts.len(), 1);
        assert_eq!(a.mesh.indices.len(), 3);
        // 清掉后可再次加入同名（parts 里已无重复）。
        lib.add(MAIN, &p, &quad("q"));
        assert_eq!(lib.get_by_name(&p).len(), 1);
    }

    #[test]
    fn clear_subset_empties_atlas_and_filters_prefab_lists() {
        let mut lib = MeshAtlasLibrary::default();
        lib.add(MAIN, "a", &quad("q"));
        lib.add(BUSH, "a", &quad("q"));
        lib.add(MAIN, "b", &quad("q"));
        lib.clear_world_subset_atlas(MAIN);
        let a = &lib.get_by_world_subset(MAIN).atlas;
        assert!(a.mesh.indices.is_empty() && a.layouts.is_empty() && a.mesh.attributes.is_empty() && a.is_empty());
        assert_eq!(lib.get_by_name("a"), &[PrefabMeshAtlasPart { part_index: 0, subset: BUSH }]);
        assert!(lib.get_by_name("b").is_empty());
        assert_eq!(lib.get_by_world_subset(BUSH).atlas.len(), 1);
    }

    #[test]
    fn register_meshes_adds_then_updates_in_place() {
        let mut lib = MeshAtlasLibrary::default();
        let mut meshes = AssetMeshLibrary::new();
        lib.register_meshes(&mut meshes); // 全空：只 warn
        assert!(lib.get_by_world_subset(MAIN).registered.is_none());

        lib.add(MAIN, "a", &quad("q"));
        lib.register_meshes(&mut meshes);
        let h = lib.get_by_world_subset(MAIN).registered.expect("registered");
        assert_eq!(meshes.get(h).unwrap().indices.len(), 3);
        assert_eq!(meshes.get_handle_by_name("Canopy:atlas"), h);
        assert!(lib.get_by_world_subset(BUSH).registered.is_none());

        lib.add(MAIN, "b", &quad("q"));
        lib.register_meshes(&mut meshes);
        assert_eq!(lib.get_by_world_subset(MAIN).registered, Some(h));
        assert_eq!(meshes.get(h).unwrap().indices.len(), 6);
        // O：`get_mut` 每次登记后更新都会把 handle 压入 modified_meshes（驱动 VAO 重建）。
        assert_eq!(meshes.modified_meshes, vec![h]);
    }
}
