//! B `systems/wall/shapes/circle/create_circle.rs`: circle wall creation.
//!
//! Evidence: `chapters/systems/wall/shapes/circle/create_circle.rs.md` (O,
//! 0x14097e560). Circle2d::as_curve2(0.2) builds the wall curve; the wall is
//! created with the circle enclosure; a CreateRoofCmd is emitted with the
//! computed height and a wall-creation history record. The exact
//! `roof_height_01_from_wall_shape` interpolation constants are 待取证
//! (approximation below).

use bevy_ecs::prelude::*;
use ::utils::geometry::Circle2d;

use super::super::{
    CreateCircleCmd, build_roof_create_request, to_authority_curve,
};
use crate::recovered_building::wall_state::WallShapeSnapshot;
use crate::resources::walls::{InnerWalls, public_walls::PublicWalls};
use crate::systems::wall::{
    history_recording::RecordWallHistory,
    valid_enclosure::{Shape, ShapeParameters},
};

/// Default wall elevation for shape creation (4.0, 0x40800000).
const SHAPE_ELEVATION: f32 = 4.0;
/// Circle sample step (0x3e4ccccd @0x1429cd068).
const CIRCLE_SAMPLE_STEP: f32 = 0.2;

/// B 0x14097e560 (15-param system).
pub fn create_circle(
    mut requests: EventReader<CreateCircleCmd>,
    mut inner: ResMut<InnerWalls>,
    _public: Res<PublicWalls>,
    mut roofs: EventWriter<crate::resources::roofs::commands::CreateRoofCmd>,
    mut wall_history: EventWriter<RecordWallHistory>,
) {
    for event in requests.read() {
        let circle = Circle2d::new(event.center, event.radius);
        let curve = circle.as_curve2(CIRCLE_SAMPLE_STEP);
        let authority = to_authority_curve(&curve);

        let shape = WallShapeSnapshot::from_observed(
            ShapeParameters::from_shape(Shape::Circle(circle)).to_record(),
        );
        let id = inner.new_wall_w_params(authority, SHAPE_ELEVATION, 0, 0, false);
        inner.update_wall_state_enclosure(id, shape);

        // Roof height: circle walls use a fixed height_01 of 1.0 (exact
        // interpolation constants 待取证).
        roofs.write(build_roof_create_request(id, 1.0, 1.0, 0.0, 0.0, 0));

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
    fn creating_a_circle_writes_wall_and_roof_and_history() {
        let mut world = World::new();
        world.insert_resource(InnerWalls::default());
        world.insert_resource(PublicWalls::default());
        register_wall_recording_events(&mut world);
        world.init_resource::<Events<CreateCircleCmd>>();
        world.init_resource::<Events<crate::resources::roofs::commands::CreateRoofCmd>>();
        world.init_resource::<Events<RecordWallHistory>>();
        world.resource_mut::<Events<CreateCircleCmd>>().send(CreateCircleCmd {
            center: glam::Vec2::ZERO,
            radius: 3.0,
        });
        let mut schedule = Schedule::default();
        schedule.add_systems(create_circle);
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
        assert_eq!(
            roofs[0].0.wall_id(),
            crate::recovered_building::wall_state::WallId(1)
        );
        let history = world
            .resource_mut::<Events<RecordWallHistory>>()
            .drain()
            .count();
        assert_eq!(history, 1);
    }
}
