//! 羊群的移动状态和快照所需字段。B 原版将移动状态保存在 Sheep+0x30，
//! 世界位置仍由 Transform.translation 持有；完整原生 Sheep 为 104 字节，
//! 此类型仅表示已恢复的语义字段，不声明原生布局。
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

/// B capture/apply读取+0x30移动状态；being_petted读取+0x58。
/// 未恢复的动画/附件字段不填充猜测默认值。
#[derive(Component, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sheep {
    pub move_state: SheepMoveState,
    pub being_petted_timer: f32,
}

impl Sheep {
    /// B 0x14089d4c0：严格大于0.01；NaN返回false。
    pub fn being_petted(&self) -> bool {
        self.being_petted_timer > f32::from_bits(0x3c23_d70a)
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
                move_state: SheepMoveState::Idle { timeout_s: 0.0 },
                being_petted_timer: value,
            };
            assert_eq!(sheep.being_petted(), expected);
        }
    }
}
