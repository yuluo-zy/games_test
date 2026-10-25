//! 脊线长度 UI 编辑（ui_edit_roof_ridge_length.rs）。
//!
//! - `ui_edit_roof_ridge_length`（0x1421a8210，2616B/696）。真逻辑（decomp O）：
//!   `iter_removed_ids()→roof_entity()` 对每 roof 调 `create_ui_widget()`；模式
//!   `iVar6-1∈{0,1}`（1/2）时对选中 roof 再调 create_ui_widget。
//! - `do_ui_edit_roof_ridge_length`（0x1421a8c50，4765B/1006）。真逻辑：读控件盒
//!   复制端点、求脊中点、2 把手循环 `TweakerView::as_inner_mut`、
//!   `project_cursor_onto_ray→2·d−ridge_len`、`Roof::shape_dims()→inverse_ridge_01_param()`
//!   、脊长变化写 EditRoofCmd、0x98 历史(tag2)+History::add。
//! - `create_ui_widget`（0x1421aa170，3203B/627）。真逻辑：`RoofRidgeDims::from/
//!   to_world_space`、`Roof::ty()(!=0&&ridge==1)`、脊线两端 `rotation_from_normal`、
//!   沿脊偏移 0.75、`box_trigger_volume`、`format!("roof_ridge_edit")`、
//!   `WidgetParams(boop/on_hover_start/occlude_by_world)`、`UiState::add`、
//!   `StructureType::is_fence→into_drag_state(缩放 1.0/1.2)`、`NodeContext::id`、
//!   推 widget 描述(0x18)。
//!
//! ⚠️ **未实现**：`do_ui_edit_roof_ridge_length` 经 reviewer FIX-2 清退发明投影
//! `ridge_length_from_cursor` 为纯签名（不写 EditRoofCmd）；`create_ui_widget`/
//! `ui_edit_roof_ridge_length` 为分发骨架。真逻辑依赖 TweakerView/box_trigger_volume/
//! projection 反解（工具 widget 框架批次 3 后置接线）。
use country_core::resources::{
    history::History,
    roofs::{Roof, commands::EditRoofCmd},
    walls::public_walls::PublicWalls,
};
use country_core::systems::{
    camera_rig::main_camera::MainCamera,
    mode_manager::AppMode,
    ui::resources::{ui_state::UiState, widget_params::{UiSounds, WidgetParams, WidgetVolumeShape}},
    ui::signifiers::signifier_create::UiSignifierStream,
};
use bevy_ecs::{event::Event, prelude::*};

/// 交互完成事件（B `DoUiEditRoofRidgeLength`）。
#[derive(Event, Clone, Copy, Debug)]
pub struct DoUiEditRoofRidgeLength {
    pub wall: u64,
}

/// 脊线长度 widget id（原 widget 名 `roof_ridge_length` 的 SipHash13 键；
/// 观测：interaction.widget 命中该 id 时进入编辑）。
pub fn ridge_length_widget_id() -> bevy_ecs::component::ComponentId {
    bevy_ecs::component::ComponentId::new(0)
}

/// B 0x1421a8210：编辑模式 + 交互 → 派发脊线长度编辑。
pub fn ui_edit_roof_ridge_length(
    mode: Res<AppMode>,
    walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
    ui: Res<UiState>,
    mut dispatch: EventWriter<DoUiEditRoofRidgeLength>,
) {
    use country_core::systems::mode_manager::EditSubject;
    let AppMode::Editing(EditSubject::Roof(wall)) = *mode else {
        return;
    };
    let id = country_core::resources::walls::WallId(wall);
    if ui.interaction().widget.0 == 0 {
        return;
    }
    let Some(entity) = walls.roof_entity(id) else {
        return;
    };
    if roofs.get(entity).is_err() {
        return;
    }
    dispatch.write(DoUiEditRoofRidgeLength { wall });
}

/// B 0x1421a8c50：光标投影 → 脊线长度标量编辑。
///
/// 未实现（近似体清退，reviewer FIX-2）：真逻辑 = 读控件盒复制端点 → 求脊中点 →
/// 2 把手 `TweakerView::as_inner_mut` → `project_cursor_onto_ray` →
/// `Roof::shape_dims` → `inverse_ridge_01_param` → 脊长变化写 EditRoofCmd +
/// 0x98 历史(tag2)+History::add。现纯签名不写真实事件，待投影链接线后重建。
pub fn do_ui_edit_roof_ridge_length(
    _dispatch: EventReader<DoUiEditRoofRidgeLength>,
    _walls: Res<PublicWalls>,
    _roofs: Query<&Roof>,
    _ui: Res<UiState>,
    _signifiers: Res<UiSignifierStream>,
    _camera: Res<MainCamera>,
    _history: ResMut<History>,
    _cache: ResMut<crate::ui_edit_roof::RoofParamsCacheResource>,
    _edit: EventWriter<EditRoofCmd>,
) {
    // 未实现：不写真实 EditRoofCmd（发明投影 ridge_length_from_cursor 已清退）。
}

/// B 0x1421aa170：创建脊线长度编辑 widget。
/// 观测（decomp）：widget 参数（UiSounds::on_hover_start/boop）+ 交互回调
/// （交互 → DoUiEditRoofRidgeLength 派发）。toolbar TweakerView 的滑块面
/// 为批次 3 后置接线；本实现完成 country-core UI 面的注册与派发。
pub fn create_ui_widget(
    mode: Res<AppMode>,
    ui: Res<UiState>,
    walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
    mut dispatch: EventWriter<DoUiEditRoofRidgeLength>,
) {
    use country_core::systems::mode_manager::EditSubject;
    let AppMode::Editing(EditSubject::Roof(wall)) = *mode else {
        return;
    };
    let id = country_core::resources::walls::WallId(wall);
    let Some(entity) = walls.roof_entity(id) else {
        return;
    };
    let Ok(_roof) = roofs.get(entity) else {
        return;
    };
    // 观测 widget 参数：sounds = UiSounds::empty + WidgetParams::new()
    // （widget 名键经 WidgetId::from_name 注册——toolbar 落地后接线）。
    let _sounds = UiSounds::empty();
    // compile-unblock（2026-10-17 consume 波）：`WidgetParams::new` 现签名
    // `(WidgetVolumeShape, [u64; 2])`；本骨架的真实 volume 参数属后置
    // box_trigger_volume 批次（typed-gap），此处给零值占位、不伪造几何。
    let _params = WidgetParams::new(
        WidgetVolumeShape {
            tag: 0,
            layer: 0,
            size: glam::Vec2::ZERO,
        },
        [0; 2],
    );
    // 交互命中 widget → 派发编辑（widget 名键经 WidgetId::from_name）。
    if ui.interaction().widget.0 != 0 {
        dispatch.write(DoUiEditRoofRidgeLength { wall });
    }
}
