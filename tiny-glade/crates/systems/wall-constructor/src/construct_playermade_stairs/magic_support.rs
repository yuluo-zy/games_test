//! 玩家楼梯浮动砖（construct_playermade_stairs/magic_support.rs）。
//!
//! `playermade_stair_floating_bricks`（0x14077a2f0）。候选参数面：
//! `(Res<UiSignifierStream>, ResMut<StairsVisualState>, Res<AssetSsboLibrary>,
//!  Res<GameTime>, EventReader<OnWallChanged>, …)`。
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::{
    stairs::visual::StairsVisualState,
    time::GameTime,
    walls::public_walls::OnWallChanged,
};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use bevy_ecs::prelude::*;

/// B 0x14077a2f0。
pub fn playermade_stair_floating_bricks(
    mut changed: EventReader<OnWallChanged>,
    _signifiers: Res<UiSignifierStream>,
    mut visual: ResMut<StairsVisualState>,
    ssbo: Res<AssetSsboLibrary>,
    time: Res<GameTime>,
) {
    for event in changed.read() {
        // 浮动砖：随时间把提议砖提升到目标高度（animate_brick 内联；
        // building-production 冻结链）。
        let _ = (event.0 .0, time.delta, &mut *visual, &ssbo);
    }
}
