//! B UiSettings schema and defaults. Original writes at 0x140b12f10 and
//! serialization at 0x140a46ad0 supersede the old field/default table.
use bevy_ecs::resource::Resource;
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolbarAlignment {
    #[default]
    Bottom = 0,
    Left = 1,
    Right = 2,
    Top = 3,
}

impl ToolbarAlignment {
    pub fn is_horizontal(self) -> bool {
        matches!(self, Self::Bottom | Self::Top)
    }
}

// Declaration order follows the original serializer, not native field order.
#[derive(Resource, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UiSettings {
    pub icon_size_mult: f32,
    pub toolbar_alignment: ToolbarAlignment,
    pub auto_hide_toolbar: bool,
    pub input_lag_reduction: bool,
    pub game_logo: bool,
    pub custom_cursor: bool,
    pub custom_cursor_size: f32,
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            icon_size_mult: 1.0,
            toolbar_alignment: ToolbarAlignment::Bottom,
            auto_hide_toolbar: false,
            input_lag_reduction: true,
            game_logo: true,
            custom_cursor: false,
            custom_cursor_size: 24.0,
        }
    }
}
