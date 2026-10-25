//! Proposed canonical GUI-field patch, held here because GUI family is owned
//! by core-gui-20261015. Do not apply through another owner's claim.
//! Original 13 bytes zeroed @1405a1a36/41 before insert@1405a1a95.
use bevy_ecs::resource::Resource;

#[repr(C)]
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct TimeOfDayScreen {
    pub time_of_day: f32,         // +00
    pub sun_xz_rotation: f32,     // +04
    pub just_opened: bool,        // +08 = visible && !was_visible
    pub was_visible: bool,        // +09 = visible at GUI call end
    pub visible: bool,            // +0a = UI open
    pub time_input_active: bool,  // +0b = clock circle drag pending
    pub angle_input_active: bool, // +0c = angle circle drag pending
}
const _: () = assert!(core::mem::size_of::<TimeOfDayScreen>() == 16);
const _: () = assert!(core::mem::offset_of!(TimeOfDayScreen, visible) == 10);
