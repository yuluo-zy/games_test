//! `systems::backend::input` — WP20 input-backend systems (7 units).
//!
//! Source: B skeleton `crates/country-core/src/systems/backend/input/{mod,
//! calculate_input_delta,input_record_replay,input_replay_for_trailers,
//! reset_input_state,translate_input,translate_input_to_cursor_moved,
//! update_cursor_position}.rs`.
//! Business logic contract: `contracts/input-backend.md`.

pub mod calculate_input_delta;
pub mod input_record_replay;
pub mod input_replay_for_trailers;
pub mod reset_input_state;
pub mod translate_input;
pub mod translate_input_to_cursor_moved;
pub mod update_cursor_position;

use crate::Gap;

/// `InputReplayCommand` (O named entity; keyboard-driven record/replay
/// command; layout U).
#[derive(Clone, Copy, Debug, Default)]
pub struct InputReplayCommand {
    pub _gap: Gap,
}
