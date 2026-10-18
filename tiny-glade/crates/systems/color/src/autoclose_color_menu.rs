//! autoclose_color_menu.rs — 相机远离→菜单自动关闭（B 0x142298d30，1207B；
//! restorer-state-color 2026-10-17 fresh 反编译）。
//!
//! **未实现**（分歧点）：真实控制流（fresh 反编译）：
//! - 双事件源（stride **0x28**）遍历，任一事件 `kind==0xc` → 直接进入关闭路径。
//!   当前 core 的 `ModeJustChanged`/`EscapeMenuJustOpened` 为 unit struct（stride
//!   8、无 kind 字段），与反编译 stride 0x28 不符——事件源类型 U（GAP）。
//! - 图标遍历（icon 世界位置经装饰器目录 stride 0xa0/墙目录 stride 0x48 定位，
//!   core `CustomizationIcon` typed-gap 未建模）：与光标（in_R9）2D 距离
//!   `size.y*2.0 + size.x*1.2 < dist` → 跳过该图标；与 `MainCamera::eye_position()`
//!   3D 距离 `> 10.0` → 关闭。
//! - 关闭 = 向命令写者写 HideColorMenuCmd。
//! **FIX-3（reviewer-state-color）**：本实现用 `menu.center` 近似图标世界位置、
//! 省略光标 2D 跳过、忽略 kind==0xc 直闭——行为不等原二进制，仅作 stub 登记，
//! **勿当判等**；真实逻辑待 core 图标目录/事件源布局恢复。

use bevy_ecs::prelude::*;

use country_core::resources::events::{EscapeMenuJustOpened, ModeJustChanged};
use country_core::systems::camera_rig::main_camera::MainCamera;

use crate::radial_menu::ColorRadialMenu;
use crate::HideColorMenuCmd;

/// 自动关闭距离阈值（O：反编译 3D 距离 `> 10.0`）。
pub const AUTOCLOSE_DISTANCE: f32 = 10.0;

/// `autoclose_color_menu` @0x142298d30（1207B，未实现——事件源/图标目录未对齐；
/// eye_position 距离闭合面为 **stub**（FIX-3，行为不等原二进制））。
pub fn autoclose_color_menu(
    mut modes: EventReader<ModeJustChanged>,
    mut escapes: EventReader<EscapeMenuJustOpened>,
    menus: Query<&ColorRadialMenu>,
    _cursor: Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,
    camera: Res<MainCamera>,
    mut hides: EventWriter<HideColorMenuCmd>,
) {
    // O：事件源 kind==0xc 检查（反编译 `*event == 0xc` 直接关闭）——GAP：
    // core 事件为 unit struct，无法读取 kind。
    let _ = (modes.read().next(), escapes.read().next());
    let eye = camera.eye_position();
    for menu in menus.iter() {
        // FIX-3 stub：真实逻辑按「图标世界位置」判 3D 距离 >10（目录 typed-gap 未
        // 恢复）；此处用 menu.center 近似，仅登记非判等 stub。
        let anchor = glam::Vec3::new(menu.center.x, 0.0, menu.center.y);
        if (anchor - eye).length() > AUTOCLOSE_DISTANCE {
            hides.write(HideColorMenuCmd);
        }
    }
}
