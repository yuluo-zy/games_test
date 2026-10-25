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
    pub always_white: bool,
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
