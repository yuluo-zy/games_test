//! 纹理热重载通知。监听路径与逻辑名字分开，回调只登记变更名字。
//! 原作来源见项目 evidence；当前接口、行为变化与验证范围见 README.md。

use std::collections::HashSet;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub const WATCH_DELAY: Duration = Duration::from_millis(100);

#[cfg(not(test))]
pub mod hotwatch_contract {
    pub use crate::reused::watch::GenericHotwatch;
    pub use hotwatch::Event as DebouncedEvent;
}

#[cfg(test)]
pub mod hotwatch_contract {
    use std::path::{Path, PathBuf};
    use std::sync::Mutex;
    use std::time::Duration;

    /// `notify::DebouncedEvent`（notify 4.0.17 的变体顺序：NoticeWrite=0, NoticeRemove=1, Create=2, Write=3,
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum DebouncedEvent {
        NoticeWrite(PathBuf),
        NoticeRemove(PathBuf),
        Create(PathBuf),
        Write(PathBuf),
        Chmod(PathBuf),
        Remove(PathBuf),
        Rename(PathBuf, PathBuf),
        Rescan,
        Error(String, Option<PathBuf>),
    }

    #[derive(Debug)]
    pub struct HotwatchError(pub String);

    type Handler = Box<dyn Fn(DebouncedEvent) + Send + 'static>;

    pub struct GenericHotwatch {
        pub delay: Duration,
        handlers: Mutex<Vec<(PathBuf, Handler)>>,
    }

    impl GenericHotwatch {
        pub fn new_with_custom_delay(delay: Duration) -> Result<Self, HotwatchError> {
            Ok(GenericHotwatch {
                delay,
                handlers: Mutex::new(Vec::new()),
            })
        }
        pub fn watch(
            &self,
            path: &Path,
            handler: impl Fn(DebouncedEvent) + Send + 'static,
        ) -> Result<(), HotwatchError> {
            self.handlers
                .lock()
                .unwrap()
                .push((path.to_path_buf(), Box::new(handler)));
            Ok(())
        }
        /// 测试用：向某个被监视路径的回调投递事件。
        pub fn fire(&self, path: &Path, event: DebouncedEvent) {
            for (p, h) in self.handlers.lock().unwrap().iter() {
                if p == path {
                    h(event.clone());
                }
            }
        }
        pub fn watched(&self) -> usize {
            self.handlers.lock().unwrap().len()
        }
    }
}

use hotwatch_contract::{DebouncedEvent, GenericHotwatch};

pub use relative_path::RelativePathBuf;

/// 变更名字集合（`Arc<Mutex<HashSet<RelativePathBuf>>>`，O）。
pub type ChangedSet = Arc<Mutex<HashSet<RelativePathBuf>>>;

/// 逻辑纹理名字通知；不在文件线程中访问 GPU。
pub struct TextureWatch {
    hotwatch: Option<GenericHotwatch>,
    changed: ChangedSet,
}

impl bevy_ecs::resource::Resource for TextureWatch {}

/// 只接收创建、写入和重命名事件。
pub fn on_texture_event(
    changed: &Mutex<HashSet<RelativePathBuf>>,
    name: &str,
    event: &DebouncedEvent,
) {
    if matches!(
        event,
        DebouncedEvent::Create(_) | DebouncedEvent::Write(_) | DebouncedEvent::Rename(_, _)
    ) {
        let mut set = changed.lock().unwrap();
        set.insert(RelativePathBuf::from(name.to_owned()));
    }
}

