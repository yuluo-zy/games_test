//! Original set-44 Taffy finalization → layout → Ui2dExtent producer chain.
//!
//! Fresh evidence (2026-10-19, B patched EXE):
//! - `taffy_finalize_element_size` 0x142340890, PDB body 1160 B: prepared and
//!   decompiled in `evidence/round11/tbd-jobs/decomp/142340890.c`.
//!   The native loop walks Bevy's two query storage maps (`+0x108` selects
//!   table stride 0xa0 vs sparse-set stride 0x48) — that is exactly what
//!   `Query::iter_mut` compiles to, so the port uses the real `Query`.
//!   Per entity: `size = base(+8) * once(+0x10) * (UiScaleWithUserSetting ?
//!   GameConfig.icon_size_mult(+0xE8) : 1.0) * ToolbarSizeMult`; if a
//!   `TextFragment` is present `size = Vec2::max(size, bounds())`.  The style
//!   write is `taffy_mut_style<perform_layout::closure_env$1>` 0x142376120,
//!   which stores `Style.size = {Dimension::Points(x), Dimension::Points(y)}`
//!   (verified: four dwords `{0,w,0,h}` at Style+0xD8).  Finally `+0x10` is
//!   reset to `1.0` (`0x3f800000`).
//! - `taffy_update_drawables` 0x142340d20, PDB body 852 B
//!   (`decomp/142340d20.c`): per node `Taffy::layout` (unwrap), then
//!   `Ui2dExtent.pos = size*0.5 + location` and `Ui2dExtent.size = size`;
//!   the remaining u32 store is Bevy's `&mut` changed-tick column
//!   (`TicksMut::set_changed`), not game data.
use bevy_ecs::prelude::*;
use country_core::{
    components::Ui2dExtent, game_config::GameConfig, resources::window::WindowSize,
};
use glam::Vec2;
use taffy::geometry::Size;
use taffy::style::Dimension;
use tg_text::TextFragment;

use crate::{
    resouces_and_components::{TaffyNode, UiScaleWithUserSetting, taffy_layout::TaffyLayout},
    toolbar_update::ToolbarSizeMult,
};

/// `TaffyNode` tail layout (fresh 0x142340890 / 0x142340d20):
/// `+0x08` base width, `+0x0c` base height, `+0x10` one-frame multiplier.
const TAIL_BASE: std::ops::Range<usize> = 0..8;
const TAIL_MULT: std::ops::Range<usize> = 8..12;

/// Base size stored in `TaffyNode+0x08..+0x10` (O, 0x142340890 reads the two
/// f32 there; `add_toolbar_item` spawn writes them).
pub fn taffy_node_base_size(node: &TaffyNode) -> Vec2 {
    Vec2::new(read_f32(&node._tail, 0), read_f32(&node._tail, 4))
}

/// Replace the base size at `TaffyNode+0x08..+0x10`.
pub fn set_taffy_node_base_size(node: &mut TaffyNode, size: Vec2) {
    node._tail[0..4].copy_from_slice(&size.x.to_ne_bytes());
    node._tail[4..8].copy_from_slice(&size.y.to_ne_bytes());
}

