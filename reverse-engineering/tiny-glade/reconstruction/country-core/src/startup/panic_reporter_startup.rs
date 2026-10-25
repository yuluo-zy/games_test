//! `startup/panic_reporter_startup.rs` — panic-reporter capture hook.
//!
//! Source: B skeleton `crates/country-core/src/startup/panic_reporter_startup.rs`.
//! O evidence: `panic_reporter_startup` (system reading `MainCamera`,
//! `TimeOfDay`, `History`, `InnerWalls`, `InputRecordReplayState` — used to
//! capture a snapshot into the crash payload).

#![allow(dead_code, unused_imports)]

use crate::Gap;

pub fn panic_reporter_startup(
    _camera: &Gap,
    _time_of_day: &Gap,
    _history: &Gap,
    _inner_walls: &Gap,
    _replay: &Gap,
) {
    todo!("recover panic_reporter_startup")
}
