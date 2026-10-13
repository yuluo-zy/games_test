//! texture_watch.rs — 纹理文件监视器（O）。
//!
//! 源 `asset_libraries/texture_watch.rs`；实体 `TextureWatch`。
//! `TextureWatch::new`（0x1408bab30）：可选 hotwatch + 变更队列（O）。
//! `TextureWatch::watch`（0x1408bad70）：注册监听；变更读字节入队。

#![allow(dead_code)]

use std::path::PathBuf;
use std::sync::Arc;

use bevy_ecs::resource::Resource;

use crate::reused::watch;
use crate::typed_gap::Gap;

/// 纹理监视器（O：`Res<TextureWatch>`）。
#[derive(Resource)]
pub struct TextureWatch {
    pub hotwatch: Option<watch::GenericHotwatch>,
    pub pending: Arc<std::sync::Mutex<Option<Vec<u8>>>>,
    pub _gap: Gap,
}

impl TextureWatch {
    /// `new(use_watch)`（O 0x1408bab30）。
    pub fn new(use_watch: bool) -> Self {
        TextureWatch {
            hotwatch: if use_watch {
                Some(watch::GenericHotwatch::new_with_custom_delay(std::time::Duration::from_millis(50)).unwrap_or_else(|_| watch::GenericHotwatch(crate::Gap)))
            } else {
                None
            },
            pending: Arc::new(std::sync::Mutex::new(None)),
            _gap: Gap,
        }
    }

    /// `watch(path)`（O 0x1408bad70）。
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

    /// 后端系统排空（O：texture_watch 系统）。
    pub fn take_pending(&self) -> Option<Vec<u8>> {
        self.pending.lock().ok().and_then(|mut q| q.take())
    }
}
