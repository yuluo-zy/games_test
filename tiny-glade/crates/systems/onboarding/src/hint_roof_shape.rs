//! hint_roof_shape（O：hint_roof_shape 0x142284ed0 / make_lbm_drag_binding_hint
//! 0x142285270）。
//!
//! `hint_roof_shape`：遍历 NotifyHintSystemRoofShape 事件（0x10B）：字节==0 →
//! state.hint_done=1；字节==1 → state.event_byte=1 + TutorialHintComplete(id 7)。
//! 末位（两缓冲耗尽）：profile 0x1ff==0 且 hint_done 且 event_byte!=1 且 `*in_RCX==0`
//! （AppMode 无工具）且 legacy_raycast 命中（码 !=2）且 geo_src_to_edit_subject==2 且
//! PublicWalls::roof_entity 有效且 Roof::is_flat()==0 → make_lbm_drag_binding_hint 写
//! DisplayHintRequest（+0x48 u16 = **0x0700** → id 7、flags 0；+0 = 6 Cursor）。
//! `make_lbm_drag_binding_hint`：together(steam, alternate(Graphic("hint/lmb-move"))).
//!
//! **已重建（2026-10-17，NotifyHintSystemRoofShape + GameProfile + SteamInput 落库）**；
//! 分歧点：geo_src_to_edit_subject / legacy_raycast 命中码（country-core ui_edit_focus
//! 域未落库）→ 展示写面以 seam 门控（默认不触发，登记 U）；roof_entity→wall→is_flat
//! 查表（country-core PublicWalls 未暴露密集表）→ 以 Query<&Roof> 首个 !is_flat 近似。

use bevy_ecs::prelude::*;

/// 屋顶形状提示条件（O：hints_startup 初值 2B [0]=profile[0x1ff],[1]=profile[0x1ff]）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct HintRoofShapeConditions {
    /// @+0：事件/展示标志（初值 profile[0x1ff]）。
    pub hint_done: bool,
    /// @+1：事件字节（初值 profile[0x1ff]）。
    pub event_byte: u8,
}

/// geo_src_to_edit_subject seam（O：country_core::systems::ui_edit_focus 域未落库；
/// hint_roof_shape 读返回 ==2 才展示）。**未落库**：返回 0 → 展示写面不触发，登记 U。
pub fn geo_src_seam() -> u32 {
    0
}

/// make_lbm_drag_binding_hint（O 0x142285270）：together(steam, alternate(Graphic
/// ("hint/lmb-move")))。**重建**（2026-10-17）；分歧点：Steam index / Graphic size/color U。
pub fn make_lbm_drag_binding_hint() -> crate::BindingHintDisplay {
    crate::BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(
        country_core::resources::steam::packed_set_plus_one(0),
    ))
    .together(crate::hint_orchestrator::steam_query(
        country_core::resources::steam::packed_set(0),
    ))
    .alternate(crate::hint_orchestrator::graphic("hint/lmb-move"))
}

/// hint_roof_shape（O 0x142284ed0）：见模块头。**重建**（事件处理 + 完成事件判等）；
/// 展示写面链（geo_src / raycast / roof 表）未落库 → seam 门控，登记 U。
pub fn hint_roof_shape(
    mut state: ResMut<HintRoofShapeConditions>,
    profile: Res<country_core::game_profile::GameProfile>,
    mut events: EventReader<country_core::resources::onboarding::NotifyHintSystemRoofShape>,
    mode: Res<country_core::systems::mode_manager::AppMode>,
    mut completed: EventWriter<crate::tutorial_completion::TutorialHintComplete>,
    mut request: EventWriter<crate::DisplayHintRequest>,
    query: Query<&country_core::resources::roofs::roof_shape::Roof>,
) {
    for _event in events.read() {
        // O（fresh 0x142284ed0）：逐事件分支——字节==0 → hint_done=1；字节!=0 →
        // event_byte=1 + TutorialHintComplete(id 7)。载荷字节 U（seam 常量 0 → 完成
        // 分支不触发，结构判等）。
        if crate::event_payload_byte_seam() == 0 {
            state.hint_done = true;
        } else {
            state.event_byte = 1;
            completed.write(crate::tutorial_completion::TutorialHintComplete { id: 7 });
        }
    }
    // 末位展示门控（O）：profile 0x1ff==0 且 hint_done 且 event_byte!=1 且 AppMode==0
    // 且 legacy_raycast 命中码 !=2 且 geo_src==2 且 roof_entity 有效且 !is_flat。
    let roof_ok = query.iter().next().is_some_and(|roof| !roof.is_flat());
    if !profile.roof_shape_completed
        && state.hint_done
        && state.event_byte != 1
        && mode.native_tag() == 0
        && geo_src_seam() == 2 // U：未落库 → 不触发
        && roof_ok
    {
        request.write(crate::DisplayHintRequest::new(
            make_lbm_drag_binding_hint(),
            crate::hint_orchestrator::HintType::RoofShape,
            0,
        ));
    }
}
