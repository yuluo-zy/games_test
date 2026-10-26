use super::*;
use crate::resouces_and_components::*;
use bevy_ecs::{prelude::*, world::CommandQueue};

fn flatten(item: ToolbarItem, depth: usize, out: &mut Vec<String>) {
    let (kind, sub, children) = match item.kind {
        K::Custom(callback) => {
            let mut world = World::new();
            let entity = world.spawn_empty().id();
            let mut queue = CommandQueue::default();
            callback(&mut Commands::new(&mut queue, &world).entity(entity));
            queue.apply(&mut world);
            (0, world.get::<PhotomodeButton>(entity).unwrap().tag, None)
        }
        K::Slider(kind) => (2, kind, None),
        K::SliderReset(kind) => (3, kind, None),
        K::Row(children) => (4, 0, Some(children)),
        K::Column(children) => (5, 0, Some(children)),
        K::Spacer => (6, 0, None),
        K::Locked => (7, 0, None),
        K::AppMode(_) => panic!("original photo tree has no AppMode leaf"),
    };
    out.push(format!(
        "{depth}:{kind}:{}:{:08x}:{:08x}:{}:{sub}",
        item.label,
        item.size.x.to_bits(),
        item.size.y.to_bits(),
        u8::from(item.show_only_fpv)
    ));
    if let Some(children) = children {
        for child in children {
            flatten(child, depth + 1, out);
        }
    }
}

#[test]
fn complete_edition_trees_and_executed_callbacks_match_original_capture() {
    const FULL: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/native/photo-tree-full.txt"
    ));
    const DEMO: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/native/photo-tree-demo.txt"
    ));
    for (demo, expected) in [(false, FULL), (true, DEMO)] {
        let mut actual = Vec::new();
        for (label, category, items) in photomode_category_items(demo) {
            actual.push(format!("category:{}:{label}", category as u8));
            for item in items {
                flatten(item, 0, &mut actual);
            }
        }
        assert_eq!(
            actual,
            expected.lines().collect::<Vec<_>>(),
            "original demo={demo}"
        );
    }
}

#[test]
fn real_photo_setup_preserves_every_root_callback_size_parent_and_nav_ring() {
    for demo in [false, true] {
        for alignment in [
            ToolbarAlignment::Bottom,
            ToolbarAlignment::Left,
            ToolbarAlignment::Right,
            ToolbarAlignment::Top,
        ] {
            let mut world = World::new();
            let mut queue = CommandQueue::default();
            let mut taffy = TaffyLayout::default();
            let layout = setup_photomode_toolbar(
                &mut taffy,
                &mut Commands::new(&mut queue, &world),
                demo,
                alignment,
            );
            queue.apply(&mut world);
            let order = [12, 13, 14, 16, 15, 17, 18, 19, 20, 11, 21];
            assert_eq!(
                layout.menus.iter().map(|r| r.tag).collect::<Vec<_>>(),
                order
            );
            assert_eq!(taffy.children(layout.node_b).len(), 11);
            assert_eq!(taffy.anchored.len(), if demo { 5 + 1 } else { 6 + 1 });
            for (index, row) in layout.menus.iter().enumerate() {
                let entity = row.parent_entity;
                let expected_action = match row.tag {
                    12 => Some(4),
                    13 => Some(5),
                    15 => Some(15),
                    17 => Some(9),
                    19 if !demo => Some(11),
                    21 => Some(17),
                    _ => None,
                };
                assert_eq!(
                    world.get::<PhotomodeButton>(entity).map(|b| b.tag),
                    expected_action
                );
                assert_eq!(
                    world.get::<UiLocked>(entity).is_some(),
                    [14, 16, 18, 20].contains(&row.tag)
                );
                assert_eq!(
                    world.get::<TimeOfDayButtonV2>(entity).is_some(),
                    row.tag == 11
                );
                assert!(world.get::<ShowOnlyInPhotomode>(entity).is_some());
                assert!(world.get::<SwitchToolbarCategoryButton>(entity).is_none());
                assert!(world.get::<ToolbarExpand>(entity).is_some());
                assert!(world.get::<HideIfMenuIsOpen>(entity).is_some());
                assert_eq!(
                    world
                        .get::<ToolbarButtonCooldown>(entity)
                        .unwrap()
                        .remaining,
                    0.
                );
                assert_eq!(world.get::<UiAnimationState>(entity).unwrap().appear, 1.);
                assert_eq!(world.get::<UiInteractionState>(entity).unwrap().state, 1);
                let node = world.get::<TaffyNode>(entity).unwrap();
                assert_eq!(
                    crate::taffy_update::taffy_node_base_size(node),
                    Vec2::splat(if row.tag == 17 { 137.5 } else { 110. })
                );
                let nav = world.get::<UiNav>(entity).unwrap();
                let (next, prev) = if alignment.is_horizontal() {
                    (1, 0)
                } else {
                    (2, 3)
                };
                assert_eq!(
                    nav.links[next].value,
                    layout.menus[(index + 1) % 11].parent_entity.to_bits()
                );
                assert_eq!(
                    nav.links[prev].value,
                    layout.menus[(index + 10) % 11].parent_entity.to_bits()
                );
            }
            let mut query = world.query::<(Entity, &UiShowIfParentIsInFocus)>();
            for (entity, focus) in query.iter(&world) {
                assert!(
                    layout
                        .menus
                        .iter()
                        .any(|row| row.parent_entity.to_bits() == focus.parent)
                );
                assert!(world.get::<ShowOnlyInPhotomode>(entity).is_none());
            }
            // Full-only distortion row has all three leaves gated in FPV/free.
            let mut query = world.query_filtered::<Entity, With<ShowOnlyInFpvAndFree>>();
            assert_eq!(query.iter(&world).count(), if demo { 0 } else { 3 });
            let mut query = world.query::<&UiSlider>();
            let mut kinds = query
                .iter(&world)
                .map(|slider| slider.kind())
                .collect::<Vec<_>>();
            kinds.sort();
            assert_eq!(
                kinds,
                if demo {
                    vec![0, 6, 7, 8]
                } else {
                    vec![0, 1, 2, 3, 4, 6, 7, 8]
                }
            );
        }
    }
}