impl TextureWatch {
    /// 创建初始状态。
    pub fn new(use_watch: bool) -> Self {
        let hotwatch = if use_watch {
            Some(
                GenericHotwatch::new_with_custom_delay(WATCH_DELAY)
                    .expect("hotwatch failed to initialize"),
            )
        } else {
            None
        };
        TextureWatch {
            hotwatch,
            changed: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// 注册文件通知；未启用监听时直接返回。
    pub fn watch(&self, name: String, path: &Path) {
        let changed = Arc::clone(&self.changed);
        if let Some(hotwatch) = &self.hotwatch {
            hotwatch
                .watch(path, move |event: DebouncedEvent| {
                    on_texture_event(&changed, &name, &event)
                })
                .expect("failed to watch file!");
        }
    }

    /// 变更集合句柄（消费端读取/清空；消费逻辑不在本文件三个过程里）。
    pub fn changed(&self) -> &ChangedSet {
        &self.changed
    }

    /// 是否启用了监视（`hotwatch` 为 Some）。
    pub fn is_active(&self) -> bool {
        self.hotwatch.is_some()
    }

    pub fn hotwatch(&self) -> Option<&GenericHotwatch> {
        self.hotwatch.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn names(w: &TextureWatch) -> Vec<String> {
        let mut v: Vec<String> = w
            .changed()
            .lock()
            .unwrap()
            .iter()
            .map(|r| r.as_str().to_owned())
            .collect();
        v.sort();
        v
    }

    #[test]
    fn new_without_watch_has_no_hotwatch() {
        let w = TextureWatch::new(false);
        assert!(!w.is_active());
        assert!(names(&w).is_empty());
    }

    #[test]
    fn new_with_watch_uses_100ms_delay() {
        let w = TextureWatch::new(true);
        assert!(w.is_active());
        assert_eq!(w.hotwatch().unwrap().delay, Duration::new(0, 100_000_000));
    }

    #[test]
    fn watch_without_hotwatch_is_noop() {
        let w = TextureWatch::new(false);
        w.watch("a".into(), Path::new("/x/a.png"));
        assert!(names(&w).is_empty());
        // Arc clone 已被释放：只剩 TextureWatch 自己持有
        assert_eq!(Arc::strong_count(w.changed()), 1);
    }

    #[test]
    fn create_write_rename_record_the_registered_name_not_the_path() {
        let w = TextureWatch::new(true);
        let p = Path::new("/x/a.png");
        w.watch("tex/a".into(), p);
        let hw = w.hotwatch().unwrap();
        assert_eq!(hw.watched(), 1);
        hw.fire(p, DebouncedEvent::Create(PathBuf::from("/ignored")));
        assert_eq!(names(&w), vec!["tex/a"]);
    }

    #[test]
    fn only_create_write_rename_are_handled() {
        let p = Path::new("/x/a.png");
        let q = || PathBuf::from("/x/a.png");
        let ignored = [
            DebouncedEvent::NoticeWrite(q()),
            DebouncedEvent::NoticeRemove(q()),
            DebouncedEvent::Chmod(q()),
            DebouncedEvent::Remove(q()),
            DebouncedEvent::Rescan,
            DebouncedEvent::Error("e".into(), None),
        ];
        for ev in ignored {
            let w = TextureWatch::new(true);
            w.watch("n".into(), p);
            w.hotwatch().unwrap().fire(p, ev.clone());
            assert!(names(&w).is_empty(), "{ev:?} 不应被处理");
        }
        for ev in [
            DebouncedEvent::Create(q()),
            DebouncedEvent::Write(q()),
            DebouncedEvent::Rename(q(), q()),
        ] {
            let w = TextureWatch::new(true);
            w.watch("n".into(), p);
            w.hotwatch().unwrap().fire(p, ev.clone());
            assert_eq!(names(&w), vec!["n"], "{ev:?} 应被处理");
        }
    }

    #[test]
    fn repeated_events_dedupe_and_multiple_names_accumulate() {
        let w = TextureWatch::new(true);
        let (pa, pb) = (Path::new("/a"), Path::new("/b"));
        w.watch("a".into(), pa);
        w.watch("b".into(), pb);
        let hw = w.hotwatch().unwrap();
        hw.fire(pa, DebouncedEvent::Write(PathBuf::new()));
        hw.fire(pa, DebouncedEvent::Write(PathBuf::new()));
        hw.fire(pb, DebouncedEvent::Create(PathBuf::new()));
        assert_eq!(names(&w), vec!["a", "b"]);
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn poisoned_mutex_panics_in_handler() {
        let set: Arc<Mutex<HashSet<RelativePathBuf>>> = Arc::new(Mutex::new(HashSet::new()));
        let s2 = Arc::clone(&set);
        let _ = std::thread::spawn(move || {
            let _g = s2.lock().unwrap();
            panic!("poison");
        })
        .join();
        on_texture_event(&set, "n", &DebouncedEvent::Write(PathBuf::new()));
    }
}
