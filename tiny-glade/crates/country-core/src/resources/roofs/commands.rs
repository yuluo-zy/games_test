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
pub struct EditRoofCmd(pub crate::resources::roofs::geometry::edit_rules::RoofEditEvent);
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
/// Original name for `RoofGeometryChanged` in the system faces:
/// `country_core::resources::roofs::commands::RecalculateRoofCmd`.
#[derive(Event, Clone, Copy, Debug)]
pub struct RecalculateRoofCmd(pub WallId);
#[derive(Event, Clone, Copy, Debug)]
pub struct RecalculateWallHoles(pub WallId);
/// B `country_core::resources::roofs::commands::SwitchRoofTypeCmd`: emitted by
/// `edit_roof` on pitched/flat crossings, consumed by `switch_roof_type`.
#[derive(Event, Clone, Copy, Debug)]
pub struct SwitchRoofTypeCmd {
    pub entity: Entity,
    pub wall: WallId,
    pub crossing: crate::resources::roofs::geometry::edit_rules::HeightCrossing,
    pub source_flag: u8,
}
/// B `country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks`:
/// roof changes that invalidate brick construction.
#[derive(Event, Clone, Copy, Debug)]
pub struct RecalculateWallBricks(pub WallId);
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
    pub feedback: crate::resources::roofs::geometry::edit_rules::EditFeedback,
}

#[derive(Event, Clone, Debug)]
pub struct ReplayRoofEvent(pub crate::recovered_building::roof_replay::ObservedReplayEvent);
