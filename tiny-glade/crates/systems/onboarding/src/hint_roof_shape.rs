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
//! **2026-10-26 复审重写**：去掉 `geo_src_seam`（`ui_edit_focus::geo_src_to_edit_subject`
//! 早已落库）、去掉共享的 `event_payload_byte_seam`（事件载荷字节落库为
//! `NotifyHintSystemRoofShape.flag`）、`Query<&Roof>` 首个 !is_flat 近似改为按
//! `PublicWalls::roof_entity(WallId)` 精确取屋顶；提示图形改为反编译的 Steam×2 +
//! 两帧动画。

use bevy_ecs::prelude::*;

/// 屋顶形状提示条件（O：hints_startup 初值 2B [0]=profile[0x1ff],[1]=profile[0x1ff]）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct HintRoofShapeConditions {
    /// @+0：事件/展示标志（初值 profile[0x1ff]）。
    pub hint_done: bool,
    /// @+1：事件字节（初值 profile[0x1ff]）。
    pub event_byte: u8,
}

/// hint_roof_shape 的 Steam 动作：`FUN_1409e8d90(0)` = `packed_set_plus_one(0)`，
/// 与 `FUN_1409e8d80(1)` = `packed_set(1)`。
const LBM_STEAM_SET_A: u8 = 0;
const LBM_STEAM_SET_B: u8 = 1;

/// make_lbm_drag_binding_hint（O 0x142285270，PDB `hint_roof_shape`）：
/// `Together[Steam(set0 +1), Steam(set1)]`，再 `alternate<HintGraphicParams>`：两帧
/// `["hint/lmb"(8B), "hint/lmb-move"(13B)]`（RNE 0x143038ea0/0x143038ea8 实读）、
/// `0.7` 秒/帧（0x3f333333）、高 `100.0`（0x42c80000）。
pub fn make_lbm_drag_binding_hint() -> crate::BindingHintDisplay {
    crate::BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(
        country_core::resources::steam::packed_set_plus_one(LBM_STEAM_SET_A),
    ))
    .together(crate::hint_orchestrator::steam_query(
        country_core::resources::steam::packed_set(LBM_STEAM_SET_B),
    ))
    .alternate(crate::hint_orchestrator::HintGraphicParams::animated(
        vec!["hint/lmb".to_string(), "hint/lmb-move".to_string()],
        0.7,
        100.,
    ))
}

