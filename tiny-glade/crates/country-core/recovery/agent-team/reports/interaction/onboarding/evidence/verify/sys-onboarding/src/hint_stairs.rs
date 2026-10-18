//! hint_stairs（O 0x142285a20，3217）。
//!
//! 楼梯多阶段教程状态机（GameProfile 标志 0x201..0x205，id 13..17）。真逻辑：多
//! EventReader<NotifyHintSystem*Stairs*>；生成 alternate(Option<KeyCode>) /
//! alternate(Graphic("hint/rmb"))，写 DisplayHintRequest（id 6，flags 0xd00/0xe00/
//! 0x1000/0x1100/0x100）；按条件写 TutorialHintComplete（tag 0xd/0xe/0xf/0x10/0x11）。
//!
//! **未实现**：五阶段触发条件/文案 + 6 个 NotifyHintSystem*Stairs* 事件 + GameProfile
//! 0x201..0x205 未落库 → 标未实现。

use bevy_ecs::prelude::*;

/// 楼梯提示条件（O：hints_startup 初值 12B 清零：u64@+8=0，u32@+0x10=0）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct StairsHintConditions {
    pub counter: u64,
    pub flags: u32,
}

/// hint_stairs（O 0x142285a20）：见模块头。
/// **未实现**：6×NotifyHintSystem*Stairs* 事件 + GameProfile 0x201..0x205 未落库。
pub fn hint_stairs(
    _profile: Res<country_core::game_profile::GameProfile>,
    _state: ResMut<StairsHintConditions>,
    _mode: Res<country_core::systems::mode_manager::AppMode>,
) {
    // 未实现：见模块头。
}
