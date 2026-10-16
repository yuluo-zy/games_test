//! 聚焦模式屋顶高度+脊线编辑（ui_focused/ui_edit_roof_height_focused.rs）。
//!
//! `ui_edit_roof_ridge_and_height_focused`（0x1421ad440）。候选参数面
//! （skeleton 富化）：`(Local<HeightGrabParams>, Res<AppMode>, Res<PublicWalls>,
//!  Query<&Roof>, DecoratorArchivalResources, Res<UiState>, ResMut<History>,
//!  ResMut<RoofParamsCacheResource>, …)`。
//!
//! 聚焦抓取：Local `HeightGrabParams`（拖拽起点/当前值）；高度经光标
//! 投影更新 height 01 并写 EditRoofCmd；脊线方向切换写 SwitchRoofRidgeDirCmd。
use country_core::resources::{
    history::History,
    roofs::{Roof, commands::EditRoofCmd},
    walls::public_walls::PublicWalls,
};
use country_core::systems::{mode_manager::AppMode, ui::resources::ui_state::UiState};
use bevy_ecs::{event::Event, prelude::*};
use glam::Vec2;

/// B `system_roof::ui_focused::ui_edit_roof_height_focused::HeightGrabParams`
/// （Local 拖拽状态）。
#[derive(Default, Debug)]
pub struct HeightGrabParams {
    pub grabbing: bool,
    pub start_cursor: Option<Vec2>,
    pub start_height: f32,
}

/// 聚焦抓取事件（内部派发）。
#[derive(Event, Clone, Copy, Debug)]
pub struct FocusRoofGrab {
    pub wall: u64,
}

/// B 0x1421ad440。
pub fn ui_edit_roof_ridge_and_height_focused(
    mut grab: Local<HeightGrabParams>,
    mode: Res<AppMode>,
    walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
    ui: Res<UiState>,
    mut history: ResMut<History>,
    mut cache: ResMut<crate::ui_edit_roof::RoofParamsCacheResource>,
    mut edit: EventWriter<EditRoofCmd>,
) {
    use country_core::systems::mode_manager::EditSubject;
    let AppMode::Editing(EditSubject::Roof(wall)) = *mode else {
        return;
    };
    let id = country_core::resources::walls::WallId(wall);
    let Some(entity) = walls.roof_entity(id) else {
        return;
    };
    let Ok(roof) = roofs.get(entity) else {
        return;
    };
    // 高度抓取：光标开始/结束之间的高度差 → height 01（观测 clamp 0.1..1）。
    if ui.interaction().widget.0 != 0 {
        if !grab.grabbing {
            grab.grabbing = true;
            grab.start_cursor = cache.cursor;
            grab.start_height = f32::from_le_bytes(roof.observed()[0x30..0x34].try_into().unwrap());
        }
    } else if grab.grabbing {
        grab.grabbing = false;
        if let (Some(start), Some(current)) = (grab.start_cursor, cache.cursor) {
            let delta = (current.y - start.y) / 200.0;
            let height = (grab.start_height + delta).clamp(0.0, 1.0);
            let mut observed = [0u8; 64];
            observed[0x08..0x0c].copy_from_slice(&1u32.to_le_bytes());
            observed[0x0c..0x10].copy_from_slice(&height.to_le_bytes());
            let mut event =
                country_core::resources::roofs::geometry::edit_rules::RoofEditEvent::from_observed(
                    &observed,
                );
            event.wall_id = wall;
            event.source_flag = 1;
            edit.write(EditRoofCmd(event));
            let _ = history;
        }
    }
}
