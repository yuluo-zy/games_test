//! B 0x140aa4a60. Only matching commands in the front mixed bucket advance.
use crate::{
    resources::{
        history::{
            ReplayBuffer, ReplayCommand,
            auto_clutter_edit::{AutoClutterEdit, AutoClutterReplayCmd},
        },
        time::GameTime,
    },
    systems::ui_clutter::DeletedAutoClutter,
};
use bevy_ecs::prelude::*;

/// Returns true when the native command is complete. Native uses elapsed==0
/// as its first-application condition, including a repeated application at dt=0.
pub fn advance_auto_clutter(
    command: &mut AutoClutterReplayCmd,
    deleted: &mut DeletedAutoClutter,
    speed: f32,
    dt: f32,
) -> bool {
    if command.elapsed == 0.0 {
        let AutoClutterEdit::Delete { id } = *command.edit;
        if command.undo {
            deleted.0.remove(&id);
        } else {
            deleted.0.insert(id);
        }
    }
    if !command.timed {
        return true;
    }
    let duration = command.duration / speed;
    // MAXSS selects its second operand on NaN, unlike the old decompilation's
    // `if duration <= minimum`. Minimum's original bits are 0x3d4ccccd.
    let threshold = if duration > 0.05 { duration } else { 0.05 };
    let done = command.elapsed > threshold;
    command.elapsed += dt;
    done
}

pub fn replay_auto_clutter(
    mut replay: ResMut<ReplayBuffer>,
    time: Res<GameTime>,
    mut deleted: ResMut<DeletedAutoClutter>,
) {
    let speed = replay.speed;
    let Some(bucket) = replay.front_mut_bucket() else {
        return;
    };
    bucket.retain_commands_mut(|command| {
        let ReplayCommand::AutoClutter(command) = command else {
            return true;
        };
        !advance_auto_clutter(command, &mut deleted, speed, time.delta)
    });
}
