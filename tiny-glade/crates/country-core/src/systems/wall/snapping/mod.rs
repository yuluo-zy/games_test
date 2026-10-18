//! B `systems/wall/snapping/mod.rs`: wall-end/segment snapping mode resource.
//!
//! Evidence: `chapters/systems/wall/snapping/mod.rs.md` (O, multi-system cross
//! read). `SnappingMode` stores mode 0=none / 1=segment candidate /
//! 2=end snap / 3=segment snap active, the occupancy flag, the shared
//! `+4..+0xc` snap region and the wall/end ids. `new` (0x140ade610) only
//! zeroes mode and the `+0x20` sound-dedup byte.

pub mod snapping_audio;
pub mod snapping_init;
pub mod snapping_maintain;
pub mod snapping_to_ends;
pub mod snapping_to_segments;
pub mod telegraph_walls_are_merged;

use bevy_ecs::{event::Event, resource::Resource};
use glam::Vec3;

use crate::resources::walls::WallId;

/// Snap-mode constants (B `mode` byte@+0).
pub const SNAP_MODE_NONE: u8 = 0;
pub const SNAP_MODE_SEGMENT_CANDIDATE: u8 = 1;
pub const SNAP_MODE_END: u8 = 2;
pub const SNAP_MODE_SEGMENT: u8 = 3;

/// B SnappingMode 发布区副本（+0x20..+0x38，fresh 0x140a6ad00）：snapping_maintain
/// 把活动区全状态发布到此处；snapping_audio 按发布区模式字节去重。原生偏移：
/// mode@+0x20、snapped@+0x21、snap@+0x24..+0x30、wall@+0x30、end@+0x38。
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct SnappingModePublished {
    /// +0x20：发布的模式字节（snapping_audio 去重键）。
    pub mode: u8,
    /// +0x21：发布的占用标志。
    pub snapped: bool,
    /// +0x24..+0x30：发布的共享 snap 区。
    pub snap: Vec3,
    /// +0x30：发布的墙 id。
    pub wall: WallId,
    /// +0x38：发布的端 id。
    pub end: u8,
}

/// B SnappingMode (0x140ade610 `new`; snapping_to_ends/segments write the
/// shared region). The `+4..+0xc` region is reused per mode:
/// - mode 2 (end): `(x, y, z)` world snap point.
/// - mode 3 (segment): `(terrain_height, x, y)`.
/// Both modes therefore swizzle the same audio position `(snap.y, snap.x,
/// snap.z)` in snapping_audio.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct SnappingMode {
    /// 0 none / 1 segment candidate / 2 end snap / 3 segment snap active.
    pub mode: u8,
    /// Occupancy flag; when false end-snapping probing is allowed.
    pub snapped: bool,
    /// Shared +4..+0xc region (see the type comment for per-mode layout).
    pub snap: Vec3,
    /// Wall being snapped to (+0x10).
    pub wall: WallId,
    /// End id (+0x18): mode 2 only, 0 = start near, 1 = end near.
    pub end: u8,
    /// 发布区 +0x20..+0x38（snapping_maintain 0x140a6ad00 写全状态副本）。
    pub published: SnappingModePublished,
}

impl Default for SnappingMode {
    fn default() -> Self {
        Self::new()
    }
}

impl SnappingMode {
    /// B 0x140ade610 (11 B): `mode = 0; +0x20 = 0`. `snapped` is left
    /// uninitialized by the original constructor.
    pub fn new() -> Self {
        Self {
            mode: SNAP_MODE_NONE,
            snapped: false,
            snap: Vec3::ZERO,
            wall: WallId(0),
            end: 0,
            published: SnappingModePublished::default(),
        }
    }

    /// Inline B line 67: any active snap mode.
    pub fn is_active(&self) -> bool {
        self.mode != SNAP_MODE_NONE
    }

    /// Inline B line 74: world-space snap point per mode.
    pub fn position_to_snap_to(&self) -> Vec3 {
        match self.mode {
            SNAP_MODE_END => Vec3::new(self.snap.x, self.snap.y, self.snap.z),
            SNAP_MODE_SEGMENT => Vec3::new(self.snap.y, self.snap.z, self.snap.x),
            _ => Vec3::ZERO,
        }
    }
}

/// B `DisableSnapping` event (check_for_disable_snapping 0x140a6a560, 0x10 B
/// row): `flag == 1` also resets the mode, `flag == 0` only notifies.
#[derive(Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DisableSnapping(pub bool);
