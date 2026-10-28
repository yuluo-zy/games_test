//! UNMOUNTED candidate for toolbar_systems child module; freeze prohibits export.
//! Original B toolbar_steal_mouse_input142344df0/2706B, body SHA256
//! 9b00afb82fd97223222be7f4c04718ec342a75df07675620972b92f8882e115c.
//! Six original SystemParams; this producer never mutates ActionSetStates.
use crate::{
    resouces_and_components::{
        taffy_layout::TaffyLayout,
        toolbar_layout::{PhotomodeToolbarLayout, ToolbarContext, ToolbarLayout},
    },
    toolbar_update::ToolbarSizeMult,
};
use bevy_ecs::prelude::*;
use country_core::{
    resources::gui::DebugGui,
    systems::ui::resources::{
        ui_state::UiState,
        widget_params::{UiSounds, WidgetParams},
    },
};
use glam::Vec2;

/// Original set8: after toolbar_size_mult_update, before the button tuple.
/// Registrations feed the following frame's set6 picker. This is an input
/// volume for toolbar backgrounds, not an Escape-menu input-reset system.
pub fn toolbar_steal_mouse_input(
    toolbar: Res<ToolbarLayout>,
    photomode: Res<PhotomodeToolbarLayout>,
    layout: Res<TaffyLayout>,
    ui: Res<UiState>,
    debug: Res<DebugGui>,
    multiplier: Res<ToolbarSizeMult>,
) {
    // Original142344ecf reads DebugGui+f7, established Show 2D UI.
    if !debug.show_2d_ui {
        return;
    }
    for context in [&*toolbar as &dyn ToolbarContext, &*photomode] {
        // Original chained iterator first yields node_b@20 unconditionally,
        // then each non-null menu node@10 in the stride20 definition Vec.
        for node in std::iter::once(context.node_b()).chain(
            context
                .contextual_menus()
                .iter()
                .filter(|menu| menu.node != taffy::node::Node::default())
                .map(|menu| menu.node),
        ) {
            // Native Taffy::layout Result unwrap is deliberate: a malformed
            // registered node is an invariant failure, not silently omitted.
            let actual = layout.layout(node);
            let size = Vec2::new(actual.size.width, actual.size.height);
            let location = Vec2::new(actual.location.x, actual.location.y);
            // 14234526e..2b1: keep multiplication/addition and center rounding.
            // This expands from location toward max, not symmetrically about
            // the old rectangle. Do not simplify min to location (f32 changes).
            let padding = multiplier.multiplier * 10.0;
            let doubled = padding + padding;
            let half = (size + Vec2::splat(doubled)) * 0.5;
            let center = location + half;
            let min = center - half;
            let max = center + half;
            let name = format!("steal_input_{node:?}");
            let params = WidgetParams::default()
                .screen_aabb(min, max, f32::from_bits(0x3ca3d70a))
                .no_history_recording()
                .sounds(UiSounds::empty());
            // Native makes no occlude_by_world / allow_negative_distance call:
            // preserve true / false defaults, unlike the button producers.
            ui.add(&name, params);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resouces_and_components::toolbar_layout::ContextualMenu;
    use country_core::systems::ui::resources::{ui_state::WidgetId, widget_params::WidgetGeometry};
    use taffy::{
        geometry::Size,
        style::{AvailableSpace, Dimension, Style},
    };

    fn fixture() -> (World, Schedule, Vec<taffy::node::Node>) {
        let mut layout = TaffyLayout::default();
        let mut nodes = Vec::new();
        for _ in 0..3 {
            let node = layout
                .add_node(Style {
                    size: Size {
                        width: Dimension::Points(40.),
                        height: Dimension::Points(20.),
                    },
                    ..Default::default()
                })
                .root();
            layout
                .taffy
                .compute_layout(
                    node,
                    Size {
                        width: AvailableSpace::Definite(100.),
                        height: AvailableSpace::Definite(100.),
                    },
                )
                .unwrap();
            nodes.push(node);
        }
        let mut world = World::new();
        world.insert_resource(ToolbarLayout {
            node_b: nodes[0],
            menus: vec![
                ContextualMenu::default(),
                ContextualMenu {
                    node: nodes[2],
                    ..Default::default()
                },
            ],
            ..Default::default()
        });
        world.insert_resource(PhotomodeToolbarLayout {
            node_b: nodes[1],
            ..Default::default()
        });
        world.insert_resource(layout);
        world.insert_resource(UiState::at_startup());
        world.insert_resource(DebugGui::default());
        world.insert_resource(ToolbarSizeMult { multiplier: 2.0 });
        let mut schedule = Schedule::default();
        schedule.add_systems(toolbar_steal_mouse_input);
        (world, schedule, nodes)
    }

    #[test]
    fn normal_photo_and_existing_contextual_roots_register_real_layout_backgrounds() {
        let (mut world, mut schedule, nodes) = fixture();
        // There deliberately is no ActionSetStates, GameConfig or AppMode:
        // these are not original dependencies and both layouts participate.
        schedule.run(&mut world);
        let ui = world.resource::<UiState>();
        assert_eq!(ui.registry_snapshot().len(), 3);
        for node in nodes {
            let params = ui
                .widget(WidgetId::from_name(&format!("steal_input_{node:?}")))
                .unwrap();
            match params.resolved_geometry().as_ref() {
                WidgetGeometry::ScreenAabb { min, max, distance } => {
                    assert_eq!(*min, Vec2::ZERO);
                    assert_eq!(*max, Vec2::new(80., 60.));
                    assert_eq!(distance.to_bits(), 0x3ca3d70a);
                }
                other => panic!("unexpected geometry {other:?}"),
            }
            assert!(params.occlude_by_world);
            assert!(params.flags.no_history_recording());
            assert!(!params.flags.allow_negative_distance());
            assert_eq!(params.sounds, UiSounds::empty());
        }
    }

    #[test]
    fn show_2d_ui_false_gates_before_even_invalid_layout_access() {
        let (mut world, mut schedule, _) = fixture();
        world.resource_mut::<DebugGui>().show_2d_ui = false;
        world.resource_mut::<ToolbarLayout>().node_b = Default::default();
        schedule.run(&mut world);
        assert!(world.resource::<UiState>().registry_snapshot().is_empty());
    }

    #[test]
    #[should_panic(expected = "taffy node layout")]
    fn invalid_unconditional_toolbar_node_retains_original_unwrap_failure() {
        let (mut world, mut schedule, _) = fixture();
        world.resource_mut::<ToolbarLayout>().node_b = Default::default();
        schedule.run(&mut world);
    }
}
