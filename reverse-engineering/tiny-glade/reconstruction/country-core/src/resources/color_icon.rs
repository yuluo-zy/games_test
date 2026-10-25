//! color_icon.rs — 调色菜单图标组件（WP21）。
//!
//! O：`ColorMenuBrightnessValue::to_f32` 0xb06770 / `default_f32` 0xb06780；15+ 图标
//! 组件各自 `register_required_components`（0xb06b20..0xb0c550，多数折叠）。

use bevy_ecs::component::Component;

use crate::typed_gap::Gap;

/// `ColorMenuBrightnessValue`（O：亮度值枚举；to_f32/default_f32）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ColorMenuBrightnessValue {
    #[default]
    Default,
    Custom(f32),
}

impl ColorMenuBrightnessValue {
    /// `to_f32`（O 0xb06770）。
    pub fn to_f32(self) -> f32 {
        match self {
            ColorMenuBrightnessValue::Default => 1.0,
            ColorMenuBrightnessValue::Custom(v) => v,
        }
    }
    /// `default_f32`（O 0xb06780）。
    pub fn default_f32() -> f32 {
        1.0
    }
}

/// 图标组件（O：Bevy 组件注册；布局 U）。语义：调色菜单各图标。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ColorIcon(pub Gap);
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ColorWheelSeparator(pub Gap);
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct MagicColorIcon(pub Gap);
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct WallStyleIcon(pub Gap);
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct FlatRoofStyleIcon(pub Gap);
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct FlatRoofRimTypeToggle(pub Gap);
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct PlasterPeelIcon(pub Gap);
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct CustomizationIcon(pub Gap);
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ChimneySmokeToggle(pub Gap);
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct FreehandHeightTypeToggleIcon(pub Gap);
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct StairRailingIcon(pub Gap);
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ColorMenuBrightnessIcon(pub Gap);
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct FlagConnectionIcon(pub Gap);
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct CustomizationIconMarkedForDeletion(pub Gap);

/// `impl$N::register_required_components`（O：各图标组件注册面；折叠 stub）。
macro_rules! register_icon {
    ($($t:ty),*) => {$(
        impl $t {
            pub fn register_required_components() {}
        }
    )*};
}
register_icon!(
    ColorIcon,
    ColorWheelSeparator,
    MagicColorIcon,
    WallStyleIcon,
    FlatRoofStyleIcon,
    FlatRoofRimTypeToggle,
    PlasterPeelIcon,
    CustomizationIcon,
    ChimneySmokeToggle,
    FreehandHeightTypeToggleIcon,
    StairRailingIcon,
    ColorMenuBrightnessIcon,
    FlagConnectionIcon,
    CustomizationIconMarkedForDeletion
);
