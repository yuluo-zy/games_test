//! B142298d30: close with cancellation when leaving Customize, opening Escape,
//! moving the cursor outside the radial catchment, or moving the camera away.
use crate::HideColorMenuCmd;
use crate::radial_menu::ColorRadialMenu;
use bevy_ecs::prelude::*;
use country_core::resources::events::{EscapeMenuJustOpened, ModeJustChanged};
use country_core::resources::mouse_terrain_raycast::CursorPositionSS;
use country_core::systems::camera_rig::main_camera::MainCamera;
pub const AUTOCLOSE_DISTANCE: f32 = 10.0;
pub fn autoclose_color_menu(
    mut modes: EventReader<ModeJustChanged>,
    mut escapes: EventReader<EscapeMenuJustOpened>,
    menus: Query<&ColorRadialMenu>,
    cursor: Res<CursorPositionSS>,
    camera: Res<MainCamera>,
    mut hides: EventWriter<HideColorMenuCmd>,
) {
    // Only the latest mode command participates, not any earlier command in
    // this reader batch. A non-Customize mode does not consume escape events.
    if modes.read().last().is_some_and(|mode| mode.current.native_tag() != 12) {
        hides.write(HideColorMenuCmd { cancel: true });
        return;
    }
    if escapes.read().last().is_some() {
        hides.write(HideColorMenuCmd { cancel: true });
        return;
    }
    let eye = camera.eye_position();
    for menu in &menus {
        let catchment = menu.icon_size * 2.0 + menu.radius * 1.2;
        if catchment < (cursor.0 - menu.center).length()
            || !((eye - menu.world_anchor).length() <= AUTOCLOSE_DISTANCE)
        {
            hides.write(HideColorMenuCmd { cancel: true });
            return;
        }
    }
}
