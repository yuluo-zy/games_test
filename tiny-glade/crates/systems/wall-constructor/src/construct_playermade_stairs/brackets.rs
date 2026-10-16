//! 玩家楼梯支架（construct_playermade_stairs/brackets.rs）。
//!
//! `construct_playermade_stair_brackets`（0x1407834f0，closure$7 → walk
//! 0x141231ea0）。候选参数面（skeleton 富化）：`(ResMut<StairsBracketsRequests>,
//!  Res<UiSignifierStream>, ResMut<StairsVisualState>, Res<AssetSsboLibrary>,
//!  Res<PlayermadeStairsAssemblyRequests>, …)`。
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::{
    stairs::visual::StairsVisualState,
    stairs_walls::StairsBracketsRequests,
};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use bevy_ecs::prelude::*;

/// B 0x1407834f0。
pub fn construct_playermade_stair_brackets(
    mut requests: ResMut<StairsBracketsRequests>,
    _signifiers: Res<UiSignifierStream>,
    mut visual: ResMut<StairsVisualState>,
    ssbo: Res<AssetSsboLibrary>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    // ArchWalker::walk<closure$7>（0x141231ea0）沿请求段发支架砖（64B）。
    for (_wall, record) in requests.requests.iter() {
        let p = glam::Vec3::new(
            f32::from_le_bytes(record[0..4].try_into().unwrap_or([0; 4])),
            0.0,
            f32::from_le_bytes(record[4..8].try_into().unwrap_or([0; 4])),
        );
        let mut plank = [0u8; 64];
        plank[..12].copy_from_slice(&p.to_array().map(f32::to_le_bytes).concat());
        output.plank_instances.push(plank);
    }
    let _ = (&mut *visual, ssbo);
}
