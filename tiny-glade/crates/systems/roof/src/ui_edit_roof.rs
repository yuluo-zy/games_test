//! 屋顶编辑 UI（ui_edit_roof.rs）。
//!
//! - `ui_edit_roof`（0x141a18dd0，3254B/786）。真逻辑（decomp O）：`AppMode==编辑`
//!   时 puffin "roof_edit_" scope；chunk 迭代屋顶，`RoofAnimation::is_being_animated()`
//!   跳过动画中 roof；`format!` widget 名；`RoofTriggerVolume::triangles()→Vec<Triangle>`
//!   射线相交拾取；`Roof::ty()→local_24f='\f'-ty(tag12/11)`；`WidgetParams::new`；
//!   `UiState::add<String>`；`UiWidgetContext::hovered/intersecting`；
//!   `StructureType::is_fence→HighlightRoofCmd`；`slider3d::into_drag_state`；
//!   `EventWriter<RecordRidgeSwitchHistoryCmd>`。
//! - `do_ui_edit_roof`（0x141a19a90，4228B/985）。真逻辑（decomp O）：`switch`
//!   （clamp(payload[0x10]-2,0,3)）三 case：历史 0x40 事件 / 拖拽量 SQRT(dx²+dy²)
//!   `>0.2` NotifyHintSystemRoofShape、墙高 1.6、Roof::ty、0x40 VisualEdit 事件 /
//!   roof 字段与控件新值比较 → 0x98 历史(tag2)+History::add+wall_history::ev_height_changed。
//!
//! ⚠️ **未实现**：现实现为骨架近似（无触发体拾取、无 slider3d、无历史构造）。
//! 依赖 toolbar widget 框架（WidgetParams/UiSounds/slider3d/TweakerView，
//! 批次 3）与 DecoratorArchivist/墙高历史接线未完成 → 标未实现，不伪造交互逻辑。
use country_core::resources::{
    roofs::{Roof, RoofAnimation, RoofTriggerVolume},
    walls::inner_walls::InnerWalls,
};
use country_core::systems::{mode_manager::AppMode, ui::resources::ui_state::UiState};
use bevy_ecs::{event::Event, prelude::*};
use glam::Vec2;

/// B `system_roof::ui_edit_roof::RoofParamsCacheResource`（编辑缓存）。
#[derive(Resource, Default, Debug)]
pub struct RoofParamsCacheResource {
    /// 悬停/拾取中的屋顶参数缓存（观测 0x58 字节 + 命中实体）。
    pub entity: Option<Entity>,
    pub roof: Option<[u8; 88]>,
    pub cursor: Option<Vec2>,
}

/// 交互完成事件（B `DoUiEditRoof`）。
#[derive(Event, Clone, Copy, Debug)]
pub struct DoUiEditRoof {
    pub entity: Entity,
}

/// B 0x141a18dd0：编辑模式中把相机移动/悬停位置转成屋顶编辑派发。
///
/// 未实现（见模块头）：真逻辑含触发体三角相交拾取、slider3d 控件、HighlightRoofCmd
/// 与 RecordRidgeSwitchHistoryCmd 派发。现骨架保留参数面，不伪造交互。
pub fn ui_edit_roof(
    mode: Res<AppMode>,
    _ui: Res<UiState>,
    _roofs: Query<(Entity, &Roof, &RoofAnimation, &RoofTriggerVolume)>,
    _camera: EventReader<country_core::systems::camera_rig::main_camera::MainCameraMoved>,
    mut _dispatch: EventWriter<DoUiEditRoof>,
    _commands: Commands,
) {
    use country_core::systems::mode_manager::EditSubject;
    let AppMode::Editing(EditSubject::Roof(_wall)) = *mode else {
        return;
    };
    // 真逻辑：RoofAnimation::is_being_animated 跳过 → 触发体射线相交 →
    // WidgetParams/UiState::add → hovered/intersecting → into_drag_state →
    // HighlightRoofCmd + RecordRidgeSwitchHistoryCmd。未实现。
}

/// B 0x141a19a90：光标/交互驱动屋顶参数编辑（缓存命中 → 写编辑命令）。
///
/// 未实现（见模块头）：真逻辑为 switch(clamp(payload[0x10]-2,0,3)) 三 case
/// （0x40 历史 / 拖拽量 SQRT>0.2 / 字段比较→0x98 历史+ev_height_changed）。
pub fn do_ui_edit_roof(
    _dispatch: EventReader<DoUiEditRoof>,
    _roofs: Query<&Roof>,
    _cache: ResMut<RoofParamsCacheResource>,
    _archivist: ResMut<country_core::resources::walls::decorator_archivist::changes::DecoratorArchivist<()>>,
    _storage: Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,
    _inner_walls: ResMut<InnerWalls>,
    _cursor: EventReader<country_core::resources::window::CursorMovedScreenSpace>,
) {
    // 真逻辑（decomp-0x141a19a90）：action 枚举、InnerWalls 墙高、DecoratorArchivist
    // 归档、RoofEdit 0x40/0x98 事件构造。未实现。
}
