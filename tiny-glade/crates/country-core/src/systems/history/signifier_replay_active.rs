//! B140b1b860/1959B, original replay preview lifecycle and damped pulse.
pub use super::replay_mode::ReplayModeElapsedTime;
use crate::{
    resources::{
        cursor_terrain_raycast::CursorTerrainRaycast, history::ReplayBuffer, time::GameTime,
    },
    systems::camera_rig::main_camera::MainCamera,
    utils::components::transform::Transform,
};
use ::utils::scalar::{clamp01, max_f32};
use bevy_ecs::prelude::*;
use glam::Vec3;
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignifierReplayIndex(pub usize);
fn pulse(phase: f32) -> f32 {
    let wave = (phase * f32::from_bits(0x40fb53d2)).sin();
    (phase * -10.).exp2() * wave * 0.5
}
pub fn signifier_replay_active(
    replay: Res<ReplayBuffer>,
    mut elapsed: ResMut<ReplayModeElapsedTime>,
    mut query: Query<(ComponentWrite<Transform>, &SignifierReplayIndex)>,
    camera: Res<MainCamera>,
    time: Res<GameTime>,
    cursor: Res<CursorTerrainRaycast>,
    speed: Res<ReplayBuffer>,
) {
    let Some(elapsed) = elapsed.0.as_mut() else {
        return;
    };
    *elapsed = time.delta + *elapsed;
    let period = max_f32(f32::from_bits(0x3f666666) / speed.speed, 0.5);
    let normalized = *elapsed / period;
    let mut phase = normalized - normalized.trunc();
    for (mut transform, index) in &mut query {
        let Some(front) = replay.buckets.front() else {
            transform.translation = cursor.position();
            continue;
        };
        let up = if front.undo {
            Vec3::new(-0.0, -1., -0.0)
        } else {
            Vec3::Y
        };
        transform.look_at(camera.eye_position(), up);
        let target = if index.0 == 1 {
            -0.15
        } else {
            // Original phase is shared across the query loop: preserve mutation
            // for later rows, including repeated secondary indices.
            phase = clamp01(phase + f32::from_bits(0xbd4ccccd));
            0.15
        };
        let oscillation = pulse(phase);
        let direction = transform.rotation * Vec3::X;
        let base = cursor.position() - direction * oscillation * f32::from_bits(0x40333333);
        transform.translation = direction * target * f32::from_bits(0x40333333) + base;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::history::replay_cmd_buffer::ReplayBucket;
    #[test]
    fn actual_schedule_matches90_native_full_function_cases() {
        let fixtures:serde_json::Value=serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../reverse-engineering/tiny-glade/evidence/round12/brush-runtime-full/native-signifier-replay-fixtures.json"))).unwrap();
        for case in fixtures.as_array().unwrap() {
            let input = &case["input"];
            let native = &case["native"];
            let mut world = World::new();
            let mut replay = ReplayBuffer::new(1.);
            if input["pending"].as_bool().unwrap() {
                replay.push(ReplayBucket {
                    entry_index: 0,
                    undo: input["undo"].as_bool().unwrap(),
                    commands: vec![],
                    timing: vec![],
                });
            }
            world.insert_resource(replay);
            world.insert_resource(ReplayModeElapsedTime(
                input["elapsed"].as_f64().map(|f| f as f32),
            ));
            world.insert_resource(GameTime {
                delta: 0.016,
                ..Default::default()
            });
            let mut cursor = CursorTerrainRaycast::new();
            cursor.update(Some(Vec3::new(10., 20., 30.)));
            world.insert_resource(cursor);
            let mut camera = MainCamera::default();
            camera.set_camera_matrices(glam::Mat4::from_translation(Vec3::new(15., 25., 35.)));
            world.insert_resource(camera);
            let ids: Vec<_> = input["rows"]
                .as_array()
                .unwrap()
                .iter()
                .map(|r| {
                    world
                        .spawn((
                            SignifierReplayIndex(r[0].as_u64().unwrap() as usize),
                            Transform {
                                translation: Vec3::new(1., 2., 3.),
                                scale: Vec3::splat(2.),
                                ..Transform::identity()
                            },
                        ))
                        .id()
                })
                .collect();
            let mut schedule = Schedule::default();
            schedule.add_systems(signifier_replay_active);
            schedule.run(&mut world);
            let elapsed = world.resource::<ReplayModeElapsedTime>().0;
            if input["elapsed"].is_null() {
                assert_eq!(elapsed, None);
            } else {
                assert_eq!(
                    elapsed.unwrap().to_bits(),
                    native["elapsed_bits"].as_u64().unwrap() as u32
                );
            }
            for (id, expected) in ids
                .into_iter()
                .zip(native["transforms"].as_array().unwrap())
            {
                let t = world.get::<Transform>(id).unwrap();
                let actual: Vec<_> = t
                    .rotation
                    .to_array()
                    .into_iter()
                    .chain(t.translation.to_array())
                    .chain(t.scale.to_array())
                    .collect();
                for (i, (a, b)) in actual
                    .into_iter()
                    .zip(expected.as_array().unwrap())
                    .enumerate()
                {
                    let b = f32::from_bits(b.as_u64().unwrap() as u32);
                    assert!(
                        (a - b).abs() < 0.00005,
                        "input={input},field={i},actual={a:?},original={b:?}"
                    );
                }
            }
        }
    }
}

use crate::render::extract::ComponentWrite;
