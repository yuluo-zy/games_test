//! snapping — wall snapping-mode state + snapping systems.

use bevy_ecs::prelude::Resource;

use crate::typed_gap::Gap;

/// Snapping mode (O: `SnappingMode::new` 0x140ade610; `is_active`,
/// `position_to_snap_to`). Byte @0 is the mode (1 = to segments, 2 = to ends);
/// bool @1 is the snapped flag.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct SnappingMode {
    pub mode: u8,
    pub snapped: bool,
}
impl SnappingMode {
    pub fn new() -> Self {
        Self { mode: 0, snapped: false }
    }
    pub fn is_active(&self) -> bool {
        self.mode != 0
    }
    pub fn position_to_snap_to(&self) -> glam::Vec3 {
        glam::Vec3::ZERO
    }
}

pub mod snapping_audio;
pub mod snapping_init;
pub mod snapping_maintain;
pub mod snapping_to_ends;
pub mod snapping_to_segments;
pub mod telegraph_walls_are_merged;
