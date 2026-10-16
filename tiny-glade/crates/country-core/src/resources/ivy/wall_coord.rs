//! B 0x140b199e0: measured wall distance and Y → world X/Y/Z.
use crate::resources::walls::public_wall_state::PublicWallState;
use glam::Vec3;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WallCoord {
    pub u: f32,
    pub v: f32,
}
impl WallCoord {
    pub fn from_wall_u(u: f32) -> Self {
        Self { u, v: 0.0 }
    }
    pub fn into_world_space(self, wall: &PublicWallState, side: bool) -> Vec3 {
        let curve = wall.simplified_curve_2d();
        assert!(
            !(curve.length < self.u),
            "wall coordinate exceeds curve length"
        );
        assert!(curve.length != 0.0, "wall curve has zero length");
        let normalized = self.u / curve.length;
        assert!(
            normalized >= 0.0 && normalized <= 1.0,
            "invalid wall coordinate"
        );
        let (index, t) = curve.coord_at_u(normalized);
        let a = curve.points[index];
        let b = curve.points[index + 1];
        let x = t * b[0] + (1.0 - t) * a[0];
        let z = t * b[1] + (1.0 - t) * a[1];
        let dx = b[0] - a[0];
        let dz = b[1] - a[1];
        let reciprocal = 1.0 / (dz * dz + dx * dx).sqrt();
        let offset_x = (-dz * reciprocal) * 0.63 * 0.5;
        let offset_z = (dx * reciprocal) * 0.63 * 0.5;
        // RNE 0x140b19acd/b72..ba7: the normal's second component offsets both
        // the Y and Z outputs — out.y = v ± offset_z, out.z = z ± offset_z.
        // The earlier draft added the offset to Z only.
        if side {
            Vec3::new(x - offset_x, self.v - offset_z, z - offset_z)
        } else {
            Vec3::new(x + offset_x, self.v + offset_z, z + offset_z)
        }
    }
}
