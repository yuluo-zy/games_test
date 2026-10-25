//! `startup::load_prefabs` — 预制体资产载入与热重载（O：`crates/country-core/src/startup/load_prefabs.rs`）。
//!
//! fresh 证据（2026-10-17，ghidra shared）：
//! * `PrefabWatcher::new` 0x1412a8e00：`hotwatch::GenericHotwatch::new_with_custom_delay`
//!   （`MOV R8D,0x5f5e100` = **100 ms**）+ 两个 `Arc<Mutex<HashSet<…>>>`（+0x30/+0x38）。
//!   监视关闭时 watcher 判别 = 3（不调用 hotwatch）。
//! * `watch_prefab` / `watch_atlas`：`Hotwatch::watch(path, closure)`；closure 捕获
//!   `Arc<Mutex<HashSet<PrefabName>>>` + `Vec<PrefabName>`，或
//!   `Arc<Mutex<HashSet<GpuWorldSubsetKind>>>` + kind 字节。
//! * closure body 0x140922830 / 0x140922a80：仅当
//!   `*event >> 1 == 0x4000000000000001` 时锁集合并逐个 insert。
//! * `reload_prefabs` 0x1412a1a60：Mutex 守卫 + 重入检查 → `load_prefabs_impl`
//!   → 清空变更集。
//! * `load_prefabs_impl` 0x1412a1df0：大 monolith（`assets/prefabs` RON →
//!   `asset::prefab::config::PrefabConfig` → `load_prefab_mesh` +
//!   `filter_mesh_by_vertex_color`×2 → `PrefabLibrary::add` + 依赖图 + canopy quad）。
//! * `PrefabWatchPlugin::build` 0x1408e4b00（胶水）：注册 `PrefabsModifiedCmd` /
//!   `MeshAtlasesModifiedCmd` 事件、插入 `PrefabWatcher`、`add_systems(Update, …)`。
//!
//! # seam / 登记
//! 1. `hotwatch` 在 O 上实例化为 `GenericHotwatch<notify::windows::ReadDirectoryChangesWatcher>`；
//!    本 crate 用平台别名 `hotwatch::Hotwatch`（`RecommendedWatcher`）。
//! 2. closure 的事件判别是原始 tag 谓词 `*event >> 1 == 0x4000000000000001`；本 crate 以
//!    `DebouncedEvent::Write` 近似（枚举 tag 未锚定，登记）。
//! 3. `load_prefab_mesh`（mesh 属性提取 / `MeshProcessorConfig` panic）依赖 AssetMeshLibrary，
//!    尚未接线：`PrefabPart.mesh` 留空、`bounds` 为默认，登记 mesh 载入 seam。
//! 4. `PrefabPartConfig` → `PrefabPart` 的逐字段映射只实现已锚定部分（albedo 三套、
//!    audio kind、rt_kind 1/2）；`rt.semantic`、`audio.shape_scale/name/rank` 的运行时
//!    映射未锚定，登记。

use std::collections::HashSet;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use glam::Vec3;

use crate::render::mesh::Mesh;
use crate::render::nani::gpu_world::GpuWorldSubsetKind;
use crate::resources::prefab::{Prefab, PrefabAudioTrigger, PrefabLibrary, PrefabPart};
use crate::resources::render::mesh_atlas_library::PrefabName;
use crate::utils_contract::Aabb3;

/// hotwatch 的防抖延迟（O：`PrefabWatcher::new` 0x1412a8e00 `MOV R8D,0x5f5e100` = 100 ms）。
pub const WATCH_DELAY: Duration = Duration::from_millis(100);

/// 预制体热重载 watcher（O：`startup::load_prefabs::PrefabWatcher`，0x40 B）。
///
/// 布局（fresh disasm）：`+0x00..0x30` = `Result<Hotwatch>`（判别 3 = 未监视）、
/// `+0x30` = `Arc<Mutex<HashSet<PrefabName>>>`、`+0x38` =
/// `Arc<Mutex<HashSet<GpuWorldSubsetKind>>>`。
pub struct PrefabWatcher {
    /// `+0x00`：`None` = O 判别 3（未监视）。
    watcher: Option<hotwatch::Hotwatch>,
    /// `+0x30`：预制体变更集合。
    pub prefab_changes: Arc<Mutex<HashSet<PrefabName>>>,
    /// `+0x38`：mesh atlas subset 变更集合。
    pub atlas_changes: Arc<Mutex<HashSet<GpuWorldSubsetKind>>>,
}

