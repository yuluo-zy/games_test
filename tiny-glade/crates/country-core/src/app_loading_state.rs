//! B AppLoadingState: fmt @140308a00 indexes the three original name/length
//! tables @1429b8460 / @1429b8448. init_state @1403258c0 initializes tag 0.
//! Kept as the real Bevy state identity, not an unrelated host loading boolean.
use bevy_state::prelude::States;

#[derive(States, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AppLoadingState {
    #[default]
    Splash = 0,
    Game = 1,
    Reload = 2,
}
