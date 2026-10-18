//! B wall destination subtype consumer. Actual roof and published wall caches
//! are input; roof UV destinations and trigger construction are separate work.

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::neg_cmp_op_on_partial_ord)]
use crate::resources::roofs::geometry::surface::Curve2;
use crate::resources::walls::decorator::placement::CollisionBoundingBox;
use crate::resources::walls::decorator::placement::PlacementError;
use crate::resources::walls::decorator::placement::wall_coord_to_world;
use crate::resources::walls::publication_fields::WallShapeSnapshot;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WallWindowKind {
    Cottage,
    Gothic,
}
#[derive(Clone, Copy, Debug)]
pub struct WallDecoratorSubtypeContext<'a> {
    pub source_curve: &'a Curve2,
    pub mapped_outline: &'a [[f32; 3]],
    pub arch_heights: &'a [f32],
    pub wall_shape: &'a WallShapeSnapshot,
    pub roof: Option<&'a [u8; 88]>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum SubtypeError {
    Placement(PlacementError),
    InvalidPublishedOutline,
    InvalidHeights,
    InvalidRoofRectangle,
    InvalidRoofDirection,
    InvalidCollision,
    UnsupportedOpeningSubtype,
}

/// Original producer accept sets: cottage 0/1, gothic 6/7, doors 4/9.
/// A cottage corner (3) has its own unresolved visual/opening consumer.
pub fn wall_opening_family(
    subtype: u8,
) -> Result<crate::resources::walls::wall_holes::opening_rules::OpeningFamily, SubtypeError> {
    use crate::resources::walls::wall_holes::opening_rules::OpeningFamily;
    match subtype {
        0 | 1 => Ok(OpeningFamily::CottageWindow),
        6 | 7 => Ok(OpeningFamily::GothicWindow),
        4 => Ok(OpeningFamily::CottageBalconyDoor),
        9 => Ok(OpeningFamily::GothicBalconyDoor),
        _ => Err(SubtypeError::UnsupportedOpeningSubtype),
    }
}

pub fn b_wall_subtype_collision_asset_name(
    subtype: u8,
    rank: u8,
) -> Result<&'static str, SubtypeError> {
    let family = wall_opening_family(subtype)?;
    crate::resources::walls::decorator::placement::wall_decorator_key(family, rank)
        .map_err(SubtypeError::Placement)?;
    if subtype == 1 {
        Ok([
            "decorators/window_cottage_1x1_halfdormer_collision",
            "decorators/window_cottage_2x1_halfdormer_collision",
            "decorators/window_cottage_3x1_halfdormer_collision",
        ][usize::from(rank - 1)])
    } else {
        crate::resources::walls::decorator::placement::b_wall_collision_asset_name(family, rank)
            .map_err(SubtypeError::Placement)
    }
}
fn read(roof: &[u8; 88], offset: usize) -> f32 {
    f32::from_le_bytes(roof[offset..offset + 4].try_into().unwrap())
}
fn max(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}
fn min(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}
/// Original game-owned thresholded XZ search. Curve3 Y values do not enter
/// nearest-segment selection. Local fraction is not Curve2 normalized U.
pub fn nearest_segment(
    points: &[[f32; 3]],
    position: [f32; 3],
) -> Result<(usize, f32), SubtypeError> {
    if points.len() < 2 || points.iter().flatten().any(|v| !v.is_finite()) {
        return Err(SubtypeError::InvalidPublishedOutline);
    }
    let mut best_distance = f32::MAX;
    let mut best = (0, 0.0);
    for (i, pair) in points.windows(2).enumerate() {
        let a = pair[0];
        let b = pair[1];
        let dx = b[0] - a[0];
        let dz = b[2] - a[2];
        let numerator = (position[2] - a[2]) * dz + (position[0] - a[0]) * dx;
        let denominator = max(dz * dz + dx * dx, f32::from_bits(0x322bcc77));
        let t = min(1., max(0., numerator / denominator));
        let x = t * b[0] + (1. - t) * a[0];
        let z = t * b[2] + (1. - t) * a[2];
        let ex = position[0] - x;
        let ez = position[2] - z;
        let distance = (ez * ez + ex * ex).sqrt();
        if distance < best_distance {
            best_distance = distance;
            best = (i, t);
        }
    }
    Ok(best)
}
/// Rectangle2d::get_closest_edge_id preserves first strictly closer edge and
/// the original 1000-unit initial distance. Shape fields are from actual Roof.
fn closest_roof_edge(roof: &[u8; 88], position: [f32; 3]) -> Result<usize, SubtypeError> {
    if u32::from_le_bytes(roof[8..12].try_into().unwrap()) != 1 {
        return Err(SubtypeError::InvalidRoofRectangle);
    }
    let [ux, uz, cx, cz, w, l] = [
        read(roof, 12),
        read(roof, 16),
        read(roof, 20),
        read(roof, 24),
        read(roof, 28),
        read(roof, 32),
    ];
    if [ux, uz, cx, cz, w, l].iter().any(|v| !v.is_finite()) || !(w * l > 0.) {
        return Err(SubtypeError::InvalidRoofRectangle);
    }
    let local = [
        [-w * 0.5, -l * 0.5],
        [-w * 0.5, l * 0.5],
        [w * 0.5, l * 0.5],
        [w * 0.5, -l * 0.5],
    ];
    let points = local.map(|[x, z]| [(-uz * z + ux * x) + cx, 0., (ux * z + uz * x) + cz]);
    let mut best_distance = 1000.;
    let mut best = 0;
    for i in 0..4 {
        let a = points[i];
        let b = points[(i + 1) % 4];
        let dx = b[0] - a[0];
        let dz = b[2] - a[2];
        let numerator = (position[2] - a[2]) * dz + (position[0] - a[0]) * dx;
        let t = min(
            1.,
            max(
                0.,
                numerator / max(dz * dz + dx * dx, f32::from_bits(0x1e3ce508)),
            ),
        );
        let x = t * b[0] + (1. - t) * a[0];
        let z = t * b[2] + (1. - t) * a[2];
        let ex = position[0] - x;
        let ez = position[2] - z;
        let distance = (ez * ez + ex * ex).sqrt();
        if distance < best_distance {
            best_distance = distance;
            best = i;
        }
    }
    Ok(best)
}

