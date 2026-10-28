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
use crate::systems::audio::{PlayBuildingAudio, RequestWallMaterialAudio};

/// B 0x14097f620 (5-param system).
pub fn edit_rectangle(
    mut requests: EventReader<EditRectangleCmd>,
    mut inner: ResMut<InnerWalls>,
    mut material: EventWriter<RequestWallMaterialAudio>,
    mut building: EventWriter<PlayBuildingAudio>,
    time: Res<bevy_time::Time>,
) {
    if let Some(event) = requests.read().last() {
        let rect = Rectangle2d::new(event.center, event.dimensions, event.orientation);
        let curve = rect.as_curve2();
        inner.update_wall_state_curve(event.wall, to_authority_curve(&curve));
        let snapshot = WallShapeSnapshot::from_observed(
            ShapeParameters::from_shape(Shape::Rectangle(rect)).to_record(),
        );
        inner.update_wall_state_enclosure(event.wall, snapshot);
        material.write(RequestWallMaterialAudio { id: event.wall.0, position: event.position, kind: 4, flag: 0 });
        let divisor = if event.rotating { 20.0 } else { 200.0 };
        let value = (event.movement / time.delta_secs()) / divisor;
        let value = if 0.0 <= value { value } else { 0.0 };
        let value = if value <= 1.0 { value } else { 1.0 };
        building.write(PlayBuildingAudio { id: event.wall.0, position: event.position, editing_speed: value, kind: u8::from(event.rotating) });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::{Vec2, Vec3};
    use crate::resources::walls::WallState;
    use crate::resources::roofs::geometry::surface::Curve2;

    #[test]
    fn last_request_updates_authority_and_writes_matching_audio() {
        let mut world = World::new();
        let mut inner = InnerWalls::new();
        let wall = inner.new_wall_from_curve(WallState::new(Curve2::try_new(vec![[0.0, 0.0], [1.0, 0.0]], false).unwrap(), 2.0, 0, 0, false));
        world.insert_resource(inner);
        world.init_resource::<Events<EditRectangleCmd>>();
        world.init_resource::<Events<RequestWallMaterialAudio>>();
        world.init_resource::<Events<PlayBuildingAudio>>();
        let mut time = bevy_time::Time::<()>::default();
        time.advance_by(std::time::Duration::from_millis(500));
        world.insert_resource(time);
        for (center, rotating, movement) in [(Vec2::ZERO, false, 10.0), (Vec2::new(3.0, 4.0), true, 5.0)] {
            world.send_event(EditRectangleCmd { wall, center, dimensions: Vec2::new(4.0, 6.0), orientation: Vec2::X, position: Vec3::new(3.0, 2.0, 4.0), movement, rotating });
        }
        let mut schedule = Schedule::default();
        schedule.add_systems(edit_rectangle);
        schedule.run(&mut world);
        let shape = world.resource::<InnerWalls>().get(wall).unwrap().get_enclosure_params().unwrap();
        assert_eq!(ShapeParameters::from_snapshot(shape).as_rectangle().center, Vec2::new(3.0, 4.0));
        let audio: Vec<_> = world.resource_mut::<Events<PlayBuildingAudio>>().drain().collect();
        assert_eq!(audio.len(), 1);
        assert_eq!((audio[0].kind, audio[0].editing_speed, audio[0].position), (1, 0.5, Vec3::new(3.0, 2.0, 4.0)));
        let material: Vec<_> = world.resource_mut::<Events<RequestWallMaterialAudio>>().drain().collect();
        assert_eq!(material.len(), 1);
        assert_eq!((material[0].id, material[0].kind, material[0].flag), (wall.0, 4, 0));
        schedule.run(&mut world);
        assert!(world.resource::<Events<PlayBuildingAudio>>().is_empty());
        for (dt, movement, expected) in [(0.5, 50.0, 0.5), (0.0, 1.0, 1.0), (0.0, 0.0, 0.0), (0.5, f32::NAN, 0.0)] {
            world.resource_mut::<bevy_time::Time<()>>().advance_by(std::time::Duration::from_secs_f32(dt));
            world.send_event(EditRectangleCmd { wall, center: Vec2::ZERO, dimensions: Vec2::splat(2.0), orientation: Vec2::X, position: Vec3::ZERO, movement, rotating: false });
            schedule.run(&mut world);
            let audio: Vec<_> = world.resource_mut::<Events<PlayBuildingAudio>>().drain().collect();
            assert_eq!(audio.len(), 1);
            assert_eq!((audio[0].kind, audio[0].editing_speed), (0, expected));
        }
    }
}
