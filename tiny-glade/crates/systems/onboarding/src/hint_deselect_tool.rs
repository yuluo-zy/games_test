//! hint_deselect_tool（O 0x1422853c0，1116）。
//!
//! 真逻辑（fresh 0x1422853c0）：GameProfile+0x1f4!=0 → 返回。读事件：reader A
//! （ToolWasJustUsed，0x10B）有事件 → state@+9=1；reader B（0x28B，U）有事件 →
//! state@+9=0 + reader C（0x10B，U）事件字节==0 → state@+8=1 + TutorialHintComplete(0)；
//! reader D/E（0x10B，U）事件 → count++。AppMode==6 且形状字节!=0 且 count!=0 →
//! state@+9=1。AppMode::is_tool_selected() 且 state@+8==0 且 `DigitalActionState::pressed()`
//! 非按下且 state@+9==1 → make_focus_or_camera_binding_hint；DisplayHintRequest：
//! `+0x48` u16 = **0x0000** → id 0 DeselectTool、flags 0；`+0` placement =
//! `6 + WallShape`（Freeform→Cursor / Circle→WorldPoint / Rectangle→Bounds）。
//!
//! **已重建（2026-10-17，NotifyHintSystemToolWasJustUsed + GameProfile 0x1f4 +
//! AppMode 落库 + DisplayHintRequest 布局判等）**；分歧点：reader B/C/D/E 事件类型 U
//! （seam `DeselectReset`）、pressed 动作 U、placement 7/8 payload 来源 U。

use bevy_ecs::prelude::*;

/// 取消选择提示条件（O：hints_startup 初值 10B {u64 count@+0=0, u8 hint_done@+8=
/// profile[0x1f4], u8 state@+9=0}；deselect_tool 置 state@+9）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct DeselectToolHintConditions {
    /// @+0：事件计数（u64，O：reader 事件 → +1，`!=0` 门控）。
    pub count: u64,
    /// @+8：完成/展示标志（初值 profile[0x1f4]）。
    pub hint_done: bool,
    /// @+9：状态字节（O：deselect_tool 置 1）。
    pub state: u8,
}

/// reader B/C/D/E seam（O：0x28B 复位读者 + 0x10B 计数读者——类型 U，seam unit）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeselectReset;

/// pressed seam（O：`DigitalActionState::pressed()`；动作 U → 本重建 false）。
pub fn deselect_pressed_seam() -> bool {
    false
}

/// hint_deselect_tool（O 0x1422853c0）：见模块头。**重建**（2026-10-17）；
/// 分歧点：reader B/C/D/E / pressed 动作 U（seam）。
pub fn hint_deselect_tool(
    profile: Res<country_core::game_profile::GameProfile>,
    mut state: ResMut<DeselectToolHintConditions>,
    mode: Res<country_core::systems::mode_manager::AppMode>,
    mut tool_used: EventReader<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,
    mut reset: EventReader<DeselectReset>,
    mut completed: EventWriter<crate::tutorial_completion::TutorialHintComplete>,
    mut request: EventWriter<crate::DisplayHintRequest>,
) {
    if profile.tutorial {
        return;
    }
    // reader A：工具刚使用 → state@+9=1。
    if tool_used.read().next().is_some() {
        state.state = 1;
    }
    // reader B：复位事件 → state@+9=0；reader C 字节==0 → done + 完成。
    if reset.read().next().is_some() {
        state.state = 0;
        if crate::event_payload_byte_seam() == 0 {
            state.hint_done = true;
            completed.write(crate::tutorial_completion::TutorialHintComplete { id: 0 });
        }
    }
    // reader D/E seam：count++（U → 每帧 +1 的近似不成立，保持 0 → count 门控不触发，
    // 见分歧点）。
    // AppMode==6 且形状!=0 且 count!=0 → state@+9=1。
    let shape = match *mode {
        country_core::systems::mode_manager::AppMode::Wall(shape) => shape as u8,
        _ => 0,
    };
    if mode.native_tag() == 6 && shape != 0 && state.count != 0 {
        state.state = 1;
    }
    if mode.is_tool_selected() && !state.hint_done && !deselect_pressed_seam() && state.state == 1 {
        let display = crate::hint_clutter_focus_edit::make_focus_or_camera_binding_hint(&profile);
        // O 0x1422853c0：`MOV EAX,0x6` / Wall 时 `MOVZX EAX,[RDI+4]; ADD EAX,0x6`
        // → placement；`MOV word ptr [RBP+0x38],0x0` → +0x48 flags/id = 0/0。
        let shape = match *mode {
            country_core::systems::mode_manager::AppMode::Wall(shape) => shape as u8,
            _ => 0,
        };
        let placement = if mode.native_tag() == 6 {
            crate::hint_orchestrator::wall_shape_placement(shape)
        } else {
            crate::hint_orchestrator::HintPlacement::Cursor
        };
        request.write(
            crate::DisplayHintRequest::new(
                display,
                crate::hint_orchestrator::HintType::DeselectTool,
                0,
            )
            .with_placement(placement),
        );
    }
}
