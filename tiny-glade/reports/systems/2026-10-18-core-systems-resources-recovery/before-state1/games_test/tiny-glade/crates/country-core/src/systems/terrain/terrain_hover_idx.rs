//! `terrain_hover_idx.rs` — 地形悬停笔画索引（B 0x140aff720，O）。
//!
//! 证据：`terrain-water/decomp/decomp-0x140aff720.txt`（全文 96 行）、
//! `terrain-water/ALGORITHMS.md`、re-analyst-interaction 深度逆向（interaction-group）：
//! - AppMode 位掩码 **0x421**（位 0/5/10 = Editing(None)/Editing(Terrain)/Terrain）门；
//! - 读地形资源（FUN_140969720 + 串名 `UNK_142a9df5b`（0x6c 长度）校验）→
//!   hovered stroke id = `*(i32*)(*plVar9 + 0x10)`（vtable 高度读 `(**(code**)(puVar8[1]+0x18))`）；
//! - 光标有效性两分支（**decomp 行 75-90**）：
//!   - `fVar12 <= 0.0`（高度 ≤ 0）：**仅当 id == -1 才 return**（id 有效即写）；
//!   - 否则（高度 > 0）：必须悬停位 `(*pbVar4 & 1)`、id 有效、`距离 > 0.5`（
//!     `_UNK_142925870`），任一不满足 → return；
//! - 命中 → `*param_3 = 1; param_3[1] = iVar11`（写 `HoveredTerrainStrokeId { tag=1, id }`）。
//!
//! `MouseTerrainRaycastSsbo` 布局未恢复（`render_ssbo` 为 `Gap`），这里用
//! [`MouseTerrainHoverReadback`] 适配资源承载宿主从 SSBO 拷贝的 id/悬停位/距离
//! （待取证：SSBO 字段）；高度采样用 [`TerrainWaterSample`] 适配（I：WaterMaskSsbo Gap）。
use bevy_ecs::prelude::*;

use crate::{resources::mouse_terrain_raycast::CursorPositionSS, systems::mode_manager::AppMode};

/// 悬停地形笔画 id 资源（O：`HoveredTerrainStrokeId { tag=1, id }`，0x140aff720 写）。
///
/// **布局 = tag@+0（8B）、id@+8（8B），≥16B**（fresh 反编译 `*puVar4 = 1; puVar4[1] = iVar7`：
/// 8 字节写 tag、8 字节写 id）。早期 Rust `{u8 tag@0, i32 id@4}`（8B）id 偏移错（+4≠+8），
/// 已修正。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HoveredTerrainStrokeId {
    /// 命中标记（写 1；函数开头无条件复位 0）。
    pub tag: u64,
    /// 笔画 id（-1 = 无命中；@+8）。
    pub id: i64,
}

/// 鼠标地形射线回读适配（I：`MouseTerrainRaycastSsbo` 布局 Gap，宿主从 SSBO 拷贝）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct MouseTerrainHoverReadback {
    /// 命中笔画 id（-1 哨兵 = 未命中）。
    pub id: i32,
    /// 悬停位（O：decomp `(*pbVar4 & 1)`，仅在高度>0 分支检查）。
    pub hovered: bool,
    /// 射线距离（O：decomp `*(float*)(pbVar4 + 8)`；仅高度>0 分支的 0.5 界外阈值）。
    pub distance: f32,
}

/// 光标高度采样适配（I：`WaterMaskSsbo` 布局 Gap；对应 decomp `FUN_140a14df0` 输出）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TerrainWaterSample {
    pub level: f32,
}

/// 界外距离阈值（O：`_UNK_142925870` = 0.5）。
pub const HOVER_DISTANCE_GATE: f32 = 0.5;

/// `terrain_hover_idx` @0x140aff720（O 两分支门控）。
pub fn terrain_hover_idx(
    mode: Res<AppMode>,
    hover: Res<MouseTerrainHoverReadback>,
    water: Res<TerrainWaterSample>,
    _cursor: Res<CursorPositionSS>,
    mut hovered: ResMut<HoveredTerrainStrokeId>,
) {
    // **函数开头无条件复位输出 tag=0**（O：fresh 反编译 `*puVar4 = 0` 在 mode 门之前；
    // miss/门失败帧输出被清）。早期 Rust 不清 → 门失败帧残留上帧命中。
    hovered.tag = 0;
    // AppMode 0x421 掩码门（O：`mode<0xb && (0x421>>(mode&0x1f))&1`，bits {0,5,10} =
    // All/Garden/Terrain 工具；`& 0x1f` 为原生移位掩码）。
    let disc = mode.native_tag();
    if disc >= 0xb || ((0x421u32 >> (disc & 0x1f)) & 1) == 0 {
        return;
    }
    // -1 哨兵 → 未命中，不更新。
    if hover.id == -1 {
        return;
    }
    // 光标高度采样两分支（O：decomp 行 75-90）：
    //   `fVar12 <= 0.0` → 仅当 id == -1 才 return（**高度≤0 且 id 有效要写**）；
    //   否则 → 悬停位门 + dist ≤ 0.5 → return（dist 门**仅**在高度>0 分支）。
    if water.level <= 0.0 {
        if hover.id == -1 {
            return;
        }
    } else {
        if !hover.hovered {
            return;
        }
        if hover.id == -1 {
            return;
        }
        if hover.distance <= HOVER_DISTANCE_GATE {
            return;
        }
    }
    hovered.tag = 1;
    hovered.id = hover.id as i64;
}
