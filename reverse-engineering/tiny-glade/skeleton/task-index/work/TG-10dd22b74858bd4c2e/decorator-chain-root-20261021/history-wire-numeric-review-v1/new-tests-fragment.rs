#[test]
fn original_terrain_attachment_preserves_exact_f32_bits_across_value_and_compact_wire() -> Result<()>
{
    use country_core::resources::{
        history::serialized::SavedMultiEdit,
        walls::{decorator::DecoratorId, decorator_storage::DecoratorStorageAddr},
    };
    // 原03-halftimber-roof entry53的真实装饰物；保留原数字文本。
    let original: serde_json::Value = serde_json::from_str(
        r#"{
        "edits":[{"DecoratorV5":{"Place":{}}}],
        "decorators":{"diffs":[["Terrain",{"before":{},"after":{"0":{
            "dst":{"Terrain":{"pos_xz":[3.9437943,-5.4331703],"yaw_radians":2.5964363}},
            "ty":"Stairs","rank":{"x":1},"color_id":100003,"seed":0,"clv":5,
            "allow_connection":false,"stair_width":1.5
        }}}]]}
    }"#,
    )?;
    let entry: SavedMultiEdit = serde_json::from_value(original.clone())?;
    let source = entry
        .0
        .auxiliary
        .decorator_diff
        .as_ref()
        .unwrap()
        .get(&DecoratorStorageAddr::Terrain)
        .unwrap()
        .after
        .states
        .get(&DecoratorId(0))
        .unwrap();
    assert_eq!(source.dst_wall as u32, 0x407c_6720);
    let middle = serde_json::to_value(&entry)?;
    let saved: serde_json::Value = serde_json::from_slice(&serde_json::to_vec(&entry)?)?;
    let pointer = "/decorators/diffs/0/1/after/0/dst/Terrain/pos_xz/0";
    assert_eq!(
        middle.pointer(pointer).unwrap().as_f64().unwrap().to_bits(),
        0x400f_8ce4_0000_0000
    );
    // 当前锁定serde_json1.0.85/default解析分支的实际数值合同。
    assert_eq!(
        saved.pointer(pointer).unwrap().as_f64().unwrap().to_bits(),
        0x400f_8ce4_0000_0001
    );
    assert_ne!(original.pointer(pointer), saved.pointer(pointer));
    assert_original_fields_retained(&original, &saved, "fixture entry53");
    let restored: SavedMultiEdit = serde_json::from_value(saved)?;
    assert_eq!(restored.0.auxiliary, entry.0.auxiliary);
    Ok(())
}

#[test]
fn terrain_numeric_comparison_rejects_one_f32_bit_change_and_signed_zero_loss() {
    use serde_json::Value;
    let path = "fixture/decorators/diffs/0/1/after/0/dst/Terrain/pos_xz/0";
    for (original, corrupted) in [
        (f32::from_bits(0x407c_6720), f32::from_bits(0x407c_6721)),
        (-0.0_f32, 0.0_f32),
    ] {
        let failure = std::panic::catch_unwind(|| {
            assert_original_fields_retained(&Value::from(original), &Value::from(corrupted), path);
        });
        assert!(failure.is_err(), "不能放过实际f32位变化");
    }
}

#[test]
fn terrain_numeric_scope_keeps_ids_unknown_fields_and_other_domains_strict() {
    use serde_json::json;
    let a: serde_json::Value = serde_json::from_str("3.9437943").unwrap();
    let b = serde_json::to_value(f32::from_bits(0x407c_6720)).unwrap();
    for path in [
        "fixture/decorators/diffs/0/1/after/0/seed",
        "fixture/decorators/diffs/0/1/after/0/dst/Terrain/unknown",
        "fixture/decorators/diffs/0/1/after/0/dst/Terrain/pos_xz/2",
        "fixture/decorators/diffs/0/1/after/0/dst/Roof/mesh_uv/0",
        "fixture/other/dst/Terrain/pos_xz/0",
    ] {
        assert!(!is_native_terrain_attachment_f32_path(path));
        let failure = std::panic::catch_unwind(|| assert_original_fields_retained(&a, &b, path));
        assert!(failure.is_err(), "未知字段仍必须精确相等：{path}");
    }
    let failure = std::panic::catch_unwind(|| {
        assert_original_fields_retained(
            &json!(16_777_216_u32),
            &json!(16_777_217_u32),
            "fixture/decorators/diffs/0/1/after/0/seed",
        );
    });
    assert!(failure.is_err(), "不能将u32 ID降为f32后误判相等");
    for suffix in [
        "before/0/dst/Terrain/pos_xz/0",
        "after/42/dst/Terrain/pos_xz/1",
        "after/0/dst/Terrain/yaw_radians",
    ] {
        let path = format!("fixture/decorators/diffs/0/1/{suffix}");
        assert!(is_native_terrain_attachment_f32_path(&path));
        assert_original_fields_retained(&a, &b, &path);
    }
}
