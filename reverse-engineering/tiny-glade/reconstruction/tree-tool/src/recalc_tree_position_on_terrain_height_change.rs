//! tree-tool：recalc_tree_position_on_terrain_height_change 业务恢复草稿
//!（B 主基线，反汇编级证据）
//!
//! 依据：evidence/multi-baseline/tree-tool-rest-analysis.md（2026-10-13 reva/radare2
//! 精读 0x142268dc0，218 条指令）。
//!
//! 硬证据：
//! - system_type 包装器族（0x141bb2050 等）签名 =
//!   Query<&mut PrefabInstanceData>, Res<TerrainHeightsData<TerrainHeightSsbo>>,
//!   EventReader<OnTerrainChange>。
//! - 反汇编：prologue 内联 Events 双缓冲"有新事件"检查，无新事件直接返回；
//!   逐实体写 bevy change tick；|x|<=65 && |z|<=65 门控后 position()→
//!   sample_at_world_xz<TerrainHeightSsbo>→写 y@+4 = max(高度, 0.0)。
//! - 常量（radare2 确认）：阈值 65.0 = 0x42820000 @ 0x142a83814；abs 掩码
//!   0x7fffffff @ 0x14292e980。
//!
//! 依赖复用：bevy_ecs 0.16 直接复用。OnTerrainChange 为 country_core::
//! systems::terrain::delayed_terrain_changed_msg::OnTerrainChange 事件；
//! TerrainHeightsData<TerrainHeightSsbo> 为 country_core 地形资源。

use bevy_ecs::change_detection::Res;
use bevy_ecs::event::EventReader;
use bevy_ecs::prelude::*;
use bevy_ecs::query::Query;

/// 预制体实例数据组件（country_core::systems::prefab::PrefabInstanceData）。
/// 本轮证据 size 0x30：position Vec3 @+0（position() 复制 12B；recalc 更新 y@+4）；
/// f32 @+0xc（process_tree_cmds Move 分支 inverse_lerp 读）；@+0x24/@+0x28 由
/// tree_move_bend 写 step（见该文件）。其余字段 unknown。
pub struct PrefabInstanceData {
    /// [evidence] 位置 Vec3 @+0；recalc 将 y 更新为地形采样高度。
    pub position: Vec3,
    /// [evidence] f32 @+0xc（unknown 语义）。
    pub field_c: f32,
    /// @+0x10..+0x24 未确认（unknown）。
    pub unknown: [u8; 0x14],
    /// [evidence] f32 @+0x24 step_x（tree_move_bend 写入）。
    pub step_x: f32,
    /// [evidence] f32 @+0x28 step_y（tree_move_bend 写入）。
    pub step_y: f32,
    /// @+0x2c..+0x30 未确认（unknown）。
    pub unknown_tail: [u8; 0x4],
}

/// 地形高度资源（country_core::resources::terrain_height）。
/// `sample_at_world_xz<TerrainHeightSsbo>`（0x141246fb0）世界 xz → 高度。
pub struct TerrainHeightsData<T> {
    pub _marker: std::marker::PhantomData<T>,
}

/// 地形高度 SSBO（country_core::resources::terrain_height::TerrainHeightSsbo）。
pub struct TerrainHeightSsbo;

/// 地形改变延迟事件（country_core::systems::terrain::delayed_terrain_changed_msg::
/// OnTerrainChange）。有事件到达即触发本系统。
pub struct OnTerrainChange;

/// 地形改变后重算所有预制体（树）的地面高度。
///
/// 签名来自 system_type 包装器（硬证据）。PDB size 902（0x386），
/// 源文件 recalc_tree_position_on_terrain_height_change.rs 12–35 行。
pub fn recalc_tree_position_on_terrain_height_change(
    mut prefab_instances: Query<&mut PrefabInstanceData>,
    terrain_heights: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    mut terrain_changed: EventReader<OnTerrainChange>,
) {
    // 1) prologue 内联 Events 检查：无新事件 → 直接返回（反汇编 0x142268de3..0x142268e59）。
    if terrain_changed.is_empty() {
        return;
    }
    let _ = &terrain_heights;

    for mut instance in &mut prefab_instances {
        // [evidence] 写 bevy change tick（0x142268f4f / 0x1422690bf）。
        // 2) 边界门控：|x| <= 65.0 && |z| <= 65.0（abs 掩码 + 阈值，0x142268f53..）。
        if instance.position.x.abs() <= 65.0 && instance.position.z.abs() <= 65.0 {
            // 3) position()（0x140ad51a0）取 x/z →
            //    sample_at_world_xz<TerrainHeightSsbo>（0x141246fb0）→ 高度。
            let height = sample_at_world_xz(
                &terrain_heights,
                instance.position.x,
                instance.position.z,
            );
            // 4) 若采样高度 != 已存 y → 写 max(高度, 0.0)（0x142268fa9..0x142268fbe）。
            if height != instance.position.y {
                instance.position.y = height.max(0.0);
            }
        }
    }
}

/// [evidence] 0x141246fb0；世界 xz → 地形高度（依赖实现不还原）。
fn sample_at_world_xz(_heights: &TerrainHeightsData<TerrainHeightSsbo>, _x: f32, _z: f32) -> f32 {
    0.0
}
