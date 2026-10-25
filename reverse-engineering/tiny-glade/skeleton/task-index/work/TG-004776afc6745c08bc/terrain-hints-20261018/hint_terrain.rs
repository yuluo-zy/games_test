//! B hint_terrain_layering 0x142283cc0: real terrain notifications and hover gate.
//!
//! 真逻辑：工具==10（Terrain）且 GameProfile+0x1f9==0 且事件计数>1 且悬停 tag 非零且条件尚未完成 → alternate(Option<KeyCode>) 写 DisplayHintRequest（+0x48 u16 = 0x0600
//! → id 6 Terrain，flags 0）；
//! 事件 tag1 且尚未完成 → 写 TutorialHintComplete（tag 6）。

use bevy_ecs::prelude::*;

/// 地形分层提示条件（O：hints_startup 初值 9B {u64 count@+0=0, u8 hint_done@+8=
/// profile[0x1f9]}；terrain_layering count++/`1<count` 门控、byte@+8 事件标志）。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct HintTerrainConditions {
    /// @+0：完成绘笔 tag0 计数；原 INC 按 u64 回绕。
    pub count: u64,
    /// @+8：事件/完成标志（初值 profile[0x1f9]）。
    pub hint_done: bool,
    /// @+9：保留字节（O 未写）。
    pub event_byte: u8,
}

/// hint_terrain_layering（O 0x142283cc0）：遍历 NotifyHintSystemTerrain 事件；tag0 才 count++；
/// 事件 tag1 且状态 hint_done==0 → TutorialHintComplete(6)、byte8=1；末位：AppMode==10 且 profile 0x1f9==0 且 count>1 且
/// 悬停 tag 非零且 byte8==0 → Steam+Key alternate 写 DisplayHintRequest（id 6，flags 0）。
/// B 原事件首字节由真实 producer 提供；末位读取 HoveredTerrainStrokeId.tag。
pub fn hint_terrain_layering(
    profile: Res<country_core::game_profile::GameProfile>,
    mut state: ResMut<HintTerrainConditions>,
    mode: Res<country_core::systems::mode_manager::AppMode>,
    mut events: EventReader<country_core::resources::onboarding::NotifyHintSystemTerrain>,
    mut request: EventWriter<crate::DisplayHintRequest>,
    mut completed: EventWriter<crate::tutorial_completion::TutorialHintComplete>,
    hovered: Res<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>,
) {
    for event in events.read() {
        match event.tag {
            0 => state.count = state.count.wrapping_add(1),
            1 => {
                if !state.hint_done {
                    completed.write(crate::tutorial_completion::TutorialHintComplete { id: 6 });
                }
                state.hint_done = true;
            }
            _ => {}
        }
    }
    if mode.native_tag() == 10
        && !profile.terrain_layering_completed
        && 1 < state.count
        && hovered.tag != 0
        && !state.hint_done
    {
        let display = crate::BindingHintDisplay::Steam(crate::hint_orchestrator::steam_query(
            country_core::resources::steam::packed_set_plus_one(0),
        ))
        .alternate(None::<crate::KeyCode>);
        request.write(crate::DisplayHintRequest::new(
            display,
            crate::hint_orchestrator::HintType::Terrain,
            0,
        ));
    }
}

#[cfg(test)]
mod native_regressions {
    use super::*;
    use crate::{
        hint_terrain_focus::{HintFocusTerrainConditions, hint_terrain_focus},
        hint_terrain_steepness::{HintTerrainSteepnessConditions, hint_terrain_steepness},
        tutorial_completion::TutorialHintComplete,
    };
    use country_core::{
        game_profile::GameProfile,
        resources::onboarding::{NotifyHintSystemTerrain, NotifyHintSystemTerrainSteepness},
        systems::{mode_manager::AppMode, terrain::terrain_hover_idx::HoveredTerrainStrokeId},
    };

    fn world() -> World {
        let mut world = World::new();
        world.insert_resource(GameProfile::default());
        world.insert_resource(AppMode::default());
        world.insert_resource(HoveredTerrainStrokeId::default());
        world.init_resource::<HintTerrainConditions>();
        world.init_resource::<HintFocusTerrainConditions>();
        world.init_resource::<HintTerrainSteepnessConditions>();
        world.init_resource::<Events<NotifyHintSystemTerrain>>();
        world.init_resource::<Events<NotifyHintSystemTerrainSteepness>>();
        world.init_resource::<Events<crate::DisplayHintRequest>>();
        world.init_resource::<Events<TutorialHintComplete>>();
        world
    }

