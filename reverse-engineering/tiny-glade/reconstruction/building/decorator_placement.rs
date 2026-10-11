//! B wall-space placement and collision metadata. Asset decoding and standard
//! containers stay in their upstream libraries. Roof/corner/dormer attachment
//! selection is deliberately a separate, unresolved consumer.
use crate::{
    building_opening_rules::{DecoratorCollisionMetrics, DecoratorOpeningInput, OpeningFamily},
    building_wall_state::WallShapeSnapshot,
    surface::Curve2,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DecoratorLibKey {
    /// Original meaningful byte order: subtype first, rank second.
    pub subtype: u8,
    pub rank: u8,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CollisionBoundingBox {
    pub center_xz: [f32; 2],
    pub extent_xz: [f32; 2],
    pub vertical_bounds: [f32; 2],
}
impl CollisionBoundingBox {
    pub fn metrics(self) -> DecoratorCollisionMetrics {
        DecoratorCollisionMetrics {
            extent_xz: self.extent_xz,
            vertical_bounds: self.vertical_bounds,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallDecoratorRequest {
    pub family: OpeningFamily,
    pub rank: u8,
    /// Actual world position, not an already projected wall-space center.
    pub world_position: [f32; 3],
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallDecoratorPlacement {
    pub key: DecoratorLibKey,
    pub opening: DecoratorOpeningInput,
    pub projected_world_position: [f32; 3],
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlacementError {
    InvalidRank,
    MissingVertices,
    NonfiniteVertices,
    NonfinitePosition,
    InvalidCurve,
    UnsupportedWallShape,
    MissingCollisionAsset,
}

/// The four plain wall producer families use subtype 0/6/4/9. Half dormers and
/// corner attachment are not selected by this API. The family is an explicit
/// placement intent; it is not a recovered complete DerivedDecoratorSubtypes.
pub fn wall_decorator_key(
    family: OpeningFamily,
    rank: u8,
) -> Result<DecoratorLibKey, PlacementError> {
    if !(1..=3).contains(&rank) {
        return Err(PlacementError::InvalidRank);
    }
    let subtype = match family {
        OpeningFamily::CottageWindow => 0,
        OpeningFamily::GothicWindow => 6,
        OpeningFamily::CottageBalconyDoor => 4,
        OpeningFamily::GothicBalconyDoor => 9,
    };
    Ok(DecoratorLibKey { subtype, rank })
}

/// Asset names selected by B load_decorators for the four plain wall subtypes.
/// The original gothic availability gate belongs to the caller's feature policy.
pub fn b_wall_collision_asset_name(
    family: OpeningFamily,
    rank: u8,
) -> Result<&'static str, PlacementError> {
    wall_decorator_key(family, rank)?;
    let names = match family {
        OpeningFamily::CottageWindow => [
            "decorators/window_cottage_1x1_collision",
            "decorators/window_cottage_2x1_collision",
            "decorators/window_cottage_3x1_collision",
        ],
        OpeningFamily::GothicWindow => [
            "decorators/window_gothic_1x1_collision",
            "decorators/window_gothic_2x1_collision",
            "decorators/window_gothic_3x1_collision",
        ],
        OpeningFamily::CottageBalconyDoor => [
            "decorators/balcony_door_rank1_collision",
            "decorators/balcony_door_rank2_collision",
            "decorators/balcony_door_rank3_collision",
        ],
        OpeningFamily::GothicBalconyDoor => [
            "decorators/balcony_door_gothic_rank1_collision",
            "decorators/balcony_door_gothic_rank2_collision",
            "decorators/balcony_door_gothic_rank3_collision",
        ],
    };
    Ok(names[usize::from(rank - 1)])
}

/// B Mesh::aabb_bounds and collision_bbx over a valid Vertex_Position float3
/// attribute. Original scalar/SIMD reductions preserve the previous endpoint
/// on equal finite values, including signed-zero bits.
pub fn mesh_collision_bbx(vertices: &[[f32; 3]]) -> Result<CollisionBoundingBox, PlacementError> {
    let Some(&first) = vertices.first() else {
        return Err(PlacementError::MissingVertices);
    };
    if vertices.iter().flatten().any(|v| !v.is_finite()) {
        return Err(PlacementError::NonfiniteVertices);
    }
    let mut minimum = first;
    let mut maximum = first;
    for point in &vertices[1..] {
        for axis in 0..3 {
            if point[axis] < minimum[axis] {
                minimum[axis] = point[axis];
            }
            if point[axis] > maximum[axis] {
                maximum[axis] = point[axis];
            }
        }
    }
    let extent_xz = [maximum[0] - minimum[0], maximum[2] - minimum[2]];
    if extent_xz.iter().any(|v| !v.is_finite()) {
        return Err(PlacementError::NonfiniteVertices);
    }
    Ok(CollisionBoundingBox {
        center_xz: [
            (minimum[0] + maximum[0]) * 0.5,
            (minimum[2] + maximum[2]) * 0.5,
        ],
        extent_xz,
        vertical_bounds: [minimum[1], maximum[1]],
    })
}

fn validate_curve(curve: &Curve2) -> Result<(), PlacementError> {
    if curve.points.len() < 2
        || curve.points_u.len() != curve.points.len()
        || !(curve.length > 0.0)
        || !curve.length.is_finite()
        || curve
            .points
            .iter()
            .flatten()
            .chain(curve.points_u.iter())
            .any(|v| !v.is_finite())
    {
        return Err(PlacementError::InvalidCurve);
    }
    Ok(())
}

/// B private Curve::get_approx_u_from_pos: nearest projected segment, first
/// winner on equal distance; length denominator is floored at original 1e-8.
pub fn wall_coord_from_world(
    curve: &Curve2,
    position: [f32; 3],
) -> Result<[f32; 2], PlacementError> {
    validate_curve(curve)?;
    if position.iter().any(|v| !v.is_finite()) {
        return Err(PlacementError::NonfinitePosition);
    }
    let mut best_distance = f32::MAX;
    let mut best_segment = 0;
    let mut best_fraction = 0.0;
    for (segment, endpoints) in curve.points.windows(2).enumerate() {
        let [a, b] = [endpoints[0], endpoints[1]];
        let delta = [b[0] - a[0], b[1] - a[1]];
        let relative = [position[0] - a[0], position[2] - a[1]];
        let dot = relative[0] * delta[0] + relative[1] * delta[1];
        let denominator = delta[0] * delta[0] + delta[1] * delta[1];
        let denominator = if denominator > f32::from_bits(0x322b_cc77) {
            denominator
        } else {
            f32::from_bits(0x322b_cc77)
        };
        let t = dot / denominator;
        let t = if t > 0.0 { t } else { 0.0 };
        let t = if t < 1.0 { t } else { 1.0 };
        let projected = [b[0] * t + a[0] * (1.0 - t), b[1] * t + a[1] * (1.0 - t)];
        let dx = position[0] - projected[0];
        let dz = position[2] - projected[1];
        let distance = (dx * dx + dz * dz).sqrt();
        if distance < best_distance {
            best_distance = distance;
            best_segment = segment;
            best_fraction = t;
        }
    }
    let normalized = (1.0 - best_fraction) * curve.points_u[best_segment]
        + best_fraction * curve.points_u[best_segment + 1];
    let normalized = if normalized > 0.0 { normalized } else { 0.0 };
    let normalized = if normalized < 1.0 { normalized } else { 1.0 };
    Ok([normalized * curve.length, position[1]])
}

/// B WallSpaceCoord::to_world_space. In-range placement uses the original
/// Curve2 parameter search/interpolation. Out-of-range endpoint policy is
/// retained rather than replacing the observed negative-end arithmetic.
pub fn wall_coord_to_world(
    curve: &Curve2,
    coordinate: [f32; 2],
) -> Result<[f32; 3], PlacementError> {
    validate_curve(curve)?;
    if coordinate.iter().any(|v| !v.is_finite()) {
        return Err(PlacementError::NonfinitePosition);
    }
    let u = coordinate[0] / curve.length;
    if !u.is_finite() {
        return Err(PlacementError::NonfinitePosition);
    }
    let (i, t) = curve.coord_at_u(u.clamp(0.0, 1.0));
    let [a, b] = [curve.points[i], curve.points[i + 1]];
    let mut output = [
        a[0] * (1.0 - t) + b[0] * t,
        coordinate[1],
        a[1] * (1.0 - t) + b[1] * t,
    ];
    if coordinate[0] < 0.0 || coordinate[0] > curve.length {
        let dx = b[0] - a[0];
        let dz = b[1] - a[1];
        let reciprocal = 1.0 / (dx * dx + dz * dz).sqrt();
        let tangent = [reciprocal * dx, reciprocal * dz];
        let reciprocal2 = 1.0 / (tangent[0] * tangent[0] + tangent[1] * tangent[1]).sqrt();
        let tangent = [tangent[0] * reciprocal2, tangent[1] * reciprocal2];
        if coordinate[0] > curve.length {
            let excess = coordinate[0] - curve.length;
            output[0] += excess * tangent[0];
            output[1] += excess * (0.0 * reciprocal2);
            output[2] += excess * tangent[1];
        } else {
            output[0] -= coordinate[0] * tangent[0];
            output[1] -= coordinate[0] * (0.0 * reciprocal2);
            output[2] -= coordinate[0] * tangent[1];
        }
    }
    Ok(output)
}

pub fn place_wall_decorator(
    request: WallDecoratorRequest,
    curve: &Curve2,
    shape: &WallShapeSnapshot,
    collision: CollisionBoundingBox,
) -> Result<WallDecoratorPlacement, PlacementError> {
    if shape.tag() > 1 {
        return Err(PlacementError::UnsupportedWallShape);
    }
    let key = wall_decorator_key(request.family, request.rank)?;
    let center = wall_coord_from_world(curve, request.world_position)?;
    Ok(WallDecoratorPlacement {
        key,
        opening: DecoratorOpeningInput {
            family: request.family,
            size_variant: request.rank,
            center,
            collision: collision.metrics(),
        },
        projected_world_position: wall_coord_to_world(curve, center)?,
    })
}
