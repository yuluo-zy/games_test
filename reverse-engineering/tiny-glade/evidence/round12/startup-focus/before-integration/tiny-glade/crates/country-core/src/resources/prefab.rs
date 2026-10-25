//! prefab.rs — 预制体引用组件 + 预制体库（`PrefabLibrary`，fresh 反编译 2026-10-17）。
//!
//! 依据：`chapters/resources/prefab.rs.md` + skeleton A `resources/prefab.rs` + 本次 fresh
//! `get_functions`（add 0x1408e2540 / get 0x141552d70 / has 0x141552ff0）。
//!
//! # `PrefabLibrary`（O：`HashMap<PrefabName, Prefab>`，hashbrown RandomState）
//! | 函数 | VA | fresh 证据 |
//! |---|---|---|
//! | `add(name, prefab)` | 0x1408e2540 | `insert<PrefabName,Prefab,RandomState,Global>`；被替换旧值逐 `PrefabPart` drop（`HashMap::insert` 语义即此） |
//! | `get<&PrefabName>` | 0x141552d70 | 空表直接 miss；否则 `as_name_ref`→`hash_one<&str>` SIMD probe + memcmp（按 str 内容）；miss → `from_iter<Keys>` 收 `Vec<&PrefabName>` + `panic_fmt` |
//! | `has<&PrefabNameRef>` | 0x141552ff0 | `contains_key`（同 get 的 str 哈希/比较）→ bool |
//! | `iter` | inline | `inner.iter()`（PDB 内联宿主 `load_mesh_atlases`） |
//!
//! panic 文案（O，read_memory 0x142c6c7a0/0x142c6c7c0 逐字）：`No such prefab {:?}; available: {:?}`
//! （两格式参数 = 请求名 `PrefabNameRef` Debug + `Vec<&PrefabName>` Debug，`core::fmt::impl$74::fmt<PrefabNameRef>`
//! + `alloc::vec::impl$27::fmt<&PrefabName>`，2 段 "No such prefab " / "; available: "）。
//!
//! `PrefabName`/`PrefabNameRef` 权威定义在 `resources::render::mesh_atlas_library`
//! （外部 `asset::prefab` 的库内建模，A3 res_render 已按机器码核对）；本文件 re-export。
//!
//! Part is reconstructed from the actual1412a1df0 producer and14129f400/140840610
//! consumers. Audio and RT bounds have separate owners.

use bevy_ecs::component::Component;
use bevy_ecs::event::Event;
use bevy_ecs::resource::Resource;
use glam::Vec3;
use hashbrown::HashMap;

use crate::render::mesh::Mesh;
use crate::resources::render::mesh_atlas_library::{PrefabName, PrefabNameRef};
use crate::utils_contract::Aabb3;

/// Mutable placement component. B hide_trees indexes this component with a
/// 48-byte stride and copies its first 24 bytes into the history placement.
/// The owning prefab name is in systems::prefab::PrefabInstanceData.
#[repr(transparent)]
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct PrefabInstance(pub crate::resources::placement_data::PlacementDataSsbo);

/// 预制体库值（O：`HashMap<PrefabName, Prefab>` 的值；`add` 被替换旧值逐 `PrefabPart` drop）。
/// 字段布局 U（`parts` 为语义 I；`PrefabPart` 原生 0x98 B）。
#[derive(Clone, Debug, Default)]
pub struct Prefab {
    /// 组成部件（I：`Prefab { parts: Vec<PrefabPart> }`，`add` 的 drop_in_place 逐个 `PrefabPart`）。
    pub parts: Vec<PrefabPart>,
}

