//! autoclose_color_menu.rs — 相机远离→菜单自动关闭（B 0x142298d30，O；
//! re-analyst-A 契约 §2）。
//!
//! 语义（O）：`MainCamera::eye_position` ×2 → 与菜单锚距离超阈值 → 关闭菜单
//! （HideColorMenuCmd），Vec grow 0x140562630。

use bevy_ecs::prelude::*;

use country_core::resources::events::{EscapeMenuJustOpened, ModeJustChanged};
use country_core::resources::mouse_terrain_raycast::CursorPositionSS;
use country_core::systems::camera_rig::main_camera::MainCamera;

use crate::radial_menu::ColorRadialMenu;
use crate::HideColorMenuCmd;

/// 自动关闭距离阈值（I：机器码阈值常量未显式解出；re-analyst §2 记为
/// 距离超阈值）。
pub const AUTOCLOSE_DISTANCE: f32 = 20.0;

/// `autoclose_color_menu` @0x142298d30（O 面）。
pub fn autoclose_color_menu(
    mut modes: EventReader<ModeJustChanged>,
    mut escapes: EventReader<EscapeMenuJustOpened>,
    menus: Query<&ColorRadialMenu>,
    _cursor: Res<CursorPositionSS>,
    camera: Res<MainCamera>,
    mut hides: EventWriter<HideColorMenuCmd>,
) {
    let _ = modes.read().next();
    let _ = escapes.read().next();
    let eye = camera.eye_position();
    for menu in menus.iter() {
        // O：与菜单锚距离超阈值 → 关闭菜单。
        let anchor = glam::Vec3::new(menu.center.x, 0.0, menu.center.y);
        if (anchor - eye).length() > AUTOCLOSE_DISTANCE {
            hides.write(HideColorMenuCmd);
        }
    }
}
