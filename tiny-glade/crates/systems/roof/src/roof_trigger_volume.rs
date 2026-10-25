//! B `calculate_roof_trigger_volumes@1421a3220` (12048 original bytes).
//! The producer retains the two shape branches, flat profile, gable omissions,
//! original strip/cap multiplicity, UV domains and pre-noise normals. The
//! underlying profile, rectangle, circle, quaternion and curve algorithms are
//! reused from the current canonical core/utils/glam implementations.
use bevy_ecs::prelude::*;
use country_core::resources::roofs::{
    Roof, RoofTriggerVolume,
    commands::RecalculateRoofCmd,
    geometry::surface::{self, Curve2},
    roof_noise::apply_roof_noise,
    roof_ridge::calc_roof_tip_center_ls,
    roof_visual_shape::get_roof_expanded_top_and_bottom,
};
use glam::{Quat, Vec2, Vec3};
use utils::geometry::{circle::Circle2d, intersect::Triangle, rectangle::Rectangle2d};
use ::utils::bytes::read_f32_le;

const FLAT_Y: f32 = f32::from_bits(0xbf3d70a4);

pub fn calculate_roof_trigger_volumes(
    mut recalc: EventReader<RecalculateRoofCmd>,
    mut volumes: Query<(&mut RoofTriggerVolume, &Roof)>,
) {
    for event in recalc.read() {
        // Native event+8 is the actual roof entity. No wall-map indirection.
        let (mut volume, roof) = volumes
            .get_mut(event.entity)
            .unwrap_or_else(|_| panic!("Couldn't recalcualte roof volume: {event:?}"));
        let mesh = generate_roof_trigger_volume(roof);
        // Original ownership transfer/tick order is triangles, UVs, normals.
        volume.set_triangles(mesh.triangles);
        volume.set_uvs(mesh.uvs);
        volume.set_normals(mesh.normals);
    }
}

#[derive(Debug)]
pub struct RoofTriggerMesh {
    pub triangles: Vec<[Vec3; 3]>,
    pub uvs: Vec<[Vec2; 3]>,
    pub normals: Vec<[Vec3; 3]>,
}
impl RoofTriggerMesh {
    fn new() -> Self {
        Self {
            triangles: Vec::new(),
            uvs: Vec::new(),
            normals: Vec::new(),
        }
    }
    fn push(&mut self, triangle: [Vec3; 3], uv: [Vec2; 3], normals: [Vec3; 3]) {
        self.triangles.push(triangle);
        self.uvs.push(uv);
        self.normals.push(normals);
    }
}

/// Complete original producer, also exposed for independent native comparison.
/// Mesh Y is the original local height; consumers add the roof floor themselves.
pub fn generate_roof_trigger_volume(roof: &Roof) -> RoofTriggerMesh {
    let subdivisions = if roof.is_flat() { 4 } else { 9 };
    let mut mesh = RoofTriggerMesh::new();
    if roof.observed()[8] & 1 != 0 {
        rectangle(roof, subdivisions, &mut mesh);
    } else {
        circle(roof, subdivisions, &mut mesh);
    }
    // Native normals are intentionally generated before noise and not recomputed.
    for triangle in &mut mesh.triangles {
        for vertex in triangle {
            let strength = vertex.y - roof.floor_y();
            apply_roof_noise(vertex.x, vertex.z, strength, &mut vertex.y);
        }
    }
    mesh
}

