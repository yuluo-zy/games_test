//! Recovered game publication reductions and fields; Arc/Vec/hashbrown excluded.
use crate::resources::walls::publication_fields::PublicWallBuildingState;
use crate::resources::walls::publication_fields::WallShapeSnapshot;

#[derive(Clone, Copy, Debug)]
pub struct WallPublicationMetrics {
    pub minimum_relative_height: f32,
    pub minimum_position_y: f32,
    pub maximum_position_y: f32,
    /// Original source field name is not established; do not call this wall
    /// height. Public +0x30 is source Curve2 length, not wall height.
    pub constrained_extra: f32,
}
fn min_fold(a: f32, b: f32) -> f32 {
    if a == b { a } else { a.min(b) }
}
fn max_fold(a: f32, b: f32) -> f32 {
    if a == b { a } else { a.max(b) }
}
pub fn publication_metrics(
    relative_heights: &[f32],
    positions: &[[f32; 3]],
    extra: f32,
    special: bool,
) -> WallPublicationMetrics {
    assert_eq!(
        relative_heights.len(),
        positions.len(),
        "original resolved arrays share a count"
    );
    let minimum_relative_height = relative_heights.iter().copied().fold(24.0, min_fold);
    let minimum_position_y = positions.iter().map(|p| p[1]).fold(100.0, min_fold);
    let maximum_position_y = positions.iter().map(|p| p[1]).fold(0.0, max_fold);
    let (lo, hi) = if special { (0.3, 2.5) } else { (0.5, 14.0) };
    let low = if lo > extra { lo } else { extra };
    let constrained_extra = if hi < low { hi } else { low };
    WallPublicationMetrics {
        minimum_relative_height,
        minimum_position_y,
        maximum_position_y,
        constrained_extra,
    }
}

/// Source values are resolved before this projection. Geometry ownership and
/// dirty/terrain gathering are not guessed; caller retains those dependencies.
pub struct WallPublicationCore<'a> {
    pub source_point_count: u64,
    pub source_curve_length: f32,
    pub shape: &'a WallShapeSnapshot,
    pub style: u8,
    pub metrics: WallPublicationMetrics,
}
impl WallPublicationCore<'_> {
    /// Update only identified game fields of a projection; never manufacture
    /// Vec/Arc headers. New publication invalidates both max-y cache flags.
    pub fn write_projection(&self, bytes: &mut [u8; 672]) -> PublicWallBuildingState {
        bytes[0x10..0x18].copy_from_slice(&self.source_point_count.to_le_bytes());
        bytes[0x30..0x34].copy_from_slice(&self.source_curve_length.to_le_bytes());
        bytes[0x28c..0x290].copy_from_slice(&self.source_curve_length.to_le_bytes());
        if self.shape.tag() < 2 {
            bytes[0x258..0x274].copy_from_slice(self.shape.bytes());
        } else {
            bytes[0x258..0x25c].copy_from_slice(&self.shape.bytes()[..4]);
        }
        bytes[0x108..0x10c].fill(0);
        bytes[0x110..0x114].fill(0);
        for (offset, value) in [
            (0x274, self.metrics.minimum_position_y),
            (0x278, self.metrics.maximum_position_y),
            (0x27c, self.metrics.minimum_relative_height),
            (0x284, self.metrics.constrained_extra),
        ] {
            bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        }
        bytes[0x299] = self.style;
        PublicWallBuildingState::from_observed(*bytes)
    }
}
