//! hint_clutter_focus_edit（O：condition 0x141c3d890 / hint 0x141c3db80 /
//! make_focus_or_camera_binding_hint 0x141c3de00）。
//!
//! `hint_clutter_focused_edit_condition`：GameProfile+0x200!=0 → 状态置 1 返回；工具
//! ==3 → 置状态、写完成事件 tag 9；焦点实体工具字节==9 写事件 tag 0x13。
//! `hint_clutter_focused_edit_hint`：state.hint_done==0 时读
//! `EventReader<NotifyHintSystemClutterWasMoved>`（未读事件**一次性耗尽** →
//! state@+1=1）；
//! `UiState@+0x18 != 2` 且 state@+1!=0 且 `Res<AppMode>==0` →
//! `RaycastWorld::raycast`（mask 0x100，判别 +0x34 == 2 → return）+
//! `geo_src_to_edit_subject_w_deco_clutter`：判别 ==3 才写
//! `DisplayHintRequest`（placement = DecoClutter subject + 0x0100，flags 0、id 1）。
//! `make_focus_or_camera_binding_hint`：Steam(0x0101) 基底 → profile 0x1c4==1/2 时
//! **单元素** `Graphic{[hint/mouse, hint/rmb|hint/mmb], 0.7, 100.0}`；其它 →
//! alternate(MouseButton)（profile+0x1c6，U）。
//!
//! **已重建（2026-10-17，geo_src/legacy 命中码落库后）**；fresh 证据见
//! `evidence/result-fresh.json`（ghidra 0x141c3db80）、`evidence/result-r2.json`、
//! `evidence/result-callers.json`（PDB SystemParam 全签名）。

use bevy_ecs::prelude::*;

/// 杂物焦点提示条件（O：hints_startup 初值 2B {u8 hint_done@0=profile[0x200],
/// u8 state@1=0}）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct FocusedClutterHintConditions {
    /// @+0：完成/展示标志（初值 profile[0x200]）。
    pub hint_done: bool,
    /// @+1：状态字节。
    pub state: u8,
}

/// Steam 焦点动作码 index（O 0x141c3de20 `MOV CL,1; CALL impl$31::from`
/// —— 0x1409e8d90 = `(cl<<8)+1` ⇒ **0x0101**）。
const STEAM_FOCUS_ACTION: u8 = 1;

/// make_focus_or_camera_binding_hint（O 0x141c3de00）：
/// `Steam(packed_set_plus_one(1)) = Steam(0x0101)` 为基底；
/// `profile+0x1c4 == 1/2`（u16）时 **只调用一次** `alternate<HintGraphicParams>`，
/// 单个 Graphic 载荷 = 图标数组 `[hint/mouse, hint/rmb|hint/mmb]` + `0.7f` + `100.0f`
/// （0x141c3de89/0x141c3df38；`0x3f333333`/`0x42c80000`）；
/// 其它 → `alternate<MouseButton>(profile+0x1c6)`。
/// **重建（2026-10-17 fresh 反汇编）**；分歧点：`profile+0x1c6` 鼠标键字段
/// country-core `GameProfile` 未落库（登记 U，暂以 `MouseButton::Left` 占位）。
pub fn make_focus_or_camera_binding_hint(
    profile: &country_core::game_profile::GameProfile,
) -> crate::BindingHintDisplay {
    let base = crate::BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(
        country_core::resources::steam::packed_set_plus_one(STEAM_FOCUS_ACTION),
    ));
    match profile.focus_glyph_mode {
        1 | 2 => {
            let second = if profile.focus_glyph_mode == 1 {
                "hint/rmb"
            } else {
                "hint/mmb"
            };
            base.alternate(crate::hint_orchestrator::HintGraphicParams {
                graphics: vec!["hint/mouse".to_string(), second.to_string()],
                value_0x18: 0.7,
                value_0x1c: 100.0,
            })
        }
        // U：原生 `MOVZX R9D,word [profile+0x1c6]`；core GameProfile 无该字段。
        _ => base.alternate(crate::MouseButton::Left),
    }
}

