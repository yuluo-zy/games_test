//! hint_disable_snapping（O 0x14227e140，661）。
//!
//! 真逻辑（fresh 0x14227e140）：读 `NotifyHintSystemSnapping`（0x10B），首事件字节
//! ==0 → profile 0x1fc==0 且 state.hint_done==0 且 AppMode==6 → `Steam(packed_set_
//! plus_one)` 后 `alternate(Option<KeyCode>)`（键来自 GameProfile+0x120/+0x128 槽）
//! 写 DisplayHintRequest（+0x48 u16 = 0x0300 → id **3** DisableSnapping、flags 0；
//! +0 字节 placement = `6 + WallShape`：0 Freeform→Cursor / 1 Circle→WorldPoint /
//! 2 Rectangle→Bounds）；字节!=0 → !hint_done 则 TutorialHintComplete(id 3)，hint_done=1。
//!
//! **已重建（2026-10-17，NotifyHintSystemSnapping + GameProfile 0x1fc + SteamInput
//! 落库）**；分歧点：事件载荷字节 / 键槽（0x120/0x128）/ Steam index U（seam）。

use bevy_ecs::prelude::*;

/// 吸附禁用提示条件（O：hints_startup 初值 1B [0]=profile[0x1fc]）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct HintDisableSnappingConditions {
    /// @+0：完成/展示标志（初值 profile[0x1fc]）。
    pub hint_done: bool,
}

/// 吸附键码 seam（O：GameProfile+0x120/+0x128 键槽；槽语义 U → None）。
pub fn snapping_keys_seam() -> Option<crate::KeyCode> {
    None
}

/// hint_disable_snapping（O 0x14227e140）：见模块头。**重建**（2026-10-17）；
/// 分歧点：载荷字节 / 键槽 / Steam index U（seam）。
pub fn hint_disable_snapping(
    profile: Res<country_core::game_profile::GameProfile>,
    mut state: ResMut<HintDisableSnappingConditions>,
    mode: Res<country_core::systems::mode_manager::AppMode>,
    mut events: EventReader<country_core::resources::onboarding::NotifyHintSystemSnapping>,
    mut request: EventWriter<crate::DisplayHintRequest>,
    mut completed: EventWriter<crate::tutorial_completion::TutorialHintComplete>,
) {
    if let Some(_first) = events.read().next() {
        if crate::event_payload_byte_seam() == 0 {
            let shape = match *mode {
                country_core::systems::mode_manager::AppMode::Wall(shape) => shape as u8,
                _ => 0,
            };
            if !profile.disable_snapping_completed && !state.hint_done && mode.native_tag() == 6 {
                let display = crate::BindingHintDisplay::Steam(
                    crate::hint_orchestrator::steam_query(
                        country_core::resources::steam::packed_set_plus_one(0),
                    ),
                )
                .alternate(snapping_keys_seam());
                // O 0x14227e140：`+0x48` u16 = 0x0300 → id 3；`+0` placement =
                // `6 + WallShape`（形状字节选择 Cursor/WorldPoint/Bounds）。
                request.write(
                    crate::DisplayHintRequest::new(
                        display,
                        crate::hint_orchestrator::HintType::DisableSnapping,
                        0,
                    )
                    .with_placement(crate::hint_orchestrator::wall_shape_placement(shape)),
                );
            }
        } else {
            if !state.hint_done {
                completed.write(crate::tutorial_completion::TutorialHintComplete { id: 3 });
            }
            state.hint_done = true;
        }
    }
}
