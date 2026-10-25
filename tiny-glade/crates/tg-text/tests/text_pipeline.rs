use std::sync::Arc;
use tg_text::{
    Align, AtlasConfig, FontFamily, FontMetrics, FontTable, TextError, TextFragment, TextRenderer,
};

fn font_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .find(|path| path.join("ljxsj_92385").exists())
        .expect("test font asset root")
        .join("ljxsj_92385/Tiny Glade/assets/fonts/ubuntu/Ubuntu-Regular.ttf")
}
fn text() -> TextFragment {
    TextFragment::new(
        FontMetrics::new(24., 30.),
        FontFamily::Label,
        Arc::new(FontTable::from_ron("(fonts:{Label:(family:\"Ubuntu\",scale:1.0)})").unwrap()),
    )
    .unwrap()
}
#[test]
fn real_font_warm_cache_and_same_frame_reset_rebuild() {
    let mut renderer = TextRenderer::new();
    renderer.load_font_from_file(font_path()).unwrap();
    let mut text = text();
    text.set_text(&mut renderer, "Glade e\u{301}").unwrap();
    text.set_align(Align::Left);
    text.prepare(&mut renderer).unwrap();
    text.render(&mut renderer, [100., 50.], [200., 100.], u32::MAX)
        .unwrap();
    let first = renderer.consume().unwrap();
    assert!(!first.primitives.is_empty());
    assert!(!first.glyph_uploads.is_empty());
    let rasterized = renderer.stats.rasterized;
    renderer.begin_frame();
    text.render(&mut renderer, [100., 50.], [200., 100.], u32::MAX)
        .unwrap();
    let warm = renderer.consume().unwrap();
    assert_eq!(warm.primitives, first.primitives);
    assert!(warm.glyph_uploads.is_empty());
    assert_eq!(renderer.stats.rasterized, rasterized);
    let previous = renderer.cache_token();
    renderer.reset_glyph_cache().unwrap();
    renderer.reset_glyph_cache().unwrap();
    assert_ne!(renderer.cache_token(), previous);
    text.render(&mut renderer, [100., 50.], [200., 100.], u32::MAX)
        .unwrap();
    assert!(!renderer.consume().unwrap().glyph_uploads.is_empty());
    text.set_text(&mut renderer, "a a a a a a").unwrap();
    text.render(&mut renderer, [100., 50.], [200., 200.], u32::MAX)
        .unwrap();
    let wide = text.bounds();
    renderer.begin_frame();
    text.render(&mut renderer, [16., 100.], [32., 200.], u32::MAX)
        .unwrap();
    let narrow = text.bounds();
    assert!(
        narrow[0] < wide[0],
        "width must reflect the new wrapped layout"
    );
    assert!(narrow[0] <= 32.);
    assert!(narrow[1] > wide[1]);
}
#[test]
fn bounded_growth_requires_redraw_and_never_emits_old_generation() {
    let mut renderer = TextRenderer::with_config(AtlasConfig {
        width: 8,
        height: 8,
        max_width: 64,
        max_height: 64,
        max_bytes: 64 * 64 * 4,
    })
    .unwrap();
    renderer.load_font_from_file(font_path()).unwrap();
    let mut text = text();
    text.set_text(&mut renderer, "W").unwrap();
    for _ in 0..4 {
        renderer.begin_frame();
        match text.render(&mut renderer, [10., 10.], [100., 100.], u32::MAX) {
            Err(TextError::AtlasChanged) => {
                assert!(matches!(renderer.consume(), Err(TextError::AtlasChanged)));
            }
            Ok(()) => {
                let batch = renderer.consume().unwrap();
                assert!(!batch.primitives.is_empty());
                assert!(
                    batch
                        .glyph_uploads
                        .iter()
                        .all(|upload| upload.generation == renderer.atlas().generation)
                );
                return;
            }
            Err(error) => panic!("unexpected rendering failure: {error}"),
        }
    }
    panic!("glyph did not fit the configured 64x64 budget");
}
#[test]
fn renderer_revisions_are_distinct_and_font_errors_are_recoverable() {
    let mut a = TextRenderer::new();
    let mut b = TextRenderer::new();
    assert_ne!(a.atlas().generation, b.atlas().generation);
    assert_ne!(a.cache_token(), b.cache_token());
    let token = b.cache_token();
    assert!(b.load_font_data(vec![0, 1, 2]).is_err());
    assert_eq!(b.cache_token(), token);
    b.load_font_from_file(font_path()).unwrap();
    assert_ne!(b.cache_token(), token);
    a.load_font_from_file(font_path()).unwrap();
    let mut fragment = text();
    // Shape on A before the fragment has ever generated renderer-bound primitives.
    fragment.set_text(&mut a, "Across renderers").unwrap();
    fragment.prepare(&mut a).unwrap();
    fragment
        .render(&mut b, [100., 50.], [200., 100.], u32::MAX)
        .unwrap();
    let batch = b.consume().unwrap();
    assert!(!batch.primitives.is_empty());
    assert!(!batch.glyph_uploads.is_empty());
    assert!(
        batch
            .glyph_uploads
            .iter()
            .all(|upload| upload.generation == b.atlas().generation)
    );
}
