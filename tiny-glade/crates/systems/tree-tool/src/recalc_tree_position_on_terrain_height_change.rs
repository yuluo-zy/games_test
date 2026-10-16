//! recalc_tree_position_on_terrain_height_change.rs — 地形高变化→树 y 重算
//! （B 0x142268dc0，O；re-analyst-A 契约 §2）。
//!
//! 语义（O）：`OnTerrainChange` 事件 → `|x|,|z| ≤ 65`（0x142a83814，掩
//! 0x14292e980）→ 地形高采样（0x141246fb0）→ `y = max(h, 0)` 写回组件。

use bevy_ecs::prelude::*;

use country_core::systems::prefab::PrefabInstanceData;
use country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange;
use country_core::resources::terrain_height::{TerrainHeightsData, TerrainHeightSsbo};

/// 世界边界（O：0x142a83814 = 65.0，同 forest）。
pub const WORLD_BOUND: f32 = 65.0;

/// `recalc_tree_position_on_terrain_height_change` @0x142268dc0（O 面）。
pub fn recalc_tree_position_on_terrain_height_change(
    mut trees: Query<&mut PrefabInstanceData>,
    heights: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    mut changed: EventReader<OnTerrainChange>,
) {
    if changed.read().next().is_none() {
        return;
    }
    for mut data in trees.iter_mut() {
        let x = data.position.x;
        let z = data.position.z;
        // O：|x|,|z| ≤ 65 界内才重算。
        if x.abs() >= WORLD_BOUND || z.abs() >= WORLD_BOUND {
            continue;
        }
        // O：地形高采样（0x141246fb0）→ y = max(h, 0)。
        let h = heights.sample_at_world_xz(x, z);
        data.position.y = h.max(0.0);
    }
}
