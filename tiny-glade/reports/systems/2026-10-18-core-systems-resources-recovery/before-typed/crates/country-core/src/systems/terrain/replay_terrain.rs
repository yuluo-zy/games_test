//! `replay_terrain.rs` — 地形回放（B 0x140995150，O；`TerrainReplayCmd::forward/backward`
//! 0x140994f30 / 0x140995040，O）。
//!
//! 证据：`terrain-water/decomp/decomp-0x140995150.txt`（365 行）、
//! `decomp-0x140994f30.txt`（forward）、`decomp-0x140995040.txt`（backward 同构）、
//! RNE 常量直读：默认时长 0.3（`_UNK_1429258d8`=0x3e99999a）、clamp 下限 0.4
//! （`_UNK_142925970`=0x3ecccccd）、系数 0.01（`_UNK_142960674`=0x3c23d70a）、
//! 上限 3.0（`_UNK_1429cd078`=0x40400000）、1.0（`_UNK_142925904`=0x3f800000）。
//!
//! decomp 结构（replay_terrain）：
//! - 门：ReplayBuffer `+0x18` 活动计数 == 0 → return；speed = `+0x28`；
//! - 前桶 cmds（ptr@+0x08, len@+0x10）0x30 步长迭代，**仅 tag 3**；载荷 = TerrainEdit
//!   （niche @+0x10 判变体，变体字段 @+0x18 起：点集 ptr/len @+0x20/+0x28、stroke id @+0x34）；
//! - **instant 路径**（`duration(+0x10)==0.0 且非 NaN 且 undo(+0x1c)==0`）：拷贝点集
//!   （12B/点，断言点数 < 0x3fffff @0x142aae990）→ 更新地形 stroke 计数（+0x38 =
//!   max(索引+1, 原值)）→ `FUN_141526d10` 插入 TerrainEditingData → 释放拷贝；构造默认
//!   命令 {ptr, 0.4f（0x3ecccccd）, 2, stroke_id}；
//! - **动画路径**：`t = clamp(elapsed/duration, 0, 1)`；`eased = 1-(1-t)^3`
//!   （ease_out_cubic；duration < 0.4 时 eased = 1.0 立即应用）；`undo==0 → reapply
//!   0x1408592d0(payload+0x10, …, eased)`，`!=0 → revert 0x140858d30`；`eased >= 1.0`
//!   完成入列表；`elapsed += dt*speed`（+0x10）。
//!
//! **共享边界**：revert/reapply（0x140858d30/0x1408592d0，CurveU 进度更新）属共享
//! `resources/history/terrain_edit` owner（本章 TerrainEdit 仅为延迟历史记录
//! `{pending, previous_state}`，无点集载荷）；本模块以 [`TerrainReplayCmd`] 自带
//! `TerrainStroke` 点集 + 首帧完整 revert/渐进 reapply 的 CPU 侧模型落地，共享
//! revert/reapply 语义待该 owner 恢复（待取证/仲裁）。`ReplayCommand::Terrain` 变体接入
//! 共享 `replay_cmd_buffer.rs` 亦经仲裁。
use bevy_ecs::prelude::*;
use std::sync::Arc;

use crate::{
    resources::{
        history::{ReplayBuffer, ReplayCommand},
        terrain_editing::{StrokeId, TerrainEditingData},
        time::GameTime,
    },
    systems::terrain::deferred_terrain_history::TerrainEdit,
};
use tg_terrain::stroke::TerrainStroke;

/// 默认时长（O：`_UNK_1429258d8` = 0.3f；非 NewStroke 变体）。
pub const DEFAULT_DURATION: f32 = 0.3;
/// duration clamp 下限（O：`_UNK_142925970` = 0.4f）。
pub const DURATION_MIN: f32 = 0.4;
/// 时长系数（O：`_UNK_142960674` = 0.01f，cm→m）。
pub const DURATION_FACTOR: f32 = 0.01;
/// duration clamp 上限（O：`_UNK_1429cd078` = 3.0f）。
pub const DURATION_MAX: f32 = 3.0;
/// 动画阈值（O：duration < 0.4 时 eased = 1.0 立即应用）。
pub const ANIMATION_THRESHOLD: f32 = 0.4;

/// 地形回放命令（O：forward/backward 输出 {data@0, +0x8=0, duration@+0xc, +0x10=2,
/// +0x14=undo}；0x30 步长前桶条目 tag 3 判别）。
#[derive(Clone, Debug)]
pub struct TerrainReplayCmd {
    /// 回放的地形笔画（点集 + falloff + height_ws；O 载荷源）。
    pub stroke: Arc<TerrainStroke>,
    /// 目标 stroke id（O：变体 @+0x34）。
    pub stroke_id: StrokeId,
    /// 已播放时间（+0x10 槽）。
    pub elapsed: f32,
    /// 时长（+0x0c）。
    pub duration: f32,
    /// undo（+0x14 字节；1 = revert）。
    pub undo: bool,
}

