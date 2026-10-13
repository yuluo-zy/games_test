//! `MeshWatch` — 网格源文件热重载监视器（layer A3，逐函数对照原版机器码重写）。
//!
//! 证据全文：`evidence/country-core-audit/A3_mesh.md`。标记：O＝机器码直接观察，I＝推断，U＝未知。
//!
//! # 布局（O：`new` 0x1408ba490，总大小 0x40）
//! | 偏移 | 字段 | 说明 |
//! |---|---|---|
//! | +0x00 | `changed_paths: Arc<Mutex<HashSet<PathBuf>>>` | `Arc::drop_slow<Mutex<HashSet<PathBuf,..>>>`；ArcInner 0x48 B：std `Mutex`（futex 字节@+0x10、poison@+0x11）+ `HashSet`@+0x18 |
//! | +0x08 | `watch_lock: Mutex<()>` | std `Mutex`（futex@+8、poison@+9），`watch` 全程持有它（数据是 ZST：`new` 没有初始化数据字节；PDB 名里的 `MutexGuard<bool>` 是 ICF 合并名，I） |
//! | +0x10 | `hotwatch: Option<GenericHotwatch<ReadDirectoryChangesWatcher>>` | niche：判别字 == 3 ⇒ `None`（O：`new` 里 `ecx = 3`、`watch` 里 `cmp [rsi+0x10], 3`） |
//!
//! # 旧版错误（已修）
//! * 旧版把共享状态建成 `Arc<Mutex<Option<Vec<u8>>>>`（“变更文件的字节”），回调里 `fs::read`；原版共享的是
//!   **变更路径集合** `HashSet<PathBuf>`，回调只把**被监视的路径**（不是事件里的路径）插进去，真正的重载由
//!   `systems::backend::meshwatch`（0x14085b4b0）逐个调 `AssetMeshLibrary::reload_mesh_src` 后清空集合。
//! * 旧 `watch(path: PathBuf)`；原版 `watch(&self, mesh: &Mesh)`：只有 `mesh.source_file` 为 `Some` 才登记（`None` 立刻返回）。
//! * 旧防抖延迟 50 ms；原版 `Duration::new(0, 0x5f5e100)` = **100 ms**。
//! * 旧版失败静默吞掉；原版 `new` 里 `.expect("hotwatch failed to initialize")`（meshwatch.rs 第 20 行 col 18），
//!   `watch` 里 `.expect("failed to watch file!")`（第 52 行 col 14）。`use_watch == false` 时不创建 watcher。

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use bevy_ecs::resource::Resource;

use crate::render::mesh::Mesh;
use crate::reused::watch::GenericHotwatch;

/// 防抖延迟（O：`new_with_custom_delay(secs = 0, nanos = 0x5f5e100)`）。
pub const WATCH_DEBOUNCE: Duration = Duration::from_nanos(100_000_000);

/// 网格监视器（O：`Res<MeshWatch>`）。
#[derive(Resource)]
pub struct MeshWatch {
    /// +0x00（O）。
    changed_paths: Arc<Mutex<HashSet<PathBuf>>>,
    /// +0x08（O 位置 / I 数据类型）。
    watch_lock: Mutex<()>,
    /// +0x10（O）。
    hotwatch: Option<GenericHotwatch>,
}

impl MeshWatch {
    /// `new(use_watch)`（O 0x1408ba490）：`use_watch` 才创建 watcher（100 ms 防抖，失败 `expect` panic）。
    pub fn new(use_watch: bool) -> Self {
        let hotwatch = if use_watch {
            Some(
                GenericHotwatch::new_with_custom_delay(WATCH_DEBOUNCE)
                    .expect("hotwatch failed to initialize"),
            )
        } else {
            None
        };
        MeshWatch {
            changed_paths: Arc::new(Mutex::new(HashSet::new())),
            watch_lock: Mutex::new(()),
            hotwatch,
        }
    }

    /// `watch(mesh)`（O 0x1408ba6e0）。
    /// 1. `mesh.source_file` 为 `None` → 直接返回（连锁都不取）；
    /// 2. 克隆 `Arc` 与路径，取 `watch_lock`（中毒则 `Result::unwrap` panic，第 39 行 col 43）；
    /// 3. 有 watcher 才 `hotwatch.watch(&path, 回调)`，失败 `expect("failed to watch file!")`；回调持有 `Arc` 与路径克隆。
    pub fn watch(&self, mesh: &Mesh) {
        let Some(source) = mesh.source_file.as_ref() else {
            return;
        };
        let watched = PathBuf::from(&source.path);
        let changed = Arc::clone(&self.changed_paths);
        let _guard = self.watch_lock.lock().unwrap();
        if let Some(hotwatch) = &self.hotwatch {
            let callback_path = watched.clone();
            // 占位 `GenericHotwatch::watch` 的回调只携带 PathBuf；真实 notify 回调收到 `DebouncedEvent`，
            // 其分类由 `on_debounced_event` 处理（I：占位只会投递 Create/Write）。
            hotwatch
                .watch(&watched, move |_event_path| {
                    Self::on_debounced_event(&changed, &callback_path, true);
                })
                .expect("failed to watch file!");
        }
    }

