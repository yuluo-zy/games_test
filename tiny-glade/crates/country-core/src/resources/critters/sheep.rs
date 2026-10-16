//! B Sheep's recovered movement state, embedded at native Sheep +0x30.
//! The complete Sheep component is not replaced by a partial/default owner.
pub mod control;

use serde::{Deserialize, Serialize};

/// Original externally tagged wire shape, verified by 0x1413ff520 and
/// Debug 0x1416f2800. This is embedded state, not a separate ECS component.
///
/// Native memory layout (re-analyst, 2026-10-15): discriminant byte at +0,
/// `Idle { timeout_s }` when 0, otherwise `Walking { goal_change_timeout,
/// timeout_s, force_reach_goal }`. The *serde* order (what the save wire
/// carries) is timeout_s, goal_change_timeout, force_reach_goal — the current
/// declaration matches that order; the memory order of the Walking fields is
/// `goal_change_timeout, <f32>, force_reach_goal` with the middle field name
/// 待取证.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum SheepMoveState {
    Idle {
        timeout_s: f32,
    },
    Walking {
        timeout_s: f32,
        goal_change_timeout: f32,
        force_reach_goal: bool,
    },
}
