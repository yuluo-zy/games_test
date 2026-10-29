//! hint_undo（O 0x142284220，1280）。
//!
//! 真逻辑（fresh 0x142284220）：`DigitalActionState::just_pressed()`（动作 U）→ 写
//! TutorialHintComplete(id 2)。读 `NotifyHintSystemSomethingWasErased`（0x10B）有事件
//! → state.flag=1。Query 当前提示 HintType==Undo → timer+=dt；timer>10.0 → 完成；再 just_pressed
//! → 完成。末尾：state.flag==1 且 profile 0x1fe!=1 → `Steam(q1).adjacent(Steam(q2))`
//! → `adjacent(Option<KeyCode>)` → `alternate` 写 DisplayHintRequest（+0x48 u16 =
//! 0x0200 → id **2** Undo、flags 0、placement Cursor(6)）。常量：+0xfc/+0x104 键码
//! （GameKeyBindings 槽 21 域）、阈值 10.0、tag 2、标志 0x1fe。
//!
//! **2026-10-26 复审重写**：去掉 `undo_just_pressed_seam`/`undo_keys_seam` 常量 seam，
//! 读真实 `MainActionSet` 与 `GameProfile.key_bindings`；展示树改为反编译的
//! `Alternate[Adjacent[Steam,Steam],Adjacent[Key,Key]]`；焦点分支内第二次判定读 redo。

use bevy_ecs::prelude::*;

/// 撤销提示条件（O：hints_startup 初值 8B 清零：u32 timer@+0=0，u8 flag@+4=0）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct UndoHintConditions {
    /// @+0：计时（f32/u32）。
    pub timer: f32,
    /// @+4：事件标志。
    pub flag: u8,
}

/// `MainActionSet.digital` 槽号：`DIGITAL_NAMES[16]="undo"`、`[17]="redo"`。原版以
/// `MainActionSet+0x80/+0x88`（每槽 8B）读取（`FUN_14093fa90` = `DigitalActionState::
/// just_pressed`：JustPressed→真，Repeating→timer<=0，其余假）。
const UNDO_SLOT: usize = 16;
const REDO_SLOT: usize = 17;
/// GameProfile.key_bindings.bindings 下标：原版读 `+0xfc`（12B）与 `+0x108`（12B），即
/// 21 号 undo 与 22 号 redo（`SerializedKeyBindings` 次序，12B/槽）。
const UNDO_BINDING: usize = 21;
const REDO_BINDING: usize = 22;
/// `_UNK_1429345a8`（RNE 实读 0x41200000）：焦点主体停留 10 秒后自动完成。
const AUTO_COMPLETE_SECONDS: f32 = 10.0;

