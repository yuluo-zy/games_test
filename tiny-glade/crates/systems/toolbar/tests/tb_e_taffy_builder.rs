//! TB-E (round 11): `TaffyNodeBuilder` root/anchor/child primitives.
//!
//! Contract `tb-toolbar-CONTRACT.md` §2.11 (#55-#58): `anchored_to` appends the
//! `AnchoredNode` record, `child_of` creates the real taffy child edge, and
//! `root`/`root_mut`/`roots_mut` expose the registered root list.
use glam::Vec2;
use taffy::style::{Dimension, Style};
use taffy::tree::LayoutTree;
use toolbar::resouces_and_components::taffy_layout::{AnchorSide, TaffyLayout};

fn leaf_style(width: f32, height: f32) -> Style {
    Style {
        size: taffy::geometry::Size {
            width: Dimension::Points(width),
            height: Dimension::Points(height),
        },
        ..Default::default()
    }
}

#[test]
fn root_registers_both_roots_and_root_mut_finds_by_node_id() {
    let mut layout = TaffyLayout::default();
    let first = layout.add_node(leaf_style(10.0, 10.0)).root();
    let second = layout.add_node(leaf_style(20.0, 20.0)).root();

    assert_eq!(layout.roots(), &[first, second][..]);
    assert_eq!(*layout.root_mut(first), first);
    assert_eq!(*layout.root_mut(second), second);
    // roots_mut is the mutable slice consumer used by toolbar_alignment.
    let extra = layout_add_leaf(&mut layout, 5.0);
    layout.roots_mut().push(extra);
    assert_eq!(layout.roots().len(), 3);
}

/// Helper that adds a node and returns its id without registering it as root.
fn layout_add_leaf(layout: &mut TaffyLayout, size: f32) -> taffy::node::Node {
    layout
        .taffy
        .new_leaf(leaf_style(size, size))
        .expect("taffy leaf")
}

#[test]
fn child_of_creates_the_real_taffy_parent_child_edge() {
    let mut layout = TaffyLayout::default();
    let root = layout.add_node(leaf_style(100.0, 100.0)).root();
    let child = layout.add_node(leaf_style(20.0, 30.0)).child_of(root);
    let grandchild = layout
        .add_node(leaf_style(5.0, 5.0))
        .child_of(child);

    assert_eq!(layout.children(root), vec![child]);
    assert_eq!(layout.children(child), vec![grandchild]);
    assert!(layout.children(grandchild).is_empty());
    // The child edge is a real taffy relation, not just a local list.
    assert_eq!(layout.taffy.parent(child), Some(root));
    assert_eq!(layout.taffy.parent(root), None);
}

#[test]
fn anchored_to_appends_the_native_record_and_returns_the_node() {
    let mut layout = TaffyLayout::default();
    let root = layout.add_node(leaf_style(100.0, 100.0)).root();
    let node = layout.add_node(leaf_style(20.0, 10.0)).anchored_to(
        7,
        Vec2::new(3.0, 4.0),
        root,
        2.5,
        AnchorSide::Right,
    );

    assert_eq!(layout.anchored.len(), 1);
    let record = layout.anchored[0];
    assert_eq!(record.order, 7);
    assert_eq!(record.offset, Vec2::new(3.0, 4.0));
    assert_eq!(record.node, node);
    assert_eq!(record.relative_to, root);
    assert_eq!(record.gap, 2.5);
    assert_eq!(record.side, AnchorSide::Right);

    // anchored_to only appends the anchor record: no root and no child edge.
    assert_eq!(layout.roots(), &[root][..]);
    assert!(layout.children(root).is_empty());

    // A second anchor keeps push order; perform_layout sorts by order later.
    let second = layout
        .add_node(leaf_style(1.0, 1.0))
        .anchored_to(2, Vec2::ZERO, root, 0.0, AnchorSide::Above);
    assert_eq!(layout.anchored.len(), 2);
    assert_eq!(layout.anchored[1].node, second);
    assert_eq!(layout.anchored[1].side, AnchorSide::Above);
}

#[test]
fn anchored_perform_layout_orders_and_absolutizes_registered_records() {
    let mut layout = TaffyLayout::default();
    let root = layout.add_node(leaf_style(200.0, 100.0)).root();
    // Register dependants out of order and anchored to the root's origin.
    let button = layout
        .add_node(leaf_style(40.0, 20.0))
        .anchored_to(5, Vec2::new(1.0, 2.0), root, 3.0, AnchorSide::Below);
    // Window entry must sort first and provide the layout root size.
    let window = layout
        .add_node(Style::default())
        .anchored_to(0, Vec2::ZERO, root, 0.0, AnchorSide::Window);
    // Make the relative target part of the computed window subtree so its
    // real size/location are available to the anchored placement pass.
    layout
        .taffy
        .add_child(window, root)
        .expect("window -> root edge");

    layout.perform_layout(Vec2::new(400.0, 300.0), 1.0);

    let window_layout = layout.layout(window);
    assert_eq!(window_layout.size.width, 400.0);
    assert_eq!(window_layout.size.height, 300.0);
    // The anchor record is relative to `root`, now computed inside the window
    // subtree at origin 0, size 200x100. Below: centered x, y = height + gap,
    // plus the record offset (1, 2).
    let button_layout = layout.layout(button);
    assert_eq!(button_layout.size.width, 40.0);
    assert_eq!(button_layout.size.height, 20.0);
    assert_eq!(
        Vec2::new(button_layout.location.x, button_layout.location.y),
        Vec2::new(81.0, 105.0)
    );
}
