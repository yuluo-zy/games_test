//! TB-E (round 11): `deserialize_optional` / `value_to_param` boundaries.
//!
//! Contract `tb-toolbar-CONTRACT.md` §2.2 (#3) and §2.11 (#54):
//! - `deserialize_optional` is the transparent `Option<T>` field seed used by
//!   `LabelConfig` (all six optional fields) and read through
//!   `read_node_config_from_file`/template instantiation.
//! - `LogSlider::value_to_param` is `(log2(value)-base).clamp(min, max)` and the inverse
//!   of `param_to_value` (`exp2(clamp(param) + log2_base)`).
use serde::Deserialize;
use toolbar::mei::config::{LabelConfig, TextAlign, deserialize_optional};
use toolbar::resouces_and_components::LogSlider;

#[derive(Deserialize)]
struct OptionalField {
    #[serde(default, deserialize_with = "deserialize_optional")]
    value: Option<u32>,
}

#[test]
fn deserialize_optional_accepts_some_none_and_rejects_a_wrong_type() {
    let some: OptionalField = ron::from_str("(value: Some(7))").expect("Some payload");
    assert_eq!(some.value, Some(7));
    let none: OptionalField = ron::from_str("(value: None)").expect("None payload");
    assert_eq!(none.value, None);
    // The seed is transparent: it forwards `Option<T>` deserialization, so the
    // wrong payload type is still rejected by serde.
    assert!(ron::from_str::<OptionalField>("(value: \"text\")").is_err());
}

#[test]
fn label_config_optional_fields_round_trip_through_the_real_ron_schema() {
    // `read_node_config_from_file` uses IMPLICIT_SOME, so bare values become Some.
    let options = ron::Options::default()
        .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME);

    let config: LabelConfig = options
        .from_str("(text:\"Title\",color:0x12345678,size:20.0,max_width:120.0)")
        .expect("implicit Option payloads");
    assert_eq!(config.text, "Title");
    assert_eq!(config.color, Some(0x1234_5678));
    assert_eq!(config.size, Some(20.0));
    assert_eq!(config.max_width, Some(120.0));
    assert!(config.align.is_none());
    assert!(config.weight.is_none());
    assert!(config.font.is_none());

    // Absent optional fields stay None through #[serde(default)].
    let plain: LabelConfig = options.from_str("(text:\"Plain\")").expect("minimal label");
    assert!(plain.color.is_none());
    assert!(plain.align.is_none());
    assert!(plain.size.is_none());
    assert!(plain.max_width.is_none());
    assert!(plain.weight.is_none());
    assert!(plain.font.is_none());

    // Explicit enum values are also accepted through deserialize_optional.
    let aligned: LabelConfig = options
        .from_str("(text:\"Aligned\",align:Center)")
        .expect("explicit align");
    assert!(matches!(aligned.align, Some(TextAlign::Center)));

    // deny_unknown_fields stays in force for the template schema.
    assert!(options.from_str::<LabelConfig>("(text:\"X\",bogus:1)").is_err());
}

#[test]
fn value_to_param_is_log2_clamped_to_the_slider_range() {
    let slider = LogSlider {
        min: -1.0,
        max: 2.0,
        _pad: 0.0,
        log2_base: 0.0,
    };
    assert_eq!(slider.value_to_param(1.0), 0.0);
    assert_eq!(slider.value_to_param(4.0), 2.0);
    // log2(0.25) = -2 -> clamped to min.
    assert_eq!(slider.value_to_param(0.25), -1.0);
    // log2(0) = -inf -> min; log2(f32::MAX) = 128 -> max.
    assert_eq!(slider.value_to_param(0.0), -1.0);
    assert_eq!(slider.value_to_param(f32::MAX), 2.0);
    assert_eq!(slider.value_to_param(f32::MIN_POSITIVE), -1.0);
    // Negative inputs produce NaN, which f32::clamp propagates unchanged.
    assert!(slider.value_to_param(-1.0).is_nan());
    // With base 0 the two mappings are exact inverses inside the range.
    for param in [-1.0, 0.0, 1.5, 2.0] {
        assert!((slider.value_to_param(slider.param_to_value(param)) - param).abs() < 1e-5);
    }
    // A nonzero base must be removed by the inverse. The native focus slider
    // uses base -3; omitting this step shifts both display and reset position.
    let shifted = LogSlider {
        min: -4.0,
        max: 4.0,
        _pad: 0.0,
        log2_base: 3.0,
    };
    for param in [-1.0, 0.0, 0.5] {
        let value = shifted.param_to_value(param);
        assert!((shifted.value_to_param(value) - param).abs() < 1e-5);
    }
    assert_eq!(shifted.param_to_value(2.0), 32.0);
    let focus = LogSlider { min: -2.0, max: 2.0, _pad: 0.5, log2_base: -3.0 };
    for (value, parameter) in [(0.03125, -2.0), (0.0625, -1.0), (0.125, 0.0), (0.25, 1.0), (0.5, 2.0)] {
        assert_eq!(focus.value_to_param(value), parameter);
        assert_eq!(focus.param_to_value(parameter), value);
    }
}

#[test]
#[should_panic]
fn value_to_param_panics_when_the_bounds_are_inverted() {
    let slider = LogSlider {
        min: 2.0,
        max: -2.0,
        _pad: 0.0,
        log2_base: 0.0,
    };
    // f32::clamp panics on min > max (native `do_panic::runtime`).
    let _ = slider.value_to_param(4.0);
}
