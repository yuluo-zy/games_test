//! B's shared load/LUT thumbnail scroll content. The original 0x78-byte owner,
//! vtable142baf4f0 and all four methods are bound in round12/render evidence.
use crate::mei::{
    core::{
        GlobalId, MeiInputEvent, NodeContent, NodeContext, NodeDrawContext, NodePrepareContext,
        calculate_hash,
    },
    layout::LengthPercentage,
    widgets::DefaultNodeContent,
};
use country_core::input::action_set::ActionSetAction;
use country_core::resources::gui::LoadMenuThumbnailMetrics;
use country_core::resources::time_of_day::time_continuous_lerp_mut;
use glam::Vec2;
use ::utils::scalar::{max_f32, min_f32};
use std::sync::Arc;

// Use the existing core re-export of parking_lot; toolbar's canonical owner
// arrives as the same Arc type, with no duplicate lock/schema implementation.
type Metrics = Arc<country_core::resources::gui::ThumbnailMetricsMutex>;

pub struct ThumbnailScrollContent {
    pub hovered: Option<usize>,
    pub clicked: Option<usize>,
    pub keyboard_selected: Option<usize>,
    pub scroll_limits: Option<f32>,
    pub metrics: Metrics,
    pub count: usize,
    pub hovered_position: Vec2,
    pub cell_fraction: f32,
    pub gap_fraction: f32,
    pub cells_per_line: u32,
    pub scroll: f32,
    pub velocity: f32,
    pub previous_cursor: f32,
    pub drag_distance: f32,
    pub units_to_cells: f32,
    pub navigation_speed: f32,
    pub column_major: bool,
    pub initialized: bool,
    pub input_disabled: bool,
    pub navigation_boundary: bool,
}
impl DefaultNodeContent for ThumbnailScrollContent {
    fn from_font_table(_: Arc<tg_text::FontTable>) -> Self {
        Self {
            hovered: None,
            clicked: None,
            keyboard_selected: None,
            scroll_limits: None,
            metrics: Arc::default(),
            count: 0,
            hovered_position: Vec2::ZERO,
            cell_fraction: 0.75,
            gap_fraction: 0.1125,
            cells_per_line: 1,
            scroll: 0.,
            velocity: 0.,
            previous_cursor: 0.,
            drag_distance: 0.,
            units_to_cells: 0.,
            navigation_speed: 1.,
            column_major: true,
            initialized: false,
            input_disabled: false,
            navigation_boundary: false,
        }
    }
}

/// O inline (source line 165) `run_count`: number of thumbnail lines for
/// `count` cells at `cells_per_line`, rounded up, never zero (the native
/// divisor path asserts `cells_per_line != 0` before this helper is used).
pub fn run_count(count: usize, cells_per_line: u32) -> u32 {
    let count = count as u32;
    (count / cells_per_line + u32::from(count % cells_per_line != 0)).max(1)
}

/// O inline (source line 191) `min_scroll_pos`: the first cell offset that
/// keeps the grid centered in the primary axis (ANALYSIS C6 scroll math).
pub fn min_scroll_pos(cell: f32, gap: f32, primary_extent: f32, lines: u32) -> f32 {
    let content = lines as f32 * (cell + gap) - gap;
    let visible = primary_extent.min(content);
    let pad = (primary_extent - visible) * -0.5 - gap;
    (pad + primary_extent * 0.5) / cell - 0.5
}

/// O inline (source line 191) upper bound paired with `min_scroll_pos`.
pub fn max_scroll_pos(cell: f32, gap: f32, primary_extent: f32, lines: u32) -> f32 {
    let content = lines as f32 * (cell + gap) - gap;
    let visible = primary_extent.min(content);
    let pad = (primary_extent - visible) * -0.5 - gap;
    (content + pad - visible + (gap + gap) + primary_extent * 0.5) / cell - 0.5
}

/// O inline (source line 197) `nav_scroll_speed`: keyboard navigation lerp
/// speed, inversely proportional to the remaining distance.
pub fn nav_scroll_speed(distance: f32) -> f32 {
    15. / min_f32(1., max_f32(0.01, distance))
}

/// O inline (source line 133) `hovered_item_pos`: the hovered thumbnail
/// index published by `update_metrics` for contextual buttons.
pub fn hovered_item_pos(content: &ThumbnailScrollContent) -> Option<usize> {
    content.hovered
}