fn at(roof: &Roof, offset: usize) -> f32 {
    read_f32_le(roof.observed(), offset)
}
fn blend2(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    b * t + a * (1.0 - t)
}
fn blend3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    b * t + a * (1.0 - t)
}
fn position(roof_rectangle: Rectangle2d, local: Vec2, height: f32) -> Vec3 {
    let orientation = roof_rectangle.orientation_mat2();
    let point = Vec2::new(
        roof_rectangle.center.x + (local.y * orientation.y_axis.x + local.x * orientation.x_axis.x),
        roof_rectangle.center.y + (local.y * orientation.y_axis.y + local.x * orientation.x_axis.y),
    );
    Vec3::new(point.x, height, point.y)
}
fn rectangle(roof: &Roof, subdivisions: usize, out: &mut RoofTriggerMesh) {
    // Original copies the authoritative rectangle; it does not call new() here.
    let native = Rectangle2d {
        center: Vec2::new(at(roof, 0x14), at(roof, 0x18)),
        size: Vec2::new(at(roof, 0x1c), at(roof, 0x20)),
        axis: Vec2::new(at(roof, 0x0c), at(roof, 0x10)),
    };
    let (mut bottom, mut top) = get_roof_expanded_top_and_bottom(roof).as_rectangle();
    if !roof.is_flat() {
        for value in bottom.iter_mut().chain(top.iter_mut()) {
            *value += 0.4;
        }
    }
    let profile = surface::normalized_profile_curve(at(roof, 0x2c))
        .expect("original normalized roof profile");
    let mut bottom = Rectangle2d::new_centered_axis_aligned(Vec2::from(bottom));
    // Both scale arguments are exactly zero at original 1421a360d..3616.
    let center = Vec2::from(calc_roof_tip_center_ls(roof, [0.0, 0.0]));
    let mut top = Rectangle2d::new_axis_aligned(center, Vec2::from(top));
    let gables = roof.gable_sides();
    if gables.is_some() {
        let axis = usize::from(roof.observed()[0x3c] != 0);
        let eave = surface::eave_length_ws(at(roof, 0x2c), at(roof, 0x38));
        bottom.size[axis] -= eave + 0.56;
        top.size[axis] += -0.56;
    }
    // Original as_curve2 also validates positive area and curve ownership.
    let bottom = bottom.as_curve2();
    let top = top.as_curve2();
    let height = |u: f32| {
        if roof.is_flat() {
            FLAT_Y
        } else {
            let y = profile.pos_at_u(u)[1];
            roof.height() * y + (1.0 - y) * 0.0
        }
    };
    for side in 0..4usize {
        if gables.is_some_and(|s| s.contains(&(side as u8))) {
            continue;
        }
        let b0 = bottom.points[side];
        let b1 = bottom.points[side + 1];
        let t0 = top.points[side];
        let t1 = top.points[side + 1];
        let start_u = side as f32 * 0.25;
        let end_u = (side as f32 + 1.0) * 0.25;
        for i in 0..subdivisions {
            let v0 = i as f32 / subdivisions as f32;
            let v1 = (i as f32 + 1.0) / subdivisions as f32;
            let p0 = profile.pos_at_u(v0)[0];
            let p1 = profile.pos_at_u(v1)[0];
            let l0 = blend2(b0, t0, p0);
            let r0 = blend2(b1, t1, p0);
            let l1 = blend2(b0, t0, p1);
            let r1 = blend2(b1, t1, p1);
            let y0 = height(v0);
            let y1 = height(v1);
            for q in 0..4 {
                let s0 = q as f32 * 0.25;
                let s1 = (q as f32 + 1.0) * 0.25;
                let a = position(native, blend2(l0, r0, s0), y0);
                let b = position(native, blend2(l0, r0, s1), y0);
                let c = position(native, blend2(l1, r1, s0), y1);
                let d = position(native, blend2(l1, r1, s1), y1);
                let normal = Triangle { points: [a, b, c] }.normal();
                let u0 = s0 * end_u + (1.0 - s0) * start_u;
                let u1 = s1 * end_u + (1.0 - s1) * start_u;
                out.push(
                    [a, b, c],
                    [Vec2::new(u0, v0), Vec2::new(u1, v0), Vec2::new(u0, v1)],
                    [normal; 3],
                );
                out.push(
                    [b, d, c],
                    [Vec2::new(u1, v0), Vec2::new(u1, v1), Vec2::new(u0, v1)],
                    [normal; 3],
                );
            }
        }
        // The original cap is inside the accepted-side loop (1421a52f0).
        // Its repeated triangles and the gable-skip behavior are observable UV
        // lookup order and must not be deduplicated as a mesh optimization.
        let y = if roof.is_flat() {
            FLAT_Y
        } else {
            roof.height()
        };
        let corners: [Vec3; 4] = std::array::from_fn(|i| position(native, top.points[i], y));
        let (a0, a1, b0, b1) = if roof.observed()[0x3c] & 1 == 0 {
            (corners[0], corners[3], corners[1], corners[2])
        } else {
            (corners[3], corners[2], corners[0], corners[1])
        };
        for q in 0..4 {
            let s0 = q as f32 * 0.25;
            let s1 = (q as f32 + 1.0) * 0.25;
            let a = blend3(a0, a1, s0);
            let b = blend3(a0, a1, s1);
            let c = blend3(b0, b1, s0);
            let d = blend3(b0, b1, s1);
            // Explicit +0 products match native UV interpolation.
            let u0 = s0 + (1.0 - s0) * 0.0;
            let u1 = s1 + (1.0 - s1) * 0.0;
            out.push(
                [a, b, c],
                [Vec2::new(u0, 2.0), Vec2::new(u1, 2.0), Vec2::new(u0, 2.1)],
                [Vec3::Y; 3],
            );
            out.push(
                [b, d, c],
                [Vec2::new(u1, 2.0), Vec2::new(u1, 2.1), Vec2::new(u0, 2.1)],
                [Vec3::Y; 3],
            );
        }
    }
}

