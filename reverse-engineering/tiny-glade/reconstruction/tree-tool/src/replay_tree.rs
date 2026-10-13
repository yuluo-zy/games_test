//! tree-tool：replay_tree 业务恢复草稿（B 主基线，反汇编级证据）
//!
//! 依据：evidence/multi-baseline/tree-tool-rest-analysis.md（2026-10-13 reva/radare2
//! 精读 0x141bc34f0，448 条指令）。
//!
//! 硬证据：
//! - plugin build（0x140650520）in_set<GameSystemSet> 注册签名 =
//!   ResMut<ReplayBuffer>, EventWriter<InsertTreeCmd>, EventWriter<MoveTreeCmd>,
//!   EventWriter<DeleteTreeCmd>, Res<GameTime>。
//! - ReplayCmd 槽 stride 0x30；tag @+0=8 为 TreeEdit；TreeEdit 变体 @+0x10：
//!   1=Move、2=Insert、其余=Delete。
//! - 事件槽布局（MoveTreeCmd 0x20 / DeleteTreeCmd 0x10 / InsertTreeCmd 0x48）与
//!   process_tree_cmds 一致；本函数 Insert 槽从 slot+0 直接写 payload
//!   （bevy id 不入槽——印证框架 id 放置不统一，依赖内部）。
//!
//! 依赖复用：bevy_ecs 0.16 直接复用。ReplayBuffer 为 country_core::
//! resources::history::replay_cmd_buffer::ReplayBuffer；TreeEdit 为
//! country_core::resources::history::tree_edit::TreeEdit（serde 序列化，字段部分
//! unknown）。GameTime delta 证据：f32 @+0x28（两处独立调用点一致）。

use bevy_ecs::change_detection::{Res, ResMut};
use bevy_ecs::event::EventWriter;
use bevy_ecs::prelude::*;
use glam::Vec3;

// ---- country_core::resources::history::replay_cmd_buffer ----

/// 历史回放命令缓冲资源（ReplayBuffer）。
/// `front_mut_bucket()`（0x1409f1a80）取前端桶（Vec<ReplayCmd>）。
/// [evidence] f32 @+0x28 = 延时除数（回放倍速，语义 unknown）。
pub struct ReplayBuffer {
    /// 内部双缓冲桶（front/back；细节 unknown）。
    pub _private: [u8; 0x28],
    /// [evidence] f32 @+0x28：ReplayCmd.delay 的除数（回放倍速，语义 unknown）。
    pub replay_time_scale: f32,
}

impl ReplayBuffer {
    /// [evidence] 0x1409f1a80；返回前端可变桶，空时返回 None。
    fn front_mut_bucket(&mut self) -> Option<ReplayCmdBucket<'_>> {
        None // 依赖内部，不还原
    }
}

/// 回放命令桶（Vec<ReplayCmd> 的轻量视图）。
pub struct ReplayCmdBucket<'a> {
    pub _marker: std::marker::PhantomData<&'a mut ()>,
}

impl<'a> ReplayCmdBucket<'a> {
    fn iter_mut(&mut self) -> impl Iterator<Item = (usize, &mut ReplayCmd)> {
        std::iter::empty()
    }
    fn remove(&mut self, _index: usize) -> ReplayCmd {
        unreachable!()
    }
}

/// 回放命令槽（stride 0x30，反汇编证据）。
/// tag 8 = TreeEdit（回放树编辑）；其余 tag（0–9）为墙/顶/地形/装饰等编辑
/// （本系统仅处理 8，其余留给各自回放系统）。
pub struct ReplayCmd {
    /// tag 字节 @+0（8 = TreeEdit）。
    pub tag: u8,
    /// flags 字节 @+8（bit0 = 延时回放；0 = 立即移除）。
    pub flags: u8,
    /// f32 @+0xc：延时（除以 ReplayBuffer+0x28，下钳 0.05）。
    pub delay: f32,
    /// u64 @+0x10：TreeEdit 负载指针。
    pub payload: *const TreeEdit,
    /// f32 @+0x18：已累积回放时长（elapsed）。
    pub elapsed: f32,
    /// byte @+0x1c：flag（选两种负载布局；语义 unknown）。
    pub layout_flag: u8,
}

// ---- country_core::resources::history::tree_edit ----

/// 树编辑（serde 序列化历史负载）。变体 @+0x10：
/// 1 = Move、2 = Insert、其余 = Delete。字段布局部分证据，其余 unknown。
pub struct TreeEdit {
    /// [evidence] 变体 u32 @+0x10（1=Move, 2=Insert, 其余=Delete）。
    pub variant: u32,
    /// 各变体字段按证据偏移读取，见 analysis 文档：
    /// - Move：index @+0x30（或 @+0x14）；position @+0x20/+0x28（或 @+0x14/+0x1c）。
    /// - Insert：name（String）、index @+0x18 等。
    /// - Delete：index @+0x18。
    pub unknown: [u8; 0x60],
}

// ---- country_core::resources::tree_storage（命令事件）----

