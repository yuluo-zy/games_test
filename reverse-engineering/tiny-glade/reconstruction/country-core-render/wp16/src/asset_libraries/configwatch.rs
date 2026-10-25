//! configwatch.rs — 配置文件监视器（O）。
//!
//! 源 `asset_libraries/configwatch.rs`；实体 `ConfigWatch`。
//! `ConfigWatch::new`（0x1408ba0f0）：可选 `hotwatch::GenericHotwatch`
//! （`new_with_custom_delay<ReadDirectoryChangesWatcher>`）+
//! `Arc<Mutex<Option<Vec<u8>>>>`（变更文件字节队列，O）。
//! `ConfigWatch::watch`（0x1408ba290）：注册路径监听；变更 → 读字节入队。

#![allow(dead_code)]

use std::path::PathBuf;
use std::sync::Arc;

use bevy_ecs::resource::Resource;

use crate::reused::watch;
use crate::typed_gap::Gap;

/// 配置监视器（O：`Res<ConfigWatch>`）。
#[derive(Resource)]
pub struct ConfigWatch {
    /// hotwatch（O：`Option<GenericHotwatch>`，`in_DL != 0` 时创建）。
    pub hotwatch: Option<watch::GenericHotwatch>,
    /// 变更文件字节队列（O：`Arc<Mutex<Option<Vec<u8>>>>`，watch 闭包写）。
    pub pending: Arc<std::sync::Mutex<Option<Vec<u8>>>>,
    pub _gap: Gap,
}

impl ConfigWatch {
    /// `new(use_watch)`（O 0x1408ba0f0）。
    pub fn new(use_watch: bool) -> Self {
        ConfigWatch {
            hotwatch: if use_watch {
                Some(watch::GenericHotwatch::new_with_custom_delay(std::time::Duration::from_millis(50)).unwrap_or_else(|_| watch::GenericHotwatch(crate::Gap)))
            } else {
                None
            },
            pending: Arc::new(std::sync::Mutex::new(None)),
            _gap: Gap,
        }
    }

    /// `watch(path)`（O 0x1408ba290）：注册监听；变更读字节入队。
    pub fn watch(&self, path: PathBuf) {
        if let Some(hw) = &self.hotwatch {
            let pending = self.pending.clone();
            let _ = hw.watch(path, move |p| {
                if let Ok(bytes) = std::fs::read(&p) {
                    if let Ok(mut q) = pending.lock() {
                        *q = Some(bytes);
                    }
                }
            });
        }
    }

    /// 后端系统排空（O：configwatch 系统）。
    pub fn take_pending(&self) -> Option<Vec<u8>> {
        self.pending.lock().ok().and_then(|mut q| q.take())
    }
}