    #[test]
    fn original_b_tag_sequences_preserve_state_and_completion_order() {
        // Actual native42 cases, with its display backend excluded by mode0.
        let cases: &[(&str, &[u8], bool, u64, bool, u8, &[u8])] = &[
            ("layering", &[], false, 0, false, 0, &[]),
            ("layering", &[], true, 0, true, 0, &[]),
            ("layering", &[0], false, 1, false, 0, &[]),
            ("layering", &[0], true, 1, true, 0, &[]),
            ("layering", &[1], false, 0, true, 0, &[6]),
            ("layering", &[1], true, 0, true, 0, &[]),
            ("layering", &[2], false, 0, false, 0, &[]),
            ("layering", &[2], true, 0, true, 0, &[]),
            ("layering", &[0, 0, 1], false, 2, true, 0, &[6]),
            ("layering", &[0, 0, 1], true, 2, true, 0, &[]),
            ("layering", &[1, 1], false, 0, true, 0, &[6]),
            ("layering", &[1, 1], true, 0, true, 0, &[]),
            ("layering", &[2, 2], false, 0, false, 0, &[]),
            ("layering", &[2, 2], true, 0, true, 0, &[]),
            ("focus", &[], false, 0, false, 0, &[]),
            ("focus", &[], true, 0, true, 0, &[]),
            ("focus", &[0], false, 0, false, 0, &[]),
            ("focus", &[0], true, 0, true, 0, &[]),
            ("focus", &[1], false, 0, false, 0, &[]),
            ("focus", &[1], true, 0, true, 0, &[]),
            ("focus", &[2], false, 0, true, 0, &[5]),
            ("focus", &[2], true, 0, true, 0, &[5]),
            ("focus", &[0, 0, 1], false, 0, false, 0, &[]),
            ("focus", &[0, 0, 1], true, 0, true, 0, &[]),
            ("focus", &[1, 1], false, 0, false, 0, &[]),
            ("focus", &[1, 1], true, 0, true, 0, &[]),
            ("focus", &[2, 2], false, 0, true, 0, &[5, 5]),
            ("focus", &[2, 2], true, 0, true, 0, &[5, 5]),
            ("steepness", &[], false, 0, false, 0, &[]),
            ("steepness", &[], true, 0, true, 1, &[]),
            ("steepness", &[0], false, 0, true, 0, &[]),
            ("steepness", &[0], true, 0, true, 1, &[]),
            ("steepness", &[1], false, 0, false, 1, &[4]),
            ("steepness", &[1], true, 0, true, 1, &[4]),
            ("steepness", &[2], false, 0, false, 1, &[4]),
            ("steepness", &[2], true, 0, true, 1, &[4]),
            ("steepness", &[0, 0, 1], false, 0, true, 1, &[4]),
            ("steepness", &[0, 0, 1], true, 0, true, 1, &[4]),
            ("steepness", &[1, 1], false, 0, false, 1, &[4, 4]),
            ("steepness", &[1, 1], true, 0, true, 1, &[4, 4]),
            ("steepness", &[2, 2], false, 0, false, 1, &[4, 4]),
            ("steepness", &[2, 2], true, 0, true, 1, &[4, 4]),
        ];
        for &(kind, tags, initial_done, count, done, flag, completed) in cases {
            let mut world = world();
            world.resource_mut::<HintTerrainConditions>().hint_done = initial_done;
            world.resource_mut::<HintFocusTerrainConditions>().hint_done = initial_done;
            world
                .resource_mut::<HintTerrainSteepnessConditions>()
                .hint_done = initial_done;
            world
                .resource_mut::<HintTerrainSteepnessConditions>()
                .event_byte = initial_done as u8;
            let mut schedule = Schedule::default();
            match kind {
                "layering" => {
                    schedule.add_systems(hint_terrain_layering);
                }
                "focus" => {
                    schedule.add_systems(hint_terrain_focus);
                }
                "steepness" => {
                    schedule.add_systems(hint_terrain_steepness);
                }
                _ => unreachable!(),
            }
            for &tag in tags {
                if kind == "steepness" {
                    world.send_event(NotifyHintSystemTerrainSteepness { tag });
                } else {
                    world.send_event(NotifyHintSystemTerrain { tag });
                }
            }
            schedule.run(&mut world);
            match kind {
                "layering" => {
                    let state = world.resource::<HintTerrainConditions>();
                    assert_eq!(state.count, count);
                    assert_eq!(state.hint_done, done);
                }
                "focus" => assert_eq!(
                    world.resource::<HintFocusTerrainConditions>().hint_done,
                    done
                ),
                "steepness" => {
                    let state = world.resource::<HintTerrainSteepnessConditions>();
                    assert_eq!(state.hint_done, done);
                    assert_eq!(state.event_byte, flag);
                }
                _ => unreachable!(),
            }
            let mut reader = bevy_ecs::event::EventCursor::<TutorialHintComplete>::default();
            let actual: Vec<_> = reader
                .read(world.resource::<Events<TutorialHintComplete>>())
                .map(|event| event.id)
                .collect();
            assert_eq!(actual.as_slice(), completed, "{kind} {tags:?}");
        }
    }

