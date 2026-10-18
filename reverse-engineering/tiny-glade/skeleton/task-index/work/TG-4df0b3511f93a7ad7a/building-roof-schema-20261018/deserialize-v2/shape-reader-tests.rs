#[cfg(test)]
mod original_shape_reader_compatibility_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn legacy_circle_origin_is_converted_without_a_second_shape_owner() {
        let shape: ShapeParameters = serde_json::from_value(json!({
            "Circle": {"origin": [3.0, -5.0], "radius": -2.0, "rotation": "old ignored field"}
        })).unwrap();
        let circle = shape.as_circle();
        assert_eq!(circle.center, Vec2::new(3.0, -5.0));
        assert_eq!(circle.radius, -2.0);
        assert_eq!(circle.rotation.to_bits(), 0);
        let saved = serde_json::to_value(shape).unwrap();
        assert_eq!(saved["Circle"]["center"], json!([3.0, -5.0]));
        assert!(saved["Circle"].get("origin").is_none());
    }

    #[test]
    fn new_shape_is_tried_first_and_circular_value_is_not_normalized() {
        let shape: ShapeParameters = serde_json::from_value(json!({
            "Circle": {"center": [1.0, 2.0], "origin": [8.0, 9.0],
                       "radius": 3.0, "rotation": {"val": -7.0}}
        })).unwrap();
        let circle = shape.as_circle();
        assert_eq!(circle.center, Vec2::new(1.0, 2.0));
        assert_eq!(circle.rotation.to_bits(), (-7.0_f32).to_bits());
        // The old reader is the fallback after *any* new representation error.
        let legacy: ShapeParameters = serde_json::from_value(json!({
            "Circle": {"center": [1.0, 2.0], "origin": [8.0, 9.0],
                       "radius": 3.0, "rotation": "invalid for new"}
        })).unwrap();
        assert_eq!(legacy.as_circle().center, Vec2::new(8.0, 9.0));
        assert_eq!(legacy.as_circle().rotation.to_bits(), 0);
    }

    #[test]
    fn rectangle_reader_copies_fields_without_geometric_constructor_guards() {
        let shape: ShapeParameters = serde_json::from_value(json!({
            "Rectangle": {"orientation": [2.0, 3.0], "width": -4.0,
                          "length": 5.0, "center": [6.0, 7.0]}
        })).unwrap();
        let rectangle = shape.as_rectangle();
        assert_eq!(rectangle.axis, Vec2::new(2.0, 3.0));
        assert_eq!(rectangle.size, Vec2::new(-4.0, 5.0));
        assert_eq!(rectangle.center, Vec2::new(6.0, 7.0));
        let sequence: ShapeParameters = serde_json::from_value(json!({
            "Rectangle": [[2.0, 3.0], -4.0, 5.0, [6.0, 7.0]]
        })).unwrap();
        assert_eq!(sequence.bytes(), shape.bytes());
        for field in ["orientation", "width", "length", "center"] {
            let mut value = json!({"Rectangle": {"orientation": [1.0, 0.0],
                "width": 4.0, "length": 5.0, "center": [6.0, 7.0]}});
            value["Rectangle"].as_object_mut().unwrap().remove(field);
            assert!(serde_json::from_value::<ShapeParameters>(value).is_err(), "{field}");
        }
    }

    #[test]
    #[should_panic(expected = "legacy ShapeParameters must be Circle")]
    fn legacy_unit_rectangle_keeps_the_original_conversion_failure() {
        let _ = serde_json::from_value::<ShapeParameters>(json!("Rectangle"));
    }
}
