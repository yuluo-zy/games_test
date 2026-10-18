//! B TaffyLayout consumer closure. Original instructions and field reads are
//! retained in task-index work TG-d76bef83fa63884426/meihost-20261018.
use bevy_ecs::prelude::Resource;
use glam::Vec2;
use taffy::{
    geometry::{Point, Size},
    node::{Node, Taffy},
    style::{AvailableSpace, Dimension, Style},
    tree::LayoutTree,
};

/// Semantic names for original side tags consumed at 0x141e4c8c8.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AnchorSide {
    Above = 0,
    Below = 1,
    Left = 2,
    Right = 3,
    Window = 4,
}

/// Original record reads: order@0, offset@8, node@16, relative_to@24,
/// gap@32 and side@36. The safe owner does not mirror Windows enum padding.
#[derive(Clone, Copy, Debug)]
pub struct AnchoredNode {
    pub order: u64,
    pub offset: Vec2,
    pub node: Node,
    pub relative_to: Node,
    pub gap: f32,
    pub side: AnchorSide,
}

#[derive(Resource, Default)]
pub struct TaffyLayout {
    pub taffy: Taffy,
    pub anchored: Vec<AnchoredNode>,
}

impl TaffyLayout {
    pub fn style(&self, node: Node) -> &Style {
        self.taffy.style(node).expect("taffy node style")
    }

    /// Native taffy_mut_style: clone the actual dependency style, edit, then
    /// set_style so its dirty/cache protocol remains the pinned dependency's.
    pub fn style_mut(&mut self, node: Node, update: impl FnOnce(&mut Style)) {
        let mut style = self.style(node).clone();
        update(&mut style);
        self.taffy
            .set_style(node, style)
            .expect("set taffy node style");
    }

    /// Original 0x141e4c6c0. `scale` is icon_size_mult * ToolbarSizeMult.
    pub fn perform_layout(&mut self, window: Vec2, scale: f32) {
        self.anchored.sort_unstable_by_key(|entry| entry.order);
        for index in 0..self.anchored.len() {
            let entry = self.anchored[index];
            if entry.side == AnchorSide::Window {
                self.style_mut(entry.node, |style| {
                    style.size = Size {
                        width: Dimension::Points(window.x),
                        height: Dimension::Points(window.y),
                    };
                });
            }
            self.taffy
                .compute_layout(
                    entry.node,
                    Size {
                        width: AvailableSpace::MaxContent,
                        height: AvailableSpace::MaxContent,
                    },
                )
                .expect("compute anchored taffy layout");
            let origin = if entry.side == AnchorSide::Window {
                entry.offset
            } else {
                let relative = *self
                    .taffy
                    .layout(entry.relative_to)
                    .expect("anchor target layout");
                let own = *self.taffy.layout(entry.node).expect("anchored node layout");
                let gap = entry.gap * scale;
                let offset = match entry.side {
                    AnchorSide::Above => Vec2::new(
                        relative.size.width * 0.5 - own.size.width * 0.5,
                        -own.size.height - gap,
                    ),
                    AnchorSide::Below => Vec2::new(
                        relative.size.width * 0.5 - own.size.width * 0.5,
                        relative.size.height + gap,
                    ),
                    AnchorSide::Left => Vec2::new(
                        -own.size.width - gap,
                        relative.size.height * 0.5 - own.size.height * 0.5,
                    ),
                    AnchorSide::Right => Vec2::new(
                        relative.size.width + gap,
                        relative.size.height * 0.5 - own.size.height * 0.5,
                    ),
                    AnchorSide::Window => unreachable!(),
                };
                offset + Vec2::new(relative.location.x, relative.location.y) + entry.offset
            };
            make_layout_absolute(&mut self.taffy, entry.node, origin);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchors_use_original_side_tags_gap_scale_and_reference_origin() {
        let mut layout = TaffyLayout::default();
        let root = layout.taffy.new_leaf(Style::default()).unwrap();
        layout.anchored.push(AnchoredNode {
            order: 0,
            offset: Vec2::new(5.0, 7.0),
            node: root,
            relative_to: root,
            gap: 0.0,
            side: AnchorSide::Window,
        });
        let expected = [
            Vec2::new(46.0, -4.0),
            Vec2::new(46.0, 92.0),
            Vec2::new(-17.0, 44.0),
            Vec2::new(109.0, 44.0),
        ];
        let mut nodes = Vec::new();
        for (i, side) in [
            AnchorSide::Above,
            AnchorSide::Below,
            AnchorSide::Left,
            AnchorSide::Right,
        ]
        .into_iter()
        .enumerate()
        {
            let node = layout
                .taffy
                .new_leaf(Style {
                    size: Size {
                        width: Dimension::Points(20.0),
                        height: Dimension::Points(10.0),
                    },
                    ..Default::default()
                })
                .unwrap();
            nodes.push(node);
            layout.anchored.push(AnchoredNode {
                order: i as u64 + 1,
                offset: Vec2::new(1.0, 2.0),
                node,
                relative_to: root,
                gap: 2.0,
                side,
            });
        }
        // The native unsigned sort must put the root before its dependants.
        layout.anchored.reverse();
        layout.perform_layout(Vec2::new(100.0, 80.0), 1.5);
        for (node, expected) in nodes.into_iter().zip(expected) {
            let actual = layout.taffy.layout(node).unwrap();
            assert_eq!(Vec2::new(actual.location.x, actual.location.y), expected);
        }
    }
}

/// Original 0x142374e10. LayoutTree::layout_mut accesses the selected rounded
/// or unrounded layout exactly as the pinned dependency implementation does.
pub fn make_layout_absolute(tree: &mut Taffy, node: Node, parent: Vec2) {
    let origin = {
        let layout = LayoutTree::layout_mut(tree, node);
        layout.location = Point {
            x: layout.location.x + parent.x,
            y: layout.location.y + parent.y,
        };
        Vec2::new(layout.location.x, layout.location.y)
    };
    tree.mark_dirty(node)
        .expect("mark absolute taffy node dirty");
    for child in tree.children(node).expect("absolute taffy children") {
        make_layout_absolute(tree, child, origin);
    }
}
