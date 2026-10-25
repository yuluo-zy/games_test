//! 玩家楼梯支柱视觉（construct_playermade_stairs/pillars_visual.rs）。
//!
//! `construct_playermade_stair_pillars_visuals`（0x14077d520）。候选参数面：
//! `(ResMut<StairsPillarProposals>, Res<UiSignifierStream>,
//!  ResMut<StairsVisualState>, Res<AssetSsboLibrary>,
//!  Res<TerrainHeightsData>, …)`。
use bevy_ecs::prelude::*;
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::{
    stairs::visual::StairsVisualState, stairs_walls::StairsPillarProposals,
    terrain_height::TerrainHeightsData,
};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;

/// B 0x14077d520。**未实现（U）**：`StairsState::graph`/`StairSegment::new`/
/// `Rectangle2d` 边法向/`WallConstructor::from_curve` + `random_splits` +
/// `construct_pillar_stub`(inlinee 24718) 的柱高/数量未逐语句。
/// 旧“44B 提议首 8 字节直推 96B”的近似体已移除（不伪造输出）。
pub fn construct_playermade_stair_pillars_visuals(
    mut proposals: ResMut<StairsPillarProposals>,
    _signifiers: Res<UiSignifierStream>,
    mut visual: ResMut<StairsVisualState>,
    ssbo: Res<AssetSsboLibrary>,
    terrain: Res<TerrainHeightsData>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    let _ = (
        &mut *proposals,
        &mut *visual,
        &ssbo,
        &terrain,
        &mut *output,
    );
    todo!(
        "未实现: construct_playermade_stair_pillars_visuals 0x14077d520（graph/from_curve/construct_pillar_stub 未逐语句）"
    )
}
