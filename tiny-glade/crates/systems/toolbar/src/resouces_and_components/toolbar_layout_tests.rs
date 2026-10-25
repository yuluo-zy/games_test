use super::*;
use crate::resouces_and_components::*;
use crate::resouces_and_components::taffy_layout::{AnchorSide, TaffyLayout};
use bevy_ecs::{prelude::*, world::CommandQueue};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

#[derive(Component)]
struct CustomPayload(u32);

#[test]
fn all_item_branches_create_real_components_dimensions_and_parent_identity() {
    let mut world = World::new();
    let parent_entity = world.spawn_empty().id();
    let mut queue = CommandQueue::default();
    let mut taffy = taffy::node::Taffy::new();
    let parent = taffy.new_leaf(Default::default()).unwrap();
    let calls = Arc::new(AtomicUsize::new(0));
    let closure_calls = calls.clone();
    let kinds = vec![
        ToolbarItemKind::Custom(Box::new(move |entity| {
            closure_calls.fetch_add(1, Ordering::SeqCst);
            entity.insert(CustomPayload(73));
        })),
        ToolbarItemKind::AppMode(AppMode::default()),
        ToolbarItemKind::Slider(5),
        ToolbarItemKind::SliderReset(12),
        ToolbarItemKind::Spacer,
        ToolbarItemKind::Locked,
    ];
    let mut ids = Vec::new();
    {
        let mut commands = Commands::new(&mut queue, &world);
        for (i, kind) in kinds.into_iter().enumerate() {
            let mut item = ToolbarItem::new(kind, format!("actual-item-{i}"), Vec2::new(0.5, 2.));
            item.show_only_fpv = i == 5;
            ids.push(add_toolbar_item(&mut taffy, parent, item, &mut commands, parent_entity).unwrap());
        }
    }
    queue.apply(&mut world);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(world.get::<CustomPayload>(ids[0]).unwrap().0, 73);
    for &id in &ids {
        let node = world.get::<TaffyNode>(id).unwrap();
        assert_eq!(crate::taffy_update::taffy_node_base_size(node), Vec2::new(41.25, 165.));
        assert_eq!(node.scale(), 1.);
        assert_eq!(world.get::<UiMouseInteractable>(id).unwrap().bytes, 0.011f32.to_le_bytes());
        assert_eq!(world.get::<UiShowIfParentIsInFocus>(id).unwrap().parent, parent_entity.to_bits());
        assert_eq!(taffy.style(node.node).unwrap().align_items, Some(taffy::style::AlignItems::Center));
    }
    assert!(world.get::<SwitchAppModeButton>(ids[1]).is_some());
    let slider = world.get::<UiSlider>(ids[2]).unwrap();
    assert_eq!(slider.kind(), 5);
    assert_eq!(slider.previous_value_01(), 0.5);
    assert_eq!(slider.hover_progress(), 0.);
    assert!(slider.knob_entity().is_none());
    assert_eq!(world.get::<UiNav>(ids[2]).unwrap().links.map(|link|link.tag), [2,2,0,0]);
    assert_eq!(world.get::<UiSliderReset>(ids[3]).unwrap().flag, 12);
    let spacer = world.get::<UiCustomSize>(ids[4]).unwrap();
    assert_eq!((spacer.kind, spacer.value), (1,1.));
    assert!(world.get::<UiNav>(ids[4]).is_none());
    assert!(world.get::<ToolbarExpand>(ids[4]).is_none());
    assert!(world.get::<UiLocked>(ids[5]).is_some());
    assert!(world.get::<UiNav>(ids[5]).is_none());
    assert!(world.get::<ToolbarExpand>(ids[5]).is_some());
    assert!(world.get::<ShowOnlyInFpvAndFree>(ids[5]).is_some());
    for &id in &ids[..4] {
        assert!(world.get::<ToolbarExpand>(id).is_some());
        assert!(world.get::<UiNav>(id).is_some());
    }
}

