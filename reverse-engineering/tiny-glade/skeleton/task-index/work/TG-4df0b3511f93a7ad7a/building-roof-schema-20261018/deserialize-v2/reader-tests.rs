#[cfg(test)]
mod original_roof_reader_tests {
    use super::*;
    use serde_json::{Value, json};

    fn required_params() -> Value {
        json!({"profile_01": -2.0, "height_01": 3.0, "tip_offset_01": [0.25, -0.5]})
    }

    fn required_roof() -> Value {
        json!({
            "id": u64::MAX,
            "params": required_params(),
            "wall_shape": {"Circle": {"center": [2.0, -4.0], "radius": 3.0}},
            "pivot_y": -0.0,
            "color_id": u32::MAX
        })
    }

    #[test]
    fn minimal_map_uses_original_defaults_without_clamping_fields() {
        let roof: Roof = serde_json::from_value(required_roof()).unwrap();
        assert_eq!(roof.wall_id().0, u64::MAX);
        assert_eq!(roof.floor_y().to_bits(), (-0.0_f32).to_bits());
        assert_eq!(roof.height_01(), 3.0);
        assert_eq!(roof.0[0x54], 0, "missing gable style is Stepped");
        assert_eq!(roof.0[0x40], 1, "missing flat style is MatchingStoneOnly");
        let params = RoofShapeParams::from_roof(&roof.0);
        assert_eq!(params.profile_01(), -2.0);
        assert_eq!(params.ridge_length_01().to_bits(), 0x3ecccccd);
        assert_eq!(at(params.observed(), 20).to_bits(), 0x3e4ccccd);
        assert_eq!(&params.observed()[24..26], &[0, 0]);
        let value = serde_json::to_value(&roof).unwrap();
        assert_eq!(value["color_id"], u32::MAX);
        assert_eq!(value["gable_style"], "Stepped");
        assert_eq!(value["flat_roof_style"], "MatchingStoneOnly");
        assert_eq!(value["params"]["ridge_dir"], "Width");
        assert_eq!(value["params"]["flat_rim_style"], "Rim");
    }

    #[test]
    fn old_param_aliases_and_unknown_fields_follow_the_original_field_visitor() {
        let params: RoofShapeParams = serde_json::from_value(json!({
            "profile": -1.25, "height": 2.5, "tip_offset": [-0.0, 12.0],
            "unknown": {"nested": [null, true, "ignored"]}
        })).unwrap();
        assert_eq!(params.profile_01(), -1.25);
        assert_eq!(params.height_01(), 2.5);
        assert_eq!(at(params.observed(), 0).to_bits(), (-0.0_f32).to_bits());
        assert_eq!(at(params.observed(), 4), 12.0);
        assert!(serde_json::from_str::<RoofShapeParams>(
            r#"{"profile":1,"profile_01":2,"height":3,"tip_offset":[0,0]}"#
        ).is_err());
        assert!(serde_json::from_str::<Roof>(
            r#"{"id":1,"id":2,"params":{"profile":1,"height":1,"tip_offset":[0,0]},"wall_shape":{"Circle":{"center":[0,0],"radius":1}},"pivot_y":0,"color_id":0}"#
        ).is_err());
        let mut roof = required_roof();
        roof["unused"] = json!([1, {"anything": true}]);
        assert!(serde_json::from_value::<Roof>(roof).is_ok());
    }

    #[test]
    fn missing_required_null_and_invalid_enum_values_are_rejected() {
        for field in ["id", "params", "wall_shape", "pivot_y", "color_id"] {
            let mut value = required_roof();
            value.as_object_mut().unwrap().remove(field);
            assert!(serde_json::from_value::<Roof>(value).is_err(), "{field}");
        }
        for field in ["profile_01", "height_01", "tip_offset_01"] {
            let mut value = required_params();
            value.as_object_mut().unwrap().remove(field);
            assert!(serde_json::from_value::<RoofShapeParams>(value).is_err(), "{field}");
        }
        for field in ["gable_style", "flat_roof_style"] {
            let mut value = required_roof();
            value[field] = Value::Null;
            assert!(serde_json::from_value::<Roof>(value).is_err(), "{field}");
        }
        for field in ["ridge_length_01", "ridge_dir", "eave_length_01", "flat_rim_style"] {
            let mut value = required_params();
            value[field] = Value::Null;
            assert!(serde_json::from_value::<RoofShapeParams>(value).is_err(), "{field}");
        }
        for (field, invalid) in [("gable_style", "Matching"), ("flat_roof_style", "Stone")] {
            let mut value = required_roof();
            value[field] = json!(invalid);
            assert!(serde_json::from_value::<Roof>(value).is_err());
        }
        for (field, invalid) in [("ridge_dir", "Diagonal"), ("flat_rim_style", "None")] {
            let mut value = required_params();
            value[field] = json!(invalid);
            assert!(serde_json::from_value::<RoofShapeParams>(value).is_err());
        }
        let mut value = required_roof();
        value["color_id"] = json!(u64::from(u32::MAX) + 1);
        assert!(serde_json::from_value::<Roof>(value).is_err());
        assert!(serde_json::from_str::<Roof>(
            r#"{"id":18446744073709551616,"params":{"profile":1,"height":1,"tip_offset":[0,0]},"wall_shape":{"Circle":{"center":[0,0],"radius":1}},"pivot_y":0,"color_id":0}"#
        ).is_err());
    }

