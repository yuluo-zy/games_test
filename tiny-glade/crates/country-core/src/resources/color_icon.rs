//! B brightness selector, 0x140b06770. Variant spellings are not available;
//! preserve the three observed tags rather than inventing a Custom(f32) variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorMenuBrightnessValue(u8);

impl Default for ColorMenuBrightnessValue {
    fn default() -> Self {
        Self(1)
    }
}

impl ColorMenuBrightnessValue {
    pub fn from_tag(tag: u8) -> Option<Self> {
        (tag < 3).then_some(Self(tag))
    }

    pub fn tag(self) -> u8 {
        self.0
    }

    pub fn to_f32(self) -> f32 {
        [0.25, 1.0, 4.0][usize::from(self.0)]
    }

    pub fn default_f32() -> f32 {
        1.0
    }
}

// ---------------------------------------------------------------- 组 A 接线

/// B CustomizationIcon register@0x1422b58f0 records size 0x24/align 4.
/// ui_2d_pass reads alpha@0x18 and the flag byte@0x21. The animation system
/// 1422979b0 consumes the three Vec2 values and hover state below. The old
/// 0xa0 state record belongs to the color-menu algorithm, not this component.
#[repr(C)]
#[derive(bevy_ecs::component::Component, Clone, Debug, Default)]
pub struct CustomizationIcon {
    pub size: glam::Vec2,
    pub position: glam::Vec2,
    pub initial_position: glam::Vec2,
    pub alpha_18: f32,
    pub hover_progress: f32,
    pub hovered: u8,
    pub flag_21: u8,
    pub opaque_22: [u8; 2],
}

const _: () = assert!(std::mem::size_of::<CustomizationIcon>() == 0x24);
const _: () = assert!(std::mem::offset_of!(CustomizationIcon, alpha_18) == 0x18);
const _: () = assert!(std::mem::offset_of!(CustomizationIcon, flag_21) == 0x21);

/// B1425f3057 inserts the original u32 color payload into ColorIcon.
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
pub struct ColorIcon(pub u32);

/// B1425f3297 inserts the wall-style selector byte.
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
pub struct WallStyleIcon(pub u8);

#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
pub struct MagicColorIcon(pub u32);
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
pub struct PlasterPeelIcon(pub f32);
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
pub struct StairRailingIcon(pub u8);
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug)]
pub struct ColorMenuBrightnessIcon(pub ColorMenuBrightnessValue);
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug)]
pub struct FlatRoofStyleIcon(pub crate::resources::roofs::customization::FlatRoofStyleObserved);
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
pub struct ColorWheelSeparator;
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
pub struct FreehandHeightTypeToggleIcon;
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
pub struct FlagConnectionIcon;
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
pub struct FlatRoofRimTypeToggle;
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
pub struct ChimneySmokeToggle;

/// 图标删除标记（O：`resources::color_icon::CustomizationIconMarkedForDeletion`；
/// delete_color_icon_entities 0x142297ff0 的查询面）。
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CustomizationIconMarkedForDeletion;
