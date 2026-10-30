//! B 14097f620：应用最后一个矩形编辑请求并更新对应建筑音频。

use ::utils::geometry::Rectangle2d;
use bevy_ecs::prelude::*;

use super::super::{EditRectangleCmd, to_authority_curve};
use crate::resources::walls::InnerWalls;
use crate::resources::walls::publication_fields::WallShapeSnapshot;
use crate::systems::audio::{PlayBuildingAudio, RequestWallMaterialAudio};
use crate::systems::wall::valid_enclosure::{Shape, ShapeParameters};

/// 原inverse_lerp后maxss/minss钳制，两个指令的源操作数都保留NaN。
fn editing_audio_speed(speed: f32, delta: f32, rotating: bool) -> f32 {
    // B常量142a13848=200.0，142a7fa30=20.0。
    let upper = if rotating { 20.0 } else { 200.0 };
    let t = ::utils::scalar::inverse_lerp(0.0, upper, speed / delta);
    t.clamp(0.0, 1.0)
}

/// 原五参数。一帧多个请求只采用最后一个，旧消息仍全部被reader消费。
pub fn edit_rectangle(
    mut requests: EventReader<EditRectangleCmd>,
    mut inner: ResMut<InnerWalls>,
    mut material: EventWriter<RequestWallMaterialAudio>,
    mut audio: EventWriter<PlayBuildingAudio>,
    time: Res<bevy_time::Time>,
) {
    let Some(event) = requests.read().last() else {
        return;
    };
    let rect = Rectangle2d::new(event.center, event.dimensions, event.orientation);
    inner.update_wall_state_curve(event.wall, to_authority_curve(&rect.as_curve2()));
    inner.update_wall_state_enclosure(
        event.wall,
        WallShapeSnapshot::from_observed(
            ShapeParameters::from_shape(Shape::Rectangle(rect)).to_record(),
        ),
    );
    material.write(RequestWallMaterialAudio {
        id: event.wall.0,
        position: event.position,
        kind: 4,
        flag: 0,
    });
    audio.write(PlayBuildingAudio {
        id: event.wall.0,
        position: event.position,
        editing_speed: editing_audio_speed(event.movement, time.delta_secs(), event.rotating),
        kind: u8::from(event.rotating),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::{Vec2, Vec3};
    use crate::resources::walls::WallState;
    use crate::resources::roofs::geometry::surface::Curve2;
    #[test]
    fn original_audio_scales_and_unordered_clamp() {
        assert_eq!(
            editing_audio_speed(25.0, 0.25, false).to_bits(),
            0.5f32.to_bits()
        );
        assert_eq!(
            editing_audio_speed(2.5, 0.25, true).to_bits(),
            0.5f32.to_bits()
        );
        assert!(editing_audio_speed(f32::NAN, 1.0, false).is_nan());
        assert!(editing_audio_speed(0.0, 0.0, true).is_nan());
        assert_eq!(
            editing_audio_speed(f32::INFINITY, 1.0, false).to_bits(),
            1.0f32.to_bits()
        );
    }
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
        for (dt, movement, expected) in [(0.5, 50.0, 0.5), (0.0, 1.0, 1.0), (0.0, 0.0, f32::NAN), (0.5, f32::NAN, f32::NAN)] {
            world.resource_mut::<bevy_time::Time<()>>().advance_by(std::time::Duration::from_secs_f32(dt));
            world.send_event(EditRectangleCmd { wall, center: Vec2::ZERO, dimensions: Vec2::splat(2.0), orientation: Vec2::X, position: Vec3::ZERO, movement, rotating: false });
            schedule.run(&mut world);
            let audio: Vec<_> = world.resource_mut::<Events<PlayBuildingAudio>>().drain().collect();
            assert_eq!(audio.len(), 1);
            assert_eq!(audio[0].kind, 0);
            if expected.is_nan() { assert!(audio[0].editing_speed.is_nan()); } else { assert_eq!(audio[0].editing_speed, expected); }
        }
    }
}