/// Original B 0x98B runtime part, projected to current owned types.
/// +0 part name, +18 optional audio String/Aabb, +48 RT bounds,
/// +60/6c/78 seasonal albedos, +84 RT semantic/None2,
/// +88 Arc<Mesh> and +90 GpuWorldSubsetKind. No native pointer is retained.
#[derive(Clone, Debug)]
pub struct PrefabPart {
    pub name: String,
    pub audio_trigger: Option<PrefabAudioTrigger>,
    pub render_mesh: std::sync::Arc<Mesh>,
    pub render_subset: crate::render::nani::gpu_world::GpuWorldSubsetKind,
    pub rt_kind: u8,
    pub albedos: [Vec3; 3],
    /// RT-only box scaled by PrefabRtConfig.mesh_scale. Audio has its own box.
    pub bounds: Aabb3,
}

/// Existing construction adapter; actual startup builds every field from its
/// resolved prefab and mesh. This is not a recovered native Part::default.
impl Default for PrefabPart {
    fn default() -> Self {
        Self {
            name: String::new(),
            audio_trigger: None,
            render_mesh: std::sync::Arc::new(Mesh::default()),
            render_subset: crate::render::nani::gpu_world::GpuWorldSubsetKind::Canopy,
            rt_kind: 0,
            albedos: [Vec3::ZERO; 3],
            bounds: Aabb3 {
                min: Vec3::ZERO,
                max: Vec3::ZERO,
            },
        }
    }
}

impl PrefabPart {
    /// `create_prefab_rt` 的季节表选择（O：三张 u64 偏移表按 season 索引）。
    /// 季节 0..=2 → `[0]`、3 → `[1]`、4 → `[2]`；表外索引原版无边界检查，
    /// 本 crate 取最后一组（越界属调用方违约，登记）。
    pub fn albedo_for_season(&self, season: usize) -> Vec3 {
        match season {
            0..=2 => self.albedos[0],
            3 => self.albedos[1],
            _ => self.albedos[2],
        }
    }
}

/// Native optional audio payload: String(cap/ptr/len) @+18/+20/+28,
/// then its Aabb3 @+30..48. The observed +28 value4 is "Tree".len(), not a tag.
#[derive(Clone, Debug, PartialEq)]
pub struct PrefabAudioTrigger {
    pub kind_name: String,
    pub bounds: Aabb3,
}
impl PrefabAudioTrigger {
    pub fn is_tree(&self) -> bool {
        self.kind_name == "Tree"
    }
}

/// `HashMap<PrefabName, _>::get(&PrefabNameRef)` 的借用查找（O：`has<&PrefabNameRef>` 0x141552ff0）。
/// 两者按 `str` 内容哈希/比较（`hash_one<&str>` + memcmp），故 `Equivalent` 成立。
impl hashbrown::Equivalent<PrefabName> for PrefabNameRef {
    fn equivalent(&self, key: &PrefabName) -> bool {
        self.as_str() == key.as_name_ref().as_str()
    }
}

/// 预制体库资源（O：`HashMap<PrefabName, Prefab>` + RandomState；`resources::impl$1::insert_resources`
/// 0x140940d80 里 `impl$1::default` 构造）。
#[derive(Resource, Clone, Debug)]
pub struct PrefabLibrary {
    pub inner: HashMap<PrefabName, Prefab>,
}

impl Default for PrefabLibrary {
    fn default() -> Self {
        PrefabLibrary {
            inner: HashMap::new(),
        }
    }
}

impl PrefabLibrary {
    /// `add`（O 0x1408e2540）：`HashMap::insert`。被替换旧值由 `insert` 返回并 drop（其 `PrefabPart`
    /// 逐个 `drop_in_place`，与 fresh 反编译一致）。
    pub fn add(&mut self, name: PrefabName, prefab: Prefab) {
        self.inner.insert(name, prefab);
    }

    /// `get<&PrefabName>`（O 0x141552d70）：空表直接 miss；否则按名字 `str` 内容哈希/比较查找。
    /// miss → `panic!("No such prefab {:?}; available: {:?}", name.as_name_ref(), keys)`。
    /// 返回 `&Prefab`（不返回 Option——原版 miss 走 `panic_fmt`，不返回）。
    pub fn get(&self, name: &PrefabName) -> &Prefab {
        // O：`as_name_ref` → `hash_one<&str>` SIMD probe + memcmp（按 str 内容）。`PrefabNameRef`
        // derive(Hash) 与 str 同哈希、`Equivalent<PrefabName>` 已实现，故以借用名查找。
        self.inner.get(name.as_name_ref()).unwrap_or_else(|| {
            let available: Vec<&PrefabName> = self.inner.keys().collect();
            panic!(
                "No such prefab {:?}; available: {:?}",
                name.as_name_ref(),
                available
            )
        })
    }

