//! B 0x141244310 木地板边条：先解析墙关联屋顶、样式和已完成孔洞。
use bevy_ecs::prelude::*;
use country_core::resources::{roofs::roof_shape::Roof, walls::{WallId,
    public_walls::{PublicWalls, PublicWallState}, wall_holes::WallHoles}};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;

/// 按原孔洞曲线顺序构造圆形与矩形边条。
pub fn wooden_floor_borders(
    walls: &PublicWalls, id: WallId, wall: &PublicWallState, holes: &WallHoles,
    _signifiers: &UiSignifierStream, roofs: &Query<&Roof>, _commands: &mut Commands,
    _output: &mut Vec<Entity>,
) {
    let Some(roof) = walls.roof_entity(id).and_then(|entity| roofs.get(entity).ok()) else { return };
    let bytes = roof.observed();
    let enabled = match bytes[0x40] {
        0 => wall.authority.style() >= 2,
        1 => false,
        _ => bytes[0x41] & 1 != 0,
    };
    if !enabled { return; }
    let Some(storage) = holes.storage.get(&id) else { return };
    let finalized = storage.finalized_holes().expect("原木边条要求已完成的墙洞");
    for hole in finalized.iter().filter(|hole| hole.kind == 1 || hole.kind == 4) {
        // 原 get_wall_shape 对自由形状保持原先的失败边界。
        wall.get_wall_shape();
        if let Some(circle) = wall.is_circle() {
            let scale = std::f32::consts::TAU / wall.wall_space_length();
            let rotation = circle.rotation % std::f32::consts::TAU;
            let rotation = if rotation < 0.0 { rotation + std::f32::consts::TAU } else { rotation };
            let middle = ((hole.aabb[0] + hole.aabb[2]) * 0.5) * scale - rotation;
            let half = ((hole.aabb[2] - hole.aabb[0]) * 0.5) * scale;
            let data = country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::new_curved_xz(
                glam::Vec3::new(circle.center.x, hole.aabb[1], circle.center.y),
                circle.radius, 0.54, 0.095, middle - half, middle + half, 0,
            );
            super::spawn_instances(id, 0, &[data], _commands, _output);
        } else {
            let rectangle = wall.is_rectangle().expect("原墙形必须是圆或矩形");
            let curve = border_curve(rectangle, hole.aabb[0], hole.aabb[2]);
            let Ok(curve) = curve else { continue };
            let vertices = rectangle.as_curve2().points;
            for pair in curve.points.windows(2) {
                let data = straight_border(pair[0], pair[1], hole.aabb[1], &vertices);
                super::spawn_instance(id, 0, data, 4, _commands, _output);
            }
        }
    }
}

/// 原孔区间按周长环绕切片，足够长时使用完整矩形。
fn border_curve(rectangle: utils::geometry::Rectangle2d, left: f32, right: f32)
    -> Result<utils::curve::Curve2, utils::curve::CurveError> {
    let perimeter = rectangle.perimeter();
    if !(right - left + 0.05 > perimeter) {
        let wrap = |value: f32| {
            assert!(!value.is_nan(), "原曲线坐标不得为 NaN");
            let remainder = value % 1.0;
            if remainder < 0.0 { remainder + 1.0 } else { remainder }
        };
        rectangle.as_curve2().circular_slice(wrap(left / perimeter), wrap(right / perimeter))
    } else { rectangle.try_as_curve2() }
}

/// 原 141244ad5–c7e：从切片段端点确定方向、两端角帽和完整实例。
fn straight_border(a: glam::Vec2, b: glam::Vec2, height: f32, vertices: &[glam::Vec2])
    -> country_core::render::nani::instance_data::CurvedHalfTimberInstanceData {
    let delta = b - a;
    let dy = height - height;
    let length = ((delta.x * delta.x + dy * dy) + delta.y * delta.y).sqrt();
    let inverse = 1.0 / length;
    let start_cap = vertices.iter().any(|vertex| vertex.distance(a) < 0.05);
    let end_cap = vertices.iter().any(|vertex| vertex.distance(b) < 0.05);
    country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::new_straight(
        glam::Vec3::new((a.x + b.x) * 0.5, (height + height) * 0.5, (a.y + b.y) * 0.5),
        (-inverse) * delta.y, delta.x * inverse, 0.54, length, start_cap, u8::from(end_cap),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle_segment_caps_match_original_64_byte_records() {
        let vertices = [glam::Vec2::new(-2.,-1.), glam::Vec2::new(-2.,1.),
            glam::Vec2::new(2.,1.), glam::Vec2::new(2.,-1.)];
        let segments = [(vertices[0],vertices[1]), (vertices[1],vertices[2]),
            (glam::Vec2::new(1.,-1.),vertices[0])];
        let expected = [
            [0xc0000000,0,0,0,0xbf800000,0,0x3f0a3d71,0x40000000,4,1,0x3dc28f5c,0,1,0,0,0],
            [0,0,0x3f800000,0,0x80000000,0x3f800000,0x3f0a3d71,0x40800000,4,1,0x3dc28f5c,0,1,0,0,0],
            [0xbf000000,0,0xbf800000,0,0x80000000,0xbf800000,0x3f0a3d71,0x40400000,3,1,0x3dc28f5c,0,1,0,0,0],
        ];
        for (i, (a,b)) in segments.into_iter().enumerate() {
            let v = straight_border(a,b,0.,&vertices);
            let words = [v.position.x.to_bits(),v.position.y.to_bits(),v.position.z.to_bits(),v.pad_0c,
                v.x0.to_bits(),v.x1.to_bits(),v.y0.to_bits(),v.y1.to_bits(),v.w20,v.w24,
                v.w28.to_bits(),v.w2c,v.w30,v.w34,v.w38,v.w3c];
            assert_eq!(words,expected[i],"原矩形木边条端帽 {i}");
        }
    }

    #[test]
    fn rectangle_cross_seam_slice_matches_original_records() {
        let rectangle = utils::geometry::Rectangle2d { center: glam::Vec2::ZERO,
            axis: glam::Vec2::X, size: glam::Vec2::new(4.,2.) };
        let vertices = rectangle.as_curve2().points;
        let curve = border_curve(rectangle, -1., 1.).unwrap();
        let expected = [
            [0xbfc00001,0,0xbf800000,0,0x80000000,0xbf800000,0x3f0a3d71,0x3f7ffffc,3,1,0x3dc28f5c,0,1,0,0,0],
            [0xc0000000,0,0xbf000000,0,0xbf800000,0,0x3f0a3d71,0x3f800000,2,1,0x3dc28f5c,0,1,0,0,0],
        ];
        assert_eq!(curve.points.len(), 3);
        for (i,pair) in curve.points.windows(2).enumerate() {
            let v = straight_border(pair[0],pair[1],0.,&vertices);
            let words = [v.position.x.to_bits(),v.position.y.to_bits(),v.position.z.to_bits(),v.pad_0c,
                v.x0.to_bits(),v.x1.to_bits(),v.y0.to_bits(),v.y1.to_bits(),v.w20,v.w24,
                v.w28.to_bits(),v.w2c,v.w30,v.w34,v.w38,v.w3c];
            assert_eq!(words,expected[i],"原跨缝矩形木边条 {i}");
        }
    }
}
