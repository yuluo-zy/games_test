//! Shared live sheep component, used by the current critter systems and core
//! snapshot consumers. B movement occupies +0x30; position stays in Transform.
//! Existing animation fields are retained semantically, without a native ABI.
pub mod control;

use crate::resources::walls::WallId;
use bevy_ecs::{component::Component, event::Event};
use serde::{Deserialize, Serialize};

/// 原事件的墙ID位于+8；ivy_pruner只消费该字段。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct CmdCheckForSheepInsideBuilding {
    pub wall: WallId,
}

/// B 0x1413ff520 的外部标签协议。原内存 timeout_s@+4、
/// goal_change_timeout@+8、force_reach_goal@+1；无需位转换坐标。
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

/// The single live sheep owner. Animation/behavior fields come from the
/// existing full critter component; the former flag_30/shared_timers are the
/// typed movement state, including the previously dropped force byte+0x31.
/// No birth/default constructor is inferred from this consolidation.
#[derive(Component, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sheep {
    /// Original +8 / +0xc: follow behavior and its timer.
    pub behavior_state: u32,
    pub goal_timer: f32,
    /// Existing pet animation state, progress and target identifier.
    pub pet_phase: u32,
    pub pet_progress: f32,
    pub pet_target: u64,
    pub move_state: SheepMoveState,
    /// Original +0x58, shared with the pet animation consumer.
    pub amplitude: f32,
    pub control_flag: u8,
}

impl Sheep {
    /// B 0x14089d4c0：严格大于0.01；NaN返回false。
    pub fn being_petted(&self) -> bool {
        self.amplitude > f32::from_bits(0x3c23_d70a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn being_petted_keeps_the_original_strict_threshold() {
        for (value, expected) in [
            (0.009, false),
            (0.01, false),
            (0.011, true),
            (f32::NAN, false),
        ] {
            let sheep = Sheep {
                behavior_state: 0,
                goal_timer: 0.0,
                pet_phase: 0,
                pet_progress: 0.0,
                pet_target: 0,
                control_flag: 0,
                move_state: SheepMoveState::Idle { timeout_s: 0.0 },
                amplitude: value,
            };
            assert_eq!(sheep.being_petted(), expected);
        }
    }
}

/// 羊行走动画记录（二进制位于 Sheep 0x68 记录 +0x10/+0x14/+0x28/+0x48/+0x4c/+0x50/+0x54/+0x5c）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SheepWalkRecord {
    /// +0x10：动画状态（1=行走、2=静止过渡、3=静止；跳表 0x14304b0f8 索引）。
    pub state: u32,
    /// +0x14：动画进度。
    pub progress: f32,
    /// +0x28：walk 动画帧索引 = `(idx + n*20 + 3) mod 30`（idx = clamp_f32_to_i64(accum*24.0)）。
    pub frame_index: u64,
    /// +0x48：速度 = |目标位-记录位| / dt。
    pub speed: f32,
    /// +0x4c x/y +0x54 z：上一帧位置记录。
    pub last_pos: glam::Vec3,
    /// +0x5c：walk 累积计时（每帧 `+= dt` 后 `fmodf(accum, 1.25)` 回写）。
    pub accum: f32,
}
