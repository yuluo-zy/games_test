//! B 0x1408eae40/0x1408e9ed0: wall quads and cached default collision triangles.
//! Quad is 120 meaningful bytes, not the old report's guessed 96 bytes. The
//! corner order is [bottom/start, bottom/end, top/start, top/end]. Freehand
//! second faces reverse bottom/top so the two faces have opposite winding.
use super::public_wall_state::PublicWallState;
use crate::systems::wall::wall_state::pending_changed::approximate_curve_u;
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};
use utils::{curve::Curve3, geometry::intersect::Triangle};

#[derive(Clone, Debug)]
pub struct Quad {
    pub wall_uv: [Vec2; 4],
    pub positions: [Vec3; 4],
    pub prepared_u: [f32; 2],
    pub endpoint_normals: [Vec3; 2],
    pub segment_heights: [f32; 2],
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WallTriggerError {
    MissingPreparedPath,
    MissingHeightCache,
    InvalidCurve,
    ArrayCount,
    Projection,
}

#[derive(Component, Clone, Debug, Default)]
pub struct WallTriggerVolume {
    pub quads: Vec<Quad>,
    triangles: Vec<Triangle>,
    normals: Vec<[Vec3; 3]>,
    uvs: Vec<[Vec2; 3]>,
    special: u8,
}
fn clip_u_sse(low: f32, high: f32, u: f32) -> f32 {
    let v = if low > u { low } else { u };
    if high < v { high } else { v }
}
impl WallTriggerVolume {
    pub fn is_valid(&self) -> bool {
        !self.quads.is_empty()
    }
    pub fn default_tris(&self) -> &[Triangle] {
        &self.triangles
    }

    /// B `get_tris` with the circle-edit parameters: no u clipping,
    /// thickness 0.2875, minimum bottom 0, top offset -0.5, adjustment enabled.
    pub fn edit_shape_tris(&self) -> Vec<Triangle> {
        self.edit_shape_tris_in_range(None)
    }

    /// Same native edit-shape parameters with an optional CurveU range.
    pub fn edit_shape_tris_in_range(&self, range: Option<[f32; 2]>) -> Vec<Triangle> {
        assert!(!self.quads.is_empty(), "wall trigger volume has no quads");
        let mut result = Vec::with_capacity(self.quads.len() * 2);
        for q in &self.quads {
            let [u0, u1] = q.prepared_u;
            let [v0, v1] = if let Some([low, high]) = range {
                assert!(low <= high, "invalid wall trigger range");
                let clip = |u: f32| {
                    let v = clip_u_sse(low, high, u);
                    assert!((0.0..=1.0).contains(&v), "invalid CurveU");
                    v
                };
                [clip(u0), clip(u1)]
            } else { [u0, u1] };
            if v0 == v1 {
                continue;
            }
            let mut p = q.positions;
            for i in 0..2 {
                let h = q.segment_heights[i];
                let bottom = if h.is_nan() || h < 0.0 { 0.0 } else { h };
                let offset = q.endpoint_normals[i] * 0.2875;
                p[i] = (p[i] - Vec3::Y * h) + Vec3::Y * bottom + offset;
                let top = if self.special & 1 != 0 && q.segment_heights[0] > 0.0 {
                    q.segment_heights[0] - q.positions[i + 2].y
                } else {
                    -0.5
                };
                p[i + 2] = Vec3::Y * top + p[i + 2] + offset;
            }
            if v0 != u0 || v1 != u1 {
                let t0 = utils::scalar::inverse_lerp(u0, u1, v0);
                let t1 = utils::scalar::inverse_lerp(u0, u1, v1);
                let old = p;
                p[0] = old[1] * t0 + old[0] * (1.0 - t0);
                p[1] = old[1] * t1 + old[0] * (1.0 - t1);
                p[2] = old[3] * t0 + old[2] * (1.0 - t0);
                p[3] = old[3] * t1 + old[2] * (1.0 - t1);
            }
            result.push(Triangle::new(p[0], p[1], p[2]));
            result.push(Triangle::new(p[1], p[3], p[2]));
        }
        assert!(!result.is_empty(), "wall trigger volume has no triangles");
        result
    }
    pub fn triangle_normals(&self) -> &[[Vec3; 3]] {
        &self.normals
    }
    pub fn triangle_uvs(&self) -> &[[Vec2; 3]] {
        &self.uvs
    }

