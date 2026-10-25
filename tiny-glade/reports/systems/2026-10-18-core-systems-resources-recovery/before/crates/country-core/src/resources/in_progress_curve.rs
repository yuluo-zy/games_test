//! B `resources/in_progress_curve.rs`: the freehand raw-input curve resource.
//!
//! The snapping and eraser systems read the active wall, the accumulated raw
//! 2D input points and a flag byte. The B layout is
//! `{Option<WallId>@+0 (i64::MIN sentinel), raw points std Vec {ptr@+8,len@+0x10},
//! current wall@+0x80, flag@+0x9e}`; this typed representation keeps the
//! semantic fields and does not claim native ABI. `compute_final_curve`
//! (0x140ad41a0, 16 KB) performs resampling/fitting whose full algorithm is
//! not detailed in the chapter — registered 待取证 (approximation below).

use bevy_ecs::resource::Resource;
use glam::Vec2;
use crate::resources::roofs::geometry::surface::Curve2;

use crate::resources::walls::WallId;

/// B 0x140ad3f50's raw-input validation dispatch (FUN_141466860 returns 3 for
/// valid input). Exact shared-utils rule is 待取证; here we require at least
/// two finite points (the wall consumers all assert count >= 2 and length > 0).
fn validate_raw_input_points(points: &[Vec2]) -> bool {
    points.len() >= 2 && points.iter().all(|p| p.x.is_finite() && p.y.is_finite())
}

#[derive(Resource, Debug, Default)]
pub struct InProgressCurve {
    /// None while no freehand wall is being drawn (B +0x0, i64::MIN sentinel).
    pub wall: Option<WallId>,
    /// Accumulated raw 2D input points (B Vec at +0x8).
    pub raw_input_points: Vec<Vec2>,
    /// The wall currently being extended (B +0x80). Zero when absent.
    pub current_wall: WallId,
    /// Flag byte (B +0x9e): bit 0 forces `snapped` in end snapping.
    pub flags: u8,
    /// Cache of the last final-curve computation.
    pub final_curve: Option<Curve2>,
}

impl InProgressCurve {
    /// B 0x140ad3070: empty curve constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// B 0x140ad3f50: validate then append raw input points; invalid input
    /// clears the accumulation.
    pub fn try_add_raw_input_points(&mut self, points: &[Vec2]) -> bool {
        if !validate_raw_input_points(points) {
            self.raw_input_points.clear();
            self.final_curve = None;
            return false;
        }
        self.raw_input_points.extend_from_slice(points);
        self.final_curve = None;
        true
    }

    pub fn clear(&mut self) {
        self.raw_input_points.clear();
        self.wall = None;
        self.current_wall = WallId(0);
        self.flags = 0;
        self.final_curve = None;
    }

    /// Inline B line 130: first raw input point.
    pub fn first_raw_input_point(&self) -> Vec2 {
        *self
            .raw_input_points
            .first()
            .expect("InProgressCurve has no raw input points")
    }

    /// Inline B line 126: last raw input point.
    pub fn last_raw_input_point(&self) -> Vec2 {
        *self
            .raw_input_points
            .last()
            .expect("InProgressCurve has no raw input points")
    }

    /// Inline B line 141: summed world length of the raw input segments.
    pub fn raw_input_curve_length_ws(&self) -> f32 {
        let mut length = 0.0;
        for pair in self.raw_input_points.windows(2) {
            length += (pair[1] - pair[0]).length();
        }
        length
    }

    /// Inline B line 220: raw input validity predicate (count >= 2 and length > 0).
    pub fn is_raw_input_curve_valid(&self) -> bool {
        let length = self.raw_input_curve_length_ws();
        self.raw_input_points.len() >= 2 && length > 0.0
    }

    /// B 0x140ad41a0 (16 KB resample/fit). The original fitting/resampling
    /// algorithm is not detailed in the chapter; this is a documented
    /// approximation that builds the Curve2 directly from the raw points.
    /// (待取证: exact resampling/fitting.)
    pub fn compute_final_curve(&mut self) -> Option<Curve2> {
        if !self.is_raw_input_curve_valid() {
            self.final_curve = None;
            return None;
        }
        let points = self
            .raw_input_points
            .iter()
            .map(|p| [p.x, p.y])
            .collect::<Vec<_>>();
        let curve = Curve2::try_new(points, false).ok();
        self.final_curve = curve.clone();
        curve
    }
}
