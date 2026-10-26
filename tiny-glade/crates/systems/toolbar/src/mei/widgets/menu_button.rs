//! Original B MenuButtonContent and NodeContext widget producers.
//! Staged for mei/widgets/menu_button.rs; root owns production integration.
use super::{DefaultNodeContent, LabelContent, PillButtonContent};
use crate::mei::{
    core::{
        GlobalId, MeiInputEvent, NodeContent, NodeContext, NodeDrawContext, NodePrepareContext,
        calculate_hash,
    },
    layout::{Justify, LayoutStyle, LengthPercentage},
};
use glam::Vec2;
use std::sync::Arc;
use tg_text::{FontFamily, FontTable, TextFragment, Weight};

/// B vtable142baf6d0, native0x260B. The pending string is a source adapter:
/// current tg-text needs its actual renderer to set text; prepare applies it
/// before the same owning TextFragment's original layout operation.
pub struct MenuButtonContent {
    pub text: TextFragment,
    pub color: u32,
    /// Native `+0x25c`: forcing white is what `force_highlight` writes; hover
    /// and this flag share the draw branch at 0x14067ea50.
    pub always_white: bool,
    /// Native `+0x25d`: digital action byte. `0x12` means "no hotkey" and is
    /// the branch that skips the tooltip at 0x14067e6d0.
    pub hotkey: u8,
    pub action: u8,
    pub pending_text: Option<String>,
}
impl DefaultNodeContent for MenuButtonContent {
    /// B140f9dcd6..140f9dd19: MenuButton role1,48px,Weight700.
    fn from_font_table(fonts: Arc<FontTable>) -> Self {
        let mut text = TextFragment::new(48., FontFamily::MenuButton, fonts)
            .expect("MEI menu button font role");
        text.set_weight(Weight(700));
        Self {
            text,
            color: 0xffffffa0,
            always_white: false,
            hotkey: 0x12,
            action: 0x28,
            pending_text: None,
        }
    }
}
impl MenuButtonContent {
    pub fn set_text(&mut self, text: String) {
        self.pending_text = Some(text);
    }
}

/// B14067e510 is physically shared by LabelContent and MenuButtonContent.
/// Use one preparation path while preserving each concrete text owner.
pub(super) fn prepare_text(
    text: &mut TextFragment,
    pending: &mut Option<String>,
    ctx: &mut NodePrepareContext<'_>,
) -> Option<[LengthPercentage; 2]> {
    if let Some(value) = pending.take() {
        text.set_text(ctx.text, value).expect("set MEI text");
    }
    text.prepare(ctx.text).expect("prepare MEI text");
    Some(text.bounds().map(LengthPercentage::absolute))
}
impl NodeContent for MenuButtonContent {
    fn type_name(&self) -> &'static str {
        "toolbar::mei::widgets::MenuButtonContent"
    }
    /// Native +40 leaf14003bb70 returns1.
    fn interactive(&self) -> bool {
        true
    }
    /// Native +48 leaf14003bb80 returns without consuming the event.
    fn process_input_action(&mut self, _: &MeiInputEvent) -> bool {
        false
    }
    /// Native +50 leaf140002370 returnsfalse for every action/phase.
    fn on_input(&mut self, _: &country_core::input::action_set::ActionSetAction) -> bool {
        false
    }
    /// Native14067ea20: absent28; otherwise just_pressed ORs bit20.
    fn pre_update(
        &mut self,
        actions: &country_core::input::action_set::MainActionSet,
        _: Vec2,
        state: &mut u8,
    ) {
        if self.action != 0x28 && actions.digital[usize::from(self.action)].just_pressed() {
            *state |= 0x20;
        }
    }
    fn prepare_layout(
        &mut self,
        ctx: &mut NodePrepareContext<'_>,
    ) -> Option<[LengthPercentage; 2]> {
        prepare_text(&mut self.text, &mut self.pending_text, ctx)
    }
    /// Native14067ea50: hover or always_white selectswhite, then scalesRGBA
    /// by opacity using the same packed-color routine as other text consumers.
    fn draw(&mut self, ctx: &mut NodeDrawContext<'_>) {
        let color = if ctx.state & 1 != 0 || self.always_white {
            u32::MAX
        } else {
            self.color
        };
        let tint = tg_text::scale_u32_packed_srgba_color(color, ctx.opacity);
        self.text
            .render(ctx.text, ctx.center.to_array(), ctx.extent.to_array(), tint)
            .expect("render MEI menu button");
    }
}