fn circle(roof: &Roof, subdivisions: usize, out: &mut RoofTriggerMesh) {
    let center = Vec2::new(at(roof, 0x0c), at(roof, 0x10));
    let rotation = at(roof, 0x18);
    let (mut bottom, mut top) = get_roof_expanded_top_and_bottom(roof).as_circle();
    if !roof.is_flat() {
        bottom += 0.13;
        top += 0.13;
    }
    let profile = roof
        .profile_curve_ws(bottom, top, roof.height())
        .expect("original world roof profile");
    let tip = Vec2::from(roof.roof_tip_offset_xz());
    for i in 0..subdivisions {
        let v0 = i as f32 / subdivisions as f32;
        let v1 = (i as f32 + 1.0) / subdivisions as f32;
        let p0 = profile.pos_at_u(v0);
        let p1 = profile.pos_at_u(v1);
        let y0 = if roof.is_flat() { FLAT_Y } else { p0[1] };
        let y1 = if roof.is_flat() { FLAT_Y } else { p1[1] };
        // Original circles are zero-centered; translation is added afterward.
        let c0 = Circle2d::new_rotated(Vec2::ZERO, p0[0], rotation);
        let c1 = Circle2d::new_rotated(Vec2::ZERO, p1[0], rotation);
        let center0 = center + tip * v0;
        let center1 = center + tip * v1;
        let n0 = profile_normal(&profile, v0);
        let n1 = profile_normal(&profile, v1);
        for q in 0..15 {
            let u0 = q as f32 / 15.0;
            let u1 = (q as f32 + 1.0) / 15.0;
            let a = circle_position(c0, u0, center0, y0);
            let b = circle_position(c0, u1, center0, y0);
            let c = circle_position(c1, u0, center1, y1);
            let d = circle_position(c1, u1, center1, y1);
            let r = utils::circular_f32::CircularF32::new(rotation).as_f32();
            let r0 = Quat::from_rotation_y(u0 * std::f32::consts::TAU - r);
            let r1 = Quat::from_rotation_y(u1 * std::f32::consts::TAU - r);
            let na = r0 * n0;
            let nb = r1 * n0;
            let nc = r0 * n1;
            let nd = r1 * n1;
            out.push(
                [a, b, c],
                [Vec2::new(u0, v0), Vec2::new(u1, v0), Vec2::new(u0, v1)],
                [na, nb, nc],
            );
            out.push(
                [b, d, c],
                [Vec2::new(u1, v0), Vec2::new(u1, v1), Vec2::new(u0, v1)],
                [nb, nd, nc],
            );
        }
    }
    let cap = Circle2d::new_rotated(center, top, rotation);
    let uv_cap = Circle2d::new_rotated(Vec2::new(0.05, 2.05), 0.05, rotation);
    let y = if roof.is_flat() {
        FLAT_Y
    } else {
        roof.height()
    };
    for q in 0..15 {
        let u0 = q as f32 / 15.0;
        let u1 = (q as f32 + 1.0) / 15.0;
        let a = cap.get_pos_at_u(u0);
        let b = cap.get_pos_at_u(u1);
        out.push(
            [
                Vec3::new(a.x, y, a.y),
                Vec3::new(b.x, y, b.y),
                Vec3::new(center.x, y, center.y),
            ],
            [
                uv_cap.get_pos_at_u(u0),
                uv_cap.get_pos_at_u(u1),
                uv_cap.center,
            ],
            [Vec3::Y; 3],
        );
    }
}
fn circle_position(circle: Circle2d, u: f32, center: Vec2, y: f32) -> Vec3 {
    let p = circle.get_pos_at_u(u);
    Vec3::new(p.x + center.x, 0.0 + y, p.y + center.y)
}
fn profile_normal(profile: &Curve2, u: f32) -> Vec3 {
    let (i, _) = profile.coord_at_u(u);
    let a = profile.points[i];
    let b = profile.points[i + 1];
    let dx = b[0] - a[0];
    let dy = b[1] - a[1];
    let inv = 1.0 / (dy * dy + dx * dx).sqrt();
    Vec3::new(
        dy * inv,
        f32::from_bits(dx.to_bits() ^ 0x8000_0000) * inv,
        -0.0,
    )
}

