//! 屋顶高亮（highlight_roof.rs）。
//!
//! `highlight_roof`：PDB 标 inline（无独立 VA），以 ui_edit_roof/
//! update_roof_spatial 的内联实例为准（re-analyst-C 未单列）。
//! 候选参数面（skeleton 富化）：`(Res<AppMode>, Res<PublicWalls>,
//!  Query<&Roof>, Res<UiState>, EventWriter<…>)` —— 编辑模式中把当前
//! 屋顶写入高亮状态。实现为文档化胶水（Bevy 注册不计业务）。
use country_core::resources::{roofs::Roof, walls::public_walls::PublicWalls};
use country_core::systems::{mode_manager::AppMode, ui::resources::ui_state::UiState};
use bevy_ecs::prelude::*;

/// 屋顶高亮维护（inline 实例的独立化；行为以 ui_edit_roof 内联为准）。
pub fn highlight_roof(
    mode: Res<AppMode>,
    walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
    _ui: Res<UiState>,
) {
    use country_core::systems::mode_manager::EditSubject;
    let AppMode::Editing(EditSubject::Roof(wall)) = *mode else {
        return;
    };
    let Some(entity) = walls.roof_entity(country_core::resources::walls::WallId(wall)) else {
        return;
    };
    let _ = roofs.get(entity);
}
