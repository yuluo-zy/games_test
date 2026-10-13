//! `backend/input/calculate_input_delta.rs` — per-analog-action delta
//! computation.
//!
//! PDB: `calculate_input_delta` @0x1416feeb0 (5502 B). Named entity
//! `UiHistoryButtonPressed` (layout U).
//!
//! O: for each analog action `i` (active byte @+0x30+i, kind byte @+0x88+i,
//! raw value @+0x58+i*8), writes an output pair `{tag i32 @, value i32}`:
//!   - inactive: `tag = (kind ^ 1) << 2` (kind 0→4, 1→0, 2→12), value = prev.
//!   - active, kind 0: tag = 3 (prev-action-tag == 1 ? keep : 0), value = prev.
//!   - active, kind 2: deadzone — `value = clamp(prev + 0.1 - base_delta, 0.05,
//!     …)`, tag = 2.
//!   - active, kind 1: `value = prev + base_delta` (`base_delta` = `*+0x58` of
//!     the source action state), tag = 1.
//! Output also mirrors 10 qwords of the source action values @+0xa0.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;
use glam::Vec2;

use crate::Gap;

/// `UiHistoryButtonPressed` (O named entity; layout U).
#[derive(Clone, Copy, Debug, Default)]
pub struct UiHistoryButtonPressed {
    pub _gap: Gap,
}

/// `calculate_input_delta` @0x1416feeb0 (O): smooth the analog action values
/// into per-action `{tag, value}` pairs consumed by the action-set resources.
pub fn calculate_input_delta(
    _actions: Res<Gap>,
    _delta: Res<Gap>,
    _out: ResMut<Gap>,
) {
    // Recovered (O): N analog actions each read active byte / kind byte / raw
    // value and emit {tag,value}; kind-2 deadzone 0.1/0.05; kind-1 accumulate
    // base delta @+0x58. Field offsets -> typed gaps.
    todo!("calculate_input_delta per-action tag/value delta (offsets O, fields U)")
}
