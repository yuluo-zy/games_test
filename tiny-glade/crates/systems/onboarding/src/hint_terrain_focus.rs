//! B hint_terrain_focus 0x142283980: tag2 completes focus, hover gates display.
//!
//! 真逻辑：工具==10 且 GameProfile+0x1fb==0 且悬停 tag 非零且条件 hint_done==0 →
//! make_focus_or_camera_binding_hint 写 DisplayHintRequest（+0x48 u16 = 0x0100 →
//! id **1** FocusedEdit 通用 focus-or-camera 文本，flags 0）；
//! 事件字节==2 → 写 TutorialHintComplete（tag 5）。

use bevy_ecs::prelude::*;

/// 地形焦点提示条件（O：hints_startup 初值 1B [0]=profile[0x1fb]）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct HintFocusTerrainConditions {
    pub hint_done: bool,
}

/// hint_terrain_focus（O 0x142283980）：遍历 NotifyHintSystemTerrain 事件；任一事件
/// 字节==2 → hint_done=1 + TutorialHintComplete(id 5)；末位：AppMode==10 且 profile 0x1fb==0 且悬停 tag 非零且 hint_done==0 →
/// make_focus_or_camera_binding_hint 写 DisplayHintRequest（id 1，flags 0）。
/// B 原事件首字节由真实 producer 提供；末位读取 HoveredTerrainStrokeId.tag。
pub fn hint_terrain_focus(
    profile: Res<country_core::game_profile::GameProfile>,
    mut state: ResMut<HintFocusTerrainConditions>,
    mode: Res<country_core::systems::mode_manager::AppMode>,
    mut events: EventReader<country_core::resources::onboarding::NotifyHintSystemTerrain>,
    mut request: EventWriter<crate::DisplayHintRequest>,
    mut completed: EventWriter<crate::tutorial_completion::TutorialHintComplete>,
    hovered: Res<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>,
) {
    for event in events.read() {
        // O：tag2 completes focus, including repeated notifications.
        if event.tag == 2 {
            completed.write(crate::tutorial_completion::TutorialHintComplete { id: 5 });
            state.hint_done = true;
        }
    }
    if mode.native_tag() == 10
        && !profile.terrain_focus_completed
        && hovered.tag != 0
        && !state.hint_done
    {
        let display = crate::hint_clutter_focus_edit::make_focus_or_camera_binding_hint(&profile);
        request.write(crate::DisplayHintRequest::new(
            display,
            crate::hint_orchestrator::HintType::FocusedEdit,
            0,
        ));
    }
}
