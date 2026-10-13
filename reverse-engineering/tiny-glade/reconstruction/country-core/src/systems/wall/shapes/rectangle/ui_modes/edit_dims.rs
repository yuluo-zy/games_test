//! shapes/rectangle/ui_modes/edit_dims.rs — rectangle dims edit widget.
//!
//! PDB procedures: `UiRectangleDimsEdit::{new, widget, signifier, trigger_volume}`,
//! `impl$0::to_widget_name_hash` (0x14155eba0..). Widget-layer (O call chain).

use crate::typed_gap::Gap;
use crate::ui::signifiers::SignifierEntityBuilder;
use crate::ui::widget_params::WidgetParams;
use crate::utils::curve::CurveU;

/// Rectangle dims edit widget (O).
#[derive(Clone, Debug)]
pub struct UiRectangleDimsEdit {
    pub raw: Gap,
}
impl UiRectangleDimsEdit {
    pub fn new() -> Self {
        Self { raw: Gap }
    }
    pub fn trigger_volume(&self) -> Gap {
        Gap
    }
    pub fn signifier(&mut self, _builder: SignifierEntityBuilder) {
        let _ = CurveU { raw: Gap };
    }
    pub fn widget(&mut self, _params: WidgetParams) {}
    pub fn to_widget_name_hash(&self) -> u32 {
        0
    }
}
