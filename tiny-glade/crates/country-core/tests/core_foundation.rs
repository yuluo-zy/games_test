use std::any::TypeId;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use bevy_ecs::{
    event::{Event, Events},
    world::World,
};
use country_core::{
    resources::{
        color_icon::ColorMenuBrightnessValue,
        cursor_terrain_raycast::CursorTerrainRaycast,
        editing_highlight::EditingHighlight,
        gui::{DebugGui, EscMenuStateView},
        ui_settings::{ToolbarAlignment, UiSettings},
        walls::EraserSizeMultiplier,
        window::WindowSize,
    },
    systems::render_loop::draw_passes::{
        common_draw_params::BrickEraseInputs, draw_callbacks_a, draw_callbacks_b,
    },
    utils::{
        self, components::transform::Transform, dirty_tiles::DirtyTiles,
        project_cursor_onto_ray::ProportionalScreenSpace,
    },
};
use glam::{Vec2, Vec3};
use serde_json::Value;

fn oracle() -> Value {
    serde_json::from_str(include_str!("fixtures/core-foundation-b.json")).unwrap()
}

fn word(value: &Value) -> u32 {
    value.as_u64().unwrap() as u32
}

#[test]
fn window_state_and_screen_parameters_match_original_entries() {
    for case in oracle()["windows"].as_array().unwrap() {
        let width = word(&case["input"][0]);
        let height = word(&case["input"][1]);
        let mut window = WindowSize::new(width, height);
        assert_eq!(
            window.as_dims(),
            [word(&case["constructed"][0]), word(&case["constructed"][1])]
        );
        window.set(height, width);
        window.set(width, height);
        assert_eq!(
            window.as_dims(),
            [word(&case["fields"][0]), word(&case["fields"][1])]
        );
        assert_eq!(
            window.as_vec2().to_array().map(f32::to_bits),
            [word(&case["vec2_bits"][0]), word(&case["vec2_bits"][1])]
        );
        assert_eq!(
            window.aspect_ratio().to_bits(),
            word(&case["ratio_bits"]),
            "{case}"
        );
        let space = ProportionalScreenSpace::from_window_size(window.as_dims());
        assert_eq!(
            [space.size.x, space.size.y, space.aspect, space.unit].map(f32::to_bits),
            std::array::from_fn(|i| word(&case["screen_space_bits"][i]))
        );
    }
}

#[test]
fn settings_default_and_alignment_match_original_bytes() {
    let fixture = oracle();
    let native: Vec<u8> = fixture["ui_default"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| word(v) as u8)
        .collect();
    let settings = UiSettings::default();
    assert_eq!(
        settings.icon_size_mult.to_bits(),
        u32::from_le_bytes(native[0..4].try_into().unwrap())
    );
    assert_eq!(
        settings.custom_cursor_size.to_bits(),
        u32::from_le_bytes(native[4..8].try_into().unwrap())
    );
    assert_eq!(
        [
            settings.toolbar_alignment as u8,
            settings.auto_hide_toolbar as u8,
            settings.input_lag_reduction as u8,
            settings.game_logo as u8,
            settings.custom_cursor as u8
        ],
        native[8..13]
    );
    for (i, alignment) in [
        ToolbarAlignment::Bottom,
        ToolbarAlignment::Left,
        ToolbarAlignment::Right,
        ToolbarAlignment::Top,
    ]
    .into_iter()
    .enumerate()
    {
        assert_eq!(
            alignment.is_horizontal(),
            fixture["horizontal"][i].as_bool().unwrap()
        );
    }
    assert_eq!(
        serde_json::to_string(&settings).unwrap(),
        r#"{"icon_size_mult":1.0,"toolbar_alignment":"Bottom","auto_hide_toolbar":false,"input_lag_reduction":true,"game_logo":true,"custom_cursor":false,"custom_cursor_size":24.0}"#
    );
}

