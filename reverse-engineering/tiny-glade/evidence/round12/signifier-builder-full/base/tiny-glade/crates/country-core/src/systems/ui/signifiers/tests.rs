//! 真实 World 多帧回归：创建、去重、保活、撤销及瞬态网格回收。

use super::*;
use crate::{
    asset_libraries::{Handle, mesh_library::AssetMeshLibrary, shader_library::AssetShaderLibrary},
    render::{mesh::Mesh, shader::RasterShaderProgram, test_device::TestDevice},
    resources::time::GameTime,
    systems::ui::resources::ui_state::{UiInteraction, UiState, WidgetId},
    utils::debug_viz::DebugLine,
};
use glam::Vec3;

fn setup() -> (World, Schedule) {
    let mut world = World::new();
    register_signifier_resources(&mut world);
    world.init_resource::<AssetMeshLibrary>();
    world.init_resource::<GameTime>();
    let shaders = AssetShaderLibrary::new();
    shaders.add_raster(
        RasterShaderProgram::new_draw("vertex_color_shader", Default::default(), &TestDevice(1)),
        false,
    );
    world.insert_resource(shaders);
    world.insert_resource(UiState::new(UiInteraction {
        native_tag: 1,
        native_phase: 0,
        widget: WidgetId::from_name("signifier_lifecycle"),
    }));
    let mut schedule = Schedule::default();
    add_signifier_systems(&mut schedule);
    (world, schedule)
}

fn submit(world: &mut World, line: &DebugLine) {
    line.try_show(&mut world.resource_mut::<UiSignifierStream>())
        .unwrap();
}

fn entities(world: &mut World) -> Vec<Entity> {
    world
        .query_filtered::<Entity, With<SignifierId>>()
        .iter(world)
        .collect()
}

#[test]
fn requested_lines_are_reused_and_only_withdrawn_lines_are_reclaimed() {
    let (mut world, mut schedule) = setup();
    let first = DebugLine::new(vec![Vec3::ZERO, Vec3::X]);
    let second = DebugLine::new(vec![Vec3::ZERO, Vec3::Y]);
    submit(&mut world, &first);
    submit(&mut world, &first);
    schedule.run(&mut world);
    let first_entity = entities(&mut world)[0];
    let first_mesh = *world.get::<Handle<Mesh>>(first_entity).unwrap();
    assert_eq!(entities(&mut world).len(), 1);
    let registry = world
        .resource::<AssetMeshLibrary>()
        .transient_meshes()
        .clone();
    assert!(registry.lock().unwrap().is_empty());

    submit(&mut world, &first);
    submit(&mut world, &second);
    schedule.run(&mut world);
    assert_eq!(entities(&mut world).len(), 2);
    assert_eq!(
        *world.get::<Handle<Mesh>>(first_entity).unwrap(),
        first_mesh
    );
    let second_entity = entities(&mut world)
        .into_iter()
        .find(|entity| *entity != first_entity)
        .unwrap();
    let second_mesh = *world.get::<Handle<Mesh>>(second_entity).unwrap();

    submit(&mut world, &second);
    schedule.run(&mut world);
    assert!(world.get_entity(first_entity).is_err());
    assert_eq!(entities(&mut world), vec![second_entity]);
    assert_eq!(*registry.lock().unwrap(), vec![first_mesh]);

    schedule.run(&mut world);
    assert!(entities(&mut world).is_empty());
    assert_eq!(*registry.lock().unwrap(), vec![first_mesh, second_mesh]);
    schedule.run(&mut world);
    assert_eq!(*registry.lock().unwrap(), vec![first_mesh, second_mesh]);
}

#[test]
fn inactive_interaction_discards_requests_without_replaying_them_on_reactivation() {
    let (mut world, mut schedule) = setup();
    let line = DebugLine::new(vec![Vec3::ZERO, Vec3::X]);
    submit(&mut world, &line);
    schedule.run(&mut world);
    let mut interaction = world.resource::<UiState>().interaction();
    interaction.native_tag = 0;
    world
        .resource_mut::<UiState>()
        .replace_interaction(interaction);
    submit(&mut world, &line);
    schedule.run(&mut world);
    assert!(entities(&mut world).is_empty());
    assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
    interaction.native_tag = 1;
    world
        .resource_mut::<UiState>()
        .replace_interaction(interaction);
    schedule.run(&mut world);
    assert!(entities(&mut world).is_empty());
    submit(&mut world, &line);
    schedule.run(&mut world);
    assert_eq!(entities(&mut world).len(), 1);
}

#[test]
fn withdrawn_line_respects_minimum_lifespan_and_can_be_requested_again() {
    let (mut world, mut schedule) = setup();
    let line = DebugLine::new(vec![Vec3::ZERO, Vec3::X]);
    submit(&mut world, &line);
    schedule.run(&mut world);
    let entity = entities(&mut world)[0];
    world.entity_mut(entity).insert(SignifierMinimumLifespan {
        t: 0.0,
        min_lifespan: 1.0,
    });
    schedule.run(&mut world);
    assert!(world.get_entity(entity).is_ok());
    // 模拟寿命达到阈值：本测试只验证回收条件，仍有请求时必须继续保留。
    world.get_mut::<SignifierMinimumLifespan>(entity).unwrap().t = 1.0;
    submit(&mut world, &line);
    schedule.run(&mut world);
    assert_eq!(entities(&mut world), vec![entity]);
    schedule.run(&mut world);
    assert!(world.get_entity(entity).is_err());
}
