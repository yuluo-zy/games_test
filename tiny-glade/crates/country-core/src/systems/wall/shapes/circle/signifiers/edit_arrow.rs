//! 原 B 140b35120：活动圆编辑资源驱动箭头工厂和已有实体的变换。
use super::super::ui_edit_circle::AnimatedCircleSignifierResource;
use crate::{
    systems::ui::signifiers::{SignifierEntityBuilder, SignifierFactory, UiSignifierStream},
    utils::components::transform::Transform,
};
use bevy_ecs::prelude::*;
use std::sync::Arc;

/// 原组件为零大小标记；圆弧实体也携带此标记以共享实时位置和法线。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct EditCircleSignifier;

pub fn signifier_edit_circle_arrow(
    mut query: Query<&mut Transform, With<EditCircleSignifier>>,
    subject: Res<AnimatedCircleSignifierResource>,
    mut stream: ResMut<UiSignifierStream>,
) {
    if subject.dragging.is_none() {
        return;
    }
    let rotation = utils::geometry::rotation_from_normal(subject.normal);
    let position = subject.position;
    let mut factory = SignifierFactory::from_closure(
        &format!(
            "edit_circle_WidgetId {{ hash: {} }}_arrow",
            subject.widget.0
        ),
        Arc::new(move |meshes, shaders, commands| {
            let entity = SignifierEntityBuilder::from_mesh_name("flat_arrow")
                .rotation(rotation)
                .position(position)
                .scale(2.0)
                .animation_duration(0.0)
                .build(meshes, shaders, commands);
            commands.entity(entity).insert(EditCircleSignifier);
            vec![entity]
        }),
    );
    factory.marker = 0x142aed888;
    stream.write_factory(&factory);
    for mut transform in &mut query {
        // 原查询写旋转和位置，保留动画系统拥有的 scale。
        transform.rotation = rotation;
        transform.translation = position;
    }
}

