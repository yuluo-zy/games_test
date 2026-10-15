//! Location command consumer B 0x140999330, producer B 0x140999860.
//! The separate wall-change validity pruner has additional spatial dependencies.
use crate::resources::ivy::{IvyLeafStorage, IvyStorage};
use bevy_ecs::prelude::*;
use glam::Vec3;

/// Native payload is 40 bytes. The first two u64s are forwarded by the producer
/// but are not read by this consumer; retain them without invented semantics.
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct RemoveIvyCmd {
    pub source_0: u64,
    pub source_8: u64,
    pub center: Vec3,
    pub dimensions: Vec3,
}
impl RemoveIvyCmd {
    pub fn at_location(
        center: Vec3,
        width: f32,
        height: f32,
        source_0: u64,
        source_8: u64,
    ) -> Self {
        Self {
            source_0,
            source_8,
            center,
            dimensions: Vec3::new(width, height, width),
        }
    }
}

pub fn remove_ivy_at_location(
    commands: &mut EventWriter<RemoveIvyCmd>,
    center: Vec3,
    width: f32,
    height: f32,
    source_0: u64,
    source_8: u64,
) {
    commands.write(RemoveIvyCmd::at_location(
        center, width, height, source_0, source_8,
    ));
}

/// Removes the suffix beginning at the first point strictly inside the box.
/// Equality and NaN reject the hit. Even an entirely removed ivy keeps its map
/// entry and cached root; the native command does not delete whole ivies.
pub fn apply_remove_ivy_cmd(
    command: &RemoveIvyCmd,
    storage: &mut IvyStorage,
    leaves: &mut IvyLeafStorage,
) {
    let half = command.dimensions * 0.5;
    for (_, ivy) in storage.iter_mut() {
        let first_hit = ivy.points.iter().position(|point| {
            let delta = (point.position - command.center).abs();
            delta.x < half.x && delta.y < half.y && delta.z < half.z
        });
        let Some(first_hit) = first_hit else {
            continue;
        };
        while ivy.point_count() > first_hit {
            let removed = ivy.remove_last_point().expect("ivy tail existed");
            ivy.growth_progress = 0.0;
            ivy.pending_growth = None;
            leaves.delete_affected_leaves(ivy.id, removed.id);
        }
    }
}

pub fn ivy_pruner_cmd(
    mut commands: EventReader<RemoveIvyCmd>,
    mut storage: ResMut<IvyStorage>,
    mut leaves: ResMut<IvyLeafStorage>,
) {
    for command in commands.read() {
        apply_remove_ivy_cmd(command, &mut storage, &mut leaves);
    }
}