/// hint_undo（O 0x142284220，1280B，8 参数）。逐语句对应反编译：
/// 1. `just_pressed(MainActionSet+0x80 = undo)` → `TutorialHintComplete{id:2}`；
/// 2. `SomethingWasErased` 读者有未读事件 → 整个读者推进到末尾，`state.flag=1`；
/// 3. 当前提示 HintType == Undo → `timer += GameTime.delta`，`timer > 10.0` → 完成；再
///    `just_pressed(MainActionSet+0x88 = redo)` → 完成（**不是**再读 undo）；
/// 4. `flag==1 && profile+0x1fe != 1` → 展示：`Alternate[Adjacent[Steam(set2+1),
///    Steam(set3+1)], Adjacent[Key(bindings[21]), Key(bindings[22])]]`，placement
///    Cursor(6)，id 2、flags 0。
pub fn hint_undo(
    profile: Res<country_core::game_profile::GameProfile>,
    actions: Res<country_core::input::action_set::MainActionSet>,
    mut state: ResMut<UndoHintConditions>,
    mut erased: EventReader<
        country_core::resources::onboarding::NotifyHintSystemSomethingWasErased,
    >,
    time: Res<country_core::resources::time::GameTime>,
    mut completed: EventWriter<crate::tutorial_completion::TutorialHintComplete>,
    mut request: EventWriter<crate::DisplayHintRequest>,
    query: Query<&crate::HintType>,
) {
    if actions.digital[UNDO_SLOT].just_pressed() {
        completed.write(crate::tutorial_completion::TutorialHintComplete { id: 2 });
    }
    // 原版先算未读数并把读者游标推到末尾；`.count()` 与之等价（不能 `.next()` 只读一个）。
    if erased.read().count() != 0 {
        state.flag = 1;
    }
    if query.iter().next() == Some(&crate::HintType::Undo) {
        state.timer += time.delta;
        if AUTO_COMPLETE_SECONDS < state.timer {
            completed.write(crate::tutorial_completion::TutorialHintComplete { id: 2 });
        }
        if actions.digital[REDO_SLOT].just_pressed() {
            completed.write(crate::tutorial_completion::TutorialHintComplete { id: 2 });
        }
    }
    if state.flag == 1 && !profile.undo_completed {
        let bindings = &profile.key_bindings.bindings;
        let steam = crate::BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(
            country_core::resources::steam::packed_set_plus_one(2),
        ))
        .adjacent(crate::hint_orchestrator::steam_query(
            country_core::resources::steam::packed_set_plus_one(3),
        ));
        let keys = crate::BindingHintDisplay::Key(bindings[UNDO_BINDING])
            .adjacent(bindings[REDO_BINDING]);
        request.write(crate::DisplayHintRequest::new(
            steam.alternate(keys),
            crate::hint_orchestrator::HintType::Undo,
            0,
        ));
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::system::RunSystemOnce;
    use country_core::input::action_set::{DigitalActionPhase, DigitalActionState, MainActionSet};
    use crate::tutorial_completion::TutorialHintComplete;

    fn world() -> World {
        let mut world = World::new();
        world.insert_resource(country_core::game_profile::GameProfile::default());
        world.insert_resource(MainActionSet::default());
        world.insert_resource(UndoHintConditions::default());
        world.insert_resource(country_core::resources::time::GameTime::default());
        world.init_resource::<Events<
            country_core::resources::onboarding::NotifyHintSystemSomethingWasErased,
        >>();
        world.init_resource::<Events<TutorialHintComplete>>();
        world.init_resource::<Events<crate::DisplayHintRequest>>();
        world
    }
    fn press(world: &mut World, slot: usize, phase: DigitalActionPhase, timer: f32) {
        world.resource_mut::<MainActionSet>().digital[slot] =
            DigitalActionState { phase, timer_seconds: timer };
    }
    fn run(world: &mut World) {
        world.run_system_once(hint_undo).unwrap();
    }
    fn completions(world: &mut World) -> usize {
        world.resource_mut::<Events<TutorialHintComplete>>().drain().filter(|e| e.id == 2).count()
    }

    /// 原版第一句读 MainActionSet+0x80（槽 16 = undo）的 `just_pressed`，与焦点主体无关。
    #[test]
    fn undo_just_pressed_completes_and_held_or_other_slots_do_not() {
        let mut world = world();
        press(&mut world, UNDO_SLOT, DigitalActionPhase::Held, 0.3);
        press(&mut world, REDO_SLOT, DigitalActionPhase::JustPressed, 0.0);
        run(&mut world);
        assert_eq!(completions(&mut world), 0, "Held 与 redo 槽（焦点主体≠2）都不完成");
        press(&mut world, UNDO_SLOT, DigitalActionPhase::JustPressed, 0.0);
        run(&mut world);
        assert_eq!(completions(&mut world), 1);
        // Repeating 仅在 timer<=0 时算 just_pressed（FUN_14093fa90 的 ucomiss 分支）。
        press(&mut world, UNDO_SLOT, DigitalActionPhase::Repeating, 0.0);
        run(&mut world);
        assert_eq!(completions(&mut world), 1);
        press(&mut world, UNDO_SLOT, DigitalActionPhase::Repeating, 0.1);
        run(&mut world);
        assert_eq!(completions(&mut world), 0);
    }

    /// 当前提示 HintType==Undo：计时 `> 10.0`（严格大于）完成；分支内第二次判定读 **redo** 槽。
    #[test]
    fn focus_branch_times_out_after_ten_seconds_and_completes_on_redo() {
        let mut world = world();
        world.spawn(crate::HintType::Undo);
        world.resource_mut::<country_core::resources::time::GameTime>().delta = 5.0;
        run(&mut world);
        run(&mut world);
        assert_eq!(world.resource::<UndoHintConditions>().timer, 10.0);
        assert_eq!(completions(&mut world), 0, "恰好 10.0 不大于 10.0");
        run(&mut world);
        assert_eq!(completions(&mut world), 1);
        world.resource_mut::<UndoHintConditions>().timer = 0.0;
        world.resource_mut::<country_core::resources::time::GameTime>().delta = 0.0;
        press(&mut world, REDO_SLOT, DigitalActionPhase::JustPressed, 0.0);
        run(&mut world);
        assert_eq!(completions(&mut world), 1, "redo just_pressed 在焦点分支内完成");
    }

    #[test]
    fn other_hint_type_never_advances_the_timer() {
        let mut world = world();
        world.spawn(crate::HintType::FocusedEdit);
        world.resource_mut::<country_core::resources::time::GameTime>().delta = 60.0;
        press(&mut world, REDO_SLOT, DigitalActionPhase::JustPressed, 0.0);
        run(&mut world);
        assert_eq!(world.resource::<UndoHintConditions>().timer, 0.0);
        assert_eq!(completions(&mut world), 0);
    }

    /// 展示树与键位来自真实配置：undo=bindings[21]，redo=bindings[22]，Steam 为 set2/set3 +1。
    #[test]
    fn display_tree_is_alternate_of_adjacent_steam_pair_and_adjacent_key_pair() {
        let mut world = world();
        {
            let mut profile = world.resource_mut::<country_core::game_profile::GameProfile>();
            profile.key_bindings.bindings[21] = Some(crate::KeyCode::KeyU);
            profile.key_bindings.bindings[22] = None;
        }
        world.send_event_batch([
            country_core::resources::onboarding::NotifyHintSystemSomethingWasErased::default(),
            country_core::resources::onboarding::NotifyHintSystemSomethingWasErased::default(),
        ]);
        run(&mut world);
        assert_eq!(world.resource::<UndoHintConditions>().flag, 1);
        let requests: Vec<_> = world.resource_mut::<Events<crate::DisplayHintRequest>>().drain().collect();
        assert_eq!(requests.len(), 1);
        let request = &requests[0];
        assert_eq!(request.placement, crate::HintPlacement::Cursor);
        assert_eq!((request.id, request.flags), (crate::HintType::Undo as u8, 0));
        let crate::BindingHintDisplay::Alternate(parts) = &request.display else { panic!("Alternate") };
        assert_eq!(parts.len(), 2);
        let crate::BindingHintDisplay::Adjacent(steam) = &parts[0] else { panic!("Adjacent steam") };
        assert_eq!(
            steam,
            &vec![
                crate::BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(0x0201)),
                crate::BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(0x0301)),
            ]
        );
        assert_eq!(
            parts[1],
            crate::BindingHintDisplay::Adjacent(vec![
                crate::BindingHintDisplay::Key(Some(crate::KeyCode::KeyU)),
                crate::BindingHintDisplay::Key(None),
            ])
        );
    }

    /// profile+0x1fe（undo_completed）为真时不再展示；事件标志仍保留。
    #[test]
    fn completed_profile_flag_suppresses_the_display_but_not_the_flag() {
        let mut world = world();
        world.resource_mut::<country_core::game_profile::GameProfile>().undo_completed = true;
        world.send_event(country_core::resources::onboarding::NotifyHintSystemSomethingWasErased::default());
        run(&mut world);
        assert_eq!(world.resource::<UndoHintConditions>().flag, 1);
        assert!(world.resource::<Events<crate::DisplayHintRequest>>().is_empty());
    }
}