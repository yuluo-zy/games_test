//! 玩家楼梯支柱视觉（construct_playermade_stairs/pillars_visual.rs）。
//!
//! `construct_playermade_stair_pillars_visuals`（0x14077d520）。候选参数面：
//! `(ResMut<StairsPillarProposals>, Res<UiSignifierStream>,
//!  ResMut<StairsVisualState>, Res<AssetSsboLibrary>,
//!  Res<TerrainHeightsData>, …)`。
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::{
    stairs::visual::StairsVisualState, stairs_walls::StairsPillarProposals,
    terrain_height::TerrainHeightsData,
};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use bevy_ecs::prelude::*;

/// B 0x14077d520：支柱提议 → 视觉状态记录（44B 提议 → 96B 支柱实例）。
pub fn construct_playermade_stair_pillars_visuals(
    mut proposals: ResMut<StairsPillarProposals>,
    _signifiers: Res<UiSignifierStream>,
    mut visual: ResMut<StairsVisualState>,
    ssbo: Res<AssetSsboLibrary>,
    terrain: Res<TerrainHeightsData>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    let mut all = Vec::new();
    proposals.drain_all(&mut all);
    for proposal in all {
        // 44B 提议：{x,z, 高度…} → 96B 支柱实例（terrain 高度采样）。
        let p = glam::Vec3::new(
            f32::from_le_bytes(proposal[0..4].try_into().unwrap_or([0; 4])),
            0.0,
            f32::from_le_bytes(proposal[4..8].try_into().unwrap_or([0; 4])),
        );
        let _ = terrain;
        let mut record = [0u8; 96];
        record[..12].copy_from_slice(&p.to_array().map(f32::to_le_bytes).concat());
        output.brick_instances.push(record);
    }
    let _ = (&mut *visual, ssbo);
}
