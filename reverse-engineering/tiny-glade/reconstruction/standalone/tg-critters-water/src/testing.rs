//! Test helpers: a real `bevy_app::App` with the game-side resources and all events, plus RNGs
//! seeded deterministically. Systems under test are added by each test.

use crate::critters::shared::DespawnDuckCmd;
use crate::critters::wren::TreeShaken;
use crate::env::{EnvPlugin, Time};
use crate::rng::{FastRng, PerchanceRes};
use crate::water::{RecalculateHeightMapCmd, WaterDrawEvent, WaterEraseEvent};
use bevy_app::App;

pub const PERCHANCE_SEED: u128 = 1;
pub const FASTRAND_SEED: u64 = 1;

pub fn base_app(dt: f32) -> App {
    let mut app = App::new();
    app.add_plugins(EnvPlugin)
        .insert_resource(Time::from_delta(dt))
        .insert_resource(PerchanceRes::seeded(PERCHANCE_SEED))
        .insert_resource(FastRng::seeded(FASTRAND_SEED))
        .add_event::<DespawnDuckCmd>()
        .add_event::<TreeShaken>()
        .add_event::<WaterDrawEvent>()
        .add_event::<WaterEraseEvent>()
        .add_event::<RecalculateHeightMapCmd>();
    app
}
