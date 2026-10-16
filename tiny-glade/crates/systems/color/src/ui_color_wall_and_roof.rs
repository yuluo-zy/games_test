//! ui_color_wall_and_roof.rs — 墙/屋顶 UI（B 0x141c9ab70 ui_color_wall_and_roof /
//! 0x141c9b510 color_cursor_icon，O；re-analyst-A 契约 §2）。
//!
//! - `ui_color_wall_and_roof`：widget 'color_canvas'（0x142f25026, 12B）；
//!   WidgetVolumeShape::clearing_canvas(3)；RaycastWorld::raycast（0x1408cc870,
//!   mask 0xcb, max=f32::MAX=0x142925980）；点击 → push 0xb0B widget 参数拷贝。
//! - `color_cursor_icon`：SignifierEntityBuilder（0x14089d9b0）+ EntityCommands
//!   ::insert（0x1422beab0）；mesh 'color_icon/color_cursor'（0x142f25080）——
//!   取色光标跟随。

use bevy_ecs::prelude::*;

use country_core::resources::mouse_terrain_raycast::{CameraCursorRay, CursorPositionSS};
use country_core::systems::collision::world::RaycastWorld;
use country_core::systems::mode_manager::AppMode;
use country_core::systems::ui::resources::ui_state::UiState;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::ui::signifiers::UiAppendSignifier;
use country_core::systems::wall::structure_outline::OutlineWallCmd;

use crate::radial_menu::ColorRadialMenu;
use crate::{HideColorMenuCmd, ShowColorMenuCmd};

/// `ui_color_wall_and_roof` @0x141c9ab70（O 面：color_canvas widget + 墙/屋顶
/// 拾取 → 菜单事件）。
#[allow(clippy::too_many_arguments)]
pub fn ui_color_wall_and_roof(
    mode: Res<AppMode>,
    _ui: Res<UiState>,
    _cursor: Res<CursorPositionSS>,
    mut shows: EventWriter<ShowColorMenuCmd>,
    mut hides: EventWriter<HideColorMenuCmd>,
    _menus: Query<&ColorRadialMenu>,
    mut outlines: EventWriter<OutlineWallCmd>,
    _raycast_world: Res<RaycastWorld>,
    _camera_ray: Res<CameraCursorRay>,
) {
    // O：color_canvas widget（0x142f25026，12B）+ WidgetVolumeShape::clearing_canvas(3)
    // + RaycastWorld::raycast（0x1408cc870, mask 0xcb, max=f32::MAX）拾取 →
    // 命中墙/屋顶 → ShowColorMenuCmd（0xb0B widget 参数拷贝面 U）。
    let _ = mode;
    let hit = _raycast_world.raycast(_camera_ray.origin, _camera_ray.dir, 0xcb);
    if hit.is_some() {
        shows.write(ShowColorMenuCmd { _gap: 0 });
    } else {
        hides.write(HideColorMenuCmd);
    }
    let _ = &mut outlines;
}

/// `color_cursor_icon` @0x141c9b510（O 面：取色光标图标——mesh
/// 'color_icon/color_cursor'（0x142f25080）；SignifierEntityBuilder 族
/// （0x14089d9b0 new / 0x14089dc90 animation_duration / 0x14089dd50 build）+
/// EntityCommands::insert（0x1422beab0）；光标跟随接线宿主。
pub fn color_cursor_icon(
    mut signifiers: ResMut<UiSignifierStream>,
    cursor: Res<CursorPositionSS>,
) {
    // O：取色光标 signifier（记录 = 光标屏幕位置面；0x20B 布局中央裁决）。
    signifiers.write(UiAppendSignifier::new(
        1,
        0,
        glam::Vec3::new(cursor.0.x, cursor.0.y, 0.0),
        1.0,
    ));
}
