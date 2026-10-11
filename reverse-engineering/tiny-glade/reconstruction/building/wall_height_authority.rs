//! B InnerWalls::update_wall_state_height's successful game row mutation.
//! std HashMap lookup/dirty storage and Bevy resource change ticks belong to
//! actual dependencies/callers. This owner does not clamp or compare heights.
//! Source+54 is the editable height input also passed to publication's low/high
//! constraint, not the derived public maximum Y or raw Curve2.length. It maps
//! to RawWallPublicationSource.extra; source+58 is a distinct terrain floor.
use crate::building_wall_state::WallId;

#[derive(Clone, Copy, Debug)]
pub struct WallHeightAuthorityOutcome {
    pub wall_id: WallId,
    pub requested_height: f32,
    /// The original caller inserts into InnerWalls' dirty WallId set, even for
    /// identical bits, before writing the source scalar at +0x54.
    pub mark_dirty: bool,
    pub source_written: bool,
}

pub fn assign_resolved_height(
    current_height: &mut f32,
    wall_id: WallId,
    requested_height: f32,
) -> WallHeightAuthorityOutcome {
    *current_height = requested_height;
    WallHeightAuthorityOutcome {
        wall_id,
        requested_height,
        mark_dirty: true,
        source_written: true,
    }
}

/// Source projection includes only observed bytes; unlike an ABI-shaped
/// object, it does not own or reconstruct the Curve2 Vec/HashMap headers.
pub fn write_resolved_source_height(
    source: &mut [u8; 112],
    wall_id: WallId,
    requested_height: f32,
) -> WallHeightAuthorityOutcome {
    source[0x54..0x58].copy_from_slice(&requested_height.to_le_bytes());
    WallHeightAuthorityOutcome {
        wall_id,
        requested_height,
        mark_dirty: true,
        source_written: true,
    }
}
