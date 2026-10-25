//! B actual platform stair consumers. Step ranges are local to the preceding
//! mask; actual manual-stair blockers remove their horizontal neighbourhood.
//! Shared materialization and wall instance producers remain shared owners.

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::manual_range_patterns, clippy::neg_cmp_op_on_partial_ord)]
use crate::resources::roofs::geometry::surface::Curve2;
use crate::resources::roofs::geometry::tiles::TileRng;
use crate::resources::roofs::geometry::tiles::random_splits;
use crate::resources::walls::decorator::placement::PlacementError;
use crate::resources::walls::decorator::placement::wall_coord_from_world;
use crate::resources::walls::decorator::placement::wall_coord_to_world;
use crate::resources::walls::publication_source::TerrainHeightGrid512;
use crate::systems::wall::construction::platform_segmentation::WallPlatformStepRange;
use crate::systems::wall::construction::segmentation::WallLoopRange;
use crate::systems::wall::construction::segmentation::WallSegmentationError;
use crate::systems::wall::construction::segmentation::process_segment;
use crate::systems::wall::construction::structures::MaterializedWallSegment;
use crate::systems::wall::construction::structures::WallSegmentSource;
use crate::systems::wall::construction::structures::WallStructuresError;
use crate::systems::wall::construction::structures::materialize_wall_segments;
use glam::{Vec2, Vec3};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallPlatformStairBlocker {
    /// Original WallSpace coordinate in physical metres, not normalized CurveU.
    pub coordinate: [f32; 2],
    pub width: f32,
}
#[derive(Clone, Copy)]
pub struct WallPlatformStairSource<'a> {
    pub raw_curve: &'a Curve2,
    pub public: &'a [u8; 672],
    pub prepared_positions: &'a [[f32; 3]],
    pub prepared_u: &'a [f32],
    pub segment_values: &'a [f32],
    pub arch_heights: &'a [f32],
    pub steps: &'a [WallPlatformStepRange],
    pub blockers: &'a [WallPlatformStairBlocker],
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WallPlatformStairError {
    InvalidInput,
    Placement(PlacementError),
    Segmentation(WallSegmentationError),
    Materialization(WallStructuresError),
    InvalidStairExtraInfo,
}
/// Exact projection of the original 80B decorator row. The source record is
/// observed authority; this does not assert the original Rust struct ABI.
#[derive(Clone, Copy)]
pub struct WallPlatformDecoratorObserved(pub [u8; 80]);
pub fn collect_platform_stair_blockers(
    raw_curve: &Curve2,
    terrain: &TerrainHeightGrid512,
    decorators: &[WallPlatformDecoratorObserved],
) -> Result<Vec<WallPlatformStairBlocker>, WallPlatformStairError> {
    let mut result = Vec::new();
    for record in decorators {
        let raw = &record.0;
        if raw[0x4e] != 0x3f || matches!(raw[0x2a], 2 | 3 | 4) {
            continue;
        }
        let f = |at| f32::from_le_bytes(raw[at..at + 4].try_into().unwrap());
        let coordinate = [f(8), f(12)];
        let point = wall_coord_to_world(raw_curve, coordinate)
            .map_err(WallPlatformStairError::Placement)?;
        if !(coordinate[1] - terrain.sample_at_world_xz(point[0], point[2]) < 3.0) {
            continue;
        }
        // DecoratorExtraInfo::as_stair accepts only lowtag bits0/1; caller
        // then reads the actual variant width at extra+4 (record+3c).
        if raw[0x49] & 7 > 1 {
            return Err(WallPlatformStairError::InvalidStairExtraInfo);
        }
        let width = f(0x3c);
        if !width.is_finite() {
            return Err(WallPlatformStairError::InvalidInput);
        }
        result.push(WallPlatformStairBlocker { coordinate, width });
    }
    Ok(result)
}
fn wall_space_distance(a: f32, b: f32, closed: bool, period: f32) -> f32 {
    let difference = (a - b).abs();
    if closed {
        (period - difference).abs().min(difference)
    } else {
        difference
    }
}
/// Original process_loop_range_into_wallpath_segments, invoked by the actual
/// creator before random horizontal partitions and stair geometry creation.
/// Blockers must come from its actual decorator/terrain filter; none is not an
/// invented substitute for a missing source resource.
pub fn prepare_platform_stair_segments(
    input: WallPlatformStairSource<'_>,
) -> Result<Vec<MaterializedWallSegment>, WallPlatformStairError> {
    if input.steps.is_empty() {
        return Ok(Vec::new());
    }
    if input.prepared_positions.len() != input.prepared_u.len()
        || input.prepared_positions.len() != input.segment_values.len()
        || input.prepared_positions.len() != input.arch_heights.len()
    {
        return Err(WallPlatformStairError::InvalidInput);
    }
    let shape = u32::from_le_bytes(input.public[0x258..0x25c].try_into().unwrap());
    let closed = shape != 2;
    let period = f32::from_le_bytes(input.public[0x28c..0x290].try_into().unwrap());
    if !period.is_finite()
        || period <= 0.0
        || input
            .blockers
            .iter()
            .any(|b| !b.width.is_finite() || b.coordinate.iter().any(|v| !v.is_finite()))
    {
        return Err(WallPlatformStairError::InvalidInput);
    }
    let mut result = Vec::new();
    for step in input.steps {
        if step.start > step.end || step.end > step.source_range.len() {
            return Err(WallPlatformStairError::InvalidInput);
        }
        let range = WallLoopRange {
            start: step.source_range.start + step.start as i32,
            end: step.source_range.start + step.end as i32,
            modulus: step.source_range.modulus,
        };
        let mut permitted = Vec::new();
        for j in 0..range.len() {
            let k = range
                .index_at(j)
                .map_err(WallPlatformStairError::Segmentation)?;
            let point = *input
                .prepared_positions
                .get(k)
                .ok_or(WallPlatformStairError::InvalidInput)?;
            let coord = wall_coord_from_world(input.raw_curve, point)
                .map_err(WallPlatformStairError::Placement)?;
            permitted.push(!input.blockers.iter().any(|b| {
                wall_space_distance(coord[0], b.coordinate[0], closed, period) < b.width * 0.5 + 0.2
            }));
        }
        let mut begin = 0;
        while begin < permitted.len() {
            if !permitted[begin] {
                begin += 1;
                continue;
            }
            let mut end = begin + 1;
            while end < permitted.len() && permitted[end] {
                end += 1;
            }
            if end - begin >= 2 {
                let sub = WallLoopRange {
                    start: range.start + begin as i32,
                    end: range.start + end as i32,
                    modulus: range.modulus,
                };
                let points: Vec<_> = (0..sub.len())
                    .map(|i| sub.index_at(i).map(|k| input.prepared_positions[k]))
                    .collect::<Result<_, _>>()
                    .map_err(WallPlatformStairError::Segmentation)?;
                let mask = process_segment(&points, &points, sub, 10000.0)
                    .map_err(WallPlatformStairError::Segmentation)?;
                result.extend(
                    materialize_wall_segments(WallSegmentSource {
                        mask: &mask,
                        segment_values: input.segment_values,
                        arch_heights: input.arch_heights,
                        prepared_positions: input.prepared_positions,
                        prepared_u: input.prepared_u,
                        regular: shape < 2,
                        closed,
                    })
                    .map_err(WallPlatformStairError::Materialization)?,
                );
            }
            begin = end;
        }
    }
    Ok(result)
}

