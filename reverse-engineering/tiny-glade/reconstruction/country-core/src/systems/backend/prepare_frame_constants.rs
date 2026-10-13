//! prepare_frame_constants.rs — 帧常量系统（O 0x140af40b0）。
//!
//! 源 `systems/backend/prepare_frame_constants.rs`。每帧组装 `FrameConstants`
//! 与 `ViewConstants`（wp4 类型）：
//!  - 帧序号推进并写 `%4`（O：`*(frame_constants+0x108) = idx & 3`；TAA jitter 表
//!    按 `USE_TAA_JITTER` 全局用帧序号采样 `%table_len`）。
//!  - `ViewportConstantBuilder::build` + `ViewConstants::set_pixel_offset`（O）。
//!  - 由相机/窗口/太阳参数投影（O 调用；字段映射 U）。

#![allow(dead_code)]

use crate::boundary::{FrameConstants, ViewConstants};

/// `prepare_frame_constants`（O 0x140af40b0）。
pub fn prepare_frame_constants(_world: &mut bevy_ecs::world::World) {
    // U：相机/窗口/时间源读取 + 投影矩阵组装（typed gap）。
}

/// 前/后帧常量对（O：系统读写两个 0x100 字节帧常量缓冲）。
pub struct FrameConstantsPair {
    pub previous: FrameConstants,
    pub current: FrameConstants,
}

impl FrameConstantsPair {
    pub fn new() -> Self {
        FrameConstantsPair {
            previous: FrameConstants {
                frame_index: 0,
                _gap: crate::Gap,
            },
            current: FrameConstants {
                frame_index: 0,
                _gap: crate::Gap,
            },
        }
    }

    /// 帧序号推进（O：写入 `& 3`）。
    pub fn advance_frame_index(&mut self) -> u32 {
        let next = self.current.frame_index.wrapping_add(1) & 3;
        self.previous.frame_index = self.current.frame_index;
        self.current.frame_index = next;
        next
    }
}

/// 视口常量构建入口（O：`ViewportConstantBuilder::build` 内联）。
pub struct ViewportConstantBuilder;

impl ViewportConstantBuilder {
    pub fn build(&self, _view: &mut ViewConstants) {}
}
