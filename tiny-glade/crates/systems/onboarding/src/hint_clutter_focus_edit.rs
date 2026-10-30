//! hint_clutter_focus_edit（O：condition 0x141c3d890 / hint 0x141c3db80 /
//! make_focus_or_camera_binding_hint 0x141c3de00）。
//!
//! `hint_clutter_focused_edit_condition`：GameProfile+0x200!=0 → 状态置 1 返回；工具
//! ==3 → 置状态、写完成事件 tag 9；当前提示 HintType==ClutterFocus 写事件 tag 0x13。
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
/// `Steam(packed_set_plus_one(1)) = Steam(0x0101)` 为基底；再读 FocusEdit 鼠标绑定
/// （`mouse[2]`，原 `profile+0x1c4` tag / `+0x1c6` 载荷：`assign_mouse_button`
/// 0x14089b280 写 `profile+0x1bc+4*action`，camera_rotate/pan 读 `+0x1bc/+0x1c0`，
/// 故 `+0x1c4` 是 `MouseBinding::FocusEdit`，说明键 `KeyBindings.FocusOrDeselect`）：
/// tag 1（Right）/ 2（Middle）时 **只调用一次** `alternate<HintGraphicParams>`，
/// 单个 Graphic 载荷 = **两帧** `[hint/mouse, hint/rmb|hint/mmb]` + `0.7f` + `100.0f`
/// （0x142f0ee08/0x142f0ee12/0x142f0ee1a 字符串；`0x3f333333`/`0x42c80000`）；
/// 其它 → `alternate<MouseButton>(tag, +0x1c6 载荷)`，即该绑定本身（相机提示共用的
/// 约定：未绑定不替换成默认左键）。
pub fn make_focus_or_camera_binding_hint(
    profile: &country_core::game_profile::GameProfile,
) -> crate::BindingHintDisplay {
    let base = crate::BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(
        country_core::resources::steam::packed_set_plus_one(STEAM_FOCUS_ACTION),
    ));
    let binding = profile.key_bindings.mouse[country_core::game_key_bindings::MouseBinding::FocusEdit as usize];
    match binding {
        Some(crate::MouseButton::Right) => base.alternate(
            crate::hint_orchestrator::HintGraphicParams::animated(
                vec!["hint/mouse".to_string(), "hint/rmb".to_string()],
                0.7,
                100.,
            ),
        ),
        Some(crate::MouseButton::Middle) => base.alternate(
            crate::hint_orchestrator::HintGraphicParams::animated(
                vec!["hint/mouse".to_string(), "hint/mmb".to_string()],
                0.7,
                100.,
            ),
        ),
        Some(button) => base.alternate(button),
        None => base.alternate(crate::BindingHintDisplay::Empty),
    }
}

