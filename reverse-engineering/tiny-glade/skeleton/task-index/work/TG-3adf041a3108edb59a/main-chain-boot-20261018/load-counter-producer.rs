//! Candidate only. Original B load_game state1 @14146d7xx..d9xx.
//! The canonical four-byte HistoryCmd transport is presently named
//! mode_manager::HistoryCommand; the original consumer identity still needs
//! coordinated integration before registering this producer.
use crate::systems::{
    mode_manager::HistoryCommand,
    save::load_game::{LoadGameManager, LoadGameState},
};
use bevy_ecs::event::Events;

/// Run only after original state0 has actually applied/restored owned History
/// and set its signed cursor delta. Does not emit LoadingGameComplete.
pub fn emit_load_history_commands(
    manager: &mut LoadGameManager,
    commands: &mut Events<HistoryCommand>,
) {
    assert_eq!(manager.state, LoadGameState::ApplyCounter);
    let delta = manager.progress_counter;
    // Preserve x86 signed NEG and the signed loop comparison, including the
    // original empty loop at i32::MIN rather than inventing a huge replay.
    let (count, payload) = if delta < 0 {
        (delta.wrapping_neg(), 0x0301_0001_u32)
    } else {
        (delta, 0x0301_0101_u32)
    };
    for _ in 0..count {
        commands.write(HistoryCommand {
            bytes: payload.to_le_bytes(),
        });
    }
    manager.state = LoadGameState::Loaded;
}
