//! hint_terrain（O 0x142283cc0，844）。
//!
//! 真逻辑：工具==10（Terrain）且 GameProfile+0x1f9==0 且事件计数>1 且存在实体且事件
//! 字节[+1]==0 → alternate(Option<KeyCode>) 写 DisplayHintRequest（+0x48 u16 = 0x0600
//! → id 6 Terrain，flags 0）；
//! 事件字节==1 且 [+1]==0 → 写 TutorialHintComplete（tag 6）。

use bevy_ecs::prelude::*;

/// 地形分层提示条件（O：hints_startup 初值 9B {u64 count@+0=0, u8 hint_done@+8=
/// profile[0x1f9]}；terrain_layering count++/`1<count` 门控、byte@+8 事件标志）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct HintTerrainConditions {
    /// @+0：地形事件计数（u64，O：每事件 +1，`1<count` 门控）。
    pub count: u64,
    /// @+8：事件/完成标志（初值 profile[0x1f9]）。
    pub hint_done: bool,
    /// @+9：保留字节（O 未写）。
    pub event_byte: u8,
}

/// hint_terrain_layering（O 0x142283cc0）：遍历 NotifyHintSystemTerrain 事件；count++；
/// 事件字节==1 且 byte8==0 → TutorialHintComplete(6)、byte8=1（**载荷字节 U**，本重建
/// seam 常量 0 → 完成分支不触发）；末位：AppMode==10 且 profile 0x1f9==0 且 count>1 且
/// 有实体且 byte8==0 → Steam+Key alternate 写 DisplayHintRequest（id 6，flags 0）。
/// **重建（2026-10-17，NotifyHintSystemTerrain + GameProfile 落库）**；分歧点：事件载荷
/// 字节 U。
pub fn hint_terrain_layering(
    profile: Res<country_core::game_profile::GameProfile>,
    mut state: ResMut<HintTerrainConditions>,
    mode: Res<country_core::systems::mode_manager::AppMode>,
    mut events: EventReader<country_core::resources::onboarding::NotifyHintSystemTerrain>,
    mut request: EventWriter<crate::DisplayHintRequest>,
    mut completed: EventWriter<crate::tutorial_completion::TutorialHintComplete>,
    query: Query<Entity>,
) {
    for _event in events.read() {
        // O：事件字节（country-core unit，载荷字节 U）；字节==1 且 byte8==0 → 完成。
        if crate::event_payload_byte_seam() == 1 && !state.hint_done {
            completed.write(crate::tutorial_completion::TutorialHintComplete { id: 6 });
            state.hint_done = true;
        }
        state.count += 1;
    }
    if mode.native_tag() == 10
        && !profile.terrain_layering_completed
        && 1 < state.count
        && !query.is_empty()
        && !state.hint_done
    {
        let display = crate::BindingHintDisplay::Steam(
            crate::hint_orchestrator::steam_query(
                country_core::resources::steam::packed_set_plus_one(0),
            ),
        )
        .alternate(None::<crate::KeyCode>);
        request.write(crate::DisplayHintRequest::new(
            display,
            crate::hint_orchestrator::HintType::Terrain,
            0,
        ));
    }
}
