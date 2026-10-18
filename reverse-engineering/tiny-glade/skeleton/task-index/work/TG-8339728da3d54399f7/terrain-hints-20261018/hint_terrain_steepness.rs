//! B hint_terrain_steepness 0x142283640.
//! Tag0 arms the prompt; nonzero completes tutorial4. Display requires hover.

use bevy_ecs::prelude::*;

/// 地形陡峭提示条件（O：hints_startup 初值 2B [0]=profile[0x1fa],[1]=profile[0x1fa]）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct HintTerrainSteepnessConditions {
    /// Original first byte: tag0/release observed (existing API name).
    pub hint_done: bool,
    /// Original second byte: completion observed.
    pub event_byte: u8,
}

/// hint_terrain_steepness（O 0x142283640）：遍历 NotifyHintSystemTerrainSteepness 事件；
/// 字节==0 → hint_done=1；字节!=0 → event_byte=1 + TutorialHintComplete(id 4)；末位：AppMode==10 且 profile 0x1fa==0 且
/// hint_done 且 event_byte==0 且悬停 tag 非零 → make_lbm_drag_binding_hint 写
/// DisplayHintRequest（id 4，flags 0）。
/// B 原事件首字节由真实 producer 提供；末位读取 HoveredTerrainStrokeId.tag。
pub fn hint_terrain_steepness(
    profile: Res<country_core::game_profile::GameProfile>,
    mut state: ResMut<HintTerrainSteepnessConditions>,
    mode: Res<country_core::systems::mode_manager::AppMode>,
    mut events: EventReader<country_core::resources::onboarding::NotifyHintSystemTerrainSteepness>,
    mut request: EventWriter<crate::DisplayHintRequest>,
    mut completed: EventWriter<crate::tutorial_completion::TutorialHintComplete>,
    hovered: Res<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>,
) {
    for event in events.read() {
        // O：字节==0 → hint_done；!=0 → event_byte=1 + 完成。
        if event.tag == 0 {
            state.hint_done = true;
        } else {
            state.event_byte = 1;
            completed.write(crate::tutorial_completion::TutorialHintComplete { id: 4 });
        }
    }
    if mode.native_tag() == 10
        && !profile.terrain_steepness_completed
        && state.hint_done
        && state.event_byte == 0
        && hovered.tag != 0
    {
        let display = crate::hint_roof_shape::make_lbm_drag_binding_hint();
        request.write(crate::DisplayHintRequest::new(
            display,
            crate::hint_orchestrator::HintType::TerrainSteepness,
            0,
        ));
    }
}
