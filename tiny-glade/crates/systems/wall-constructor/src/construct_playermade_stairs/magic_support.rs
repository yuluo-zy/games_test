//! 玩家楼梯浮动砖（construct_playermade_stairs/magic_support.rs）。
//!
//! `playermade_stair_floating_bricks`（0x14077a2f0）。候选参数面：
//! `(Res<UiSignifierStream>, ResMut<StairsVisualState>, Res<AssetSsboLibrary>,
//!  Res<GameTime>, EventReader<OnWallChanged>, …)`。
use bevy_ecs::prelude::*;
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::resources::{
    stairs::visual::StairsVisualState, time::GameTime, walls::public_walls::OnWallChanged,
};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;

/// B 0x14077a2f0。**未实现（U）**：`MagicSupports::clear`、`Rng::f32`×15、
/// `EasingFunction::apply` 动画曲线与 `BrickSourceId::from_wall_id` 记录未逐语句；
/// 旧的空循环（消费事件但无输出）已替换为显式 U。
pub fn playermade_stair_floating_bricks(
    mut changed: EventReader<OnWallChanged>,
    _signifiers: Res<UiSignifierStream>,
    mut visual: ResMut<StairsVisualState>,
    ssbo: Res<AssetSsboLibrary>,
    time: Res<GameTime>,
) {
    let _ = (
        &mut changed,
        &mut *visual,
        &ssbo,
        &time,
    );
    todo!(
        "未实现: playermade_stair_floating_bricks 0x14077a2f0（MagicSupports/easing/RNG 动画未逐语句）"
    )
}
