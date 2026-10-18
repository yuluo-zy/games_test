//! Metadata for the existing live InProgressCurve owner. The curve and wall ID
//! remain in that resource so snapping and drawing never have separate owners.
use glam::Vec2;

/// B InProgressWall fields following its curve and wall ID. Field names are
/// semantic projections; no original-process Vec/Option layout is reproduced.
#[derive(Clone, Debug)]
pub struct WallDrawingMetadata {
    /// Original +0x88 Option<Vec2>, captured on the first continued sample.
    pub first_unsnapped_cursor: Option<Vec2>,
    /// Original +0x94, retained desired height for unsnapped drawing.
    pub desired_height: f32,
    /// Original +0x98, captured from actual customization on creation.
    pub color_id: u32,
    /// Original +0x9c: wall0 / fence1.
    pub kind: u8,
    /// Original +0x9d: customization.freehand_ty, zero for fences.
    pub freehand_ty: u8,
}

/// Compatibility path for consumers that name the original enclosing owner.
/// This alias preserves the current InProgressCurve Bevy resource identity.
pub type InProgressWallResource = crate::resources::in_progress_curve::InProgressCurve;