/// O 0x141e23b90 `display_save_file_load_error`: error template with a title
/// slot and a detail slot. Native builds a `template_with_id` and both
/// `with_slot` closures; the safe owner keeps that shape and takes the strings
/// the caller already produced (template id is the one used by the load UI).
pub fn display_save_file_load_error(
    parent: &NodeContext,
    title: &str,
    detail: &str,
) -> NodeContext {
    let view = parent.template("menu/backup_load_error");
    view.with_slot("header", |slot| {
        let node = slot.label(title.to_owned());
        let content = node.content_mut::<crate::mei::widgets::LabelContent>();
        content.color = u32::MAX;
        content
            .text
            .set_metrics(48.)
            .expect("save load error header metrics");
    });
    view.with_slot("error", |slot| {
        let node = slot.label(detail.to_owned());
        let content = node.content_mut::<crate::mei::widgets::LabelContent>();
        content.color = 0xffffffe0;
        content
            .text
            .set_metrics(28.)
            .expect("save load error detail metrics");
        content
            .text
            .set_max_width(700.)
            .expect("save load error max width");
        content.text.set_align(tg_text::Align::Left);
    });
    view.context
}

/// O 0x141e23d20 `show_contextual_buttons`: build the hovered-session action
/// strip (delete / switch glade / view backups) after the native iced Event
/// request `{x, y+250, 0, -1}` sized 200×30. `index` is the 1-based thumbnail
/// index; `boundary` is the one-row navigation handoff from the scroll view.
pub fn show_contextual_buttons(
    mei: &mut crate::mei::core::MeiContext,
    position: Vec2,
    index: usize,
    state: &mut country_core::resources::gui::LoadMenuState,
    sessions: &[country_core::resources::session::SavedSession],
    boundary: bool,
) {
    let root = mei.new_at_pos(
        "delete",
        position + Vec2::new(0., 250.),
        Some(Vec2::new(200., 30.)),
        Vec2::new(0., -1.),
        false,
    );
    let hovered = root.node().state_bits & 1 != 0;
    let view = root.template("other/hint_text_centered");
    view.with_slot("text", |slot| {
        let size = if hovered { 65. } else { 60. };
        let id = sessions[index - 1].id;
        if index != 1 {
            let b = slot.pill_button("delete_button", "menu/delete");
            let c = b.content_mut::<crate::mei::widgets::PillButtonContent>();
            c.strength = 0.6;
            b.node_mut().flags |= 0x10;
            // Native bit8 requests default focus. It does NOT disable input.
            if boundary {
                b.node_mut().flags |= 0x18;
            }
            b.node_mut().style.size = crate::mei::layout::LengthPercentage::absolute(size);
            b.node_mut().style.min = crate::mei::layout::LengthPercentage::absolute(size);
            if b.node().used() {
                state.warning_session = Some(id);
            }
        }
        let b = slot.pill_button("switch_glade", "menu/switch_glade");
        b.content_mut::<crate::mei::widgets::PillButtonContent>().strength = 0.6;
        b.node_mut().flags |= 0x10;
        if index == 1 && boundary {
            b.node_mut().flags |= 0x18;
        }
        b.node_mut().style.size = crate::mei::layout::LengthPercentage::absolute(size);
        b.node_mut().style.min = crate::mei::layout::LengthPercentage::absolute(size);
        if b.node().used() {
            state.switch_glade_type = Some(id);
        }
        let b = slot.pill_button("view_backups", "menu/glade_backups");
        b.content_mut::<crate::mei::widgets::PillButtonContent>().strength = 0.6;
        b.node_mut().flags |= 0x10;
        b.node_mut().style.size = crate::mei::layout::LengthPercentage::absolute(size);
        b.node_mut().style.min = crate::mei::layout::LengthPercentage::absolute(size);
        if b.node().used() {
            state.backups = Some(country_core::resources::gui::LoadMenuBackupState {
                session: id,
                page: 0,
            });
        }
    });
}

#[cfg(test)]
mod scroll_helper_tests {
    use super::*;

    #[test]
    fn hovered_item_pos_returns_the_published_index() {
        let mut content = <ThumbnailScrollContent as crate::mei::widgets::DefaultNodeContent>::from_font_table(
            std::sync::Arc::new(tg_text::FontTable::default()),
        );
        assert_eq!(hovered_item_pos(&content), None);
        content.hovered = Some(3);
        content.hovered_position = Vec2::new(10., 20.);
        assert_eq!(hovered_item_pos(&content), Some(3));
        assert_eq!(content.hovered_position, Vec2::new(10., 20.));
    }

    #[test]
    fn run_count_rounds_up_and_never_returns_zero() {
        assert_eq!(run_count(0, 3), 1);
        assert_eq!(run_count(3, 3), 1);
        assert_eq!(run_count(4, 3), 2);
        assert_eq!(run_count(9, 3), 3);
    }

