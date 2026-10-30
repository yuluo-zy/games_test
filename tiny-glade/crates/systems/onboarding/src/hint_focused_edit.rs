//! hint_focused_edit（O：condition 0x14227da70 / hint 0x14227dd60）。
//!
//! `hint_focused_edit_condition`：GameProfile+0x1f5!=0 → 状态置 1 返回；否则读工具变化
//! 事件（ModeJustChanged），工具∈{1,2}（`*piVar5-1U<2`）→ 置状态、写完成事件 tag 1；
//! 当前提示 HintType==FocusedEdit → 写事件 tag 0x13。
//! `hint_focused_edit_hint`：state.hint_done==0 时读
//! `EventReader<NotifyHintSystemShapeEditWasPerformed>`（有事件 → count++、
//! `state@+9 = 4<count`）；`Query<(&Roof,&RoofTriggerVolume)>` 非空 &&
//! state@+9!=0 && `Res<AppMode>==0` → `RaycastWorld::legacy_raycast`
//! （命中码 +0x30 != 2）+ `geo_src_to_edit_subject` → 原跳转表 0x1430386a0：
//! subject 1→Wall（额外 `PublicWalls::get` + 形状 tag<2 门）/ 2→Roof → 写
//! `DisplayHintRequest`（placement=EditSubject+0x101、flags 0、id 1）；0/3/4/5 无提示。
//!
//! **已重建（2026-10-17，geo_src_to_edit_subject/legacy_raycast 命中码落库后）**；
//! fresh 证据：本目录 `evidence/result-fresh.json`（ghidra 0x14227dd60）、
//! `evidence/result-r2.json`（radare2 跳表解码）、`evidence/result-callers.json`
//! （PDB SystemParam 全签名）。分歧点见 RESTORE。

use bevy_ecs::prelude::*;

/// 焦点编辑提示条件（O：hints_startup 初值 10B {u64 count@+0=0, u8 hint_done@+8=
/// profile[0x1f5], u8 state@+9=0}；condition 置 hint_done@+8、hint 函数 count++ 与
/// `4<count` 置 state@+9）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct FocusedEditHintConditions {
    /// @+0：事件计数（u64，O：hint_focused_edit_hint count++）。
    pub count: u64,
    /// @+8：完成/展示标志（初值 profile[0x1f5]；condition 置 1）。
    pub hint_done: bool,
    /// @+9：状态字节（O：hint 函数 `4<count` 置位）。
    pub state: u8,
}

/// hint_focused_edit_condition（O 0x14227da70）：profile 0x1f5!=0 → hint_done=1 返回；
/// 读 `ModeJustChanged`（0x28B），末事件当前模式 tag ∈ {1,2}（`*piVar5-1U<2`）→ hint_done=1；
/// 未 done 且工具不满足 → false；写 TutorialHintComplete(id 1)；当前提示 HintType==FocusedEdit →
/// PlayUiSound(kind 0x13)。
/// PDB FunctionSystem 参数明确为 Query<&HintType>，读取 orchestrator 的真实提示实体。
// O：反编译返回 void（0x14227da70 / 0x141c3d890 全部 `return;`），且原 add_onboarding 经 InfallibleSystemWrapper
// 以普通系统入链（Out=()）；早先 `-> bool` 属误判，已改为 `()`，各早退路径语义不变。
pub fn hint_focused_edit_condition(
    profile: Res<country_core::game_profile::GameProfile>,
    mut state: ResMut<FocusedEditHintConditions>,
    mut mode: EventReader<country_core::resources::events::ModeJustChanged>,
    mut completed: EventWriter<crate::tutorial_completion::TutorialHintComplete>,
    mut sound: EventWriter<country_core::systems::ui::resources::widget_params::PlayUiSound>,
    query: Query<&crate::HintType>,
) {
    if profile.focused_edit_completed {
        state.hint_done = true;
        return;
    }
    if let Some(latest) = mode.read().last() {
        if matches!(latest.current.native_tag(), 1 | 2) {
            state.hint_done = true;
        } else if !state.hint_done {
            return;
        }
    } else {
        return;
    }
    completed.write(crate::tutorial_completion::TutorialHintComplete { id: 1 });
    if query.iter().next() == Some(&crate::HintType::FocusedEdit) {
        let _ = sound.write(
            country_core::systems::ui::resources::widget_params::PlayUiSound {
                kind: 0x13,
                _payload: [0; 20],
            },
        );
    }
}

