//! B1408b2b30/2516B. Debug-gated digital34 toggles every reference.
use crate::{
    input::action_set::MainActionSet,
    resources::{cursor_terrain_raycast::CursorTerrainRaycast, gui::DebugGui},
    utils::components::transform::Transform,
};
use bevy_ecs::prelude::*;
use glam::Vec3;
/// Original stride12; Y size is read at+4 when centering the mesh over terrain.
#[derive(Component, Debug, Default)]
pub struct HumanScaleRef(pub Vec3);
pub fn human_scale_ref(
    query: Query<(Entity, Option<&Transform>, &HumanScaleRef)>,
    actions: Res<MainActionSet>,
    cursor: Res<CursorTerrainRaycast>,
    mut commands: Commands,
    debug: Res<DebugGui>,
) {
    if !debug.human_scale_reference || !actions.digital[34].just_pressed() {
        return;
    }
    for (entity, transform, reference) in &query {
        if transform.is_some() {
            commands.entity(entity).remove::<Transform>();
        } else {
            let h = reference.0.y;
            let offset = Vec3::new(h * 0.0, h, h * 0.0) * 0.5;
            commands.entity(entity).insert(Transform {
                translation: cursor.position() + offset,
                ..Transform::identity()
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::action_set::{DigitalActionPhase, DigitalActionState};
    fn bits(t: &Transform) -> Vec<u32> {
        t.rotation
            .to_array()
            .into_iter()
            .chain(t.translation.to_array())
            .chain(t.scale.to_array())
            .map(f32::to_bits)
            .collect()
    }
    #[test]
    fn native40_full_body_debug_toggle_cases() {
        let fixtures:serde_json::Value=serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../reverse-engineering/tiny-glade/evidence/round12/brush-runtime-full/native-human-scale-fixtures.json"))).unwrap();
        for case in fixtures.as_array().unwrap() {
            let input = &case["input"];
            let native = &case["native"];
            let mut world = World::new();
            let mut debug = DebugGui::default();
            debug.human_scale_reference = input["debug_enabled"].as_bool().unwrap();
            world.insert_resource(debug);
            let mut actions = MainActionSet::default();
            let phase = match input["phase"].as_u64().unwrap() {
                0 => DigitalActionPhase::JustPressed,
                1 => DigitalActionPhase::Held,
                2 => DigitalActionPhase::Repeating,
                3 => DigitalActionPhase::JustReleased,
                _ => DigitalActionPhase::Idle,
            };
            actions.digital[34] = DigitalActionState {
                phase,
                timer_seconds: f32::from_bits(input["timer_bits"].as_u64().unwrap() as u32),
            };
            world.insert_resource(actions);
            let mut cursor = CursorTerrainRaycast::new();
            cursor.update(Some(Vec3::new(10., 20., 30.)));
            world.insert_resource(cursor);
            let a = world.spawn(HumanScaleRef(Vec3::new(1., 2., 3.))).id();
            let b = world
                .spawn((
                    HumanScaleRef(Vec3::new(1., 4., 3.)),
                    Transform {
                        translation: Vec3::new(1., 2., 3.),
                        scale: Vec3::splat(2.),
                        ..Transform::identity()
                    },
                ))
                .id();
            let c = world.spawn(HumanScaleRef(Vec3::new(1., f32::NAN, 3.))).id();
            let ids = [a, b, c];
            let mut expected: Vec<Option<Vec<u32>>> = native["transforms"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| {
                    if v.is_null() {
                        None
                    } else {
                        Some(
                            v.as_array()
                                .unwrap()
                                .iter()
                                .map(|x| x.as_u64().unwrap() as u32)
                                .collect(),
                        )
                    }
                })
                .collect();
            for command in native["commands"].as_array().unwrap() {
                if let Some(id) = command["remove"].as_u64() {
                    expected[(id - 0x100000000) as usize] = None;
                }
                if let Some(id) = command["insert"].as_u64() {
                    expected[(id - 0x100000000) as usize] = Some(
                        command["transform_bits"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|x| x.as_u64().unwrap() as u32)
                            .collect(),
                    );
                }
            }
            let mut schedule = Schedule::default();
            schedule.add_systems(human_scale_ref);
            schedule.run(&mut world);
            let actual: Vec<_> = ids
                .into_iter()
                .map(|id| world.get::<Transform>(id).map(bits))
                .collect();
            assert_eq!(actual, expected, "input={input}");
        }
    }
}
