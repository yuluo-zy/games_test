//! exponential_back_off（O 0x142284800，1294）：提示重复触发的指数退避。
//!
//! 真逻辑（evidence/batch3-core.txt fresh 反编译）：
//! - `Local<TimeSinceMouseMoved>` 累加 dt；`CursorMovedScreenSpace` 有事件 → 归零。
//! - 每帧 `GameProfile[0x1f0] += dt`（累加器；仅当鼠标事件或 local<30.0 时）。
//! - 两对 (计时,冷却)：A=0x1e0/0x1e4、B=0x1e8/0x1ec；冷却缺省 720.0（0x44340000）。
//! - 对 A：读事件（**读者类型 U**，本重建以 DisplayHintRequest 承载）→ 计时归零、
//!   `cooldown=clamp(old*1.5,720,10800)` 写回、写 WriteSettingsFileCmd。
//! - 对 B：读 `ModeJustChanged`（0x28B 步长，首载荷工具 id ∈ {1,2} 才触发）→ 同上。
//! - 标志 A（profile[0x1f4]）/ B（profile[0x1f5]）：`local<10.0` 才计时+=dt、
//!   `clamp[0,cooldown]`（cooldown<0 走 f32::clamp panic）；`timer>=cooldown` →
//!   写 WriteSettingsFileCmd + 清对应条件资源（10B）+ 清标志。
//!
//! **已重建（2026-10-17，coregap-2 GameProfile 计时区 + WriteSettingsFileCmd 落库）**；
//! 分歧点：对 A 读者事件类型 U（候选 DisplayHintRequest，stride 无取证）。

use bevy_ecs::prelude::*;
use country_core::game_profile::TutorialCooldowns;
use country_core::resources::time::GameTime;

/// 自上次鼠标移动的时间（O：`Local<TimeSinceMouseMoved>`，f32 累加器）。
#[derive(Clone, Copy, Debug, Default)]
pub struct TimeSinceMouseMoved(pub f32);

/// exponential_back_off（O 0x142284800）：见模块头真逻辑。**重建**（2026-10-17）。
pub fn exponential_back_off(
    mut local: Local<TimeSinceMouseMoved>,
    mut mouse: EventReader<country_core::resources::window::CursorMovedScreenSpace>,
    time: Res<GameTime>,
    mut profile: ResMut<country_core::game_profile::GameProfile>,
    mut writer: EventWriter<country_core::resources::events::WriteSettingsFileCmd>,
    mut trigger_a: EventReader<crate::DisplayHintRequest>,
    mut mode_b: EventReader<country_core::resources::events::ModeJustChanged>,
    mut deselect: ResMut<crate::hint_deselect_tool::DeselectToolHintConditions>,
    mut focused: ResMut<crate::hint_focused_edit::FocusedEditHintConditions>,
) {
    let dt = time.delta;
    local.0 += dt;
    // O：鼠标事件 → local 归零且本帧累加 profile[0x1f0]；无事件仅 local<30.0 累加。
    let mouse_moved = mouse.read().next().is_some();
    let accumulate = if mouse_moved {
        local.0 = 0.0;
        true
    } else {
        local.0 < 30.0
    };
    if accumulate {
        profile.tutorial_cooldowns.time_since_mouse_moved += dt;
    }
    let flag_a = profile.tutorial; // 0x1f4
    let flag_b = profile.focused_edit_completed; // 0x1f5

    // ---- 对 A（0x1e0/0x1e4，tutorial 标志 0x1f4）----
    let mut cooldown_a = profile.tutorial_cooldowns.cooldown_a;
    if cooldown_a == 0.0 && !cooldown_a.is_nan() {
        cooldown_a = TutorialCooldowns::DEFAULT_COOLDOWN;
        profile.tutorial_cooldowns.cooldown_a = cooldown_a;
    }
    if trigger_a.read().next().is_some() {
        profile.tutorial_cooldowns.timer_a = 0.0;
        cooldown_a = (cooldown_a * 1.5).clamp(720.0, 10800.0);
        profile.tutorial_cooldowns.cooldown_a = cooldown_a;
        writer.write(country_core::resources::events::WriteSettingsFileCmd);
    }
    if flag_a {
        if local.0 < 10.0 {
            let mut timer = profile.tutorial_cooldowns.timer_a + dt;
            timer = timer.clamp(0.0, cooldown_a); // cooldown<0 → f32::clamp panic（O）
            profile.tutorial_cooldowns.timer_a = timer;
        }
        if cooldown_a <= profile.tutorial_cooldowns.timer_a {
            writer.write(country_core::resources::events::WriteSettingsFileCmd);
            *deselect = crate::hint_deselect_tool::DeselectToolHintConditions::default();
            profile.tutorial = false;
        }
    }

    // ---- 对 B（0x1e8/0x1ec，focused_edit 标志 0x1f5）----
    let mut cooldown_b = profile.tutorial_cooldowns.cooldown_b;
    if cooldown_b == 0.0 && !cooldown_b.is_nan() {
        cooldown_b = TutorialCooldowns::DEFAULT_COOLDOWN;
        profile.tutorial_cooldowns.cooldown_b = cooldown_b;
    }
    // O：B 读者（ModeJustChanged 0x28B）末事件当前模式 tag ∈ {1,2}（`*piVar11-1U<2`）。
    let mode_triggered = mode_b
        .read()
        .last()
        .is_some_and(|e| (1..=2).contains(&e.current.native_tag()));
    if mode_triggered {
        profile.tutorial_cooldowns.timer_b = 0.0;
        cooldown_b = (cooldown_b * 1.5).clamp(720.0, 10800.0);
        profile.tutorial_cooldowns.cooldown_b = cooldown_b;
        writer.write(country_core::resources::events::WriteSettingsFileCmd);
    }
    if flag_b {
        if local.0 < 10.0 {
            let mut timer = profile.tutorial_cooldowns.timer_b + dt;
            timer = timer.clamp(0.0, cooldown_b);
            profile.tutorial_cooldowns.timer_b = timer;
        }
        if cooldown_b <= profile.tutorial_cooldowns.timer_b {
            writer.write(country_core::resources::events::WriteSettingsFileCmd);
            *focused = crate::hint_focused_edit::FocusedEditHintConditions::default();
            profile.focused_edit_completed = false;
        }
    }
}