/// A single actual platform creator cell after Curve3::get_coord_at_u. Each
/// endpoint refers to the original prepared curve, including its tangent.
#[derive(Clone, Copy)]
pub struct WallPlatformStairCellSource<'a> {
    pub prepared_positions: &'a [[f32; 3]],
    pub coordinates: [(usize, f32); 2],
    pub maximum_y: f32,
    pub constrained_extra: f32,
    pub terrain: &'a TerrainHeightGrid512,
    pub index_offset: u32,
}
#[derive(Clone, Debug)]
pub struct WallPlatformStairCell {
    pub top: f32,
    /// Selected render base. Collider uses the separately computed terrain
    /// base, so this field must not replace its lower vertices.
    pub render_bottom: f32,
    pub rise_count: u64,
    pub depth: f32,
    pub vertices: [[f32; 3]; 8],
    pub indices: [u32; 36],
}
pub fn platform_stair_cell(
    input: WallPlatformStairCellSource<'_>,
) -> Result<WallPlatformStairCell, WallPlatformStairError> {
    if !input.maximum_y.is_finite() || !input.constrained_extra.is_finite() {
        return Err(WallPlatformStairError::InvalidInput);
    }
    let mut points = [[0.; 2]; 2];
    let mut normals = [[0.; 2]; 2];
    for (j, &(i, t)) in input.coordinates.iter().enumerate() {
        let a = *input
            .prepared_positions
            .get(i)
            .ok_or(WallPlatformStairError::InvalidInput)?;
        let b = *input
            .prepared_positions
            .get(i + 1)
            .ok_or(WallPlatformStairError::InvalidInput)?;
        if !t.is_finite() || !(0.0..=1.0).contains(&t) || a.iter().chain(&b).any(|v| !v.is_finite())
        {
            return Err(WallPlatformStairError::InvalidInput);
        }
        let delta = Vec3::from_array(b) - Vec3::from_array(a);
        let length = delta.length();
        if length == 0. || Vec2::new(delta.x, delta.z).length() == 0. {
            return Err(WallPlatformStairError::InvalidInput);
        }
        let inverse = 1.0 / length;
        normals[j] = [-delta.z * inverse, delta.x * inverse];
        points[j] = [(1.0 - t) * a[0] + t * b[0], (1.0 - t) * a[2] + t * b[2]];
    }
    let rise = f32::from_bits(0x3eb0a3d7);
    let top = (input.maximum_y + f32::from_bits(0xbf30a3d7)) + f32::from_bits(0xbe30a3d7);
    let mut terrain_bottom = (input.maximum_y - input.constrained_extra) + rise;
    for j in 0..2 {
        let norm = Vec2::from_array(normals[j]);
        let inverse = 1.0 / norm.length();
        let offset = norm * inverse * f32::from_bits(0x3ff95810);
        let ground = input
            .terrain
            .sample_at_world_xz(points[j][0] + offset.x, points[j][1] + offset.y);
        terrain_bottom = terrain_bottom.min(ground + rise);
    }
    let rise_count = (((top - terrain_bottom) / rise).floor() as u64).clamp(2, 4);
    let mut bottom = terrain_bottom.max(top - rise_count as f32 * rise);
    let mut count = rise_count;
    let mut depth = (count as f32 * f32::from_bits(0x3ee56042)).min(1.5);
    let selected_height = top - bottom;
    if (top - terrain_bottom) - selected_height > selected_height {
        bottom = terrain_bottom.max(top + f32::from_bits(0xbf30a3d7));
        count = 2;
        depth = f32::from_bits(0x3f656042);
    }
    let mut outer = [[0.; 2]; 2];
    for j in 0..2 {
        let normal = Vec3::new(normals[j][0], 0.0, normals[j][1]);
        let inverse = 1.0 / normal.length();
        outer[j] = [
            (normal.x * inverse) * depth + points[j][0],
            (normal.z * inverse) * depth + points[j][1],
        ];
    }
    let xvy = |p: [f32; 2], y| [p[0], y, p[1]];
    let upper = [
        xvy(points[0], top),
        xvy(outer[0], terrain_bottom + 0.05),
        xvy(outer[1], terrain_bottom + 0.05),
        xvy(points[1], top),
    ];
    let lower = [
        xvy(points[0], terrain_bottom),
        xvy(outer[0], terrain_bottom),
        xvy(outer[1], terrain_bottom),
        xvy(points[1], terrain_bottom),
    ];
    let vertices = std::array::from_fn(|i| if i < 4 { upper[i] } else { lower[i - 4] });
    let local_indices = [
        0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 3, 2, 6, 6, 7, 3, 1, 0, 4, 4, 5, 1, 2, 1, 5, 5, 6, 2,
        0, 3, 7, 7, 4, 0,
    ];
    let indices = local_indices.map(|i: u32| i.wrapping_add(input.index_offset));
    Ok(WallPlatformStairCell {
        top,
        render_bottom: bottom,
        rise_count: count,
        depth,
        vertices,
        indices,
    })
}

