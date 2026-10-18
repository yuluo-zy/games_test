//! Editable text, layout invalidation, and renderer-specific glyph primitives.
use crate::renderer::{Glyph, Primitive, TextRenderer};
use crate::{FontFamily, FontMetrics, FontTable, TextError, TextResult};
use cosmic_text::{
    Action, Align, Attrs, AttrsList, AttrsOwned, Buffer, Cursor, Edit, Editor, Family, FamilyOwned,
    Metrics, Selection, Shaping, Weight,
};
use std::sync::Arc;
use unicode_segmentation::UnicodeSegmentation;

// Renderer colors use RRGGBBAA, with alpha in the least significant byte.
const SELECTION_COLOR: u32 = 0xffff_ff80;

/// Editable text with shared role configuration and renderer-bound layout caches.
/// Mutations invalidate cached primitives; prepare/render apply queued actions.
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
    // Primitive validity depends on renderer identity and atlas generation.
    cache_token: Option<(u64, u64)>,
    // Shaping may precede primitive generation, so font IDs need a separate owner token.
    shaping_token: Option<(u64, u64)>,
    last_extent: Option<[f32; 2]>,
    dirty: bool,
    cursor_visible: bool,
}

impl TextFragment {
    /// Construct a fragment using a validated role from the shared font table.
    /// Font bytes are loaded separately into TextRenderer. Missing roles and
    /// invalid metrics/scales return an error before constructing the editor.
    pub fn new(
        metrics: impl Into<FontMetrics>,
        family: FontFamily,
        font_table: Arc<FontTable>,
    ) -> TextResult<Self> {
        let metrics = metrics.into();
        metrics.validate()?;
        let entry = font_table.require(family)?;
        let scaled = entry.scaled(metrics.font_size, metrics.line_height)?;
        let buffer = Buffer::new_empty(Metrics::new(scaled.font_size, scaled.line_height));
        let attrs = AttrsOwned::new(Attrs::new().family(Family::Name(scaled.family)));
        Ok(Self {
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
            cache_token: None,
            shaping_token: None,
            last_extent: None,
            dirty: true,
            cursor_visible: false,
        })
    }
    /// Borrow the editor without changing layout or glyph-cache validity.
    pub fn editor(&self) -> &Editor<'static> {
        &self.editor
    }
    /// Access the editor directly, discarding generated primitives and shaping identity.
    /// Callers must keep cursor/selection positions valid for the buffer's UTF-8 text.
    pub fn editor_mut(&mut self) -> &mut Editor<'static> {
        self.shaping_token = None;
        self.invalidate();
        &mut self.editor
    }
    /// Replace the editor and discard actions queued for its predecessor.
    /// Invalid metrics return an error before the existing editor is changed.
    pub fn replace_editor(&mut self, editor: Editor<'static>) -> TextResult<()> {
        let metrics = editor.with_buffer(|buffer| buffer.metrics());
        FontMetrics::new(metrics.font_size, metrics.line_height).validate()?;
        self.actions.clear();
        self.invalidate();
        self.editor = editor;
        self.shaping_token = None;
        Ok(())
    }
    /// Move the editor out, leaving an empty editor with its scaled metrics.
    /// Pending actions and generated primitives are discarded.
    pub fn take_editor(&mut self) -> Editor<'static> {
        self.actions.clear();
        self.invalidate();
        let metrics = self.editor.with_buffer(|buffer| buffer.metrics());
        let buffer = Buffer::new_empty(metrics);
        std::mem::replace(&mut self.editor, Editor::new(buffer))
    }
    /// Collect the current buffer's logical lines, separated by newline characters.
    /// Queued actions become visible only after prepare/render.
    pub fn text(&self) -> String {
        self.editor.with_buffer(|buffer| {
            buffer
                .lines
                .iter()
                .map(|line| line.text())
                .collect::<Vec<_>>()
                .join("\n")
        })
    }
    /// Replace text only when its lines differ, then queue a move to buffer end.
    /// Clears the current selection. A renderer without loaded fonts returns an error.
    pub fn set_text(
        &mut self,
        renderer: &mut TextRenderer,
        text: impl AsRef<str>,
    ) -> TextResult<()> {
        if renderer.font_faces().next().is_none() {
            return Err(TextError::InvalidInput(
                "text renderer has no loaded fonts".into(),
            ));
        }
        self.sync_renderer(renderer);
        let text = text.as_ref();
        let changed = !self.editor.with_buffer(|buffer| {
            let mut incoming = text.split('\n');
            buffer
                .lines
                .iter()
                .all(|line| incoming.next() == Some(line.text()))
                && incoming.next().is_none()
        });
        let attrs = self.attrs.as_attrs();
        if changed {
            self.editor.with_buffer_mut(|buffer| {
                buffer.set_text(&mut renderer.font_system, text, attrs, Shaping::Advanced)
            });
        }
        self.editor.set_cursor(Cursor::default());
        self.editor.set_selection(Selection::None);
        self.editor_action(Action::Motion(cosmic_text::Motion::BufferEnd));
        Ok(())
    }
    /// Apply the fragment's family and weight to every existing buffer line.
    /// This deliberately replaces per-span attributes and invalidates layout.
    pub fn apply_buffer_attrs(&mut self) {
        self.invalidate();
        let attrs = AttrsList::new(self.attrs.as_attrs());
        self.editor.with_buffer_mut(|buffer| {
            for line in &mut buffer.lines {
                line.set_attrs_list(attrs.clone());
            }
            buffer.set_redraw(true);
        });
    }
    /// Change role, family attributes and scale while retaining unscaled metrics.
    /// Missing roles, invalid scales or scaling overflow leave the fragment unchanged.
    pub fn set_family(&mut self, family: FontFamily) -> TextResult<()> {
        let entry = self.font_table.require(family)?;
        let scaled = entry.scaled(self.metrics.font_size, self.metrics.line_height)?;
        if self.family == family {
            return Ok(());
        }
        let scaled_metrics = FontMetrics::new(scaled.font_size, scaled.line_height);
        self.attrs.family_owned = FamilyOwned::Name(entry.family.clone());
        self.family = family;
        self.apply_buffer_attrs();
        self.apply_scaled_metrics(scaled_metrics);
        Ok(())
    }
    /// Set a uniform weight for all lines and invalidate layout when it changes.
    pub fn set_weight(&mut self, weight: Weight) {
        if self.attrs.weight != weight {
            self.attrs.weight = weight;
            self.apply_buffer_attrs();
        }
    }
    /// Set unscaled metrics using the current role's scale.
    /// Rejects nonpositive/nonfinite values and scaling overflow before mutation.
    pub fn set_metrics(&mut self, metrics: impl Into<FontMetrics>) -> TextResult<()> {
        let metrics = metrics.into();
        metrics.validate()?;
        let entry = self.font_table.require(self.family)?;
        let scaled = entry.scaled(metrics.font_size, metrics.line_height)?;
        self.apply_scaled_metrics(FontMetrics::new(scaled.font_size, scaled.line_height));
        self.metrics = metrics;
        Ok(())
    }
    /// Set already-scaled buffer metrics without changing the stored role or base metrics.
    /// Values must be finite and positive. Preserves text, cursor, selection, scroll,
    /// indentation and tab width; wrapping and available size are reapplied on prepare.
    pub fn set_internal_scaled_metrics(
        &mut self,
        font_size: f32,
        line_height: f32,
    ) -> TextResult<()> {
        let metrics = FontMetrics::new(font_size, line_height);
        metrics.validate()?;
        self.apply_scaled_metrics(metrics);
        Ok(())
    }
    // Public scalar input or FontTableEntry::scaled validates before this commit.
    fn apply_scaled_metrics(&mut self, metrics: FontMetrics) {
        let FontMetrics {
            font_size,
            line_height,
        } = metrics;
        let current = self.editor.with_buffer(|buffer| buffer.metrics());
        if current.font_size != font_size || current.line_height != line_height {
            // Buffer::set_metrics needs a FontSystem; rebuilding only its metrics
            // here keeps this setter independent of renderer ownership.
            let mut lines = self
                .editor
                .with_buffer_mut(|buffer| std::mem::take(&mut buffer.lines));
            for line in &mut lines {
                line.reset_layout();
            }
            let scroll = self.editor.with_buffer(|buffer| buffer.scroll());
            let cursor = self.editor.cursor();
            let selection = self.editor.selection();
            let mut buffer = Buffer::new_empty(Metrics::new(font_size, line_height));
            buffer.lines = lines;
            buffer.set_scroll(scroll);
            let mut editor = Editor::new(buffer);
            editor.set_cursor(cursor);
            editor.set_selection(selection);
            editor.set_auto_indent(self.editor.auto_indent());
            editor.set_tab_width(self.editor.tab_width());
            self.editor = editor;
            self.invalidate();
        }
    }
    /// Set a finite positive wrapping limit; render also respects its available width.
    /// Invalid values leave the previous limit and layout unchanged.
    pub fn set_max_width(&mut self, width: f32) -> TextResult<()> {
        if !width.is_finite() || width <= 0. {
            return Err(TextError::InvalidInput(
                "text width must be finite and positive".into(),
            ));
        }
        if self.max_width != width {
            self.invalidate();
            self.max_width = width;
            self.editor.set_redraw(true);
        }
        Ok(())
    }
    /// Select horizontal alignment and invalidate layout if it changes.
    pub fn set_align(&mut self, align: Align) {
        if self.align != Some(align) {
            self.invalidate();
        }
        self.align = Some(align);
    }
    /// Restore the shaping engine's default horizontal alignment.
    pub fn clear_align(&mut self) {
        if self.align.is_some() {
            self.invalidate();
        }
        self.align = None;
    }
    /// Enable or hide the caret in subsequently generated primitives.
    pub fn set_cursor_visible(&mut self, visible: bool) {
        if self.cursor_visible != visible {
            self.invalidate();
            self.cursor_visible = visible;
            self.editor.set_redraw(true);
        }
    }
    /// Return dimensions of the last successfully prepared or rendered layout.
    /// Mutations do not update this snapshot until prepare/render succeeds.
    pub fn bounds(&self) -> [f32; 2] {
        self.bounds
    }
    /// Queue an editing action for the next prepare/render and invalidate primitives.
    pub fn editor_action(&mut self, action: Action) {
        self.invalidate();
        self.actions.push(action);
    }
    /// Count actions that have not yet been applied to the editor.
    pub fn pending_actions(&self) -> usize {
        self.actions.len()
    }
    /// Apply queued actions, configure wrapping and shape the current text.
    /// Requires loaded fonts and valid buffer metrics; updates bounds without
    /// appending primitives or advancing the renderer's frame.
    pub fn prepare(&mut self, renderer: &mut TextRenderer) -> TextResult<()> {
        if renderer.font_faces().next().is_none() {
            return Err(TextError::InvalidInput(
                "text renderer has no loaded fonts".into(),
            ));
        }
        self.sync_renderer(renderer);
        let metrics = self.editor.with_buffer(|buffer| buffer.metrics());
        FontMetrics::new(metrics.font_size, metrics.line_height).validate()?;
        for action in self.actions.drain(..) {
            self.editor.action(&mut renderer.font_system, action);
        }
        let size = if self.dirty {
            [self.max_width, f32::MAX]
        } else {
            let extent = self.last_extent.unwrap_or([self.max_width, f32::MAX]);
            [extent[0].min(self.max_width), extent[1]]
        };
        self.editor.with_buffer_mut(|buffer| {
            for line in &mut buffer.lines {
                line.set_align(self.align);
            }
            buffer.set_wrap(&mut renderer.font_system, cosmic_text::Wrap::Word);
            buffer.set_size(&mut renderer.font_system, size[0], size[1]);
        });
        self.editor
            .shape_as_needed(&mut renderer.font_system, false);
        self.update_bounds();
        Ok(())
    }
    /// Recompute both dimensions from the same shaped layout after prepare or resize.
    fn update_bounds(&mut self) {
        self.bounds = self.editor.with_buffer(|buffer| {
            let (width, last_line_top) = buffer
                .layout_runs()
                .fold((0_f32, 0_f32), |(width, top), run| {
                    (width.max(run.line_w), top.max(run.line_top))
                });
            [width, buffer.metrics().line_height + last_line_top]
        });
    }

    // Font IDs inside cached shaping are local to a renderer's font database.
    // A fragment can already be shaped before it has any cached primitives.
    fn sync_renderer(&mut self, renderer: &TextRenderer) {
        let token = renderer.cache_token();
        if self.shaping_token != Some(token) {
            self.editor.with_buffer_mut(|buffer| {
                for line in &mut buffer.lines {
                    line.reset_shaping();
                }
            });
            self.invalidate();
            self.shaping_token = Some(token);
        }
    }
    fn invalidate(&mut self) {
        self.dirty = true;
        self.cache_token = None;
        self.primitives.clear();
        self.glyphs.clear();
        self.editor.set_redraw(true);
    }
    fn generate_primitives(
        &mut self,
        renderer: &mut TextRenderer,
        extent: [f32; 2],
    ) -> TextResult<()> {
        if !self.dirty
            && !self.editor.redraw()
            && self.cache_token == Some(renderer.cache_token())
            && self.last_extent == Some(extent)
            && renderer.validate_glyph_cache_allocs(&self.glyphs)
        {
            return Ok(());
        }
        self.dirty = true;
        self.primitives.clear();
        self.glyphs.clear();
        self.layout_for_render(renderer, extent, self.align);
        self.editor.shape_as_needed(&mut renderer.font_system, true);
        let scale = renderer.pixels_per_point();
        let selection = self.editor.selection_bounds();
        let cursor = self.editor.cursor();
        let visible = self.cursor_visible;
        let generated = self.editor.with_buffer(|buffer| -> TextResult<()> {
            for run in buffer.layout_runs() {
                if let Some((x, width)) =
                    selection.and_then(|(start, end)| run.highlight(start, end))
                {
                    self.primitives.push(Primitive::rect(
                        [x, run.line_top],
                        [width, buffer.metrics().line_height],
                        SELECTION_COLOR,
                    ));
                }
                let cursor_in_run = run
                    .glyphs
                    .iter()
                    .any(|glyph| cursor.index >= glyph.start && cursor.index < glyph.end)
                    || (cursor.index == run.text.len()
                        && run
                            .glyphs
                            .last()
                            .is_none_or(|glyph| glyph.end == run.text.len()));
                if visible && cursor.line == run.line_i && cursor_in_run {
                    let mut cursor_x = if (cursor.index == 0) != run.rtl {
                        0.
                    } else {
                        run.line_w
                    };
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
                        [4., buffer.metrics().line_height],
                        u32::MAX,
                    ));
                }
                for glyph in run.glyphs {
                    let physical = glyph.physical((0., 0.), scale);
                    if let Some(cached) = renderer.get_glyph(physical.cache_key)? {
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
                            color: glyph
                                .color_opt
                                .map_or(u32::MAX, crate::color::pack_text_color),
                            stream: 0,
                            padding: [0; 2],
                        });
                    }
                }
            }
            Ok(())
        });
        if let Err(error) = generated {
            self.primitives.clear();
            self.glyphs.clear();
            self.cache_token = None;
            return Err(error);
        }
        self.update_bounds();
        self.cache_token = Some(renderer.cache_token());
        self.last_extent = Some(extent);
        self.dirty = false;
        self.editor.set_redraw(false);
        Ok(())
    }
    /// Borrow the last generated primitives in fragment-local coordinates.
    /// Editing clears this cache; render must regenerate it for the current renderer.
    pub fn primitives(&self) -> &[Primitive] {
        &self.primitives
    }
    fn layout_for_render(
        &mut self,
        renderer: &mut TextRenderer,
        extent: [f32; 2],
        align: Option<Align>,
    ) {
        self.editor.with_buffer_mut(|buffer| {
            for line in &mut buffer.lines {
                line.set_align(align);
            }
            buffer.set_size(
                &mut renderer.font_system,
                extent[0].min(self.max_width),
                extent[1],
            );
        });
    }
    /// Prepare changed text, then append primitives centered within the extent.
    /// Center and extent must be finite, and both extent components positive.
    /// The color is a packed RRGGBBAA tint. Atlas errors require the renderer's
    /// documented frame recovery; a failed generation appends no partial fragment.
    pub fn render(
        &mut self,
        renderer: &mut TextRenderer,
        center: [f32; 2],
        extent: [f32; 2],
        tint: u32,
    ) -> TextResult<()> {
        if center.iter().any(|value| !value.is_finite())
            || extent
                .iter()
                .any(|value| !value.is_finite() || *value <= 0.)
        {
            return Err(TextError::InvalidInput(
                "text geometry must be finite with positive extent".into(),
            ));
        }
        self.sync_renderer(renderer);
        if self.dirty || self.editor.redraw() || !self.actions.is_empty() {
            self.prepare(renderer)?;
        }
        self.generate_primitives(renderer, extent)?;
        renderer.append_primitives(
            &self.primitives,
            [
                center[0] - extent[0] * 0.5,
                center[1] - self.bounds[1] * 0.5,
            ],
            tint,
        )
    }
}
