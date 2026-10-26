//! Game policy at the B construct_walls -> WallConstructor::from_curve call.
//! Prepared path ownership is explicit: initial terrain mapping and subsequent
//! segmentation are different original PublicWallState fields.

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::too_many_arguments)]
use crate::resources::roofs::geometry::surface::Curve2;
use crate::resources::roofs::geometry::surface::CurveError;
use crate::resources::walls::publication_fields::WallId;
use crate::resources::walls::publication_fields::WallShapeSnapshot;
use ::utils::scalar::{clamp01, max_f32};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConstructionRoof {
    Absent,
    Pitched,
    Flat { has_rim: bool },
}

/// Original Public +295, +299 and resolved Roof predicates. This policy does
/// not create a default ClutterVersion for a raw wall whose field is unknown.
pub fn construction_kind(clutter_version: u8, wall_style: u8, roof: ConstructionRoof) -> u8 {
    match roof {
        ConstructionRoof::Absent => u8::from(clutter_version == 0) * 2,
        ConstructionRoof::Flat { has_rim: true } => 1 + 2 * u8::from(wall_style >= 2),
        _ => 0,
    }
}

/// Public +48 is the initial mapped Curve3 point count, not the subsequently
/// prepared Curve3 +98 point count, nor an authored random seed.
pub fn source_seed(wall: WallId, terrain_mapped_point_count: u64) -> u64 {
    wall.0
        .wrapping_mul(0x4b28021)
        .wrapping_add(terrain_mapped_point_count)
}

/// Original BrickSourceId::from(WallId). This identifies the brick owner and
/// is separate from material/style selection.
pub fn brick_source_id(wall: WallId) -> u32 {
    let low = wall.0 as u32;
    let reserved = low.wrapping_sub(100_000);
    if matches!(reserved, 0 | 1 | 2 | 3 | 11) {
        0x8000_0000 | reserved
    } else {
        low
    }
}