    /// 回调闭包体（O：`watch_path::closure$0` 0x14091e7a0）。事件判别（`tag ^ i64::MIN`）为 2（Create）或 3（Write）
    /// 时：锁 `changed_paths`（中毒 → `unwrap` panic），把**被监视路径**（闭包捕获的克隆，不是事件里的路径）
    /// 插入集合；其它事件（NoticeWrite/Remove/Chmod/Rename/Rescan/Error…）只 drop。
    pub fn on_debounced_event(
        changed: &Mutex<HashSet<PathBuf>>,
        watched: &Path,
        is_create_or_write: bool,
    ) {
        if is_create_or_write {
            changed.lock().unwrap().insert(watched.to_path_buf());
        }
    }

    /// `systems::backend::meshwatch`（0x14085b4b0）读取并清空的集合（O：它对 `[res+0]` 这个 Arc 里的 Mutex 加锁、
    /// 逐个 `reload_mesh_src`、再 `clear`）。
    pub fn changed_paths(&self) -> &Arc<Mutex<HashSet<PathBuf>>> {
        &self.changed_paths
    }

    /// 是否启用了 watcher（`hotwatch` 判别字 != 3，O）。
    pub fn is_watching(&self) -> bool {
        self.hotwatch.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::mesh::SourceFile;

    fn mesh_with(path: Option<&str>) -> Mesh {
        let mut m = Mesh::new("m");
        if let Some(p) = path {
            m.set_source_file(SourceFile { path: p.into(), stamp: 0 });
        }
        m
    }

    #[test]
    fn debounce_is_100_ms() {
        assert_eq!(WATCH_DEBOUNCE, Duration::new(0, 0x5f5e100));
        assert_eq!(WATCH_DEBOUNCE.as_millis(), 100);
    }

    #[test]
    fn new_without_watch_has_no_hotwatch_and_empty_set() {
        let w = MeshWatch::new(false);
        assert!(!w.is_watching());
        assert!(w.changed_paths().lock().unwrap().is_empty());
    }

    #[test]
    fn new_with_watch_creates_hotwatch() {
        assert!(MeshWatch::new(true).is_watching());
    }

    #[test]
    fn watch_without_source_file_is_noop_even_if_lock_poisoned() {
        let w = MeshWatch::new(true);
        let lock = &w.watch_lock;
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _g = lock.lock().unwrap();
            panic!("poison it");
        }));
        // source_file == None：在取锁之前就返回，所以不会因中毒 panic（O：jo 早退）。
        w.watch(&mesh_with(None));
    }

    #[test]
    #[should_panic]
    fn watch_with_source_panics_when_lock_poisoned() {
        let w = MeshWatch::new(false);
        let lock = &w.watch_lock;
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _g = lock.lock().unwrap();
            panic!("poison it");
        }));
        w.watch(&mesh_with(Some("a.json")));
    }

    #[test]
    fn watch_without_hotwatch_is_noop() {
        let w = MeshWatch::new(false);
        w.watch(&mesh_with(Some("a.json")));
        assert!(w.changed_paths().lock().unwrap().is_empty());
    }

    #[test]
    fn event_callback_inserts_watched_path_not_event_path() {
        let set = Mutex::new(HashSet::new());
        MeshWatch::on_debounced_event(&set, Path::new("meshes/a.json"), true);
        MeshWatch::on_debounced_event(&set, Path::new("meshes/a.json"), true); // 去重
        let got: Vec<_> = set.lock().unwrap().iter().cloned().collect();
        assert_eq!(got, vec![PathBuf::from("meshes/a.json")]);
    }

    #[test]
    fn non_create_write_events_are_ignored() {
        let set = Mutex::new(HashSet::new());
        MeshWatch::on_debounced_event(&set, Path::new("a.json"), false);
        assert!(set.lock().unwrap().is_empty());
    }

    #[test]
    #[should_panic]
    fn event_callback_panics_on_poisoned_set() {
        let set = Mutex::new(HashSet::<PathBuf>::new());
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _g = set.lock().unwrap();
            panic!("poison it");
        }));
        MeshWatch::on_debounced_event(&set, Path::new("a.json"), true);
    }
}
