//! 屋顶回放（replay_roof.rs）。
//!
//! - `replay_roof`（0x141a37ad0）：候选参数面（skeleton 富化，全 7 参）：
//!   `(ResMut<ReplayBuffer>, Res<GameTime>, EventWriter<CreateRoofCmd>,
//!    EventWriter<DeleteRoofCmd>, EventWriter<EditRoofCmd>,
//!    EventWriter<SwitchRoofRidgeDirCmd>, EventWriter<SetFlatRoofRimStyleCmd>)`
//!   只访问前桶、过滤 Roof 命令、按序处理每个匹配命令后移除完成命令。
//! - `process`（0x1421b48e0）：单命令分派（switch tag@+0x10）——
//!   Create/Delete/RidgeDir/FlatRimStyle 离散分支 + VisualEdit 分支。
use country_core::recovered_building::{
    building_history_transactions::BuildingReplayOutput,
    roof_creation::RoofCreateRequestObserved,
    roof_customization::FlatRoofRimRequest,
    roof_replay::ObservedReplayEvent,
    wall_state::WallId as ObservedWallId,
};
use country_core::resources::{
    history::{HistoryReplayError, ReplayBuffer, ReplayCommand},
    roofs::commands::*,
    time::GameTime,
    walls::WallId,
};
use country_core::systems::history::HistoryReplayFailed;
use bevy_ecs::prelude::*;
use crate::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd;

/// B 0x141a37ad0：只访问前桶的 Roof 命令并直接写入命令事件。
pub fn replay_roof(
    time: Res<GameTime>,
    mut replay: ResMut<ReplayBuffer>,
    mut create: EventWriter<CreateRoofCmd>,
    mut delete: EventWriter<DeleteRoofCmd>,
    mut visual: EventWriter<EditRoofCmd>,
    mut ridge_dir: EventWriter<SwitchRoofRidgeDirCmd>,
    mut rim: EventWriter<SetFlatRoofRimStyleCmd>,
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
                process(&event, &mut create, &mut delete, &mut ridge_dir, &mut rim);
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

/// B 0x1421b48e0：`ReplayRoofEvent::process(dt, writers)` 的离散分支。
/// tag 判别在观测事件字节 +0x10（Create/Delete/RidgeDir/FlatRimStyle）。
pub fn process(
    event: &ObservedReplayEvent,
    create: &mut EventWriter<CreateRoofCmd>,
    delete: &mut EventWriter<DeleteRoofCmd>,
    ridge_dir: &mut EventWriter<SwitchRoofRidgeDirCmd>,
    rim: &mut EventWriter<SetFlatRoofRimStyleCmd>,
) {
    match event {
        ObservedReplayEvent::Create { .. } => {
            create.write(CreateRoofCmd(
                RoofCreateRequestObserved::from_replay_event(event).unwrap(),
            ));
        }
        ObservedReplayEvent::Delete { wall_index } => {
            delete.write(DeleteRoofCmd(WallId(*wall_index)));
        }
        ObservedReplayEvent::Direction {
            wall_index,
            direction: value,
        } => {
            ridge_dir.write(SwitchRoofRidgeDirCmd {
                wall: WallId(*wall_index),
                direction: *value,
                previous_direction: *value,
            });
        }
        ObservedReplayEvent::FlatRimStyle { wall_index, style } => {
            rim.write(SetFlatRoofRimStyleCmd(FlatRoofRimRequest {
                wall_id: ObservedWallId(*wall_index),
                from: *style,
                to: *style,
                observed_record_history: 0,
            }));
        }
    }
}
