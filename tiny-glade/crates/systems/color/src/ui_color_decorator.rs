//! ui_color_decorator.rs — 窗/旗 UI（B 0x1422b0e00，O；re-analyst-A 契约 §2）。
//!
//! 语义（O）：legacy_raycast_w_params（0x1408cc770）拾取；DecoratorStorage
//! lookup 族（0x140a1c930/0x140a1c3e0/0x140a1c5f0）；StairsState::graph
//! （0x1408fe8b0）；DerivedDecoratorInfo::subtype（0x140b26970）+0x140b26a70；
//! 0x1425f4610/0x1425f4950（装饰物→颜色 UI 辅助）；EventWriter ×3
//! （0x142291100 菜单事件族，类型名 U）。

use bevy_ecs::prelude::*;

use country_core::resources::mouse_terrain_raycast::{CameraCursorRay, CursorPositionSS};
use country_core::resources::stairs::stairs_state::StairsState;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::systems::collision::world::RaycastWorld;
use country_core::systems::mode_manager::AppMode;
use country_core::systems::ui::resources::ui_state::UiState;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::wall::structure_outline::OutlineStairsCmd;

use crate::{HideColorMenuCmd, ShowColorMenuCmd};

/// `ui_color_windows_and_flags` @0x1422b0e00（O 面：窗/旗装饰物拾取 → 菜单事件；
/// 事件类型名族（0x142291100 ×3）U——re-analyst §6）。
#[allow(clippy::too_many_arguments)]
pub fn ui_color_windows_and_flags(
    mode: Res<AppMode>,
    _ui: Res<UiState>,
    _storage: Res<DecoratorStorage>,
    mut shows: EventWriter<ShowColorMenuCmd>,
    mut hides: EventWriter<HideColorMenuCmd>,
    _cursor: Res<CursorPositionSS>,
    _transforms: Res<CachedDecoratorTransforms>,
    _signifiers: Res<UiSignifierStream>,
    _walls: Res<PublicWalls>,
    _raycast_world: Res<RaycastWorld>,
    _camera_ray: Res<CameraCursorRay>,
    mut outlines: EventWriter<OutlineStairsCmd>,
    _stairs: Res<StairsState>,
) {
    // O：legacy_raycast_w_params（0x1408cc770）拾取装饰物 → 窗/旗 → 菜单事件
    // （0x142291100 事件族 ×3，类型名 U——re-analyst §6）；0x1425f4610/
    // 0x1425f4950（装饰物→颜色 UI 辅助）面登记。
    let _ = mode;
    if _raycast_world.raycast(_camera_ray.origin, _camera_ray.dir, 0xcb).is_some() {
        shows.write(ShowColorMenuCmd { _gap: 0 });
    } else {
        hides.write(HideColorMenuCmd);
    }
    let _ = &mut outlines;
}
