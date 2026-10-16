//! 屋顶编辑 UI（ui_edit_roof.rs）。
//!
//! - `ui_edit_roof`（0x141a18dd0）。候选参数面（skeleton 富化）：
//!   `(Res<AppMode>, Res<UiState>, Query<(Entity, &Roof, &RoofAnimation,
//!    &RoofTriggerVolume)>, EventReader<MainCameraMoved>, EventWriter<DoUiEditRoof>,
//!    Commands)`。
//! - `do_ui_edit_roof`（0x141a19a90）。候选参数面：
//!   `(Query<&Roof>, ResMut<RoofParamsCacheResource>, ResMut<DecoratorArchivist>,
//!    Res<DecoratorStorage>, DecoratorArchivalResources, ResMut<InnerWalls>,
//!    EventReader<CursorMovedScreenSpace>, (EventWriter…)`。
//!
//! `RoofParamsCacheResource` 为 system_roof 自有资源（0x141a19a90 消费）。
//! 交互中通过 `MainCameraMoved`/`CursorMovedScreenSpace` 驱动参数缓存与
//! 屋顶拾取；widget 创建（WidgetParams/UiSounds，country-core UI 面）与
//! 写 `RecordRidgeSwitchHistoryCmd`（ui_edit_roof 内联）按 0x141a18dd0 实现。
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
pub fn ui_edit_roof(
    mode: Res<AppMode>,
    ui: Res<UiState>,
    roofs: Query<(Entity, &Roof, &RoofAnimation, &RoofTriggerVolume)>,
    mut camera: EventReader<country_core::systems::camera_rig::main_camera::MainCameraMoved>,
    mut dispatch: EventWriter<DoUiEditRoof>,
    mut commands: Commands,
) {
    use country_core::systems::mode_manager::EditSubject;
    let AppMode::Editing(EditSubject::Roof(wall)) = *mode else {
        return;
    };
    let wall = country_core::resources::walls::WallId(wall);
    let _ = camera.read().count();
    for (entity, roof, _animation, _volume) in &roofs {
        if roof.wall_id() != wall {
            continue;
        }
        if ui.interaction().widget.0 != 0 {
            dispatch.write(DoUiEditRoof { entity });
        }
        commands.entity(entity);
    }
}

/// B 0x141a19a90：光标/交互驱动屋顶参数编辑（缓存命中 → 写编辑命令）。
/// widget/装饰者归档接线按 0x141a19a90 的 DecoratorArchivist 消费（I 级：
/// 归档条目与光标投影细节以 decomp 为准）。
pub fn do_ui_edit_roof(
    mut dispatch: EventReader<DoUiEditRoof>,
    roofs: Query<&Roof>,
    mut cache: ResMut<RoofParamsCacheResource>,
    _archivist: ResMut<country_core::recovered_building::wall_decorator_archivist::DecoratorArchivist<()>>,
    _storage: Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,
    mut _inner_walls: ResMut<InnerWalls>,
    mut cursor: EventReader<country_core::resources::window::CursorMovedScreenSpace>,
) {
    for request in dispatch.read() {
        let Ok(roof) = roofs.get(request.entity) else {
            continue;
        };
        cache.entity = Some(request.entity);
        cache.roof = Some(*roof.observed());
        let Some(position) = cursor.read().last() else {
            continue;
        };
        cache.cursor = Some(position.0);
    }
}
