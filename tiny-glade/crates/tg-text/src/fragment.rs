//! 原 Cosmic Editor 上的游戏文本片段策略；不实现整形、字体光栅或 Unicode 分割。
use crate::renderer::{Glyph, Primitive, TextRenderer};
use crate::{FontFamily, FontMetrics, FontTable};
use cosmic_text::{
    Action, Align, Attrs, AttrsList, AttrsOwned, Buffer, Cursor, Edit, Editor, Family, FamilyOwned,
    Metrics, Selection, Shaping, Weight,
};
use std::sync::Arc;
use unicode_segmentation::UnicodeSegmentation;

#[derive(bevy_ecs::prelude::Component)]
pub struct TextFragment {
    editor: Editor<'static>,
    actions: Vec<Action>,
    metrics: FontMetrics,
    font_table: Arc<FontTable>,
    family: FontFamily,
    attrs: AttrsOwned,
    max_width: f32,
    align: Option<Align>,
    bounds: [f32; 2],
    primitives: Vec<Primitive>,
    glyphs: Vec<Glyph>,
    generated_at: u32,
    validated_at: u32,
    cursor_visible: bool,
}

impl TextFragment {
    /// Original new<f32>: the shared table is moved into the fragment.
    /// Buffer construction does not load fonts or require a FontSystem.
    pub fn new(
        metrics: impl Into<FontMetrics>,
        family: FontFamily,
        font_table: Arc<FontTable>,
    ) -> Self {
        let metrics = metrics.into();
        let entry = font_table.get(family).unwrap();
        let scaled = entry.scaled(metrics.font_size, metrics.line_height);
        let buffer = Buffer::new_empty(Metrics::new(scaled.font_size, scaled.line_height));
        let attrs = AttrsOwned::new(Attrs::new().family(Family::Name(scaled.family)));
        Self {
            editor: Editor::new(buffer),
            actions: Vec::new(),
            metrics,
            font_table,
            family,
            attrs,
            max_width: f32::MAX,
            align: None,
            bounds: [0.; 2],
            primitives: Vec::new(),
            glyphs: Vec::new(),
            generated_at: 0,
            validated_at: 0,
            cursor_visible: false,
        }
    }
    pub fn editor(&self) -> &Editor<'static> {
        &self.editor
    }
    pub fn editor_mut(&mut self) -> &mut Editor<'static> {
        &mut self.editor
    }
    /// 原 0x140b906d0：drop 旧 editor 后装入新 editor，返回 void（丢弃旧值）。
    pub fn replace_editor(&mut self, editor: Editor<'static>) {
        self.generated_at = 0;
        self.editor = editor;
    }
    pub fn take_editor(&mut self) -> Editor<'static> {
        let metrics = self.editor.with_buffer(|b| b.metrics());
        let buffer = Buffer::new_empty(metrics);
        std::mem::replace(&mut self.editor, Editor::new(buffer))
    }
    pub fn text(&self) -> String {
        self.editor.with_buffer(|b| {
            b.lines
                .iter()
                .map(|l| l.text())
                .collect::<Vec<_>>()
                .join("\n")
        })
    }
    pub fn set_text(&mut self, renderer: &mut TextRenderer, text: impl AsRef<str>) {
        let changed = self.text() != text.as_ref();
        let attrs = self.attrs.as_attrs();
        if changed {
            self.editor.with_buffer_mut(|b| {
                b.set_text(
                    &mut renderer.font_system,
                    text.as_ref(),
                    attrs,
                    Shaping::Advanced,
                )
            });
        }
        self.editor.set_cursor(Cursor::default());
        self.editor.set_selection(Selection::None);
        self.editor_action(Action::Motion(cosmic_text::Motion::BufferEnd));
    }
    pub fn apply_buffer_attrs(&mut self) {
        let attrs = AttrsList::new(self.attrs.as_attrs());
        self.editor.with_buffer_mut(|b| {
            for line in &mut b.lines {
                line.set_attrs_list(attrs.clone());
            }
            b.set_redraw(true);
        });
    }
    /// Original 0x140b90c80 compares the construction role but never writes it.
    /// Preserve that observed behavior: a later metrics change uses the original role.
    pub fn set_family(&mut self, family: FontFamily) {
        if self.family == family {
            return;
        }
        let table = Arc::clone(&self.font_table);
        let entry = table.get(family).unwrap();
        self.attrs.family_owned = FamilyOwned::Name(entry.family.clone());
        self.apply_buffer_attrs();
        self.set_internal_scaled_metrics(
            self.metrics.font_size * entry.scale,
            entry.scale * self.metrics.line_height,
        );
    }
    pub fn set_weight(&mut self, weight: Weight) {
        if self.attrs.weight != weight {
            self.attrs.weight = weight;
            self.apply_buffer_attrs();
        }
    }
    pub fn set_metrics(&mut self, metrics: impl Into<FontMetrics>) {
        let metrics = metrics.into();
        if self.metrics != metrics {
            let (font_size, line_height) = {
                let entry = self.font_table.get(self.family).unwrap();
                let scaled = entry.scaled(metrics.font_size, metrics.line_height);
                (scaled.font_size, scaled.line_height)
            };
            self.set_internal_scaled_metrics(font_size, line_height);
            self.metrics = metrics;
        }
    }
    pub fn set_internal_scaled_metrics(&mut self, font_size: f32, line_height: f32) {
        // 原 0x140b90780：传入已缩放 metrics，与当前 buffer 相等且均非 NaN 时早退。
        let current = self
            .editor
            .with_buffer(|b| (b.metrics().font_size, b.metrics().line_height));
        let equal = !current.0.is_nan()
            && !font_size.is_nan()
            && current.0 == font_size
            && !current.1.is_nan()
            && !line_height.is_nan()
            && current.1 == line_height;
        if equal {
            return;
        }
        // 原 0x140b90780：move 原 buffer 的 lines 到 Buffer::new_empty，
        // 然后 Editor::new；保留文本/attrs，重置 cursor、selection、scroll、
        // 尺寸、wrap 以及 editor 的 auto_indent/tab_width/change 状态。
        // cosmic 的 set_metrics 会保留上述状态，不能替代这段游戏策略。
        let lines = self
            .editor
            .with_buffer_mut(|b| std::mem::take(&mut b.lines));
        let mut buffer = Buffer::new_empty(Metrics::new(font_size, line_height));
        buffer.lines = lines;
        self.editor = Editor::new(buffer);
    }
    pub fn set_max_width(&mut self, width: f32) {
        if self.max_width != width {
            self.max_width = width;
            self.editor.set_redraw(true);
        }
    }
    pub fn set_align(&mut self, align: Align) {
        self.align = Some(align);
    }
    pub fn clear_align(&mut self) {
        self.align = None;
    }
    pub fn set_cursor_visible(&mut self, visible: bool) {
        if self.cursor_visible != visible {
            self.cursor_visible = visible;
            self.editor.set_redraw(true);
        }
    }
    pub fn bounds(&self) -> [f32; 2] {
        self.bounds
    }
    pub fn editor_action(&mut self, action: Action) {
        self.actions.push(action);
    }
    pub fn pending_actions(&self) -> usize {
        self.actions.len()
    }
    pub fn prepare(&mut self, renderer: &mut TextRenderer) {
        for action in self.actions.drain(..) {
            self.editor.action(&mut renderer.font_system, action);
        }
        self.editor.with_buffer_mut(|b| {
            for line in &mut b.lines {
                line.set_align(None);
            }
            b.set_wrap(&mut renderer.font_system, cosmic_text::Wrap::Word);
            b.set_size(&mut renderer.font_system, self.max_width, f32::MAX);
        });
        self.editor
            .shape_as_needed(&mut renderer.font_system, false);
        // 原 0x140b90f60：bounds[0] 由 with_buffer 布局闭包取 max(line_w)；
        // bounds[1] reads the active Buffer's scaled line_height. The original
        // Owned/Borrowed/Arc BufferRef branches are unrelated to font roles.
        let width = self
            .editor
            .with_buffer(|b| b.layout_runs().fold(0_f32, |acc, run| acc.max(run.line_w)));
        self.bounds = [
            width,
            self.editor.with_buffer(|b| b.metrics().line_height) + 0.0,
        ];
    }
    fn needs_regeneration(&mut self, renderer: &mut TextRenderer) -> bool {
        if !self.editor.redraw() && self.generated_at >= renderer.invalidation_frame() {
            if self.validated_at < renderer.current_frame() {
                if renderer.validate_glyph_cache_allocs(&self.glyphs) {
                    self.validated_at = renderer.current_frame();
                } else {
                    self.generated_at = 0;
                }
            }
            if self.generated_at >= renderer.invalidation_frame() {
                return false;
            }
        }
        true
    }
    fn generate_primitives(&mut self, renderer: &mut TextRenderer, extent: [f32; 2]) {
        if !self.needs_regeneration(renderer) {
            return;
        }
        self.primitives.clear();
        self.glyphs.clear();
        self.layout_for_render(renderer, extent, self.align);
        self.editor.shape_as_needed(&mut renderer.font_system, true);
        let scale = renderer.pixels_per_point();
        let selection = self.editor.selection_bounds();
        let cursor = self.editor.cursor();
        let visible = self.cursor_visible;
        self.editor.with_buffer(|b| {
            for run in b.layout_runs() {
                if let Some((x, width)) =
                    selection.and_then(|(start, end)| run.highlight(start, end))
                {
                    self.primitives.push(Primitive::rect(
                        [x, run.line_top],
                        [width, b.metrics().line_height],
                        0x80ff_ffff,
                    ));
                }
                if visible && cursor.line == run.line_i {
                    let mut cursor_x = if run.rtl { run.line_w } else { 0. };
                    for glyph in run.glyphs {
                        if cursor.index >= glyph.start && cursor.index <= glyph.end {
                            let cluster = &run.text[glyph.start..glyph.end];
                            let total = cluster.grapheme_indices(true).count();
                            let before = cluster
                                .grapheme_indices(true)
                                .filter(|(i, _)| glyph.start + i < cursor.index)
                                .count();
                            let width = if total == 0 {
                                0.
                            } else {
                                glyph.w * before as f32 / total as f32
                            };
                            cursor_x = if glyph.level.is_rtl() {
                                glyph.x + glyph.w - width
                            } else {
                                glyph.x + width
                            };
                            break;
                        }
                    }
                    self.primitives.push(Primitive::rect(
                        [cursor_x, run.line_top],
                        [4., b.metrics().line_height],
                        u32::MAX,
                    ));
                }
                for glyph in run.glyphs {
                    let physical = glyph.physical((0., 0.), scale);
                    if let Some(cached) = renderer.get_glyph(physical.cache_key) {
                        self.glyphs.push(cached);
                        self.primitives.push(Primitive {
                            pos_min: [
                                (physical.x as f32 + cached.bearing[0] as f32) / scale,
                                (physical.y as f32
                                    + cached.bearing[1] as f32
                                    + (run.line_y * scale) as i32 as f32)
                                    / scale,
                            ],
                            pos_extent: [
                                cached.size[0] as f32 / scale,
                                cached.size[1] as f32 / scale,
                            ],
                            uv_min: [cached.atlas[0] as f32, cached.atlas[1] as f32],
                            uv_extent: [cached.size[0] as f32, cached.size[1] as f32],
                            color: glyph.color_opt.map_or(u32::MAX, |c| c.0),
                            stream: 0,
                            padding: [0; 2],
                        });
                    }
                }
            }
        });
        self.bounds[1] = self.editor.with_buffer(|b| {
            b.metrics().line_height
                + b.layout_runs()
                    .fold(0_f32, |top, run| top.max(run.line_top))
        });
        self.generated_at = renderer.invalidation_frame();
    }
    pub fn primitives(&self) -> &[Primitive] {
        &self.primitives
    }
    fn layout_for_render(
        &mut self,
        renderer: &mut TextRenderer,
        extent: [f32; 2],
        align: Option<Align>,
    ) {
        // Original closure returns immediately for Align::None. Layout changes
        // are performed only after the original cache predicate requests generation.
        if align.is_none() {
            return;
        }
        self.editor.with_buffer_mut(|b| {
            for line in &mut b.lines {
                line.set_align(align);
            }
            b.set_size(&mut renderer.font_system, extent[0], extent[1]);
        });
    }
    /// Original render takes a center and extent. Preparation is a separate step;
    /// render does not force preparation, left alignment, or a new cache epoch.
    pub fn render(
        &mut self,
        renderer: &mut TextRenderer,
        center: [f32; 2],
        extent: [f32; 2],
        color: u32,
    ) {
        self.generate_primitives(renderer, extent);
        renderer.append_primitives(
            &self.primitives,
            [
                center[0] - extent[0] * 0.5,
                center[1] - self.bounds[1] * 0.5,
            ],
            color,
        );
    }
}

