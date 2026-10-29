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
        editing_speed: editing_audio_speed(event.speed, time.delta_secs(), event.rotation),
        kind: u8::from(event.rotation),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