/// hint_clutter_focused_edit_condition（O 0x141c3d890）：profile 0x200!=0 →
/// hint_done=1 返回；读 `ModeJustChanged`，首事件工具==3 → hint_done=1；未 done 且
/// 工具!=3 → false；写 TutorialHintComplete(id 9)；焦点实体字节==9 → PlayUiSound(kind 0x13)。
/// **重建（2026-10-17，ModeJustChanged + GameProfile 落库）**；分歧点：焦点字节 Query
/// 组件类型 U（seam `FocusSubjectByte`）、PlayUiSound id U。
// O：反编译返回 void（0x14227da70 / 0x141c3d890 全部 `return;`），且原 add_onboarding 经 InfallibleSystemWrapper
// 以普通系统入链（Out=()）；早先 `-> bool` 属误判，已改为 `()`，各早退路径语义不变。
pub fn hint_clutter_focused_edit_condition(
    profile: Res<country_core::game_profile::GameProfile>,
    mut state: ResMut<FocusedClutterHintConditions>,
    mut mode: EventReader<country_core::resources::events::ModeJustChanged>,
    mut completed: EventWriter<crate::tutorial_completion::TutorialHintComplete>,
    mut sound: EventWriter<country_core::systems::ui::resources::widget_params::PlayUiSound>,
    query: Query<&crate::hint_focused_edit::FocusSubjectByte>,
) {
    if profile.clutter_focused_edit_completed {
        state.hint_done = true;
        return;
    }
    if let Some(first) = mode.read().next() {
        if first.tool == 3 {
            state.hint_done = true;
        } else if !state.hint_done {
            return;
        }
    } else {
        return;
    }
    completed.write(crate::tutorial_completion::TutorialHintComplete { id: 9 });
    if query.iter().next().is_some_and(|b| b.0 == 9) {
        let _ = sound.write(country_core::systems::ui::resources::widget_params::PlayUiSound {
            kind: 0x13,
            _payload: [0; 0x10],
            id: 0,
        });
    }
}

