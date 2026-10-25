//! Original rectangular face-tile loop, separated at its recovered row-vector
//! boundary. Row interpolation and whole-roof assembly are composed by callers.
use super::surface::Curve2;
use super::tiles::{
    ObservedRectangle, RoofTileRecord, TileRng, half, matrix_quat, random_splits, record,
    rectangular_context_from_observed,
};

#[derive(Clone, Debug)]
pub struct FaceRowsInput<'a> {
    /// [left x,z, right x,z, normalized row u, world y], excluding repeated last row.
    pub rows: &'a [[f32; 6]],
    pub world_profile: &'a Curve2,
    pub row_count: f32,
    pub rectangle: [f32; 6],
    pub seam_planes: [[f32; 4]; 2],
    pub roof_id: u32,
    pub special_mode: bool,
    pub overhang: bool,
}
#[derive(Clone, Debug)]
pub struct PreparedRectangularFace {
    pub rows: Vec<[f32; 6]>,
    pub world_profile: Curve2,
    pub row_count: f32,
    pub seam_planes: [[f32; 4]; 2],
    pub face_index: usize,
}
fn plane(rect: [f32; 6], from: [f32; 2], to: [f32; 2], origin: [f32; 2]) -> [f32; 4] {
    let delta = [to[0] - from[0], to[1] - from[1]];
    let world_x = delta[1] * (-rect[1]) + delta[0] * rect[0];
    let world_z = delta[1] * rect[0] + delta[0] * rect[1];
    let inv = 1.0 / (world_z * world_z + world_x * world_x).sqrt();
    let normal = [world_z * (-inv), 0.0, world_x * inv];
    let p = world(rect, origin);
    let d = f32::from_bits(0xbd99999a) - ((p[1] * normal[2] + p[0] * normal[0]) + 0.0 * normal[1]);
    [normal[0], normal[1], normal[2], d]
}
/// Complete original CurveU row interpolation and seam-plane preparation.
pub fn prepare_rectangular_faces_observed(
    rect: &[u8; 24],
    roof: &[u8; 88],
) -> Result<Vec<PreparedRectangularFace>, super::surface::CurveError> {
    let f = |o| f32::from_le_bytes(roof[o..o + 4].try_into().unwrap());
    let context = rectangular_context_from_observed(rect, roof);
    let frame = ObservedRectangle::from_bytes(rect).0;
    let normalized = super::surface::normalized_profile_curve(f(0x2c))?;
    let height = super::surface::height(roof);
    let mut out = Vec::new();
    let gable = (f(0x30) > 0.1 || f(0x30).is_nan()) && f(0x34) == 1.0;
    for index in 0..4 {
        if gable && (index == usize::from(roof[0x3c]) || index == usize::from(roof[0x3c] | 2)) {
            continue;
        }
        let next = (index + 1) % 4;
        let lower = [context.bottom_corners[index], context.bottom_corners[next]];
        let upper = [context.top_corners[index], context.top_corners[next]];
        let radial = |a: [f32; 2], b: [f32; 2]| {
            let dx = context.local_tip_center[0] - (a[0] + b[0]) * 0.5;
            let dz = context.local_tip_center[1] - (a[1] + b[1]) * 0.5;
            (dz * dz + dx * dx).sqrt()
        };
        let world_profile = Curve2::try_new(
            super::surface::profile_curve_ws_points(
                &normalized.points,
                radial(lower[0], lower[1]),
                radial(upper[0], upper[1]),
                height,
            ),
            false,
        )?;
        let count = (world_profile.length / 0.4375).ceil().max(2.0);
        let end = (count as i32) - 1;
        let mut rows = Vec::new();
        for row in 0..end {
            let u = (row as f32) / (count + (-1.0));
            let p = normalized.pos_at_u(u);
            let x = p[0];
            let one = 1.0 - x;
            rows.push([
                x * upper[0][0] + lower[0][0] * one,
                x * upper[0][1] + lower[0][1] * one,
                x * upper[1][0] + lower[1][0] * one,
                x * upper[1][1] + lower[1][1] * one,
                u,
                height * p[1] + (1.0 - p[1]) * 0.0,
            ]);
        }
        let seam_planes = [
            plane(frame, lower[0], upper[0], lower[0]),
            plane(frame, upper[1], lower[1], lower[1]),
        ];
        out.push(PreparedRectangularFace {
            rows,
            world_profile,
            row_count: count,
            seam_planes,
            face_index: index,
        });
    }
    Ok(out)
}

/// Original rectangle/Roof inputs through all pitched face records and fillers.
/// Caller passes the RNG and ordinal left by edge and ridge-cap stages.
pub fn assemble_faces_from_observed(
    rect: &[u8; 24],
    roof: &[u8; 88],
    special_mode: bool,
    rng: &mut TileRng,
    ordinal: &mut u32,
) -> Result<Vec<RoofTileRecord>, super::surface::CurveError> {
    let faces = prepare_rectangular_faces_observed(rect, roof)?;
    let mut out = Vec::new();
    for face in faces {
        let input = FaceRowsInput {
            rows: &face.rows,
            world_profile: &face.world_profile,
            row_count: face.row_count,
            rectangle: ObservedRectangle::from_bytes(rect).0,
            seam_planes: face.seam_planes,
            roof_id: u32::from_le_bytes(roof[..4].try_into().unwrap()),
            special_mode,
            overhang: roof[0x54] & 1 != 0,
        };
        out.extend(assemble_rectangular_face_tiles(&input, rng, ordinal));
    }
    Ok(out)
}
fn world(rect: [f32; 6], p: [f32; 2]) -> [f32; 2] {
    [
        rect[2] + (p[1] * (-rect[1]) + p[0] * rect[0]),
        rect[3] + (p[1] * rect[0] + p[0] * rect[1]),
    ]
}
fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    glam::Vec3A::from(a).cross(glam::Vec3A::from(b)).into()
}
fn packed(
    pos: [f32; 3],
    dims: [f32; 3],
    quat: [f32; 4],
    u: f32,
    plane: [f32; 4],
    roof: u32,
    ordinal: u32,
    mode: u32,
) -> RoofTileRecord {
    let mut tile = record(pos, dims, quat, u, roof, ordinal, mode);
    for (i, p) in plane.iter().enumerate() {
        tile.0[16 + i * 2..18 + i * 2].copy_from_slice(&half(*p).to_le_bytes());
    }
    tile.0[56..60].copy_from_slice(&0u32.to_le_bytes());
    tile
}
fn center_world(rect: [f32; 6], r: &[f32; 6]) -> [f32; 3] {
    let _ = rect;
    [(r[2] + r[0]) * 0.5, r[5], (r[3] + r[1]) * 0.5]
}
fn distance3(a: [f32; 3], b: [f32; 3]) -> f32 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    (dz * dz + (dy * dy + dx * dx)).sqrt()
}