fn read_f32(bytes: &[u8], offset: usize) -> f32 {
    f32::from_ne_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

/// Pure native formula of 0x142340890 (used by the system and unit tests).
///
/// `base` = `TaffyNode+0x08` Vec2, `once` = `TaffyNode+0x10` f32,
/// `icon_mult` = `Some(GameConfig.ui.icon_size_mult)` when the entity carries
/// `UiScaleWithUserSetting` else `None` (native substitutes `1.0`),
/// `mult` = `ToolbarSizeMult.multiplier`, `text` = `TextFragment::bounds()`
/// when the optional component is present.
pub fn finalize_element_size(
    base: Vec2,
    once: f32,
    icon_mult: Option<f32>,
    mult: f32,
    text: Option<Vec2>,
) -> Vec2 {
    let user = icon_mult.unwrap_or(1.0);
    let size = (base * once) * user * mult;
    match text {
        Some(bounds) => size.max(bounds),
        None => size,
    }
}

/// B 0x142340890 `taffy::taffy_update::taffy_finalize_element_size`.
///
/// PDB signature (structures: IsFunctionSystem instantiation):
/// `fn(Query<(&mut TaffyNode, Option<&UiScaleWithUserSetting>,
/// Option<&TextFragment>)>, ResMut<TaffyLayout>, Res<GameConfig>,
/// Res<ToolbarSizeMult>)`.
pub fn taffy_finalize_element_size(
    mut nodes: Query<(
        &mut TaffyNode,
        Option<&UiScaleWithUserSetting>,
        Option<&TextFragment>,
    )>,
    mut layout: ResMut<TaffyLayout>,
    config: Res<GameConfig>,
    mult: Res<ToolbarSizeMult>,
) {
    for (mut node, user_scale, text) in &mut nodes {
        let size = finalize_element_size(
            taffy_node_base_size(&node),
            node.scale(),
            user_scale.is_some().then_some(config.ui.icon_size_mult),
            mult.multiplier,
            text.map(|text| Vec2::from_array(text.bounds())),
        );
        layout.style_mut(node.node, |style| {
            style.size = Size {
                width: Dimension::Points(size.x),
                height: Dimension::Points(size.y),
            };
        });
        // Native `puVar12[4] = 0x3f800000` (TaffyNode+0x10) after the style write.
        node.set_scale(1.0);
    }
}

/// Native `taffy_perform_layout` (0x141e9eb40 inline + `TaffyLayout::perform_layout`
/// 0x141e4c6c0): layout with the actual window size and the scaled gap.
/// PDB signature: `fn(ResMut<TaffyLayout>, Res<WindowSize>, Res<GameConfig>,
/// Res<ToolbarSizeMult>)`.
pub fn taffy_perform_layout(
    mut layout: ResMut<TaffyLayout>,
    window: Res<WindowSize>,
    config: Res<GameConfig>,
    mult: Res<ToolbarSizeMult>,
) {
    layout.perform_layout(
        window.as_vec2(),
        config.ui.icon_size_mult * mult.multiplier,
    );
}

/// B 0x142340d20 `taffy::taffy_update::taffy_update_drawables`.
///
/// PDB signature: `fn(Query<(&mut Ui2dExtent, &TaffyNode)>, Res<TaffyLayout>)`.
pub fn taffy_update_drawables(
    mut drawables: Query<(&mut Ui2dExtent, &TaffyNode)>,
    layout: Res<TaffyLayout>,
) {
    for (mut extent, node) in &mut drawables {
        let actual = *layout
            .taffy
            .layout(node.node)
            .expect("drawable taffy layout");
        let size = Vec2::new(actual.size.width, actual.size.height);
        // Native order: center first, then size (both written from the same
        // computed layout record).
        extent.pos = Vec2::new(actual.location.x, actual.location.y) + size * 0.5;
        extent.size = size;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finalize_formula_matches_native_order_and_text_max() {
        let base = Vec2::new(10.0, 20.0);
        // base * once * user * mult
        assert_eq!(
            finalize_element_size(base, 2.0, Some(0.5), 3.0, None),
            Vec2::new(30.0, 60.0)
        );
        // no UiScaleWithUserSetting -> icon multiplier 1.0
        assert_eq!(
            finalize_element_size(base, 1.0, None, 2.0, None),
            Vec2::new(20.0, 40.0)
        );
        // Text bounds only raise the size (glam Vec2::max).
        assert_eq!(
            finalize_element_size(base, 1.0, None, 1.0, Some(Vec2::new(5.0, 50.0))),
            Vec2::new(10.0, 50.0)
        );
    }

    #[test]
    fn taffy_node_tail_carries_base_size_and_one_frame_multiplier() {
        let mut node = TaffyNode::default();
        assert_eq!(taffy_node_base_size(&node), Vec2::ZERO);
        assert_eq!(node.scale(), 1.0);
        set_taffy_node_base_size(&mut node, Vec2::new(7.0, 9.0));
        node.set_scale(0.25);
        assert_eq!(taffy_node_base_size(&node), Vec2::new(7.0, 9.0));
        assert_eq!(node.scale(), 0.25);
        // The multiplier occupies exactly +0x10..+0x14, base ends at +0x10.
        assert_eq!(&node._tail[TAIL_BASE], &[0x00, 0x00, 0xe0, 0x40, 0x00, 0x00, 0x10, 0x41]);
        assert_eq!(&node._tail[TAIL_MULT], &0.25f32.to_ne_bytes());
    }
}
