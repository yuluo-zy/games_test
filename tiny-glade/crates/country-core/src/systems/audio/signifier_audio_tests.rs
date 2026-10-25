use super::*;
use crate::resources::audio_instance::PlaybackState;
use crate::resources::audio_resources::{ListenerAttributes, VcaHandle};
use crate::systems::mode_manager::AppModeChanged;
use crate::systems::ui::signifiers::UiSignifierStream;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq)]
enum Call {
    Create(String),
    Start(String),
    Release(String),
    Global(String, f32),
    Parameter(String, String, f32),
    Position(String, [f32; 3]),
}
#[derive(Debug)]
struct Backend(Arc<Mutex<Vec<Call>>>);
struct Handle {
    name: String,
    calls: Arc<Mutex<Vec<Call>>>,
}
struct Vca;
impl VcaHandle for Vca {
    type Error = &'static str;
    fn set_volume(&self, _: f32) -> Result<(), Self::Error> {
        Ok(())
    }
}
impl EventInstance for Handle {
    type Error = &'static str;
    fn start(&mut self) -> Result<(), Self::Error> {
        self.calls
            .lock()
            .unwrap()
            .push(Call::Start(self.name.clone()));
        Ok(())
    }
    fn stop(&mut self, _: i32) -> Result<(), Self::Error> {
        Ok(())
    }
    fn get_playback_state(&self) -> Result<PlaybackState, Self::Error> {
        Ok(PlaybackState::Stopped)
    }
    fn set_volume(&mut self, _: f32) -> Result<(), Self::Error> {
        Ok(())
    }
    fn set_parameter_by_name(&mut self, name: &str, value: f32) -> Result<(), Self::Error> {
        self.calls
            .lock()
            .unwrap()
            .push(Call::Parameter(self.name.clone(), name.into(), value));
        Ok(())
    }
    fn has_parameter(&self, _: &str) -> Result<bool, Self::Error> {
        Ok(false)
    }
    fn set_3d_attributes(
        &mut self,
        position: [f32; 3],
        _: [f32; 3],
        _: [f32; 3],
        _: [f32; 3],
    ) -> Result<(), Self::Error> {
        self.calls
            .lock()
            .unwrap()
            .push(Call::Position(self.name.clone(), position));
        Ok(())
    }
    fn get_timeline_position(&self) -> Result<i32, Self::Error> {
        Ok(0)
    }
    fn pause(&mut self, _: bool) -> Result<(), Self::Error> {
        Ok(())
    }
    fn install_beat_callback(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn release(&mut self) -> Result<(), Self::Error> {
        self.calls
            .lock()
            .unwrap()
            .push(Call::Release(self.name.clone()));
        Ok(())
    }
}
impl Studio for Backend {
    type Error = &'static str;
    type Event = Handle;
    type Vca = Vca;
    fn load_bank_file(&mut self, _: &str) -> Result<(), Self::Error> {
        Ok(())
    }
    fn load_sample_data(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn get_vca(&self, _: &str) -> Result<Vca, Self::Error> {
        Ok(Vca)
    }
    fn set_listener_attributes(&mut self, _: ListenerAttributes) -> Result<(), Self::Error> {
        Ok(())
    }
    fn update(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn set_parameter_by_name(&self, name: &str, value: f32) -> Result<(), Self::Error> {
        self.0
            .lock()
            .unwrap()
            .push(Call::Global(name.into(), value));
        Ok(())
    }
    fn create_event_instance(&self, name: &str) -> Result<Option<Handle>, Self::Error> {
        self.0.lock().unwrap().push(Call::Create(name.into()));
        Ok(Some(Handle {
            name: name.into(),
            calls: self.0.clone(),
        }))
    }
}
fn fixture() -> (World, Schedule, Arc<Mutex<Vec<Call>>>) {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let core = AudioCore {
        studio: Some(Backend(calls.clone())),
        banks: Default::default(),
    };
    let state = SignifierAudioState::new(&core);
    let mut world = World::new();
    world.insert_resource(core);
    world.insert_resource(state);
    crate::systems::audio::add_audio_events(&mut world);
    world.init_resource::<PublicWalls>();
    world.init_resource::<UiSignifierStream>();
    let mut schedule = Schedule::default();
    schedule.add_systems(signifier_audio_system::<Backend>);
    calls.lock().unwrap().clear();
    (world, schedule, calls)
}
#[test]
fn native_table_all36_variants_dispatch_exact_names_and_release_one_shots() {
    let native: serde_json::Value =
        serde_json::from_str(include_str!("signifier_audio_native.json")).unwrap();
    for row in native["sound_jump_table"].as_array().unwrap() {
        let kind = row["kind"].as_u64().unwrap() as u32;
        if matches!(kind, 1 | 34) {
            continue;
        }
        let (mut world, mut schedule, calls) = fixture();
        world.send_event(PlayUiSound {
            kind,
            ..Default::default()
        });
        schedule.run(&mut world);
        let expected = match row["event"].as_str() {
            Some(name) => vec![
                Call::Create(name.into()),
                Call::Start(name.into()),
                Call::Release(name.into()),
            ],
            None => Vec::new(),
        };
        assert_eq!(
            *calls.lock().unwrap(),
            expected,
            "original sound kind {kind}"
        );
        calls.lock().unwrap().clear();
        schedule.run(&mut world);
        assert!(
            calls.lock().unwrap().is_empty(),
            "sound reader must advance"
        );
    }
}
#[test]
fn last_unread_sound_wins_preserves_u64_boop_context_and_never_consumes_visual_stream() {
    let (mut world, mut schedule, calls) = fixture();
    world
        .resource_mut::<UiSignifierStream>()
        .write_raw([1, 2, 3, 4, 5]);
    let committed = world.resource::<UiSignifierStream>().committed_len();
    world.send_event(PlayUiSound {
        kind: 15,
        ..Default::default()
    });
    let mut boop = PlayUiSound {
        kind: 1,
        ..Default::default()
    };
    boop._payload[4..12].copy_from_slice(&u64::MAX.to_le_bytes());
    world.send_event(boop);
    schedule.run(&mut world);
    assert_eq!(
        *calls.lock().unwrap(),
        [
            Call::Global("BoopContext".into(), u64::MAX as f32),
            Call::Start(SIGNIFIER_EVENTS[0].into())
        ]
    );
    assert_eq!(
        world.resource::<UiSignifierStream>().committed_len(),
        committed
    );
}
#[test]
fn rotate_loop_owns_original_ring_complete_position_speed_and_stop_request() {
    let (mut world, mut schedule, calls) = fixture();
    let ring = &world
        .resource::<SignifierAudioState<Handle>>()
        .rotate_moving_back;
    assert_eq!(ring.entries.len(), 4);
    assert_eq!(ring.threshold_ms, 50);
    let mut sound = PlayUiSound {
        kind: 34,
        ..Default::default()
    };
    for (offset, value) in [(0, -2.5f32), (4, 1.), (8, -3.), (12, 8.)] {
        sound._payload[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    world.send_event(sound);
    schedule.run(&mut world);
    assert_eq!(
        *calls.lock().unwrap(),
        [
            Call::Start(SIGNIFIER_LOOP_EVENT.into()),
            Call::Position(SIGNIFIER_LOOP_EVENT.into(), [1., -3., 8.]),
            Call::Parameter(SIGNIFIER_LOOP_EVENT.into(), "speed".into(), -2.5)
        ]
    );
    assert!(world
        .resource::<SignifierAudioState<Handle>>()
        .rotate_moving_back
        .start
        .is_none());
    schedule.run(&mut world);
    assert!(world
        .resource::<SignifierAudioState<Handle>>()
        .rotate_moving_back
        .start
        .is_some());
}
#[test]
fn photo_reads_every_record_mode_skips_replay_and_height_edges_use_cached_instances() {
    let (mut world, mut schedule, calls) = fixture();
    world.send_event(PlayPhotomodeSound(0));
    world.send_event(PlayPhotomodeSound(1));
    world.send_event(AppModeChanged {
        current: AppMode::Water,
        previous: AppMode::Replay,
    });
    world.send_event(AppModeChanged {
        current: AppMode::Tree,
        previous: AppMode::Path,
    });
    world.send_event(AppModeChanged {
        current: AppMode::Garden,
        previous: AppMode::Tree,
    });
    world.send_event(OnWallChanged(WallId(1)));
    world.send_event(OnWallChanged(WallId(99)));
    world.send_event(InitAdjWallHeightMode);
    world.send_event(InitAdjWallHeightMode);
    world.send_event(FinishAdjWallHeightMode::default());
    schedule.run(&mut world);
    let mut expected = Vec::new();
    for name in [FIRST_PERSON_ON, FIRST_PERSON_OFF] {
        expected.extend([
            Call::Create(name.into()),
            Call::Start(name.into()),
            Call::Release(name.into()),
        ]);
    }
    expected.extend([
        Call::Global("Tool".into(), 13.),
        Call::Global("Tool".into(), 8.),
        Call::Create(UI_TOOL_CHANGE_EVENT.into()),
        Call::Start(UI_TOOL_CHANGE_EVENT.into()),
        Call::Release(UI_TOOL_CHANGE_EVENT.into()),
        Call::Start(SIGNIFIER_EVENTS[1].into()),
        Call::Start(SIGNIFIER_EVENTS[2].into()),
    ]);
    assert_eq!(*calls.lock().unwrap(), expected);
    assert_eq!(
        world
            .resource::<SignifierAudioState<Handle>>()
            .last_changed_wall,
        Some(WallId(99))
    );
    calls.lock().unwrap().clear();
    schedule.run(&mut world);
    assert!(calls.lock().unwrap().is_empty());
}
#[test]
fn tool_parameters_cover_complete_original_mode_and_decorator_dispatch() {
    use crate::systems::mode_manager::{DecoratorType as D, EraserLayer, WallShape};
    let walls = PublicWalls::default();
    for (mode, expected) in [
        (AppMode::Editing(EditSubject::None), Some(0.)),
        (AppMode::Editing(EditSubject::Wall(999)), Some(15.)),
        (AppMode::Editing(EditSubject::Terrain(3)), Some(3.)),
        (AppMode::Wall(WallShape::Freeform), Some(15.)),
        (AppMode::Wall(WallShape::Circle), Some(0.)),
        (AppMode::Path, Some(1.)),
        (AppMode::Cobblestone(3), Some(1.)),
        (AppMode::Eraser(EraserLayer::All), Some(2.)),
        (AppMode::Terrain, Some(3.)),
        (AppMode::Color, Some(6.)),
        (AppMode::Replay, None),
        (AppMode::LoadGame, Some(4.)),
        (AppMode::Water, Some(7.)),
        (AppMode::Tree, Some(13.)),
        (AppMode::Garden, Some(8.)),
        (AppMode::Fences, Some(14.)),
        (AppMode::Photomode, Some(16.)),
        (AppMode::CloneClutter, None),
    ] {
        assert_eq!(tool_parameter(mode, &walls), expected, "{mode:?}");
    }
    for (deco, value) in [
        (D::CottageWindow, 5.),
        (D::GothicWindow, 5.),
        (D::ArrowSlit, 5.),
        (D::Chimney, 9.),
        (D::Lantern, 11.),
        (D::Flag, 10.),
        (D::Stairs, 12.),
    ] {
        assert_eq!(
            tool_parameter(AppMode::Decorator(deco), &walls),
            Some(value)
        );
    }
}

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
                world.send_event(FinishAdjWallHeightMode::default());
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
                    .consume_with_paths(&mut inner, &terrain, None, &crate::resources::walls::public_walls::WallPublicationDirty::default(), &crate::resources::walls::public_walls::WallSpatialHash::default());
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