#[cfg(test)]
mod internal_contract_tests {
    use super::*;
    use crate::{FontTableEntry, renderer::AtlasAllocUsage};

    fn table() -> Arc<FontTable> {
        Arc::new(FontTable {
            fonts: [
                (
                    FontFamily::Label,
                    FontTableEntry {
                        family: "FamilyZero".into(),
                        scale: 1.,
                    },
                ),
                (
                    FontFamily::MenuButton,
                    FontTableEntry {
                        family: "FamilyOne".into(),
                        scale: 2.,
                    },
                ),
                (
                    FontFamily::TextEditor,
                    FontTableEntry {
                        family: "FamilyTwo".into(),
                        scale: 0.5,
                    },
                ),
                (
                    FontFamily::Hand,
                    FontTableEntry {
                        family: "FamilyThree".into(),
                        scale: 3.,
                    },
                ),
            ]
            .into_iter()
            .collect(),
        })
    }
    fn role(tag: u64) -> FontFamily {
        [
            FontFamily::Label,
            FontFamily::MenuButton,
            FontFamily::TextEditor,
            FontFamily::Hand,
        ][tag as usize]
    }
    fn native_font_state(
        fragment: &TextFragment,
        table_ptr: *const FontTable,
    ) -> serde_json::Value {
        let metrics = fragment.editor.with_buffer(|b| b.metrics());
        let family = match &fragment.attrs.family_owned {
            FamilyOwned::Name(name) => name.as_str(),
            _ => panic!("original table entries select named families"),
        };
        serde_json::json!({
            "role": fragment.family as u8,
            "metrics_base": [fragment.metrics.font_size, fragment.metrics.line_height],
            "metrics_scaled": [metrics.font_size, metrics.line_height],
            "bounds": fragment.bounds,
            "max_width_bits": fragment.max_width.to_bits(),
            "align_tag": if fragment.align.is_none() { 5 } else { panic!("expected None") },
            "attrs_family": family,
            "attrs_weight": fragment.attrs.weight.0,
            "buffer_lines": fragment.editor.with_buffer(|b| b.lines.len()),
            "pending_actions": fragment.actions.len(),
            "arc_strong": Arc::strong_count(&fragment.font_table),
            "stored_arc_matches": Arc::as_ptr(&fragment.font_table) == table_ptr,
        })
    }
    #[test]
    fn original_font_constructor_and_role_switch_sequence() {
        let original: serde_json::Value =
            serde_json::from_str(include_str!("../tests/fixtures/internal-native-font.json"))
                .unwrap();
        for case in original["role_cases"].as_array().unwrap() {
            let decoded: FontFamily = serde_json::from_value(case["name"].clone()).unwrap();
            assert_eq!(u64::from(decoded as u8), case["tag"].as_u64().unwrap());
        }
        let cases = original["scenarios"].as_array().unwrap();
        for case in &cases[..4] {
            let table = table();
            let ptr = Arc::as_ptr(&table);
            let fragment = TextFragment::new(20_f32, role(case["role"].as_u64().unwrap()), table);
            assert_eq!(native_font_state(&fragment, ptr), case["state"]);
        }
        let table = table();
        let ptr = Arc::as_ptr(&table);
        let mut fragment = TextFragment::new(20_f32, FontFamily::Label, table);
        fragment.editor_action(Action::Insert('X'));
        fragment.set_family(FontFamily::MenuButton);
        assert_eq!(native_font_state(&fragment, ptr), cases[4]["state"]);
        fragment.set_family(FontFamily::Label);
        assert_eq!(native_font_state(&fragment, ptr), cases[5]["state"]);
        fragment.set_metrics(25_f32);
        assert_eq!(native_font_state(&fragment, ptr), cases[6]["state"]);
        fragment.take_editor();
        assert_eq!(native_font_state(&fragment, ptr), cases[7]["state"]);
    }
    #[test]
    fn shared_font_table_is_retained_and_missing_role_uses_unwrap() {
        let table = table();
        let weak = Arc::downgrade(&table);
        let mut first = TextFragment::new(20_f32, FontFamily::Label, Arc::clone(&table));
        let second = TextFragment::new(10_f32, FontFamily::Hand, Arc::clone(&table));
        assert!(Arc::ptr_eq(&first.font_table, &second.font_table));
        assert_eq!(Arc::strong_count(&table), 3);
        first.set_family(FontFamily::MenuButton);
        assert_eq!(Arc::strong_count(&table), 3);
        drop(table);
        drop(first);
        assert!(weak.upgrade().is_some());
        drop(second);
        assert!(weak.upgrade().is_none());
        let empty = Arc::new(FontTable::default());
        assert!(
            std::panic::catch_unwind(|| TextFragment::new(20_f32, FontFamily::Label, empty))
                .is_err()
        );
    }
    #[test]
    fn original_cache_decision_matrix_and_prefix_updates() {
        let original: serde_json::Value = serde_json::from_str(include_str!(
            "../tests/fixtures/internal-native-cache-oracle.json"
        ))
        .unwrap();
        let mut renderer = TextRenderer::new();
        let table = table();
        for case in original["prefix_cases"].as_array().unwrap() {
            let mut fragment = TextFragment::new(20_f32, FontFamily::Label, Arc::clone(&table));
            fragment.editor.set_redraw(false);
            fragment.generated_at = case["fragment_stamp"].as_u64().unwrap() as u32;
            fragment.validated_at = case["validated_frame"].as_u64().unwrap() as u32;
            let pattern = case["pattern"].as_str().unwrap();
            renderer.set_cache_fixture(
                case["frame"].as_u64().unwrap() as u32,
                case["renderer_stamp"].as_u64().unwrap() as u32,
                &[
                    [9, 65535],
                    [if pattern == "zero-sentinel" { 0 } else { 9 }, 65535],
                ],
            );
            let mut ids = if pattern == "empty" {
                Vec::new()
            } else {
                vec![0xffff0000]
            };
            match pattern {
                "generation-mismatch" => ids.push(0xfffe0001),
                "zero-sentinel" => ids.push(0xffff0001),
                "out-of-bounds" => ids.push(0xffff0002),
                _ => {}
            }
            fragment.glyphs = ids
                .into_iter()
                .map(|id| Glyph {
                    allocation: etagere::AllocId::deserialize(id),
                    atlas: [0; 2],
                    size: [1; 2],
                    bearing: [0; 2],
                })
                .collect();
            let actual = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                fragment.needs_regeneration(&mut renderer)
            }));
            let branch = actual.map_or(2, u32::from);
            assert_eq!(
                u64::from(branch),
                case["branch"].as_u64().unwrap(),
                "{case}"
            );
            assert_eq!(
                u64::from(fragment.generated_at),
                case["after_fragment_stamp"].as_u64().unwrap()
            );
            assert_eq!(
                u64::from(fragment.validated_at),
                case["after_validated_frame"].as_u64().unwrap()
            );
            assert_eq!(
                serde_json::json!(renderer.usage_fixture()),
                case["after_usage"]
            );
        }
        // Editor redraw is a separate original regeneration trigger, even at frame zero.
        renderer.set_cache_fixture(0, 0, &[]);
        let mut fragment = TextFragment::new(20_f32, FontFamily::Label, table);
        fragment.editor.set_redraw(true);
        assert!(fragment.needs_regeneration(&mut renderer));
        assert_eq!(std::mem::size_of::<AtlasAllocUsage>(), 8);
    }
    #[test]
    fn render_reuses_without_implicit_prepare_or_layout_changes() {
        let mut renderer = TextRenderer::new();
        renderer.set_cache_fixture(2, 1, &[]);
        let mut fragment = TextFragment::new(20_f32, FontFamily::Label, table());
        fragment.editor.set_redraw(false);
        fragment.generated_at = u32::MAX;
        fragment.validated_at = u32::MAX;
        fragment.editor_action(Action::Insert('X'));
        fragment
            .primitives
            .push(Primitive::rect([0.; 2], [1.; 2], u32::MAX));
        fragment.set_align(Align::Center);
        fragment.render(&mut renderer, [100.; 2], [50.; 2], u32::MAX);
        assert_eq!(fragment.pending_actions(), 1);
        assert_eq!(fragment.editor.with_buffer(|b| b.size()), (0., 0.));
        assert!(!fragment.editor.redraw());
        assert_eq!(fragment.generated_at, u32::MAX);
        assert_eq!(fragment.validated_at, u32::MAX);
        assert_eq!(renderer.consume().primitives.len(), 1);
    }

    #[test]
    fn generation_preserves_redraw_and_the_previous_validation_frame() {
        let mut renderer = TextRenderer::new();
        let font = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .find(|p| p.join("ljxsj_92385").exists())
            .unwrap()
            .join("ljxsj_92385/Tiny Glade/assets/fonts/ubuntu/Ubuntu-Regular.ttf");
        renderer.load_font_from_file(font).unwrap();
        renderer.set_cache_fixture(5, 3, &[]);
        let table = Arc::new(FontTable {
            fonts: [(
                FontFamily::Label,
                FontTableEntry {
                    family: "Ubuntu".into(),
                    scale: 1.,
                },
            )]
            .into_iter()
            .collect(),
        });
        let mut fragment = TextFragment::new(20_f32, FontFamily::Label, table);
        fragment.set_text(&mut renderer, "glyph");
        fragment.prepare(&mut renderer);
        fragment.validated_at = u32::MAX;
        fragment.render(&mut renderer, [0.; 2], [100.; 2], u32::MAX);
        assert!(!fragment.primitives.is_empty());
        assert_eq!(fragment.generated_at, 3);
        assert_eq!(fragment.validated_at, u32::MAX);
        assert!(fragment.editor.redraw());
    }
    #[test]
    fn failing_metric_rebuild_keeps_original_mutation_order() {
        let mut table = FontTable::default();
        table.fonts.insert(
            FontFamily::Label,
            FontTableEntry {
                family: "Before".into(),
                scale: 1.,
            },
        );
        table.fonts.insert(
            FontFamily::MenuButton,
            FontTableEntry {
                family: "After".into(),
                scale: 0.,
            },
        );
        let mut fragment = TextFragment::new(20_f32, FontFamily::Label, Arc::new(table));
        let attrs = AttrsList::new(fragment.attrs.as_attrs());
        fragment.editor.with_buffer_mut(|buffer| {
            buffer.lines.push(cosmic_text::BufferLine::new(
                "before",
                attrs,
                Shaping::Advanced,
            ))
        });
        fragment.editor_action(Action::Insert('X'));
        let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            fragment.set_family(FontFamily::MenuButton)
        }));
        assert!(failed.is_err());
        assert_eq!(fragment.family, FontFamily::Label);
        assert_eq!(fragment.metrics, FontMetrics::from(20_f32));
        assert_eq!(
            fragment.attrs.family_owned,
            FamilyOwned::Name("After".into())
        );
        assert_eq!(fragment.text(), "");
        assert_eq!(fragment.actions.len(), 1);
        assert_eq!(Arc::strong_count(&fragment.font_table), 1);
    }
    #[test]
    fn reused_raw_alloc_id_zero_matches_original_validation() {
        let evidence: serde_json::Value = serde_json::from_str(include_str!(
            "../tests/fixtures/internal-native-same-id.json"
        ))
        .unwrap();
        let mut renderer = TextRenderer::new();
        for case in evidence["validation_cases"].as_array().unwrap() {
            let usage: Vec<[u32; 2]> =
                serde_json::from_value(case["usage_before"].clone()).unwrap();
            renderer.set_cache_fixture(case["frame"].as_u64().unwrap() as u32, 1, &usage);
            let mut fragment = TextFragment::new(20_f32, FontFamily::Label, table());
            fragment.editor.set_redraw(false);
            fragment.generated_at = 1;
            fragment.glyphs = case["ids"]
                .as_array()
                .unwrap()
                .iter()
                .map(|id| Glyph {
                    allocation: etagere::AllocId::deserialize(id.as_u64().unwrap() as u32),
                    atlas: [0; 2],
                    size: [1; 2],
                    bearing: [0; 2],
                })
                .collect();
            let regenerate = fragment.needs_regeneration(&mut renderer);
            assert_eq!(u64::from(regenerate), case["branch"].as_u64().unwrap());
            assert_eq!(
                u64::from(fragment.generated_at),
                case["fragment_stamp"].as_u64().unwrap()
            );
            assert_eq!(
                u64::from(fragment.validated_at),
                case["validated_at"].as_u64().unwrap()
            );
            assert_eq!(
                serde_json::json!(renderer.usage_fixture()),
                case["usage_after"]
            );
        }
    }
}
