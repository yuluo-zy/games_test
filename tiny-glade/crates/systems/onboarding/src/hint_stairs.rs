//! hint_stairs（O 0x142285a20，3217B，14 个 SystemParam）：楼梯教程的 5 阶段状态机，
//! `GameProfile` 标志 `0x201..0x205`，完成 id 13..17。
//!
//! 2026-10-26 复审重写。旧版是"骨架"：把计数器当成独立语义、用常量 seam 关闭展示、
//! 展示图形全部用错、阶段顺序也不对。这一版逐语句对照 `decomp-0x142285a20.txt`，参数表取自
//! PDB（`reports/module-recovery-2026-10-23/m01-system-params-from-pdb.md`）：
//! `(AppMode, GameProfile, ResMut<StairsHintConditions>, EventWriter<TutorialHintComplete>,
//! EventWriter<DisplayHintRequest>, EventReader<StairsPlaced>, <HoveringOverStairs>,
//! <StartedStairsContinue>, <CanContinueStairs>, <StairsDetachedFromWall>,
//! <StairsCanDetachFromWall>, <ToolSelectionCancelled>, <ModeJustChanged>, Res<GameTime>)`。
//!
//! 常量（RNE 实读）：动画帧串 0x143038ed8 `hint/mouse`(10B)、0x143038ee2 `hint/lmb`(8B)、
//! 0x143038eea `hint/rmb`(8B)；每帧 0.7 秒（0x3f333333）、高 100.0（0x42c80000）；
//! 阈值 1.0（0x142925904）与整数 2。

use bevy_ecs::prelude::*;

/// 楼梯提示条件（O：hints_startup 初值 12B 清零）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct StairsHintConditions {
    /// @+0：`StairsPlaced` 事件的累计数（原版逐事件 `*puVar6 += 1`）。
    pub counter: u64,
    /// @+8：在"装饰物=楼梯"模式（`AppMode` 判别 0xb 且装饰物类型字节 `'?'` = 63）里累加的
    /// `GameTime.delta`。旧版把这 4 字节当成保留的 u32。
    pub time_in_stairs_mode: f32,
}

/// `AppMode::Decorator(DecoratorType::Stairs)`（`*piVar8 == 0xb && (char)piVar8[1] == '?'`）。
fn in_stairs_decorator_mode(mode: country_core::systems::mode_manager::AppMode) -> bool {
    use country_core::systems::mode_manager::{AppMode, DecoratorType};
    matches!(mode, AppMode::Decorator(DecoratorType::Stairs))
}

/// `Steam(set N +1)` 后接 `alternate<HintGraphicParams>`：两帧 `[hint/mouse, <第二帧>]`。
fn steam_with_two_frame_hint(set: u8, second: &str) -> crate::BindingHintDisplay {
    crate::BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(
        country_core::resources::steam::packed_set_plus_one(set),
    ))
    .alternate(crate::hint_orchestrator::HintGraphicParams::animated(
        vec!["hint/mouse".to_string(), second.to_string()],
        0.7,
        100.,
    ))
}

/// 原版阶段 5 展示的键位：`GameProfile+0x120`（8B）+`+0x128`（4B）= `bindings[24]`。
const DETACH_BINDING: usize = 24;
/// 阶段 5 的 Steam 动作：`func_0x0001409e8d90(0xf)`。
const DETACH_STEAM_ACTION: u8 = 0xf;

