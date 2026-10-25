//! B `systems/wall/shapes/rectangle/edit_rectangle.rs`: rectangle edit apply.
//!
//! Evidence: `chapters/systems/wall/shapes/rectangle/edit_rectangle.rs.md`
//! (O, 0x14097f620). `EditRectangleCmd` events rebuild the rectangle curve and
//! enclosure on the target wall. The roof-height ease event is render/UI
//! domain (待接线).

use bevy_ecs::prelude::*;
use ::utils::geometry::Rectangle2d;

use super::super::{EditRectangleCmd, to_authority_curve};
use crate::resources::walls::publication_fields::WallShapeSnapshot;
use crate::resources::walls::InnerWalls;
use crate::systems::wall::valid_enclosure::{Shape, ShapeParameters};

/// B 0x14097f620 (5-param system).
pub fn edit_rectangle(
    mut requests: EventReader<EditRectangleCmd>,
    mut inner: ResMut<InnerWalls>,
) {
    for event in requests.read() {
        let rect = Rectangle2d::new(event.center, event.dimensions, event.orientation);
        let curve = rect.as_curve2();
        inner.update_wall_state_curve(event.wall, to_authority_curve(&curve));
        let snapshot = WallShapeSnapshot::from_observed(
            ShapeParameters::from_shape(Shape::Rectangle(rect)).to_record(),
        );
        inner.update_wall_state_enclosure(event.wall, snapshot);
    }
}
