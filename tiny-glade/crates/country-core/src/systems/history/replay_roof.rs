use crate::{
    recovered_building::building_history_transactions::BuildingReplayOutput,
    resources::{
        history::{HistoryReplayError, ReplayBuffer, ReplayCommand},
        roofs::commands::{EditRoofCmd, ReplayRoofEvent},
        time::GameTime,
    },
    systems::history::HistoryReplayFailed,
};
use bevy_ecs::prelude::*;

/// B 0x141a37ad0: visit only the front bucket, filter Roof commands, process
/// every matching command in order, then remove finished commands. No flush
/// between commands and no advancement to a second bucket in this system.
pub fn replay_roof(
    time: Res<GameTime>,
    mut replay: ResMut<ReplayBuffer>,
    mut discrete: EventWriter<ReplayRoofEvent>,
    mut visual: EventWriter<EditRoofCmd>,
    mut failures: EventWriter<HistoryReplayFailed>,
) {
    let dt = time.delta * replay.speed;
    let Some(bucket) = replay.front_mut_bucket() else {
        return;
    };
    for command in &mut bucket.commands {
        let ReplayCommand::Roof(command) = command else {
            continue;
        };
        match command.advance(dt) {
            Ok(BuildingReplayOutput::Discrete(event)) => {
                discrete.write(ReplayRoofEvent(event));
            }
            Ok(BuildingReplayOutput::Visual(event)) => {
                visual.write(EditRoofCmd(event));
            }
            Ok(BuildingReplayOutput::NoEvent) => {}
            Err(error) => {
                failures.write(HistoryReplayFailed {
                    command: None,
                    error: HistoryReplayError::Roof(error),
                });
            }
        }
    }
    bucket.commands.retain(|command| {
        !matches!(command,
        ReplayCommand::Roof(command) if command.state().bytes()[21] == 1)
    });
}
