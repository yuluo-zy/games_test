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

/// 自定义图标组件（O：`resources::color_icon::CustomizationIcon`；color_menu
/// 0x142291230 的图标状态条目（0xa0B 数组族）与 color_button_animations 消费；
/// 字段 U（typed gap）。
#[derive(bevy_ecs::component::Component, Clone, Debug, Default)]
pub struct CustomizationIcon {
    /// typed gap：0xa0B 图标状态条目字段。
    pub _gap: u64,
}

/// 颜色图标组件（O：`resources::color_icon::ColorIcon`；BASELINE
/// `Option<&ColorIcon>` 查询面；字段 U）。
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
pub struct ColorIcon;

/// 墙样式图标组件（O：`resources::color_icon::WallStyleIcon`；字段 U）。
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
pub struct WallStyleIcon;

/// 图标删除标记（O：`resources::color_icon::CustomizationIconMarkedForDeletion`；
/// delete_color_icon_entities 0x142297ff0 的查询面）。
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CustomizationIconMarkedForDeletion;
