//! 网格热重载通知。回调将源路径去重，消费线程负责加载和发布。
//! 原作来源见项目 evidence；当前接口、行为变化与验证范围见 README.md。

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use bevy_ecs::resource::Resource;

use crate::render::mesh::Mesh;
use crate::reused::watch::GenericHotwatch;

pub const WATCH_DEBOUNCE: Duration = Duration::from_nanos(100_000_000);

/// 网格源路径通知；重复事件合并为一次重载。
#[derive(Resource)]
pub struct MeshWatch {
    changed_paths: Arc<Mutex<HashSet<PathBuf>>>,
    watch_lock: Mutex<()>,
    hotwatch: Option<GenericHotwatch>,
}

impl MeshWatch {
    /// 创建初始状态。
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

    /// 注册文件通知；未启用监听时直接返回。
    pub fn watch(&self, mesh: &Mesh) {
        let Some(source) = mesh.source_file.as_ref() else {
            return;
        };
        let watched = PathBuf::from(&source.path);
        let changed = Arc::clone(&self.changed_paths);
        let _guard = self.watch_lock.lock().unwrap();
        if let Some(hotwatch) = &self.hotwatch {
            let callback_path = watched.clone();
            hotwatch
                .watch(&watched, move |event| {
                    Self::on_debounced_event(
                        &changed,
                        &callback_path,
                        matches!(
                            event,
                            hotwatch::Event::Create(_)
                                | hotwatch::Event::Write(_)
                                | hotwatch::Event::Rename(_, _)
                        ),
                    );
                })
                .expect("failed to watch file!");
        }
    }

    pub fn on_debounced_event(
        changed: &Mutex<HashSet<PathBuf>>,
        watched: &Path,
        is_create_or_write: bool,
    ) {
        if is_create_or_write {
            changed.lock().unwrap().insert(watched.to_path_buf());
        }
    }

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
            m.set_source_file(SourceFile {
                path: p.into(),
                stamp: 0,
            });
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
