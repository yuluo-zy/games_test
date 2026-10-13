//! 原 Cosmic Editor 上的游戏文本片段策略；不实现整形、字体光栅或 Unicode 分割。
use crate::renderer::{Glyph, Primitive, TextRenderer};
use cosmic_text::{
    Action, Align, Attrs, AttrsList, Buffer, Cursor, Edit, Editor, Family, Metrics, Selection,
    Shaping, Weight,
};
use unicode_segmentation::UnicodeSegmentation;

#[derive(bevy_ecs::prelude::Component)]
pub struct TextFragment {
    editor: Editor<'static>,
    actions: Vec<Action>,
    metrics: Metrics,
    font_scale: f32,
    family: String,
    weight: Weight,
    max_width: f32,
    align: Option<Align>,
    bounds: [f32; 2],
    primitives: Vec<Primitive>,
    glyphs: Vec<Glyph>,
    prepared_scale: f32,
    generation: u64,
    cursor_visible: bool,
}
impl TextFragment {
    pub fn new(renderer: &mut TextRenderer, font_size: f32, line_height: f32) -> Self {
        let metrics = Metrics::new(font_size, line_height);
        let mut buffer = Buffer::new(&mut renderer.font_system, metrics);
        buffer.set_size(&mut renderer.font_system, f32::MAX, f32::MAX);
        Self {
            editor: Editor::new(buffer),
            actions: Vec::new(),
            metrics,
            font_scale: 1.,
            family: String::new(),
            weight: Weight::NORMAL,
            max_width: f32::MAX,
            align: None,
            bounds: [0., line_height],
            primitives: Vec::new(),
            glyphs: Vec::new(),
            prepared_scale: renderer.pixels_per_point(),
            generation: 0,
            cursor_visible: false,
        }
    }
    pub fn with_font_table(
        renderer: &mut TextRenderer,
        table: &crate::FontTable,
        role: crate::FontFamily,
        font_size: f32,
        line_height: f32,
    ) -> Self {
        let entry = table.get(role).expect("font role missing from font table");
        let mut fragment = Self::new(renderer, font_size, line_height);
        fragment.font_scale = entry.scale;
        fragment.set_family(entry.family.clone());
        fragment.set_internal_scaled_metrics(
            renderer,
            fragment.metrics.font_size * fragment.font_scale,
            fragment.metrics.line_height * fragment.font_scale,
        );
        fragment
    }
    pub fn set_font_role(
        &mut self,
        renderer: &mut TextRenderer,
        table: &crate::FontTable,
        role: crate::FontFamily,
    ) {
        let entry = table.get(role).expect("font role missing from font table");
        self.font_scale = entry.scale;
        self.set_family(entry.family.clone());
        let scaled_font_size = self.metrics.font_size * self.font_scale;
        let scaled_line_height = self.metrics.line_height * self.font_scale;
        self.set_internal_scaled_metrics(renderer, scaled_font_size, scaled_line_height);
    }
    pub fn editor(&self) -> &Editor<'static> {
        &self.editor
    }
    pub fn editor_mut(&mut self) -> &mut Editor<'static> {
        self.generation = 0;
        &mut self.editor
    }
    /// 原 0x140b906d0：drop 旧 editor 后装入新 editor，返回 void（丢弃旧值）。
    pub fn replace_editor(&mut self, editor: Editor<'static>) {
        self.generation = 0;
        self.editor = editor;
    }
    pub fn take_editor(&mut self, renderer: &mut TextRenderer) -> Editor<'static> {
        let mut buffer = Buffer::new(&mut renderer.font_system, self.metrics);
        buffer.set_size(&mut renderer.font_system, self.max_width, f32::MAX);
        self.generation = 0;
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
        let family = self.family.clone();
        let attrs = Attrs::new()
            .family(if family.is_empty() {
                Family::SansSerif
            } else {
                Family::Name(&family)
            })
            .weight(self.weight);
        if changed {
            self.editor.with_buffer_mut(|b| {
                b.set_text(
                    &mut renderer.font_system,
                    text.as_ref(),
                    attrs,
                    Shaping::Advanced,
                )
            });
            self.generation = 0;
        }
        self.editor.set_cursor(Cursor::default());
        self.editor.set_selection(Selection::None);
        self.editor_action(Action::Motion(cosmic_text::Motion::BufferEnd));
    }
    fn attrs(&self) -> Attrs<'_> {
        Attrs::new()
            .family(if self.family.is_empty() {
                Family::SansSerif
            } else {
                Family::Name(&self.family)
            })
            .weight(self.weight)
    }
    pub fn apply_buffer_attrs(&mut self) {
        let attrs = AttrsList::new(self.attrs());
        self.editor.with_buffer_mut(|b| {
            for line in &mut b.lines {
                line.set_attrs_list(attrs.clone());
            }
            b.set_redraw(true);
        });
    }
    pub fn set_family(&mut self, family: impl Into<String>) {
        let family = family.into();
        if self.family != family {
            self.family = family;
            self.apply_buffer_attrs();
        }
    }
    pub fn set_weight(&mut self, weight: Weight) {
        if self.weight != weight {
            self.weight = weight;
            self.apply_buffer_attrs();
        }
    }
    pub fn set_metrics(&mut self, renderer: &mut TextRenderer, font_size: f32, line_height: f32) {
        let metrics = Metrics::new(font_size, line_height);
        if self.metrics != metrics {
            self.metrics = metrics;
            let scaled_font_size = self.metrics.font_size * self.font_scale;
            let scaled_line_height = self.metrics.line_height * self.font_scale;
            self.set_internal_scaled_metrics(renderer, scaled_font_size, scaled_line_height);
        }
    }
    pub fn set_internal_scaled_metrics(
        &mut self,
        renderer: &mut TextRenderer,
        font_size: f32,
        line_height: f32,
    ) {
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
        self.editor.with_buffer_mut(|b| {
            b.set_metrics(&mut renderer.font_system, Metrics::new(font_size, line_height));
        });
        self.editor.set_redraw(true);
        self.generation = 0;
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
        // bounds[1] 直取当前字体 metrics 的 scaled line_height（fVar1 + 0.0，归零 -0.0），
        // 不用 layout_runs 折叠的最大高度。role 0 对应内联 scaled metrics，
        // role 1/其他原直读 font table 条目的 line_height（候选 FontTableEntry 无该字段，见缺口登记）。
        let width = self.editor.with_buffer(|b| {
            b.layout_runs().fold(0_f32, |acc, run| acc.max(run.line_w))
        });
        self.bounds = [width, (self.metrics.line_height * self.font_scale) + 0.0];
        if self.prepared_scale != renderer.pixels_per_point() {
            self.prepared_scale = renderer.pixels_per_point();
            self.generation = 0;
        }
    }
    pub fn generate_primitives(&mut self, renderer: &mut TextRenderer) {
        if !self.editor.redraw()
            && self.generation == renderer.atlas().generation
            && renderer.validate_glyph_cache_allocs(&self.glyphs)
        {
            return;
        }
        self.primitives.clear();
        self.glyphs.clear();
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
            b.layout_runs()
                .fold(b.metrics().line_height, |height, run| {
                    height.max(run.line_top + b.metrics().line_height)
                })
        });
        self.generation = renderer.atlas().generation;
        self.editor.set_redraw(false);
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
        self.editor.with_buffer_mut(|b| {
            for line in &mut b.lines {
                line.set_align(align);
            }
            b.set_size(&mut renderer.font_system, extent[0], extent[1]);
        });
        self.editor.shape_as_needed(&mut renderer.font_system, true);
    }
    /// 原 render 包装：调用方提供居中位置与布局区域；Y anchor使用排版自身高度。
    pub fn render_in_box(
        &mut self,
        renderer: &mut TextRenderer,
        center: [f32; 2],
        extent: [f32; 2],
        color: u32,
    ) {
        self.prepare(renderer);
        if self.align.is_some() {
            self.layout_for_render(renderer, extent, self.align);
        }
        self.generate_primitives(renderer);
        renderer.append_primitives(
            &self.primitives,
            [
                center[0] - extent[0] * 0.5,
                center[1] - self.bounds[1] * 0.5,
            ],
            color,
        );
    }
    /// 独立左上角消费者适配；无界宽度先收敛到实际内容宽度，避免RTL坐标溢出。
    pub fn render(&mut self, renderer: &mut TextRenderer, position: [f32; 2], color: u32) {
        self.prepare(renderer);
        self.layout_for_render(
            renderer,
            [self.bounds[0].max(1.), f32::MAX],
            self.align.or(Some(Align::Left)),
        );
        self.generate_primitives(renderer);
        renderer.append_primitives(&self.primitives, position, color);
    }
}