/// hint_focused_edit_hint（O 0x14227dd60，PDB 全签名：
/// `(Res<AppMode>, Res<PublicWalls>, Query<(&Roof,&RoofTriggerVolume)>,
/// ResMut<FocusedEditHintConditions>, EventReader<NotifyHintSystemShapeEditWasPerformed>,
/// EventWriter<DisplayHintRequest>, Res<RaycastWorld>, Res<CameraCursorRay>,
/// Res<GameProfile>)`）。
///
/// 逐语句（fresh，见 `evidence/result-fresh.json` / `result-r2.json`）：
/// 1. `if state.hint_done != 0 → return`（state@+8）；
/// 2. EventReader 有事件（`EventReader::read` 剩余计数 >0）→
///    `count = count.wrapping_add(1)`；`state@+9 = 4 < count`；
/// 3. 查询非空才继续（原版取末实体 item 指针做 `!=0` 检查，等价非空）；
/// 4. `state@+9 == 0 → return`；`AppMode @+0 != 0 → return`；
/// 5. `legacy_raycast`，命中码（+0x30 位型）== 2（miss）→ return；
/// 6. `geo_src_to_edit_subject(handle[24B])` 跳表：0/3/4/5 → 无提示；1→Wall
///    （`PublicWalls::get(id).unwrap()`，形状 tag@+0x258 >= 2（Freehand）→ return）；
///    2→Roof；
/// 7. Wall/Roof → `make_focus_or_camera_binding_hint(profile)` 写
///    `DisplayHintRequest{placement=EditSubject + u16 0x0101, flags=0, id=1}`。
pub fn hint_focused_edit_hint(
    mode: Res<country_core::systems::mode_manager::AppMode>,
    public_walls: Res<country_core::resources::walls::public_walls::PublicWalls>,
    query: Query<(
        &country_core::resources::roofs::roof_shape::Roof,
        &country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume,
    )>,
    mut state: ResMut<FocusedEditHintConditions>,
    mut shape_edit_events: EventReader<
        country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed,
    >,
    mut request: EventWriter<crate::DisplayHintRequest>,
    raycast: Res<country_core::systems::collision::world::RaycastWorld>,
    camera_ray: Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,
    profile: Res<country_core::game_profile::GameProfile>,
) {
    use country_core::systems::mode_manager::EditSubject;
    use country_core::systems::ui_edit_focus::{GeoSource, geo_src_to_edit_subject};

    // 1. state@+8 != 0 → 整个系统早退。
    if state.hint_done {
        return;
    }
    // 2. EventReader<NotifyHintSystemShapeEditWasPerformed>：**一次性耗尽**全部未读
    //    （原生：未读>0 → 游标置 event_count），每帧 count 最多 +1；`state@+9 = 4<count`
    //    （原版 `setae byte [rax+9]`）。用 `.count()` 而非 `.next()`，避免多事件帧
    //    把剩余事件留到后续帧再 +1。
    if shape_edit_events.read().count() > 0 {
        state.count = state.count.wrapping_add(1);
        state.state = u8::from(4 < state.count);
    }
    // 3. Query<(&Roof,&RoofTriggerVolume)> 非空门（原版查询遍历后 lVar12 != 0）。
    if query.iter().next().is_none() {
        return;
    }
    // 4. state@+9 == 0 或 AppMode!=0 → 不展示。
    if state.state == 0 || mode.native_tag() != 0 {
        return;
    }
    // 5. legacy_raycast：调用方读 +0x30 的 u32 命中码，==2 视为 miss。
    let hit = raycast.legacy_raycast(camera_ray.origin, camera_ray.dir);
    if legacy_raycast_miss(&hit) {
        return;
    }
    // 6. 跳表 0x1430386a0（6 项，subject=geo_src 判别）：
    //    0,3,4,5 → epilogue；1 → Wall 校验后落公共写面；2 → Roof 公共写面。
    let subject = geo_src_to_edit_subject(GeoSource::from_hit_handle(hit.handle));
    match subject {
        EditSubject::Wall(wall) => {
            // 原版 `PublicWalls::get` 结果 unwrap（0x14227e0b6 `je` → panic 0x14227e127，
            // Location = hint_focused_edit.rs:133:47）；形状判别 @+0x258 >= 2 → return。
            let wall_state = public_walls
                .get(country_core::resources::walls::WallId(wall))
                .unwrap();
            if wall_state.authority.source().shape.tag() >= 2 {
                return;
            }
            write_focused_edit_hint(subject, 0x0101, &profile, &mut request);
        }
        EditSubject::Roof(_) => {
            write_focused_edit_hint(subject, 0x0101, &profile, &mut request);
        }
        // 0 None / 3 DecoClutter / 4 StairPoint / 5 Terrain：跳表到 epilogue。
        _ => {}
    }
}

/// legacy_raycast 命中码判断（O 调用方：`cmp dword [result+0x30], 2`）。
///
/// `RaycastWorld::legacy_raycast` 在 miss 时把 u32 2 的**位型**写入 +0x30，
/// 故 native 等价判断为 `hit.toi.to_bits() == 2`。非 native-physics 壳返回
/// `RaycastWorldResult::miss()`（+0x30 未写位型、判别 +0x34 == 2），此分支作
/// shell seam 一并视为 miss（登记 I；native 下两条件恒等价）。
fn legacy_raycast_miss(hit: &country_core::systems::collision::world::RaycastWorldResult) -> bool {
    hit.toi.to_bits() == country_core::systems::collision::world::RAYCAST_WORLD_RESULT_MISS
        || hit.is_miss()
}