/// Original plain/top-tag2 perturbation policy uses raw Curve2 +28c length.
pub fn row_end_perturbation(top_tag: u8, clutter_version: u8, original_curve_length: f32) -> f32 {
    if top_tag != 2 {
        return 0.0;
    }
    if clutter_version != 0 {
        return f32::from_bits(0x3d3c6a7f);
    }
    let t = clamp01(
        (original_curve_length + f32::from_bits(0xbf30a3d7)) / f32::from_bits(0x3f30a3d7),
    );
    (3.0 - (t + t)) * (t * t)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CircleConstructionOptions {
    pub width_padding: Option<f32>,
    pub thickness_scale: Option<f32>,
}

pub fn circle_construction_options(shape: &WallShapeSnapshot) -> CircleConstructionOptions {
    if shape.tag() != 0 {
        return CircleConstructionOptions {
            width_padding: None,
            thickness_scale: None,
        };
    }
    let radius = f32::from_le_bytes(shape.bytes()[12..16].try_into().unwrap());
    let lower = f32::from_bits(0x3f666667);
    let width = clamp01((radius - lower) / (f32::from_bits(0x40666666) - lower));
    let thickness = clamp01((radius - lower) / (3.0 - lower));
    CircleConstructionOptions {
        width_padding: Some((1.0 - width) * f32::from_bits(0x3d75c28f)),
        thickness_scale: Some((1.0 - thickness) * f32::from_bits(0x3f666666) + thickness),
    }
}

#[derive(Clone, Debug)]
pub struct PreparedConstructionArrays {
    pub curve: Curve2,
    pub arch_heights: Vec<f32>,
    pub second_heights: Vec<f32>,
    pub third_heights: Vec<f32>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum ConstructionSourceError {
    ArrayCount,
    Curve(CurveError),
    NotPlainConstruction,
    MissingMaximumY,
    RectangleRequired,
    Gable(crate::resources::walls::gable_heights::GableError),
    SeparateSpecialConstruction,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConstructionTopPolicy {
    Plain,
    Gable {
        height: f32,
        intervals: [[f32; 2]; 2],
        tag: u8,
    },
}
#[derive(Clone, Copy, Debug)]
pub struct WallConstructionPolicy {
    pub kind: u8,
    pub seed: u64,
    pub brick_source_id: u32,
    pub base: f32,
    pub row_end_perturbation: f32,
    pub options: CircleConstructionOptions,
    pub wall_float: f32,
    pub top: ConstructionTopPolicy,
    /// Original constructor modifies the cloned input, not Public arch cache.
    pub lower_flat_no_rim_arches: bool,
}
impl WallConstructionPolicy {
    pub fn constructor_arch_heights(&self, published_arch_heights: &[f32]) -> Vec<f32> {
        published_arch_heights
            .iter()
            .map(|&h| {
                if self.lower_flat_no_rim_arches {
                    max_f32(h + f32::from_bits(0xbf23d70a), 0.5)
                } else {
                    h
                }
            })
            .collect()
    }
}
/// General actual Roof/style/cache caller policy. `cached_maximum_y` is the
/// resolved PublicWallState max-y cache; only the gable branch reads it. Plain
/// top-tag2 fields stay inactive instead of being represented as author values.
pub fn construction_policy(
    wall: WallId,
    terrain_mapped_point_count: u64,
    original_curve_length: f32,
    clutter_version: u8,
    wall_style: u8,
    shape: &WallShapeSnapshot,
    minimum_position_y: f32,
    constrained_extra: f32,
    cached_maximum_y: Option<f32>,
    roof: Option<&[u8; 88]>,
    observed_special_flag: u8,
) -> Result<WallConstructionPolicy, ConstructionSourceError> {
    if observed_special_flag != 0 {
        return Err(ConstructionSourceError::SeparateSpecialConstruction);
    }
    let read =
        |roof: &[u8; 88], offset| f32::from_le_bytes(roof[offset..offset + 4].try_into().unwrap());
    let roof_kind = roof.map_or(ConstructionRoof::Absent, |roof| {
        if crate::resources::roofs::roof_shape::ty_observed(read(roof, 0x30)) {
            ConstructionRoof::Pitched
        } else {
            ConstructionRoof::Flat {
                has_rim: roof[0x3d] == 0,
            }
        }
    });
    let top = if let Some(roof) = roof.filter(|roof| {
        crate::resources::roofs::roof_shape::is_gable_observed(read(roof, 0x30), read(roof, 0x34))
    }) {
        if shape.tag() != 1 {
            return Err(ConstructionSourceError::RectangleRequired);
        }
        let height = cached_maximum_y.ok_or(ConstructionSourceError::MissingMaximumY)?;
        let width = f32::from_le_bytes(shape.bytes()[20..24].try_into().unwrap());
        let length = f32::from_le_bytes(shape.bytes()[24..28].try_into().unwrap());
        let first = roof[0x3c] as usize;
        let second = first | 2;
        let point = |index| {
            crate::resources::walls::gable_heights::rectangle_point_u(width, length, index)
                .map_err(ConstructionSourceError::Gable)
        };
        ConstructionTopPolicy::Gable {
            height,
            intervals: [
                [point(first)?, point(first + 1)?],
                [point(second)?, point(second + 1)?],
            ],
            tag: u8::from(wall_style >= 2),
        }
    } else {
        ConstructionTopPolicy::Plain
    };
    let tag = match top {
        ConstructionTopPolicy::Plain => 2,
        ConstructionTopPolicy::Gable { tag, .. } => tag,
    };
    Ok(WallConstructionPolicy {
        kind: construction_kind(clutter_version, wall_style, roof_kind),
        seed: source_seed(wall, terrain_mapped_point_count),
        brick_source_id: brick_source_id(wall),
        base: minimum_position_y,
        row_end_perturbation: row_end_perturbation(tag, clutter_version, original_curve_length),
        options: circle_construction_options(shape),
        wall_float: constrained_extra,
        top,
        lower_flat_no_rim_arches: roof
            .is_some_and(|roof| read(roof, 0x30) <= f32::from_bits(0x3dcccccd) && roof[0x3d] != 0),
    })
}

/// The original conversion reads the *prepared* Public Curve3 +90 points,
/// drops Y, and rebuilds Curve2 with reject_duplicates=false. Caller must supply
/// actual prepared-path output, not raw control points or a guessed resample.
pub fn prepared_construction_arrays(
    prepared_positions: &[[f32; 3]],
    arch_heights: &[f32],
    third_heights: &[f32],
) -> Result<PreparedConstructionArrays, ConstructionSourceError> {
    if arch_heights.len() != prepared_positions.len()
        || third_heights.len() != prepared_positions.len()
    {
        return Err(ConstructionSourceError::ArrayCount);
    }
    Ok(PreparedConstructionArrays {
        curve: Curve2::try_new(
            prepared_positions.iter().map(|p| [p[0], p[2]]).collect(),
            false,
        )
        .map_err(ConstructionSourceError::Curve)?,
        arch_heights: arch_heights.to_vec(),
        second_heights: prepared_positions.iter().map(|p| p[1]).collect(),
        third_heights: third_heights.to_vec(),
    })
}

/// Original init cache explicitly alloc_zeroed(Public segmentVec +d0 * 4).
/// The count must come from the prepared segmentation owner, independently of
/// raw/mapped point counts. Later writers may change this array.
pub fn initialize_segment_height_array(prepared_segment_count: usize) -> Vec<f32> {
    vec![0.0; prepared_segment_count]
}

#[derive(Clone, Copy, Debug)]
pub struct PlainConstructionPolicy {
    pub seed: u64,
    pub brick_source_id: u32,
    pub base: f32,
    pub row_end_perturbation: f32,
    pub options: CircleConstructionOptions,
    pub wall_float: f32,
}
/// base and wall_float are actual publication metrics +274/+284. The inactive
/// tag2 five-float payload and false +A8 flag have no invented field defaults.
pub fn plain_construction_policy(
    wall: WallId,
    terrain_mapped_point_count: u64,
    original_curve_length: f32,
    clutter_version: u8,
    wall_style: u8,
    roof: ConstructionRoof,
    shape: &WallShapeSnapshot,
    minimum_position_y: f32,
    constrained_extra: f32,
) -> Result<PlainConstructionPolicy, ConstructionSourceError> {
    if construction_kind(clutter_version, wall_style, roof) != 2 {
        return Err(ConstructionSourceError::NotPlainConstruction);
    }
    Ok(PlainConstructionPolicy {
        seed: source_seed(wall, terrain_mapped_point_count),
        brick_source_id: brick_source_id(wall),
        base: minimum_position_y,
        row_end_perturbation: row_end_perturbation(2, clutter_version, original_curve_length),
        options: circle_construction_options(shape),
        wall_float: constrained_extra,
    })
}
