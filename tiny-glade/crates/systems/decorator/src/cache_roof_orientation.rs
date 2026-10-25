//! B140815cf2..140815e0f: non-stair roof normal policy. Shared Roof section
//! geometry and Rectangle2d closest-edge algorithms remain caller-owned.
use glam::{Quat, Vec2, Vec3};

pub(super) fn roof_uses_yaw(tag: u8) -> bool {
    (!tag & 0x3e) == 0 || (!tag & 0x38) != 0
}

pub(super) fn compose_roof_orientation(base: Quat, normal: Quat) -> Quat {
    base * normal
}

pub(super) fn roof_decorator_normal(
    rank: u8,
    position: Vec3,
    rectangular: bool,
    center: Vec2,
    mut rectangle_normal: impl FnMut(Vec3, Vec2) -> Vec2,
) -> Vec2 {
    let xz = Vec2::new(position.x, position.z);
    if rectangular {
        assert!((1..=3).contains(&rank), "invalid roof decorator rank");
        rectangle_normal(position + Vec3::Y, xz)
    } else if (center - xz).length() < f32::from_bits(0x3c23d70a) {
        Vec2::X
    } else {
        (xz - center).normalize()
    }
}
