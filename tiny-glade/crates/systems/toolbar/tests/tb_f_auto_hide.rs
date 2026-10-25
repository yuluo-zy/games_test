//! Original B 0x142343a10: real system ownership, mode filtering and geometry.
use bevy_ecs::prelude::*;
use bevy_time::Time;
use country_core::{
    components::{Ui2dExtent, UiInteractionState},
    game_config::GameConfig,
    input::action_set::ActionSetStates,
    resources::{gui::EscMenuState, mouse_terrain_raycast::CursorPositionSS, window::WindowSize},
    systems::{mode_manager::AppMode, ui::resources::widget_params::PlayUiSound},
};
use glam::Vec2;
use toolbar::{
    resouces_and_components::{
        DebugToolbarActivationZone, ShowOnlyInPhotomode, ToolbarButtonCooldown,
        taffy_layout::{AnchorSide, AnchoredNode, TaffyLayout},
        toolbar_layout::{PhotomodeToolbarLayout, ToolbarLayout},
    },
    toolbar_update::ToolbarSizeMult,
};

#[test]
fn real_auto_hide_system_uses_padded_hit_zone_and_only_active_mode_buttons() {
    let mut world = World::new();
    let mut config = GameConfig::default();
    config.ui.auto_hide_toolbar = true;
    world.insert_resource(config);
    world.insert_resource(Time::<()>::default());
    world.resource_mut::<Time>().advance_by(std::time::Duration::from_secs_f32(0.15));
    world.insert_resource(AppMode::Water);
    // Inside original padded bottom activation zone, outside the bare 33px strip.
    world.insert_resource(CursorPositionSS(Vec2::new(500.0, 500.0)));
    world.insert_resource(EscMenuState::Closed);
    world.insert_resource(WindowSize::new(1000, 600));
    world.init_resource::<ActionSetStates>();
    world.insert_resource(ToolbarSizeMult { multiplier: 1.0 });
    world.init_resource::<Events<PlayUiSound>>();
    let mut layout = TaffyLayout::default();
    let normal = layout.taffy.new_leaf(Default::default()).unwrap();
    let photo = layout.taffy.new_leaf(Default::default()).unwrap();
    for node in [normal, photo] {
        layout.anchored.push(AnchoredNode {
            order: 0, offset: Vec2::ZERO, node, relative_to: node,
            gap: 0.0, side: AnchorSide::Window,
        });
    }
    world.insert_resource(layout);
    world.insert_resource(ToolbarLayout {
        node_a: normal, hide_progress: 1.0, ..Default::default()
    });
    world.insert_resource(PhotomodeToolbarLayout {
        node_a: photo, hide_progress: 1.0, ..Default::default()
    });
    for _ in 0..6 {
        world.spawn((ToolbarButtonCooldown::default(), UiInteractionState::default()));
    }
    for _ in 0..3 {
        world.spawn((ToolbarButtonCooldown::default(), UiInteractionState::default(), ShowOnlyInPhotomode));
    }
    // Every matching debug drawable is written; unrelated extents are preserved.
    let zones: Vec<_> = (0..2).map(|_| world.spawn((Ui2dExtent::default(), DebugToolbarActivationZone)).id()).collect();
    let unrelated = world.spawn(Ui2dExtent {pos: Vec2::ONE, size: Vec2::ONE}).id();
    let mut schedule = Schedule::default();
    schedule.add_systems(toolbar::toolbar_systems::toolbar_auto_hide);
    schedule.run(&mut world);
    for &zone in &zones {
        let actual = world.get::<Ui2dExtent>(zone).unwrap();
        assert_eq!(actual.pos, Vec2::new(500.0, 583.5));
        assert_eq!(actual.size, Vec2::new(790.0, 193.0));
    }
    assert_eq!(world.get::<Ui2dExtent>(unrelated).unwrap().pos, Vec2::ONE);
    assert_eq!(world.resource::<ToolbarLayout>().hide_progress, 0.5);
    assert_eq!(world.resource::<PhotomodeToolbarLayout>().hide_progress, 1.0);
    assert_eq!(world.resource_mut::<Events<PlayUiSound>>().drain().map(|event|event.kind).collect::<Vec<_>>(), vec![0x11]);
    assert_eq!(world.resource::<TaffyLayout>().anchored[0].offset, Vec2::new(0.0, 100.0));
    assert_eq!(world.resource::<TaffyLayout>().anchored[1].offset, Vec2::ZERO);

    // Same real system selects the other layout and counts only its three buttons.
    world.insert_resource(AppMode::Photomode);
    schedule.run(&mut world);
    assert_eq!(world.get::<Ui2dExtent>(zones[0]).unwrap().size, Vec2::new(430.0, 193.0));
    assert_eq!(world.resource::<ToolbarLayout>().hide_progress, 0.5);
    assert_eq!(world.resource::<PhotomodeToolbarLayout>().hide_progress, 0.5);

    // Disabled auto-hide resets selected progress/offset and does not rewrite zones.
    world.resource_mut::<GameConfig>().ui.auto_hide_toolbar = false;
    world.resource_mut::<GameConfig>().ui.icon_size_mult = 2.0;
    schedule.run(&mut world);
    assert_eq!(world.resource::<PhotomodeToolbarLayout>().hide_progress, 0.0);
    assert_eq!(world.resource::<TaffyLayout>().anchored[1].offset, Vec2::ZERO);
    assert_eq!(world.get::<Ui2dExtent>(zones[0]).unwrap().size, Vec2::new(430.0, 193.0));
}
