//! game_frame_index.rs — 帧序号（O：`game_frame_index` 全内联，无独立物理函数）。
//!
//! 源 `systems/backend/game_frame_index.rs`（dependencies.json 记录函数
//! `game_frame_index`，inline_instances [560, 40168]/[1611, 2310144]）。
//! 每帧递增 `GameFrameIndex`；`prepare_frame_constants` 读 `&3`（O）。

#![allow(dead_code)]

use bevy_ecs::resource::Resource;

/// 帧序号资源（O：`Res<GameFrameIndex>`；每帧 +1，写入 frame const `&3`）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct GameFrameIndex(pub u32);

impl GameFrameIndex {
    pub fn new() -> Self {
        GameFrameIndex(0)
    }
    /// `game_frame_index`（O）：帧推进。
    pub fn next(&mut self) -> u32 {
        self.0 = self.0.wrapping_add(1);
        self.0
    }
}
