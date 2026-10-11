//! Entire B regular-wall gable arch-height consumer after public-wall/roof lookup.
//! Rectangle perimeter and the rebuilt XZ CurveU are game-owned algorithms;
//! standard Vec, glam inputs, and powf are reused. No ECS/container ABI is ported.
use crate::{
    roof_parameters::RoofParameterBlock,
    surface::{Curve2, CurveError},
};

#[derive(Clone, Debug, PartialEq)]
pub enum GableError {
    Curve(CurveError),
    InvalidRectangle,
    UnsupportedDirection,
    HeightCount,
}
#[derive(Clone, Debug)]
pub struct GableArchHeights {
    pub heights: Vec<f32>,
    pub maximum_y: f32,
    pub curve_u: Vec<f32>,
    pub closing_endpoint_offset: Option<f32>,
}
fn f(roof: &[u8; 88], offset: usize) -> f32 {
    f32::from_le_bytes(roof[offset..offset + 4].try_into().unwrap())
}
/// Original Rectangle2d::get_point_u jump cases; addition order is intentional.
pub fn rectangle_point_u(width: f32, length: f32, index: usize) -> Result<f32, GableError> {
    let u = match index {
        0 => 0.,
        1 => length / ((width + length) + (width + length)),
        2 => {
            let sum = length + width;
            sum / (sum + sum)
        }
        3 => (length + length + width) / ((length + width) + (length + width)),
        4 => 1.,
        _ => return Err(GableError::InvalidRectangle),
    };
    if !u.is_finite() || !(0.0..=1.0).contains(&u) {
        return Err(GableError::InvalidRectangle);
    }
    Ok(u)
}
fn max_source(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}
fn min_source(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

/// Source_length is original source Curve2 length at public state+0x28c;
/// curve_u here is newly computed from published outline projected to XZ.
/// These are distinct quantities even on terrain-following walls.
pub fn apply_gable_arch_heights(
    outline: &[[f32; 3]],
    base_heights: &[f32],
    base_maximum: f32,
    source_length: f32,
    wall_halftimbered: bool,
    roof: &[u8; 88],
) -> Result<GableArchHeights, GableError> {
    if outline.len() != base_heights.len() {
        return Err(GableError::HeightCount);
    }
    let mut result = GableArchHeights {
        heights: base_heights.to_vec(),
        maximum_y: base_maximum,
        curve_u: Vec::new(),
        closing_endpoint_offset: None,
    };
    if u32::from_le_bytes(roof[8..12].try_into().unwrap()) != 1
        || !crate::roof_state::is_gable_observed(f(roof, 0x30), f(roof, 0x34))
    {
        return Ok(result);
    }
    let direction = roof[0x3c] as usize;
    if direction > 1 {
        return Err(GableError::UnsupportedDirection);
    }
    let width = f(roof, 0x1c);
    let length = f(roof, 0x20);
    let endpoints = [
        rectangle_point_u(width, length, direction)?,
        rectangle_point_u(width, length, direction + 1)?,
        rectangle_point_u(width, length, direction + 2)?,
        rectangle_point_u(width, length, direction + 3)?,
    ];
    let half_span = if direction == 0 { length } else { width } * 0.5;
    let profile = f(roof, 0x2c);
    let exponent = profile * 1.5 + (1.0 - profile) * 0.5;
    let eave = if wall_halftimbered {
        0.0
    } else {
        RoofParameterBlock::from_roof(roof).eave_length_ws() * ((profile - 0.5) / (1.0 - 0.5))
    };
    let scale = ((half_span + eave) + f32::from_bits(0x3e8f5c29)) / half_span;
    let margin = (0.5 / source_length) * f32::from_bits(0x3f733333);
    let curve = Curve2::try_new(outline.iter().map(|p| [p[0], p[2]]).collect(), false)
        .map_err(GableError::Curve)?;
    let roof_height = crate::surface::height(roof);
    let baseline = if wall_halftimbered {
        0.0
    } else {
        f32::from_bits(0x3eeb851f)
    };
    for (index, &u) in curve.points_u.iter().enumerate().take(result.heights.len()) {
        for [start, end] in [[endpoints[0], endpoints[1]], [endpoints[2], endpoints[3]]] {
            if u >= start - margin && u < end + margin {
                let t = min_source(1.0, max_source(0.0, (u - start) / (end - start)));
                let centered = t + (-0.5);
                let magnitude = f32::from_bits(centered.to_bits() & 0x7fffffff);
                let power = (1.0 - (magnitude + magnitude) / scale).powf(exponent);
                let offset = (roof_height + baseline) * power + (1.0 - power) * baseline;
                result.heights[index] += offset;
                if !wall_halftimbered && u == 1.0 {
                    result.closing_endpoint_offset = Some(offset);
                }
            }
        }
    }
    if let Some(offset) = result.closing_endpoint_offset {
        let first = result.heights.first_mut().ok_or(GableError::HeightCount)?;
        *first += offset;
    }
    result.maximum_y = (base_maximum + roof_height) + f32::from_bits(0x3eeb851f);
    result.curve_u = curve.points_u;
    Ok(result)
}
