//! Roof commands and downstream notifications. Bevy owns event retention.
use crate::recovered_building::{
    building_history_transactions::RoofHistoryEdit,
    roof_creation::RoofCreateRequestObserved,
    roof_customization::{FlatRoofRimRequest, FlatRoofStyleRequest},
};
use crate::resources::walls::WallId;
use bevy_ecs::{entity::Entity, event::Event};

#[derive(Event, Clone, Debug)]
pub struct CreateRoofCmd(pub RoofCreateRequestObserved);
#[derive(Event, Clone, Copy, Debug)]
pub struct DeleteRoofCmd(pub WallId);
#[derive(Event, Clone, Copy, Debug)]
pub struct SwitchRoofDirectionCmd {
    pub wall: WallId,
    pub direction: u8,
}
#[derive(Event, Clone, Copy, Debug)]
pub struct SetFlatRoofStyleCmd(pub FlatRoofStyleRequest);
#[derive(Event, Clone, Copy, Debug)]
pub struct SetFlatRoofRimStyleCmd(pub FlatRoofRimRequest);
#[derive(Event, Clone, Copy, Debug)]
pub struct EditRoofCmd(pub system_roof::edit_rules::RoofEditEvent);
#[derive(Event, Clone, Copy, Debug)]
pub struct RoofIsInvalid(pub WallId);
#[derive(Event, Clone, Copy, Debug)]
pub struct ResetRoofs;
#[derive(Event, Clone, Copy, Debug)]
pub struct WallRoofPresenceChanged(pub WallId);
#[derive(Event, Clone, Copy, Debug)]
pub struct WallGableChanged(pub WallId);
#[derive(Event, Clone, Copy, Debug)]
pub struct RoofGeometryChanged {
    pub entity: Entity,
    pub wall: WallId,
    pub kind: u8,
}
#[derive(Event, Clone, Copy, Debug)]
pub struct RecalculateWallHoles(pub WallId);
#[derive(Event, Clone, Copy, Debug)]
pub struct RoofSound {
    pub entity: Entity,
    pub wall: WallId,
    pub pivot: [f32; 3],
    pub kind: u8,
    /// Only sound variant 6 reads its extra byte (Roof +0x54).
    pub flat_surface: Option<u8>,
}
/// A mutation awaiting the history owner's transaction; emission is not commit.
#[derive(Event, Clone, Debug)]
pub struct RecordRoofHistory(pub RoofHistoryEdit);
#[derive(Event, Clone, Debug)]
pub struct RoofEditFeedback {
    pub entity: Entity,
    pub wall: WallId,
    pub feedback: system_roof::edit_rules::EditFeedback,
}
#[derive(Event, Clone, Copy, Debug)]
pub struct RoofTypeChanged {
    pub entity: Entity,
    pub wall: WallId,
    pub crossing: system_roof::edit_rules::HeightCrossing,
    pub source_flag: u8,
}

#[derive(Event, Clone, Debug)]
pub struct ReplayRoofEvent(pub crate::recovered_building::roof_replay::ObservedReplayEvent);
