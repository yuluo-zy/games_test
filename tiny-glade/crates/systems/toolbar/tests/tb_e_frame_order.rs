//! TB-E (round 11): MEI set-44 toolbar-side frame chain.
//!
//! Contract `evidence/round11/contracts/tb-toolbar-CONTRACT.md` §1.1 S7-S9:
//!
//! ```text
//! taffy_finalize_element_size -> taffy_perform_layout (compute + absolute)
//!   -> taffy_update_drawables
//! ```
//!
//! The surrounding S11/S12/S13 stages (`render_text_fragments`,
//! `mei_post_update`, `flush_mei_frame`) live in the `tiny-glade` host
//! (`crates/tiny-glade/src/mei_host.rs:56-67`,
//! `crates/tiny-glade/src/loading_graph.rs:170`) and require a real GPU device
//! (`GraphicsDeviceResource` / `AssetTextureLibrary`). They cannot be linked
//! from this crate's integration tests without a circular dev-dependency, so
//! this test asserts the complete toolbar-owned sub-chain with real data flow
//! between the stages, including the `TextFragment` measurement input.
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::IntoScheduleConfigs;
use country_core::components::Ui2dExtent;
use country_core::game_config::GameConfig;
use country_core::resources::window::WindowSize;
use glam::Vec2;
use taffy::style::{Dimension, Style};
use toolbar::resouces_and_components::taffy_layout::{AnchorSide, TaffyLayout};
use toolbar::resouces_and_components::TaffyNode;
use toolbar::taffy_update::{
    taffy_finalize_element_size, taffy_perform_layout, taffy_update_drawables,
};
use toolbar::toolbar_update::ToolbarSizeMult;

#[derive(Resource, Clone, Copy)]
struct ProbeNodes {
    button: taffy::node::Node,
    child: taffy::node::Node,
}

fn points(dimension: Dimension) -> f32 {
    match dimension {
        Dimension::Points(value) => value,
        other => panic!("expected Dimension::Points, got {other:?}"),
    }
}

/// S7 witness: finalize wrote the measured size into the real taffy style and
/// consumed the one-frame multiplier back to 1.0.
fn witness_after_finalize(layout: Res<TaffyLayout>, probe: Res<ProbeNodes>) {
    let style = layout.style(probe.button);
    assert_eq!(points(style.size.width), 400.0);
    assert_eq!(points(style.size.height), 200.0);
}

/// S8 witness: the anchored relative placement produced absolute coordinates
/// for the anchored node and recursively for its child.
fn witness_after_layout(layout: Res<TaffyLayout>, probe: Res<ProbeNodes>) {
    let button = layout.layout(probe.button);
    assert_eq!(Vec2::new(button.location.x, button.location.y), Vec2::new(201.0, -206.0));
    assert_eq!(button.size.width, 400.0);
    assert_eq!(button.size.height, 200.0);
    let child = layout.layout(probe.child);
    assert_eq!(Vec2::new(child.location.x, child.location.y), Vec2::new(201.0, -206.0));
}

/// S9 witness: the drawable bridge converted the layout origin into a center.
fn witness_after_drawables(drawables: Query<&Ui2dExtent, With<TaffyNode>>) {
    let mut seen = 0;
    for extent in &drawables {
        assert_eq!(extent.size, Vec2::new(400.0, 200.0));
        assert_eq!(extent.pos, Vec2::new(401.0, -106.0));
        seen += 1;
    }
    assert_eq!(seen, 1);
}

fn zero_bounds_fragment() -> tg_text::TextFragment {
    // Real text owner component: bounds() is [0, 0] until a renderer shapes
    // text, so finalize must keep its measured size (max with 0).
    let table = tg_text::FontTable::from_ron("(fonts:{Label:(family:\"Baloo2\")})")
        .expect("font table");
    tg_text::TextFragment::new(
        tg_text::FontMetrics::new(10.0, 12.0),
        tg_text::FontFamily::Label,
        std::sync::Arc::new(table),
    )
    .expect("text fragment")
}