/// 公共写面（O 0x14227e0c1..0x14227e10e）：`make_focus_or_camera_binding_hint`
/// 生成 display；placement = 原 EditSubject 16B + native +0x10 u16；
/// `mov word [rsp+0xb8], 0x100` → flags@+0x48 = 0、id@+0x49 = 1（FocusedEdit）。
fn write_focused_edit_hint(
    subject: country_core::systems::mode_manager::EditSubject,
    word_0x10: u16,
    profile: &country_core::game_profile::GameProfile,
    request: &mut EventWriter<crate::DisplayHintRequest>,
) {
    let display = crate::hint_clutter_focus_edit::make_focus_or_camera_binding_hint(profile);
    request.write(
        crate::DisplayHintRequest::new(display, crate::hint_orchestrator::HintType::FocusedEdit, 0)
            .with_placement(crate::HintPlacement::Subject { subject, word_0x10 }),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::onboarding::NotifyHintSystemShapeEditWasPerformed;

    /// FIX-4 回归：一帧 2 个 shape-edit 事件 → count 只 +1（原生一次性耗尽）；
    /// 下一帧无新事件 → 不再增长。
    #[test]
    fn shape_edit_events_are_exhausted_once_per_frame() {
        use country_core::systems::mode_manager::{AppMode, EditSubject};

        let mut world = World::new();
        world.insert_resource(AppMode::Editing(EditSubject::None));
        world.insert_resource(country_core::resources::walls::public_walls::PublicWalls::default());
        world.insert_resource(FocusedEditHintConditions::default());
        world.init_resource::<Events<NotifyHintSystemShapeEditWasPerformed>>();
        world.init_resource::<Events<crate::DisplayHintRequest>>();
        world.insert_resource(country_core::systems::collision::world::RaycastWorld::default());
        world.insert_resource(
            country_core::resources::mouse_terrain_raycast::CameraCursorRay::default(),
        );
        world.insert_resource(country_core::game_profile::GameProfile::default());

        {
            let mut events = world.resource_mut::<Events<NotifyHintSystemShapeEditWasPerformed>>();
            events.send(NotifyHintSystemShapeEditWasPerformed);
            events.send(NotifyHintSystemShapeEditWasPerformed);
        }

        let mut schedule = Schedule::default();
        schedule.add_systems(hint_focused_edit_hint);
        schedule.run(&mut world);
        let state = *world.resource::<FocusedEditHintConditions>();
        assert_eq!(state.count, 1);
        assert_eq!(state.state, 0);

        schedule.run(&mut world);
        assert_eq!(world.resource::<FocusedEditHintConditions>().count, 1);
    }
}

#[cfg(test)]
mod canonical_mode_publication_tests {
    use super::*;
    use crate::hint_clutter_focus_edit::{
        FocusedClutterHintConditions, hint_clutter_focused_edit_condition,
    };
    use crate::tutorial_completion::TutorialHintComplete;
    use country_core::resources::events::ModeJustChanged;
    use country_core::systems::mode_manager::{
        AppMode, EditSubject, PreviousAppMode, send_mode_just_changed_event,
    };

    #[test]
    fn published_latest_mode_controls_both_hint_consumers_and_drains_old_modes() {
        for (target, expected_hint) in [
            (
                AppMode::Editing(EditSubject::Wall(0xfedc_ba98_7654_3210)),
                1,
            ),
            (
                AppMode::Editing(EditSubject::DecoClutter {
                    decorator_id: 0xdead_beef,
                }),
                9,
            ),
        ] {
            let mut world = World::new();
            world.init_resource::<country_core::game_profile::GameProfile>();
            world.init_resource::<FocusedEditHintConditions>();
            world.init_resource::<FocusedClutterHintConditions>();
            world.init_resource::<PreviousAppMode>();
            world.init_resource::<Events<ModeJustChanged>>();
            world.init_resource::<Events<TutorialHintComplete>>();
            world.init_resource::<Events<country_core::systems::ui::resources::widget_params::PlayUiSound>>();
            let mut publish = Schedule::default();
            publish.add_systems(send_mode_just_changed_event);
            let mut consume = Schedule::default();
            consume.add_systems((
                hint_focused_edit_condition,
                hint_clutter_focused_edit_condition,
            ));

            // A qualifying old-buffer mode must not override the newest Path.
            world.insert_resource(target);
            publish.run(&mut world);
            world.resource_mut::<Events<ModeJustChanged>>().update();
            world.insert_resource(AppMode::Path);
            publish.run(&mut world);
            consume.run(&mut world);
            assert!(!world.resource::<FocusedEditHintConditions>().hint_done);
            assert!(!world.resource::<FocusedClutterHintConditions>().hint_done);
            assert_eq!(world.resource::<Events<TutorialHintComplete>>().len(), 0);
            consume.run(&mut world);
            assert_eq!(world.resource::<Events<TutorialHintComplete>>().len(), 0);

            // Re-enter through the real publisher; only the latest target qualifies.
            world.insert_resource(target);
            publish.run(&mut world);
            consume.run(&mut world);
            let completed: Vec<_> = world
                .resource_mut::<Events<TutorialHintComplete>>()
                .drain()
                .map(|event| event.id)
                .collect();
            assert_eq!(completed, vec![expected_hint]);
            consume.run(&mut world);
            assert_eq!(world.resource::<Events<TutorialHintComplete>>().len(), 0);
        }
    }
}