#[cfg(test)]
mod original_tests {
    use super::*;
    // Frozen unchanged-RNE execution; never synthesized from this implementation.
    const FIXTURES: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../reverse-engineering/tiny-glade/evidence/round12/roof-visual/native-trigger-fixtures.bin"
    ));
    #[test]
    fn whole_trigger_producer_matches_original_rne_shape_branches() {
        let mut input = FIXTURES;
        fn word(input: &mut &[u8]) -> u32 {
            let (a, b) = input.split_at(4);
            *input = b;
            u32::from_le_bytes(a.try_into().unwrap())
        }
        let count = word(&mut input);
        let mut max_abs = 0.0f32;
        let mut unequal_bits = 0usize;
        for case in 0..count {
            let (raw, rest) = input.split_at(88);
            input = rest;
            let roof = Roof::from_observed(raw.try_into().unwrap());
            let expected_count = word(&mut input) as usize;
            let mesh = generate_roof_trigger_volume(&roof);
            assert_eq!(mesh.triangles.len(), expected_count, "case {case}");
            assert_eq!(mesh.normals.len(), expected_count);
            assert_eq!(mesh.uvs.len(), expected_count);
            let positions: Vec<f32> = mesh
                .triangles
                .iter()
                .flat_map(|t| t.iter().flat_map(|v| v.to_array()))
                .collect();
            let uvs: Vec<f32> = mesh
                .uvs
                .iter()
                .flat_map(|t| t.iter().flat_map(|v| v.to_array()))
                .collect();
            let normals: Vec<f32> = mesh
                .normals
                .iter()
                .flat_map(|t| t.iter().flat_map(|v| v.to_array()))
                .collect();
            for (field, actual) in [("positions", positions), ("uvs", uvs), ("normals", normals)] {
                for (index, value) in actual.into_iter().enumerate() {
                    let original = f32::from_bits(word(&mut input));
                    if value.to_bits() != original.to_bits() {
                        unequal_bits += 1;
                    }
                    let error = (value - original).abs();
                    max_abs = max_abs.max(error);
                    assert!(
                        value.is_finite() && original.is_finite() && error <= 0.00002,
                        "case {case} {field}[{index}]: actual {value:?} {:08x}, original {original:?} {:08x}, delta {error}",
                        value.to_bits(),
                        original.to_bits()
                    );
                }
            }
        }
        assert!(input.is_empty());
        eprintln!(
            "original B trigger: {count} shapes, maximum absolute error {max_abs}, {unequal_bits} unequal f32 bit patterns"
        );
    }
}
