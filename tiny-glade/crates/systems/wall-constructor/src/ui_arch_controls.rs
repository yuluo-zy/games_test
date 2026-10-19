//! 拱 UI 控制（ui_arch_controls.rs）。
//!
//! `ui_arch_controls`（0x140766e70）/`do_ui_arch_controls`（0x140767b70）。
//! 候选参数面（skeleton 富化）：
//! `(Res<AppMode>, Res<PublicWalls>, Res<ArchSegments>, Res<UiState>,
//!  EventWriter<DoUiArchControls>)` 与
//! `(Res<AppMode>, ResMut<InnerWalls>, Res<PublicWalls>, Res<ArchSegments>,
//!  DecoratorArchivalResources, Local<…>, …)`。
use bevy_ecs::{event::Event, prelude::*};
use country_core::resources::walls::{arches::ArchSegments, public_walls::PublicWalls};
use country_core::systems::{mode_manager::AppMode, ui::resources::ui_state::UiState};

/// B `system_wall_constructor::ui_arch_controls::DoUiArchControls`。
#[derive(Event, Clone, Copy, Debug)]
pub struct DoUiArchControls {
    pub wall: u64,
}

/// B 0x140766e70：拱编辑 UI 分发（模式 + 交互命中 → 派发）。
pub fn ui_arch_controls(
    mode: Res<AppMode>,
    walls: Res<PublicWalls>,
    segments: Res<ArchSegments>,
    ui: Res<UiState>,
    mut dispatch: EventWriter<DoUiArchControls>,
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
    let _ = &*segments;
    dispatch.write(DoUiArchControls { wall });
}

/// B 0x140767b70：拱控制执行（段选择 → 拱高编辑命令；toolbar 交互后置）。
pub fn do_ui_arch_controls(
    mut dispatch: EventReader<DoUiArchControls>,
    _mode: Res<AppMode>,
    walls: Res<PublicWalls>,
    _segments: Res<ArchSegments>,
) {
    for request in dispatch.read() {
        // 拱段选择 → 拱高编辑（EditArchHeightCmd 族；toolbar 后置登记）。
        let _ = walls.get(country_core::resources::walls::WallId(request.wall));
    }
}
