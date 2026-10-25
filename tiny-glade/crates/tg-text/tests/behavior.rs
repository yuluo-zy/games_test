//! Public configuration and shape-encoding contracts.
use tg_text::{
    FontFamily, FontTable, FontTableEntry, ImageScaling, RawShapeStream, ShapeStream, TextError,
};

#[test]
fn font_configuration_validates_roles_and_scaled_metrics() {
    let table = FontTable::from_ron(include_str!("fixtures/font-table.ron")).unwrap();
    assert_eq!(table.fonts.len(), 4);
    assert_eq!(table.get(FontFamily::Label).unwrap().family, "Baloo 2");
    let entry = FontTableEntry {
        family: "Example".into(),
        scale: 1.5,
    };
    let scaled = entry.scaled(20., 30.).unwrap();
    assert_eq!((scaled.font_size, scaled.line_height), (30., 45.));
    for scale in [0., -1., f32::NAN, f32::INFINITY] {
        assert!(
            FontTableEntry {
                family: "Example".into(),
                scale
            }
            .scaled(20., 30.)
            .is_err()
        );
    }
    assert!(entry.scaled(f32::MAX, 30.).is_err());
    assert!(FontTable::from_ron("(fonts:{Label:(family: \"Example\",scale:0.0)})").is_err());
    assert!(FontTable::from_ron("(fonts:{Unknown:(family: \"Example\")})").is_err());
}

#[test]
fn shape_encoding_preserves_packets_and_rolls_back_failures() {
    let shape = ShapeStream::new()
        .fill_color(7)
        .image(65535u32, ImageScaling::Nine)
        .corner_radius(12.);
    assert_eq!(
        shape.encode_no_images().unwrap().words(),
        &[0, 7, 0x03ffff01, 0x00000c02]
    );
    let mut raw = RawShapeStream(vec![0, 42]);
    for bad in [
        ShapeStream::new()
            .fill_color(1)
            .image(65536u32, ImageScaling::Stretch),
        ShapeStream::new().corner_radius(f32::NAN),
        ShapeStream::new().corner_radius(-1.),
        ShapeStream::new().corner_radius(256.),
        ShapeStream::new().image("missing", ImageScaling::Stretch),
    ] {
        let before = raw.clone();
        assert!(
            raw.try_append(&bad, &mut |_| Err(TextError::InvalidInput(
                "unresolved image".into()
            )))
            .is_err()
        );
        assert_eq!(raw, before);
    }
    let oversized = (0..128).fold(ShapeStream::new(), |shape, _| shape.fill_color(0));
    assert!(oversized.encode_no_images().is_err());
    assert!(
        ShapeStream::new()
            .image("named", ImageScaling::Stretch)
            .encode_no_images()
            .is_err()
    );
}

#[test]
fn shape_configuration_and_resolver_order_are_explicit() {
    let shape=ShapeStream::from_ron(r#"[FillColor(9), Image(source:"first",scaling:Nine), Image(source:"second",scaling:Stretch)]"#).unwrap();
    let mut raw = RawShapeStream::new();
    let mut calls = Vec::new();
    raw.append(&shape, &mut |name| {
        calls.push(name.to_owned());
        calls.len() as u32
    })
    .unwrap();
    assert_eq!(calls, ["first", "second"]);
    assert_eq!(raw.words(), &[0, 9, 0x03000101, 0x00000201]);
    let capacity = raw.0.capacity();
    raw.clear();
    assert!(raw.words().is_empty());
    assert_eq!(raw.0.capacity(), capacity);
    assert!(ShapeStream::from_ron("[Image(source:1,scaling:Nine)]").is_err());
    assert!(ShapeStream::from_ron("[FillColor(-1)]").is_err());
}