impl TerrainReplayCmd {
    /// `forward` 0x140994f30：duration = clamp(Σ相邻点距*0.01, 0.4, 3.0)，无点 0.3；
    /// 断言 `stroke_length.is_finite()`（@0x142a94c34）。
    pub fn forward(stroke: Arc<TerrainStroke>, stroke_id: StrokeId) -> Self {
        Self::new(stroke, stroke_id, false)
    }
    /// `backward` 0x140995040（同构）：undo = true。
    pub fn backward(stroke: Arc<TerrainStroke>, stroke_id: StrokeId) -> Self {
        Self::new(stroke, stroke_id, true)
    }
    fn new(stroke: Arc<TerrainStroke>, stroke_id: StrokeId, undo: bool) -> Self {
        let duration = if stroke.points.is_empty() {
            DEFAULT_DURATION
        } else {
            let length = adjacent_distance_sum(&stroke.points);
            assert!(
                length.is_finite(),
                "assertion failed: stroke_length.is_finite()"
            );
            (length * DURATION_FACTOR)
                .clamp(DURATION_MIN, DURATION_MAX)
        };
        Self {
            stroke,
            stroke_id,
            elapsed: 0.0,
            duration,
            undo,
        }
    }
    /// 动画路径进度（O：`t = clamp(elapsed/duration, 0, 1)`；`eased = 1-(1-t)^3`；
    /// duration < 0.4 时 eased = 1.0 立即应用）。
    pub fn eased(&self) -> f32 {
        if self.duration >= ANIMATION_THRESHOLD && self.duration > 0.0 && self.duration.is_finite() {
            let t = (self.elapsed / self.duration).clamp(0.0, 1.0);
            1.0 - (1.0 - t).powi(3)
        } else {
            1.0
        }
    }
    /// `make_instant`（O：ReplayBuffer 动画立即化；duration 置 0 → instant 路径）。
    pub fn make_instant(&mut self) {
        self.duration = 0.0;
    }
}

/// Σ相邻点距（O：`FUN_141476710`，pair-unrolled；点对 [f32;3]，本实现逐相邻点求和）。
pub fn adjacent_distance_sum(points: &[[f32; 3]]) -> f32 {
    let distance = |a: [f32; 3], b: [f32; 3]| {
        let dx = a[0] - b[0];
        let dy = a[1] - b[1];
        let dz = a[2] - b[2];
        (dz * dz + (dy * dy + dx * dx)).sqrt()
    };
    points
        .windows(2)
        .map(|pair| distance(pair[0], pair[1]))
        .sum()
}

/// `replay_terrain` 0x140995150（O 语义；CPU 侧模型。共享 revert/reapply 待 owner）。
///
/// 消费共享 `ReplayBuffer` 前桶中的 `ReplayCommand::Terrain`（中央仲裁接入
/// `replay_cmd_buffer.rs`）；按完成序逆序移除（O 收尾同通用骨架）。
pub fn replay_terrain(
    mut replay: ResMut<ReplayBuffer>,
    time: Res<GameTime>,
    mut editing: ResMut<TerrainEditingData>,
) {
    let speed = replay.speed;
    let Some(bucket) = replay.front_mut_bucket() else {
        return;
    };
    let dt = time.delta;
    bucket.commands.retain_mut(|command| {
        let ReplayCommand::Terrain(command) = command else {
            return true;
        };
        !advance_terrain_replay(command, &mut editing, speed, dt)
    });
}

/// 由延迟地形历史记录构建回放命令（I：`TerrainEdit` 载荷 {pending, previous_state}
/// 语义 U；此处仅登记转换入口，点集来源待共享 TerrainEdit owner）。
pub fn terrain_replay_from_edit(
    _edit: &TerrainEdit,
    _stroke_id: StrokeId,
) -> Option<TerrainReplayCmd> {
    None
}

/// 单条地形回放命令推进（O 语义：instant/动画两路径）。///
/// - **instant**（`duration==0.0` 且非 NaN 且 `undo==0`）：整条点集插入
///   `TerrainEditingData`，`next_index` 提升（O：+0x38 = max(索引+1, 原值)）；
/// - **动画**（其余）：`eased = 1-(1-t)^3`；`undo==0 → 渐进 reapply`（点集按 eased 比例
///   写入笔画）；`undo!=0 → revert`（移除笔画；共享 revert 0x140858d30 的局部消除语义
///   待 owner）；`elapsed += dt*speed`；`eased >= 1.0` 完成。
pub fn advance_terrain_replay(
    command: &mut TerrainReplayCmd,
    editing: &mut TerrainEditingData,
    speed: f32,
    dt: f32,
) -> bool {
    if command.undo {
        // revert：移除目标笔画。共享 `TerrainEdit::revert` 0x140858d30 的动画局部
        // 消除（CurveU 进度）属共享 terrain_edit owner（待取证）；此处为完整移除。
        editing.strokes.remove(&command.stroke_id);
        return true;
    }
    let eased = command.eased();
    let n = command.stroke.points.len();
    let k = if n == 0 {
        0
    } else {
        ((eased * n as f32).ceil() as usize).min(n)
    };
    let entry = editing.strokes.entry(command.stroke_id).or_insert_with(|| TerrainStroke {
        points: Vec::new(),
        falloff: command.stroke.falloff,
        index: command.stroke_id,
        height_ws: command.stroke.height_ws,
    });
    entry.points = command.stroke.points[..k].to_vec();
    entry.falloff = command.stroke.falloff;
    entry.height_ws = command.stroke.height_ws;
    // O：+0x38 = max(索引+1, 原值)。
    let next = command.stroke_id.0.saturating_add(1);
    if editing.next_index < next {
        editing.next_index = next;
    }
    if eased >= 1.0 {
        return true;
    }
    command.elapsed += dt * speed;
    false
}
