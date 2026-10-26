//! hint_unlink_windows（O 0x14227d760，707）。
//!
//! 真逻辑（fresh 0x14227d760）：GameProfile+0x1fd!=0 → 返回。遍历
//! NotifyHintSystemWindow 事件：字节!=0 → TutorialHintComplete(id 8)；字节==0 →
//! `Steam(packed_set_plus_one) → alternate(Option<KeyCode>)` 写 DisplayHintRequest
//! （+0x48 u16 = 0x0800 → id 8，flags 0）。
//!
//! **2026-10-26 复审重写**：事件载荷字节落库（`NotifyHintSystemWindow.flag`），去掉共享常量
//! seam；Steam 动作改为反编译的 set 0xf，键位读 `bindings[24]`（旧版误用 set 0 与 None）。

use bevy_ecs::prelude::*;

/// 原版展示键位：`GameProfile+0x120`（8B）+`+0x128`（4B）= `key_bindings.bindings[24]`
/// （`disable_snapping`，12B/槽）。
const UNLINK_BINDING: usize = 24;
/// 原版 Steam 查询：`packed_set_plus_one(0xf)`（`CONCAT71(.., 0xf)` → `FUN_1409e8d90`）。
const UNLINK_STEAM_ACTION: u8 = 0xf;

/// hint_unlink_windows（O 0x14227d760，707B，4 参数）：
/// `profile+0x1fd != 0` → 返回；逐个 `NotifyHintSystemWindow`：载荷字节 `!=0` →
/// `TutorialHintComplete{id:8}`；`==0` → `Alternate[Steam(set 0xf +1), Key(bindings[24])]`，
/// placement Cursor(6)，id 8、flags 0。
pub fn hint_unlink_windows(
    profile: Res<country_core::game_profile::GameProfile>,
    mut events: EventReader<country_core::resources::onboarding::NotifyHintSystemWindow>,
    mut request: EventWriter<crate::DisplayHintRequest>,
    mut completed: EventWriter<crate::tutorial_completion::TutorialHintComplete>,
) {
    if profile.unlink_windows_completed {
        return;
    }
    for event in events.read() {
        if event.flag != 0 {
            completed.write(crate::tutorial_completion::TutorialHintComplete { id: 8 });
        } else {
            let display = crate::BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(
                country_core::resources::steam::packed_set_plus_one(UNLINK_STEAM_ACTION),
            ))
            .alternate(profile.key_bindings.bindings[UNLINK_BINDING]);
            request.write(crate::DisplayHintRequest::new(
                display,
                crate::hint_orchestrator::HintType::UnlinkWindows,
                0,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::system::RunSystemOnce;
    use country_core::resources::onboarding::NotifyHintSystemWindow;
    use crate::tutorial_completion::TutorialHintComplete;

    fn world() -> World {
        let mut world = World::new();
        world.insert_resource(country_core::game_profile::GameProfile::default());
        world.init_resource::<Events<NotifyHintSystemWindow>>();
        world.init_resource::<Events<TutorialHintComplete>>();
        world.init_resource::<Events<crate::DisplayHintRequest>>();
        world
    }

    /// 同一帧的两个事件各自分支：字节 1 完成、字节 0 展示，顺序保持。
    #[test]
    fn each_event_byte_selects_completion_or_display() {
        let mut world = world();
        world.resource_mut::<country_core::game_profile::GameProfile>().key_bindings.bindings[24] =
            Some(crate::KeyCode::AltLeft);
        world.send_event(NotifyHintSystemWindow { flag: 1 });
        world.send_event(NotifyHintSystemWindow { flag: 0 });
        world.run_system_once(hint_unlink_windows).unwrap();
        let done: Vec<_> = world.resource_mut::<Events<TutorialHintComplete>>().drain().map(|e| e.id).collect();
        assert_eq!(done, [8]);
        let requests: Vec<_> = world.resource_mut::<Events<crate::DisplayHintRequest>>().drain().collect();
        assert_eq!(requests.len(), 1);
        assert_eq!(
            requests[0].display,
            crate::BindingHintDisplay::Alternate(vec![
                crate::BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(0x0f01)),
                crate::BindingHintDisplay::Key(Some(crate::KeyCode::AltLeft)),
            ])
        );
        assert_eq!((requests[0].id, requests[0].flags), (crate::HintType::UnlinkWindows as u8, 0));
        assert_eq!(requests[0].placement, crate::HintPlacement::Cursor);
    }

    #[test]
    fn completed_profile_flag_ignores_every_event() {
        let mut world = world();
        world.resource_mut::<country_core::game_profile::GameProfile>().unlink_windows_completed = true;
        world.send_event(NotifyHintSystemWindow { flag: 0 });
        world.send_event(NotifyHintSystemWindow { flag: 1 });
        world.run_system_once(hint_unlink_windows).unwrap();
        assert!(world.resource::<Events<TutorialHintComplete>>().is_empty());
        assert!(world.resource::<Events<crate::DisplayHintRequest>>().is_empty());
    }
}