#[test]
fn nested_row_column_preserve_focus_parent_and_do_not_spawn_container_entities() {
    let mut world = World::new();
    let focus = world.spawn_empty().id();
    let mut queue = CommandQueue::default();
    let mut layout = TaffyLayout::default();
    let (screen, toolbar) = ToolbarLayout::setup_screen_and_toolbar_nodes(&mut layout);
    let item = ToolbarItem::new(ToolbarItemKind::Row(vec![
        ToolbarItem::new(ToolbarItemKind::Column(vec![
            ToolbarItem::new(ToolbarItemKind::Locked, "leaf", Vec2::ONE)
        ]), "column", Vec2::ONE)
    ]), "row", Vec2::ONE);
    let mut commands = Commands::new(&mut queue, &world);
    assert!(add_toolbar_item(&mut layout.taffy, toolbar, item, &mut commands, focus).is_none());
    queue.apply(&mut world);
    assert_eq!(world.entities().len(), 2);
    let row = layout.children(toolbar)[0];
    let column = layout.children(row)[0];
    assert_eq!(layout.taffy.style(row).unwrap().flex_direction, taffy::style::FlexDirection::Row);
    assert_eq!(layout.taffy.style(column).unwrap().flex_direction, taffy::style::FlexDirection::Column);
    assert_eq!(layout.taffy.style(row).unwrap().gap.width, taffy::style::LengthPercentage::Points(10.));
    let id = world.query_filtered::<Entity, With<UiLocked>>().single(&world).unwrap();
    assert_eq!(world.get::<UiShowIfParentIsInFocus>(id).unwrap().parent, focus.to_bits());
    assert_eq!(layout.children(screen), vec![toolbar]);
}

#[test]
fn contextual_menu_empty_branch_skips_lookup_and_nonempty_uses_registered_parent() {
    let mut world = World::new();
    let focus = world.spawn_empty().id();
    let mut queue = CommandQueue::default();
    let mut taffy = TaffyLayout::default();
    let (screen, toolbar) = ToolbarLayout::setup_screen_and_toolbar_nodes(&mut taffy);
    let mut layout = ToolbarLayout::default();
    let mut commands = Commands::new(&mut queue, &world);
    assert!(layout.add_contextual_menu(&mut taffy, 255, vec![], &mut commands).is_none());
    assert_eq!(taffy.anchored.len(), 1);
    layout.menus.push(ContextualMenu { relative_to: toolbar, parent_entity: focus, tag: 5, ..Default::default() });
    let menu = layout.add_contextual_menu(&mut taffy, 5, vec![ToolbarItem::new(ToolbarItemKind::Locked, "locked", Vec2::ONE)], &mut commands).unwrap();
    queue.apply(&mut world);
    assert_eq!(layout.menus[0].node, menu);
    assert_eq!(taffy.anchored[1].relative_to, toolbar);
    assert_eq!(taffy.anchored[1].gap, 10.);
    assert_eq!(taffy.anchored[1].side, AnchorSide::Above);
    assert_eq!(taffy.taffy.style(menu).unwrap().align_items, Some(taffy::style::AlignItems::Center));
    let id = world.query_filtered::<Entity, With<UiLocked>>().single(&world).unwrap();
    assert_eq!(world.get::<UiShowIfParentIsInFocus>(id).unwrap().parent, focus.to_bits());
    assert_eq!(taffy.children(screen), vec![toolbar]);
}

#[test]
fn original_category_size_override_and_panic_drop_lifetimes() {
    let mut world=World::new(); let mut queue=CommandQueue::default();
    let mut taffy=TaffyLayout::default();
    let (screen,root)=ToolbarLayout::setup_screen_and_toolbar_nodes(&mut taffy);
    let mut layout=ToolbarLayout {node_a:screen,node_b:root,..Default::default()};
    let id=layout.add_category_button(&mut taffy,country_core::resources::toolbar::ToolbarCategory::Building,"probe",
        &mut Commands::new(&mut queue,&world), |entity| {entity.insert(CustomPayload(34));},Some(0.75));
    queue.apply(&mut world);
    assert_eq!(crate::taffy_update::taffy_node_base_size(world.get::<TaffyNode>(id).unwrap()),Vec2::splat(82.5));
    assert_eq!(world.get::<CustomPayload>(id).unwrap().0,34);

    struct DropProbe(Arc<AtomicUsize>);
    impl Drop for DropProbe {fn drop(&mut self){self.0.fetch_add(1,Ordering::SeqCst);}}
    let drops=Arc::new(AtomicUsize::new(0));
    let pending=DropProbe(drops.clone());
    let item=ToolbarItem::new(ToolbarItemKind::Row(vec![ToolbarItem::new(
        ToolbarItemKind::Custom(Box::new(move |_| {drop(pending);})),"owned",Vec2::ONE)]),"row",Vec2::ONE);
    let invalid=taffy::node::Node::default();
    let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{
        add_toolbar_item(&mut taffy.taffy,invalid,item,&mut Commands::new(&mut queue,&world),id);
    }));
    assert!(result.is_err());
    assert_eq!(drops.load(Ordering::SeqCst),1,"unconsumed callback and child tail must drop once on panic");
}