/// Native hash-value-TypeId1405bdf20 reads only the highu64 at tuple+10.
/// Keep the existing source-level identity contract shared by templates.
fn widget_child(parent: &NodeContext, name: &str, type_high: u64) -> NodeContext {
    let value = calculate_hash(&(name, ()));
    let local = calculate_hash(&(value, type_high));
    parent.child_with_global_id(GlobalId(calculate_hash(&(parent.id.0, local))))
}
impl NodeContext {
    /// 原menu_button_no_nav保留相同内容、声音和身份，仅清除导航标志。
    pub fn menu_button_no_nav(&self, text: &str) -> Self {
        let node = self.menu_button(text);
        node.node_mut().with_nav(false);
        node
    }
    /// O 0x14067e9e0 `MenuButtonView::force_highlight`: writes the
    /// `MenuButtonContent` byte at native `+0x25c` (the `always_white` draw
    /// branch) and returns the same node context.
    pub fn force_highlight(&self, highlight: bool) -> Self {
        self.content_mut::<MenuButtonContent>().always_white = highlight;
        *self
    }

    /// 原14067e6d0：同步热键动作，并创建原相对根和ActionHint子节点。
    /// 0x12只跳过提示建树，仍然绑定ui_cancel输入。
    pub fn menu_hotkey(&self, action: u8) -> Self {
        let content = self.content_mut::<MenuButtonContent>();
        // Both source fields project the same original+0x25d byte: the
        // tooltip identifier and the action read by pre_update14067ea20.
        content.hotkey = action;
        content.action = action;
        self.make_hotkey_tooltip(action);
        *self
    }

    /// O 0x1406814f0 `PillButtonView::inactive_opacity`: writes the f32 at
    /// native `PillButtonContent+0x40` (this owner's `strength`).
    pub fn inactive_opacity(&self, opacity: f32) -> Self {
        self.content_mut::<PillButtonContent>().strength = opacity;
        *self
    }

    /// 原1406816c0：写入原+0x44动作的两个API投影，并建立真实提示。
    pub fn pill_hotkey(&self, action: u8) -> Self {
        let content = self.content_mut::<PillButtonContent>();
        // 原版 +0x44 同时用于提示和 pre_update；保留现有 API 的两个投影，
        // 绑定及重新绑定时必须一起更新，不能让输入仍读取旧动作。
        content.hotkey = action;
        content.action = action;
        self.make_hotkey_tooltip(action);
        *self
    }
}

impl NodeContext {
    /// Original14068bfc0,670B. Text itself is the stable local name.
    pub fn menu_button(&self, text: &str) -> Self {
        let child = widget_child(self, text, 0x615bfcfe4597ab8f);
        child
            .get_content_mut_impl::<MenuButtonContent>(false)
            .set_text(text.to_owned());
        let node = child.node_mut();
        node.flags |= 0x20;
        node.set_sounds(0, 2);
        child
    }
    /// 原14068c790：同一MenuButton身份、导航和0/2声音；字号40、字重700。
    pub fn settings_button(&self,text:&str)->Self {
        let child=self.menu_button(text);let content=child.content_mut::<MenuButtonContent>();
        content.text.set_metrics(40.).expect("原设置按钮字号");content.text.set_weight(Weight(700));child
    }

    /// Original14068bf00 and dynamic pagination140f9b789..140f9b7ea use
    /// the same LabelContent TypeId; String allocation isn't another widget.
    pub fn label(&self, text: impl Into<String>) -> Self {
        let text = text.into();
        let child = widget_child(self, &text, 0xce69dbf0aee13da2);
        child
            .get_content_mut_impl::<LabelContent>(false)
            .set_text(text);
        child
    }
    /// Original140fa0130,950B: preserves PillButtonContent owner and all its
    /// preparation/render/input algorithms. Caller may override80x60 layout.
    pub fn pill_button(&self, name: &str, image: &str) -> Self {
        let child = widget_child(self, name, 0xa6ddf7d2d741de4d);
        child.get_content_mut_impl::<PillButtonContent>(false).image = image.to_owned();
        let node = child.node_mut();
        node.flags |= 0x20;
        node.style = LayoutStyle {
            size: LengthPercentage::absolute(80.),
            min: LengthPercentage::absolute(60.),
            align: 2,
            justify: Justify(1),
            ..Default::default()
        };
        node.set_sounds(0, 2);
        child
    }
}

impl NodeContext {
    /// B14067df20, complete1125B: retained, tuple-keyed relative decoration
    /// roots exist only while hovered. Each image's local z is100.
    pub fn decorate_hovered(&self) -> Self {
        if !self.node().hover() {
            return *self;
        }
        let key = calculate_hash(&(self.id.0, 0xab23d50du32));
        for (index, offset, reference, anchor, image) in [
            (
                0u64,
                Vec2::new(-5., 0.),
                Vec2::new(-1., -0.),
                Vec2::new(1., 0.),
                "menu/button/select_left",
            ),
            (
                1u64,
                Vec2::new(5., 0.),
                Vec2::new(1., 0.),
                Vec2::new(-1., -0.),
                "menu/button/select_right",
            ),
        ] {
            let id = GlobalId(key.wrapping_add(index) ^ crate::mei::core::GLOBAL_ID_MASK);
            let root = self
                .mei_mut()
                .new_relative_with_id(id, self.id, None, offset, reference, anchor, false);
            // Both native calls use the same local image name; the root differs.
            root.image("deco left", image, 32.).node_mut().local_z = 100.;
        }
        *self
    }
}
