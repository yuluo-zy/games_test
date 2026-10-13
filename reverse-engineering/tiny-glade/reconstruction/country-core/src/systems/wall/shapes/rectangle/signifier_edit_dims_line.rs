//! shapes/rectangle/signifier_edit_dims_line.rs — rectangle dims edit line signifier.
//!
//! PDB procedure `signifier_edit_dims` 0x1408dda10 (3525 B). FunctionSystem signature O.

use bevy_ecs::prelude::Component;
use bevy_ecs::system::{Query, Res, ResMut};

use crate::render::mesh::Mesh;
use crate::asset_libraries::Handle;
use crate::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::UiEditRectangleActiveWidgetResource;
use crate::typed_gap::Gap;
use crate::ui::signifiers::signifier_create::UiSignifierStream;
use crate::ui::signifiers::{SignifierAnimation, SignifierEntityBuilder, SignifierFactory};
use crate::utils::curve::CurveU;

/// Rectangle dims line preview marker (O).
#[derive(Component, Clone, Debug, Default)]
pub struct EditRectangleLinePreview {
    pub raw: Gap,
}

pub fn signifier_edit_dims(
    active_widget: Res<UiEditRectangleActiveWidgetResource>,
    mut signifiers: ResMut<UiSignifierStream>,
    mut query: Query<(&EditRectangleLinePreview, &Handle<Mesh>, &SignifierAnimation)>,
) {
    let _ = (&active_widget, &mut signifiers);
    for (_preview, _handle, animation) in query.iter_mut() {
        let _ = animation.t();
        let _ = CurveU { raw: Gap };
        let _ = SignifierEntityBuilder::new();
        let _ = SignifierFactory::new();
    }
}