#[derive(Clone, Debug)]
pub struct WallPlatformStairGeometry {
    pub cells: Vec<WallPlatformStairCell>,
    /// Original MaterialStats::add_stairs_length receives the 3D segment sum,
    /// including geometry cells whose later render-curve creation can fail.
    pub stairs_length: f32,
}
/// Actual creator geometry after its platform-step/materialized-segment owner.
/// Render instance rows and OriginWallHoles remain separate consumers.
pub fn platform_stair_geometry(
    public: &[u8; 672],
    prepared_positions: &[[f32; 3]],
    prepared_u: &[f32],
    segments: &[MaterializedWallSegment],
    terrain: &TerrainHeightGrid512,
) -> Result<WallPlatformStairGeometry, WallPlatformStairError> {
    if segments.is_empty() {
        return Ok(WallPlatformStairGeometry {
            cells: Vec::new(),
            stairs_length: 0.0,
        });
    }
    if prepared_positions.len() < 2
        || prepared_positions.len() != prepared_u.len()
        || prepared_u.iter().any(|u| !u.is_finite())
        || prepared_u.windows(2).any(|p| p[0] > p[1])
    {
        return Err(WallPlatformStairError::InvalidInput);
    }
    // Curve3 and Curve2 share the original CurveU coordinate selector. Reuse
    // the existing private utility implementation, preserving actual 3D Us.
    let coordinate_curve = Curve2 {
        points: prepared_positions.iter().map(|p| [p[0], p[2]]).collect(),
        points_u: prepared_u.to_vec(),
        length: 1.0,
    };
    let maximum =
        crate::resources::walls::publication_fields::PublicWallBuildingState::from_observed(
            *public,
        )
        .max_y()
        .map_err(|_| WallPlatformStairError::InvalidInput)?;
    let extra = f32::from_le_bytes(public[0x284..0x288].try_into().unwrap());
    let mut cells = Vec::new();
    let mut stairs_length = 0.0;
    for segment in segments {
        if segment.positions.len() < 2
            || segment.points_u.len() != segment.positions.len()
            || segment.points_u.iter().any(|u| !u.is_finite())
        {
            return Err(WallPlatformStairError::InvalidInput);
        }
        let length: f32 = segment
            .positions
            .windows(2)
            .map(|p| (Vec3::from_array(p[0]) - Vec3::from_array(p[1])).length())
            .sum();
        if length < 0.5 {
            continue;
        }
        let length_xz: f32 = segment
            .positions
            .windows(2)
            .map(|p| Vec2::new(p[0][0] - p[1][0], p[0][2] - p[1][2]).length())
            .sum();
        if !length.is_finite() || !length_xz.is_finite() {
            return Err(WallPlatformStairError::InvalidInput);
        }
        let count = ((length_xz * 0.5).round() as usize).max(1) + 1;
        let mut rng = TileRng(0x4d58618879da0616); // actual utils::init_rng(0)
        let boundaries = random_splits(count, 0.0, &mut rng);
        let project = |u: f32| {
            let r = u % 1.0;
            if r < 0.0 { r + 1.0 } else { r }
        };
        let first = project(segment.points_u[0]);
        let mut end = project(*segment.points_u.last().unwrap());
        if first == 0.0 && length_xz > 0.0 && end == 0.0 {
            end = 1.0;
        }
        stairs_length += length;
        for pair in boundaries.windows(2) {
            let target = if end < first { end + 1.0 } else { end };
            let coordinates = std::array::from_fn(|j| {
                coordinate_curve.coord_at_u(project(pair[j] * target + (1.0 - pair[j]) * first))
            });
            cells.push(platform_stair_cell(WallPlatformStairCellSource {
                prepared_positions,
                coordinates,
                maximum_y: maximum,
                constrained_extra: extra,
                terrain,
                index_offset: (cells.len() as u32).wrapping_mul(8),
            })?);
        }
    }
    Ok(WallPlatformStairGeometry {
        cells,
        stairs_length,
    })
}

