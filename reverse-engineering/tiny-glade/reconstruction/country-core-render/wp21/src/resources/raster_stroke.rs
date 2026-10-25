//! raster_stroke.rs — 栅格笔刷（WP21）。
//!
//! O：`RasterStroke`：`{ points: Arc<[Vec3]>, radius: f32 }`；`add_point`、`smooth`
//! （11 内联）、`radius`、`length`、`iter_points_xz`（0xa97860，50 内联）、
//! `impl$4::from`（0x9f1c90 fmt=Debug）。地形/栅格笔刷类。

use std::sync::Arc;

use bevy_ecs::resource::Resource;
use glam::Vec3;

use crate::typed_gap::Gap;

/// `RasterStroke`（O：栅格笔刷点集）。
#[derive(Resource, Clone, Debug)]
pub struct RasterStroke {
    /// 点集（O：Arc<[Vec3]>）。
    pub points: Arc<[Vec3]>,
    /// 半径（O：`radius` getter，22 内联）。
    pub radius: f32,
    /// 未映射区（U）。
    pub _gap: Gap,
}

impl Default for RasterStroke {
    fn default() -> Self {
        Self { points: Arc::from([]), radius: 0.0, _gap: Gap::default() }
    }
}

impl RasterStroke {
    /// `add_point`（O 名，1 内联）。
    pub fn add_point(&mut self, p: Vec3) {
        let mut v = self.points.to_vec();
        v.push(p);
        self.points = Arc::from(v);
    }
    /// `smooth`（O 名，11 内联）。
    pub fn smooth(&mut self) {
        let pts = self.points.to_vec();
        self.points = Arc::from(pts);
    }
    /// `radius`（O：getter）。
    pub fn radius(&self) -> f32 {
        self.radius
    }
    /// `length`（O 名，2 内联）。
    pub fn length(&self) -> f32 {
        self.points.windows(2).map(|w| (w[1] - w[0]).length()).sum()
    }
    /// `iter_points_xz`（O 0xa97860）。
    pub fn iter_points_xz(&self) -> impl Iterator<Item = (f32, f32)> + '_ {
        self.points.iter().map(|p| (p.x, p.z))
    }
}
