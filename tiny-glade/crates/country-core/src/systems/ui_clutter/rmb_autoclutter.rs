//! `rmb_autoclutter.rs` — 右键移除 auto-clutter（B 0x140976ca0，O）。
//!
//! 证据：`chapters/systems/ui_clutter/rmb_autoclutter.rs.md`、`ui-input/DIGEST.md`、
//! re-analyst-interaction（interaction-group）、`ui-input/decomp/decomp-0x140976ca0.txt`：
//! - 右键命中 → **hashbrown SIMD probe**（FUN_1405b0d70 哈希 + control-byte 位掩码
//!   折叠 + 逐位键比对）→ 实体位集/组件表双重校验 → clutter 记录（0x58B 步长）；
//! - 旋转 = acos 链 + `ClutterMeshTy::from`（O：三向量点积 + 开方 + `FUN_1408b83d0`）；
//! - 推 0x48 B `EvRightClickedAutoClutter` 事件（tag=2@0、subtype=3@+0x29、
//!   type@+0x3c、entity@+0x40）；
//! - 末尾 alloc 0x18 B Arc → 入历史（tag 0xd = AutoClutter，**无条件**——reviewer 基线）
//!   并调 `FUN_140a86270` = `History::start_input_recording`（录制会话门控）；
//! - **原版不触碰 `DeletedAutoClutter` 集合**（该集合由 generation/mesh owner 消费）。
//!
//! crate 无原生 ECS 实体/组件表机制（`DecoratorStorage` 在 recovered_building，宿主提供
//! 实体机制），本实现以 [`AutoClutterEntityIndex`]（hashbrown 表，宿主维护记录）承载
//! 右键 probe 目标；probe 用 hashbrown `raw_entry`（真实哈希表键查找，与 FUN_1405b0d70
//! 语义对应）。0x48 B spawn 事件 + `History::start_input_recording` 完整生成落地。
use bevy_ecs::prelude::*;

use crate::systems::mode_manager::{AppMode, ClutterMeshTy};

use super::{defered_autoclutter_history::RecordAutoClutterHistory, AutoClutterEntityIndex};

/// 右键命中 auto-clutter（I：宿主从 UI 悬停/射线解析出实体 id）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct RmbAutoClutterHit(pub u64);

/// 右键转换产生的 0x48 B spawn 事件（O 布局：tag=2@0、subtype=3@+0x29、type@+0x3c、
/// entity@+0x40；payload 其余字段语义 I）。
#[derive(Event, Clone, Copy, Debug)]
pub struct EvRightClickedAutoClutter {
    pub entity: u64,
    pub ty: ClutterMeshTy,
    pub position: glam::Vec3,
    pub rotation: f32,
}

/// 旋转公式（O，re-analyst 定案，RE-ANALYSIS2）：
/// `rot = |fabs(sqrt(−2·w·y − 2·x²)) − (−π/2 − fmod(entry_angle, 2π))| · 0.5`，
/// 输出角度；调用方取 cos/sin。系数 {−0,−0,−1} 使"点积"实为 −x；基常量
/// `_UNK_142a10ff0` = −π/2；`CircularF32::as_f32`（0x1416f3210）= fmod 2π。
pub fn get_clutter_rot_in_roof_space(w: f32, x: f32, y: f32, entry_angle: f32) -> f32 {
    let sqrt_term = (-2.0 * w * y - 2.0 * x * x).sqrt().abs();
    let base = -core::f32::consts::FRAC_PI_2 - fmod_2pi(entry_angle);
    (sqrt_term - base).abs() * 0.5
}

/// `CircularF32::as_f32`（O：0x1416f3210）= 角度归约到 [0, 2π)。
pub fn fmod_2pi(angle: f32) -> f32 {
    let two_pi = core::f32::consts::TAU;
    let mut reduced = angle % two_pi;
    if reduced < 0.0 {
        reduced += two_pi;
    }
    reduced
}

/// `handle_rmb_autoclutter` @0x140976ca0（O 结构；原版**不触碰** DeletedAutoClutter）。
///
/// hashbrown entity probe（raw_entry）+ 旋转（get_clutter_rot_in_roof_space）+ 0x48B
/// `EvRightClickedAutoClutter`（@+0x29=3）+ 无条件历史（tag 0xd，经 defered 链）+
/// `History::start_input_recording` 门控。
pub fn handle_rmb_autoclutter<S: Send + Sync + 'static>(
    mut hits: EventReader<RmbAutoClutterHit>,
    index: Res<AutoClutterEntityIndex>,
    mode: Res<AppMode>,
    mut history: ResMut<crate::resources::history::History<S>>,
    mut records: EventWriter<RecordAutoClutterHistory>,
    mut right_clicked: EventWriter<EvRightClickedAutoClutter>,
) {
    let mut any_hit = false;
    for hit in hits.read() {
        // hashbrown raw_entry probe（O：FUN_1405b0d70 + control-byte probe 语义）。
        let Some(record) = index.0.raw_entry().from_key(&hit.0).map(|entry| *entry.1) else {
            // 未命中路径（O：FUN_1428d9310(&UNK_142ab6950) log + 无点击分支）。
            log::warn!("Right-click did not hit an auto-clutter record");
            continue;
        };
        any_hit = true;
        // 旋转（O：get_clutter_rot_in_roof_space；原死代码已接入转换路径）。
        let rotation =
            get_clutter_rot_in_roof_space(record.w, record.x, record.y, record.entry_angle);
        // 0x48B EvRightClickedAutoClutter（@+0x29 = 3）。
        right_clicked.write(EvRightClickedAutoClutter {
            entity: hit.0,
            ty: record.ty,
            position: record.position,
            rotation,
        });
        // 无条件入历史（O：末尾 alloc Arc → FUN_140a84ec0 tag 0xd；经 defered 链）。
        records.write(RecordAutoClutterHistory(hit.0));
    }
    // 录制会话门控（O：FUN_140a86270 = History::start_input_recording 0x140a86270）。
    if any_hit {
        let _ = history.start_input_recording(mode.native_tag());
    }
}
