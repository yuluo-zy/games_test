//! Compare semantic snapshot fields with original B instruction output, then
//! exercise the same mapping through the production ECS capture/restore pair.
use bevy_ecs::prelude::*;
use country_core::{
    resources::critters::{
        CritterGoal,
        sheep::{
            Sheep, SheepMoveState,
            control::{SheepControlSnapshot, SheepSnapshot, capture_sheep_snapshot},
        },
    },
    systems::debug::sheep_controls::{SheepControl, sheep_controls},
    utils::components::transform::Transform,
};
use glam::{Quat, Vec3};
use serde::Deserialize;

#[derive(Deserialize)]
struct NativeFixture {
    cases: Vec<NativeCase>,
}
#[derive(Deserialize)]
struct NativeCase {
    rows: Vec<InputRow>,
    native_snapshot_hex: Vec<String>,
}
#[derive(Deserialize)]
struct InputRow {
    position: Vec3,
    goal: Option<Vec3>,
    state: InputState,
}
#[derive(Deserialize)]
struct InputState {
    tag: u32,
    force: bool,
    timeout: f32,
    goal_timeout: f32,
}
impl InputState {
    fn movement(&self) -> SheepMoveState {
        if self.tag == 0 {
            SheepMoveState::Idle {
                timeout_s: self.timeout,
            }
        } else {
            SheepMoveState::Walking {
                timeout_s: self.timeout,
                goal_change_timeout: self.goal_timeout,
                force_reach_goal: self.force,
            }
        }
    }
}

/// Offsets are established by the original serializer's actual r9 arguments,
/// independently of Rust declaration order and the capture implementation.
fn original_snapshot(encoded: &str) -> SheepSnapshot {
    let bytes = hex_bytes(encoded);
    let word = |at| u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap());
    let scalar = |at| f32::from_bits(word(at));
    let vector = |at| Vec3::new(scalar(at), scalar(at + 4), scalar(at + 8));
    SheepSnapshot {
        position: vector(28),
        goal: (word(0) != 0).then(|| vector(4)),
        move_state: if bytes[16] == 0 {
            SheepMoveState::Idle {
                timeout_s: scalar(20),
            }
        } else {
            SheepMoveState::Walking {
                timeout_s: scalar(20),
                goal_change_timeout: scalar(24),
                force_reach_goal: bytes[17] != 0,
            }
        },
    }
}

