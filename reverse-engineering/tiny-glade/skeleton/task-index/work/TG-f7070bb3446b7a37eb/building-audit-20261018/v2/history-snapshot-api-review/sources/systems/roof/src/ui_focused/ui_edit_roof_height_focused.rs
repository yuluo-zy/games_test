//! 聚焦模式屋顶高度+脊线编辑（ui_focused/ui_edit_roof_height_focused.rs）。
//!
//! `ui_edit_roof_ridge_and_height_focused`（0x1421ad440，5393B/1164）。真逻辑
//! （decomp O）：聚焦态（模式 1/2,<6）；`RoofRidgeDims::from/to_world_space` 求
//! 脊中点、偏移 0.75；`CameraMatrices::eye_right`、`xz_rotation_bad`、
//! `box_trigger_volume`；`DigitalActionState::pressed` 选 UiSounds；
//! `format!("roof_edit")`、`WidgetParams(sounds/occlude_by_world)`、
//! `UiState::add`；`StructureType::is_fence→into_drag_state(缩放 1.0/1.2)`；
//! `format!("height ↓")` 指示符 + `SignifierFactory/UiSignifierStream`；
//! switch(drag)：case1 project_cursor_onto_ray→距离差→VisualEdit 0x40 事件 +
//! DecoratorArchivist/decorator_cull_allow_wall + InnerWalls 墙高；case2
//! project_cursor_onto_ray→inverse_lerp→clamp 新高度、墙高 1.6 且跨 0.1→
//! update_wall_state_height、0x40 事件；case3 字段比较→0x98 历史(tag2)+
//! History::add+ev_height_changed。
//!
//! ⚠️ **未实现**：本文件主函数经 reviewer FIX-3 清退发明拖拽差（`/200.0` 写
//! EditRoofCmd）为纯签名。真逻辑依赖 CameraMatrices/box_trigger_volume/
//! signifier/反解链（UI 工具框架批次 3 后置），待接线后按 decomp 重建。
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
///
/// 未实现（近似体清退，reviewer FIX-3）：真逻辑 = `CameraMatrices::eye_right`、
/// `xz_rotation_bad`、`box_trigger_volume`、`project_cursor_onto_ray` →
/// `inverse_lerp` → clamp 新高度、`update_wall_state_height`（1.6 跨 0.1）、
/// `ev_height_changed`、0x98 历史(tag2)+History::add。现纯签名不写真实
/// EditRoofCmd（发明拖拽差 `/200.0` 已清退）。
pub fn ui_edit_roof_ridge_and_height_focused(
    _grab: Local<HeightGrabParams>,
    mode: Res<AppMode>,
    _walls: Res<PublicWalls>,
    _roofs: Query<&Roof>,
    _ui: Res<UiState>,
    _history: ResMut<History>,
    _cache: ResMut<crate::ui_edit_roof::RoofParamsCacheResource>,
    _edit: EventWriter<EditRoofCmd>,
) {
    use country_core::systems::mode_manager::EditSubject;
    let AppMode::Editing(EditSubject::Roof(_wall)) = *mode else {
        return;
    };
    // 未实现：不写真实 EditRoofCmd。
}