    #[test]
    fn scroll_bounds_center_short_content() {
        // One line shorter than the viewport: the upper bound leaves one cell
        // of centering slack and the lower bound starts centered.
        let min = min_scroll_pos(100., 10., 200., 1);
        let max = max_scroll_pos(100., 10., 200., 1);
        assert!((min + 0.1).abs() < 1e-6);
        assert!((max - 0.1).abs() < 1e-6);
    }

    #[test]
    fn nav_speed_uses_native_reciprocal_clamp() {
        assert_eq!(nav_scroll_speed(0.005), 1500.);
        assert_eq!(nav_scroll_speed(0.5), 30.);
        assert_eq!(nav_scroll_speed(2.0), 15.);
    }
}

impl ThumbnailScrollContent {
    /// Native 141e1fcf0, including analog/mouse mode reset, confirm, all four
    /// navigation directions and the boundary handoff when there is one row.
    fn input(&mut self, action: &ActionSetAction, phase: [bool; 2]) -> bool {
        if !phase[0] {
            return false;
        }
        let analog = action.as_analog(0).unwrap_or(Vec2::ZERO);
        let mouse = analog.length() > 1. || action.just_pressed(0);
        let use_hovered;
        if mouse {
            self.keyboard_selected = None;
            use_hovered = true;
        } else if let Some(index) = self.keyboard_selected {
            if action.just_pressed(0x13) {
                self.clicked = Some(index);
            }
            use_hovered = false;
        } else {
            use_hovered = true;
        }
        let step = if action.just_pressed(0x14) {
            (self.cells_per_line as i32).wrapping_neg()
        } else if action.just_pressed(0x15) {
            self.cells_per_line as i32
        } else if action.just_pressed(0x17) {
            if self.cells_per_line <= 1 {
                self.navigation_boundary = true;
                return false;
            }
            -1
        } else if action.just_pressed(0x16) {
            if self.cells_per_line <= 1 {
                self.navigation_boundary = true;
                return false;
            }
            1
        } else {
            return false;
        };
        if step == 0 {
            return false;
        }
        let base = if use_hovered {
            let index = self.hovered.unwrap_or(0);
            self.keyboard_selected = Some(index);
            index as i32
        } else {
            self.keyboard_selected.unwrap_or(0) as i32
        };
        let candidate = base.wrapping_add(step);
        if candidate >= 0 && candidate < self.count as i32 {
            self.keyboard_selected = Some(candidate as u32 as usize);
        }
        true
    }