/// Original final pass iterates every cached materialized stair segment, even
/// when its <.5 length skipped geometry or later render Curve2 creation fails.
pub fn platform_stair_holes(
    raw_curve: &Curve2,
    maximum_y: f32,
    segments: &[MaterializedWallSegment],
) -> Result<Vec<crate::resources::walls::wall_holes::geometry::RawWallHole>, WallPlatformStairError>
{
    if !maximum_y.is_finite() || !raw_curve.length.is_finite() || raw_curve.length <= 0.0 {
        return Err(WallPlatformStairError::InvalidInput);
    }
    let mut output = Vec::new();
    for segment in segments {
        let first = segment
            .positions
            .first()
            .ok_or(WallPlatformStairError::InvalidInput)?;
        let last = segment
            .positions
            .last()
            .ok_or(WallPlatformStairError::InvalidInput)?;
        let a = wall_coord_from_world(raw_curve, [first[0], maximum_y, first[2]])
            .map_err(WallPlatformStairError::Placement)?[0];
        let mut b = wall_coord_from_world(raw_curve, [last[0], maximum_y, last[2]])
            .map_err(WallPlatformStairError::Placement)?[0];
        if b < a {
            b += raw_curve.length;
        }
        let u = (b * 0.5 + a * 0.5) / raw_curve.length;
        let r = u % 1.0;
        let r = if r < 0.0 { r + 1.0 } else { r };
        let length_xz: f32 = segment
            .positions
            .windows(2)
            .map(|p| Vec2::new(p[0][0] - p[1][0], p[0][2] - p[1][2]).length())
            .sum();
        output.push(crate::resources::walls::wall_holes::geometry::RawWallHole {
            center: [raw_curve.length * r, maximum_y],
            dimensions: [length_xz + 0.4, f32::from_bits(0x3f9ef9db)],
            kind: 3,
        });
    }
    Ok(output)
}