/// hint_clutter_focused_edit_condition（O 0x141c3d890）：profile 0x200!=0 →
/// hint_done=1 返回；读 `ModeJustChanged`，末事件当前模式 tag==3 → hint_done=1；未 done 且
/// 工具!=3 → false；写 TutorialHintComplete(id 9)；当前提示 HintType==ClutterFocus → PlayUiSound(kind 0x13)。
/// PDB FunctionSystem 参数明确为 Query<&HintType>，读取 orchestrator 的真实提示实体。
// O：反编译返回 void（0x14227da70 / 0x141c3d890 全部 `return;`），且原 add_onboarding 经 InfallibleSystemWrapper
// 以普通系统入链（Out=()）；早先 `-> bool` 属误判，已改为 `()`，各早退路径语义不变。
pub fn hint_clutter_focused_edit_condition(
    profile: Res<country_core::game_profile::GameProfile>,
    mut state: ResMut<FocusedClutterHintConditions>,
    mut mode: EventReader<country_core::resources::events::ModeJustChanged>,
    mut completed: EventWriter<crate::tutorial_completion::TutorialHintComplete>,
    mut sound: EventWriter<country_core::systems::ui::resources::widget_params::PlayUiSound>,
    query: Query<&crate::HintType>,
) {
    if profile.clutter_focused_edit_completed {
        state.hint_done = true;
        return;
    }
    if let Some(latest) = mode.read().last() {
        if latest.current.native_tag() == 3 {
            state.hint_done = true;
        } else if !state.hint_done {
            return;
        }
    } else {
        return;
    }
    completed.write(crate::tutorial_completion::TutorialHintComplete { id: 9 });
    if query.iter().next() == Some(&crate::HintType::ClutterFocus) {
        let _ = sound.write(
            country_core::systems::ui::resources::widget_params::PlayUiSound {
                kind: 0x13,
                _payload: [0; 20],
            },
        );
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
    use country_core::systems::ui_edit_focus::{GeoSource, geo_src_to_edit_subject_w_deco_clutter};

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
    use country_core::game_key_bindings::MouseBinding;

    fn profile_with(button: Option<crate::MouseButton>) -> country_core::game_profile::GameProfile {
        let mut profile = country_core::game_profile::GameProfile::default();
        profile.key_bindings.mouse[MouseBinding::FocusEdit as usize] = button;
        profile
    }

    fn graphic_of(display: crate::BindingHintDisplay) -> crate::hint_orchestrator::HintGraphicParams {
        let crate::BindingHintDisplay::Alternate(items) = display else { panic!("Alternate") };
        assert_eq!(items.len(), 2, "Steam + 单个 Graphic");
        match &items[0] {
            crate::BindingHintDisplay::Steam(q) => assert_eq!(q.0, 0x0101),
            other => panic!("{other:?}"),
        }
        match items.into_iter().nth(1).unwrap() {
            crate::BindingHintDisplay::Graphic(p) => p,
            other => panic!("{other:?}"),
        }
    }

    /// 原 0x141c3de00：FocusEdit 绑定为 Right/Middle 时是 **两帧**（不是相机提示的三帧）动画，
    /// 0.7 秒/帧、高 100；其余绑定按 alternate<MouseButton> 原样显示。
    #[test]
    fn focus_hint_reads_the_focus_edit_mouse_binding_with_two_frame_animation() {
        for (button, second) in [
            (crate::MouseButton::Right, "hint/rmb"),
            (crate::MouseButton::Middle, "hint/mmb"),
        ] {
            let graphic = graphic_of(make_focus_or_camera_binding_hint(&profile_with(Some(button))));
            assert_eq!(
                graphic.images,
                crate::hint_orchestrator::HintGraphicImages::Animation {
                    frames: vec!["hint/mouse".to_string(), second.to_string()],
                    seconds_per_frame: 0.7,
                }
            );
            assert_eq!(graphic.height, 100.);
            assert_eq!(graphic.image_at_time(0.7), second);
        }
    }

    #[test]
    fn focus_hint_shows_other_bindings_as_is_and_keeps_unbound_empty() {
        for button in [
            crate::MouseButton::Left,
            crate::MouseButton::Back,
            crate::MouseButton::Forward,
            crate::MouseButton::Other(7),
        ] {
            let crate::BindingHintDisplay::Alternate(items) =
                make_focus_or_camera_binding_hint(&profile_with(Some(button)))
            else {
                panic!("Alternate")
            };
            assert_eq!(items[1], crate::BindingHintDisplay::Mouse(button));
        }
        let crate::BindingHintDisplay::Alternate(items) =
            make_focus_or_camera_binding_hint(&profile_with(None))
        else {
            panic!("Alternate")
        };
        assert_eq!(items[1], crate::BindingHintDisplay::Empty);
    }

    /// 回归：旧 `focus_glyph_mode` 误当作绑定来源，玩家改键后提示仍显示默认字形。
    /// 另外两个鼠标绑定（Rotate/Pan）不能影响聚焦提示。
    #[test]
    fn focus_hint_ignores_the_rotate_and_pan_bindings() {
        let mut profile = profile_with(Some(crate::MouseButton::Left));
        profile.key_bindings.mouse[MouseBinding::Rotate as usize] = Some(crate::MouseButton::Right);
        profile.key_bindings.mouse[MouseBinding::Pan as usize] = Some(crate::MouseButton::Middle);
        let crate::BindingHintDisplay::Alternate(items) = make_focus_or_camera_binding_hint(&profile)
        else {
            panic!("Alternate")
        };
        assert_eq!(items[1], crate::BindingHintDisplay::Mouse(crate::MouseButton::Left));
    }
}