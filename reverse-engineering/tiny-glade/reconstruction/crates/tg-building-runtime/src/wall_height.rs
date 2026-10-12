//! Authoritative source+54 update; original owner marks dirty even on equal bits.
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use tiny_glade_reconstruction::building_wall_height_command::{
    AdjustWallHeightRequest, WallHeightCommandOutcome, WallHeightFeedback,
};
use tiny_glade_reconstruction::{
    building_wall_height_authority::WallHeightAuthorityOutcome, building_wall_state::WallId,
};

#[derive(Event, Clone, Copy, Debug)]
pub struct SetWallSourceHeight {
    pub wall: WallId,
    pub height: f32,
}

#[derive(Resource, Default, Debug)]
pub struct WallHeightUpdates(pub Vec<WallHeightAuthorityOutcome>);

#[derive(Event, Clone, Copy, Debug)]
pub struct AdjustWallHeight {
    pub request: AdjustWallHeightRequest,
}
#[derive(Event, Clone, Copy, Debug)]
pub struct WallHeightAudio(pub WallHeightFeedback);
#[derive(Resource, Default, Debug)]
pub struct WallHeightCommandUpdates(pub Vec<WallHeightCommandOutcome>);

#[derive(SystemParam)]
pub struct WallHeightEnvironment<'w, 's> {
    pub frame: Res<'w, crate::BuildingFrame>,
    pub associations: Res<'w, crate::WallRoofMap>,
    pub roofs: Query<'w, 's, &'static crate::Roof>,
}

pub fn register(app: &mut bevy_app::App) {
    app.add_event::<SetWallSourceHeight>()
        .add_event::<AdjustWallHeight>()
        .add_event::<WallHeightAudio>()
        .init_resource::<WallHeightUpdates>()
        .init_resource::<WallHeightCommandUpdates>();
}