/// hint_roof_shape（O 0x142284ed0，10 参数）：
/// 1. 逐个 `NotifyHintSystemRoofShape`：`flag==0` → `hint_done=1`；否则 `event_byte=1`
///    并完成 id 7（两个事件缓冲区按序遍历，等价于一个 `EventReader`）。
/// 2. `profile+0x1ff==0 && hint_done && event_byte!=1 && AppMode tag==0` 才继续；
/// 3. `legacy_raycast(CameraCursorRay)`（mask 0xFB、无限距），`+0x30==2`（miss）→ 结束；
/// 4. `geo_src_to_edit_subject(命中句柄)` 必须是 `Roof(id)`；
/// 5. `PublicWalls::roof_entity(id)` 的实体必须在 `Query<&Roof>` 中且 `!is_flat()`；
/// 6. 写 `DisplayHintRequest{Cursor(6), make_lbm_drag_binding_hint(), id 7, flags 0}`。
pub fn hint_roof_shape(
    mut state: ResMut<HintRoofShapeConditions>,
    profile: Res<country_core::game_profile::GameProfile>,
    mut events: EventReader<country_core::resources::onboarding::NotifyHintSystemRoofShape>,
    mode: Res<country_core::systems::mode_manager::AppMode>,
    mut completed: EventWriter<crate::tutorial_completion::TutorialHintComplete>,
    mut request: EventWriter<crate::DisplayHintRequest>,
    camera_ray: Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,
    raycast: Res<country_core::systems::collision::world::RaycastWorld>,
    public_walls: Res<country_core::resources::walls::public_walls::PublicWalls>,
    query: Query<&country_core::resources::roofs::roof_shape::Roof>,
) {
    use country_core::systems::mode_manager::EditSubject;
    use country_core::systems::ui_edit_focus::{GeoSource, geo_src_to_edit_subject};

    for event in events.read() {
        if event.flag == 0 {
            state.hint_done = true;
        } else {
            state.event_byte = 1;
            completed.write(crate::tutorial_completion::TutorialHintComplete { id: 7 });
        }
    }
    if profile.roof_shape_completed
        || !state.hint_done
        || state.event_byte == 1
        || mode.native_tag() != 0
    {
        return;
    }
    let hit = raycast.legacy_raycast(camera_ray.origin, camera_ray.dir);
    if hit.toi.to_bits() == country_core::systems::collision::world::RAYCAST_WORLD_RESULT_MISS
        || hit.is_miss()
    {
        return;
    }
    let EditSubject::Roof(id) = geo_src_to_edit_subject(GeoSource::from_hit_handle(hit.handle)) else {
        return;
    };
    let Some(entity) = public_walls.roof_entity(country_core::resources::walls::WallId(id)) else {
        return;
    };
    let Ok(roof) = query.get(entity) else {
        return;
    };
    if roof.is_flat() {
        return;
    }
    request.write(crate::DisplayHintRequest::new(
        make_lbm_drag_binding_hint(),
        crate::hint_orchestrator::HintType::RoofShape,
        0,
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::system::RunSystemOnce;
    use country_core::resources::onboarding::NotifyHintSystemRoofShape;
    use crate::tutorial_completion::TutorialHintComplete;

    fn world() -> World {
        let mut world = World::new();
        world.insert_resource(HintRoofShapeConditions::default());
        world.insert_resource(country_core::game_profile::GameProfile::default());
        world.insert_resource(country_core::systems::mode_manager::AppMode::default());
        world.insert_resource(country_core::resources::mouse_terrain_raycast::CameraCursorRay::default());
        world.insert_resource(country_core::systems::collision::world::RaycastWorld::default());
        world.insert_resource(country_core::resources::walls::public_walls::PublicWalls::default());
        world.init_resource::<Events<NotifyHintSystemRoofShape>>();
        world.init_resource::<Events<TutorialHintComplete>>();
        world.init_resource::<Events<crate::DisplayHintRequest>>();
        world
    }

    /// 事件字节 0 → hint_done；非 0 → event_byte 并完成 id 7；同帧多个事件按序全部处理。
    #[test]
    fn event_flag_selects_hint_done_or_completion() {
        let mut world = world();
        world.send_event(NotifyHintSystemRoofShape { flag: 0 });
        world.run_system_once(hint_roof_shape).unwrap();
        let state = *world.resource::<HintRoofShapeConditions>();
        assert!(state.hint_done);
        assert_eq!(state.event_byte, 0);
        assert!(world.resource::<Events<TutorialHintComplete>>().is_empty());

        world.send_event(NotifyHintSystemRoofShape { flag: 3 });
        world.run_system_once(hint_roof_shape).unwrap();
        assert_eq!(world.resource::<HintRoofShapeConditions>().event_byte, 1);
        let done: Vec<_> = world.resource_mut::<Events<TutorialHintComplete>>().drain().map(|e| e.id).collect();
        assert_eq!(done, [7]);
        // event_byte==1 之后永不展示（即使 hint_done 为真）。
        assert!(world.resource::<Events<crate::DisplayHintRequest>>().is_empty());
    }

    /// 没有命中屋顶时（默认射线世界 = miss）不展示；完成标志同样压制。
    #[test]
    fn no_roof_under_the_cursor_means_no_display() {
        let mut world = world();
        world.send_event(NotifyHintSystemRoofShape { flag: 0 });
        world.run_system_once(hint_roof_shape).unwrap();
        assert!(world.resource::<HintRoofShapeConditions>().hint_done);
        assert!(world.resource::<Events<crate::DisplayHintRequest>>().is_empty());
    }

    /// 反编译的展示树：Together[Steam(0x0001), Steam(0x0100)] 再 alternate 两帧动画。
    #[test]
    fn lbm_hint_is_two_steam_glyphs_plus_a_two_frame_animation() {
        let crate::BindingHintDisplay::Alternate(parts) = make_lbm_drag_binding_hint() else {
            panic!("Alternate")
        };
        assert_eq!(parts.len(), 2);
        assert_eq!(
            parts[0],
            crate::BindingHintDisplay::Together(vec![
                crate::BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(0x0001)),
                crate::BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(0x0100)),
            ])
        );
        let crate::BindingHintDisplay::Graphic(graphic) = &parts[1] else { panic!("Graphic") };
        assert_eq!(
            graphic.images,
            crate::hint_orchestrator::HintGraphicImages::Animation {
                frames: vec!["hint/lmb".to_string(), "hint/lmb-move".to_string()],
                seconds_per_frame: 0.7,
            }
        );
        assert_eq!(graphic.height, 100.);
    }
}