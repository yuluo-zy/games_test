//! Original UndoRedoLayout::setup B140fad870/2580B. The complete native
//! constructor was executed with boundary-only Taffy/Bevy allocation hooks;
//! styles, bundles and deferred component inserts are retained in
//! progress-20261020/central-undo-layout/native-undo-setup.json.
use bevy_ecs::{prelude::Resource, system::Commands};
use glam::Vec2;
use taffy::{geometry::{Rect, Size}, node::Node,
    style::{AlignItems, JustifyContent, LengthPercentage, Style}};

use super::{taffy_layout::{AnchorSide, AnchoredNode, TaffyLayout},
    HideIfMenuIsOpen, HistoryButton, TaffyNode, UiAnimationState,
    UiInteractionState, UiMouseInteractable, UiScaleWithUserSetting};
use country_core::components::{Ui2dExtent, Ui2dImage, UiDrawOrder};

#[derive(Resource)]
pub struct UndoRedoLayout {
    /// Native +0: window anchor node.
    pub screen: Node,
    /// Native +8: button row, child of screen.
    pub buttons: Node,
    /// Native +0x10: undo/redo auto-hide progress, initialized to zero.
    pub hide_progress: f32,
}

impl UndoRedoLayout {
    pub fn setup(layout: &mut TaffyLayout, commands: &mut Commands) -> Self {
        let screen = layout.taffy.new_leaf(Style {
            padding: Rect { left: LengthPercentage::Points(15.0),
                right: LengthPercentage::Points(15.0),
                top: LengthPercentage::Points(15.0),
                bottom: LengthPercentage::Points(15.0) },
            align_items: Some(AlignItems::FlexStart),
            justify_content: Some(JustifyContent::Center),
            ..Style::default()
        }).expect("undo/redo screen node");
        layout.anchored.push(AnchoredNode {
            order: 0, offset: Vec2::ZERO, node: screen,
            relative_to: Node::default(), gap: 0.0, side: AnchorSide::Window,
        });
        let item_style = || Style {
            align_items: Some(AlignItems::Center),
            justify_content: Some(JustifyContent::Center),
            ..Style::default()
        };
        let buttons = layout.taffy.new_leaf(Style {
            gap: Size { width: LengthPercentage::Points(10.0),
                height: LengthPercentage::Points(10.0) },
            ..item_style()
        }).expect("undo/redo button row");
        layout.taffy.add_child(screen, buttons).expect("undo/redo screen child");
        // The first original pointer addresses adjacent undo/redo literals;
        // its length is exactly 9. It is "icon/undo", not their concatenation.
        for (name, history) in [("icon/undo", HistoryButton::UNDO),
                                ("icon/redo", HistoryButton::REDO)] {
            let node = layout.taffy.new_leaf(item_style()).expect("undo/redo button");
            layout.taffy.add_child(buttons, node).expect("undo/redo row child");
            let mut leaf = TaffyNode { node, ..Default::default() };
            crate::taffy_update::set_taffy_node_base_size(&mut leaf, Vec2::splat(66.0));
            commands.spawn((leaf, UiScaleWithUserSetting,
                UiInteractionState { state: 1, keep: 0, selected: 0 },
                UiAnimationState { appear: 1.0, ..UiAnimationState::for_spawn() },
                UiMouseInteractable { bytes: 0.011f32.to_le_bytes() },
                Ui2dExtent::default(), Ui2dImage::new(name), UiDrawOrder::default()))
                .insert(history)
                .insert(HideIfMenuIsOpen);
        }
        Self { screen, buttons, hide_progress: 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::{prelude::World, world::CommandQueue};
    #[test]
    fn setup_builds_the_original_tree_and_history_button_payloads() {
        let mut world = World::new();
        let mut queue = CommandQueue::default();
        let mut layout = TaffyLayout::default();
        let undo = UndoRedoLayout::setup(&mut layout, &mut Commands::new(&mut queue, &world));
        assert_eq!(layout.anchored.len(), 1);
        assert_eq!(layout.anchored[0].side, AnchorSide::Window);
        assert_eq!(layout.children(undo.screen), [undo.buttons]);
        assert_eq!(layout.children(undo.buttons).len(), 2);
        assert_eq!(layout.style(undo.screen).align_items, Some(AlignItems::FlexStart));
        assert_eq!(layout.style(undo.screen).padding.left, LengthPercentage::Points(15.0));
        assert_eq!(undo.hide_progress.to_bits(), 0);
        assert_eq!(world.entities().len(), 0, "commands remain deferred");
        queue.apply(&mut world);
        let mut buttons: Vec<_> = world.query::<(&HistoryButton, &Ui2dImage, &TaffyNode,
            &UiAnimationState, &UiInteractionState, &UiMouseInteractable)>().iter(&world)
            .map(|(h, image, node, animation, interaction, mouse)| {
                assert_eq!(crate::taffy_update::taffy_node_base_size(node), Vec2::splat(66.0));
                assert_eq!(animation.appear.to_bits(), 1.0f32.to_bits());
                assert_eq!(animation.state, 2);
                assert_eq!(interaction.state, 1);
                assert_eq!(mouse.bytes, 0.011f32.to_le_bytes());
                (h.kind, image.name.clone())
            }).collect();
        buttons.sort();
        assert_eq!(buttons, [(0, "icon/undo".to_owned()), (1, "icon/redo".to_owned())]);
    }
}
