//! 玩家楼梯支架（construct_playermade_stairs/brackets.rs）。
//!
//! `construct_playermade_stair_brackets`（0x1407834f0，closure$7 → walk
//! 0x141231ea0）。候选参数面（skeleton 富化）：`(ResMut<StairsBracketsRequests>,
//!  Res<UiSignifierStream>, ResMut<StairsVisualState>, Res<AssetSsboLibrary>,
//!  Res<PlayermadeStairsAssemblyRequests>, …)`。
use bevy_ecs::prelude::*;
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::{
    stairs::visual::StairsVisualState, stairs_walls::StairsBracketsRequests,
};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;

/// B 0x1407834f0。**未实现（U）**：`ArchWalker::walk<closure$7>`（0x141231ea0）
/// 的托架几何、`fastrand` 序列与 `DecoratorExtraInfo::try_as_stair` 分支未逐语句；
/// 旧“把请求记录首 8 字节当位置直推 64B”的近似体已移除（不伪造输出）。
pub fn construct_playermade_stair_brackets(
    mut requests: ResMut<StairsBracketsRequests>,
    _signifiers: Res<UiSignifierStream>,
    mut visual: ResMut<StairsVisualState>,
    ssbo: Res<AssetSsboLibrary>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    let _ = (
        &mut *requests,
        &mut *visual,
        &ssbo,
        &mut *output,
    );
    todo!(
        "未实现: construct_playermade_stair_brackets 0x1407834f0（walk<closure$7> 托架几何/fastrand 序列未逐语句）"
    )
}
