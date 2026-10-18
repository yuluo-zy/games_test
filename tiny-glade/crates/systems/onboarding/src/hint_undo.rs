//! hint_undo（O 0x142284220，1280）。
//!
//! 真逻辑（fresh 0x142284220）：`DigitalActionState::just_pressed()`（动作 U）→ 写
//! TutorialHintComplete(id 2)。读 `NotifyHintSystemSomethingWasErased`（0x10B）有事件
//! → state.flag=1。Query 焦点字节==2 → timer+=dt；timer>10.0 → 完成；再 just_pressed
//! → 完成。末尾：state.flag==1 且 profile 0x1fe!=1 → `Steam(q1).adjacent(Steam(q2))`
//! → `adjacent(Option<KeyCode>)` → `alternate` 写 DisplayHintRequest（+0x48 u16 =
//! 0x0200 → id **2** Undo、flags 0、placement Cursor(6)）。常量：+0xfc/+0x104 键码
//! （GameKeyBindings 槽 21 域）、阈值 10.0、tag 2、标志 0x1fe。
//!
//! **已重建（2026-10-17，NotifyHintSystemSomethingWasErased + GameProfile 0x1fe +
//! SteamInput 落库）**；分歧点：just_pressed 动作 / 键码槽（0xfc/0x104）U → seam。

use bevy_ecs::prelude::*;

/// 撤销提示条件（O：hints_startup 初值 8B 清零：u32 timer@+0=0，u8 flag@+4=0）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct UndoHintConditions {
    /// @+0：计时（f32/u32）。
    pub timer: f32,
    /// @+4：事件标志。
    pub flag: u8,
}

/// 撤销动作 seam（O：`DigitalActionState::just_pressed()`；动作 U → 本重建 false）。
pub fn undo_just_pressed_seam() -> bool {
    false
}

/// 撤销提示键码 seam（O：GameProfile+0xfc/+0x104 键码；槽语义 U → None）。
pub fn undo_keys_seam() -> (Option<crate::KeyCode>, Option<crate::KeyCode>) {
    (None, None)
}

/// hint_undo（O 0x142284220）：见模块头。**重建**（2026-10-17）；分歧点：just_pressed
/// 动作 / 键码槽 U（seam）。
pub fn hint_undo(
    profile: Res<country_core::game_profile::GameProfile>,
    mut state: ResMut<UndoHintConditions>,
    mut erased: EventReader<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,
    time: Res<country_core::resources::time::GameTime>,
    mut completed: EventWriter<crate::tutorial_completion::TutorialHintComplete>,
    mut request: EventWriter<crate::DisplayHintRequest>,
    query: Query<&crate::hint_focused_edit::FocusSubjectByte>,
) {
    if undo_just_pressed_seam() {
        completed.write(crate::tutorial_completion::TutorialHintComplete { id: 2 });
    }
    if erased.read().next().is_some() {
        state.flag = 1;
    }
    if query.iter().next().is_some_and(|b| b.0 == 2) {
        state.timer += time.delta;
        if state.timer > 10.0 {
            completed.write(crate::tutorial_completion::TutorialHintComplete { id: 2 });
        }
        if undo_just_pressed_seam() {
            completed.write(crate::tutorial_completion::TutorialHintComplete { id: 2 });
        }
    }
    if state.flag == 1 && !profile.undo_completed {
        let (k1, k2) = undo_keys_seam();
        let display = crate::BindingHintDisplay::Steam(
            crate::hint_orchestrator::steam_query(
                country_core::resources::steam::packed_set_plus_one(0),
            ),
        )
        .adjacent(crate::hint_orchestrator::steam_query(
            country_core::resources::steam::packed_set_plus_one(0),
        ))
        .adjacent(k1)
        .alternate(k2);
        request.write(crate::DisplayHintRequest::new(
            display,
            crate::hint_orchestrator::HintType::Undo,
            0,
        ));
    }
}
