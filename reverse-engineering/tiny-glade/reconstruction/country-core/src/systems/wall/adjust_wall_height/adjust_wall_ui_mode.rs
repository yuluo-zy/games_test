//! adjust_wall_height/adjust_wall_ui_mode.rs — wall-height UI widget + signifier.
//!
//! PDB procedures: `UiWallHeight::{widget, signifier, new}`, `adjust_wall_height_curve`
//! (0x1409566d0..0x1409576d0). Widget-layer functions (O call chain): builds the height
//! signifier/curve UI that drives `adjust_wall_height_curve` interpolation.

use crate::typed_gap::Gap;
use crate::ui::signifiers::SignifierEntityBuilder;
use crate::ui::widget_params::WidgetParams;
use crate::utils::curve::Curve;

/// Wall-height UI widget (O: `UiWallHeight::widget/signifier/new`).
#[derive(bevy_ecs::resource::Resource, Clone, Debug)]
pub struct UiWallHeight {
    pub raw: Gap,
}
impl UiWallHeight {
    pub fn new() -> Self {
        Self { raw: Gap }
    }
    pub fn widget(&mut self, _params: WidgetParams) {}
    pub fn signifier(&mut self, _builder: SignifierEntityBuilder) {}
}

/// O: `adjust_wall_height_curve` — interpolated height curve for the UI.
pub fn adjust_wall_height_curve(_curve: &Curve, _t: f32) -> Curve {
    _curve.clone()
}