#[test]
fn settings_deserialization_matches_all_native_field_presence_combinations() {
    for case in oracle()["ui_deserialize"].as_array().unwrap() {
        let source = case["json"].as_str().unwrap();
        let settings: UiSettings = serde_json::from_str(source).unwrap();
        let mut state = Vec::new();
        state.extend(settings.icon_size_mult.to_le_bytes());
        state.extend(settings.custom_cursor_size.to_le_bytes());
        state.extend([
            settings.toolbar_alignment as u8,
            settings.auto_hide_toolbar as u8,
            settings.input_lag_reduction as u8,
            settings.game_logo as u8,
            settings.custom_cursor as u8,
        ]);
        let native: Vec<u8> = case["state"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| word(v) as u8)
            .collect();
        assert_eq!(state, native, "{source}");
    }
    // Native field dispatch has duplicate-field errors and an ignored-field arm.
    assert!(serde_json::from_str::<UiSettings>(r#"{"game_logo":true,"game_logo":false}"#).is_err());
    assert!(serde_json::from_str::<UiSettings>(r#"{"toolbar_alignment":"Horizontal"}"#).is_err());
    assert_eq!(
        serde_json::from_str::<UiSettings>(r#"{"future_setting":12}"#).unwrap(),
        UiSettings::default()
    );
}

#[test]
fn menu_prefix_predicates_match_original_without_claiming_payload_schema() {
    for case in oracle()["menus"].as_array().unwrap() {
        let menu = EscMenuStateView {
            tag: word(&case["tag"]),
            payload: word(&case["payload"]),
            flag: u8::from(case["flag"].as_bool().unwrap()),
        };
        for (name, actual) in [
            ("is_open", menu.is_open()),
            ("is_closed", menu.is_closed()),
            ("cast_ui_shadows", menu.cast_ui_shadows()),
            ("should_show_world", menu.should_show_world()),
            ("can_limit_frame_rate", menu.can_limit_frame_rate()),
        ] {
            assert_eq!(actual, case[name].as_bool().unwrap(), "{name}: {case}");
        }
        assert_eq!(menu.allow_painterly_overlay(), menu.can_limit_frame_rate());
        assert_eq!(
            menu.can_use_static_world_capture(),
            menu.can_limit_frame_rate()
        );
    }
}

#[test]
fn brightness_selector_matches_native_table() {
    let fixture = oracle();
    for tag in 0..3 {
        assert_eq!(
            ColorMenuBrightnessValue::from_tag(tag)
                .unwrap()
                .to_f32()
                .to_bits(),
            word(&fixture["brightness"][tag as usize])
        );
    }
    assert_eq!(
        ColorMenuBrightnessValue::default_f32().to_bits(),
        word(&fixture["brightness_default"])
    );
    assert!(ColorMenuBrightnessValue::from_tag(3).is_none());
}

#[test]
fn both_draw_consumers_read_the_same_highlight_resource_after_clear() {
    assert_eq!(
        TypeId::of::<draw_callbacks_a::EditingHighlightView>(),
        TypeId::of::<EditingHighlight>()
    );
    assert_eq!(
        TypeId::of::<draw_callbacks_b::EditingHighlightView>(),
        TypeId::of::<EditingHighlight>()
    );
    assert_eq!(
        TypeId::of::<draw_callbacks_a::DebugGuiView>(),
        TypeId::of::<draw_callbacks_b::DebugGuiView>()
    );
    let mut world = World::new();
    world.insert_resource(EditingHighlight {
        color: Vec3::new(2.0, 3.0, 4.0),
        enabled: true,
    });
    world.insert_resource(DebugGui::default());
    world.insert_resource(EraserSizeMultiplier(2.5));
    world.insert_resource(CursorTerrainRaycast {
        tag: 1,
        pos: Vec3::new(5.0, 6.0, 7.0),
        delta: Vec3::ZERO,
        speed: 0.0,
    });
    let before = BrickEraseInputs::from_world(&world).to_a4();
    assert!(before.enabled);
    assert_eq!(before.radius, 2.5);
    assert_eq!(before.cursor_position, Some(Vec3::new(5.0, 6.0, 7.0)));
    utils::get_res_mut::<EditingHighlight>(&mut world).clear();
    let highlight = utils::get_res::<EditingHighlight>(&world);
    let native = oracle();
    for case in native["clear"].as_array().unwrap() {
        assert_eq!(highlight.color.to_array().map(f32::to_bits), [0; 3]);
        assert!(!highlight.enabled);
        assert!(
            case["after"].as_array().unwrap()[..13]
                .iter()
                .all(|v| word(v) == 0)
        );
        assert_eq!(case["tick"], case["tick_after"]);
        assert_eq!(
            &case["after"].as_array().unwrap()[13..],
            &case["before"].as_array().unwrap()[13..]
        );
    }
    assert!(!BrickEraseInputs::from_world(&world).to_a4().enabled);
    world.resource_mut::<DebugGui>().suppress = true;
    world.resource_mut::<CursorTerrainRaycast>().tag = 0;
    let after = BrickEraseInputs::from_world(&world).to_a4();
    assert!(after.suppress);
    assert_eq!(after.cursor_position, None);
}

#[derive(Event, Debug, PartialEq)]
struct Number(u32);

#[test]
fn event_consumption_drains_both_epochs_and_keeps_dependency_counters() {
    let mut world = World::new();
    let mut events = Events::<Number>::default();
    events.send(Number(1));
    events.send(Number(2));
    events.update();
    events.send(Number(3));
    world.insert_resource(events);
    assert_eq!(
        utils::drain_events::<Number>(&mut world),
        vec![Number(1), Number(2), Number(3)]
    );
    assert!(utils::get_last_event::<Number>(&mut world).is_none());
    assert_eq!(world.resource_mut::<Events<Number>>().send(Number(4)).id, 3);
    world.resource_mut::<Events<Number>>().update();
    world.resource_mut::<Events<Number>>().send(Number(5));
    assert_eq!(utils::get_last_event::<Number>(&mut world), Some(Number(5)));
    assert!(world.resource::<Events<Number>>().is_empty());
}

#[derive(Event)]
struct DropEvent(Arc<AtomicUsize>);
impl Drop for DropEvent {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn selecting_last_event_drops_unselected_payloads() {
    let drops = Arc::new(AtomicUsize::new(0));
    let mut events = Events::default();
    events.send(DropEvent(drops.clone()));
    events.update();
    events.send(DropEvent(drops.clone()));
    events.send(DropEvent(drops.clone()));
    let mut world = World::new();
    world.insert_resource(events);
    let last = utils::get_last_event::<DropEvent>(&mut world).unwrap();
    assert_eq!(drops.load(Ordering::SeqCst), 2);
    drop(last);
    assert_eq!(drops.load(Ordering::SeqCst), 3);
}

#[test]
#[should_panic]
fn missing_event_resource_does_not_silently_become_an_empty_stream() {
    utils::get_last_event::<Number>(&mut World::new());
}

#[test]
fn core_transform_reuses_shared_orientation_and_comparison_policy() {
    let mut transform =
        Transform::from_translation_scale(Vec3::new(2.0, 3.0, 4.0), Vec3::splat(5.0));
    transform.look_at(Vec3::new(2.0, 3.0, 0.0), Vec3::Y);
    assert_eq!(transform.translation, Vec3::new(2.0, 3.0, 4.0));
    assert_eq!(transform.scale, Vec3::splat(5.0));
    assert!((transform.forward() - Vec3::NEG_Z).length() < 1.0e-6);
    assert!(transform.abs_diff_eq(&transform, 0.0));
    assert!(!transform.abs_diff_eq(&transform, -1.0));
    assert!(!transform.abs_diff_eq(&transform, f32::NAN));
    let mut invalid = transform;
    invalid.translation.x = f32::NAN;
    assert!(!invalid.abs_diff_eq(&transform, f32::INFINITY));
}

#[test]
fn dirty_cell_mapping_matches_original_truncation_nan_and_bounds_policy() {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/dirty-tile-cell-b.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let world = Vec2::new(
            f32::from_bits(word(&case["input_bits"][0])),
            f32::from_bits(word(&case["input_bits"][1])),
        );
        let expected = case["cell"]
            .as_array()
            .map(|cell| [word(&cell[0]), word(&cell[1])]);
        assert_eq!(DirtyTiles::cell_from_world_space(world), expected, "{case}");
    }
    let mut tiles = DirtyTiles::new_clear();
    let [x, y] = DirtyTiles::cell_from_world_space(Vec2::new(-66.0, 0.0)).unwrap();
    tiles.set_cell(x as i32, y as i32);
    assert!(tiles.contains(0, 32));
    assert_eq!(tiles.tile_indices().collect::<Vec<_>>(), vec![32 * 64]);
}
