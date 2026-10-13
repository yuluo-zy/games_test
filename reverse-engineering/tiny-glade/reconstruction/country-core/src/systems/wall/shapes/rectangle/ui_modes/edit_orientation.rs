//! shapes/rectangle/ui_modes/edit_orientation.rs — rectangle orientation edit widget.
//!
//! PDB procedures: `UiRectangleOrientationEdit::{new, widget, signifier}` (0x14083d3f0..).
//! Widget-layer (O call chain).

use crate::typed_gap::Gap;
use crate::ui::signifiers::SignifierEntityBuilder;
use crate::ui::widget_params::WidgetParams;
use crate::utils::curve::CurveInternalCoord;

/// Rectangle orientation edit widget (O).
#[derive(Clone, Debug)]
pub struct UiRectangleOrientationEdit {
    pub raw: Gap,
}
impl UiRectangleOrientationEdit {
    pub fn new() -> Self {
        Self { raw: Gap }
    }
    pub fn signifier(&mut self, _builder: SignifierEntityBuilder) {
        let _ = CurveInternalCoord::i0();
    }
    pub fn widget(&mut self, _params: WidgetParams) {}
    pub fn to_widget_name_hash(&self) -> u32 {
        0
    }
}