/// hint_clutter_focused_edit_hint（O 0x141c3db80，PDB 全签名：
/// `(Res<AppMode>, ResMut<FocusedClutterHintConditions>,
/// EventWriter<DisplayHintRequest>, EventReader<NotifyHintSystemClutterWasMoved>,
/// Res<DecoratorStorage>, Res<RaycastWorld>, Res<CameraCursorRay>, Res<UiState>,
/// Res<GameProfile>)`）。
///
/// 逐语句（fresh，见 `evidence/result-r2.json`）：
/// 1. `state.hint_done@+0 != 0 → return`；
/// 2. EventReader 有事件 → `state@+1 = 1`；
/// 3. `UiState@+0x18 == 2 → return`（0x141c3dc1e `lea rax,[rcx+0x18]` 后
///    `cmp byte [rax],2`；同字节即 UiInteraction::native_tag）；
/// 4. `state@+1 == 0 → return`；`AppMode @+0 != 0 → return`；
/// 5. `RaycastWorld::raycast`（0x1408cc870，mask 0x100；判别 @+0x34 == 2 → return）；
/// 6. `geo_src_to_edit_subject_w_deco_clutter(handle, &DecoratorStorage)` 判别 ==3
///    才继续（0x141c3dcb6 `cmp dword [rbp-0x30],3`）；
/// 7. `make_focus_or_camera_binding_hint(profile)` → `DisplayHintRequest{
///    placement = DecoClutter subject + native@+0x10 = 0x0100, flags 0, id 1}`。
pub fn hint_clutter_focused_edit_hint(
    mode: Res<country_core::systems::mode_manager::AppMode>,
    mut state: ResMut<FocusedClutterHintConditions>,
    mut request: EventWriter<crate::DisplayHintRequest>,
    mut moved_events: EventReader<
        country_core::resources::onboarding::NotifyHintSystemClutterWasMoved,
    >,
    decorators: Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,
    raycast: Res<country_core::systems::collision::world::RaycastWorld>,
    camera_ray: Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,
    ui: Res<country_core::systems::ui::resources::ui_state::UiState>,
    profile: Res<country_core::game_profile::GameProfile>,
) {
    use country_core::systems::mode_manager::EditSubject;
    use country_core::systems::ui_edit_focus::{
        geo_src_to_edit_subject_w_deco_clutter, GeoSource,
    };

    // 1. state@+0 != 0 → 早退。
    if state.hint_done {
        return;
    }
    // 2. EventReader<NotifyHintSystemClutterWasMoved>：有事件 → state@+1=1。
    //    O（reviewer FIX-5）：原生一次性耗尽未读事件；用 `count()>0`。
    if moved_events.read().count() > 0 {
        state.state = 1;
    }
    // 3. UiState native +0x18 == 2 → 早退（0x141c3dc1e..0x141c3dc29）。
    if ui.interaction().native_tag == 2 {
        return;
    }
    // 4. state@+1 == 0 或 AppMode!=0 → 早退。
    if state.state == 0 || mode.native_tag() != 0 {
        return;
    }
    // 5. RaycastWorld::raycast（mask 0x100），判别 +0x34 == 2（miss）→ return。
    let hit = raycast.raycast(camera_ray.origin, camera_ray.dir, 0x100);
    if hit.discriminant == country_core::systems::collision::world::RAYCAST_WORLD_RESULT_MISS {
        return;
    }
    // 6. _w_deco_clutter：仅 EditSubject::DecoClutter（判别 3）生成提示。
    let subject = geo_src_to_edit_subject_w_deco_clutter(
        GeoSource::from_hit_handle(hit.handle),
        Some(&decorators),
    );
    if !matches!(subject, EditSubject::DecoClutter { .. }) {
        return;
    }
    // 7. 写面：display = make_focus_or_camera_binding_hint；
    //    `mov word [rbp+0x28],0x100` → flags@+0x48=0、id@+0x49=1（FocusedEdit）。
    let display = make_focus_or_camera_binding_hint(&profile);
    request.write(
        crate::DisplayHintRequest::new(display, crate::hint_orchestrator::HintType::FocusedEdit, 0)
            .with_placement(crate::HintPlacement::Subject {
                subject,
                word_0x10: 0x0100,
            }),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// FIX-6/FIX-8 回归：Steam 载荷 = 0x0101；glyph_mode 1 时 **单个** Graphic
    /// 元素 = 双图标 + 0.7/100.0（不是两个 Alternate 元素）。
    #[test]
    fn make_focus_steam_query_and_double_icon_payload() {
        let mut profile = country_core::game_profile::GameProfile::default();
        profile.focus_glyph_mode = 1;
        match make_focus_or_camera_binding_hint(&profile) {
            crate::BindingHintDisplay::Alternate(items) => {
                assert_eq!(items.len(), 2, "Steam + 单个 Graphic");
                match &items[0] {
                    crate::BindingHintDisplay::Steam(q) => assert_eq!(q.0, 0x0101),
                    other => panic!("{other:?}"),
                }
                match &items[1] {
                    crate::BindingHintDisplay::Graphic(p) => {
                        assert_eq!(
                            p.graphics,
                            vec!["hint/mouse".to_string(), "hint/rmb".to_string()]
                        );
                        assert_eq!(p.value_0x18, 0.7);
                        assert_eq!(p.value_0x1c, 100.0);
                    }
                    other => panic!("{other:?}"),
                }
            }
            other => panic!("{other:?}"),
        }

        // glyph_mode 2 → 第二图标 hint/mmb。
        profile.focus_glyph_mode = 2;
        match make_focus_or_camera_binding_hint(&profile) {
            crate::BindingHintDisplay::Alternate(items) => match &items[1] {
                crate::BindingHintDisplay::Graphic(p) => {
                    assert_eq!(p.graphics[1], "hint/mmb");
                }
                other => panic!("{other:?}"),
            },
            other => panic!("{other:?}"),
        }
    }
}
