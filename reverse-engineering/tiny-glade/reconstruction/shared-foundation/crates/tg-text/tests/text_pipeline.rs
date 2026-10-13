use tg_text::{
    Action, Align, Edit, Primitive, RawShapeStream, Selection, TextFragment, TextRenderer,
};
fn renderer() -> TextRenderer {
    let mut r = TextRenderer::new();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .find(|p| p.join("ljxsj_92385").exists())
        .unwrap()
        .join("ljxsj_92385/Tiny Glade/assets/fonts");
    for p in [
        "ubuntu/Ubuntu-Regular.ttf",
        "noto sans sc/NotoSansSC-Regular.ttf",
        "noto emoji/NotoColorEmoji.ttf",
    ] {
        r.load_font_from_file(root.join(p)).unwrap();
    }
    r
}
#[test]
fn real_fonts_edit_and_warm_cache() {
    let mut r = renderer();
    let mut t = TextFragment::new(&mut r, 24., 30.);
    t.set_family("Ubuntu");
    t.set_text(&mut r, "Glade 中文 e\u{301} 😀\nمرحبا");
    t.prepare(&mut r);
    assert!(t.bounds()[0] > 0.);
    assert!(t.bounds()[1] >= 60.);
    t.editor_action(Action::Insert('A'));
    assert_eq!(t.pending_actions(), 1);
    assert!(!t.text().starts_with('A'));
    t.prepare(&mut r);
    assert!(t.text().starts_with('A'));
    assert_eq!(t.pending_actions(), 0);
    t.render(&mut r, [10., 20.], u32::MAX);
    let uploads = r.upload_pending_glyphs();
    assert!(!uploads.is_empty());
    assert!(
        uploads
            .iter()
            .all(|u| u.rgba.len() == u.width as usize * u.height as usize * 4)
    );
    let batch = r.consume();
    assert!(!batch.primitives.is_empty());
    assert_eq!(batch.primitive_bytes().len(), batch.primitives.len() * 48);
    let rasterized = r.stats.rasterized;
    r.begin_frame();
    t.render(&mut r, [10., 20.], u32::MAX);
    assert_eq!(r.stats.rasterized, rasterized);
    assert!(r.upload_pending_glyphs().is_empty());
    assert_eq!(r.consume().primitives, batch.primitives);
    let old = r.atlas().generation;
    r.set_pixels_per_point(2.);
    assert!(r.atlas().generation > old);
    assert_eq!(r.cache_len(), 0);
    r.begin_frame();
    t.render(&mut r, [10., 20.], u32::MAX);
    assert!(!r.upload_pending_glyphs().is_empty());
    assert!(r.stats.rasterized > rasterized);
}
#[test]
fn ownership_selection_alignment_and_shape_packet() {
    let mut r = renderer();
    let mut t = TextFragment::new(&mut r, 20., 25.);
    t.set_text(&mut r, "abc def");
    t.prepare(&mut r);
    t.editor_mut()
        .set_selection(Selection::Normal(tg_text::Cursor::new(0, 3)));
    t.set_cursor_visible(true);
    t.set_align(Align::Center);
    t.render(&mut r, [100., 0.], 0xffff_ffff);
    assert!(r.consume().primitives.iter().any(|p| p.stream == u32::MAX));
    let old = t.take_editor(&mut r);
    assert!(old.with_buffer(|b| b.lines.iter().any(|l| l.text() == "abc def")));
    assert_eq!(t.text(), "");
    let empty = t.replace_editor(old);
    drop(empty);
    assert_eq!(t.text(), "abc def");
    r.begin_frame();
    r.rect([1., 2.], [3., 4.], 7, &RawShapeStream(vec![0, 99]));
    r.rect([5., 6.], [7., 8.], 9, &RawShapeStream(vec![2]));
    let b = r.consume();
    assert_eq!(b.shape_words, vec![0, 99, 2]);
    assert_eq!(b.primitives[0].stream, 2);
    assert_eq!(b.primitives[1].stream, (2 << 8) | 1);
    assert_eq!(std::mem::size_of::<Primitive>(), 48);
}
#[test]
fn overflow_rebuild_is_deferred_and_recovered_next_frame() {
    let mut r = TextRenderer::with_atlas_size(8, 8);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .find(|p| p.join("ljxsj_92385").exists())
        .unwrap()
        .join("ljxsj_92385/Tiny Glade/assets/fonts/ubuntu/Ubuntu-Regular.ttf");
    r.load_font_from_file(root).unwrap();
    let mut t = TextFragment::new(&mut r, 24., 30.);
    t.set_text(&mut r, "W");
    let before = r.atlas();
    t.render(&mut r, [0., 0.], u32::MAX);
    assert_eq!(r.atlas(), before);
    assert!(r.upload_pending_glyphs().is_empty());
    assert_eq!(r.atlas().width, 16);
    assert!(r.atlas().generation > before.generation);
    for _ in 0..5 {
        r.begin_frame();
        t.render(&mut r, [0., 0.], u32::MAX);
        let uploads = r.upload_pending_glyphs();
        if !uploads.is_empty() {
            assert!(!r.consume().primitives.is_empty());
            return;
        }
    }
    panic!("glyph did not recover after atlas growth");
}

