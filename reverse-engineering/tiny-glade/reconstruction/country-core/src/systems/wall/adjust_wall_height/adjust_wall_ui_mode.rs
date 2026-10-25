//! adjust_wall_height/adjust_wall_ui_mode.rs — wall-height UI widget + signifier.
//!
//! PDB procedures: `UiWallHeight::{widget, signifier, new}`, `adjust_wall_height_curve`
//! (0x1409566d0..0x1409576d0). Widget-layer functions (O call chain): builds the height
//! signifier/curve UI that drives `adjust_wall_height_curve` interpolation.

use crate::ui::signifiers::SignifierEntityBuilder;
use crate::ui::widget_params::WidgetParams;
use crate::utils::curve::Curve;

/// Wall-height UI widget (O: `UiWallHeight::widget/signifier/new`).
///
/// Fresh B evidence（2026-10-17）：
/// - `UiWallHeight::widget` @0x1409566d0：读 `self+9`（模式字节）；0 分支
///   高度 +0.6，非 0 分支按 `+0.0` / `-0.29` 偏移。
/// - `UiWallHeight::signifier` @0x140956cd0：`if *(u8*)(self+9)==1`；
///   `self+0`（u64）写入 signifier 记录；`self+8`（bool）选择 +0.6 或
///   `0x3e4ccccd`（0.2? 浮点常量）权重。格式串前缀 `wall_height_edit_`
///   @0x142ab47b8。
/// - 观测字段：`u64@0, bool@8, bool@9`；尾部字节 U（观测下界 10B，
///   按 8 对齐取 16B 载体）。
#[repr(C)]
#[derive(bevy_ecs::resource::Resource, Clone, Copy, Debug, Default)]
pub struct UiWallHeight {
    /// @+0：写入 signifier 记录的 u64（语义 U）。
    pub raw_id: u64,
    /// @+8：signifier 权重选择。
    pub flag8: bool,
    /// @+9：模式门（signifier 仅在该字节为 1 时构造）。
    pub active: bool,
}
impl UiWallHeight {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn widget(&mut self, _params: WidgetParams) {}
    pub fn signifier(&mut self, _builder: SignifierEntityBuilder) {}
}

/// O: `adjust_wall_height_curve` — interpolated height curve for the UI.
pub fn adjust_wall_height_curve(_curve: &Curve, _t: f32) -> Curve {
    _curve.clone()
}

#[cfg(test)]
mod layout_tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn ui_wall_height_observed_offsets() {
        assert_eq!(offset_of!(UiWallHeight, raw_id), 0);
        assert_eq!(offset_of!(UiWallHeight, flag8), 8);
        assert_eq!(offset_of!(UiWallHeight, active), 9);
        assert_eq!(size_of::<UiWallHeight>(), 16);
    }
}
