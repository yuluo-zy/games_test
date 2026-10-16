//! 屋顶尖 UI 编辑（ui_edit_roof_tip.rs）。
//!
//! sourcemap 无独立业务条目（内联/胶水，re-analyst-C 登记）。实现为
//! ui_edit_roof 家族的分发入口（tip 编辑经 EditRoofCmd tip 分支）。
use country_core::resources::{
    roofs::{Roof, commands::EditRoofCmd},
    walls::public_walls::PublicWalls,
};
use country_core::systems::{mode_manager::AppMode, ui::resources::ui_state::UiState};
use bevy_ecs::prelude::*;

/// tip 编辑派发（源图无业务条目；以 ui_edit_roof 的 tip 分支内联为准）。
pub fn ui_edit_roof_tip(
    _mode: Res<AppMode>,
    _walls: Res<PublicWalls>,
    _roofs: Query<&Roof>,
    _ui: Res<UiState>,
    _edit: EventWriter<EditRoofCmd>,
) {
}