    /// Native 141e1f4c0. All values use MEI units (window height = 1000), and
    /// the published cell coordinates are divided by 1000 on both axes.
    fn update_metrics(
        &mut self,
        center: Vec2,
        extent: Vec2,
        cursor: Vec2,
        delta: f32,
        pixels_per_point: f32,
        state: u8,
    ) {
        let cell = self.cell_fraction * extent.y;
        let gap = self.gap_fraction * extent.y;
        if !self.input_disabled {
            let primary_extent = if self.column_major {
                extent.x
            } else {
                extent.y
            };
            let primary_cursor = if self.column_major {
                cursor.x
            } else {
                cursor.y
            };
            if state & 2 != 0 {
                self.previous_cursor = primary_cursor;
                self.drag_distance = 0.;
            }
            self.hovered = None;
            self.clicked = None;
            if state & 4 == 0 {
                self.scroll += self.velocity * delta;
                time_continuous_lerp_mut(&mut self.velocity, 0., delta, 2.);
            } else {
                let movement = self.previous_cursor - primary_cursor;
                self.drag_distance += movement.abs() * pixels_per_point;
                let displacement = movement * self.units_to_cells;
                self.scroll += displacement;
                self.velocity = displacement / max_f32(delta, 0.00001);
                self.previous_cursor = primary_cursor;
            }
            assert_ne!(self.cells_per_line, 0, "thumbnail cells_per_line divisor");
            let (maximum, minimum) = if let Some(minimum) = self.scroll_limits {
                (
                    (self.gap_fraction / self.cell_fraction + 1.)
                        * self.count.saturating_sub(1) as f32,
                    minimum,
                )
            } else {
                let count = self.count;
                let lines = run_count(count, self.cells_per_line);
                let maximum = max_scroll_pos(cell, gap, primary_extent, lines);
                let minimum = min_scroll_pos(cell, gap, primary_extent, lines);
                if !self.initialized {
                    self.scroll += minimum;
                }
                (maximum, minimum)
            };
            if state & 4 == 0 {
                if minimum > self.scroll {
                    let speed = nav_scroll_speed(minimum - self.scroll);
                    time_continuous_lerp_mut(&mut self.scroll, minimum, delta, speed);
                }
                if self.scroll > maximum {
                    let speed = nav_scroll_speed(self.scroll - maximum);
                    time_continuous_lerp_mut(&mut self.scroll, maximum, delta, speed);
                }
            }
            if let Some(index) = self.keyboard_selected {
                self.hovered = Some(index);
                let target = (self.gap_fraction / self.cell_fraction + 1.)
                    * (index / self.cells_per_line as usize) as f32;
                let target = target.max(minimum).min(maximum);
                time_continuous_lerp_mut(&mut self.scroll, target, delta, self.navigation_speed);
            }
        }
        self.units_to_cells = 1. / max_f32(cell, 0.00001);
        let gap_ratio = self.gap_fraction / self.cell_fraction + 1.;
        let half_line = self.cells_per_line.wrapping_sub(1) as f32 * 0.5;
        let (offset, scroll_offset) = if self.column_major {
            (
                Vec2::new(self.scroll, half_line),
                Vec2::new(self.scroll / gap_ratio, 0.),
            )
        } else {
            (
                Vec2::new(half_line, self.scroll),
                Vec2::new(0., self.scroll / gap_ratio),
            )
        };
        let mut metrics = LoadMenuThumbnailMetrics {
            highlighted_index: None,
            origin: (center - cell * offset) / 1000.,
            cell_size: Vec2::splat(cell / 1000.),
            step: Vec2::splat((cell + gap) / 1000.),
            scroll_offset,
            cell_count: self.count as u32,
            cells_per_line: self.cells_per_line,
            column_major: self.column_major,
        };
        let radius = (cell / 1000. * 1000.) * 0.5;
        for (index, position) in metrics.positions().enumerate() {
            let position = position * 1000.;
            let inside = (cursor.cmpge(position - radius) & cursor.cmple(position + radius)).all();
            let selected = self.keyboard_selected == Some(index);
            let highlight = (self.keyboard_selected.is_none() && inside) || selected;
            if state & 8 == 0 || !(self.drag_distance < 2.) {
                if highlight {
                    self.hovered = Some(index);
                    self.hovered_position = position;
                }
            } else {
                if highlight {
                    self.hovered = Some(index);
                    self.hovered_position = position;
                }
                if inside {
                    self.clicked = Some(index);
                }
            }
        }
        metrics.highlighted_index = self.hovered.map(|index| index as u32);
        *self.metrics.lock() = metrics;
        self.initialized = true;
    }
}
impl NodeContent for ThumbnailScrollContent {
    fn type_name(&self) -> &'static str {
        "toolbar::menu::load::ThumbnailScrollContent"
    }
    fn interactive(&self) -> bool {
        true
    }
    fn process_input_action(&mut self, event: &MeiInputEvent) -> bool {
        if event.tag() == 0xc3 && !self.input_disabled {
            self.scroll += event.payload_vec2().y * -0.03;
        }
        false
    }
    fn on_input_phase(&mut self, action: &ActionSetAction, phase: [bool; 2]) -> bool {
        self.input(action, phase)
    }
    fn prepare_layout(&mut self, _: &mut NodePrepareContext<'_>) -> Option<[LengthPercentage; 2]> {
        self.navigation_boundary = false;
        Some([LengthPercentage::absolute(0.); 2])
    }
    fn draw(&mut self, ctx: &mut NodeDrawContext<'_>) {
        self.update_metrics(
            ctx.center,
            ctx.extent,
            ctx.cursor,
            ctx.elapsed,
            ctx.pixels_per_point,
            ctx.state,
        );
    }
}
impl NodeContext {
    /// Native 140fa06a0: same node identity, shared metrics Arc, navigation and
    /// exact zero/default style reset. Callers then set their own geometry.
    pub fn thumbnail_scroll(&self, name: &str, count: usize, metrics: Metrics) -> Self {
        let value = calculate_hash(&(name, ()));
        // TypeId::hash writes its high qword (original1405bdf7b).
        let local = calculate_hash(&(value, 0xae5fba5e3bb8dddau64));
        let child = self.child_with_global_id(GlobalId(calculate_hash(&(self.id.0, local))));
        let content = child.get_content_mut_impl::<ThumbnailScrollContent>(false);
        content.count = count;
        content.metrics = metrics;
        content.column_major = true;
        content.cells_per_line = 1;
        child.node_mut().flags |= 0x40;
        child.layout(crate::mei::layout::LayoutStyle {
            expand: 1.,
            ..Default::default()
        });
        child
    }
}
