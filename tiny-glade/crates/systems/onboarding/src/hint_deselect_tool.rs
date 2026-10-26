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
//! **2026-10-26 复审重写**（参数表取自 PDB 的 `FunctionSystem<fn(..)>` 符号，见
//! `reports/module-recovery-2026-10-23/m01-system-params-from-pdb.md`）：读者 B 是
//! `ModeJustChanged`，读者 C 是 `ToolSelectionCancelled`，读者 D/E 是 `CreateCircleCmd`
//! /`CreateRectangleCmd`，pressed 读 `ActionSetStates.digital[0]`（click，`FUN_14093fa70`
//! 即 `phase<3`）。旧版自创的 `DeselectReset` 事件与共享常量 seam 已删除。
//! 仍未恢复：placement 7/8 的 payload 来源（见 `hint_orchestrator::wall_shape_placement`）。

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

/// hint_deselect_tool（O 0x1422853c0，1116B，11 个 SystemParam）。逐语句：
/// 1. `profile+0x1f4 != 0` 直接返回；
/// 2. `NotifyHintSystemToolWasJustUsed` 有未读 → `state@+9 = 1`；
/// 3. `ModeJustChanged` 有未读 → `state@+9 = 0`；此时若 `ToolSelectionCancelled` 也有未读
///    （读者推进到末尾）且**最后一个** `ModeJustChanged` 的 `current` 判别为 0（回到无工具）
///    → `hint_done = 1` 并完成 id 0。`ToolSelectionCancelled` 读者只在这个分支里被消费；
/// 4. `CreateCircleCmd` 或 `CreateRectangleCmd` 任一有未读（两个读者各自推进）→ `count += 1`
///    （一帧最多 +1）；
/// 5. `AppMode` 判别 6（Wall）且形状字节 != 0 且 `count != 0` → `state@+9 = 1`；
/// 6. `is_tool_selected && !hint_done && !click.pressed() && state@+9 == 1` → 写
///    `DisplayHintRequest{placement = 6 + 形状字节(仅 Wall), make_focus_or_camera_binding_hint,
///    id 0, flags 0}`。
pub fn hint_deselect_tool(
    mode: Res<country_core::systems::mode_manager::AppMode>,
    mut state: ResMut<DeselectToolHintConditions>,
    profile: Res<country_core::game_profile::GameProfile>,
    mut mode_changed: EventReader<country_core::resources::events::ModeJustChanged>,
    mut cancelled: EventReader<country_core::systems::ui_edit_focus::ToolSelectionCancelled>,
    mut tool_used: EventReader<
        country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed,
    >,
    actions: Res<country_core::input::action_set::ActionSetStates>,
    mut request: EventWriter<crate::DisplayHintRequest>,
    mut completed: EventWriter<crate::tutorial_completion::TutorialHintComplete>,
    mut circles: EventReader<country_core::systems::wall::shapes::CreateCircleCmd>,
    mut rectangles: EventReader<country_core::systems::wall::shapes::CreateRectangleCmd>,
) {
    if profile.tutorial {
        return;
    }
    // 2. 读者 A。原版先把读游标推到末尾，再判断"有无未读"，所以用 count()。
    if tool_used.read().count() != 0 {
        state.state = 1;
    }
    // 3. 读者 B / C。
    let latest = mode_changed.read().last().copied();
    if let Some(latest) = latest {
        state.state = 0;
        let cancelled_any = cancelled.read().count() != 0;
        if cancelled_any && latest.current.native_tag() == 0 {
            state.hint_done = true;
            completed.write(crate::tutorial_completion::TutorialHintComplete { id: 0 });
        }
    }
    // 4. 读者 D / E：任一有未读 → count += 1。两个读者都要推进，不能短路。
    let created_circle = circles.read().count() != 0;
    let created_rectangle = rectangles.read().count() != 0;
    if created_circle || created_rectangle {
        state.count += 1;
    }
    // 5.
    let shape = match *mode {
        country_core::systems::mode_manager::AppMode::Wall(shape) => shape as u8,
        _ => 0,
    };
    if mode.native_tag() == 6 && shape != 0 && state.count != 0 {
        state.state = 1;
    }
    // 6. digital[0] = click；`FUN_14093fa70` 为 `phase < 3`，即 `pressed()`。
    if mode.is_tool_selected()
        && !state.hint_done
        && !actions.digital[CLICK_SLOT].pressed()
        && state.state == 1
    {
        let display = crate::hint_clutter_focus_edit::make_focus_or_camera_binding_hint(&profile);
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

/// `MainActionSet.digital[0]` = `"click"`（`DIGITAL_NAMES[0]`）。原版读 `ActionSetStates+0`。
const CLICK_SLOT: usize = 0;


#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::system::RunSystemOnce;
    use country_core::input::action_set::{ActionSetStates, DigitalActionPhase, DigitalActionState};
    use country_core::resources::events::ModeJustChanged;
    use country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed;
    use country_core::systems::mode_manager::{AppMode, EditSubject, WallShape};
    use country_core::systems::ui_edit_focus::ToolSelectionCancelled;
    use country_core::systems::wall::shapes::{CreateCircleCmd, CreateRectangleCmd};
    use crate::tutorial_completion::TutorialHintComplete;

    fn new_world(mode: AppMode) -> World {
        let mut world = World::new();
        world.insert_resource(mode);
        world.insert_resource(DeselectToolHintConditions::default());
        world.insert_resource(country_core::game_profile::GameProfile::default());
        world.insert_resource(ActionSetStates::default());
        world.init_resource::<Events<ModeJustChanged>>();
        world.init_resource::<Events<ToolSelectionCancelled>>();
        world.init_resource::<Events<NotifyHintSystemToolWasJustUsed>>();
        world.init_resource::<Events<CreateCircleCmd>>();
        world.init_resource::<Events<CreateRectangleCmd>>();
        world.init_resource::<Events<TutorialHintComplete>>();
        world.init_resource::<Events<crate::DisplayHintRequest>>();
        world
    }
    fn run(world: &mut World) {
        world.run_system_once(hint_deselect_tool).unwrap();
    }
    fn state(world: &World) -> DeselectToolHintConditions {
        *world.resource::<DeselectToolHintConditions>()
    }
    fn requests(world: &mut World) -> Vec<crate::DisplayHintRequest> {
        world.resource_mut::<Events<crate::DisplayHintRequest>>().drain().collect()
    }
    fn changed(current: AppMode, previous: AppMode) -> ModeJustChanged {
        ModeJustChanged { current, previous }
    }

    /// 步骤 2 + 6：工具刚被使用 → state=1，选中工具、没按住 click 时展示；按住 click 则不展示。
    #[test]
    fn tool_use_arms_the_hint_and_a_pressed_click_suppresses_it() {
        let mut world = new_world(AppMode::Path);
        world.send_event(NotifyHintSystemToolWasJustUsed);
        world.resource_mut::<ActionSetStates>().digital[0] =
            DigitalActionState { phase: DigitalActionPhase::Held, timer_seconds: 0.2 };
        run(&mut world);
        assert_eq!(state(&world).state, 1);
        assert!(requests(&mut world).is_empty(), "click.pressed() 压制展示");
        world.resource_mut::<ActionSetStates>().digital[0] =
            DigitalActionState { phase: DigitalActionPhase::JustReleased, timer_seconds: 0.0 };
        run(&mut world);
        let shown = requests(&mut world);
        assert_eq!(shown.len(), 1);
        assert_eq!(shown[0].id, crate::HintType::DeselectTool as u8);
        assert_eq!(shown[0].placement, crate::HintPlacement::Cursor);
    }

    /// 步骤 3：最后一个 ModeJustChanged 回到 Editing(None)（判别 0）且 ToolSelectionCancelled
    /// 有未读 → 完成 id 0；若 current 不是判别 0 则只清 state。
    #[test]
    fn cancelling_back_to_no_subject_completes_the_hint() {
        let mut world = new_world(AppMode::Editing(EditSubject::None));
        world.resource_mut::<DeselectToolHintConditions>().state = 1;
        world.send_event(changed(AppMode::Path, AppMode::Editing(EditSubject::None)));
        world.send_event(changed(AppMode::Editing(EditSubject::None), AppMode::Path));
        world.send_event(ToolSelectionCancelled);
        run(&mut world);
        let s = state(&world);
        assert!(s.hint_done);
        assert_eq!(s.state, 0);
        let done: Vec<_> = world.resource_mut::<Events<TutorialHintComplete>>().drain().map(|e| e.id).collect();
        assert_eq!(done, [0]);

        // 最后一个事件的 current 不是 0：只复位 state，不完成。
        let mut world = world_with_state_one();
        world.send_event(changed(AppMode::Editing(EditSubject::None), AppMode::Path));
        world.send_event(changed(AppMode::Path, AppMode::Editing(EditSubject::None)));
        world.send_event(ToolSelectionCancelled);
        run(&mut world);
        assert!(!state(&world).hint_done);
        assert_eq!(state(&world).state, 0);
        assert!(world.resource::<Events<TutorialHintComplete>>().is_empty());
    }
    fn world_with_state_one() -> World {
        let mut world = new_world(AppMode::Editing(EditSubject::None));
        world.resource_mut::<DeselectToolHintConditions>().state = 1;
        world
    }

    /// 没有 ModeJustChanged 时 ToolSelectionCancelled 不被消费也不完成（读者只在 B 分支里推进）。
    #[test]
    fn cancel_without_a_mode_change_does_nothing() {
        let mut world = world_with_state_one();
        world.send_event(ToolSelectionCancelled);
        run(&mut world);
        assert!(!state(&world).hint_done);
        assert_eq!(state(&world).state, 1);
        assert!(world.resource::<Events<TutorialHintComplete>>().is_empty());
    }

    /// 步骤 4/5：创建圆或矩形的命令每帧最多让 count 加 1；Wall(Circle/Rectangle) 模式且 count!=0
    /// 才把 state 置 1；Freeform 不会。
    #[test]
    fn shape_creation_counts_once_per_frame_and_only_arms_circle_or_rectangle_modes() {
        let circle = CreateCircleCmd {
            previous_wall: None,
            center: glam::Vec2::ZERO,
            radius: 3.0,
            rotation: 0.0,
            height: 0.0,
            elevation: 0.0,
        };
        let rectangle = CreateRectangleCmd {
            center: glam::Vec2::ZERO,
            dimensions: glam::Vec2::ONE,
            orientation: glam::Vec2::X,
            previous_wall: None,
            height: 0.0,
            elevation: 0.0,
        };
        let mut world = new_world(AppMode::Wall(WallShape::Circle));
        // 读者游标属于系统状态，必须跨帧复用同一个 schedule。
        let mut schedule = Schedule::default();
        schedule.add_systems(hint_deselect_tool);
        world.send_event(circle);
        world.send_event(circle);
        world.send_event(rectangle);
        schedule.run(&mut world);
        assert_eq!(state(&world).count, 1, "两个读者都推进，且一帧只 +1");
        assert_eq!(state(&world).state, 1);
        let shown = requests(&mut world);
        assert_eq!(shown.len(), 1);
        assert_eq!(shown[0].placement, crate::hint_orchestrator::wall_shape_placement(WallShape::Circle as u8));
        schedule.run(&mut world);
        assert_eq!(state(&world).count, 1, "没有新命令不再增长");

        let mut world = new_world(AppMode::Wall(WallShape::Freeform));
        world.send_event(rectangle);
        run(&mut world);
        assert_eq!(state(&world).count, 1);
        assert_eq!(state(&world).state, 0, "Freeform 形状字节为 0");
        assert!(requests(&mut world).is_empty());
    }

    /// profile+0x1f4 != 0：整条系统不执行（事件也不被消费）。
    #[test]
    fn tutorial_flag_short_circuits_everything() {
        let mut world = new_world(AppMode::Path);
        world.resource_mut::<country_core::game_profile::GameProfile>().tutorial = true;
        world.send_event(NotifyHintSystemToolWasJustUsed);
        run(&mut world);
        assert_eq!(state(&world).state, 0);
        assert!(requests(&mut world).is_empty());
    }
}