    /// `has<&PrefabNameRef>`（O 0x141552ff0）：`contains_key` → bool（空表直接 false，不哈希）。
    pub fn has(&self, name: &PrefabNameRef) -> bool {
        self.inner.contains_key(name)
    }

    /// `iter`（O inline）：遍历 `(name, prefab)` 条目（PDB 内联宿主 `load_mesh_atlases`）。
    pub fn iter(&self) -> hashbrown::hash_map::Iter<'_, PrefabName, Prefab> {
        self.inner.iter()
    }
}

/// 库变更事件（O：`PrefabsModifiedCmd`；`PrefabWatchPlugin::build` 0x1408e4b00 注册）。
#[derive(Event, Clone, Debug, Default, PartialEq, Eq)]
pub struct PrefabsModifiedCmd {
    pub prefabs: std::collections::HashSet<PrefabName>,
}

/// `PrefabWatchPlugin`（O 名；`build` 0x1408e4b00 插入 `PrefabWatcher` + 注册
/// `PrefabsModifiedCmd`/`MeshAtlasesModifiedCmd` 事件 + `add_systems(Update, …)`——Bevy 注册胶水）。
/// `PrefabWatcher` 本体已落在 [`crate::startup::load_prefabs::PrefabWatcher`]
/// （hotwatch + 100 ms，fresh 0x1412a8e00）；插件挂载仍待 bevy_app 宿主。
pub struct PrefabWatchPlugin;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_get_has_share_one_name_type() {
        let mut lib = PrefabLibrary::default();
        let name = PrefabName::new("house/a");
        lib.add(name.clone(), Prefab::default());
        assert!(lib.get(&name).parts.is_empty());
        assert!(lib.has(PrefabNameRef::new("house/a")), "借用名按内容命中");
        assert!(!lib.has(PrefabNameRef::new("house/b")));
        assert_eq!(lib.iter().count(), 1);
    }

    #[test]
    fn add_replaces_previous_value() {
        let mut lib = PrefabLibrary::default();
        let name = PrefabName::new("dup");
        lib.add(name.clone(), Prefab { parts: Vec::new() });
        lib.add(
            name.clone(),
            Prefab {
                parts: vec![PrefabPart {
                    rt_kind: 3,
                    ..PrefabPart::default()
                }],
            },
        );
        assert_eq!(lib.get(&name).parts.len(), 1);
        assert_eq!(lib.iter().count(), 1);
    }

    #[test]
    fn get_miss_panics_with_available_keys() {
        let mut lib = PrefabLibrary::default();
        lib.add(PrefabName::new("tree/a"), Prefab::default());
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            lib.get(&PrefabName::new("tree/zzz"));
        }));
        let msg = r.unwrap_err().downcast::<String>().unwrap();
        // O：panic 用 `{:?}` 于 `PrefabNameRef`（derive Debug，透明 str 包装 → `PrefabNameRef("…")`）
        // 与 `Vec<&PrefabName>`（`PrefabName("…")`）。文案逐字 `No such prefab …; available: …`。
        assert!(
            msg.contains("No such prefab PrefabNameRef(\"tree/zzz\"); available: [PrefabName(\"tree/a\")]"),
            "panic message: {msg}"
        );
    }

    #[test]
    fn empty_library_misses_without_hashing() {
        let lib = PrefabLibrary::default();
        assert!(!lib.has(PrefabNameRef::new("anything")));
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            lib.get(&PrefabName::new("anything"));
        }));
        assert!(r.is_err());
    }
}
