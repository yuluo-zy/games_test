//! hint_stairs（O 0x142285a20，3217）。
//!
//! 楼梯多阶段教程状态机（GameProfile 标志 0x201..0x205，id 13..17）。真逻辑（fresh
//! 0x142285a20）：6 个 EventReader<NotifyHintSystem*Stairs*> + 计数器（puVar6）；阶段门控：
//! - 0x205==0：reader50 有事件 → reader58 有事件 → DisplayHintRequest（+0x48 u16 =
//!   0x1100 → id **17** StairsCanDetach，flags 0，Steam+Key）；否则 TutorialHintComplete(id 17)。
//! - 0x201==0：计数器!=0 → TutorialHintComplete(id 13)；否则（状态条件）→
//!   DisplayHintRequest（0xd00 → id **13** StairsPlaced，flags 0，Graphic("hint/rmb")）。
//! - 0x202==0：reader68 事件 且 计数器!=0 且 [首事件]=0 → TutorialHintComplete(id 14)；
//!   （状态条件 且 计数器>2）→ DisplayHintRequest（0xe00 → id **14** StairsFocus，flags 0）。
//! - 0x203==0：AppMode==4 → TutorialHintComplete(id 15)；0x201!=0 且 0x202==1 →
//!   reader38 事件 → DisplayHintRequest（0x100 → id **1** FocusedEdit 通用文本，flags 0）。
//! - 0x204==0：reader40 事件 → TutorialHintComplete(id 16)；否则 reader48 事件 →
//!   DisplayHintRequest（0x1000 → id **16** StairsDetach，flags 0）。
//! 以上 id 均为 native `+0x48` u16 的高字节（低字节 flags=0）；DisplayHintRequest
//! 布局判等见 hint_orchestrator。
//!
//! **已重建（2026-10-17，6×NotifyHintSystem*Stairs* + GameProfile 0x201..0x205
//! 落库）**；分歧点：reader→事件精确映射 / 计数器（puVar6）语义 / 状态条件
//! （`*piVar8==0xb && [1]=='?'`）U → 本重建按顺序接 6 事件，counter 以
//! StairsHintConditions.counter 承载，展示分支以 seam 门控。

use bevy_ecs::prelude::*;

/// 楼梯提示条件（O：hints_startup 初值 12B 清零：u64@+8=0，u32@+0x10=0）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct StairsHintConditions {
    /// @+0：计数器（O：puVar6 语义 U；本重建承载展示计数）。
    pub counter: u64,
    /// @+8..+0x10：保留。
    pub flags: u32,
}

/// 展示状态 seam（O：`*piVar8==0xb && [1]=='?'` 语义 U；本重建常量 false → 0xd00/
/// 0xe00 展示分支不触发）。
pub fn stairs_show_condition_seam() -> bool {
    false
}

