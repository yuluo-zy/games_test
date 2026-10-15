//! Owned raster-history payload, B record_raster_history 0x140957990.
//! The outer history edit holds Arc<RasterStroke>; points are Arc<[Vec3]>.
use glam::{Vec2, Vec3};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct RasterStroke {
    pub flags: u32,
    pub strength: f32,
    pub points: Arc<[Vec3]>,
    pub radius: f32,
    pub draw: bool,
}

impl RasterStroke {
    pub fn radius(&self) -> f32 {
        self.radius
    }
    pub fn iter_points_xz(&self) -> impl ExactSizeIterator<Item = Vec2> + '_ {
        self.points.iter().map(|point| Vec2::new(point.x, point.z))
    }
}
