//! Real Query/EventReader systems against fixed original instruction outputs.
use bevy_ecs::prelude::*;
use country_core::{
    resources::toolbar::ToolbarCategory,
    systems::{mode_manager::AppMode, ui::resources::widget_params::PlayUiSound},
};
use toolbar::{
    resouces_and_components::*,
    ui_interaction::{UiClickEvent, UiDragEvent, UiJustBecameInFocusEvent},
    ui_presentation::{ui_appear_animation, ui_focus_animation, ui_sounds},
};
fn n(b: &[u8], a: usize) -> u32 {
    u32::from_le_bytes(b[a..a + 4].try_into().unwrap())
}
fn f(b: &[u8], a: usize) -> f32 {
    f32::from_bits(n(b, a))
}
fn assert_float(actual: f32, expected: f32, index: usize) {
    if expected.is_nan() {
        assert!(actual.is_nan(), "NaN case{index}");
    } else {
        assert_eq!(
            actual.to_bits(),
            expected.to_bits(),
            "native bits case{index}"
        );
    }
}
#[test]
fn real_focus_query_preserves_all_optional_component_combinations_and_native_scale() {
    let fixture=include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../../reverse-engineering/tiny-glade/progress-20261020/phase2-ui-presentation/native-focus.bin"));
    let mut world = World::new();
    let mut entities = Vec::new();
    for b in fixture.chunks_exact(20) {
        let mut node = TaffyNode::default();
        node.set_scale(f(b, 12));
        let mut entity = world.spawn((
            node,
            UiAnimationState {
                select: f(b, 8),
                state: 7,
                ..Default::default()
            },
        ));
        if b[0] != 0 {
            entity.insert(UiSlider::default());
        }
        if b[1] != 0 {
            entity.insert(UiSliderReset { flag: 7 });
        }
        if b[2] != 0 {
            entity.insert(UiCustomSize {
                kind: b[3] as u32,
                value: f(b, 4),
            });
        }
        entities.push(entity.id());
    }
    let mut schedule = Schedule::default();
    schedule.add_systems(ui_focus_animation);
    schedule.run(&mut world);
    for (index, (entity, b)) in entities.iter().zip(fixture.chunks_exact(20)).enumerate() {
        assert_float(
            world.get::<TaffyNode>(*entity).unwrap().scale(),
            f(b, 16),
            index,
        );
        assert_eq!(world.get::<UiAnimationState>(*entity).unwrap().state, 7);
    }
    assert_eq!(entities.len(), 1848);
}
#[test]
fn real_appear_query_uses_original_weighted_zero_expression_and_multiplies_existing_scale() {
    let fixture=include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../../reverse-engineering/tiny-glade/progress-20261020/phase2-ui-presentation/native-appear.bin"));
    let mut world = World::new();
    let mut entities = Vec::new();
    for b in fixture.chunks_exact(12) {
        let mut node = TaffyNode::default();
        node.set_scale(f(b, 4));
        entities.push(
            world
                .spawn((
                    node,
                    UiAnimationState {
                        appear: f(b, 0),
                        state: 7,
                        ..Default::default()
                    },
                ))
                .id(),
        );
    }
    let mut schedule = Schedule::default();
    schedule.add_systems(ui_appear_animation);
    schedule.run(&mut world);
    for (index, (entity, b)) in entities.iter().zip(fixture.chunks_exact(12)).enumerate() {
        assert_float(
            world.get::<TaffyNode>(*entity).unwrap().scale(),
            f(b, 8),
            index,
        );
        assert_eq!(world.get::<UiAnimationState>(*entity).unwrap().state, 7);
    }
    assert_eq!(entities.len(), 66);
}
#[test]
fn real_sound_queries_and_three_event_readers_match_original_priority_all_drag_bytes_and_consumption(
) {
    let mut world = World::new();
    world.init_resource::<Events<UiJustBecameInFocusEvent>>();
    world.init_resource::<Events<UiClickEvent>>();
    world.init_resource::<Events<UiDragEvent>>();
    world.init_resource::<Events<PlayUiSound>>();
    let mut entities = Vec::new();
    for mask in 0..32 {
        for tag in 0..18 {
            let mut entity = world.spawn_empty();
            if mask & 1 != 0 {
                entity.insert(UiLocked);
            }
            if mask & 2 != 0 {
                entity.insert(SwitchAppModeButton {
                    mode: AppMode::Photomode,
                });
            }
            if mask & 4 != 0 {
                entity.insert(SwitchToolbarCategoryButton {
                    category: ToolbarCategory::Building,
                });
            }
            if mask & 8 != 0 {
                entity.insert(UiSlider::default());
            }
            if mask & 16 != 0 {
                entity.insert(PhotomodeButton { tag });
            }
            entities.push(entity.id());
        }
    }
    // Stale generation and out-of-range index: focus/click still emit generic
    // sounds; drag membership rejects them. Native fixture contains both.
    entities.push(Entity::from_bits(entities[0].to_bits() + (1u64 << 32)));
    entities.push(Entity::from_bits((1u64 << 32) | 100000));
    world
        .resource_mut::<Events<UiJustBecameInFocusEvent>>()
        .send_batch(
            entities
                .iter()
                .map(|&entity| UiJustBecameInFocusEvent { entity }),
        );
    world
        .resource_mut::<Events<UiClickEvent>>()
        .send_batch(entities.iter().map(|&entity| UiClickEvent { entity }));
    world.resource_mut::<Events<UiDragEvent>>().send_batch(
        entities
            .iter()
            .flat_map(|&entity| (0..=255).map(move |state| UiDragEvent { entity, state })),
    );
    let mut schedule = Schedule::default();
    schedule.add_systems(ui_sounds);
    schedule.run(&mut world);
    let actual = world
        .resource_mut::<Events<PlayUiSound>>()
        .drain()
        .map(|sound| sound.kind)
        .collect::<Vec<_>>();
    let fixture=include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../../reverse-engineering/tiny-glade/progress-20261020/phase2-ui-presentation/native-sounds-output.bin"));
    let expected = fixture.chunks_exact(4).map(|b| n(b, 0)).collect::<Vec<_>>();
    assert_eq!(actual, expected);
    assert_eq!(actual.len(), 73915);
    schedule.run(&mut world);
    assert!(
        world
            .resource_mut::<Events<PlayUiSound>>()
            .drain()
            .next()
            .is_none(),
        "all three readers consumed exactly once"
    );
    world
        .resource_mut::<Events<UiClickEvent>>()
        .send(UiClickEvent {
            entity: entities[577],
        });
    world
        .resource_mut::<Events<UiClickEvent>>()
        .send(UiClickEvent {
            entity: entities[577],
        });
    schedule.run(&mut world);
    assert_eq!(
        world
            .resource_mut::<Events<PlayUiSound>>()
            .drain()
            .map(|s| s.kind)
            .collect::<Vec<_>>(),
        [14, 14]
    );
}
