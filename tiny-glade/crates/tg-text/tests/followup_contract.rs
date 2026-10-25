use std::sync::Arc;
use tg_text::{
    Action, Align, Cursor, Edit, FontFamily, FontMetrics, FontTable, TextFragment, TextRenderer,
};

fn table() -> Arc<FontTable> {
    Arc::new(
        FontTable::from_ron(
            r#"(fonts:{Label:(family:"Sans",scale:1.0),Hand:(family:"Serif",scale:2.0)})"#,
        )
        .unwrap(),
    )
}
fn fragment() -> TextFragment {
    TextFragment::new(FontMetrics::new(12., 16.), FontFamily::Label, table()).unwrap()
}

#[test]
fn role_round_trip_and_metric_updates_use_current_role() {
    let mut text = fragment();
    text.set_family(FontFamily::Hand).unwrap();
    assert_eq!(
        text.editor().with_buffer(|buffer| buffer.metrics()),
        tg_text::Metrics::new(24., 32.)
    );
    text.set_metrics(FontMetrics::new(10., 15.)).unwrap();
    assert_eq!(
        text.editor().with_buffer(|buffer| buffer.metrics()),
        tg_text::Metrics::new(20., 30.)
    );
    text.set_family(FontFamily::Label).unwrap();
    assert_eq!(
        text.editor().with_buffer(|buffer| buffer.metrics()),
        tg_text::Metrics::new(10., 15.)
    );
}

#[test]
fn missing_roles_and_invalid_numbers_return_errors_without_mutation() {
    assert!(TextFragment::new(12., FontFamily::Label, Arc::new(FontTable::default())).is_err());
    let mut text = fragment();
    for value in [0., -1., f32::NAN, f32::INFINITY] {
        assert!(text.set_metrics(FontMetrics::new(value, 16.)).is_err());
        assert!(text.set_internal_scaled_metrics(12., value).is_err());
        assert!(text.set_max_width(value).is_err());
    }
    assert!(text.set_family(FontFamily::TextEditor).is_err());
    assert_eq!(
        text.editor().with_buffer(|buffer| buffer.metrics()),
        tg_text::Metrics::new(12., 16.)
    );
    for ron in [
        r#"(fonts:{Label:(family:"",scale:1.0)})"#,
        r#"(fonts:{Label:(family:"Sans",scale:0.0)})"#,
    ] {
        assert!(FontTable::from_ron(ron).is_err());
    }
}

#[test]
fn metrics_preserve_editor_state_and_take_discards_old_actions() {
    let mut text = fragment();
    text.editor_mut().with_buffer_mut(|buffer| {
        buffer.lines = ["first", "second"]
            .into_iter()
            .map(|line| {
                cosmic_text::BufferLine::new(
                    line.to_owned(),
                    cosmic_text::AttrsList::new(cosmic_text::Attrs::new()),
                    cosmic_text::Shaping::Advanced,
                )
            })
            .collect();
    });
    text.editor_mut().set_cursor(Cursor::new(1, 3));
    text.editor_mut().set_auto_indent(true);
    text.editor_mut().set_tab_width(8);
    text.set_metrics(FontMetrics::new(20., 24.)).unwrap();
    assert_eq!(text.editor().cursor(), Cursor::new(1, 3));
    assert!(text.editor().auto_indent());
    assert_eq!(text.editor().tab_width(), 8);
    assert_eq!(text.text(), "first\nsecond");
    text.editor_action(Action::Insert('x'));
    let old = text.take_editor();
    assert_eq!(text.pending_actions(), 0);
    assert_eq!(text.text(), "");
    assert_eq!(text.text(), "");
    text.replace_editor(old).unwrap();
    assert_eq!(text.text(), "first\nsecond");
}

#[test]
fn layout_and_direct_editor_mutation_invalidate_and_missing_fonts_return_errors() {
    let mut text = fragment();
    let mut renderer = TextRenderer::new();
    text.editor_mut().set_redraw(false);
    text.set_align(Align::Center);
    assert!(text.editor().redraw());
    text.editor_mut().set_redraw(false);
    text.clear_align();
    assert!(text.editor().redraw());
    text.editor_mut().set_cursor(Cursor::default());
    assert!(text.primitives().is_empty());
    assert!(text.set_text(&mut renderer, "hello").is_err());
    assert!(text.prepare(&mut renderer).is_err());
    assert!(
        text.render(&mut renderer, [0.; 2], [100., 100.], u32::MAX)
            .is_err()
    );
    assert_eq!(text.text(), "");
}
