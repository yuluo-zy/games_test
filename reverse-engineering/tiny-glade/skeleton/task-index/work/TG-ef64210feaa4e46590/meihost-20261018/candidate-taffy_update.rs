//! Original set-44 Taffy finalization → layout → Ui2dExtent producer chain.
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

/// B 0x142340890: the native 12-byte TaffyNode tail is base-size Vec2 and a
/// one-frame size multiplier. Preserve its existing owner and decode safely.
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
        let read = |i| f32::from_ne_bytes(node._tail[i..i + 4].try_into().expect("TaffyNode tail"));
        let base = Vec2::new(read(0), read(4));
        let once = read(8);
        let user = if user_scale.is_some() {
            config.ui.icon_size_mult
        } else {
            1.0
        };
        let mut size = ((base * once) * user) * mult.multiplier;
        if let Some(text) = text {
            size = Vec2::from_array(text.bounds()).max(size);
        }
        layout.style_mut(node.node, |style| {
            style.size = Size {
                width: Dimension::Points(size.x),
                height: Dimension::Points(size.y),
            };
        });
        node._tail[8..12].copy_from_slice(&1.0_f32.to_ne_bytes());
    }
}

/// Original inlined body in 0x141e9eb40: width/height from actual WindowSize;
/// gap scale is config.ui.icon_size_mult * ToolbarSizeMult, with no extra DPI.
pub fn taffy_perform_layout(
    mut layout: ResMut<TaffyLayout>,
    window: Res<WindowSize>,
    config: Res<GameConfig>,
    mult: Res<ToolbarSizeMult>,
) {
    layout.perform_layout(window.as_vec2(), config.ui.icon_size_mult * mult.multiplier);
}

/// B 0x142340d20: copy actual Taffy size, and convert its origin to a center.
pub fn taffy_update_drawables(
    mut drawables: Query<(&mut Ui2dExtent, &TaffyNode)>,
    layout: Res<TaffyLayout>,
) {
    for (mut extent, node) in &mut drawables {
        let actual = layout
            .taffy
            .layout(node.node)
            .expect("drawable taffy layout");
        extent.size = Vec2::new(actual.size.width, actual.size.height);
        extent.pos = Vec2::new(actual.location.x, actual.location.y) + extent.size * 0.5;
    }
}