/// 移动树命令。事件槽 0x20；payload 20B：index u64 @+8，position Vec3 @+0x10
/// （@+0 为 bevy 事件 id）。与 process_tree_cmds 布局一致。
pub struct MoveTreeCmd {
    pub index: u64,
    pub position: Vec3,
}

/// 删除树命令。事件槽 0x10；payload 8B：index u64 @+8。
pub struct DeleteTreeCmd {
    pub index: u64,
}

/// 插入树命令。事件槽 0x48；payload 字段与 process_tree_cmds 的 Insert 分支一致，
/// 本函数从 slot+0 直接写 payload（bevy id 不入槽）。
pub struct InsertTreeCmd {
    /// 树 prefab 名（String：ptr/len，本函数经 String::clone 复制）。
    pub name: String,
    /// 位置。
    pub position: Vec3,
    /// 其余字段（高度/标志/索引等）布局同 process_tree_cmds；本轮未逐项对齐。
    pub unknown: [u8; 0x38],
}

/// 回放 TreeEdit 历史命令到游戏事件。
///
/// 签名来自 plugin build 注册（硬证据）。PDB size 2722（0xAA2），
/// 源文件 replay_tree.rs 17–121 行。
pub fn replay_tree(
    mut replay_buffer: ResMut<ReplayBuffer>,
    mut insert_cmds: EventWriter<InsertTreeCmd>,
    mut move_cmds: EventWriter<MoveTreeCmd>,
    mut delete_cmds: EventWriter<DeleteTreeCmd>,
    time: Res<GameTime>,
) {
    // 1) ReplayBuffer::front_mut_bucket() → 空则返回（反汇编 0x141bc355d）。
    let Some(mut bucket) = replay_buffer.front_mut_bucket() else {
        return;
    };

    // 2) 逐 ReplayCmd（stride 0x30），只处理 tag 8（TreeEdit）。
    // 3) 变体 @+0x10：1/2/else → Move/Insert/Delete。
    // 4) 若 elapsed == 0.0 → 重发事件（index/position/name 来自 TreeEdit 字段）。
    // 5) 时序记账：flags bit0==0 立即移除；否则 delay 到期后移除，elapsed += delta。
    // 6) 循环后逆序移除已处理 ReplayCmd（Vec::remove + 按 tag drop）。
    let mut to_remove: Vec<usize> = Vec::new();

    for (idx, cmd) in bucket.iter_mut().enumerate() {
        if cmd.tag != 8 {
            continue; // 非 TreeEdit 留给其他回放系统
        }
        let edit = unsafe { &*cmd.payload };
        if cmd.elapsed == 0.0 {
            match edit.variant {
                1 => {
                    // [evidence] Move：index u64 @+0x30；position @+0x20（flag==0）。
                    let index = read_u64(&edit.unknown, 0x30);
                    move_cmds.send(MoveTreeCmd {
                        index,
                        position: Vec3::ZERO, // [evidence] position 字段偏移见 TreeEdit（unknown）
                    });
                }
                2 => {
                    // [evidence] Insert：槽 0x48，名字经 String::clone（0x140d7bd90）。
                    let _ = &insert_cmds;
                    insert_cmds.send(InsertTreeCmd {
                        name: String::new(), // [evidence] 复制 TreeEdit 名字；字段偏移 unknown
                        position: Vec3::ZERO,
                        unknown: [0; 0x38],
                    });
                }
                _ => {
                    // [evidence] Delete：index u64 @+0x18。
                    let index = read_u64(&edit.unknown, 0x18);
                    delete_cmds.send(DeleteTreeCmd { index });
                }
            }
        }
        // [evidence] 时序记账 + 移除集合（反汇编 0x141bc39cc 起）
        if cmd.flags & 1 == 0 {
            to_remove.push(idx);
        } else {
            let mut delay = cmd.delay / replay_buffer.replay_time_scale;
            if delay <= 0.05 {
                delay = 0.05;
            }
            if delay < cmd.elapsed {
                to_remove.push(idx);
            } else {
                cmd.elapsed += time.delta_seconds;
            }
        }
    }

    // 6) 逆序移除（反汇编 0x141bc3b20 起；assert_failed/remove/memmove/drop_slow）
    for idx in to_remove.into_iter().rev() {
        let _ = bucket.remove(idx); // [evidence] Vec::remove + 按 tag 释放负载
    }
}

/// [evidence] 读 ReplayCmd 偏移处的 u64（字节布局按 analysis 文档）。
fn read_u64(_slot: &[u8; 0x60], _offset: usize) -> u64 {
    0 // 依赖布局 unknown，草稿占位
}

// ---- country_core::resources::time ----

/// 游戏时间资源。本轮证据：f32 @+0x28 = delta（tree_move_bend 的
/// time_continuous_lerp 参数与 replay_tree elapsed 累积两处独立一致）。
pub struct GameTime {
    pub _private: [u8; 0x28],
    pub delta_seconds: f32,
}
