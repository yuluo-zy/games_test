//! ui_color_wall_and_roof.rs — 墙/屋顶颜色 UI（B 0x141c9ab70 ui_color_wall_and_roof /
//! 0x141c9b510 color_cursor_icon；restorer-state-color 2026-10-17 fresh 反编译）。
//!
//! **未实现**（分歧点）：`ui_color_wall_and_roof`（580B）真实控制流 =
//! `*(AppMode)==0xc` 模式守卫 → `WidgetVolumeShape::clearing_canvas(3)` +
//! `WidgetParams::new/sounds/occlude_by_world/signifier<closure_env_0>` +
//! `UiState::add("color_canvas...")` → `UiWidgetContext::clicked` → `RaycastWorld::
//! raycast`（mask 0xcb）→ 命中类型 0/2/6/7/8 分派 `bVar20`（3/位/2）→ PublicWalls
//! 墙实体定位（+0x108 两分支）→ `format!("...WallId")` → 控件构建（WidgetParams +
//! UiSounds on_hover_start/on_click + signifier closure_env_2 + `UiState::add<String>`）
//! → `is_active` 写值 1.0 与 `bVar20>1` bool → `clicked` 算 `cVar21`（clamp
//! (bVar20-2,..)：0→8、1→0、否则 (bVar20&1)+1）写 stride 0x20 命令。
//! 分歧点：core `WidgetParams::signifier<closure>` 泛型、raycast 命中类型字段
//! （core `RaycastWorldResult` 无 +0x1d8 命中类型面）、closure_env_0/2 未恢复——
//! 无法逐语句重建。
//! `color_cursor_icon`（580B）真实控制流 = `SignifierEntityBuilder::new/
//! animation_duration/build` + `insert(UiInactive)`（按 in_R8B）+ 串
//! `"color_icon/color_cursor"`（0x17B）+ `EntityCommands::insert<(Ui2dExtent,
//! Ui2dImage)>` + `insert(Ui2DFollowMouse)`。分歧点：core 组件 typed-gap。

use bevy_ecs::prelude::*;

use country_core::resources::mouse_terrain_raycast::{CameraCursorRay, CursorPositionSS};
use country_core::systems::mode_manager::AppMode;
use country_core::systems::ui::resources::ui_state::UiState;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::wall::structure_outline::OutlineWallCmd;

use crate::radial_menu::ColorRadialMenu;
use crate::{HideColorMenuCmd, ShowColorMenuCmd};

/// `ui_color_wall_and_roof` @0x141c9ab70（580B，未实现——widget/raycast 命中面阻断）。
#[allow(clippy::too_many_arguments)]
pub fn ui_color_wall_and_roof(
    mode: Res<AppMode>,
    _ui: Res<UiState>,
    mut shows: EventWriter<ShowColorMenuCmd>,
    mut hides: EventWriter<HideColorMenuCmd>,
    _menus: Query<&ColorRadialMenu>,
    _outlines: EventWriter<OutlineWallCmd>,
    _cursor: Res<CursorPositionSS>,
    _camera_ray: Res<CameraCursorRay>,
) {
    // O：模式 12 守卫（`*(int *)*in_RCX == 0xc` = AppMode::native_tag()==0xc）。
    if mode.native_tag() == 0xc {
        // O：RaycastWorld::raycast（mask 0xcb）命中 → ShowColorMenuCmd（类别由
        // 命中类型 0/2/6/7/8 分派）；widget/signifier 面未恢复（GAP）。
        let _ = (
            &mut shows,
            &mut hides,
            &_menus,
            &_outlines,
            &_cursor,
            &_camera_ray,
        );
    }
}

/// `color_cursor_icon` @0x141c9b510（580B，未实现——SignifierEntityBuilder 面
/// core 未对齐）。
pub fn color_cursor_icon(_signifiers: ResMut<UiSignifierStream>, _cursor: Res<CursorPositionSS>) {
    // O：构建取色光标图标（mesh "color_icon/color_cursor"，插入 Ui2dExtent/
    // Ui2dImage/Ui2DFollowMouse；SignifierEntityBuilder 面 core 未恢复——GAP）。
}
