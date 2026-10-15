//! Route the original RoofReplayCmd outputs to the same live command owners.
//! The shared history cursor and replay buckets own timing and completion.
use crate::{
    recovered_building::{
        roof_creation::RoofCreateRequestObserved, roof_customization::FlatRoofRimRequest,
        roof_replay::ObservedReplayEvent, wall_state::WallId as ObservedWallId,
    },
    resources::{roofs::commands::*, walls::WallId},
};
use bevy_ecs::prelude::*;

pub fn route_roof_replay(
    mut replay: EventReader<ReplayRoofEvent>,
    mut create: EventWriter<CreateRoofCmd>,
    mut delete: EventWriter<DeleteRoofCmd>,
    mut direction: EventWriter<SwitchRoofDirectionCmd>,
    mut rim: EventWriter<SetFlatRoofRimStyleCmd>,
) {
    for event in replay.read() {
        match &event.0 {
            ObservedReplayEvent::Create { .. } => {
                create.write(CreateRoofCmd(
                    RoofCreateRequestObserved::from_replay_event(&event.0).unwrap(),
                ));
            }
            ObservedReplayEvent::Delete { wall_index } => {
                delete.write(DeleteRoofCmd(WallId(*wall_index)));
            }
            ObservedReplayEvent::Direction {
                wall_index,
                direction: value,
            } => {
                direction.write(SwitchRoofDirectionCmd {
                    wall: WallId(*wall_index),
                    direction: *value,
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
}
