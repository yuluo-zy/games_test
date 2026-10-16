//! `widget_params.rs` — WidgetParams 0xb0 B 参数块（B 0x1409a8e10 起，O）。
//!
//! 证据：`chapters/systems/ui/resources/widget_params.rs.md`、`ui-input/DIGEST.md` §2.4：
//! - 默认 u32 组 `{0, 8, 0, 1, 0, 2, 3, 3}` @ `{0, 8, 0x10, 0x18, 0x20, 0x30, 0x48, 0x60}`；
//! - `UiSounds` 4×24 B @0x18..0x78；`WidgetVolumeShape` 32 B {tag u8@0, layer u8@1,
//!   size Vec2@0x10}；
//! - u16 @0xa8 = 0x101；flags 字节 @0xa9（bit0 = no_history_recording、bit2 =
//!   allow_negative_distance）；`occlude_by_world` = 字节 @0xa8 整字节（review-audit 纠错，
//!   非 bit0）；
//! - `UiFocus` u32 {None=0, Toolbar=1, Mei=2, World=3}；
//! - `clearing_canvas` volume = {tag 4, layer, 130×130}（`0x43020000_43020000` 双 130.0）。
use glam::Vec2;

/// 默认 u32 组（O：0x1409a8e10 默认值序列）。
pub const DEFAULT_U32S: [u32; 8] = [0, 8, 0, 1, 0, 2, 3, 3];

/// 焦点类型（O：u32 判别 {None=0, Toolbar=1, Mei=2, World=3}）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u32)]
pub enum UiFocus {
    #[default]
    None = 0,
    Toolbar = 1,
    Mei = 2,
    World = 3,
}

impl UiFocus {
    /// 0x140985f30：`== 0`。
    pub fn is_none(self) -> bool {
        self == Self::None
    }
    /// 0x1409a9150：`== 1`。
    pub fn is_toolbar(self) -> bool {
        self == Self::Toolbar
    }
    /// 0x1409a9140：`== 2`（与 WelcomeScreenState::is_closed 同址折叠，语义巧合）。
    pub fn is_mei(self) -> bool {
        self == Self::Mei
    }
    /// 0x1409a9130：转 Mei。
    pub fn as_mei(self) -> Self {
        Self::Mei
    }
}

/// 音量形状（32 B：tag u8@0、layer u8@1、size Vec2@0x10；O）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WidgetVolumeShape {
    pub tag: u8,
    pub layer: u8,
    pub size: Vec2,
}

impl WidgetVolumeShape {
    /// 0x1409a9110：`clearing_canvas` = {tag 4, layer, 130×130}。
    pub fn clearing_canvas(layer: u8) -> Self {
        Self {
            tag: 4,
            layer,
            size: Vec2::splat(130.0),
        }
    }
}

/// 声音槽（4×24 B @0x18..0x78；O 布局）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UiSounds {
    pub slots: [[u8; 24]; 4],
}

impl UiSounds {
    /// 0x1409a8fb0：清 4 槽首 u32（偏移 0/0x18/0x30/0x48）。
    pub fn empty() -> Self {
        Self { slots: [[0; 24]; 4] }
    }
}

/// Widget 标志字节 @0xa9（O：bit0 = no_history_recording、bit2 = allow_negative_distance）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WidgetFlags(pub u8);

impl WidgetFlags {
    pub const NO_HISTORY_RECORDING: u8 = 1 << 0;
    pub const ALLOW_NEGATIVE_DISTANCE: u8 = 1 << 2;

    /// 0x1409a8f10：字节 @0xa9 bit0。
    pub fn no_history_recording(&self) -> bool {
        self.0 & Self::NO_HISTORY_RECORDING != 0
    }
    /// 0x1409a8f40：字节 @0xa9 bit2。
    pub fn allow_negative_distance(&self) -> bool {
        self.0 & Self::ALLOW_NEGATIVE_DISTANCE != 0
    }
}

/// 0xb0 B 参数块（O：默认 u32 组 + UiSounds + WidgetVolumeShape + u16@0xa8=0x101 +
/// flags@0xa9）。
#[derive(Clone, Debug, PartialEq)]
pub struct WidgetParams {
    /// 默认 u32 组（@0/8/0x10/0x18/0x20/0x30/0x48/0x60）。
    pub fields: [u32; 8],
    /// 声音槽（@0x18..0x78）。
    pub sounds: UiSounds,
    /// 音量形状（@0x78..0xa8）。
    pub volume: WidgetVolumeShape,
    /// @0xa8：u16 = 0x101 + 整字节 occlude_by_world（O：review-audit 纠错）。
    pub occlude_by_world: bool,
    /// @0xa9 flags 字节。
    pub flags: WidgetFlags,
}

impl Default for WidgetParams {
    fn default() -> Self {
        Self::new()
    }
}

impl WidgetParams {
    /// 0x1409a8e10：0xb0 B 默认值序列。
    pub fn new() -> Self {
        Self {
            fields: DEFAULT_U32S,
            sounds: UiSounds::empty(),
            volume: WidgetVolumeShape::default(),
            occlude_by_world: false,
            flags: WidgetFlags(0),
        }
    }

    /// 0x1409a8e80：写 96 B 声音区。
    pub fn sounds(mut self, sounds: UiSounds) -> Self {
        self.sounds = sounds;
        self
    }

    /// 0x1409a8ee0：`occlude_by_world` = 字节 @0xa8 整字节（非 bit0）。
    pub fn occlude_by_world(mut self, value: bool) -> Self {
        self.occlude_by_world = value;
        self
    }

    /// 0x1409a8f10：`no_history_recording` = 字节 @0xa9 bit0。
    pub fn no_history_recording(mut self) -> Self {
        self.flags.0 |= WidgetFlags::NO_HISTORY_RECORDING;
        self
    }

    /// 0x1409a8f40：`allow_negative_distance` = 字节 @0xa9 bit2。
    pub fn allow_negative_distance(mut self) -> Self {
        self.flags.0 |= WidgetFlags::ALLOW_NEGATIVE_DISTANCE;
        self
    }
}
