//! `resources::history::terrain_edit` — 地形编辑记录（WP17）。
//!
//! O（反编译 TerrainEdit::revert 0x140858d30 / reapply 0x1408592d0 /
//! update_terrain_curve 0x140859740 / duration；ReplayCmd 判别 3 负载
//! `Arc<Option<TerrainEdit>>`；观察 JSON New/Edit/MoveTerrainStroke）：
//!   - `revert`：撤销（恢复 stroke 原状态 → EditTerrainStrokeCmd）。
//!   - `reapply`：重放（应用 stroke → EditTerrainStrokeCmd）。
//!   - `duration`：曲线点距累加 → 动画时长。
//!   - `update_terrain_curve`：更新地形曲线。

use crate::typed_gap::Gap;
use glam::Vec3;
use serde::{Deserialize, Serialize};

/// 地形笔刷编辑负载（观察 NewTerrainStroke/EditTerrainStroke/MoveTerrainStroke）。
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TerrainEdit {
    /// O：`stroke_pts`（3D 点集；duration 用点距累加）。
    #[serde(default)]
    pub stroke_pts: Vec<Vec3>,
    /// O：`index`/`id`（观察；与 id 是否互换 U）。
    #[serde(default)]
    pub index: u32,
    /// O：`height_ws` / `prev_height_ws` / `new_height_ws`。
    #[serde(default)]
    pub height_ws: f32,
    /// O：`prev_falloff` / `new_falloff`。
    #[serde(default)]
    pub falloff: f32,
    /// O：`move_vec`（MoveTerrainStroke）。
    #[serde(default)]
    pub move_vec: Vec3,
    /// U：判别位/方向（`replay_terrain` 读 payload+0x3 undo、+0x19 started）。
    #[serde(default)]
    pub aux: Gap,
}

impl TerrainEdit {
    /// O：`duration`（0x140858d30 区；enumerate+fold 点距累加，clamp(0.4, Σ*0.01,
    /// 3.0)；无点 0.3）。
    pub fn duration(&self) -> f32 {
        let sum: f32 = self
            .stroke_pts
            .iter()
            .zip(self.stroke_pts.iter().skip(1))
            .map(|(a, b)| a.distance(*b))
            .sum();
        let d = (sum * 0.01).clamp(0.4, 3.0);
        if d.is_finite() {
            d
        } else {
            0.3
        }
    }
    /// O：`revert`（0x140858d30）：撤销应用。领域 apply 边界（→ EditTerrainStrokeCmd）。
    pub fn revert(&self) {}
    /// O：`reapply`（0x1408592d0）：重放应用。
    pub fn reapply(&self) {}
    /// O：`update_terrain_curve`（0x140859740）。
    pub fn update_terrain_curve(&mut self) {}
}
