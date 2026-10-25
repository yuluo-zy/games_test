//! 配置热重载通知。回调只保存注册路径，文件读取在游戏线程执行。
//! 原作来源见项目 evidence；当前接口、行为变化与验证范围见 README.md。

#![allow(clippy::single_match)]
#![allow(dead_code)]

#[cfg(test)]
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use bevy_ecs::resource::Resource;

use crate::reused::watch;

pub const LOG_TARGET: &str = "country_core::asset_libraries::configwatch";

pub const DEBOUNCE_DELAY: Duration = Duration::from_millis(100);

pub use hotwatch::Event as DebouncedEvent;

pub type Pending = Arc<Mutex<Option<String>>>;

/// 单槽配置通知；同批多次变化只需重新读取最终文件。
#[derive(Resource)]
pub struct ConfigWatch {
    pub hotwatch: Option<watch::GenericHotwatch>,
    pub pending: Pending,
}

/// 写入事件记录注册路径；其他事件不触发配置更新。
pub fn on_event(event: DebouncedEvent, watched: &str, pending: &Mutex<Option<String>>) {
    match event {
        DebouncedEvent::Write(_event_path)
        | DebouncedEvent::Create(_event_path)
        | DebouncedEvent::Rename(_, _event_path) => {
            log::info!(target: LOG_TARGET, "Configwatch: {} has changed", watched);
            let mut guard = pending.lock().unwrap();
            *guard = Some(watched.to_owned());
        }
        _ => {}
    }
}

impl ConfigWatch {
    /// 创建初始状态。
    pub fn new(use_watch: bool) -> Self {
        let hotwatch = if use_watch {
            Some(
                watch::GenericHotwatch::new_with_custom_delay(DEBOUNCE_DELAY)
                    .expect("hotwatch failed to initialize"),
            )
        } else {
            None
        };
        ConfigWatch {
            hotwatch,
            pending: Arc::new(Mutex::new(None)),
        }
    }

    /// 注册文件通知；未启用监听时直接返回。
    pub fn watch(&mut self, path: String) {
        let pending = Arc::clone(&self.pending);
        let watched = path.clone();
        let Some(hotwatch) = &self.hotwatch else {
            return;
        };
        hotwatch
            .watch(path, move |event| {
                on_event(event, &watched, &pending);
            })
            .expect("failed to watch file!");
    }

    pub fn take_pending(&self) -> Option<String> {
        self.pending.lock().unwrap().take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_without_watch_has_no_hotwatch_and_empty_queue() {
        let cw = ConfigWatch::new(false);
        assert!(cw.hotwatch.is_none());
        assert_eq!(*cw.pending.lock().unwrap(), None);
    }

    #[test]
    fn new_with_watch_creates_hotwatch() {
        assert!(ConfigWatch::new(true).hotwatch.is_some());
    }

    #[test]
    fn debounce_is_100ms() {
        assert_eq!(DEBOUNCE_DELAY.as_secs(), 0);
        assert_eq!(DEBOUNCE_DELAY.subsec_nanos(), 0x5f5e100);
    }

    #[test]
    fn watch_without_hotwatch_is_silent_noop() {
        let mut cw = ConfigWatch::new(false);
        cw.watch("assets/colors.json".to_owned());
        assert_eq!(cw.take_pending(), None);
        // Arc 克隆已释放。
        assert_eq!(Arc::strong_count(&cw.pending), 1);
    }

    #[test]
    fn watch_with_hotwatch_registers_without_panicking() {
        let path = std::env::temp_dir().join(format!(
            "tiny-glade-configwatch-{}.json",
            std::process::id()
        ));
        std::fs::write(&path, b"{}").unwrap();
        let mut cw = ConfigWatch::new(true);
        cw.watch(path.to_str().unwrap().to_owned());
        drop(cw);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn write_event_records_registered_path_not_event_path() {
        let q = Mutex::new(None);
        on_event(
            DebouncedEvent::Write(PathBuf::from("C:/x/other.tmp")),
            "assets/colors.json",
            &q,
        );
        assert_eq!(q.lock().unwrap().as_deref(), Some("assets/colors.json"));
    }

    #[test]
    fn non_write_events_are_ignored() {
        let q = Mutex::new(None);
        for ev in [
            DebouncedEvent::NoticeWrite("a".into()),
            DebouncedEvent::NoticeRemove("a".into()),
            DebouncedEvent::Chmod("a".into()),
            DebouncedEvent::Remove("a".into()),
            DebouncedEvent::Rescan,
            DebouncedEvent::Error(
                hotwatch::notify::Error::Io(std::io::Error::other("e")),
                None,
            ),
        ] {
            on_event(ev, "w", &q);
        }
        assert_eq!(*q.lock().unwrap(), None);
    }

    #[test]
    fn repeated_writes_overwrite_single_slot() {
        let q = Mutex::new(None);
        on_event(DebouncedEvent::Write("a".into()), "first", &q);
        on_event(DebouncedEvent::Write("a".into()), "second", &q);
        assert_eq!(q.lock().unwrap().take().as_deref(), Some("second"));
        assert_eq!(*q.lock().unwrap(), None);
    }

    #[test]
    fn take_pending_drains() {
        let cw = ConfigWatch::new(false);
        on_event(DebouncedEvent::Write("a".into()), "p", &cw.pending);
        assert_eq!(cw.take_pending().as_deref(), Some("p"));
        assert_eq!(cw.take_pending(), None);
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]
    fn poisoned_queue_panics_on_write() {
        let q = Arc::new(Mutex::new(None));
        let q2 = Arc::clone(&q);
        let _ = std::thread::spawn(move || {
            let _g = q2.lock().unwrap();
            panic!("poison");
        })
        .join();
        on_event(DebouncedEvent::Write("a".into()), "p", &q);
    }
}
