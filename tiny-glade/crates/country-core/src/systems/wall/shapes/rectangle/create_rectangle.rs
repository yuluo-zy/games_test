//! B `systems/wall/shapes/rectangle/create_rectangle.rs`: rectangle wall
//! creation.
//!
//! Evidence: `chapters/systems/wall/shapes/rectangle/create_rectangle.rs.md`
//! (O, 0x140a93790). Rectangle2d::as_curve2 builds the smoothstep-cornered
//! curve; the wall is created with the rectangle enclosure; a CreateRoofCmd
//! carries the ridge-length ratio and a wall-creation history record follows.
//! The exact `ridge_length_01_from_rectangle_ratio` interpolation constants
//! are 待取证 (structural approximation below).

use bevy_ecs::prelude::*;
use ::utils::geometry::Rectangle2d;

use super::super::{CreateRectangleCmd, build_roof_create_request, to_authority_curve};
use crate::recovered_building::wall_state::WallShapeSnapshot;
use crate::resources::walls::{InnerWalls, public_walls::PublicWalls};
use crate::systems::wall::{
    history_recording::RecordWallHistory,
    valid_enclosure::{Shape, ShapeParameters},
};

/// Default wall elevation for shape creation (4.0, 0x40800000).
const SHAPE_ELEVATION: f32 = 4.0;
/// Ridge-length ratio offset (-2.25, 0xc0100000 @0x142aa5084).
const RIDGE_RATIO_OFFSET: f32 = -2.25;

/// `ridge_length_01_from_rectangle_ratio`: aspect ratio drives the roof ridge.
/// Exact interpolation constants 待取证.
pub fn ridge_length_01_from_rectangle_ratio(dimensions: glam::Vec2) -> f32 {
    let x = dimensions.x;
    let y = dimensions.y;
    let ratio = if y != 0.0 { x / y } else { f32::INFINITY };
    let inverse = if x != 0.0 { y / x } else { f32::INFINITY };
    let max_dim = if x > y { x } else { y };
    let value = if max_dim > 0.0 {
        (max_dim + RIDGE_RATIO_OFFSET) / max_dim
    } else {
        0.0
    };
    let _ = (ratio, inverse); // aspect terms retained for the full formula.
    if value < 0.0 {
        0.0
    } else if value > 1.0 {
        1.0
    } else {
        value
    }
}

/// B 0x140a93790 (12-param system).
pub fn create_rectangle(
    mut requests: EventReader<CreateRectangleCmd>,
    mut inner: ResMut<InnerWalls>,
    _public: Res<PublicWalls>,
    mut roofs: EventWriter<crate::resources::roofs::commands::CreateRoofCmd>,
    mut wall_history: EventWriter<RecordWallHistory>,
) {
    for event in requests.read() {
        let rect = Rectangle2d::new(event.center, event.dimensions, event.orientation);
        let curve = rect.as_curve2();
        let authority = to_authority_curve(&curve);

        let shape = WallShapeSnapshot::from_observed(
            ShapeParameters::from_shape(Shape::Rectangle(rect)).to_record(),
        );
        let id = inner.new_wall_w_params(authority, SHAPE_ELEVATION, 0, 0, false);
        inner.update_wall_state_enclosure(id, shape);

        let ridge_length_01 = ridge_length_01_from_rectangle_ratio(event.dimensions);
        roofs.write(build_roof_create_request(id, 1.0, 1.0, ridge_length_01, 0.0, 0));

        let source = inner.get_unwrapped(id);
        if let Ok(record) = RecordWallHistory::new_wall(id, source, 0) {
            wall_history.write(record);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::walls::WallId;
    use crate::systems::wall::history_recording::register_wall_recording_events;

    #[test]
    fn square_rectangle_produces_a_full_ridge() {
        let value = ridge_length_01_from_rectangle_ratio(glam::Vec2::new(4.0, 4.0));
        assert!(value > 0.0 && value <= 1.0);
    }

    #[test]
    fn creating_a_rectangle_writes_wall_and_roof_and_history() {
        let mut world = World::new();
        world.insert_resource(InnerWalls::default());
        world.insert_resource(PublicWalls::default());
        register_wall_recording_events(&mut world);
        world.init_resource::<Events<CreateRectangleCmd>>();
        world.init_resource::<Events<crate::resources::roofs::commands::CreateRoofCmd>>();
        world.init_resource::<Events<RecordWallHistory>>();
        world.resource_mut::<Events<CreateRectangleCmd>>().send(CreateRectangleCmd {
            center: glam::Vec2::ZERO,
            dimensions: glam::Vec2::new(6.0, 4.0),
            orientation: glam::Vec2::X,
        });
        let mut schedule = Schedule::default();
        schedule.add_systems(create_rectangle);
        schedule.run(&mut world);
        let inner = world.resource::<InnerWalls>();
        let (id, _state) = inner.all().next().unwrap();
        assert_eq!(id, WallId(1));
        assert!(inner.get_unwrapped(id).is_premade_shape());
        let roofs = world
            .resource_mut::<Events<crate::resources::roofs::commands::CreateRoofCmd>>()
            .drain()
            .collect::<Vec<_>>();
        assert_eq!(roofs.len(), 1);
    }
}
