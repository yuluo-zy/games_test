//! prepare_frame_constants.rs — 帧常量系统（O 0x140af40b0）。
//!
//! 源 `systems/backend/prepare_frame_constants.rs`。每帧组装 `FrameConstants`
//! 与 `ViewConstants`（wp4 类型）：
//!  - 帧序号推进并写 `%4`（O：`*(frame_constants+0x108) = idx & 3`；TAA jitter 表
//!    按 `USE_TAA_JITTER` 全局用帧序号采样 `%table_len`）。
//!  - `ViewportConstantBuilder::build` + `ViewConstants::set_pixel_offset`（O）。
//!  - 由相机/窗口/太阳参数投影（O 调用；字段映射 U）。

#![allow(dead_code)]

use crate::render::frame_constants::FrameConstants;

/// `prepare_frame_constants`（O 0x140af40b0）。
pub fn prepare_frame_constants(_world: &mut bevy_ecs::world::World) {
    // U：相机/窗口/时间源读取 + 投影矩阵组装（typed gap）。
}

/// 前/后帧常量对（O：系统读写两个帧常量缓冲）。
///
/// `frame_index` 是 `& 3` 的循环序号（O：`idx & 3`，TAA jitter 采样用）；它在原版里落在
/// `FrameConstants` 的哪个偏移为 U（`views` 起点 0xf0 为 I，见 `render::frame_constants`），
/// 因此不写进 `FrameConstants` 本体，由本结构单独携带。
pub struct FrameConstantsPair {
    pub previous: FrameConstants,
    pub current: FrameConstants,
    pub previous_frame_index: u32,
    pub frame_index: u32,
}

impl FrameConstantsPair {
    pub fn new() -> Self {
        FrameConstantsPair {
            previous: FrameConstants::default(),
            current: FrameConstants::default(),
            previous_frame_index: 0,
            frame_index: 0,
        }
    }

    /// 帧序号推进（O：写入 `& 3`）。
    pub fn advance_frame_index(&mut self) -> u32 {
        let next = self.frame_index.wrapping_add(1) & 3;
        self.previous_frame_index = self.frame_index;
        self.frame_index = next;
        next
    }
}

pub use crate::render::view_constants::ViewportConstantBuilder;
