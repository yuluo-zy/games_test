//! B `systems/wall/snapping/snapping_maintain.rs`: mode maintenance,
//! disable-notification and the snapping preview signifiers.
//!
//! Evidence: `chapters/systems/wall/snapping/snapping_maintain.rs.md` and
//! `snapping/snapping_signifiers.rs.md` (O). The PDB regroups
//! `snapping_signifiers` (0x140a6a650) under this file.

use bevy_ecs::prelude::*;
use glam::Vec3;

use super::{DisableSnapping, SNAP_MODE_END, SNAP_MODE_SEGMENT, SnappingMode};
use crate::resources::walls::public_walls::PublicWalls;
use crate::systems::ui::signifiers::signifier_create::{UiAppendSignifier, UiSignifierStream};

/// Local proxy for the B flag resource read by check_for_disable_snapping
/// (param_2 first byte): non-zero means a modifier that disables snapping is
/// held. Exact resource identity 待取证.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct DisableSnappingInput(pub bool);

/// B 0x140a6a560 (3-param). `flag == 1` also clears the mode; `flag == 0`
/// only writes the disable notification.
pub fn check_for_disable_snapping(
    mut mode: ResMut<SnappingMode>,
    input: Res<DisableSnappingInput>,
    mut events: EventWriter<DisableSnapping>,
) {
    match (mode.mode, input.0) {
        (SNAP_MODE_END, false) | (SNAP_MODE_SEGMENT, false) => {
            events.write(DisableSnapping(false));
        }
        (SNAP_MODE_END, true) | (SNAP_MODE_SEGMENT, true) => {
            events.write(DisableSnapping(true));
            mode.mode = super::SNAP_MODE_NONE;
        }
        (_, true) => {
            mode.mode = super::SNAP_MODE_NONE;
        }
        _ => {}
    }
}

/// B 0x140a6ad00: publish the current mode to the `+0x20` sound-dedup byte.
pub fn snapping_maintain(mut mode: ResMut<SnappingMode>) {
    mode.last_published_mode = mode.mode;
}

/// B 0x140a6a650 (6-param signifier system). Mode 2 draws the end-snap
/// preview centered on the snap point; mode 3 draws the segment preview line.
/// The exact dual signifier-stream structure is 待接线 (render host); the
/// reconstruction writes [`UiAppendSignifier`] records to the library stream.
pub fn snapping_signifiers(
    mode: Res<SnappingMode>,
    public: Res<PublicWalls>,
    mut stream: ResMut<UiSignifierStream>,
) {
    if mode.mode <= super::SNAP_MODE_SEGMENT_CANDIDATE {
        return;
    }
    if mode.mode == SNAP_MODE_END {
        // {x, height, y} swizzle of the shared snap region.
        let position = Vec3::new(mode.snap.y, mode.snap.x, mode.snap.z);
        stream.write(UiAppendSignifier {
            tag: 1,
            kind: 1,
            position,
            scale: 1.5,
            name: 0,
        });
        return;
    }
    let Some(state) = public.get(mode.wall) else {
        return;
    };
    let Some(line) = crate::systems::wall::shapes::shape_placement_signifiers::preview_snapping_line(
        state,
    ) else {
        return;
    };
    // Segment preview: a straight helper line from the wall shape toward the
    // snap point (B DebugLine/DebugSphere drawing, host 待接线).
    let start = line;
    let end = Vec3::new(mode.snap.y, mode.snap.z, mode.snap.x);
    stream.write(UiAppendSignifier {
        tag: 2,
        kind: 2,
        position: start,
        scale: end.x,
        name: end.z.to_bits() as u64,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::wall::snapping::SNAP_MODE_NONE;

    fn run_disable(input: bool, mode: u8) -> (u8, Vec<DisableSnapping>) {
        let mut world = World::new();
        world.insert_resource(SnappingMode::new());
        world.insert_resource(DisableSnappingInput(input));
        world.init_resource::<Events<DisableSnapping>>();
        world.resource_mut::<SnappingMode>().mode = mode;
        let mut schedule = Schedule::default();
        schedule.add_systems(check_for_disable_snapping);
        schedule.run(&mut world);
        let events = world
            .resource_mut::<Events<DisableSnapping>>()
            .drain()
            .collect::<Vec<_>>();
        (world.resource::<SnappingMode>().mode, events)
    }

    #[test]
    fn disable_without_flag_only_notifies() {
        let (mode, events) = run_disable(false, SNAP_MODE_END);
        assert_eq!(mode, SNAP_MODE_END);
        assert_eq!(events, vec![DisableSnapping(false)]);
    }

    #[test]
    fn disable_with_flag_also_resets_mode() {
        let (mode, events) = run_disable(true, SNAP_MODE_SEGMENT);
        assert_eq!(mode, SNAP_MODE_NONE);
        assert_eq!(events, vec![DisableSnapping(true)]);
    }
}
