//! Frozen original-B output fixtures. These tests deliberately initialize the
//! native state-machine boundary. The separate picker/interaction/history
//! tests begin at action phases; neither group proves device-input translation.
use super::*;
use crate::input::action_set::{DigitalActionPhase, DigitalActionState};
use glam::Vec2;
use serde_json::Value;

fn number(v: &Value, key: &str, fallback: u64) -> u64 {
    v.get(key).and_then(Value::as_u64).unwrap_or(fallback)
}
fn bytes(hex: &str) -> Vec<u8> {
    hex.as_bytes()
        .chunks_exact(2)
        .map(|c| u8::from_str_radix(std::str::from_utf8(c).unwrap(), 16).unwrap())
        .collect()
}
fn hit(widget: u64, flags: u8) -> [u8; 56] {
    let mut b = [0; 56];
    b[..4].copy_from_slice(&1u32.to_le_bytes());
    for (n, value) in [3.0f32, 4.0, 5.0].into_iter().enumerate() {
        b[4 + 4 * n..8 + 4 * n].copy_from_slice(&value.to_le_bytes());
    }
    b[16..24].copy_from_slice(&widget.to_le_bytes());
    b[24..32].copy_from_slice(&2u64.to_le_bytes());
    b[49] = flags;
    b
}
fn params(inputs: &Value, index: u32) -> WidgetParams {
    let geometry = match number(inputs, "geometry_tag", 7) {
        0 => WidgetGeometry::Sphere {
            center: Vec3::new(10., 20., 30.),
            radius: 1.,
        },
        1 => WidgetGeometry::SphereCloud {
            centers: vec![
                Vec3::ZERO,
                Vec3::new(10., 20., 30.),
                Vec3::new(40., 50., 60.),
            ],
            radius: 1.,
        },
        2 => WidgetGeometry::TriangleMesh(Vec::new()),
        3 => WidgetGeometry::ScreenAabb {
            min: Vec2::ZERO,
            max: Vec2::ONE,
            distance: 0.,
        },
        4 => WidgetGeometry::WorldCanvas {
            extent: Vec2::ONE,
            layer: 0,
        },
        5 => WidgetGeometry::TerrainCanvas {
            distance: 0.,
            layer: 0,
        },
        6 => WidgetGeometry::PlaneDisc {
            point: Vec3::new(10., 20., 30.),
            normal: Vec3::Z,
            radius: 2.,
        },
        7 => WidgetGeometry::Empty,
        _ => unreachable!(),
    };
    let mut p = WidgetParams::default().geometry(geometry);
    for (slot, sound) in p.sounds.slots.iter_mut().enumerate() {
        for (n, value) in [100 * index + slot as u32 + 1, 55, 66, 77, 88, 99]
            .into_iter()
            .enumerate()
        {
            sound[4 * n..4 * n + 4].copy_from_slice(&value.to_le_bytes());
        }
    }
    let mut preview = [0; 16];
    preview[0] = number(inputs, "preview_tag", 0) as u8;
    preview[1] = 7;
    for (n, value) in [1.0f32, 2., 3.].into_iter().enumerate() {
        preview[4 + 4 * n..8 + 4 * n].copy_from_slice(&value.to_le_bytes());
    }
    p.extra_78 = [
        u64::from_le_bytes(preview[..8].try_into().unwrap()),
        u64::from_le_bytes(preview[8..].try_into().unwrap()),
    ];
    p
}
fn check_cases(cases: Vec<Value>) {
    for (case_index, case) in cases.into_iter().enumerate() {
        let x = &case["inputs"];
        let expected = &case["result"];
        let mut world = World::new();
        let mut state = UiState::at_startup();
        state.add_with_id(WidgetId(11), params(x, 0));
        state.add_with_id(WidgetId(22), params(x, 1));
        let old = state.widget(WidgetId(11)).unwrap();
        let other = state.widget(WidgetId(22)).unwrap();
        let replacement = if number(x, "registry", 0) != 0 {
            state.add_with_id(WidgetId(11), params(x, 2));
            Some(state.widget(WidgetId(11)).unwrap())
        } else {
            None
        };
        let tag = number(x, "tag", 0) as u8;
        let phase = number(x, "phase", 2) as u8;
        let flags = number(x, "flags", 0) as u8;
        state.replace_interaction_full(
            UiInteraction {
                native_tag: tag,
                native_phase: phase,
                widget: WidgetId(11),
            },
            if tag == 0 { None } else { Some(old.clone()) },
            hit(11, flags),
        );
        let chosen = if x.get("hit").is_none() {
            Some(11)
        } else {
            x["hit"].as_u64()
        };
        state.set_hover_intersections(chosen.map(|id| vec![hit(id, flags)]).unwrap_or_default());
        world.insert_resource(state);
        let mut actions = ActionSetStates::default();
        actions.digital[0] = DigitalActionState {
            phase: match number(x, "action", 4) {
                0 => DigitalActionPhase::JustPressed,
                1 => DigitalActionPhase::Held,
                2 => DigitalActionPhase::Repeating,
                3 => DigitalActionPhase::JustReleased,
                4 => DigitalActionPhase::Idle,
                _ => unreachable!(),
            },
            timer_seconds: f32::from_bits(number(x, "timer_bits", 0) as u32),
        };
        world.insert_resource(actions);
        world.init_resource::<Events<WindowFocusEvent>>();
        world.init_resource::<Events<EscapeMenuJustOpened>>();
        world.init_resource::<Events<ShutdownGameCmd>>();
        world.init_resource::<Events<OverwriteBrushPreview>>();
        world.init_resource::<Events<OsCursorVisibilityCmd>>();
        world.init_resource::<Events<PlayUiSound>>();
        for _ in 0..number(x, "escape", 0) {
            world.send_event(EscapeMenuJustOpened);
        }
        for _ in 0..number(x, "shutdown", 0) {
            world.send_event(ShutdownGameCmd);
        }
        if let Some(focus) = x.get("focus").and_then(Value::as_array) {
            for value in focus {
                world.send_event(WindowFocusEvent(value.as_u64().unwrap() as u8));
            }
        }
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_update);
        schedule.run(&mut world);
        let state = world.resource::<UiState>();
        assert_eq!(
            state.hover_byte() as u64,
            expected["tag"].as_u64().unwrap(),
            "case {case_index}: {x}"
        );
        if state.hover_byte() != 0 {
            assert_eq!(
                state.click_byte() as u64,
                expected["phase"].as_u64().unwrap(),
                "case {case_index}: {x}"
            );
            assert_eq!(
                state.active_widget().0,
                expected["widget"].as_u64().unwrap(),
                "case {case_index}: {x}"
            );
            let expected_arc = match expected["arc"].as_u64().unwrap() {
                0x4000 => &old,
                0x4200 => &other,
                0x4400 => replacement.as_ref().unwrap(),
                _ => unreachable!(),
            };
            assert!(
                Arc::ptr_eq(&state.active_params().unwrap(), expected_arc),
                "captured owner case {case_index}: {x}"
            );
        } else {
            assert!(state.active_params().is_none());
        }
        assert!(state.registry_snapshot().is_empty());
        let sounds: Vec<_> = world
            .resource_mut::<Events<PlayUiSound>>()
            .drain()
            .collect();
        let expected_sounds = expected["sounds"].as_array().unwrap();
        assert_eq!(
            sounds.len(),
            expected_sounds.len(),
            "sound count case {case_index}: {x}"
        );
        for (sound, raw) in sounds.iter().zip(expected_sounds) {
            assert_eq!(
                sound.native_slot().as_slice(),
                &bytes(raw.as_str().unwrap())[..24],
                "sound case {case_index}: {x}"
            );
        }
        let brush: Vec<_> = world
            .resource_mut::<Events<OverwriteBrushPreview>>()
            .drain()
            .collect();
        assert_eq!(brush.len(), 1);
        let b = bytes(expected["brush"][0].as_str().unwrap());
        assert_eq!(brush[0].flag, b[8]);
        assert_eq!(brush[0].kind, b[24]);
        // Inactive position bytes are uninitialized in the original enum.
        if brush[0].flag != 0 || number(x, "preview_tag", 0) == 2 {
            let expected_position = Vec3::new(
                f32::from_le_bytes(b[12..16].try_into().unwrap()),
                f32::from_le_bytes(b[16..20].try_into().unwrap()),
                f32::from_le_bytes(b[20..24].try_into().unwrap()),
            );
            assert_eq!(
                brush[0].position, expected_position,
                "preview case {case_index}: {x}"
            );
        }
        let cursor: Vec<_> = world
            .resource_mut::<Events<OsCursorVisibilityCmd>>()
            .drain()
            .collect();
        let c = bytes(expected["cursor"][0].as_str().unwrap());
        let origin = OsCursorVisibilityCmdOrigin(u32::from_le_bytes(c[4..8].try_into().unwrap()));
        assert_eq!(
            cursor,
            vec![if c[0] == 2 {
                OsCursorVisibilityCmd::Suppress(origin)
            } else {
                OsCursorVisibilityCmd::Unsuppress(origin)
            }]
        );
    }
}

#[test]
fn original_600_interaction_transitions_and_lossless_effects() {
    let cases: Vec<Value> =
        serde_json::from_str(include_str!("test_data/ui-update-state.json")).unwrap();
    assert_eq!(cases.len(), 600);
    check_cases(cases);
}

#[test]
fn original_150_preview_cursor_and_captured_owner_cases() {
    let cases: Vec<Value> =
        serde_json::from_str(include_str!("test_data/ui-update-preview.json")).unwrap();
    assert_eq!(cases.len(), 150);
    check_cases(cases);
}
