//! Original toolbar_setup category tree (142345890) and its complete two
//! edition branches. Data emitted from the original machine code, not labels.
//! Native capture: phase2-toolbar-layout/native-toolbar-setup-trees.json.
use bevy_ecs::system::Commands;
use glam::Vec2;
use country_core::{resources::toolbar::ToolbarCategory, resources::ui_settings::ToolbarAlignment,
    systems::mode_manager::{AppMode, WallShape, EraserLayer, DecoratorType}};
use crate::resouces_and_components::{toolbar_layout::{ToolbarItem, ToolbarItemKind as K, ToolbarLayout},
    taffy_layout::TaffyLayout, SwitchToolbarCategoryButton, UiLocked, TimeOfDayButtonV2};

/// Original Ui2dPlugin OnEnter(AppLoadingState::Game) constructor. The
/// original resets Taffy once before constructing all three layout owners.
pub fn toolbar_setup(
    mut taffy: bevy_ecs::prelude::ResMut<TaffyLayout>,
    config: bevy_ecs::prelude::Res<country_core::game_config::GameConfig>,
    mut commands: Commands,
) {
    *taffy = TaffyLayout::default();
    let toolbar = setup_toolbar(&mut taffy, &mut commands, config.dev.demo,
        config.ui.toolbar_alignment);
    let photo = crate::photomode::setup_photomode_toolbar(&mut taffy, &mut commands,
        config.dev.demo, config.ui.toolbar_alignment);
    let undo = crate::resouces_and_components::undo_redo_layout::UndoRedoLayout::setup(
        &mut taffy, &mut commands);
    commands.insert_resource(toolbar);
    commands.insert_resource(photo);
    commands.insert_resource(undo);
}