fn hex_bytes(s: &str) -> Vec<u8> {
    s.as_bytes()
        .chunks_exact(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}

#[test]
fn capture_matches_eighty_original_instruction_records() {
    let fixture: NativeFixture =
        serde_json::from_str(include_str!("fixtures/sheep-snapshot-projection-b.json")).unwrap();
    let mut count = 0;
    for case in fixture.cases {
        for (row, native) in case.rows.iter().zip(&case.native_snapshot_hex) {
            let transform = Transform::from_translation(row.position);
            let sheep = Sheep {
                behavior_state: 0,
                goal_timer: 0.0,
                pet_phase: 0,
                pet_progress: 0.0,
                pet_target: 0,
                control_flag: 0,
                move_state: row.state.movement(),
                amplitude: 99.,
            };
            let goal = CritterGoal {
                goal_tag: u32::from(row.goal.is_some()),
                position: row.goal.unwrap_or_default(),
            };
            let captured = capture_sheep_snapshot([(&transform, &sheep, &goal)]);
            assert_eq!(captured.sheep, [original_snapshot(native)]);
            count += 1;
        }
    }
    assert_eq!(count, 80);
}

#[test]
fn ecs_freeze_preserves_other_fields_and_extra_entities() {
    let mut world = World::new();
    world.insert_resource(country_core::systems::mode_manager::AppMode::default());
    world.insert_resource(country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::default());
    world.insert_resource(country_core::systems::ui::resources::ui_state::UiState::at_startup());
    world.insert_resource(country_core::resources::gui::DebugGui::default());
    world.insert_resource(bevy_input::ButtonInput::<bevy_input::keyboard::KeyCode>::default());

    world
        .init_resource::<Events<country_core::systems::history::finish_replay::CmdUnfreezeSheep>>();
    let original = Transform {
        rotation: Quat::from_rotation_y(0.7),
        translation: Vec3::splat(21.),
        scale: Vec3::splat(3.),
    };
    let old_sheep = Sheep {
        behavior_state: 0,
        goal_timer: 0.0,
        pet_phase: 0,
        pet_progress: 0.0,
        pet_target: 0,
        control_flag: 0,
        move_state: SheepMoveState::Idle { timeout_s: 10. },
        amplitude: 7.,
    };
    let old_goal = CritterGoal {
        goal_tag: 1,
        position: Vec3::splat(31.),
    };
    let entity = world.spawn((original, old_sheep, old_goal)).id();
    let extra = world.spawn((original, old_sheep, old_goal)).id();
    let saved = SheepSnapshot {
        position: Vec3::new(1., 2., 3.),
        goal: Some(Vec3::new(4., 5., 6.)),
        move_state: SheepMoveState::Walking {
            timeout_s: 8.,
            goal_change_timeout: 9.,
            force_reach_goal: true,
        },
    };
    let wireframe = world.spawn_empty().id();
    let solid = world.spawn_empty().id();
    let control_owner = world
        .spawn(SheepControl {
            snapshot: Some(SheepControlSnapshot {
                sheep: vec![saved.clone()],
            }),
            ..SheepControl::new(wireframe, solid)
        })
        .id();
    let mut schedule = Schedule::default();
    schedule.add_systems(sheep_controls);
    schedule.run(&mut world);
    let transform = world.get::<Transform>(entity).unwrap();
    assert_eq!(transform.translation, saved.position);
    assert_eq!(transform.rotation, original.rotation);
    assert_eq!(transform.scale, original.scale);
    assert_eq!(world.get::<Sheep>(entity).unwrap().amplitude, 7.);
    assert_eq!(world.get::<Transform>(extra).unwrap(), &original);
    assert_eq!(world.get::<Sheep>(extra).unwrap(), &old_sheep);
    assert_eq!(world.get::<CritterGoal>(extra).unwrap(), &old_goal);
    assert_eq!(
        world
            .get::<SheepControl>(control_owner)
            .unwrap()
            .snapshot
            .as_ref()
            .unwrap()
            .sheep[0],
        saved
    );
}

#[test]
fn save_section_captures_live_sheep_independently_of_debug_freeze() {
    use country_core::systems::save::save_game::{
        SheepSaveSectionValue, sheep_save_section_system,
    };
    let mut world = World::new();
    world.insert_resource(country_core::systems::mode_manager::AppMode::default());
    world.insert_resource(country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::default());
    world.insert_resource(country_core::systems::ui::resources::ui_state::UiState::at_startup());
    world.insert_resource(country_core::resources::gui::DebugGui::default());
    world.insert_resource(bevy_input::ButtonInput::<bevy_input::keyboard::KeyCode>::default());

    world.init_resource::<SheepSaveSectionValue>();
    let wireframe = world.spawn_empty().id();
    let solid = world.spawn_empty().id();
    let control_owner = world
        .spawn(SheepControl {
            snapshot: Some(SheepControlSnapshot { sheep: vec![] }),
            ..SheepControl::new(wireframe, solid)
        })
        .id();
    let mut schedule = Schedule::default();
    schedule.add_systems(sheep_save_section_system);
    schedule.run(&mut world);
    assert_eq!(
        world.resource::<SheepSaveSectionValue>().0.as_str(),
        "{\"sheep\":[]}"
    );

    let transform = Transform::from_translation(Vec3::new(1.0, 2.0, 3.0));
    let sheep = Sheep {
        behavior_state: 0,
        goal_timer: 0.0,
        pet_phase: 0,
        pet_progress: 0.0,
        pet_target: 0,
        control_flag: 0,
        move_state: SheepMoveState::Walking {
            timeout_s: 4.0,
            goal_change_timeout: 5.0,
            force_reach_goal: true,
        },
        amplitude: 99.0,
    };
    let goal = CritterGoal {
        position: Vec3::new(6.0, 7.0, 8.0),
        goal_tag: 1,
    };
    world.spawn((transform, sheep, goal));
    schedule.run(&mut world);
    let saved: SheepControlSnapshot =
        serde_json::from_str(world.resource::<SheepSaveSectionValue>().0.as_str()).unwrap();
    assert_eq!(saved, capture_sheep_snapshot([(&transform, &sheep, &goal)]));
    assert!(
        world
            .get::<SheepControl>(control_owner)
            .unwrap()
            .snapshot
            .as_ref()
            .unwrap()
            .sheep
            .is_empty()
    );
}