    #[test]
    fn sequence_order_defaults_and_extra_element_guards_match_the_original() {
        let params = json!([0.25, 0.75, [1.5, -2.5]]);
        let shape = json!({"Circle": {"center": [0.0, 0.0], "radius": 1.0}});
        let roof: Roof = serde_json::from_value(json!([
            17, params.clone(), shape.clone(), 9.0, 23
        ])).unwrap();
        assert_eq!(roof.wall_id().0, 17);
        assert_eq!(roof.floor_y(), 9.0);
        assert_eq!((roof.0[0x54], roof.0[0x40]), (0, 1));
        let roof: Roof = serde_json::from_value(json!([
            17, params.clone(), shape.clone(), 9.0, 23, "Overhang", "Matching"
        ])).unwrap();
        assert_eq!((roof.0[0x54], roof.0[0x40]), (1, 0));
        assert!(serde_json::from_value::<Roof>(json!([
            17, params.clone(), shape.clone(), 9.0
        ])).is_err());
        assert!(serde_json::from_value::<Roof>(json!([
            17, params.clone(), shape, 9.0, 23, "Stepped", "Matching", 0
        ])).is_err());
        let decoded: RoofShapeParams = serde_json::from_value(json!([
            0.25, 0.75, [1.5, -2.5], 0.8, "Length", 0.9, "NoRim"
        ])).unwrap();
        assert_eq!(decoded.ridge_length_01(), 0.8);
        assert_eq!(at(decoded.observed(), 20), 0.9);
        assert_eq!(&decoded.observed()[24..26], &[1, 1]);
        assert!(serde_json::from_value::<RoofShapeParams>(json!([0.25, 0.75])).is_err());
        assert!(serde_json::from_value::<RoofShapeParams>(json!([
            0.25, 0.75, [1.5, -2.5], 0.8, "Length", 0.9, "NoRim", 0
        ])).is_err());
    }

    #[test]
    fn flat_override_requires_color_and_material_but_defaults_full_width_pattern() {
        for (material, bit) in [("Stone", 0), ("Wood", 1)] {
            let style: FlatRoofStyle = serde_json::from_value(json!({
                "Override": {"color_id": u32::MAX, "material": material}
            })).unwrap();
            assert_eq!(&style.observed()[..2], &[2, bit]);
            assert_eq!(&style.observed()[4..8], &u32::MAX.to_le_bytes());
            assert_eq!(&style.observed()[8..12], &[0; 4]);
            let style: FlatRoofStyle = serde_json::from_value(json!({
                "Override": {"color_id": 1, "material": material, "pattern_id": u32::MAX}
            })).unwrap();
            assert_eq!(&style.observed()[8..12], &u32::MAX.to_le_bytes());
            assert_eq!(serde_json::to_value(style).unwrap()["Override"]["pattern_id"], u32::MAX);
        }
        for value in [
            json!({"Override": {"material": "Stone"}}),
            json!({"Override": {"color_id": 1}}),
            json!({"Override": {"color_id": 1, "material": "Metal"}}),
            json!({"Override": {"color_id": 1, "material": "Stone", "pattern_id": -1}}),
            json!({"Override": {"color_id": 1, "material": "Stone", "pattern_id": 4294967296_u64}}),
            json!({"Override": [1, "Stone", 2]}),
        ] {
            assert!(serde_json::from_value::<FlatRoofStyle>(value).is_err());
        }
    }

    #[test]
    fn actual_world_uses_the_same_deserialized_roof_component_and_shape_owner() {
        use bevy_ecs::prelude::World;
        let value = json!({
            "id": 91, "params": required_params(),
            "wall_shape": {"Rectangle": {
                "orientation": [2.0, 3.0], "width": -4.0,
                "length": 8.0, "center": [5.0, 6.0]
            }}, "pivot_y": 7.0, "color_id": 13,
            "gable_style": "Overhang", "flat_roof_style": "Matching"
        });
        let mut world = World::new();
        let entity = world.spawn(serde_json::from_value::<Roof>(value).unwrap()).id();
        let roof = world.query::<&Roof>().single(&world).unwrap();
        assert_eq!(roof.wall_id().0, 91);
        assert_eq!(roof.shape_dims(), [-4.0, 8.0]);
        assert_eq!((at(&roof.0, 0xc), at(&roof.0, 0x10)), (2.0, 3.0));
        assert_eq!(world.get::<Roof>(entity).unwrap().floor_y(), 7.0);
        let legacy: Roof = serde_json::from_value({
            let mut value = required_roof();
            value["wall_shape"] = json!({"Circle": {"origin": [8.0, 9.0], "radius": 2.0}});
            value
        }).unwrap();
        assert_eq!((at(&legacy.0, 0xc), at(&legacy.0, 0x10)), (8.0, 9.0));
        assert_eq!(at(&legacy.0, 0x18).to_bits(), 0);
    }
}
