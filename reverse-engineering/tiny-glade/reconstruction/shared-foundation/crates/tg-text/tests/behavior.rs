use serde_json::Value;
use tg_text::{
    FontFamily, FontTable, FontTableEntry, ImageScaling, ImageSource, RawShapeStream, ShapeStream,
    ShapeStreamItem,
};

fn scaling(value: u64) -> ImageScaling {
    match value {
        0 => ImageScaling::Stretch,
        1 => ImageScaling::FitHeight,
        2 => ImageScaling::BlurExtend,
        3 => ImageScaling::Nine,
        _ => panic!("bad fixture"),
    }
}

#[test]
fn original_native_stream_cases() {
    let oracle: Value = serde_json::from_str(include_str!("fixtures/native-oracle.json")).unwrap();
    assert_eq!(oracle["target_execution"], true);
    for (case_id, case) in oracle["cases"].as_array().unwrap().iter().enumerate() {
        let mut stream = ShapeStream::new();
        for op in case["ops"].as_array().unwrap() {
            stream = match op["kind"].as_str().unwrap() {
                "fill" => stream.fill_color(op["value"].as_u64().unwrap() as u32),
                "radius" => {
                    stream.corner_radius(f32::from_bits(op["bits"].as_u64().unwrap() as u32))
                }
                "image" => stream.image(
                    op["index"].as_u64().unwrap() as u32,
                    scaling(op["scaling"].as_u64().unwrap()),
                ),
                "name" => stream.image(
                    op["name"].as_str().unwrap(),
                    scaling(op["scaling"].as_u64().unwrap()),
                ),
                _ => panic!("bad fixture"),
            };
        }
        let prefix = case["prefix"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap() as u32)
            .collect();
        let mut raw = RawShapeStream(prefix);
        let mut calls = Vec::new();
        raw.append(&stream, &mut |name: &str| {
            calls.push(name.to_owned());
            0x12345
        });
        let expected: Vec<u32> = case["words"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap() as u32)
            .collect();
        assert_eq!(raw.words(), expected, "original case {case_id}");
        let expected_calls: Vec<_> = case["resolver_calls"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_owned())
            .collect();
        assert_eq!(calls, expected_calls, "resolver order case {case_id}");
    }
}

#[test]
fn original_native_font_scaling_cases() {
    let oracle: Value = serde_json::from_str(include_str!("fixtures/native-oracle.json")).unwrap();
    assert_eq!(
        tg_text::font_table::default_scale(),
        oracle["default_scale"].as_f64().unwrap() as f32
    );
    for (case_id, case) in oracle["metrics_cases"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
    {
        let bits = |key: &str, index: usize| case[key][index].as_u64().unwrap() as u32;
        let entry = FontTableEntry {
            family: "Baloo 2".into(),
            scale: f32::from_bits(case["scale_bits"].as_u64().unwrap() as u32),
        };
        let scaled = entry.scaled(
            f32::from_bits(bits("input_bits", 0)),
            f32::from_bits(bits("input_bits", 1)),
        );
        assert_eq!(scaled.family, entry.family);
        let actual = [scaled.font_size.to_bits(), scaled.line_height.to_bits()];
        let expected = [bits("output_bits", 0), bits("output_bits", 1)];
        for (a, e) in actual.into_iter().zip(expected) {
            let a_is_nan = f32::from_bits(a).is_nan();
            let e_is_nan = f32::from_bits(e).is_nan();
            // 原构建 mulps 对 inf*0 的 NaN 规范化为 0xFFC00000(-QNaN)；本机 LLVM 规范化为
            // 0x7FC00000(+QNaN)。两者都是 quiet NaN，位模式差异属工具链边界，不作为业务差异。
            assert!(
                (a_is_nan && e_is_nan) || a == e,
                "original metric case {case_id}: expected {e:#08x}, got {a:#08x}"
            );
        }
        assert_eq!(case["family_preserved"], true);
    }
}

#[test]
fn shipped_font_table_and_missing_lookup() {
    let table = FontTable::from_ron(include_str!("fixtures/font-table.ron")).unwrap();
    assert_eq!(table.fonts.len(), 4);
    assert_eq!(table.get(FontFamily::Label).unwrap().family, "Baloo 2");
    assert_eq!(
        table.get(FontFamily::MenuButton).unwrap().family,
        "Averia Sans Libre"
    );
    assert_eq!(table.get(FontFamily::TextEditor).unwrap().scale, 1.0);
    assert_eq!(table.get(FontFamily::Hand).unwrap().family, "Playpen Sans");
    assert!(FontTable::default().get(FontFamily::Hand).is_none());
    for input in [
        r#"()"#,
        r#"(fonts:{},fonts:{})"#,
        r#"(fonts:{Label:(family:"x",scale:1.0,scale:2.0)})"#,
    ] {
        assert!(FontTable::from_ron(input).is_err(), "must reject {input}");
    }
    assert!(FontTable::from_ron("(fonts:{Label:(scale:2.0)})").is_err());
    assert!(FontTable::from_ron(r#"(fonts:{Unknown:(family:"x")})"#).is_err());
    assert!(FontTable::from_ron(r#"(fonts:{Label:(family:"x",family:"y")})"#).is_err());
}

#[test]
fn ron_shape_schema_and_failure_contract() {
    let stream = ShapeStream::from_ron(
        r#"[FillColor(0x000000e0),CornerRadius(15),Image(source:"menu/paper-bg",scaling:Nine)]"#,
    )
    .unwrap();
    assert_eq!(stream.0.len(), 3);
    assert!(
        matches!(&stream.0[2], ShapeStreamItem::Image { source: ImageSource::Name(name), scaling: ImageScaling::Nine } if name == "menu/paper-bg")
    );
    for input in [
        "[Image(source:1,scaling:Nine)]",
        r#"[Image(source:"x")]"#,
        r#"[Image(source:"x",scaling:Unknown)]"#,
        "[Image(scaling:Nine)]",
        r#"[Image(source:"x",source:"y",scaling:Nine)]"#,
        r#"[Image(source:"x",scaling:Nine,scaling:Stretch)]"#,
        "[FillColor(-1)]",
        "[FillColor(4294967296)]",
    ] {
        assert!(ShapeStream::from_ron(input).is_err(), "must reject {input}");
    }
}

#[test]
fn clear_reuse_and_no_images_numeric_route() {
    assert!(ShapeStream::new().is_empty());
    let stream = ShapeStream::new()
        .fill_color(7)
        .image(65537u32, ImageScaling::Nine);
    let mut raw = stream.encode_no_images();
    assert_eq!(raw.words(), [0, 7, 0x03000101]);
    let cap = raw.0.capacity();
    raw.clear();
    assert!(raw.words().is_empty());
    assert_eq!(raw.0.capacity(), cap);
    raw.append(&stream, &mut |_: &str| {
        panic!("numeric image must not resolve")
    });
    assert_eq!(raw.words(), [0, 7, 0x03000101]);
}

#[test]
#[should_panic(expected = "image name in encode_no_images")]
fn no_images_rejects_named_source() {
    ShapeStream::new()
        .image("x", ImageScaling::Stretch)
        .encode_no_images();
}