/// 楼梯提示（O 0x142285a20）：见模块头。**重建（骨架）**（2026-10-17）；
/// 分歧点：reader 映射 / 计数器 / 状态条件 U。
#[allow(clippy::too_many_arguments)]
pub fn hint_stairs(
    profile: Res<country_core::game_profile::GameProfile>,
    mut state: ResMut<StairsHintConditions>,
    mode: Res<country_core::systems::mode_manager::AppMode>,
    mut placed: EventReader<country_core::resources::onboarding::NotifyHintSystemStairsPlaced>,
    mut hovering: EventReader<country_core::resources::onboarding::NotifyHintSystemHoveringOverStairs>,
    mut started_continue: EventReader<country_core::resources::onboarding::NotifyHintSystemStartedStairsContinue>,
    mut can_continue: EventReader<country_core::resources::onboarding::NotifyHintSystemCanContinueStairs>,
    mut detached: EventReader<country_core::resources::onboarding::NotifyHintSystemStairsDetachedFromWall>,
    mut can_detach: EventReader<country_core::resources::onboarding::NotifyHintSystemStairsCanDetachFromWall>,
    mut request: EventWriter<crate::DisplayHintRequest>,
    mut completed: EventWriter<crate::tutorial_completion::TutorialHintComplete>,
) {
    // 阶段 5（0x205）：放置事件存在时，后续可脱离事件 → 展示（0x1100）；否则完成 id 17。
    if !profile.stairs_stage5_completed {
        if placed.read().next().is_some() {
            if can_detach.read().next().is_some() {
                let display = crate::BindingHintDisplay::Steam(
                    crate::hint_orchestrator::steam_query(
                        country_core::resources::steam::packed_set_plus_one(0),
                    ),
                )
                .alternate(None::<crate::KeyCode>);
                request.write(crate::DisplayHintRequest::new(
                    display,
                    crate::hint_orchestrator::HintType::StairsCanDetach,
                    0,
                ));
            } else {
                completed.write(crate::tutorial_completion::TutorialHintComplete { id: 17 });
            }
        }
    }
    // 阶段 1（0x201）：放置事件计数；计数!=0 → 完成 id 13；否则展示（0xd00，seam）。
    if !profile.stairs_stage1_completed {
        state.counter = state.counter.saturating_add(placed.read().count() as u64);
        if state.counter != 0 {
            completed.write(crate::tutorial_completion::TutorialHintComplete { id: 13 });
        } else if stairs_show_condition_seam() {
            let display = crate::BindingHintDisplay::Steam(
                crate::hint_orchestrator::steam_query(
                    country_core::resources::steam::packed_set_plus_one(0),
                ),
            )
            .alternate(crate::hint_orchestrator::graphic("hint/rmb"));
            request.write(crate::DisplayHintRequest::new(
                display,
                crate::hint_orchestrator::HintType::StairsPlaced,
                0,
            ));
        }
    }
    // 阶段 2（0x202）：起始续建事件存在 且 计数!=0 → 完成 id 14；展示（0xe00，seam）。
    if !profile.stairs_stage2_completed {
        let _ = hovering.read();
        let _ = started_continue.read();
        if state.counter != 0 {
            completed.write(crate::tutorial_completion::TutorialHintComplete { id: 14 });
        } else if stairs_show_condition_seam() && 2 < state.counter {
            let display = crate::BindingHintDisplay::Steam(
                crate::hint_orchestrator::steam_query(
                    country_core::resources::steam::packed_set_plus_one(0),
                ),
            )
            .alternate(crate::hint_orchestrator::graphic("hint/rmb"));
            request.write(crate::DisplayHintRequest::new(
                display,
                crate::hint_orchestrator::HintType::StairsFocus,
                0,
            ));
        }
    }
    // 阶段 3（0x203）：AppMode==4（Editing(StairPoint)）→ 完成 id 15；阶段1&2 完成且
    // 可续建事件 → 展示（0x100）。
    if !profile.stairs_stage3_completed {
        let _ = can_continue.read();
        if mode.native_tag() == 4 {
            completed.write(crate::tutorial_completion::TutorialHintComplete { id: 15 });
        } else if profile.stairs_stage1_completed && profile.stairs_stage2_completed {
            request.write(crate::DisplayHintRequest::new(
                crate::BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(
                    country_core::resources::steam::packed_set_plus_one(0),
                ))
                .alternate(crate::hint_orchestrator::graphic("hint/rmb")),
                crate::hint_orchestrator::HintType::FocusedEdit,
                0,
            ));
        }
    }
    // 阶段 4（0x204）：脱离墙事件 → 完成 id 16；否则可脱离事件 → 展示（0x1000）。
    if !profile.stairs_stage4_completed {
        if detached.read().next().is_some() {
            completed.write(crate::tutorial_completion::TutorialHintComplete { id: 16 });
        } else if can_detach.read().next().is_some() {
            request.write(crate::DisplayHintRequest::new(
                crate::BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(
                    country_core::resources::steam::packed_set_plus_one(0),
                ))
                .alternate(crate::hint_orchestrator::graphic("hint/rmb")),
                crate::hint_orchestrator::HintType::StairsDetach,
                0,
            ));
        }
    }
}
