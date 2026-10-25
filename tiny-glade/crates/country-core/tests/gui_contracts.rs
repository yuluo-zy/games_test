use country_core::resources::gui::{
    DebugTriggerProxyVis, EscMenuStateView, LoadMenuThumbnailMetrics, ShowDebugRaycastGeo,
};
use glam::Vec2;
use serde_json::Value;

fn oracle() -> Value {
    serde_json::from_str(include_str!("fixtures/gui-contracts-b.json")).unwrap()
}

fn word(value: &Value) -> u32 {
    value.as_u64().unwrap().try_into().unwrap()
}

fn vec2_bits(value: &Value) -> Vec2 {
    Vec2::new(
        f32::from_bits(word(&value[0])),
        f32::from_bits(word(&value[1])),
    )
}

#[test]
fn settings_payload_and_time_policy_match_original_time_update() {
    for case in oracle()["menus"].as_array().unwrap() {
        let menu = EscMenuStateView {
            tag: word(&case["tag"]),
            payload: word(&case["payload"]),
            flag: word(&case["flag"]) as u8,
        };
        for (name, actual) in [
            ("is_open", menu.is_open()),
            ("is_closed", menu.is_closed()),
            ("cast_ui_shadows", menu.cast_ui_shadows()),
            ("should_show_world", menu.should_show_world()),
            ("can_limit_frame_rate", menu.can_limit_frame_rate()),
            ("allow_time_passage", menu.allow_time_passage()),
        ] {
            assert_eq!(actual, case[name].as_bool().unwrap(), "{name}: {case}");
        }
        assert_eq!(
            menu.allow_time_passage(),
            word(&case["time_delta_bits"]) != 0,
            "{case}"
        );
    }
    let settings = EscMenuStateView {
        tag: 7,
        payload: 0,
        flag: 1,
    };
    assert!(settings.allow_time_passage());
    assert!(!settings.can_limit_frame_rate());
    assert!(
        !EscMenuStateView {
            flag: 3,
            ..settings
        }
        .allow_time_passage()
    );
    assert!(
        !EscMenuStateView {
            flag: 3,
            ..settings
        }
        .cast_ui_shadows()
    );
}

#[test]
fn thumbnail_positions_match_original_allocating_iterator() {
    for case in oracle()["metrics"].as_array().unwrap() {
        let metrics = LoadMenuThumbnailMetrics {
            origin: vec2_bits(&case["origin_bits"]),
            step: vec2_bits(&case["step_bits"]),
            cell_count: word(&case["count"]),
            cells_per_line: word(&case["stride"]),
            column_major: case["column_major"].as_bool().unwrap(),
            cell_size: Vec2::new(0.25, 0.5),
            scroll_offset: Vec2::new(-7.0, 19.0),
            highlighted_index: Some(3),
        };
        let result = std::panic::catch_unwind(|| metrics.positions());
        if let Some(panic) = case["panic"].as_str() {
            let error = result.expect_err("native panics before returning the iterator");
            let message = error
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| error.downcast_ref::<&str>().copied())
                .unwrap();
            assert!(message.contains(if panic == "divide_by_zero" {
                "divide"
            } else {
                "remainder"
            }));
            continue;
        }
        let positions = result.unwrap();
        assert_eq!(positions.len(), metrics.cell_count as usize);
        for (pos, expected) in positions.zip(case["positions_bits"].as_array().unwrap()) {
            assert_eq!(
                [pos.x.to_bits(), pos.y.to_bits()],
                [word(&expected[0]), word(&expected[1])],
                "{case}"
            );
        }
    }
}

#[test]
fn thumbnail_default_matches_native_without_comparing_uninitialized_option_payload() {
    let fixture = oracle();
    let bytes: Vec<u8> = fixture["default_metrics_bytes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| word(v) as u8)
        .collect();
    assert_eq!(&bytes[..4], &[0; 4]);
    assert_eq!(&bytes[8..49], &[0; 41]);
    let metrics = LoadMenuThumbnailMetrics::default();
    assert_eq!(metrics.highlighted_index, None);
    assert_eq!(metrics.origin, Vec2::ZERO);
    assert_eq!(metrics.cell_size, Vec2::ZERO);
    assert_eq!(metrics.step, Vec2::ZERO);
    assert_eq!(metrics.scroll_offset, Vec2::ZERO);
    assert_eq!(
        (
            metrics.cell_count,
            metrics.cells_per_line,
            metrics.column_major
        ),
        (0, 0, false)
    );
    assert_eq!(metrics.positions().len(), 0);
    assert_eq!(std::mem::size_of::<LoadMenuThumbnailMetrics>(), 0x34);
    assert_eq!(std::mem::offset_of!(LoadMenuThumbnailMetrics, origin), 8);
    assert_eq!(std::mem::offset_of!(LoadMenuThumbnailMetrics, step), 0x18);
    assert_eq!(
        std::mem::offset_of!(LoadMenuThumbnailMetrics, cell_count),
        0x28
    );
    assert_eq!(
        std::mem::offset_of!(LoadMenuThumbnailMetrics, column_major),
        0x30
    );
}

#[test]
fn debug_labels_match_both_native_return_contracts() {
    use DebugTriggerProxyVis::{
        ClutterCollisions, MeshesAndNormals, OnlyMeshes, RoofsWithUvs, ShowAll,
    };
    use ShowDebugRaycastGeo::{DecoratorCollisions, DecoratorInteractions, No, Roofs, Walls};
    let proxies = [
        ShowAll,
        OnlyMeshes,
        RoofsWithUvs,
        MeshesAndNormals,
        ClutterCollisions,
    ];
    let raycasts = [No, Walls, Roofs, DecoratorCollisions, DecoratorInteractions];
    for case in oracle()["labels"].as_array().unwrap() {
        let i = word(&case["tag"]) as usize;
        assert_eq!(proxies[i] as usize, i);
        assert_eq!(raycasts[i] as usize, i);
        let owned: String = proxies[i].as_string();
        let borrowed: &'static str = raycasts[i].as_string();
        assert_eq!(owned, case["proxy"].as_str().unwrap());
        assert_eq!(borrowed, case["raycast"].as_str().unwrap());
    }
}
