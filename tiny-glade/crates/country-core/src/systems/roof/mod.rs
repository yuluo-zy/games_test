//! B roof owners connected to core PublicWalls and real Bevy query/event APIs.
//! These are game systems, not a replacement Bevy scheduler. Global registration
//! order, full visual bundles and collision construction remain separate work.
pub mod editing;
pub mod lifecycle;
pub mod registration;
pub mod replay;

use crate::resources::roofs::commands::*;
use bevy_ecs::{event::Events, world::World};

/// Registers event owners without resetting existing queues. Their frame update
/// belongs to the application event-update system, as for other core events.
pub fn register_roof_events(world: &mut World) {
    world.init_resource::<Events<CreateRoofCmd>>();
    world.init_resource::<Events<DeleteRoofCmd>>();
    world.init_resource::<Events<SwitchRoofDirectionCmd>>();
    world.init_resource::<Events<SetFlatRoofStyleCmd>>();
    world.init_resource::<Events<SetFlatRoofRimStyleCmd>>();
    world.init_resource::<Events<EditRoofCmd>>();
    world.init_resource::<Events<RoofIsInvalid>>();
    world.init_resource::<Events<ResetRoofs>>();
    world.init_resource::<Events<WallRoofPresenceChanged>>();
    world.init_resource::<Events<WallGableChanged>>();
    world.init_resource::<Events<RoofGeometryChanged>>();
    world.init_resource::<Events<RecalculateWallHoles>>();
    world.init_resource::<Events<RoofSound>>();
    world.init_resource::<Events<RecordRoofHistory>>();
    world.init_resource::<Events<RoofEditFeedback>>();
    world.init_resource::<Events<RoofTypeChanged>>();
    world.init_resource::<Events<ReplayRoofEvent>>();
}
