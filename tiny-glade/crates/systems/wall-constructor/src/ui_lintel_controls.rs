//! 过梁 UI 控制（ui_lintel_controls.rs）。
//!
//! `ui_lintel_controls`（0x140771a60）。候选参数面（skeleton 富化）：
//! `(Res<AppMode>, Res<UiState>, Res<UiSignifierStream>,
//!  ResMut<InnerWalls>, Res<PublicWalls>, Local<…>, …)`。
use bevy_ecs::prelude::*;
use country_core::resources::walls::{inner_walls::InnerWalls, public_walls::PublicWalls};
use country_core::systems::{
    mode_manager::AppMode, ui::resources::ui_state::UiState,
    ui::signifiers::signifier_create::UiSignifierStream,
};

/// B 0x140771a60：过梁编辑 UI 分发（模式 + 交互 → 过梁段编辑）。
pub fn ui_lintel_controls(
    mode: Res<AppMode>,
    ui: Res<UiState>,
    _signifiers: Res<UiSignifierStream>,
    mut inner: ResMut<InnerWalls>,
    walls: Res<PublicWalls>,
) {
    use country_core::systems::mode_manager::EditSubject;
    let AppMode::Editing(EditSubject::Wall(wall)) = *mode else {
        return;
    };
    if ui.interaction().widget.0 == 0 {
        return;
    }
    if walls
        .get(country_core::resources::walls::WallId(wall))
        .is_none()
    {
        return;
    }
    // 过梁段编辑命令（toolbar 交互后置登记）。
    let _ = &mut *inner;
}
