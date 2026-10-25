//! B triangle, normal and UV ownership; generic Vec storage is reused.
use bevy_ecs::component::Component;
use glam::{Vec2, Vec3};

#[derive(Component, Clone, Debug, Default)]
pub struct RoofTriggerVolume {
    triangles: Vec<[Vec3; 3]>,
    normals: Vec<[Vec3; 3]>,
    uvs: Vec<[Vec2; 3]>,
}

impl RoofTriggerVolume {
    /// Original getter asserts the geometry producer has populated triangles.
    pub fn triangles(&self) -> &[[Vec3; 3]] {
        assert!(
            !self.triangles.is_empty(),
            "assertion failed: !self.triangles.is_empty()"
        );
        &self.triangles
    }
    pub fn normals(&self) -> &[[Vec3; 3]] {
        &self.normals
    }
    pub fn uvs(&self) -> &[[Vec2; 3]] {
        &self.uvs
    }
    pub fn set_triangles(&mut self, values: Vec<[Vec3; 3]>) {
        self.triangles = values;
    }
    pub fn set_normals(&mut self, values: Vec<[Vec3; 3]>) {
        self.normals = values;
        assert_eq!(
            self.triangles.len(),
            self.normals.len(),
            "original normal count guard"
        );
    }
    pub fn set_uvs(&mut self, values: Vec<[Vec2; 3]>) {
        self.uvs = values;
        assert_eq!(
            self.triangles.len(),
            self.uvs.len(),
            "original UV count guard"
        );
    }
    /// B 0x1408e46d0 delegates triangle UV lookup to the recovered game utility
    /// then adds the supplied floor offset, retaining the zero products on X/Z.
    pub fn get_position_from_mesh_uv(&self, uv: Vec2, floor_y: f32) -> Option<Vec3> {
        utils::geometry::position_from_mesh_uvs::get_position_from_mesh_uv(
            &self.triangles,
            &self.uvs,
            uv,
        )
        .map(|point| {
            Vec3::new(
                floor_y * 0.0 + point.x,
                floor_y + point.y,
                point.z + floor_y * 0.0,
            )
        })
    }
}