#[test]
fn set44_taffy_finalize_layout_absolute_drawables_flow_in_order() {
    let mut world = World::new();
    world.insert_resource(GameConfig::default());
    // one-frame multiplier=2 -> finalize size = base*once*user*mult.
    world.insert_resource(ToolbarSizeMult { multiplier: 2.0 });
    world.insert_resource(WindowSize::new(800, 600));

    let mut layout = TaffyLayout::default();
    // Reference node for the anchored record (Window anchors ignore relative_to).
    let reference = layout
        .taffy
        .new_leaf(Style::default())
        .expect("reference leaf");
    let window = layout
        .add_node(Style::default())
        .anchored_to(0, Vec2::ZERO, reference, 0.0, AnchorSide::Window);
    let button = layout
        .add_node(Style::default())
        .anchored_to(1, Vec2::new(1.0, 2.0), window, 4.0, AnchorSide::Above);
    // S8 absolute recursion target: the anchored child must receive the same
    // origin as its parent.
    let child = layout
        .taffy
        .new_leaf(Style {
            size: taffy::geometry::Size {
                width: Dimension::Points(10.0),
                height: Dimension::Points(10.0),
            },
            ..Default::default()
        })
        .expect("child leaf");
    layout.taffy.add_child(button, child).expect("button child edge");
    world.insert_resource(layout);
    world.insert_resource(ProbeNodes { button, child });

    // Tail: base=(100, 50), one-frame multiplier=2.
    let mut tail = [0u8; 12];
    tail[0..4].copy_from_slice(&100.0f32.to_ne_bytes());
    tail[4..8].copy_from_slice(&50.0f32.to_ne_bytes());
    tail[8..12].copy_from_slice(&2.0f32.to_ne_bytes());
    let entity = world
        .spawn((
            TaffyNode { node: button, _tail: tail },
            Ui2dExtent::default(),
            zero_bounds_fragment(),
        ))
        .id();

    let mut schedule = Schedule::default();
    schedule.add_systems(
        (
            taffy_finalize_element_size,
            witness_after_finalize,
            taffy_perform_layout,
            witness_after_layout,
            taffy_update_drawables,
            witness_after_drawables,
        )
            .chain(),
    );
    schedule.run(&mut world);
    world.flush();

    // Final state is also asserted outside the chain (the witnesses above
    // already prove each stage saw the previous stage's output).
    let layout = world.resource::<TaffyLayout>();
    let style = layout.style(button);
    assert_eq!(points(style.size.width), 400.0);
    assert_eq!(points(style.size.height), 200.0);
    let extent = world.get::<Ui2dExtent>(entity).expect("drawable extent");
    assert_eq!(extent.size, Vec2::new(400.0, 200.0));
    assert_eq!(extent.pos, Vec2::new(401.0, -106.0));
    // The one-frame multiplier was reset to 1.0 (S7 contract).
    assert_eq!(world.get::<TaffyNode>(entity).expect("taffy node").scale(), 1.0);
}

#[test]
fn finalize_scales_base_size_by_user_scale_and_toolbar_multiplier() {
    use toolbar::resouces_and_components::UiScaleWithUserSetting;

    let mut world = World::new();
    let mut config = GameConfig::default();
    config.ui.icon_size_mult = 1.5;
    world.insert_resource(config);
    world.insert_resource(ToolbarSizeMult { multiplier: 2.0 });

    let mut layout = TaffyLayout::default();
    let node = layout
        .taffy
        .new_leaf(Style::default())
        .expect("taffy leaf");
    world.insert_resource(layout);

    let tail = {
        let mut tail = [0u8; 12];
        tail[0..4].copy_from_slice(&30.0f32.to_ne_bytes());
        tail[4..8].copy_from_slice(&20.0f32.to_ne_bytes());
        tail[8..12].copy_from_slice(&2.0f32.to_ne_bytes());
        tail
    };
    // UiScaleWithUserSetting selects config.ui.icon_size_mult instead of 1.0.
    let scaled = world
        .spawn((
            TaffyNode { node, _tail: tail },
            UiScaleWithUserSetting,
        ))
        .id();

    let mut schedule = Schedule::default();
    schedule.add_systems(taffy_finalize_element_size);
    schedule.run(&mut world);

    // 30 * 2 (once) * 1.5 (user) * 2 (toolbar) = 180; 20 * 2 * 1.5 * 2 = 120.
    let style = world.resource::<TaffyLayout>().style(node);
    assert_eq!(points(style.size.width), 180.0);
    assert_eq!(points(style.size.height), 120.0);
    assert_eq!(world.get::<TaffyNode>(scaled).expect("node").scale(), 1.0);
}
