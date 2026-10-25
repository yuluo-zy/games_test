//! recalc_tree_position_on_terrain_height_change.rs — 地形高变化→树 y 重算
//! （B 0x142268dc0，O；2026-10-17 fresh 反编译）。
//!
//! `OnTerrainChange` 有事件才继续（reader 空返回，decomp 行 60-62）；逐
//! 实例变换（0x30 B `Transform`）：`|x|,|z| <= 65.0`（**含边界**，`ABS<=65`，
//! decomp 行 110/160）→ 地形高采样 → `y != h`（f32 不等，含 NaN）时写
//! `y = max(h, 0)`。

use bevy_ecs::prelude::*;

use country_core::resources::prefab::PrefabInstance;
use country_core::resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData};
use country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange;

/// 世界边界（O：0x142925980 区 `0x42820000` = 65.0；`ABS(x) <= 65.0`，含边界）。
pub const WORLD_BOUND: f32 = 65.0;

/// `recalc_tree_position_on_terrain_height_change` @0x142268dc0（O 面）。
pub fn recalc_tree_position_on_terrain_height_change(
    mut trees: Query<&mut PrefabInstance>,
    heights: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    mut changed: EventReader<OnTerrainChange>,
) {
    if changed.is_empty() {
        return;
    }
    // B advances its reader to the current event count before traversing the
    // 0x30-byte placement rows. This is the same component as tree commands,
    // not the unrelated 40-byte utility Transform (recalc_decompiled.c).
    changed.clear();
    for mut data in trees.iter_mut() {
        let x = data.0.position.x;
        let z = data.0.position.z;
        // O：`ABS(x) <= 65.0 && ABS(z) <= 65.0`（含边界；`<=` 对 NaN 为假，
        // 保持与 decomp 一致——re-analyst 记 `>=` 跳过与字节不符，以 <= 为准）。
        if !(x.abs() <= WORLD_BOUND && z.abs() <= WORLD_BOUND) {
            continue;
        }
        let h = heights.sample_at_world_xz(x, z);
        // O：`y != h || NAN(y) || NAN(h)` 才写（f32 不等即含 NaN 分支），
        // 写 `max(h, 0.0)`。
        if data.0.position.y != h {
            data.0.position.y = if h <= 0.0 { 0.0 } else { h };
        }
    }
}
