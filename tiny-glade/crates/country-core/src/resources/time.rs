//! B `GameTime` resource consumed at +0x28 by GUI log decay and roof animation.
//! The existing clock implementation follows `time_update` (0x140922c30).
//! Re-export preserves one Bevy resource identity across these consumers.
pub use super::game_clock::GameClock as GameTime;
