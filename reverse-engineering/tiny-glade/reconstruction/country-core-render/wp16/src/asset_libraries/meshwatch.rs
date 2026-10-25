//! meshwatch.rs — 网格文件监视器（O）。
//!
//! 源 `asset_libraries/meshwatch.rs`；实体 `MeshWatch`。
//! `MeshWatch::new`（0x1408ba490）：可选 hotwatch + `Arc<Mutex<Option<Vec<u8>>>>`
//! 变更字节队列（O）。
//! `MeshWatch::watch`（0x1408ba6e0）：注册监听；变更读文件字节入队
//! （`_Size/_Src` 拷贝 O）。

#![allow(dead_code)]

use std::path::PathBuf;
use std::sync::Arc;

use bevy_ecs::resource::Resource;

use crate::reused::watch;
use crate::typed_gap::Gap;

/// 网格监视器（O：`Res<MeshWatch>`）。
#[derive(Resource)]
pub struct MeshWatch {
    pub hotwatch: Option<watch::GenericHotwatch>,
    pub pending: Arc<std::sync::Mutex<Option<Vec<u8>>>>,
    pub _gap: Gap,
}

impl MeshWatch {
    /// `new(use_watch)`（O 0x1408ba490）。
    pub fn new(use_watch: bool) -> Self {
        MeshWatch {
            hotwatch: if use_watch {
                Some(watch::GenericHotwatch::new_with_custom_delay(std::time::Duration::from_millis(50)).unwrap_or_else(|_| watch::GenericHotwatch(crate::Gap)))
            } else {
                None
            },
            pending: Arc::new(std::sync::Mutex::new(None)),
            _gap: Gap,
        }
    }

    /// `watch(path)`（O 0x1408ba6e0）。
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

    /// 后端系统排空（O：meshwatch 系统）。
    pub fn take_pending(&self) -> Option<Vec<u8>> {
        self.pending.lock().ok().and_then(|mut q| q.take())
    }
}