/// Complete finite B half-dormer consumer after actual wall/roof/library lookup.
/// The collision must be the actual *plain window* asset for the current rank.
pub fn is_half_dormer(
    coord: [f32; 2],
    context: WallDecoratorSubtypeContext<'_>,
    collision: CollisionBoundingBox,
) -> Result<bool, SubtypeError> {
    if context.wall_shape.tag() > 1 {
        return Ok(false);
    }
    if let Some(roof) = context.roof {
        let height = read(roof, 0x30);
        if height <= f32::from_bits(0x3dcccccd) || height.is_nan() {
            return Ok(false);
        }
        if crate::resources::roofs::roof_shape::is_gable_observed(height, read(roof, 0x34)) {
            let direction = usize::from(roof[0x3c]);
            if direction > 1 {
                return Err(SubtypeError::InvalidRoofDirection);
            }
            let world = wall_coord_to_world(context.source_curve, coord)
                .map_err(SubtypeError::Placement)?;
            let edge = closest_roof_edge(roof, world)?;
            if edge == direction || edge == (direction | 2) {
                return Ok(false);
            }
        }
    }
    let world =
        wall_coord_to_world(context.source_curve, coord).map_err(SubtypeError::Placement)?;
    let (segment, t) = nearest_segment(context.mapped_outline, world)?;
    if context.arch_heights.len() != context.mapped_outline.len()
        || context.arch_heights.iter().any(|v| !v.is_finite())
    {
        return Err(SubtypeError::InvalidHeights);
    }
    let wall_height = t * (context.arch_heights[segment + 1] - context.arch_heights[segment])
        + context.arch_heights[segment];
    let [minimum, maximum] = collision.vertical_bounds;
    if !minimum.is_finite() || !maximum.is_finite() {
        return Err(SubtypeError::InvalidCollision);
    }
    let middle = (minimum + maximum) * 0.5;
    let top = ((maximum - minimum) + middle) * 0.5 + world[1];
    Ok(top > wall_height)
}

/// Original wall destination selection: corner, then door, then half/plain.
/// `door_attachment` is the meaningful original WallAttachment tag predicate;
/// it is not inferred from collision dimensions or Y position.
pub fn derive_wall_subtype(
    kind: WallWindowKind,
    coord: [f32; 2],
    door_attachment: bool,
    corner_attached: bool,
    context: WallDecoratorSubtypeContext<'_>,
    plain_collision: CollisionBoundingBox,
) -> Result<u8, SubtypeError> {
    if let Some(subtype) = wall_attachment_subtype(kind, door_attachment, corner_attached) {
        return Ok(subtype);
    }
    let half = is_half_dormer(coord, context, plain_collision)?;
    Ok(match kind {
        WallWindowKind::Cottage => u8::from(half),
        WallWindowKind::Gothic => 6 + u8::from(half),
    })
}

pub fn wall_attachment_subtype(
    kind: WallWindowKind,
    door_attachment: bool,
    corner_attached: bool,
) -> Option<u8> {
    if corner_attached {
        return Some(match kind {
            WallWindowKind::Cottage => 3,
            WallWindowKind::Gothic => 6,
        });
    }
    if door_attachment {
        return Some(match kind {
            WallWindowKind::Cottage => 4,
            WallWindowKind::Gothic => 9,
        });
    }
    None
}
