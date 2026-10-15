//! B Sheep's recovered movement state, embedded at native Sheep +0x30.
//! The complete Sheep component is not replaced by a partial/default owner.
pub mod control;

use serde::{Deserialize, Serialize};

/// Original externally tagged wire shape, verified by 0x1413ff520 and
/// Debug 0x1416f2800. This is embedded state, not a separate ECS component.
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
