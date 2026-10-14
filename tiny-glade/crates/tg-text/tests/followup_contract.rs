use tg_text::{Cursor, Edit, RawShapeStream, Selection, TextFragment, TextRenderer};

#[test]
fn render_borrows_atlas_and_reports_last_consumed_count() {
    let mut renderer = TextRenderer::new();
    let (atlas, count) = renderer.render();
    assert_eq!(count, 0);
    assert_eq!(*atlas, renderer.atlas());
    assert!(std::ptr::eq(atlas, renderer.render().0));
    renderer.rect([0.; 2], [1.; 2], 1., &RawShapeStream(vec![0, 7]));
    renderer.rect([1.; 2], [2.; 2], 1., &RawShapeStream(vec![2]));
    // Drawing queues primitives; the getter reads the snapshot saved by consume.
    assert_eq!(renderer.render().1, 0);
    let count = renderer.consume_with(|primitives, words| {
        assert_eq!(words, &[0, 7, 2]);
        primitives.len()
    });
    assert_eq!(renderer.render().1, count.0 as u64);
    renderer.begin_frame();
    assert_eq!(renderer.render().1, 2);
    assert!(renderer.consume().primitives.is_empty());
    assert_eq!(renderer.render().1, 0);
}

#[test]
fn callback_unwind_preserves_pending_stream_after_saving_count() {
    let mut renderer = TextRenderer::new();
    renderer.rect([3.; 2], [4.; 2], 0.5, &RawShapeStream(vec![0, 99]));
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        renderer.consume_with(|_, _| panic!("upload boundary failed"));
    }));
    assert!(result.is_err());
    assert_eq!(renderer.render().1, 1);
    let batch = renderer.consume();
    assert_eq!(batch.primitives.len(), 1);
    assert_eq!(batch.shape_words, &[0, 99]);
    assert_eq!(batch.primitives[0].pos_min, [3.; 2]);
}

#[test]
fn metrics_change_rebuilds_editor_preserving_lines_and_attributes() {
    let mut renderer = TextRenderer::new();
    let font = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .find(|p| p.join("ljxsj_92385").exists())
        .unwrap()
        .join("ljxsj_92385/Tiny Glade/assets/fonts/ubuntu/Ubuntu-Regular.ttf");
    renderer.load_font_from_file(font).unwrap();
    let mut fragment = fragment(20., 25.);

    fragment.set_text(&mut renderer, "first\nsecond");
    fragment.set_weight(tg_text::Weight::BOLD);
    fragment.prepare(&mut renderer);
    fragment.editor_mut().set_cursor(Cursor::new(1, 3));
    fragment
        .editor_mut()
        .set_selection(Selection::Normal(Cursor::new(0, 1)));
    fragment.editor_mut().set_auto_indent(true);
    fragment.editor_mut().set_tab_width(8);
    fragment.editor_mut().with_buffer_mut(|b| {
        b.set_scroll(cosmic_text::Scroll {
            line: 1,
            ..Default::default()
        });
        b.set_redraw(false);
    });
    fragment.set_internal_scaled_metrics(20., 25.);
    assert_eq!(fragment.editor().cursor(), Cursor::new(1, 3));
    assert!(fragment.editor().auto_indent());
    assert_eq!(fragment.editor().tab_width(), 8);
    fragment.set_internal_scaled_metrics(24., 30.);
    // Editor reconstruction does not consume the fragment's separate action queue.
    fragment.editor_action(tg_text::Action::Insert('X'));
    fragment.set_internal_scaled_metrics(26., 32.);
    assert_eq!(fragment.pending_actions(), 1);
    assert_eq!(fragment.text(), "first\nsecond");
    assert_eq!(fragment.editor().cursor(), Cursor::default());
    assert_eq!(fragment.editor().selection(), Selection::None);
    assert!(!fragment.editor().auto_indent());
    assert_eq!(fragment.editor().tab_width(), 4);
    fragment.editor().with_buffer(|b| {
        assert_eq!(b.metrics(), tg_text::Metrics::new(26., 32.));
        assert_eq!(b.size(), (0., 0.));
        assert_eq!(b.scroll(), cosmic_text::Scroll::default());
        assert_eq!(b.wrap(), cosmic_text::Wrap::WordOrGlyph);
        assert_eq!(
            b.lines[0].attrs_list().defaults().weight,
            tg_text::Weight::BOLD
        );
    });
}

#[test]
fn changed_width_marks_redraw_but_align_only_stores_option() {
    let mut fragment = fragment(20., 25.);
    fragment.editor_mut().set_redraw(false);
    fragment.set_align(tg_text::Align::Center);
    assert!(!fragment.editor().redraw());
    fragment.set_max_width(100.);
    assert!(fragment.editor().redraw());
    fragment.editor_mut().set_redraw(false);
    fragment.set_max_width(100.);
    assert!(!fragment.editor().redraw());
    fragment.set_max_width(f32::NAN);
    assert!(fragment.editor().redraw());
    fragment.editor_mut().set_redraw(false);
    fragment.set_max_width(f32::NAN);
    assert!(fragment.editor().redraw());
}

fn fragment(font_size: f32, line_height: f32) -> TextFragment {
    TextFragment::new(
        tg_text::FontMetrics::new(font_size, line_height),
        tg_text::FontFamily::Label,
        std::sync::Arc::new(
            tg_text::FontTable::from_ron("(fonts:{Label:(family:\"Ubuntu\",scale:1.0)})").unwrap(),
        ),
    )
}