    #[test]
    fn display_guards_require_hover_even_with_other_entities() {
        let mut world = world();
        world.insert_resource(AppMode::Terrain);
        world.resource_mut::<HintTerrainConditions>().count = 2;
        world
            .resource_mut::<HintTerrainSteepnessConditions>()
            .hint_done = true;
        world.spawn_empty();
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                hint_terrain_layering,
                hint_terrain_focus,
                hint_terrain_steepness,
            )
                .chain(),
        );
        schedule.run(&mut world);
        assert_eq!(
            world.resource::<Events<crate::DisplayHintRequest>>().len(),
            0
        );
        world.resource_mut::<HoveredTerrainStrokeId>().tag = 1;
        schedule.run(&mut world);
        let mut reader = bevy_ecs::event::EventCursor::<crate::DisplayHintRequest>::default();
        let ids: Vec<_> = reader
            .read(world.resource::<Events<crate::DisplayHintRequest>>())
            .map(|request| request.id)
            .collect();
        assert_eq!(ids, [6, 1, 4]);
    }
    #[test]
    fn configured_cpu_chain_completes_profile_and_requests_settings_once_per_event() {
        use country_core::resources::events::WriteSettingsFileCmd;
        let mut world = World::new();
        world.insert_resource(GameProfile::default());
        world.insert_resource(AppMode::Terrain);
        let mut schedule = Schedule::default();
        crate::configure(&mut world, &mut schedule);
        world.send_event(NotifyHintSystemTerrain { tag: 1 });
        world.send_event(NotifyHintSystemTerrain { tag: 2 });
        world.send_event(NotifyHintSystemTerrainSteepness { tag: 1 });
        schedule.run(&mut world);
        let profile = world.resource::<GameProfile>();
        assert!(profile.terrain_layering_completed);
        assert!(profile.terrain_focus_completed);
        assert!(profile.terrain_steepness_completed);
        assert_eq!(world.resource::<Events<WriteSettingsFileCmd>>().len(), 3);
        world.resource_mut::<HintTerrainConditions>().count = 9;
        schedule.run(&mut world);
        assert_eq!(world.resource::<HintTerrainConditions>().count, 9);
        assert_eq!(world.resource::<Events<WriteSettingsFileCmd>>().len(), 3);
    }

    #[test]
    fn configure_reuses_profile_startup_without_resetting_supplied_conditions() {
        let mut world = World::new();
        let mut profile = GameProfile::default();
        profile.terrain_layering_completed = true;
        profile.terrain_focus_completed = true;
        profile.terrain_steepness_completed = true;
        world.insert_resource(profile);
        let mut first = Schedule::default();
        crate::configure(&mut world, &mut first);
        assert!(world.resource::<HintTerrainConditions>().hint_done);
        assert!(world.resource::<HintFocusTerrainConditions>().hint_done);
        assert_eq!(
            world
                .resource::<HintTerrainSteepnessConditions>()
                .event_byte,
            1
        );
        world.resource_mut::<HintTerrainConditions>().count = 9;
        let mut second = Schedule::default();
        crate::configure(&mut world, &mut second);
        assert_eq!(world.resource::<HintTerrainConditions>().count, 9);
    }
}
