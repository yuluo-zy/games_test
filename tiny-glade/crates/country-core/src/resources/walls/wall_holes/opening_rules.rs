//! Decorator → wall-hole production after asset collision and wall placement
//! resolution. Geometry assets, wall-space projection and visual/GPU builders
//! remain separate consumers; this module never accepts precomputed holes.
use crate::resources::walls::publication_fields::WallShapeSnapshot;
use crate::resources::walls::wall_holes::geometry::RawWallHole;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpeningFamily {
    CottageWindow,
    GothicWindow,
    CottageBalconyDoor,
    GothicBalconyDoor,
}

/// Resolved DecoratorCollision fields, rather than fabricated mesh dimensions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DecoratorCollisionMetrics {
    pub extent_xz: [f32; 2],
    pub vertical_bounds: [f32; 2],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DecoratorOpeningInput {
    pub family: OpeningFamily,
    /// Observed original derived size byte: 1, 2 or 3.
    pub size_variant: u8,
    /// Authoritative DecoratorInfo::dst_as_wall coordinate, in wall space.
    pub center: [f32; 2],
    pub collision: DecoratorCollisionMetrics,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpeningRuleError {
    InvalidSizeVariant,
}

/// Full B raw-hole contribution of the four resolved wall decorator families.
/// The producer retains three separate Gothic rectangles and does not merge
/// them. Periodic duplicates and sorting belong to WallHoleStorage::finalize.
pub fn generate_wall_opening(
    input: DecoratorOpeningInput,
    host: &WallShapeSnapshot,
) -> Result<Vec<RawWallHole>, OpeningRuleError> {
    let size = input
        .size_variant
        .checked_sub(1)
        .filter(|&s| s < 3)
        .ok_or(OpeningRuleError::InvalidSizeVariant)? as usize;
    match input.family {
        OpeningFamily::CottageWindow | OpeningFamily::CottageBalconyDoor => {
            let mut width = input.collision.extent_xz[0];
            if input.family == OpeningFamily::CottageWindow && host.tag() == 0 {
                // Original ADDSS with signed constants; no independent clamp.
                width += [-0.05_f32, -0.1, -0.15][size];
            }
            Ok(vec![RawWallHole {
                center: input.center,
                dimensions: [
                    width,
                    input.collision.vertical_bounds[1] - input.collision.vertical_bounds[0],
                ],
                kind: 0,
            }])
        }
        OpeningFamily::GothicWindow | OpeningFamily::GothicBalconyDoor => {
            // Each row is [center-y offset, width, height]. Values are original
            // single-precision literals; variants are deliberately independent.
            let rows = match input.family {
                OpeningFamily::GothicWindow => [
                    [
                        [-0.2875, 0.6992, 1.334],
                        [0.5405, 0.5888, 0.4945],
                        [0.9085, 0.35994998, 0.345],
                    ],
                    [
                        [f32::from_bits(0xbdc8_b43a), 1.168, 1.7],
                        [1.002, 1.093, 0.53],
                        [1.352, 0.613, 0.3],
                    ],
                    [
                        [0.23, 1.508, 2.35],
                        [1.58, 1.243, 0.38],
                        [1.83, 0.813, 0.34],
                    ],
                ][size],
                OpeningFamily::GothicBalconyDoor => [
                    [[-0.2, 1.13, 2.56], [1.14, 0.81, 0.4], [1.34, 0.41, 0.4]],
                    [[-0.41, 1.4, 2.32], [0.98, 1.31, 0.48], [1.34, 0.8, 0.32]],
                    [[-0.33, 2.04, 2.64], [1.15, 1.62, 0.39], [1.42, 1.0, 0.46]],
                ][size],
                _ => unreachable!(),
            };
            Ok(rows
                .into_iter()
                .map(|[offset, width, height]| RawWallHole {
                    center: [input.center[0], offset + input.center[1]],
                    dimensions: [width, height],
                    kind: 0,
                })
                .collect())
        }
    }
}
