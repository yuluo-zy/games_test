//! 预制件配置扫描和热更新。监听回调只入队；文件 I/O 在锁外执行。
//! 配置全部解析成功后再替换库，失败归还通知并保留旧资产。
//! 网格与部分音频字段仍需资产域接入，不能将配置映射视为完整渲染资产。

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

/// 文件监听防抖时间。
pub const WATCH_DELAY: Duration = Duration::from_millis(100);

/// 预制件和图集的变更集合；同一资源的重复通知会自动合并。
pub struct PrefabWatcher {
    /// 关闭监听时不创建操作系统 watcher。
    watcher: Option<hotwatch::Hotwatch>,
    /// 等待重载的预制件名称。
    pub prefab_changes: Arc<Mutex<HashSet<PrefabName>>>,
    /// 等待重建的图集子集。
    pub atlas_changes: Arc<Mutex<HashSet<GpuWorldSubsetKind>>>,
}

impl Default for PrefabWatcher {
    fn default() -> Self {
        Self::new(false)
    }
}

impl PrefabWatcher {
    /// 建立可选监听；需要处理平台初始化错误的宿主可以使用 try_new。
    pub fn new(watching: bool) -> Self {
        Self::try_new(watching).expect("无法启动预制件文件监听")
    }

    /// 复用 hotwatch 的平台实现，不自行创建轮询线程。
    pub fn try_new(watching: bool) -> Result<Self, hotwatch::Error> {
        let watcher = if watching {
            Some(hotwatch::Hotwatch::new_with_custom_delay(WATCH_DELAY)?)
        } else {
            None
        };
        Ok(Self {
            watcher,
            prefab_changes: Arc::new(Mutex::new(HashSet::new())),
            atlas_changes: Arc::new(Mutex::new(HashSet::new())),
        })
    }

    /// 监听源文件；命中事件时合并其依赖预制件名称。
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

    /// 监听图集源文件；图集通知直接交给宿主重建，无需重读 RON。
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

    /// 取走当前批次；此后到达的事件会进入新集合，不与当前文件读取争用锁。
    pub fn take_changed(&self) -> (HashSet<PrefabName>, HashSet<GpuWorldSubsetKind>) {
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

/// 支持普通写入与编辑器的临时文件 rename 原子保存。
pub fn is_relevant_event(event: &hotwatch::Event) -> bool {
    matches!(
        event,
        hotwatch::Event::Write(_)
            | hotwatch::Event::Create(_)
            | hotwatch::Event::Rename(_, _)
            | hotwatch::Event::Remove(_)
            | hotwatch::Event::Rescan
    )
}

/// 重载有通知的批次。解析失败归还本批通知，不覆盖读取期间新到的事件。
/// 图集独立变化时跳过配置扫描，避免每次贴图保存都重读整个预制件目录。
pub fn reload_prefabs(
    root: &Path,
    watcher: &mut PrefabWatcher,
    library: &mut PrefabLibrary,
) -> anyhow::Result<(HashSet<PrefabName>, HashSet<GpuWorldSubsetKind>)> {
    let (mut prefabs, atlases) = watcher.take_changed();
    if prefabs.is_empty() {
        return Ok((prefabs, atlases));
    }
    let replacement = match read_prefab_library(root) {
        Ok(replacement) => replacement,
        Err(error) => {
            watcher
                .prefab_changes
                .lock()
                .expect("预制件通知锁已损坏")
                .extend(prefabs);
            watcher
                .atlas_changes
                .lock()
                .expect("图集通知锁已损坏")
                .extend(atlases);
            return Err(error);
        }
    };
    // 补上新增与删除的名称，消费者才能同步清理已不存在的实例内容。
    prefabs.extend(
        library
            .inner
            .keys()
            .filter(|name| !replacement.inner.contains_key(*name))
            .cloned(),
    );
    prefabs.extend(
        replacement
            .inner
            .keys()
            .filter(|name| !library.inner.contains_key(*name))
            .cloned(),
    );
    *library = replacement;
    Ok((prefabs, atlases))
}

/// 从完整目录创建库快照；调用者应为该目录使用独立的 PrefabLibrary。
/// 解析失败保留旧快照；成功时同时删除磁盘中已经移除的预制件。
pub fn load_prefabs_impl(root: &Path, library: &mut PrefabLibrary) -> anyhow::Result<()> {
    *library = read_prefab_library(root)?;
    Ok(())
}

fn read_prefab_library(root: &Path) -> anyhow::Result<PrefabLibrary> {
    let scan = asset::prefab::scan::scan_asset_tree(root)?;
    let mut library = PrefabLibrary::default();
    library.inner.reserve(scan.prefabs.len());
    for (name, config) in scan.prefabs {
        let parts = config
            .parts
            .values()
            .filter(|part| !*part.delete.as_concrete())
            .map(prefab_part_from_config)
            .collect();
        library.add(PrefabName::new(name.as_str()), Prefab { parts });
    }
    Ok(library)
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
        assert!(
            watcher
                .watch_prefab("nope", vec![PrefabName::new("x")])
                .is_ok()
        );
        let (prefabs, atlases) = watcher.take_changed();
        assert!(prefabs.is_empty() && atlases.is_empty());
    }

    #[test]
    fn relevant_event_is_the_write_variant() {
        // O：原始谓词 `*event >> 1 == 0x4000000000000001`（具名变体映射登记）。
        let event = hotwatch::Event::Write(std::path::PathBuf::from("a.ron"));
        assert!(is_relevant_event(&event));
        assert!(is_relevant_event(&hotwatch::Event::Rescan));
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