#[cfg(test)]
mod world_tests {
    use super::*;
    use crate::{
        asset_libraries::{
            Handle,
            mesh_library::{AssetMeshLibrary, MeshAsset},
            shader_library::AssetShaderLibrary,
        },
        render::{mesh::Mesh, shader::RasterShaderProgram, test_device::TestDevice},
        resources::walls::WallId,
        systems::ui::{
            resources::ui_state::{UiState, WidgetId},
            signifiers::{SignifierAnimation, signifier_create::UiSignifierEntry},
        },
        systems::wall::shapes::circle::signifiers::edit_line::{
            SignifierEditCircleShapeLine, signifier_edit_circle_line,
        },
    };
    use bevy_ecs::{system::RunSystemOnce, world::CommandQueue};
    use glam::{Vec2, Vec3};
    fn wake(world: &mut World, shaders: &AssetShaderLibrary) -> Vec<Entity> {
        let entries = world.resource_mut::<UiSignifierStream>().take_entries();
        let meshes = world.remove_resource::<AssetMeshLibrary>().unwrap();
        let mut queue = CommandQueue::default();
        let mut entities = Vec::new();
        let widget = world.resource::<AnimatedCircleSignifierResource>().widget;
        let mut identities = Vec::new();
        for entry in entries {
            let UiSignifierEntry::Factory(factory) = entry else {
                panic!("圆编辑禁止旧数值记录");
            };
            let spawned = factory.wake(&meshes, shaders, &mut Commands::new(&mut queue, world));
            for &entity in &spawned {
                identities.push((
                    entity,
                    crate::systems::ui::signifiers::SignifierId(
                        crate::systems::ui::signifiers::signifier_create::record_id(
                            widget,
                            factory.name(),
                        ),
                        widget,
                    ),
                ));
            }
            entities.extend(spawned);
        }
        queue.apply(world);
        for (entity, identity) in identities {
            world.entity_mut(entity).insert(identity);
        }
        world.insert_resource(meshes);
        entities
    }
    #[test]
    fn real_world_arrow_arc_follow_subject_reverse_animation_and_release_transient_mesh() {
        let mut world = World::new();
        let meshes = AssetMeshLibrary::new();
        meshes.add(MeshAsset::new(
            Mesh::new("flat_arrow"),
            Some("flat_arrow".to_owned()),
        ));
        let shaders = AssetShaderLibrary::new();
        shaders.add_raster(
            RasterShaderProgram::new_draw(
                "vertex_color_shader",
                Default::default(),
                &TestDevice(1),
            ),
            false,
        );
        world.insert_resource(meshes);
        world.insert_resource(UiState::at_startup());
        world.init_resource::<UiSignifierStream>();
        world.insert_resource(AnimatedCircleSignifierResource {
            widget: WidgetId(41),
            wall: WallId(9),
            circle: utils::geometry::Circle2d::new(Vec2::new(2.0, 3.0), 4.0),
            position: Vec3::new(6.0, 2.0, 3.0),
            normal: Vec3::X,
            dragging: Some(false),
        });
        world.run_system_once(signifier_edit_circle_arrow).unwrap();
        world.run_system_once(signifier_edit_circle_line).unwrap();
        let entities = wake(&mut world, &shaders);
        assert_eq!(entities.len(), 2);
        let arrow = entities[0];
        let line = entities[1];
        assert!(world.get::<SignifierEditCircleShapeLine>(line).is_some());
        assert!(world.get::<EditCircleSignifier>(line).is_some());
        let arrow_animation = world.get::<SignifierAnimation>(arrow).unwrap();
        assert_eq!(arrow_animation.duration, 0.0);
        assert_eq!(arrow_animation.value, 2.0);
        let line_animation = world.get::<SignifierAnimation>(line).unwrap();
        assert_eq!(line_animation.duration, 0.3);
        assert_eq!(line_animation.flag_8, 1);
        let handle = *world.get::<Handle<Mesh>>(line).unwrap();
        for progress in [0.0, 0.5, 1.0] {
            world.get_mut::<SignifierAnimation>(line).unwrap().eased = progress;
            world.run_system_once(signifier_edit_circle_line).unwrap();
            world.resource_mut::<UiSignifierStream>().take_entries();
            let mesh = world.resource::<AssetMeshLibrary>().get(handle).unwrap();
            assert!(mesh.positions_without_indices().len() >= 2);
        }
        world
            .resource_mut::<AnimatedCircleSignifierResource>()
            .position = Vec3::new(9.0, 4.0, 8.0);
        world
            .resource_mut::<AnimatedCircleSignifierResource>()
            .normal = Vec3::Z;
        world.run_system_once(signifier_edit_circle_arrow).unwrap();
        for &entity in &entities {
            assert_eq!(
                world.get::<Transform>(entity).unwrap().translation,
                Vec3::new(9.0, 4.0, 8.0)
            );
        }
        world.resource_mut::<UiSignifierStream>().take_entries();
        world
            .resource_mut::<AnimatedCircleSignifierResource>()
            .dragging = None;
        world.get_mut::<SignifierAnimation>(line).unwrap().eased = 0.25;
        world.run_system_once(signifier_edit_circle_arrow).unwrap();
        world.run_system_once(signifier_edit_circle_line).unwrap();
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
        assert_eq!(
            world.get::<Transform>(arrow).unwrap().translation,
            Vec3::new(9.0, 4.0, 8.0)
        );
        assert!(
            world
                .resource::<AssetMeshLibrary>()
                .modified_meshes
                .contains(&handle)
        );
        world.get_mut::<SignifierAnimation>(line).unwrap().t = 1.0;
        world.insert_resource(crate::resources::time::GameTime {
            delta: 0.1,
            ..Default::default()
        });
        let mut cleanup = Schedule::default();
        cleanup.add_systems(
            (
                crate::systems::ui::signifiers::signifier_mark_for_delete::signfier_mark_for_delete,
                crate::systems::ui::signifiers::animation::signifier_animation_t,
                signifier_edit_circle_line,
                crate::systems::ui::signifiers::signifier_mark_for_delete::despawn_marked,
            )
                .chain(),
        );
        cleanup.run(&mut world);
        assert!(world.get_entity(arrow).is_err());
        assert!(world.get_entity(line).is_ok(), "圆弧必须完成0.3秒反向动画");
        for _ in 0..3 {
            cleanup.run(&mut world);
        }
        assert_eq!(world.entities().len(), 0);
        let mut meshes = world.remove_resource::<AssetMeshLibrary>().unwrap();
        assert_eq!(*meshes.transient_meshes().lock().unwrap(), vec![handle]);
        crate::asset_libraries::transient_mesh::delete_dropped_transient_meshes(
            &TestDevice(1),
            &mut meshes,
            &mut crate::asset_libraries::vao_library::AssetVAOLibrary::new(),
        );
        assert!(meshes.get(handle).is_none());
        assert!(meshes.transient_meshes().lock().unwrap().is_empty());
    }
}
