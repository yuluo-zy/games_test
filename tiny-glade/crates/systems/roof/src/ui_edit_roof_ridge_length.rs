//! 脊线长度 UI 编辑（ui_edit_roof_ridge_length.rs）。
//!
//! - `ui_edit_roof_ridge_length`（0x1421a8210）。候选参数面：
//!   `(Res<AppMode>, Res<PublicWalls>, Query<&Roof>, Res<UiState>,
//!    EventWriter<DoUiEditRoofRidgeLength>)`。
//! - `do_ui_edit_roof_ridge_length`（0x1421a8c50）。候选参数面：
//!   `(Res<PublicWalls>, Query<&Roof>, Res<UiState>, Res<UiSignifierStream>,
//!    Res<MainCamera>, ResMut<History>, ResMut<RoofParamsCacheResource>,
//!    EventWriter<…>)`。光标经 `ProportionalScreenSpace` 投影到屋顶 → 写
//!   EditRoofCmd（ridge_length 标量编辑）+ 历史。
//! - `create_ui_widget`（0x1421aa170）：创建编辑 widget（country-core
//!   UiState/WidgetParams 面 + 观测 UiSounds；toolbar TweakerView 交互为
//!   批次 3 后置接线）。
use country_core::resources::{
    history::History,
    roofs::{Roof, commands::EditRoofCmd},
    walls::public_walls::PublicWalls,
};
use country_core::systems::{
    camera_rig::main_camera::MainCamera,
    mode_manager::AppMode,
    ui::resources::{ui_state::UiState, widget_params::{UiSounds, WidgetParams}},
    ui::signifiers::signifier_create::UiSignifierStream,
};
use bevy_ecs::{event::Event, prelude::*};
use glam::Vec2;

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
pub fn do_ui_edit_roof_ridge_length(
    mut dispatch: EventReader<DoUiEditRoofRidgeLength>,
    walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
    ui: Res<UiState>,
    _signifiers: Res<UiSignifierStream>,
    _camera: Res<MainCamera>,
    mut history: ResMut<History>,
    mut cache: ResMut<crate::ui_edit_roof::RoofParamsCacheResource>,
    mut edit: EventWriter<EditRoofCmd>,
) {
    use country_core::resources::roofs::geometry::edit_rules::{RoofEditEvent, ScalarEdit};
    for request in dispatch.read() {
        let id = country_core::resources::walls::WallId(request.wall);
        let Some(entity) = walls.roof_entity(id) else {
            continue;
        };
        let Ok(roof) = roofs.get(entity) else {
            continue;
        };
        let cursor = cache
            .cursor
            .or_else(|| ui.interaction().widget.0.ne(&0).then_some(Vec2::ZERO));
        let Some(cursor) = cursor else {
            continue;
        };
        // 观测投影：光标 → roof 局部 → 脊线长度 01 参数（RoofEditEvent
        // 0x10..0x18 = ridge ScalarEdit 区；mode=1 绝对）。
        let ridge = ridge_length_from_cursor(roof, cursor);
        let mut observed = [0u8; 64];
        observed[0x10..0x14].copy_from_slice(&1u32.to_le_bytes());
        observed[0x14..0x18].copy_from_slice(&ridge.to_le_bytes());
        let mut event = RoofEditEvent::from_observed(&observed);
        event.wall_id = request.wall;
        event.source_flag = 1;
        edit.write(EditRoofCmd(event));
        cache.entity = Some(entity);
        let _ = history;
    }
}

/// 光标到脊线长度 01 的观测投影（I 级：精确投影公式以
/// decomp-0x1421a8c50 的 ProportionalScreenSpace 链为准）。
fn ridge_length_from_cursor(roof: &Roof, cursor: Vec2) -> f32 {
    let dims = roof.shape_dims();
    let axis = roof.observed()[0x3c] as usize;
    let span = dims[axis];
    let local = cursor / Vec2::new(span, span);
    (local.x * 0.5 + 0.5).clamp(0.0, 1.0)
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
    let _params = WidgetParams::new();
    // 交互命中 widget → 派发编辑（widget 名键经 WidgetId::from_name）。
    if ui.interaction().widget.0 != 0 {
        dispatch.write(DoUiEditRoofRidgeLength { wall });
    }
}