pub fn toolbar_category_items(demo: bool) -> Vec<(&'static str, ToolbarCategory, Vec<ToolbarItem>)> {
    if demo {
        vec![
            ("toolbar/building", ToolbarCategory::Building, vec![ToolbarItem::new(K::Column(vec![ToolbarItem::new(K::Row(vec![ToolbarItem::new(K::AppMode(AppMode::Wall(WallShape::Freeform)), "toolbar/wall", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::Locked, "toolbar/decorators_lock_demo_fences", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::Row(vec![ToolbarItem::new(K::AppMode(AppMode::Wall(WallShape::Circle)), "toolbar/tower", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::AppMode(AppMode::Wall(WallShape::Rectangle)), "toolbar/building_2", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::Locked, "toolbar/decorators_lock_demo_stairs", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0))]),
            ("toolbar/path", ToolbarCategory::Path, vec![ToolbarItem::new(K::Column(vec![ToolbarItem::new(K::Row(vec![ToolbarItem::new(K::Slider(10), "toolbar/slider_bg_path", Vec2::new(3.0, 1.0)), ToolbarItem::new(K::SliderReset(10), "toolbar/reset_path", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::Row(vec![ToolbarItem::new(K::Locked, "toolbar/decorators_lock_demo_gothic_window", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::Locked, "toolbar/decorators_lock_demo_gothic_window", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::Locked, "toolbar/decorators_lock_demo_gothic_window", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0))]),
            ("toolbar/decorators", ToolbarCategory::Decorators, vec![ToolbarItem::new(K::Column(vec![ToolbarItem::new(K::Row(vec![ToolbarItem::new(K::AppMode(AppMode::Decorator(DecoratorType::CottageWindow)), "toolbar/cottage_window", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::Locked, "toolbar/decorators_lock_demo_gothic_window", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::Locked, "toolbar/decorators_lock_demo_arrow_slit", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::Row(vec![ToolbarItem::new(K::AppMode(AppMode::Decorator(DecoratorType::Lantern)), "toolbar/lantern", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::Locked, "toolbar/decorators_lock_demo_flag", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::Locked, "toolbar/decorators_lock_demo_chimney", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0))]),
            ("toolbar/customization", ToolbarCategory::Customization, vec![]),
            ("toolbar/eraser", ToolbarCategory::Eraser, vec![ToolbarItem::new(K::AppMode(AppMode::Eraser(EraserLayer::Wall)), "toolbar/wall", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::AppMode(AppMode::Eraser(EraserLayer::Path)), "toolbar/path_2", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::AppMode(AppMode::Eraser(EraserLayer::Garden)), "toolbar/garden", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::AppMode(AppMode::Eraser(EraserLayer::Bulldoze)), "toolbar/hammer", Vec2::new(1.0, 1.0))]),
            ("toolbar/terrain", ToolbarCategory::Terrain, vec![]),
            ("toolbar/garden", ToolbarCategory::Garden, vec![ToolbarItem::new(K::Row(vec![ToolbarItem::new(K::Slider(12), "toolbar/slider_bg_garden", Vec2::new(3.0, 1.0)), ToolbarItem::new(K::SliderReset(12), "toolbar/reset_garden", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0))]),
            ("toolbar/decorators_lock_demo_tree", ToolbarCategory::Tree, vec![]),
            ("toolbar/decorators_lock_demo_water", ToolbarCategory::Water, vec![]),
            ("toolbar/photomode", ToolbarCategory::Photomode, vec![]),
            ("toolbar/time_of_day", ToolbarCategory::TimeOfDay, vec![]),
        ]
    } else {
        vec![
            ("toolbar/building", ToolbarCategory::Building, vec![ToolbarItem::new(K::Column(vec![ToolbarItem::new(K::Row(vec![ToolbarItem::new(K::AppMode(AppMode::Wall(WallShape::Freeform)), "toolbar/wall", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::AppMode(AppMode::Fences), "toolbar/fence", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::Row(vec![ToolbarItem::new(K::AppMode(AppMode::Wall(WallShape::Circle)), "toolbar/tower", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::AppMode(AppMode::Wall(WallShape::Rectangle)), "toolbar/building_2", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::AppMode(AppMode::Decorator(DecoratorType::Stairs)), "toolbar/stairs", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0))]),
            ("toolbar/path", ToolbarCategory::Path, vec![ToolbarItem::new(K::Column(vec![ToolbarItem::new(K::Row(vec![ToolbarItem::new(K::Slider(10), "toolbar/slider_bg_path", Vec2::new(3.0, 1.0)), ToolbarItem::new(K::SliderReset(10), "toolbar/reset_path", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::Row(vec![ToolbarItem::new(K::AppMode(AppMode::Path), "toolbar/path_2_large", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::AppMode(AppMode::Cobblestone(1)), "toolbar/cobblestone_1", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::AppMode(AppMode::Cobblestone(2)), "toolbar/cobblestone_2", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::AppMode(AppMode::Cobblestone(3)), "toolbar/cobblestone_3", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0))]),
            ("toolbar/decorators", ToolbarCategory::Decorators, vec![ToolbarItem::new(K::Column(vec![ToolbarItem::new(K::Row(vec![ToolbarItem::new(K::AppMode(AppMode::Decorator(DecoratorType::CottageWindow)), "toolbar/cottage_window", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::AppMode(AppMode::Decorator(DecoratorType::GothicWindow)), "toolbar/gothic_window", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::AppMode(AppMode::Decorator(DecoratorType::ArrowSlit)), "toolbar/arrow_slit", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::Row(vec![ToolbarItem::new(K::AppMode(AppMode::Decorator(DecoratorType::Lantern)), "toolbar/lantern", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::AppMode(AppMode::Decorator(DecoratorType::Flag)), "toolbar/flag", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::AppMode(AppMode::Decorator(DecoratorType::Chimney)), "toolbar/chimney", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0))]),
            ("toolbar/customization", ToolbarCategory::Customization, vec![]),
            ("toolbar/eraser", ToolbarCategory::Eraser, vec![ToolbarItem::new(K::Column(vec![ToolbarItem::new(K::Row(vec![ToolbarItem::new(K::Slider(9), "toolbar/slider_bg_eraser", Vec2::new(3.0, 1.0))]), "", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::Row(vec![ToolbarItem::new(K::AppMode(AppMode::Eraser(EraserLayer::Garden)), "toolbar/garden", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::AppMode(AppMode::Eraser(EraserLayer::Tree)), "toolbar/tree", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::AppMode(AppMode::Eraser(EraserLayer::Water)), "toolbar/water", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::Row(vec![ToolbarItem::new(K::AppMode(AppMode::Eraser(EraserLayer::Wall)), "toolbar/wall", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::AppMode(AppMode::Eraser(EraserLayer::Path)), "toolbar/path_2", Vec2::new(1.0, 1.0)), ToolbarItem::new(K::AppMode(AppMode::Eraser(EraserLayer::Bulldoze)), "toolbar/hammer", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0))]),
            ("toolbar/terrain", ToolbarCategory::Terrain, vec![]),
            ("toolbar/garden", ToolbarCategory::Garden, vec![ToolbarItem::new(K::Row(vec![ToolbarItem::new(K::Slider(12), "toolbar/slider_bg_garden", Vec2::new(3.0, 1.0)), ToolbarItem::new(K::SliderReset(12), "toolbar/reset_garden", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0))]),
            ("toolbar/tree", ToolbarCategory::Tree, vec![]),
            ("toolbar/water", ToolbarCategory::Water, vec![ToolbarItem::new(K::Row(vec![ToolbarItem::new(K::Slider(11), "toolbar/slider_bg_water", Vec2::new(3.0, 1.0)), ToolbarItem::new(K::SliderReset(11), "toolbar/reset_water", Vec2::new(1.0, 1.0))]), "", Vec2::new(1.0, 1.0))]),
            ("toolbar/photomode", ToolbarCategory::Photomode, vec![]),
            ("toolbar/time_of_day", ToolbarCategory::TimeOfDay, vec![]),
        ]
    }
}

/// Normal toolbar portion of the original OnEnterGame setup. The outer owner
/// resets Taffy once, then constructs this, the photomode toolbar and undo/redo.
pub fn setup_toolbar(
    taffy: &mut TaffyLayout,
    commands: &mut Commands,
    demo: bool,
    alignment: ToolbarAlignment,
) -> ToolbarLayout {
    let (screen, toolbar) = ToolbarLayout::setup_screen_and_toolbar_nodes(taffy);
    let mut layout = ToolbarLayout { node_a: screen, node_b: toolbar, ..Default::default() };
    for (label, category, items) in toolbar_category_items(demo) {
        layout.add_category_button(taffy, category, label, commands, |entity| {
            if category == ToolbarCategory::TimeOfDay {
                entity.insert(TimeOfDayButtonV2);
            } else if demo && matches!(category, ToolbarCategory::Tree | ToolbarCategory::Water) {
                entity.insert(UiLocked);
            } else {
                entity.insert(SwitchToolbarCategoryButton { category });
            }
        }, None);
        layout.add_contextual_menu(taffy, category as u8, items, commands);
    }
    let ids: Vec<_> = layout.menus.iter().map(|row| (row.parent_entity, row.parent_entity.to_bits())).collect();
    layout.link_up_cyclic_list_nav(&ids, alignment.is_horizontal(), commands);
    layout
}

#[cfg(test)]
#[path = "toolbar_startup_tests.rs"]
mod startup_tests;
