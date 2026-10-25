//! `toolbar::toolbar_update` per-frame update chain.
//!
//! Fresh evidence (2026-10-19, B patched EXE):
//! - `toolbar_button_size_and_padding_update` 0x141eaec50, PDB body 3563 B,
//!   decompiled in `evidence/round11/tbd-jobs/decomp/141eaec50.c`;
//!   PDB structures give the exact system signature
//!   `fn(Query<(&mut TaffyNode, Option<&ShowOnlyInPhotomode>), With<ToolbarExpand>>,
//!   Res<ToolbarLayout>, Res<PhotomodeToolbarLayout>, ResMut<TaffyLayout>,
//!   Res<GameConfig>, Res<ToolbarSizeMult>)`.
//!   Decoded body:
//!   1. `gap = GameConfig.ui.icon_size_mult(+0xE8) * 10.0 * ToolbarSizeMult`
//!      (`MOVSS [rax+0xE8]; MULSS 10.0; MULSS [mult]`).
//!   2. For each of the two toolbar contexts (normal, photomode), walk the
//!      query.  In the **normal** context only, every node *without*
//!      `ShowOnlyInPhotomode` gets its one-frame multiplier scaled by
//!      `lerp(0.8, 1.0, BackOut::apply(ctx+0x28))`
//!      (`((1.0 - ease)*0.8 + ease) * scale`, constants 0x142a16cb8=0.8,
//!      0x142925904=1.0, easing static 0x142f986e4 has discriminant 5 = BackOut).
//!   3. `taffy_mut_style<closure_env$1>` 0x142375db0 writes
//!      `Style.gap = Size { Points(gap), Points(gap) }` on `ctx+0x20` (four
//!      dwords `{0, gap, 0, gap}` at Style+0xA8, verified by an offset probe
//!      against taffy 0.3.13: Style::gap @0xA8, LengthPercentage tag 0 =
//!      Points).
//!   4. For every contextual-menu record with a live node, the same gap is
//!      written to each direct taffy child (`TaffyLayout::children` +
//!      `taffy::style`/`set_style`).
//!
//! `toolbar_size_mult_update`: original inline body recovered from the exact
//! PDB run_unsafe host 0x140feff70 (673 B), arithmetic 0x140ff0052..0x140ff00b9.
//! The long axis is width for Bottom/Top, height for Left/Right; Left alone
//! includes its original additional reserved-width term. Native MINSS caps
//! the result at 1 and returns 1 for an unordered (NaN) ratio.

use bevy_ecs::prelude::*;
use country_core::game_config::GameConfig;
use taffy::geometry::Size;
use taffy::style::LengthPercentage;
use utils::easing_functions::EasingFunction;

use crate::resouces_and_components::{
    ShowOnlyInPhotomode, TaffyNode, ToolbarExpand,
    taffy_layout::TaffyLayout,
    toolbar_layout::{PhotomodeToolbarLayout, ToolbarContext, ToolbarLayout},
};

/// 工具栏尺寸倍率（O：`toolbar::toolbar_update::ToolbarSizeMult`；
/// `render_taffy_ui_to_widgets` 0x142338990 fresh 反编译中作为 `f32` 读入
/// （`fVar26 = *local_208`，用于 `rect = (len + scale·10·mult) · 0.5` 的扩张）。
/// **重建（I）**：单 f32 载荷；字段名 U。
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct ToolbarSizeMult {
    /// 尺寸倍率（f32@+0）。
    pub multiplier: f32,
}

/// Actual resolution/user-scale formula, in original f32 evaluation order.
/// B constants: 142bc395c=1282.5, 1429cd034=30,
/// 142bc3960=-192.5, 142bc3964=22. No lower clamp exists in the original.
pub fn toolbar_size_multiplier(alignment: u8, window: [u32; 2], icon_size_mult: f32) -> f32 {
    let axis = window[usize::from(alignment == 1 || alignment == 2)] as f32;
    let mut required = 1282.5 * icon_size_mult + 30.0;
    if alignment == 1 {
        required += icon_size_mult * -192.5;
        required += 22.0;
    }
    let ratio = axis / required;
    if ratio < 1.0 { ratio } else { 1.0 }
}

/// B original inline system. Always write the canonical owner/change tick;
/// the zero startup value is replaced by this system before sizing consumers.
pub fn toolbar_size_mult_update(
    config: Res<GameConfig>,
    window: Res<country_core::resources::window::WindowSize>,
    mut mult: ResMut<ToolbarSizeMult>,
) {
    mult.multiplier = toolbar_size_multiplier(
        config.ui.toolbar_alignment as u8,
        [window.width, window.height],
        config.ui.icon_size_mult,
    );
}

/// Native `toolbar_button_size_and_padding_update` 0x141eaec50.
///
/// The multiplier stored at `TaffyNode+0x10` is read one frame later by
/// `taffy_finalize_element_size`; the gap writes feed the next
/// `taffy_perform_layout`.
pub fn toolbar_button_size_and_padding_update(
    mut nodes: Query<(&mut TaffyNode, Option<&ShowOnlyInPhotomode>), With<ToolbarExpand>>,
    toolbar: Res<ToolbarLayout>,
    photomode: Res<PhotomodeToolbarLayout>,
    mut layout: ResMut<TaffyLayout>,
    config: Res<GameConfig>,
    mult: Res<ToolbarSizeMult>,
) {
    // `fStack_d4 = *(f32*)(config+0xE8) * 10.0 * *mult` (0x141eaf0xx).
    let gap = config.ui.icon_size_mult * 10.0 * mult.multiplier;

    for (context, is_photomode) in [
        (&*toolbar as &dyn ToolbarContext, false),
        (&*photomode as &dyn ToolbarContext, true),
    ] {
        // Native only runs the shrink easing for the normal toolbar context
        // (`(char)puStack_d0 == 0`); the photomode pass is gap-only.
        if !is_photomode {
            let ease = EasingFunction::BackOut.apply(context.scale_progress());
            let factor = (1.0 - ease) * 0.8 + ease;
            for (mut node, show_only_in_photomode) in &mut nodes {
                // Native condition: optional component absent.
                if show_only_in_photomode.is_none() {
                    let scale = node.scale();
                    node.set_scale(factor * scale);
                }
            }
        }
        // `taffy_mut_style<closure_env$1>` on the context root (+0x20).
        layout.style_mut(context.node_b(), |style| {
            style.gap = Size {
                width: LengthPercentage::Points(gap),
                height: LengthPercentage::Points(gap),
            };
        });
        // Native: for every contextual-menu record with a node, re-read each
        // child style and set the same gap (`TaffyLayout::children` deps).
        for menu in context.contextual_menus() {
            if menu.node == taffy::node::Node::default() {
                continue;
            }
            for child in layout.children(menu.node) {
                layout.style_mut(child, |style| {
                    style.gap = Size {
                        width: LengthPercentage::Points(gap),
                        height: LengthPercentage::Points(gap),
                    };
                });
            }
        }
    }
}
