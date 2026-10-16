//! `rmb_autoclutter.rs` — 右键移除 auto-clutter（B 0x140976ca0，O）。
//!
//! 证据：`chapters/systems/ui_clutter/rmb_autoclutter.rs.md`、`ui-input/DIGEST.md`、
//! re-analyst-interaction（interaction-group）：
//! - 右键命中 → hashbrown SIMD 查找 + 实体位集定位 auto-clutter 实体；
//! - 旋转 = acos 链 + `ClutterMeshTy::from`（O：三向量点积 + 开方 + `FUN_1408b83d0`）；
//! - 推 0x48 B `EvRightClickedAutoClutter` 事件（tag=2@0、subtype=3@+0x29、
//!   `type@+0x3c`、entity@+0x40）；
//! - 末尾 alloc 0x18 B Arc → 入历史（tag 0xd = AutoClutter，**无条件**——reviewer 基线）。
//!
//! crate 无实体/装饰存储接线（`DecoratorStorage` 在 recovered_building，宿主提供右键命中
//! id），本实现以 [`RmbAutoClutterHit`] 事件驱动删除/历史；旋转与 0x48 B spawn 事件的
//! 完整生成依赖实体机制，待取证。
use bevy_ecs::prelude::*;
use glam::Vec2;

use crate::systems::mode_manager::ClutterMeshTy;

use super::{DeletedAutoClutter, defered_autoclutter_history::RecordAutoClutterHistory};

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
/// `rot = |fabs(sqrt(−2·w·y − 2·x²)) − (−π/2 − fmod(entry_angle, 2π))| · 0.5`，输出 {cos, sin}；
/// 系数 {−0,−0,−1} 使“点积”实为 −x；基常量 `_UNK_142a10ff0` = −π/2；
/// `CircularF32::as_f32`（0x1416f3210）= fmod 2π。
pub fn get_clutter_rot_in_roof_space(w: f32, x: f32, y: f32, entry_angle: f32) -> Vec2 {
    let sqrt_term = (-2.0 * w * y - 2.0 * x * x).sqrt().abs();
    let base = -core::f32::consts::FRAC_PI_2 - fmod_2pi(entry_angle);
    let rot = (sqrt_term - base).abs() * 0.5;
    Vec2::new(rot.cos(), rot.sin())
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

/// `handle_rmb_autoclutter` @0x140976ca0（O 结构；删除 + 无条件历史）。
pub fn handle_rmb_autoclutter(
    mut hits: EventReader<RmbAutoClutterHit>,
    mut deleted: ResMut<DeletedAutoClutter>,
    mut records: EventWriter<RecordAutoClutterHistory>,
) {
    for hit in hits.read() {
        // 删除集合（O：原版消费右键命中后入删除集合）。
        deleted.0.insert(hit.0);
        // 无条件入历史（O：末尾 alloc Arc → tag 0xd；录音门只影响写历史，由
        // defered_autoclutter_history 侧登记失败）。
        records.write(RecordAutoClutterHistory(hit.0));
    }
}