/// hint_stairs（O 0x142285a20）。顺序即原版控制流：
/// 0. 逐个 `StairsPlaced` 事件 `counter += 1`；"装饰物=楼梯"模式下 `timer += delta`。
/// 1. 阶段 5（`0x205==0`）：`StairsDetachedFromWall` 有未读 → 完成 17；否则
///    `StairsCanDetachFromWall` 有未读 → 展示（id 17）`Alternate[Steam(set 0xf+1), Key(bindings[24])]`。
/// 2. `0x201==0` → 阶段 1：`counter != 0` → 完成 13；否则楼梯模式且 `timer > 1.0` → 展示（id 13）
///    `Steam(set0+1)` + 帧 `[mouse, lmb]`。`0x201!=0 && 0x202==0` → 阶段 2：先读 `ModeJustChanged`
///    （总是推进）；有未读且 `counter != 0` 时读 `ToolSelectionCancelled`（推进），有未读且最后一个模式
///    变化的 `current` 判别为 0 → 完成 14；再判楼梯模式且 `counter > 2` → 展示（id 14）
///    `Steam(set1+1)` + 帧 `[mouse, rmb]`。**阶段 2 只在阶段 1 已完成时执行。**
/// 3. `0x203==0`：`AppMode` 判别 4（`Editing(StairPoint)`）→ 完成 15；否则 `0x201!=0 && 0x202==1`
///    且 `HoveringOverStairs` 有未读 → 展示（id 1）`Steam(set1+1)` + 帧 `[mouse, rmb]`。
/// 4. `0x204==0`：`StartedStairsContinue` 有未读 → 完成 16；否则 `CanContinueStairs` 有未读 → 展示
///    （id 16）`Steam(set0+1)` + 帧 `[mouse, lmb]`。
#[allow(clippy::too_many_arguments)]
pub fn hint_stairs(
    mode: Res<country_core::systems::mode_manager::AppMode>,
    profile: Res<country_core::game_profile::GameProfile>,
    mut state: ResMut<StairsHintConditions>,
    mut completed: EventWriter<crate::tutorial_completion::TutorialHintComplete>,
    mut request: EventWriter<crate::DisplayHintRequest>,
    mut placed: EventReader<country_core::resources::onboarding::NotifyHintSystemStairsPlaced>,
    mut hovering: EventReader<
        country_core::resources::onboarding::NotifyHintSystemHoveringOverStairs,
    >,
    mut started_continue: EventReader<
        country_core::resources::onboarding::NotifyHintSystemStartedStairsContinue,
    >,
    mut can_continue: EventReader<
        country_core::resources::onboarding::NotifyHintSystemCanContinueStairs,
    >,
    mut detached: EventReader<
        country_core::resources::onboarding::NotifyHintSystemStairsDetachedFromWall,
    >,
    mut can_detach: EventReader<
        country_core::resources::onboarding::NotifyHintSystemStairsCanDetachFromWall,
    >,
    mut cancelled: EventReader<country_core::systems::ui_edit_focus::ToolSelectionCancelled>,
    mut mode_changed: EventReader<country_core::resources::events::ModeJustChanged>,
    time: Res<country_core::resources::time::GameTime>,
) {
    use crate::hint_orchestrator::HintType;
    use crate::tutorial_completion::TutorialHintComplete;

    // 0.
    state.counter += placed.read().count() as u64;
    let stairs_mode = in_stairs_decorator_mode(*mode);
    if stairs_mode {
        state.time_in_stairs_mode += time.delta;
    }

    // 1. 阶段 5（0x205）。
    if !profile.stairs_stage5_completed {
        if detached.read().count() != 0 {
            completed.write(TutorialHintComplete { id: 17 });
        } else if can_detach.read().count() != 0 {
            let display = crate::BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(
                country_core::resources::steam::packed_set_plus_one(DETACH_STEAM_ACTION),
            ))
            .alternate(profile.key_bindings.bindings[DETACH_BINDING]);
            request.write(crate::DisplayHintRequest::new(display, HintType::StairsCanDetach, 0));
        }
    }

    // 2. 阶段 1（0x201）与阶段 2（0x202）。
    if !profile.stairs_stage1_completed {
        if state.counter != 0 {
            completed.write(TutorialHintComplete { id: 13 });
        } else if stairs_mode && 1.0 < state.time_in_stairs_mode {
            request.write(crate::DisplayHintRequest::new(
                steam_with_two_frame_hint(0, "hint/lmb"),
                HintType::StairsPlaced,
                0,
            ));
        }
    } else if !profile.stairs_stage2_completed {
        let latest = mode_changed.read().last().copied();
        if let Some(latest) = latest {
            if state.counter != 0 {
                let cancelled_any = cancelled.read().count() != 0;
                if cancelled_any && latest.current.native_tag() == 0 {
                    completed.write(TutorialHintComplete { id: 14 });
                }
            }
        }
        if stairs_mode && 2 < state.counter {
            request.write(crate::DisplayHintRequest::new(
                steam_with_two_frame_hint(1, "hint/rmb"),
                HintType::StairsFocus,
                0,
            ));
        }
    }

    // 3. 阶段 3（0x203）。
    if !profile.stairs_stage3_completed {
        if mode.native_tag() == 4 {
            completed.write(TutorialHintComplete { id: 15 });
        } else if profile.stairs_stage1_completed
            && profile.stairs_stage2_completed
            && hovering.read().count() != 0
        {
            request.write(crate::DisplayHintRequest::new(
                steam_with_two_frame_hint(1, "hint/rmb"),
                HintType::FocusedEdit,
                0,
            ));
        }
    }

    // 4. 阶段 4（0x204）。
    if !profile.stairs_stage4_completed {
        if started_continue.read().count() != 0 {
            completed.write(TutorialHintComplete { id: 16 });
        } else if can_continue.read().count() != 0 {
            request.write(crate::DisplayHintRequest::new(
                steam_with_two_frame_hint(0, "hint/lmb"),
                HintType::StairsDetach,
                0,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::onboarding::*;
    use country_core::systems::mode_manager::{AppMode, DecoratorType, EditSubject};
    use country_core::systems::ui_edit_focus::ToolSelectionCancelled;
    use crate::tutorial_completion::TutorialHintComplete;

    struct Rig {
        world: World,
        schedule: Schedule,
    }
    impl Rig {
        fn new(mode: AppMode) -> Self {
            let mut world = World::new();
            world.insert_resource(mode);
            world.insert_resource(country_core::game_profile::GameProfile::default());
            world.insert_resource(StairsHintConditions::default());
            world.insert_resource(country_core::resources::time::GameTime::default());
            world.init_resource::<Events<TutorialHintComplete>>();
            world.init_resource::<Events<crate::DisplayHintRequest>>();
            world.init_resource::<Events<NotifyHintSystemStairsPlaced>>();
            world.init_resource::<Events<NotifyHintSystemHoveringOverStairs>>();
            world.init_resource::<Events<NotifyHintSystemStartedStairsContinue>>();
            world.init_resource::<Events<NotifyHintSystemCanContinueStairs>>();
            world.init_resource::<Events<NotifyHintSystemStairsDetachedFromWall>>();
            world.init_resource::<Events<NotifyHintSystemStairsCanDetachFromWall>>();
            world.init_resource::<Events<ToolSelectionCancelled>>();
            world.init_resource::<Events<country_core::resources::events::ModeJustChanged>>();
            let mut schedule = Schedule::default();
            schedule.add_systems(hint_stairs);
            Self { world, schedule }
        }
        fn run(&mut self) {
            self.schedule.run(&mut self.world);
        }
        fn profile(&mut self) -> Mut<'_, country_core::game_profile::GameProfile> {
            self.world.resource_mut::<country_core::game_profile::GameProfile>()
        }
        fn delta(&mut self, dt: f32) {
            self.world.resource_mut::<country_core::resources::time::GameTime>().delta = dt;
        }
        fn completions(&mut self) -> Vec<u8> {
            self.world.resource_mut::<Events<TutorialHintComplete>>().drain().map(|e| e.id).collect()
        }
        fn requests(&mut self) -> Vec<crate::DisplayHintRequest> {
            self.world.resource_mut::<Events<crate::DisplayHintRequest>>().drain().collect()
        }
    }
    const STAIRS: AppMode = AppMode::Decorator(DecoratorType::Stairs);

    /// 逐个 `StairsPlaced` 累加；计数非 0 立即完成阶段 1（id 13），之后不会再展示 0xd00。
    #[test]
    fn placed_events_count_individually_and_complete_stage_one() {
        let mut rig = Rig::new(AppMode::Path);
        rig.world.send_event(NotifyHintSystemStairsPlaced);
        rig.world.send_event(NotifyHintSystemStairsPlaced);
        rig.run();
        assert_eq!(rig.world.resource::<StairsHintConditions>().counter, 2);
        assert_eq!(rig.completions(), [13]);
        rig.run();
        assert_eq!(rig.world.resource::<StairsHintConditions>().counter, 2, "无新事件不增长");
    }

    /// 阶段 1 的展示（0xd00）：楼梯装饰物模式停留累计 `> 1.0` 秒且计数为 0。
    #[test]
    fn stage_one_hint_needs_the_stairs_decorator_mode_and_more_than_one_second() {
        let mut rig = Rig::new(STAIRS);
        rig.delta(0.5);
        rig.run();
        rig.run();
        assert!(rig.requests().is_empty(), "恰好 1.0 秒不大于 1.0");
        rig.run();
        let shown = rig.requests();
        assert_eq!(shown.len(), 1);
        assert_eq!(shown[0].id, crate::HintType::StairsPlaced as u8);
        assert_eq!(shown[0].placement, crate::HintPlacement::Cursor);
        let crate::BindingHintDisplay::Alternate(parts) = &shown[0].display else { panic!("Alternate") };
        assert_eq!(parts[0], crate::BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(0x0001)));
        let crate::BindingHintDisplay::Graphic(g) = &parts[1] else { panic!("Graphic") };
        assert_eq!(
            g.images,
            crate::hint_orchestrator::HintGraphicImages::Animation {
                frames: vec!["hint/mouse".to_string(), "hint/lmb".to_string()],
                seconds_per_frame: 0.7,
            }
        );
        assert_eq!(g.height, 100.);
        // 非楼梯模式不累加计时。
        let mut rig = Rig::new(AppMode::Decorator(DecoratorType::Chimney));
        rig.delta(5.0);
        rig.run();
        assert_eq!(rig.world.resource::<StairsHintConditions>().time_in_stairs_mode, 0.0);
        assert!(rig.requests().is_empty());
    }

    /// 阶段 2 只在阶段 1 完成后运行；展示条件是楼梯模式且 counter > 2。
    #[test]
    fn stage_two_runs_only_after_stage_one_and_shows_after_three_placements() {
        let mut rig = Rig::new(STAIRS);
        rig.world.resource_mut::<StairsHintConditions>().counter = 3;
        rig.run();
        // 阶段 1 未完成：只会完成 id 13，不展示阶段 2。
        assert_eq!(rig.completions(), [13]);
        assert!(rig.requests().is_empty());
        rig.profile().stairs_stage1_completed = true;
        rig.run();
        let shown = rig.requests();
        assert_eq!(shown.len(), 1);
        assert_eq!(shown[0].id, crate::HintType::StairsFocus as u8);
        let crate::BindingHintDisplay::Alternate(parts) = &shown[0].display else { panic!("Alternate") };
        assert_eq!(parts[0], crate::BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(0x0101)));
        // counter == 2 不够。
        rig.world.resource_mut::<StairsHintConditions>().counter = 2;
        rig.run();
        assert!(rig.requests().is_empty());
    }

    /// 阶段 2 完成：模式变化 + counter!=0 + ToolSelectionCancelled，且最后一个模式变化回到 Editing(None)。
    #[test]
    fn stage_two_completes_when_the_selection_is_cancelled_back_to_no_subject() {
        use country_core::resources::events::ModeJustChanged;
        let mut rig = Rig::new(AppMode::Editing(EditSubject::None));
        rig.profile().stairs_stage1_completed = true;
        rig.world.resource_mut::<StairsHintConditions>().counter = 1;
        rig.world.send_event(ModeJustChanged { current: AppMode::Path, previous: AppMode::Editing(EditSubject::None) });
        rig.world.send_event(ModeJustChanged { current: AppMode::Editing(EditSubject::None), previous: AppMode::Path });
        rig.world.send_event(ToolSelectionCancelled);
        rig.run();
        assert_eq!(rig.completions(), [14]);
        // 没有取消事件：不完成。
        let mut rig = Rig::new(AppMode::Editing(EditSubject::None));
        rig.profile().stairs_stage1_completed = true;
        rig.world.resource_mut::<StairsHintConditions>().counter = 1;
        rig.world.send_event(ModeJustChanged { current: AppMode::Editing(EditSubject::None), previous: AppMode::Path });
        rig.run();
        assert!(rig.completions().is_empty());
        // counter==0：不完成，且取消事件不被消费。
        let mut rig = Rig::new(AppMode::Editing(EditSubject::None));
        rig.profile().stairs_stage1_completed = true;
        rig.world.send_event(ModeJustChanged { current: AppMode::Editing(EditSubject::None), previous: AppMode::Path });
        rig.world.send_event(ToolSelectionCancelled);
        rig.run();
        assert!(rig.completions().is_empty());
    }

    /// 阶段 3：StairPoint 编辑模式完成 15；阶段 1、2 都完成后悬停楼梯才展示（id 1）。
    #[test]
    fn stage_three_completes_in_stair_point_mode_and_shows_on_hover() {
        let mut rig = Rig::new(AppMode::Editing(EditSubject::StairPoint { decorator_id: 7 }));
        rig.profile().stairs_stage1_completed = true;
        rig.profile().stairs_stage2_completed = true;
        rig.run();
        assert_eq!(rig.completions(), [15]);

        let mut rig = Rig::new(AppMode::Path);
        rig.world.send_event(NotifyHintSystemHoveringOverStairs);
        rig.run();
        assert!(rig.requests().is_empty(), "阶段 1、2 未完成不展示");
        rig.profile().stairs_stage1_completed = true;
        rig.profile().stairs_stage2_completed = true;
        rig.world.send_event(NotifyHintSystemHoveringOverStairs);
        rig.run();
        let shown = rig.requests();
        assert_eq!(shown.len(), 1);
        assert_eq!(shown[0].id, crate::HintType::FocusedEdit as u8);
    }

    /// 阶段 4：开始续建完成 16；否则可续建展示（id 16，frames mouse/lmb）。
    #[test]
    fn stage_four_prefers_completion_over_display() {
        let mut rig = Rig::new(AppMode::Path);
        rig.world.send_event(NotifyHintSystemCanContinueStairs);
        rig.run();
        let shown = rig.requests();
        assert_eq!(shown.len(), 1);
        assert_eq!(shown[0].id, crate::HintType::StairsDetach as u8);
        rig.world.send_event(NotifyHintSystemCanContinueStairs);
        rig.world.send_event(NotifyHintSystemStartedStairsContinue);
        rig.run();
        assert_eq!(rig.completions(), [16]);
        assert!(rig.requests().is_empty());
    }

    /// 阶段 5：脱离事件完成 17；只有"可脱离"事件时展示 `Alternate[Steam(set 0xf+1), Key(bindings[24])]`。
    #[test]
    fn stage_five_completes_on_detach_and_shows_the_snapping_key_otherwise() {
        let mut rig = Rig::new(AppMode::Path);
        rig.profile().key_bindings.bindings[24] = Some(crate::KeyCode::AltLeft);
        rig.world.send_event(NotifyHintSystemStairsCanDetachFromWall);
        rig.run();
        let shown = rig.requests();
        assert_eq!(shown.len(), 1);
        assert_eq!(shown[0].id, crate::HintType::StairsCanDetach as u8);
        assert_eq!(
            shown[0].display,
            crate::BindingHintDisplay::Alternate(vec![
                crate::BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(0x0f01)),
                crate::BindingHintDisplay::Key(Some(crate::KeyCode::AltLeft)),
            ])
        );
        rig.world.send_event(NotifyHintSystemStairsCanDetachFromWall);
        rig.world.send_event(NotifyHintSystemStairsDetachedFromWall);
        rig.run();
        assert_eq!(rig.completions(), [17]);
        assert!(rig.requests().is_empty());
    }
}
