// Append to production signifier_audio_tests.rs after the shared build epoch is released.
fn native_mode(tag: u64, byte: u64) -> Option<AppMode> {
    use crate::systems::mode_manager::{DecoratorType, EraserLayer, WallShape};
    Some(match tag {
        0 => AppMode::Editing(EditSubject::None),
        1 => AppMode::Editing(EditSubject::Wall(0)),
        2 => AppMode::Editing(EditSubject::Roof(0)),
        3 => AppMode::Editing(EditSubject::DecoClutter { decorator_id: 0 }),
        4 => AppMode::Editing(EditSubject::StairPoint { decorator_id: 0 }),
        5 => AppMode::Editing(EditSubject::Terrain(0)),
        6 => AppMode::Wall(match byte {
            0 => WallShape::Freeform,
            1 => WallShape::Circle,
            2 => WallShape::Rectangle,
            _ => return None,
        }),
        7 => AppMode::Path,
        8 => AppMode::Cobblestone(0),
        9 => AppMode::Eraser(EraserLayer::All),
        10 => AppMode::Terrain,
        11 => AppMode::Decorator(DecoratorType::from_native_tag(byte as u8)?),
        12 => AppMode::Color,
        13 => AppMode::Replay,
        14 => AppMode::LoadGame,
        15 => AppMode::Water,
        16 => AppMode::Tree,
        17 => AppMode::Garden,
        18 => AppMode::Fences,
        19 => AppMode::Photomode,
        20 => AppMode::CloneClutter,
        _ => return None,
    })
}
fn native_normalize(calls: &[Call]) -> Vec<serde_json::Value> {
    use serde_json::json;
    let mut temporary = false;
    calls
        .iter()
        .filter_map(|call| {
            Some(match call {
                Call::Create(_) => {
                    temporary = true;
                    return None;
                }
                Call::Release(_) => {
                    temporary = false;
                    return None;
                }
                Call::Global(name, value) => json!(["global", name, value]),
                Call::Start(name) if !temporary && name == SIGNIFIER_EVENTS[0] => {
                    json!(["cached_start", "0x0"])
                }
                Call::Start(name) if !temporary && name == SIGNIFIER_EVENTS[1] => {
                    json!(["cached_start", "0x40"])
                }
                Call::Start(name) if !temporary && name == SIGNIFIER_EVENTS[2] => {
                    json!(["cached_start", "0x70"])
                }
                Call::Start(name) if name == SIGNIFIER_LOOP_EVENT => json!(["loop_start"]),
                Call::Start(name) => json!(["oneshot", name]),
                Call::Parameter(_, name, value) => json!(["parameter", name, value]),
                Call::Position(_, p) => json!(["position", p[0], p[1], p[2]]),
            })
        })
        .collect()
}
#[test]
fn complete_original_executions_match_rust_dispatcher_boundary_effects() {
    let native: serde_json::Value =
        serde_json::from_str(include_str!("signifier_audio_native.json")).unwrap();
    let mut compared = 0;
    for row in native["execution"]["cases"].as_array().unwrap() {
        let (mut world, mut schedule, calls) = fixture();
        match row["case"].as_str().unwrap() {
            "sound" => {
                let payload = row["payload_hex"].as_str().unwrap();
                let mut sound = PlayUiSound {
                    kind: row["kind"].as_u64().unwrap() as u32,
                    ..Default::default()
                };
                for i in 0..20 {
                    sound._payload[i] = u8::from_str_radix(&payload[2 * i..2 * i + 2], 16).unwrap();
                }
                world.send_event(sound);
            }
            "mode" => {
                // Native decorator62 has no valid canonical enum variant.
                let Some(mode) =
                    native_mode(row["tag"].as_u64().unwrap(), row["byte"].as_u64().unwrap())
                else {
                    continue;
                };
                world.send_event(AppModeChanged {
                    current: mode,
                    previous: AppMode::default(),
                });
            }
            "mode_stream" => {
                for pair in row["modes"].as_array().unwrap() {
                    world.send_event(AppModeChanged {
                        current: native_mode(pair[0].as_u64().unwrap(), 0).unwrap(),
                        previous: native_mode(pair[1].as_u64().unwrap(), 0).unwrap(),
                    });
                }
            }
            "photo_height_walls" => {
                world.send_event(PlayPhotomodeSound(0));
                world.send_event(PlayPhotomodeSound(1));
                world.send_event(InitAdjWallHeightMode);
                world.send_event(FinishAdjWallHeightMode);
                world.send_event(OnWallChanged(WallId(1)));
                world.send_event(OnWallChanged(WallId(99)));
            }
            "sound_last_new" => {
                world.send_event(PlayUiSound {
                    kind: 14,
                    ..Default::default()
                });
                world.send_event(PlayUiSound {
                    kind: 15,
                    ..Default::default()
                });
            }
            "sound_last_old" => {
                world.send_event(PlayUiSound {
                    kind: 14,
                    ..Default::default()
                });
                world.send_event(PlayUiSound {
                    kind: 15,
                    ..Default::default()
                });
                world.resource_mut::<Events<PlayUiSound>>().update();
            }
            "sound_new_beats_old" => {
                world.send_event(PlayUiSound {
                    kind: 15,
                    ..Default::default()
                });
                world.resource_mut::<Events<PlayUiSound>>().update();
                world.send_event(PlayUiSound {
                    kind: 14,
                    ..Default::default()
                });
            }
            "wall_mode" => {
                use crate::resources::{
                    roofs::geometry::surface::Curve2,
                    walls::{
                        publication_fields::WallShapeSnapshot,
                        publication_source::TerrainHeightGrid512, InnerWalls, WallState,
                    },
                };
                let mut authority = WallState::new(
                    Curve2::try_new(vec![[0., 0.], [4., 0.]], false).unwrap(),
                    4.,
                    0,
                    row["special"].as_u64().unwrap() as u8,
                    false,
                );
                let mut shape = [0; 28];
                shape[..4].copy_from_slice(&(row["shape"].as_u64().unwrap() as u32).to_le_bytes());
                authority.snapshot.source.shape = WallShapeSnapshot::from_observed(shape);
                let mut inner = InnerWalls::new();
                inner.new_wall_with_id(WallId(99), authority);
                let terrain = TerrainHeightGrid512 {
                    encoded: vec![0u16; 512 * 512].into(),
                    minimum_y: 0.,
                    maximum_y: 0.,
                    sampling_fix: false,
                };
                world
                    .resource_mut::<PublicWalls>()
                    .consume(&mut inner, &terrain);
                assert!(world.resource::<PublicWalls>().get(WallId(99)).is_some());
                world.send_event(AppModeChanged {
                    current: AppMode::Editing(EditSubject::Wall(99)),
                    previous: AppMode::default(),
                });
            }
            _ => panic!("unclassified original fixture {row}"),
        }
        schedule.run(&mut world);
        let expected: Vec<_> = row["trace"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|entry| entry[0] != "loop_stop")
            .cloned()
            .collect();
        assert_eq!(native_normalize(&calls.lock().unwrap()), expected, "{row}");
        let state = world.resource::<SignifierAudioState<Handle>>();
        let expected_wall = if row["cached_wall"][0].as_u64().unwrap() == 0 {
            None
        } else {
            Some(WallId(row["cached_wall"][1].as_u64().unwrap()))
        };
        assert_eq!(state.last_changed_wall, expected_wall);
        compared += 1;
    }
    assert_eq!(
        compared, 77,
        "only invalid native decorator62 is outside canonical enum domain"
    );
}