    pub fn recompute(&mut self, wall: &PublicWallState) -> Result<(), WallTriggerError> {
        let prepared = wall
            .prepared_path
            .as_ref()
            .ok_or(WallTriggerError::MissingPreparedPath)?;
        let heights = wall
            .arch_segment_heights()
            .ok_or(WallTriggerError::MissingHeightCache)?;
        let count = prepared
            .positions
            .len()
            .min(prepared.points_u.len())
            .min(heights.len())
            .min(wall.segment_heights.len());
        if count < 2 {
            return Err(WallTriggerError::ArrayCount);
        }
        let mut curve = Curve3::try_new(
            prepared
                .positions
                .iter()
                .copied()
                .map(Vec3::from_array)
                .collect(),
            false,
        )
        .map_err(|_| WallTriggerError::InvalidCurve)?;
        curve.points_u.clone_from(&prepared.points_u);
        curve.length = prepared.length;
        let tag = wall.shape().tag();
        let endpoint = |i: usize| -> Result<Endpoint, WallTriggerError> {
            let u = prepared.points_u[i];
            let c = curve.get_coord_at_u(u);
            let d = curve.points[c.segment + 1] - curve.points[c.segment];
            // 1408e7800..784a normalizes full XYZ, then projects/rotates XZ.
            // It does not renormalize after discarding the slope's Y component.
            let inverse = 1.0 / ((d.y * d.y + d.x * d.x) + d.z * d.z).sqrt();
            let normal = Vec3::new(-d.z * inverse, 0.0, d.x * inverse);
            let source = curve.points[i];
            let segment_height = wall.segment_heights[i];
            let bottom = Vec3::new(
                source.x + segment_height * 0.0,
                source.y + segment_height,
                source.z + segment_height * 0.0,
            );
            let top = Vec3::new(bottom.x, heights[i], bottom.z);
            let raw = wall.simplified_curve_2d();
            let horizontal = approximate_curve_u(raw, [bottom.x, bottom.z])
                .map_err(|_| WallTriggerError::Projection)?
                * raw.length;
            Ok(Endpoint {
                bottom,
                top,
                normal,
                u,
                horizontal,
                source_y: source.y,
                segment_height,
            })
        };
        let mut endpoints: Vec<_> = (0..count).map(endpoint).collect::<Result<_, _>>()?;
        // The original chains the first item for every shape tag except open 2.
        if tag != 2 {
            endpoints.push(endpoints[0]);
        }
        let mut quads = Vec::new();
        for (i, pair) in endpoints.windows(2).enumerate() {
            let [a, b] = [pair[0], pair[1]];
            let uv = [
                Vec2::new(a.horizontal, a.source_y),
                Vec2::new(b.horizontal, a.source_y),
                Vec2::new(a.horizontal, a.top.y),
                Vec2::new(b.horizontal, b.top.y),
            ];
            let mut push = |positions, normals, segment_heights, wall_uv| {
                quads.push(Quad {
                    positions,
                    endpoint_normals: normals,
                    segment_heights,
                    wall_uv,
                    prepared_u: [a.u, b.u],
                });
            };
            if tag < 2 {
                push(
                    [a.bottom, b.bottom, a.top, b.top],
                    [a.normal, b.normal],
                    [a.segment_height, b.segment_height],
                    uv,
                );
                continue;
            }
            // Two multiplies are retained from the original constant loads.
            let oa = (a.normal * 0.63) * 0.5;
            let ob = (b.normal * 0.63) * 0.5;
            let top_uv = [uv[2], uv[3], uv[2], uv[3]];
            if wall.authority.source_flag_65() == 1
                && (a.segment_height > 0.0 || b.segment_height > 0.0)
            {
                if a.segment_height != 0.0 && b.segment_height != 0.0 {
                    push(
                        [a.bottom + oa, b.bottom + ob, a.bottom - oa, b.bottom - ob],
                        [Vec3::Y; 2],
                        [0.0; 2],
                        top_uv,
                    );
                }
                continue;
            }
            push(
                [a.bottom + oa, b.bottom + ob, a.top + oa, b.top + ob],
                [a.normal, b.normal],
                [a.segment_height, b.segment_height],
                uv,
            );
            push(
                [a.top - oa, b.top - ob, a.bottom - oa, b.bottom - ob],
                [a.normal, b.normal],
                [a.segment_height, b.segment_height],
                uv,
            );
            push(
                [a.top + oa, b.top + ob, a.top - oa, b.top - ob],
                [Vec3::Y; 2],
                [0.0; 2],
                top_uv,
            );
            if tag == 2 {
                if i == 0 {
                    push(
                        [a.bottom + oa, a.top + oa, a.bottom - oa, a.top - oa],
                        [a.normal; 2],
                        [0.0; 2],
                        [uv[0], uv[2], uv[0], uv[2]],
                    );
                }
                if i == prepared.positions.len() - 2 {
                    let end_bottom = Vec2::new(b.horizontal, a.source_y);
                    push(
                        [b.bottom - ob, b.top - ob, b.bottom + ob, b.top + ob],
                        [b.normal; 2],
                        [0.0; 2],
                        [end_bottom, uv[3], end_bottom, uv[3]],
                    );
                }
            }
        }
        self.quads = quads;
        self.recompute_tris(tag < 2);
        self.special = wall.authority.source_flag_65();
        Ok(())
    }

