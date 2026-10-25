//! B 0x141695e10 consumes all pending notifications and records slot five once.
use crate::{
    resources::{
        active_raster_stroke::ActiveRasterStrokesStorage,
        history::{EditType, History},
    },
    systems::world_raster::record_raster_history,
};
use bevy_ecs::prelude::*;

#[derive(Event, Clone, Copy, Debug)]
pub struct RecordGardenEditToHistoryCmd;

/// B `record_garden_history` 0x141695e10: drains the notification cursor and
/// records slot five once.
///
/// Fresh decompilation (2026-10-17): after `History::add` (0x140a84ec0) the
/// epilogue (0x14169601a → 0x14169601f) tests nothing and writes no event.
/// Native `History::add` failure goes through `panic_fmt`
/// ("Tried writing to History when th…", no `Err` return), so the Rust
/// `failures.write(HistoryRecordingFailed)` branch — a fabricated defensive
/// branch of the shared history-recording family (roof/wall/ui_clutter same
/// pattern) — is **not** native garden control flow and is removed.
pub fn record_garden_history<S: Send + Sync + 'static>(
    mut events: EventReader<RecordGardenEditToHistoryCmd>,
    mut strokes: ResMut<ActiveRasterStrokesStorage>,
    mut history: ResMut<History<S>>,
) {
    if events.read().last().is_none() {
        return;
    }
    // Native ignores the (panicking) outcome of History::add and goes straight
    // to its epilogue; the shared Rust `HistoryTransactions::add` models failure
    // as an `Err`, so the result is dropped here to match that "call, return".
    let _ = record_raster_history(&mut strokes, 5, &mut history, |stroke| {
        EditType::Garden(stroke)
    });
}
