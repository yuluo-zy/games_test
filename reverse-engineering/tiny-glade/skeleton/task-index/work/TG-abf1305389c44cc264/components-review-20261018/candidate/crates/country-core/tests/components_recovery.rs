use bevy_ecs::component::{Component, ComponentCloneBehavior};
use bevy_ecs::world::World;
use core::mem::{align_of, offset_of, size_of};
use country_core::components::*;

#[test]
fn recovered_payloads_match_b_registration_and_field_offsets() {
    use country_core::render::nani::instance_data::StoneFloorInstanceData;
    type Nani = drawable::NaniBundle<StoneFloorInstanceData>;

    assert_eq!(size_of::<SemiTransparent>(), 0);
    assert_eq!(size_of::<FlatLighting>(), 0);
    assert_eq!(size_of::<BackgroundTerrain>(), 0);
    assert_eq!(size_of::<DontReceiveContactShadows>(), 0);
    assert_eq!(size_of::<DashedLineLength>(), 4);
    assert_eq!(size_of::<OverwriteRoughness>(), 4);
    assert_eq!(size_of::<drawable::NaniInstanceDesc>(), 24);
    assert_eq!(offset_of!(Nani, transform), 0);
    assert_eq!(offset_of!(Nani, desc), 0x30);
    assert_eq!(offset_of!(Nani, marker), 0x48);
    assert_eq!(size_of::<Nani>(), 0x50);
    assert_eq!(size_of::<drawable::DrawableMeshBundle>(), 0x40);
    assert_eq!(
        (size_of::<TreeAnimation>(), align_of::<TreeAnimation>()),
        (0x10, 4)
    );
    assert_eq!(offset_of!(TreeAnimation, duration), 0);
    assert_eq!(offset_of!(TreeAnimation, progress), 4);
    assert_eq!(offset_of!(TreeAnimation, canopy_visibility), 8);
    assert_eq!(offset_of!(TreeAnimation, hidden_this_frame), 12);
    assert_eq!(
        (size_of::<TreeMoveBend>(), align_of::<TreeMoveBend>()),
        (8, 4)
    );
    assert_eq!(
        (size_of::<TreeComponent>(), align_of::<TreeComponent>()),
        (16, 8)
    );
    assert_eq!(offset_of!(TreeComponent, is_preplaced), 8);
    assert_eq!(size_of::<ConvertedPreplacedTree>(), 16);
    assert_eq!(size_of::<TreeMarkedForDeletion>(), 0);
    assert_eq!(size_of::<Ui2dExtent>(), 16);
    assert_eq!(offset_of!(Ui2dExtent, size), 8);
    assert_eq!((size_of::<Ui2dImage>(), align_of::<Ui2dImage>()), (32, 8));
    assert_eq!(offset_of!(Ui2dImage, layer), 24);
    assert_eq!(
        (size_of::<UiDrawOrder>(), align_of::<UiDrawOrder>()),
        (8, 8)
    );
}

#[test]
fn implemented_components_keep_original_default_clone_behavior() {
    fn check<T: Component>() {
        assert!(
            matches!(T::clone_behavior(), ComponentCloneBehavior::Default),
            "{}",
            core::any::type_name::<T>()
        );
        let mut world = World::new();
        let id = world.register_component::<T>();
        assert_eq!(
            world
                .components()
                .get_info(id)
                .unwrap()
                .required_components()
                .iter_ids()
                .count(),
            0
        );
    }
    check::<SemiTransparent>();
    check::<FlatLighting>();
    check::<DashedLineLength>();
    check::<BackgroundTerrain>();
    check::<DontReceiveContactShadows>();
    check::<OverwriteRoughness>();
    check::<TreeAnimation>();
    check::<TreeMoveBend>();
    check::<TreeComponent>();
    check::<ConvertedPreplacedTree>();
    check::<TreeMarkedForDeletion>();
    check::<Ui2dExtent>();
    check::<Ui2dImage>();
    check::<UiDrawOrder>();
    check::<drawable::NaniInstanceDesc>();
}

#[test]
fn tree_progress_stays_with_entity_across_removal_and_archetype_changes() {
    let mut world = World::new();
    let removed = world.spawn(TreeAnimation::new(1.0)).id();
    let survivor = world.spawn(TreeAnimation::preplaced()).id();
    world.get_mut::<TreeAnimation>(survivor).unwrap().progress = 0.5;
    for mut animation in world.query::<&mut TreeAnimation>().iter_mut(&mut world) {
        animation.advance(0.1, false, true);
    }
    let before = world.get::<TreeAnimation>(survivor).unwrap().progress;
    world.despawn(removed);
    world.entity_mut(survivor).insert(TreeMarkedForDeletion);
    for (mut animation, marked) in world
        .query::<(&mut TreeAnimation, Option<&TreeMarkedForDeletion>)>()
        .iter_mut(&mut world)
    {
        animation.advance(0.1, marked.is_some(), true);
    }
    assert_eq!(
        world.get::<TreeAnimation>(survivor).unwrap().progress,
        before - 0.25
    );

    let mut animation = TreeAnimation::preplaced();
    animation.hidden_this_frame = true;
    animation.advance(0.4, false, true);
    assert_eq!(animation.progress, 0.0);
    assert!(!animation.hidden_this_frame);
    // 回收条件读取 progress；固定的生长时长仍为 0.75。
    assert!(country_core::recovered_systems::tree::should_despawn_tree(
        true,
        animation.progress
    ));
    animation.progress = 0.01;
    assert!(!country_core::recovered_systems::tree::should_despawn_tree(
        true,
        animation.progress
    ));
    animation.progress = f32::NAN;
    assert!(!country_core::recovered_systems::tree::should_despawn_tree(
        true,
        animation.progress
    ));
}

#[test]
fn image_name_and_high_order_bits_survive_the_shared_component_boundary() {
    use country_core::systems::render_loop::draw_passes::ui_2d_pass::Ui2dImageRef;
    let mut image = Ui2dImage::new(String::from("toolbar/undo"));
    assert_eq!(image.layer, 0);
    image.layer = 7;
    let snapshot = Ui2dImageRef::from(&image);
    image.name.clear();
    assert_eq!(snapshot.name, "toolbar/undo");
    assert_eq!(snapshot.view().base_layer, 7);
    let order = UiDrawOrder(1 << 40);
    assert!(order > UiDrawOrder(u32::MAX as u64));
}
