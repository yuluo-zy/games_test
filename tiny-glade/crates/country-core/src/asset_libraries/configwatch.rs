//! `ConfigWatch` —— 配置文件热重载监视器（layer A3）。
//!
//! 证据（O = 机器码直接观察；I = 推断；U = 未知；详见 `evidence/country-core-audit/A3_shader.md`）。
//! 原版 3 个过程：
//!  * `ConfigWatch::new`        0x1408ba0f0（405 B）
//!  * `ConfigWatch::watch`      0x1408ba290（499 B）
//!  * `watch::{closure#0}`      0x14091e430（875 B）——注册给 hotwatch 的事件处理器
//!
//! 布局（O，0x38 B）：`+0x00..0x30` = `Option<hotwatch::GenericHotwatch<ReadDirectoryChangesWatcher>>`
//! （首个 dword == 3 即 None）；`+0x30` = `Arc<std::sync::Mutex<Option<String>>>`
//! （`Arc` 块 0x30 B：强/弱计数 = 1/1，`Mutex` futex 字节 +0x10 = 0、poison 字节 +0x11 = 0，
//! `Option<String>` +0x18 = 容量 niche `i64::MIN` = None）。
//!
//! 与旧版（臆造）的主要差异：
//!  1. 待处理队列是 `Mutex<Option<String>>`——**变更文件的路径字符串（注册时的路径）**，不是 `Option<Vec<u8>>` 文件字节；
//!     处理器**不读文件**，只记录路径（`system::backend::configwatch` 取走后调用 `update_color_library`）；
//!  2. `new(use_watch)`：延迟参数 `Duration { secs: 0, nanos: 0x5f5e100 }` = **100 ms**（旧版写 50 ms）；
//!     `new_with_custom_delay` 失败 → `expect("hotwatch failed to initialize")`（旧版 `unwrap_or_else` 吞错误并造空壳）；
//!     `use_watch == false` → `None`，不创建 watcher；
//!  3. `watch(path: String)`：`Arc` 克隆（计数溢出即 abort，标准库行为）+ `String::clone`；watcher 为 `None` →
//!     静默丢弃（不报错）；否则 `hotwatch.watch(path, closure).expect("failed to watch file!")`，
//!     闭包捕获 `{ watched: String（路径克隆）, pending: Arc }`；
//!  4. 处理器只对 `notify::DebouncedEvent` 的 **第 4 个变体（`Write`）** 起作用（判别量 `0x8000000000000003`，I：notify 4.x
//!     变体序 NoticeWrite/NoticeRemove/Create/Write/…，`Rename` 带两个 PathBuf 占据 niche 载体），其余变体直接 drop。
//!     `Write` 时：先 `info!("Configwatch: {watched} has changed")`（target = 模块路径，pieces `"Configwatch: "` +
//!     `" has changed"`），再 `pending.lock().unwrap()`（poison → `Result::unwrap` panic），
//!     `*guard = Some(watched.clone())`（旧值 drop），最后 drop 事件里的 PathBuf（**事件自带的路径不使用**）。
//!
//! 外部库：`hotwatch` / `notify` 在 `crate::reused::watch`（占位，其回调只给 `PathBuf`，这里把它当作 `Write` 事件）。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::single_match)]
#![allow(dead_code)]

#[cfg(test)]
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use bevy_ecs::resource::Resource;

use crate::reused::watch;

/// `watch` 处理器日志用的 target（O：模块路径，长 0x2a）。
pub const LOG_TARGET: &str = "country_core::asset_libraries::configwatch";

/// 注册给 hotwatch 的去抖延迟（O：`r8d = 0x5f5e100` ns）。
pub const DEBOUNCE_DELAY: Duration = Duration::from_millis(100);

/// Original notify event type, re-exported by the original hotwatch fork.
pub use hotwatch::Event as DebouncedEvent;

/// 待处理队列：最近一次变更文件的路径（O：`Arc<Mutex<Option<String>>>`）。
pub type Pending = Arc<Mutex<Option<String>>>;

/// 配置监视器（O：`Res<ConfigWatch>`，0x38 B）。
#[derive(Resource)]
pub struct ConfigWatch {
    /// `+0x00`：`Option<GenericHotwatch>`（`use_watch` 为假时 None）。
    pub hotwatch: Option<watch::GenericHotwatch>,
    /// `+0x30`。
    pub pending: Pending,
}

/// 处理器闭包体（O 0x14091e430）。`watched` 是闭包捕获的注册路径。
pub fn on_event(event: DebouncedEvent, watched: &str, pending: &Mutex<Option<String>>) {
    match event {
        DebouncedEvent::Write(_event_path) => {
            log::info!(target: LOG_TARGET, "Configwatch: {} has changed", watched);
            // O：`lock()` 之后 poison 字节非 0 → `unwrap_failed("called `Result::unwrap()` on an `Err` value")`。
            let mut guard = pending.lock().unwrap();
            *guard = Some(watched.to_owned());
        }
        // O：其余变体只 `drop_in_place<DebouncedEvent>`。
        _ => {}
    }
}

impl ConfigWatch {
    /// `new(use_watch)`（O 0x1408ba0f0）。
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

    /// `watch(path)`（O 0x1408ba290）：见模块注释第 3 点。
    pub fn watch(&mut self, path: String) {
        let pending = Arc::clone(&self.pending);
        let watched = path.clone();
        let Some(hotwatch) = &self.hotwatch else {
            // O：`cmp dword [self], 3` 命中 → 释放克隆出的 String / Arc / 入参，直接返回。
            return;
        };
        hotwatch
            .watch(path, move |event| {
                on_event(event, &watched, &pending);
            })
            .expect("failed to watch file!");
    }

    /// 取走待处理路径（I：消费者 `systems::backend::configwatch`（0x14085b1c0）对同一把锁做
    /// `lock().unwrap()` + `Option::take`；非 `ConfigWatch` 自有过程）。
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
            DebouncedEvent::Create("a".into()),
            DebouncedEvent::Chmod("a".into()),
            DebouncedEvent::Remove("a".into()),
            DebouncedEvent::Rename("a".into(), "b".into()),
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
