//! `ui_terrain_focus_mode.rs` — 地形聚焦模式（B 0x1409a4fb0，O 门 + 行为级骨架）。
//!
//! 证据：`terrain-water/decomp/decomp-0x1409a4fb0.txt`（171 KB，近全内联）、
//! re-analyst-interaction RE-ANALYSIS2：
//! - 门：状态首 u32 == 1；
//! - `raycast_simple`（0x1408ce180）：命中 tag 0x11 分叉 + 3D 距离门（未命中
//!   `_UNK_142925980` = f32::MAX）+ 0.5 阈值；
//! - 焦点 HashMap（hash_one 0x1405b0ed0 + xvy 0x140c9cde0）；`as_curve3`（0x140c97e10）；
//!   push `UiAppendSignifier`；
//! - **Replay 哨兵 0x8000000000000002 写入**；
//! - 常量表 10 项（0.01 阈值、-10 速度、1.2/7.0、±1 反射矩阵、CircularF32）。
//!
//! 卡点如实标注：167 KB 内联体转移 switch 表 U，本实现按行为级骨架落地。
use bevy_ecs::prelude::*;

/// 地形聚焦模式状态资源（I：门 = 首 u32 == 1；字段语义 U）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TerrainFocusModeState {
    /// 门控判别（1 = 启用聚焦模式）。
    pub mode: u32,
}

/// Replay 哨兵（O：RE-ANALYSIS2 `0x8000000000000002` 写入聚焦状态）。
pub const REPLAY_SENTINEL: u64 = 0x8000_0000_0000_0002;

/// 聚焦命中阈值（O：0.5）。
pub const FOCUS_HIT_THRESHOLD: f32 = 0.5;

/// 聚焦射线回读适配（I：`raycast_simple` 0x1408ce180 结果；未命中距离 = f32::MAX）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TerrainFocusRaycast {
    /// 命中距离（f32::MAX = 未命中）。
    pub distance: f32,
    /// 命中 tag（0x11 分叉）。
    pub tag: u8,
}

/// 聚焦模式触发事件（I：`UiAppendSignifier` 类）。
#[derive(Event, Clone, Copy, Debug)]
pub struct TerrainFocusTrigger {
    pub distance: f32,
}

/// `ui_terrain_focus_mode` @0x1409a4fb0（O 门 + 行为骨架；内联 switch 表 U）。
pub fn ui_terrain_focus_mode(
    state: Res<TerrainFocusModeState>,
    raycast: Res<TerrainFocusRaycast>,
    mut triggers: EventWriter<TerrainFocusTrigger>,
) {
    // 门：首 u32 == 1（O：RE-ANALYSIS2）。
    if state.mode != 1 {
        return;
    }
    // 距离门：未命中（f32::MAX）或距离 > 0.5 阈值 → 不触发。
    if raycast.distance == f32::MAX || raycast.distance > FOCUS_HIT_THRESHOLD {
        return;
    }
    triggers.write(TerrainFocusTrigger {
        distance: raycast.distance,
    });
    // 焦点 HashMap / as_curve3 / 常量表（0.01 阈值、-10 速度、1.2/7.0、±1 反射矩阵、
    // CircularF32）与 Replay 哨兵写入依赖内联体 switch 表（U，卡点如实标注）。
}