impl Default for PrefabWatcher {
    fn default() -> Self {
        Self::new(false)
    }
}

impl PrefabWatcher {
    /// O：0x1412a8e00。`watching == false` → 不初始化 hotwatch（判别 3），但仍建两个集合。
    /// hotwatch 初始化失败按原版 `unwrap_failed` panic。
    pub fn new(watching: bool) -> Self {
        let watcher = if watching {
            Some(
                hotwatch::Hotwatch::new_with_custom_delay(WATCH_DELAY)
                    .unwrap_or_else(|_| panic!("called `Result::unwrap()` on an `Err` value")),
            )
        } else {
            None
        };
        Self {
            watcher,
            prefab_changes: Arc::new(Mutex::new(HashSet::new())),
            atlas_changes: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// O：`impl$1::watch_prefab`。监视 `path`；命中事件时把捕获的 `names` 全部插入集合。
    /// 未监视（判别 3）→ 直接成功（原版跳过 `watch`）。
    pub fn watch_prefab(
        &mut self,
        path: impl AsRef<Path>,
        names: Vec<PrefabName>,
    ) -> Result<(), hotwatch::Error> {
        let Some(watcher) = &mut self.watcher else {
            return Ok(());
        };
        let changes = Arc::clone(&self.prefab_changes);
        watcher.watch(path, move |event| {
            if is_relevant_event(&event) {
                let mut set = changes.lock().expect("prefab watcher mutex poisoned");
                set.extend(names.iter().cloned());
            }
        })
    }

    /// O：`impl$1::watch_atlas`。监视 `path`；命中事件时把捕获的 `kind` 插入集合。
    pub fn watch_atlas(
        &mut self,
        path: impl AsRef<Path>,
        kind: GpuWorldSubsetKind,
    ) -> Result<(), hotwatch::Error> {
        let Some(watcher) = &mut self.watcher else {
            return Ok(());
        };
        let changes = Arc::clone(&self.atlas_changes);
        watcher.watch(path, move |event| {
            if is_relevant_event(&event) {
                let mut set = changes.lock().expect("prefab watcher mutex poisoned");
                set.insert(kind);
            }
        })
    }

    /// 取走两个变更集合（O：`reload_prefabs` 之后的 `drop_inner_table` 清空语义）。
    pub fn take_changed(
        &self,
    ) -> (HashSet<PrefabName>, HashSet<GpuWorldSubsetKind>) {
        let prefabs = std::mem::take(
            &mut *self
                .prefab_changes
                .lock()
                .expect("prefab watcher mutex poisoned"),
        );
        let atlases = std::mem::take(
            &mut *self
                .atlas_changes
                .lock()
                .expect("prefab watcher mutex poisoned"),
        );
        (prefabs, atlases)
    }
}

/// O：closure 0x140922830/0x140922a80 的原始谓词 `*event >> 1 == 0x4000000000000001`。
///
/// 该 tag 未映射到具名 `DebouncedEvent` 变体（登记）；这里用 `Write` 近似。
pub fn is_relevant_event(event: &hotwatch::Event) -> bool {
    matches!(event, hotwatch::Event::Write(_))
}

/// `reload_prefabs`（O：0x1412a1a60）的语义模型：仅在 watcher 有变更时重扫描，
/// 并返回被消费的两个变更集合（宿主据此写 `PrefabsModifiedCmd` /
/// `MeshAtlasesModifiedCmd`）。
pub fn reload_prefabs(
    root: &Path,
    watcher: &mut PrefabWatcher,
    library: &mut PrefabLibrary,
) -> anyhow::Result<(HashSet<PrefabName>, HashSet<GpuWorldSubsetKind>)> {
    let (prefabs, atlases) = watcher.take_changed();
    if prefabs.is_empty() && atlases.is_empty() {
        return Ok((prefabs, atlases));
    }
    load_prefabs_impl(root, library)?;
    Ok((prefabs, atlases))
}

/// `load_prefabs_impl`（O：0x1412a1df0）的可落地部分：`assets/prefabs` 树扫描
/// （`asset::prefab::scan::scan_asset_tree`）→ `PrefabConfig` → 运行时 `Prefab` →
/// `PrefabLibrary::add`。
///
/// 已实现（fresh 锚定）：albedo 三套（+0x60/0x6c/0x78 组）、rt_kind（1 = AABB、
/// 2 = 无 RT）、audio kind 名/tag 门（`"Tree"` → tag 4）。
/// 登记：mesh 载入（`load_prefab_mesh`）、AABB 缓存（+0x48）、audio name/rank、
/// `rt.semantic` 到 kind 的映射。
pub fn load_prefabs_impl(root: &Path, library: &mut PrefabLibrary) -> anyhow::Result<()> {
    let scan = asset::prefab::scan::scan_asset_tree(root)?;
    for (name, config) in &scan.prefabs {
        let parts = config
            .parts
            .values()
            .filter(|part| !*part.delete.as_concrete())
            .map(prefab_part_from_config)
            .collect();
        library.add(PrefabName::new(name.as_str()), Prefab { parts });
    }
    Ok(())
}

/// `asset::prefab::config::PrefabPartConfig` → 运行时 [`PrefabPart`]（已锚定字段）。
fn prefab_part_from_config(config: &asset::prefab::config::PrefabPartConfig) -> PrefabPart {
    let rt = config.rt.as_concrete();
    let (rt_kind, albedos) = match rt {
        Some(rt) => (
            1u8, // O：`rt_kind & 1` → AABB 触发；`2` 才是禁用（映射登记）
            [
                Vec3::from_array(*rt.summer_albedo.as_concrete()),
                Vec3::from_array(*rt.autumn_albedo.as_concrete()),
                Vec3::from_array(*rt.winter_albedo.as_concrete()),
            ],
        ),
        None => (2u8, [Vec3::ZERO; 3]),
    };
    let audio_trigger = config.audio.as_concrete().as_ref().map(|audio| {
        let kind = audio.trigger.as_concrete().kind.as_concrete().clone();
        PrefabAudioTrigger {
            // O：part+0x28 tag == 4 且 part+0x20 首 4 字节 "Tree"。
            kind_tag: if kind.starts_with("Tree") { 4 } else { 0 },
            kind_name: kind,
            // `name`/`rank` 在 PrefabPartConfig 中没有锚定来源（登记）。
            name: String::new(),
            rank: 0,
        }
    });
    PrefabPart {
        audio_trigger,
        mesh: Mesh::new(""),
        rt_kind,
        albedos,
        bounds: Aabb3 {
            min: Vec3::ZERO,
            max: Vec3::ZERO,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn watcher_starts_disabled_with_empty_sets() {
        let mut watcher = PrefabWatcher::new(false);
        assert!(watcher.watcher.is_none());
        assert!(watcher.watch_prefab("nope", vec![PrefabName::new("x")]).is_ok());
        let (prefabs, atlases) = watcher.take_changed();
        assert!(prefabs.is_empty() && atlases.is_empty());
    }

    #[test]
    fn relevant_event_is_the_write_variant() {
        // O：原始谓词 `*event >> 1 == 0x4000000000000001`（具名变体映射登记）。
        let event = hotwatch::Event::Write(std::path::PathBuf::from("a.ron"));
        assert!(is_relevant_event(&event));
        assert!(!is_relevant_event(&hotwatch::Event::Rescan));
    }

    #[test]
    fn load_prefabs_impl_maps_config_parts() {
        let dir = std::env::temp_dir().join(format!("prefab_scan_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("tree.ron"),
            r#"(
                parts: {
                    "canopy": (
                        render: (mesh: "canopy", shader: "tree"),
                        rt: (mesh: Aabb, summer_albedo: [1.0, 0.5, 0.25],
                             autumn_albedo: [0.5, 0.5, 0.5],
                             winter_albedo: [0.0, 0.0, 0.0]),
                        audio: (trigger: (shape: Aabb, kind: "Tree")),
                    ),
                },
            )"#,
        )
        .unwrap();
        let mut library = PrefabLibrary::default();
        load_prefabs_impl(&dir, &mut library).unwrap();
        let _ = std::fs::remove_dir_all(&dir);
        let prefab = library.get(&PrefabName::new("tree"));
        assert_eq!(prefab.parts.len(), 1);
        let part = &prefab.parts[0];
        assert_eq!(part.rt_kind, 1);
        assert_eq!(part.albedos[0], glam::Vec3::new(1.0, 0.5, 0.25));
        assert!(part.audio_trigger.as_ref().unwrap().is_tree());
    }
}
