//! hint_terrain_steepness（O 0x142283640，754）。
//!
//! 真逻辑：工具==10 且 GameProfile+0x1fa==0 且事件字节==1 且 [+1]==0 且存在实体 →
//! make_lbm_drag_binding_hint 写 DisplayHintRequest（+0x48 u16 = 0x0400 → id 4，flags 0）；
//! 事件字节!=0 且 [+1] → 写 TutorialHintComplete（tag 4）。

use bevy_ecs::prelude::*;

/// 地形陡峭提示条件（O：hints_startup 初值 2B [0]=profile[0x1fa],[1]=profile[0x1fa]）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct HintTerrainSteepnessConditions {
    pub hint_done: bool,
    pub event_byte: u8,
}

/// hint_terrain_steepness（O 0x142283640）：遍历 NotifyHintSystemTerrainSteepness 事件；
/// 字节==0 → hint_done=1；字节!=0 → event_byte=1 + TutorialHintComplete(id 4)（**载荷
/// 字节 U**，seam 常量 0 → 完成分支不触发）；末位：AppMode==10 且 profile 0x1fa==0 且
/// hint_done 且 event_byte==0 且有实体 → make_lbm_drag_binding_hint 写
/// DisplayHintRequest（id 4，flags 0）。
/// **重建（2026-10-17，NotifyHintSystemTerrainSteepness + GameProfile 落库）**；
/// 分歧点：事件载荷字节 U。
pub fn hint_terrain_steepness(
    profile: Res<country_core::game_profile::GameProfile>,
    mut state: ResMut<HintTerrainSteepnessConditions>,
    mode: Res<country_core::systems::mode_manager::AppMode>,
    mut events: EventReader<country_core::resources::onboarding::NotifyHintSystemTerrainSteepness>,
    mut request: EventWriter<crate::DisplayHintRequest>,
    mut completed: EventWriter<crate::tutorial_completion::TutorialHintComplete>,
    query: Query<Entity>,
) {
    for _event in events.read() {
        // O：字节==0 → hint_done；!=0 → event_byte=1 + 完成。
        if crate::event_payload_byte_seam() == 0 {
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
        && !query.is_empty()
    {
        let display = crate::hint_roof_shape::make_lbm_drag_binding_hint();
        request.write(crate::DisplayHintRequest::new(
            display,
            crate::hint_orchestrator::HintType::TerrainSteepness,
            0,
        ));
    }
}