    fn recompute_tris(&mut self, regular: bool) {
        self.triangles.clear();
        self.normals.clear();
        self.uvs.clear();
        for q in &self.quads {
            let mut p = q.positions;
            if regular {
                if q.prepared_u[0] == q.prepared_u[1] {
                    continue;
                }
                for i in 0..2 {
                    let h = q.segment_heights[i];
                    let positive = if h > 0.0 { h } else { 0.0 };
                    let offset = q.endpoint_normals[i] * 0.315;
                    p[i] = (p[i] - Vec3::Y * h) + Vec3::Y * positive + offset;
                    p[i + 2] += offset;
                    // Source recompute uses the volume's previous special byte,
                    // then stores the newly published byte at function exit.
                    if self.special & 1 != 0 && q.segment_heights[0] > 0.0 {
                        p[i + 2] += Vec3::Y * (q.segment_heights[0] - q.positions[i + 2].y);
                    }
                }
            }
            for indices in [[0, 1, 2], [1, 3, 2]] {
                self.triangles
                    .push(Triangle::new(p[indices[0]], p[indices[1]], p[indices[2]]));
                self.normals
                    .push(indices.map(|i| q.endpoint_normals[i & 1]));
                self.uvs.push(indices.map(|i| q.wall_uv[i]));
            }
        }
    }
}

#[cfg(test)]
mod edit_shape_tests {
    use super::*;

    fn volume() -> WallTriggerVolume {
        WallTriggerVolume {
            quads: vec![Quad {
                wall_uv: [Vec2::ZERO; 4],
                positions: [Vec3::ZERO, Vec3::X, Vec3::Y * 3.0, Vec3::X + Vec3::Y * 4.0],
                prepared_u: [0.0, 1.0],
                endpoint_normals: [Vec3::Z; 2],
                segment_heights: [-1.0, 2.0],
            }],
            ..Default::default()
        }
    }

    #[test]
    fn edit_shape_triangles_respect_bottom_top_and_special_height() {
        let mut wall = volume();
        let triangles = wall.edit_shape_tris();
        assert_eq!(triangles[0].points, [Vec3::new(0.0, 1.0, 0.2875), Vec3::new(1.0, 0.0, 0.2875), Vec3::new(0.0, 2.5, 0.2875)]);
        wall.special = 1;
        wall.quads[0].segment_heights = [2.0, 3.0];
        let triangles = wall.edit_shape_tris();
        assert_eq!(triangles[0].points[2].y, 2.0);
        assert_eq!(triangles[1].points[1].y, 2.0);
    }

    #[test]
    fn edit_shape_triangles_preserve_nan_propagation() {
        let mut wall = volume();
        wall.quads[0].prepared_u[0] = f32::NAN;
        assert!(wall.edit_shape_tris().iter().all(|triangle| triangle.points.iter().all(|point| point.is_nan())));
    }

    #[test]
    fn range_clips_quad_endpoints_and_keeps_reverse_winding() {
        let mut wall = volume();
        let full = wall.edit_shape_tris();
        let clipped = wall.edit_shape_tris_in_range(Some([0.25, 0.75]));
        let lerp = |a: Vec3, b: Vec3, t: f32| b * t + a * (1.0 - t);
        assert_eq!(clipped[0].points, [lerp(full[0].points[0], full[0].points[1], 0.25), lerp(full[0].points[0], full[0].points[1], 0.75), lerp(full[0].points[2], full[1].points[1], 0.25)]);
        wall.quads[0].prepared_u = [1.0, 0.0];
        let reverse = wall.edit_shape_tris_in_range(Some([0.25, 0.75]));
        assert_eq!(reverse[0].points[0], clipped[0].points[0]);
        assert_eq!(reverse[0].points[1], clipped[0].points[1]);
        let mut second = wall.quads[0].clone();
        wall.quads[0].prepared_u = [0.0, 0.5];
        second.prepared_u = [0.5, 1.0];
        wall.quads.push(second);
        assert_eq!(wall.edit_shape_tris_in_range(Some([0.25, 0.75])).len(), 4);
    }

    #[test]
    fn range_rejects_empty_invalid_and_nan() {
        assert!(std::panic::catch_unwind(|| WallTriggerVolume::default().edit_shape_tris_in_range(Some([0.0, 1.0]))).is_err());
        for range in [[0.5, 0.5], [0.75, 0.25], [f32::NAN, 1.0]] {
            assert!(std::panic::catch_unwind(|| volume().edit_shape_tris_in_range(Some(range))).is_err());
        }
        let mut wall = volume();
        wall.quads[0].prepared_u[0] = f32::NAN;
        assert!(std::panic::catch_unwind(|| wall.edit_shape_tris_in_range(Some([0.0, 1.0]))).is_err());
    }

    #[test]
    fn range_clip_matches_executed_original_sse_bits() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../reverse-engineering/tiny-glade/evidence/country-core-deep/wall/m01-get-tris-sse-fixture.json"))).unwrap();
        assert_eq!(fixture["pe_sha256"], "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03");
        for case in fixture["cases"].as_array().unwrap() {
            let bits = |key| u32::from_str_radix(case[key].as_str().unwrap(), 16).unwrap();
            assert_eq!(clip_u_sse(f32::from_bits(bits("low")), f32::from_bits(bits("high")), f32::from_bits(bits("u"))).to_bits(), bits("result"));
        }
    }
}

#[derive(Clone, Copy)]
struct Endpoint {
    bottom: Vec3,
    top: Vec3,
    normal: Vec3,
    u: f32,
    horizontal: f32,
    source_y: f32,
    segment_height: f32,
}