pub fn assemble_rectangular_face_tiles(
    input: &FaceRowsInput<'_>,
    rng: &mut TileRng,
    ordinal: &mut u32,
) -> Vec<RoofTileRecord> {
    assert!(input.rows.len() >= 2);
    let mut out = Vec::new();
    for (row_index, row) in input.rows.iter().enumerate() {
        let next = if row_index + 1 < input.rows.len() {
            &input.rows[row_index + 1]
        } else {
            row
        };
        let dx = row[0] - row[2];
        let dz = row[1] - row[3];
        let span = (dz * dz + dx * dx).sqrt();
        let columns = ((span + span).ceil().max(3.0)) as usize;
        let splits = random_splits(columns + 1, 0.0 / span, rng);
        let a = center_world(input.rectangle, row);
        let b = center_world(input.rectangle, next);
        let step = if row_index + 1 == input.rows.len() {
            distance3(a, center_world(input.rectangle, &input.rows[row_index - 1]))
        } else {
            distance3(a, b)
        };
        let tile_height = step * f32::from_bits(0x3fb6db6e);
        for us in splits.windows(2) {
            let p0local = [
                row[2] * us[0] + row[0] * (1.0 - us[0]),
                row[3] * us[0] + row[1] * (1.0 - us[0]),
            ];
            let p1local = [
                row[2] * us[1] + row[0] * (1.0 - us[1]),
                row[3] * us[1] + row[1] * (1.0 - us[1]),
            ];
            let p0 = world(input.rectangle, p0local);
            let p1 = world(input.rectangle, p1local);
            let dx = p0[0] - p1[0];
            let dz = p0[1] - p1[1];
            let width = (dz * dz + dx * dx).sqrt();
            let inv = 1.0 / width;
            let axis = [inv * dx, 0.0, inv * dz];
            let perp = [-axis[2], 0.0, axis[0]];
            let pos = [(p1[0] + p0[0]) * 0.5, row[5], (p1[1] + p0[1]) * 0.5];
            let rand = rng.next_f32();
            let scale = (1.0 - rand) * 0.8 + rand * 1.2;
            let (i, _) = input.world_profile.coord_at_u(row[4]);
            let p = input.world_profile.points[i];
            let q = input.world_profile.points[i + 1];
            let dr = q[0] - p[0];
            let dh = q[1] - p[1];
            let inv = 1.0 / (dh * dh + dr * dr).sqrt();
            let slope = ((inv * dh) / (dr * inv)).atan();
            let normal: [f32; 3] = (glam::Quat::from_axis_angle(glam::Vec3::from(axis), slope)
                * glam::Vec3A::from(perp))
            .into();
            let down = [-normal[0], -normal[1], -normal[2]];
            let quat = matrix_quat(axis, down, cross(normal, axis));
            let plane = input.seam_planes[usize::from(us[0] > 0.5)];
            let tile_h = scale * tile_height;
            let jitter = (rng.next_f32() * 0.2 + (row_index as f32) / (input.row_count + (-1.0)))
                .clamp(0.0, 1.0);
            let mode = if row_index == 0
                || input.special_mode
                || (input.overhang && (us[0] * span < 0.5 || (1.0 - us[1]) * span < 0.5))
            {
                2
            } else {
                1
            };
            out.push(packed(
                pos,
                [width, tile_h, 0.1],
                quat,
                jitter,
                plane,
                input.roof_id,
                *ordinal,
                mode,
            ));
            if row_index == 0 {
                let a = [
                    pos[0] + tile_h * normal[0] * 0.5,
                    pos[1] - tile_h * normal[1] * 0.5,
                    pos[2] + tile_h * normal[2] * 0.5,
                ];
                let b = [
                    pos[0] - tile_h * normal[0] * 0.5,
                    a[1],
                    pos[2] - tile_h * normal[2] * 0.5,
                ];
                let dx = a[0] - b[0];
                let dy = a[1] - b[1];
                let dz = a[2] - b[2];
                let len = (dz * dz + (dy * dy + dx * dx)).sqrt();
                if len > 0.1 {
                    let dir = [axis[2], 0.0, -axis[0]];
                    let q = matrix_quat(axis, dir, cross(axis, dir));
                    out.push(packed(
                        [
                            b[0] * 0.5 + a[0] * 0.5,
                            b[1] * 0.5 + a[1] * 0.5,
                            b[2] * 0.5 + a[2] * 0.5,
                        ],
                        [width, len, 0.1],
                        q,
                        0.0,
                        plane,
                        input.roof_id,
                        *ordinal,
                        2,
                    ));
                }
            }
            *ordinal += 1;
        }
    }
    out
}
