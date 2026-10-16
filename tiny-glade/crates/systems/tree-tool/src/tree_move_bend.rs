//! tree_move_bend.rs — 树移动/弯折插值（B 0x1422689c0，O；re-analyst-A 契约 §2）。
//!
//! 语义（O）：`time_continuous_lerp`（0x140c904e0）×2 → 位移 `clamp(dist, lo, hi)`
//! 段（lo/hi 常量区 U）→ 写新位置 + 滴答（`TreeMoveBend` 字段 U）。

use bevy_ecs::prelude::*;

use country_core::components::{PrefabInstanceData, TreeMoveBend};
use country_core::resources::time::GameTime;

/// `tree_move_bend` @0x1422689c0（O 面）。
pub fn tree_move_bend(
    mut trees: Query<(&mut TreeMoveBend, &mut PrefabInstanceData)>,
    _time: Res<GameTime>,
) {
    for (mut bend, mut data) in trees.iter_mut() {
        // O：time_continuous_lerp（0x140c904e0）×2 位移插值 + clamp(dist, lo, hi)
        // 段（lo/hi 常量区 U——re-analyst §6）；`TreeMoveBend` 相位字段 U，
        // 本处保持组件时间累进（I）。
        bend._gap = bend._gap.wrapping_add(1);
        let _ = &mut *data;
    }
}