/// inline helper（O 133）：冷却已过（对 A：timer >= cooldown）。语义 I。
pub fn has_cooldown_elapsed(profile: &country_core::game_profile::GameProfile) -> bool {
    let tc = &profile.tutorial_cooldowns;
    tc.timer_a >= tc.cooldown_a
}

/// inline helper（O 137）：推进退避计时（对 A：clamp 到 [0, cooldown]）。语义 I。
pub fn advance_exp_backoff(profile: &mut country_core::game_profile::GameProfile) {
    let tc = &mut profile.tutorial_cooldowns;
    tc.timer_a = tc.timer_a.clamp(0.0, tc.cooldown_a);
}

#[cfg(test)]
mod canonical_mode_backoff_tests {
    use super::*;
    use country_core::resources::events::{ModeJustChanged, WriteSettingsFileCmd};
    use country_core::systems::mode_manager::{
        send_mode_just_changed_event, AppMode, EditSubject, PreviousAppMode,
    };

    #[test]
    fn published_latest_mode_controls_backoff_once_across_both_event_buffers() {
        let mut world = World::new();
        world.init_resource::<country_core::game_profile::GameProfile>();
        world.init_resource::<GameTime>();
        world.init_resource::<PreviousAppMode>();
        world.init_resource::<crate::hint_deselect_tool::DeselectToolHintConditions>();
        world.init_resource::<crate::hint_focused_edit::FocusedEditHintConditions>();
        world.init_resource::<Events<ModeJustChanged>>();
        world.init_resource::<Events<WriteSettingsFileCmd>>();
        world.init_resource::<Events<country_core::resources::window::CursorMovedScreenSpace>>();
        world.init_resource::<Events<crate::DisplayHintRequest>>();
        world.resource_mut::<GameTime>().delta = 0.0;
        world.resource_mut::<country_core::game_profile::GameProfile>().tutorial_cooldowns.cooldown_b = 800.0;
        let mut publish = Schedule::default();
        publish.add_systems(send_mode_just_changed_event);
        let mut consume = Schedule::default();
        consume.add_systems(exponential_back_off);

        world.insert_resource(AppMode::Editing(EditSubject::Roof(0x1234_5678_90ab_cdef)));
        publish.run(&mut world);
        world.resource_mut::<Events<ModeJustChanged>>().update();
        world.insert_resource(AppMode::Path);
        publish.run(&mut world);
        consume.run(&mut world);
        assert_eq!(world.resource::<country_core::game_profile::GameProfile>().tutorial_cooldowns.cooldown_b, 800.0);
        assert_eq!(world.resource::<Events<WriteSettingsFileCmd>>().len(), 0);

        world.insert_resource(AppMode::Editing(EditSubject::Wall(0xfeed_dead_beef_1234)));
        publish.run(&mut world);
        consume.run(&mut world);
        assert_eq!(world.resource::<country_core::game_profile::GameProfile>().tutorial_cooldowns.cooldown_b, 1200.0);
        assert_eq!(world.resource::<Events<WriteSettingsFileCmd>>().len(), 1);
        consume.run(&mut world);
        assert_eq!(world.resource::<country_core::game_profile::GameProfile>().tutorial_cooldowns.cooldown_b, 1200.0);
        assert_eq!(world.resource::<Events<WriteSettingsFileCmd>>().len(), 1);
    }
}