#[test]
fn same_text_resets_cursor_and_font_roles_scale_metrics() {
    let mut r = renderer();
    let table =
        tg_text::FontTable::from_ron("(fonts:{Label:(family:\"Ubuntu\",scale:1.5)})").unwrap();
    let mut t = TextFragment::with_font_table(&mut r, &table, tg_text::FontFamily::Label, 20., 30.);
    t.set_text(&mut r, "abc");
    t.prepare(&mut r);
    assert_eq!(
        t.editor().with_buffer(|b| b.metrics()),
        tg_text::Metrics::new(30., 45.)
    );
    t.editor_mut().set_cursor(tg_text::Cursor::new(0, 2));
    t.set_text(&mut r, "abc");
    assert_eq!(t.editor().cursor(), tg_text::Cursor::default());
    assert_eq!(t.pending_actions(), 1);
    t.prepare(&mut r);
    assert_eq!(t.pending_actions(), 0);
    t.set_metrics(&mut r, 10., 15.);
    assert_eq!(
        t.editor().with_buffer(|b| b.metrics()),
        tg_text::Metrics::new(15., 22.5)
    );
}

#[test]
fn expired_allocations_are_evicted_and_stale_refs_rejected() {
    let mut r = TextRenderer::with_atlas_size(32, 32);
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .find(|p| p.join("ljxsj_92385").exists())
        .unwrap()
        .join("ljxsj_92385/Tiny Glade/assets/fonts/ubuntu/Ubuntu-Regular.ttf");
    r.load_font_from_file(root).unwrap();
    let mut a = TextFragment::new(&mut r, 24., 30.);
    a.set_text(&mut r, "W");
    a.prepare(&mut r);
    let key = a.editor().with_buffer(|b| {
        b.layout_runs().next().unwrap().glyphs[0]
            .physical((0., 0.), 1.)
            .cache_key
    });
    let first = r.get_glyph(key).unwrap();
    r.upload_pending_glyphs();
    r.consume();
    r.consume();
    r.consume();
    let mut b = TextFragment::new(&mut r, 24., 30.);
    b.set_text(&mut r, "M");
    b.render(&mut r, [0., 0.], u32::MAX);
    assert!(r.stats.evicted > 0);
    assert!(!r.validate_glyph_cache_allocs(&[first]));
    assert!(!r.upload_pending_glyphs().is_empty());
    assert_eq!(r.atlas().width, 32);
}

