//! hint_unlink_windows（O 0x14227d760，707）。
//!
//! 真逻辑（fresh 0x14227d760）：GameProfile+0x1fd!=0 → 返回。遍历
//! NotifyHintSystemWindow 事件：字节!=0 → TutorialHintComplete(id 8)；字节==0 →
//! `Steam(packed_set_plus_one) → alternate(Option<KeyCode>)` 写 DisplayHintRequest
//! （+0x48 u16 = 0x0800 → id 8，flags 0）。
//!
//! **已重建（2026-10-17，NotifyHintSystemWindow + GameProfile 0x1fd + SteamInput
//! 落库）**；分歧点：事件载荷字节 / 键码 U（seam）。

use bevy_ecs::prelude::*;

/// hint_unlink_windows（O 0x14227d760）：见模块头。**重建**（2026-10-17）。
pub fn hint_unlink_windows(
    profile: Res<country_core::game_profile::GameProfile>,
    mut events: EventReader<country_core::resources::onboarding::NotifyHintSystemWindow>,
    mut request: EventWriter<crate::DisplayHintRequest>,
    mut completed: EventWriter<crate::tutorial_completion::TutorialHintComplete>,
) {
    if profile.unlink_windows_completed {
        return;
    }
    for _event in events.read() {
        // O：事件字节（unit 载荷 U）：!=0 → 完成；==0 → 展示。
        if crate::event_payload_byte_seam() != 0 {
            completed.write(crate::tutorial_completion::TutorialHintComplete { id: 8 });
        } else {
            let display = crate::BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(
                country_core::resources::steam::packed_set_plus_one(0),
            ))
            .alternate(None::<crate::KeyCode>);
            request.write(crate::DisplayHintRequest::new(
                display,
                crate::hint_orchestrator::HintType::UnlinkWindows,
                0,
            ));
        }
    }
}
