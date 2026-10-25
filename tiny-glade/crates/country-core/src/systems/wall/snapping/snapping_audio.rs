//! B `systems/wall/snapping/snapping_audio.rs`: snap sound dedup/emission.
//!
//! Evidence: `chapters/systems/wall/snapping/snapping_audio.rs.md` (O,
//! 0x140a6a480). Mode 2 emits kind 1 and mode 3 kind 2, unless the previous
//! published mode matches (continuous same-mode frames do not re-sound). The
//! event position is the `(snap.y, snap.x, snap.z)` swizzle of the shared
//! snap region.

use bevy_ecs::prelude::*;
use glam::Vec3;

use super::{SNAP_MODE_END, SNAP_MODE_SEGMENT, SnappingMode};
use crate::systems::audio::PlayWallUiSound;

/// B 0x140a6a480 (2-param system).
pub fn snapping_audio(
    mode: Res<SnappingMode>,
    mut sounds: EventWriter<PlayWallUiSound>,
) {
    let (kind, position) = match mode.mode {
        SNAP_MODE_END => {
            if mode.published.mode == SNAP_MODE_END {
                return;
            }
            (1, Vec3::new(mode.snap.y, mode.snap.x, mode.snap.z))
        }
        SNAP_MODE_SEGMENT => {
            if mode.published.mode > SNAP_MODE_END {
                return;
            }
            (2, Vec3::new(mode.snap.y, mode.snap.x, mode.snap.z))
        }
        _ => return,
    };
    sounds.write(PlayWallUiSound {
        kind,
        position,
        selector: 0,
        value: 0.0,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_audio(mode: u8, last_published: u8) -> Vec<PlayWallUiSound> {
        let mut world = World::new();
        world.insert_resource(SnappingMode::new());
        world.init_resource::<Events<PlayWallUiSound>>();
        world.resource_mut::<SnappingMode>().mode = mode;
        world.resource_mut::<SnappingMode>().published.mode = last_published;
        world.resource_mut::<SnappingMode>().snap = Vec3::new(1.0, 2.0, 3.0);
        let mut schedule = Schedule::default();
        schedule.add_systems(snapping_audio);
        schedule.run(&mut world);
        world
            .resource_mut::<Events<PlayWallUiSound>>()
            .drain()
            .collect::<Vec<_>>()
    }

    #[test]
    fn end_snap_emits_kind_one() {
        let sounds = run_audio(SNAP_MODE_END, 0);
        assert_eq!(
            sounds,
            vec![PlayWallUiSound {
                kind: 1,
                position: Vec3::new(2.0, 1.0, 3.0),
                selector: 0,
                value: 0.0,
            }]
        );
    }

    #[test]
    fn segment_snap_emits_kind_two() {
        let sounds = run_audio(SNAP_MODE_SEGMENT, 0);
        assert_eq!(
            sounds,
            vec![PlayWallUiSound {
                kind: 2,
                position: Vec3::new(2.0, 1.0, 3.0),
                selector: 0,
                value: 0.0,
            }]
        );
    }

    #[test]
    fn repeated_mode_is_suppressed_by_last_published() {
        let sounds = run_audio(SNAP_MODE_SEGMENT, SNAP_MODE_SEGMENT);
        assert!(sounds.is_empty());
    }
}