#[test]
fn original_native_rect_opacity_and_stream_packet() {
    let evidence: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/rect-native-oracle.json")).unwrap();
    for case in evidence["cases"].as_array().unwrap() {
        let mut r = TextRenderer::new();
        r.rect([0.; 2], [0.; 2], 0, &RawShapeStream(vec![99]));
        let words = case["words"]
            .as_array()
            .unwrap()
            .iter()
            .map(|w| w.as_u64().unwrap() as u32)
            .collect::<Vec<_>>();
        r.rect_with_opacity(
            [3.25, -7.5],
            [9.5, 21.25],
            case["input_color"].as_f64().unwrap() as f32,
            &RawShapeStream(words),
        );
        let batch = r.consume();
        let actual: &[u32] = bytemuck::cast_slice(std::slice::from_ref(&batch.primitives[1]));
        assert_eq!(
            actual,
            case["primitive_words"]
                .as_array()
                .unwrap()
                .iter()
                .map(|w| w.as_u64().unwrap() as u32)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            batch.shape_words,
            case["shape_words"]
                .as_array()
                .unwrap()
                .iter()
                .map(|w| w.as_u64().unwrap() as u32)
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn centered_box_reflow_and_fresh_gpu_cache_reset() {
    let mut r = renderer();
    let mut t = TextFragment::new(&mut r, 24., 30.);
    t.set_text(&mut r, "a long line of text requiring wrapping");
    t.set_align(Align::Center);
    t.render_in_box(&mut r, [100., 100.], [60., 500.], u32::MAX);
    assert!(t.bounds()[1] > 30.);
    let b = r.consume();
    let h = t.bounds()[1];
    for (local, drawn) in t.primitives().iter().zip(&b.primitives) {
        assert_eq!(
            drawn.pos_min,
            [local.pos_min[0] + 70., local.pos_min[1] + 100. - h * 0.5]
        );
    }
    assert!(!r.upload_pending_glyphs().is_empty());
    let previous = r.atlas();
    r.reset_glyph_cache();
    assert!(r.atlas().generation > previous.generation);
    assert_eq!(r.cache_len(), 0);
    t.render_in_box(&mut r, [100., 100.], [60., 500.], u32::MAX);
    assert!(!r.upload_pending_glyphs().is_empty());
}
#[test]
fn font_initialization_and_ecs_use_dependencies() {
    let fs = tg_text::init_font_system(Some("ko-KR".into()));
    assert_eq!(fs.locale(), "ko-KR");
    assert_eq!(fs.db().faces().count(), 0);
    use cosmic_text::fontdb::Family;
    for (role, name) in [
        (Family::Serif, "Times New Roman"),
        (Family::SansSerif, "Arial"),
        (Family::Cursive, "Comic Sans MS"),
        (Family::Fantasy, "Impact"),
        (Family::Monospace, "Courier New"),
    ] {
        assert_eq!(fs.db().family_name(&role), name);
    }
    let mut r = renderer();
    let t = TextFragment::new(&mut r, 24., 30.);
    let mut world = bevy_ecs::world::World::new();
    let entity = world.spawn(t).id();
    world.insert_resource(r);
    assert!(world.get::<TextFragment>(entity).is_some());
    assert!(world.contains_resource::<TextRenderer>());
    assert!(world.despawn(entity));
}

#[test]
fn real_fork_fallback_selects_cjk_and_color_emoji_faces() {
    let mut r = renderer();
    let mut text = TextFragment::new(&mut r, 24., 30.);
    text.set_family("Ubuntu");
    text.set_text(&mut r, "中 😀");
    text.prepare(&mut r);
    let choices = text.editor().with_buffer(|b| {
        b.layout_runs()
            .flat_map(|run| {
                run.glyphs
                    .iter()
                    .map(move |g| (run.text[g.start..g.end].to_owned(), g.font_id, g.glyph_id))
            })
            .collect::<Vec<_>>()
    });
    for ch in ["中", "😀"] {
        let (_, id, glyph) = choices.iter().find(|(s, _, _)| s.contains(ch)).unwrap();
        assert_ne!(*glyph, 0);
        let face = r.font_system.db().face(*id).unwrap();
        let names = face
            .families
            .iter()
            .map(|(n, _)| n.as_str())
            .collect::<Vec<_>>();
        assert!(
            names.iter().any(|n| if ch == "中" {
                n.contains("Noto Sans SC")
            } else {
                n.contains("Noto Color Emoji")
            }),
            "{ch}: {names:?}"
        );
    }
}

#[test]
fn original_native_opacity_extremes_and_color_preservation() {
    let evidence: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/opacity-native-oracle.json")).unwrap();
    for c in evidence["cases"].as_array().unwrap() {
        let actual = tg_text::scale_u32_packed_srgba_color(
            c["color"].as_u64().unwrap() as u32,
            f32::from_bits(c["scale_bits"].as_u64().unwrap() as u32),
        );
        assert_eq!(actual, c["result"].as_u64().unwrap() as u32);
    }
}

#[test]
fn external_validation_font_rtl_glyphs_and_visual_positions() {
    // 外部验证字体，不属于原作 font initialization/发行字体选择证据。
    let mut r = renderer();
    r.load_font_from_file("C:/Windows/Fonts/arial.ttf").unwrap();
    let mut text = TextFragment::new(&mut r, 24., 30.);
    text.set_family("Arial");
    text.set_text(&mut r, "שלום");
    text.render(&mut r, [10., 20.], u32::MAX);
    let (rtl, mut positions) = text.editor().with_buffer(|b| {
        let run = b.layout_runs().next().unwrap();
        (
            run.rtl,
            run.glyphs
                .iter()
                .map(|g| (g.start, g.end, g.x, g.w, g.glyph_id, g.font_id))
                .collect::<Vec<_>>(),
        )
    });
    assert!(rtl);
    assert_eq!(positions.len(), 4);
    positions.sort_by_key(|g| g.0);
    for (start, end, x, width, glyph, font) in &positions {
        assert_ne!(*glyph, 0);
        assert!(x.is_finite() && width.is_finite() && *x >= 0. && *width > 0.);
        assert_eq!(end - start, 2);
        assert!(
            r.font_system
                .db()
                .face(*font)
                .unwrap()
                .families
                .iter()
                .any(|(name, _)| name == "Arial")
        );
    }
    // 仅验证依赖计算结果；不在产品实现中排序/重新实现 bidi。
    assert!(positions.windows(2).all(|pair| pair[0].2 > pair[1].2));
    let batch = r.consume();
    assert_eq!(batch.primitives.iter().filter(|p| p.stream == 0).count(), 4);
    assert!(!r.upload_pending_glyphs().is_empty());
}
