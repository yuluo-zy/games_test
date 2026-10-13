//! delayed_terrain_changed_msg.rs — 延迟地形变更消息/GPU 回读（WP8 恢复）。
//!
//! 源：B skeleton `systems/terrain/delayed_terrain_changed_msg.rs`。反编译：
//! `ScheduledTerrainReadback::on_terrain_modified`@0x140aabdc0（行 40）、
//! `ScheduledTerrainReadback::on_gpu_copy_issued`（行 44，内联进 read_terrain_data@0x14098c680）、
//! `delayed_terrain_changed_msg`@0x140aabdd0（行 66）。
//! ECS 签名（O）：
//! ```
//! fn delayed_terrain_changed_msg(
//!   Res<AssetSsboLibrary>, ResMut<ScheduledTerrainReadback>, Local<DelayedUpdateWallsMsgBuffer>,
//!   EventWriter<OnTerrainChange>, ResMut<TerrainHeightsData<TerrainHeightSsbo>>,
//!   ResMut<TerrainHeightsData<VisualTerrainHeightSsbo>>)
//! ```
//! 业务（O）：每帧递减 `delays` 各项；某项到 0 → `FrozenMap::get<SsboId,Box<BufferState>>`
//! （断言尺寸 0x3a/0x40）→ cpu_readback → `copy_from_gpu`（两次：高度+视觉）；每帧至多一次。
//! flag 置位时 `delays.push(2)`（延迟 2 帧）；压实移除已完成项。

#![allow(dead_code, unused_imports)]

use std::sync::atomic::AtomicUsize;

use bevy_ecs::resource::Resource;

use crate::resources::terrain_height::{
    TerrainHeightsData, TerrainHeightSsbo, VisualTerrainHeightSsbo,
};
use crate::typed_gap::Gap;

/// 地形变更事件（O 名；register_event `OnTerrainChange`）。
#[derive(Clone, Debug, Default)]
pub struct OnTerrainChange(pub Gap);

/// 延迟更新墙消息缓冲（O 名；`Local<DelayedUpdateWallsMsgBuffer>`）。布局 U。
#[derive(Clone, Debug, Default)]
pub struct DelayedUpdateWallsMsgBuffer(pub Gap);

/// 计划地形回读资源（O 名；`ScheduledTerrainReadback`）。
/// 布局部分 O：`counter`@+0x00（on_terrain_modified 自增）、`flag`@+0x08（read_terrain_data 置位）、
/// `delays: Vec<u32>`（延迟计数列表）。
#[derive(Resource, Debug, Default)]
pub struct ScheduledTerrainReadback {
    /// 修改计数（O：on_terrain_modified `*ptr += 1`，LOCK/UNLOCK）。
    pub counter: AtomicUsize,
    /// 本帧有变更 flag（O：read_terrain_data `| lVar5 != 0` 写 +0x08）。
    pub flag: bool,
    /// 延迟计数列表（I：delayed 主循环逐项递减；到 0 触发回读）。
    pub delays: Vec<u32>,
}

impl ScheduledTerrainReadback {
    /// 地形被修改（O：`on_terrain_modified`@0x140aabdc0，行 40；counter 自增）。
    pub fn on_terrain_modified(&self) {
        self.counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    /// GPU 拷贝已发出（O：`on_gpu_copy_issued`，行 44；内联进 read_terrain_data）。
    pub fn on_gpu_copy_issued(&mut self) {
        self.flag = true;
    }
}

/// 延迟地形变更系统（O：`delayed_terrain_changed_msg`@0x140aabdd0，行 66；签名见文件头）。
pub fn delayed_terrain_changed_msg(
    _ssbo_library: &Gap,
    _readback: &mut ScheduledTerrainReadback,
    _msg_buffer: &mut DelayedUpdateWallsMsgBuffer,
    _on_change: &mut Vec<OnTerrainChange>,
    _height: &mut TerrainHeightsData<TerrainHeightSsbo>,
    _visual_height: &mut TerrainHeightsData<VisualTerrainHeightSsbo>,
) {
    unimplemented!("recover delayed_terrain_changed_msg readback scheduling + copy_from_gpu")
}
